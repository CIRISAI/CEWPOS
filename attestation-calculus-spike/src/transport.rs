//! transport.rs — (C) the REAL EDGE WIRE PATH (feature = "real-edge").
//!
//! Stands up two real `ciris-edge` nodes over the production `transport-http`
//! medium and ships a **promoted** [`SignedAttestation`] from node A to node B.
//! This is the third leg of the real trio: the conformant `ciris-persist` emit
//! (`effect::persisted`) produces a real federation-tier `SignedAttestation`,
//! and this module wires it onto a real HTTP wire where node B's inbound
//! pipeline verifies the envelope and republishes it on
//! `subscribe_verified_feed`.
//!
//! KEY API NOTE (corrects the original task premise): `AttestationGossip` is
//! `Delivery::Durable`, NOT `Delivery::Federation`. `Edge::send_federation`
//! would therefore reject it with `DeliveryClassMismatch`. The correct send is
//! [`Edge::send_durable`], which enqueues to the durable persist outbound queue;
//! `Edge::run`'s dispatcher drains it and performs the actual HTTP
//! `Transport::send`. (See the `deviations` report.)
//!
//! The whole module — and every `ciris-edge` / `ciris-keyring` reference — is
//! gated behind `real-edge` (which implies `real-persist`), so neither the
//! default nor the `real-persist` build pulls in `ciris-edge` or its git-dep
//! tree.
#![cfg(feature = "real-edge")]

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine as _;
// ciris-crypto: the PATH crate (also used by the default effect path) — only
// used here to scrub-sign directory KeyRecords and read Ed25519 pubkeys. The
// signatures are raw bytes stored as base64 strings, so they never cross a
// crypto trait boundary with edge/persist (which link git ciris-crypto v8.3.0).
use ciris_crypto::{ClassicalSigner, Ed25519Signer};
use ciris_edge::identity::LocalSigner as EdgeLocalSigner;
use ciris_edge::messages::AttestationGossip;
use ciris_edge::transport::http::{HttpTransport, HttpTransportConfig};
use ciris_edge::transport::Transport;
use ciris_edge::verify::{HybridPolicy, VerifyDirectory};
use ciris_edge::{Edge, EdgeConfig, MessageType, OutboundHandle};
use ciris_persist::prelude::{
    Backend, FederationDirectorySqlite, KeyRecord, SignedAttestation, SignedKeyRecord,
};
// `FederationDirectory` must be in scope to call `put_public_key` on the
// CONCRETE `SqliteBackend` (trait-in-scope is required for non-`dyn` receivers).
use ciris_persist::federation::FederationDirectory;
use ciris_persist::store::sqlite::SqliteBackend;
use sha2::{Digest, Sha256};
use tokio::sync::watch;

/// Boxed error so `?` unifies every distinct error type on the path.
pub type BoxErr = Box<dyn std::error::Error + Send + Sync>;

/// A federation identity at the EDGE layer: a deterministic Ed25519 seed mapped
/// to an edge `key_id`. Distinct from the inner attestation's persist key.
struct FedKey {
    key_id: String,
    seed: [u8; 32],
}

impl FedKey {
    fn new(key_id: &str, seed_byte: u8) -> Self {
        Self { key_id: key_id.to_string(), seed: [seed_byte; 32] }
    }

    fn signer(&self) -> Result<Ed25519Signer, BoxErr> {
        Ed25519Signer::from_seed(&self.seed).map_err(|e| -> BoxErr { format!("ed25519 from_seed: {e}").into() })
    }

    fn pubkey_b64(&self) -> Result<String, BoxErr> {
        let pk = self
            .signer()?
            .public_key()
            .map_err(|e| -> BoxErr { format!("ed25519 public_key: {e}").into() })?;
        Ok(B64.encode(pk))
    }

    /// Build edge's classical-only `LocalSigner` from a 32-byte seed file via
    /// `ciris_keyring::load_local_seed` (no PQC — valid under
    /// `HybridPolicy::Ed25519Fallback`).
    async fn local_signer(&self, base: &Path) -> Result<Arc<EdgeLocalSigner>, BoxErr> {
        let dir = base.join(format!("edge-seed-{}", self.key_id));
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("ed25519.seed");
        std::fs::write(&path, self.seed)?;
        let (classical, _pqc) = ciris_keyring::load_local_seed(ciris_keyring::LocalSeedConfig {
            key_id: self.key_id.clone(),
            key_path: path,
            pqc_key_id: None,
            pqc_key_path: None,
        })
        .await?;
        Ok(Arc::new(EdgeLocalSigner::new(self.key_id.clone(), classical, None)))
    }
}

/// A scrub-signed [`KeyRecord`] for `subject`, signed by `signer` (the
/// bootstrap steward). Each node's directory must hold the OTHER node's Ed25519
/// pubkey row so envelope verification resolves.
fn signed_record(subject: &FedKey, signer: &FedKey, identity_type: &str) -> Result<KeyRecord, BoxErr> {
    let envelope = serde_json::json!({ "key_id": subject.key_id });
    let canonical = serde_json::to_vec(&envelope)?;
    let digest = Sha256::digest(&canonical);
    let sig = signer
        .signer()?
        .sign(digest.as_slice())
        .map_err(|e| -> BoxErr { format!("scrub sign: {e}").into() })?;
    let ts: chrono::DateTime<chrono::Utc> =
        chrono::DateTime::parse_from_rfc3339("2026-05-01T00:00:00Z")?.into();
    Ok(KeyRecord {
        key_id: subject.key_id.clone(),
        pubkey_ed25519_base64: subject.pubkey_b64()?,
        pubkey_ml_dsa_65_base64: None,
        algorithm: "hybrid".to_string(),
        identity_type: identity_type.to_string(),
        identity_ref: subject.key_id.clone(),
        valid_from: ts,
        valid_until: None,
        registration_envelope: envelope,
        original_content_hash: hex::encode(digest),
        scrub_signature_classical: B64.encode(sig),
        scrub_signature_pqc: None,
        scrub_key_id: signer.key_id.clone(),
        scrub_timestamp: ts,
        pqc_completed_at: None,
        persist_row_hash: String::new(),
        roles: Vec::new(),
        attestation_evidence: None,
    })
}

/// A fresh in-memory persist backend serving as BOTH the edge `VerifyDirectory`
/// and the durable `OutboundHandle`, seeded with the given key rows.
async fn backend_with(records: Vec<KeyRecord>) -> Result<Arc<SqliteBackend>, BoxErr> {
    let backend = FederationDirectorySqlite::open(":memory:").await?;
    backend.run_migrations().await?;
    for rec in records {
        backend.put_public_key(SignedKeyRecord { record: rec }).await?;
    }
    Ok(backend)
}

/// Build a real `Edge` node: directory + durable queue (one backend), classical
/// LocalSigner, and one HTTP transport. `Ed25519Fallback` posture (edge's own
/// e2e-test posture) — the envelope is genuinely Ed25519-signed and verified
/// through persist's `verify_hybrid_via_directory`.
async fn build_edge(
    seeds_base: &Path,
    me: &FedKey,
    backend: Arc<SqliteBackend>,
    transport: Arc<dyn Transport>,
) -> Result<Edge, BoxErr> {
    let signer = me.local_signer(seeds_base).await?;
    let config = EdgeConfig { hybrid_policy: HybridPolicy::Ed25519Fallback, ..EdgeConfig::default() };
    let edge = Edge::builder()
        .directory(backend.clone() as Arc<dyn VerifyDirectory>)
        .queue(backend as Arc<dyn OutboundHandle>)
        .signer(signer)
        .transport(transport)
        .config(config)
        .build()?;
    Ok(edge)
}

/// Pick a free localhost port (ephemeral bind-then-release).
fn free_port() -> Result<u16, BoxErr> {
    let port = std::net::TcpListener::bind("127.0.0.1:0")?.local_addr()?.port();
    Ok(port)
}

/// The verified outcome of a 2-node wire transfer, plus a printable evidence log.
pub struct WireOutcome {
    /// Inner SignedAttestation id recovered on node B (== the id A shipped).
    pub received_attestation_id: String,
    /// Envelope `signing_key_id` B verified (node A's edge key).
    pub envelope_signing_key_id: String,
    /// Envelope `destination_key_id` (node B's edge key).
    pub envelope_destination_key_id: String,
    /// Transport the verified frame arrived on (`TransportId("http")`).
    pub transport_id: String,
    /// Inner attestation type / attester / tier (carried verbatim over the wire).
    pub inner_attestation_type: String,
    pub inner_attesting_key_id: String,
    pub inner_tier: String,
    /// Captured evidence log.
    pub log: String,
}

/// Ship `signed` from node A to node B over a real `ciris-edge` transport-http
/// wire and return the verified outcome. Node B verifies the WRAPPING envelope's
/// hybrid signature (via its directory) and republishes the verified envelope on
/// `subscribe_verified_feed`; the inner `SignedAttestation` rides along verbatim.
pub async fn run_wire_transfer(signed: SignedAttestation) -> Result<WireOutcome, BoxErr> {
    use std::fmt::Write as _;
    let mut log = String::new();

    let seeds_base = std::env::temp_dir().join(format!("ciris-spike-edge-{}", std::process::id()));
    std::fs::create_dir_all(&seeds_base)?;

    // Edge-layer identities (envelope signing + directory rows).
    let bootstrap = FedKey::new("bootstrap-steward", 0x01);
    let node_a = FedKey::new("node-a", 0xAA);
    let node_b = FedKey::new("node-b", 0xBB);

    let rows = || -> Result<Vec<KeyRecord>, BoxErr> {
        Ok(vec![
            signed_record(&bootstrap, &bootstrap, "steward")?,
            signed_record(&node_a, &bootstrap, "agent")?,
            signed_record(&node_b, &bootstrap, "agent")?,
        ])
    };
    // Separate in-memory backends so the two outbound dispatchers do not contend;
    // both directories carry both Ed25519 pubkey rows so verification resolves.
    let backend_a = backend_with(rows()?).await?;
    let backend_b = backend_with(rows()?).await?;

    let port_a = free_port()?;
    let port_b = free_port()?;

    // peer_urls is the peer-resolution map used by Transport::send.
    let mut peer_urls_a = HashMap::new();
    peer_urls_a.insert(node_b.key_id.clone(), format!("http://127.0.0.1:{port_b}/edge/inbound"));
    let transport_a = Arc::new(HttpTransport::new(HttpTransportConfig {
        listen_addr: format!("127.0.0.1:{port_a}").parse()?,
        peer_urls: peer_urls_a,
        request_timeout: Duration::from_secs(10),
    })?);
    let transport_b = Arc::new(HttpTransport::new(HttpTransportConfig {
        listen_addr: format!("127.0.0.1:{port_b}").parse()?,
        peer_urls: HashMap::new(),
        request_timeout: Duration::from_secs(10),
    })?);

    let edge_a = build_edge(&seeds_base, &node_a, backend_a, transport_a as Arc<dyn Transport>).await?;
    let edge_b = build_edge(&seeds_base, &node_b, backend_b, transport_b as Arc<dyn Transport>).await?;
    writeln!(
        log,
        "built 2 edge nodes: A={} (port {port_a}) -> B={} (port {port_b})",
        node_a.key_id, node_b.key_id
    )?;

    // Subscribe to B's verified feed BEFORE moving edge_b into run().
    let mut feed = edge_b.subscribe_verified_feed();
    let (_shutdown_b_tx, shutdown_b_rx) = watch::channel(false);
    tokio::spawn(async move {
        if let Err(e) = edge_b.run(shutdown_b_rx).await {
            eprintln!("edge_b.run exited: {e}");
        }
    });

    // Let B's axum listener finish binding before A sends.
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Capture the inner attestation's identity before it is moved into the gossip.
    let att_id = signed.attestation.attestation_id.clone();
    let inner_attestation_type = signed.attestation.attestation_type.clone();
    let inner_attesting_key_id = signed.attestation.attesting_key_id.clone();
    let inner_tier = signed.attestation.tier.clone();

    // send_durable enqueues to the durable persist queue; run()'s dispatcher
    // performs the actual HTTP Transport::send.
    let handle = edge_a.send_durable(&node_b.key_id, AttestationGossip(signed)).await?;
    writeln!(
        log,
        "node A enqueued durable AttestationGossip -> B (queue_id={})",
        handle.queue_id
    )?;

    let (_shutdown_a_tx, shutdown_a_rx) = watch::channel(false);
    tokio::spawn(async move {
        if let Err(e) = edge_a.run(shutdown_a_rx).await {
            eprintln!("edge_a.run exited: {e}");
        }
    });

    let snap = tokio::time::timeout(Duration::from_secs(20), feed.recv())
        .await
        .map_err(|_| -> BoxErr { "timed out waiting for B to verify the attestation".into() })?
        .map_err(|e| -> BoxErr { format!("verified feed channel closed: {e}").into() })?;

    // Recover the inner SignedAttestation from the verified envelope body.
    let gossip: AttestationGossip = serde_json::from_str(snap.envelope.body.get())?;
    let inner = &gossip.0.attestation;

    writeln!(log, "node B RECEIVED+VERIFIED {} over ciris-edge", inner.attestation_id)?;
    writeln!(
        log,
        "  [B] envelope: message_type={:?} signing_key_id={} destination_key_id={} transport={:?}",
        snap.envelope.message_type,
        snap.envelope.signing_key_id,
        snap.envelope.destination_key_id,
        snap.transport_id,
    )?;
    writeln!(
        log,
        "  [B] inner SignedAttestation: type={} attester={} attested={} tier={}",
        inner.attestation_type, inner.attesting_key_id, inner.attested_key_id, inner.tier,
    )?;

    // Envelope identity + inner-attestation integrity.
    assert_eq!(snap.envelope.message_type, MessageType::AttestationGossip);
    assert_eq!(snap.envelope.signing_key_id, node_a.key_id, "envelope signed by node A");
    assert_eq!(snap.envelope.destination_key_id, node_b.key_id, "envelope addressed to node B");
    assert_eq!(inner.attestation_id, att_id, "inner attestation id survives the wire");

    Ok(WireOutcome {
        received_attestation_id: inner.attestation_id.clone(),
        envelope_signing_key_id: snap.envelope.signing_key_id.clone(),
        envelope_destination_key_id: snap.envelope.destination_key_id.clone(),
        transport_id: format!("{:?}", snap.transport_id),
        inner_attestation_type,
        inner_attesting_key_id,
        inner_tier,
        log,
    })
}

/// (C) THE REAL-TRIO WIRE DEMO. Node A CONFORMANT-emits a real federation-tier
/// [`SignedAttestation`] via the persist Engine one-call build-sign-admit, then
/// ships that exact attestation A→B over a real ciris-edge transport-http wire,
/// where B verifies it. Returns the captured evidence log.
pub async fn wire_demo() -> Result<String, BoxErr> {
    use std::fmt::Write as _;
    let mut log = String::new();

    // (A) Real conformant emit -> a real federation-tier SignedAttestation.
    let producer = crate::effect::persisted::Persisted::open_in_memory().await?;
    let thought = crate::pdma::Thought {
        situation: "routine quality scoring (wire demo)".to_string(),
        candidate: crate::pdma::CandidateAction {
            dimension: "evaluation:quality:v1".to_string(),
            score: 0.85,
            confidence: 0.92,
            evidence_refs: vec![],
            predicted_entropy_reduction: 1.0,
            predicted_flourishing_loss: 0.0,
            uncertainty: 0.1,
            novel: false,
            severe_ambiguous_harm: false,
        },
        attesting_key_id: producer.key_id().to_string(),
        subject_key_ids: vec![],
    };
    let form = match crate::pdma::reduce(&thought) {
        crate::pdma::Reduction::Emit(f) => f,
        crate::pdma::Reduction::Defer(_) => return Err("calm Thought unexpectedly deferred".into()),
    };
    let (att_id, signed) = producer.emit_signed(&form, Some(0.92)).await?;
    writeln!(log, "node A emitted {att_id} (conformant federation-tier SignedAttestation)")?;

    // (C) Ship it A->B over the real ciris-edge wire.
    let outcome = run_wire_transfer(signed).await?;
    log.push_str(&outcome.log);
    writeln!(
        log,
        "WIRE OK: B received+verified attestation {} over ciris-edge \
         (envelope {} -> {}, transport {}, inner: type={} attester={} tier={})",
        outcome.received_attestation_id,
        outcome.envelope_signing_key_id,
        outcome.envelope_destination_key_id,
        outcome.transport_id,
        outcome.inner_attestation_type,
        outcome.inner_attesting_key_id,
        outcome.inner_tier,
    )?;
    assert_eq!(outcome.received_attestation_id, att_id, "B must receive A's exact attestation");
    Ok(log)
}
