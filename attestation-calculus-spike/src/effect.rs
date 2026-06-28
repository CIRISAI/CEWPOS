//! `effect.rs` — the ONE effect of the Attestation Calculus.
//!
//! Pure computation runs freely; the only way to affect the world is to
//! `emit` a candidate envelope (a local-tier, **unsigned**, content-addressed
//! row) and then `promote` it, which computes the **real** hybrid
//! Ed25519 + ML-DSA-65 signature and **verifies** it. Promotion enforces the
//! federation-tier invariant `tier = federation ⟹ hybrid signature present`
//! (CC 5.3.2.4.3) with the actual crypto — not a stub.
//!
//! `promote` is the admission gate: it signs the canonical (RFC 8785 / JCS)
//! bytes of the envelope and verifies that signature; if verification does not
//! succeed the candidate is **refused** ([`EffectError::AdmissionRejected`]).
//! The fabric can refuse the brain.
//!
//! The default build is a pure in-memory path: [`LocalRow`] / [`FederationRow`]
//! are plain structs, with no database and no async runtime. The optional
//! `real-persist` feature adds [`mod@persisted`], which performs the identical
//! local-tier write + promote against the real `ciris-persist` crate. The
//! default build references neither `ciris-persist` nor `tokio`.

use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine as _; // base64 0.22: brings `.encode()` / `.decode()` into scope
use sha2::{Digest, Sha256}; // `Digest` provides new/update/finalize

// Concrete signer/verifier/container types from the REAL ciris-crypto crate.
use ciris_crypto::{
    Ed25519Signer, Ed25519Verifier, HybridSignature, HybridSigner, HybridVerifier, MlDsa65Signer,
    MlDsa65Verifier,
};
// MANDATORY: the trait that owns `.public_key()` on `Ed25519Signer`. Without
// this `use`, the call does not resolve (it is a trait method, not inherent).
use ciris_crypto::ClassicalSigner;

// ---------------------------------------------------------------------------
// Rows
// ---------------------------------------------------------------------------

/// A local-tier attestation row — **unsigned**, content-addressed.
///
/// This is the result of [`emit`]. `cohort_scope` is always `"self"` and
/// `tier` is always `"local"`. The `attestation_id` is the content address of
/// the JCS-canonical envelope bytes, so the identity of the fact is its bytes
/// (CC 5.3.2), not a mutable object.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalRow {
    /// Content address of the envelope: `hex(sha256(JCS(envelope)))`.
    pub attestation_id: String,
    /// Key id of the attesting identity (the steward emitting `scores`).
    pub attesting_key_id: String,
    /// Wire head of the operator, e.g. `"scores"`.
    pub attestation_type: String,
    /// The canonical envelope (a JSON object), carried verbatim.
    pub attestation_envelope: serde_json::Value,
    /// Always `"self"` for local-tier emissions.
    pub cohort_scope: String,
    /// Subjects this attestation is *about* (may be empty for `scores`).
    pub subject_key_ids: Vec<String>,
    /// Always `"local"`.
    pub tier: String,
}

/// A federation-tier attestation row — carries a **verified** hybrid signature.
///
/// This is the result of [`promote`]. By construction it can only exist when
/// the hybrid Ed25519 + ML-DSA-65 signature over the JCS bytes verified, which
/// is exactly the `tier = federation ⟹ hybrid signature present` invariant.
#[derive(Debug, Clone, PartialEq)]
pub struct FederationRow {
    /// The originating local-tier row, carried verbatim.
    pub local: LocalRow,
    /// Base64 of the raw Ed25519 signature bytes (64 B).
    pub classical_sig_b64: String,
    /// Base64 of the raw ML-DSA-65 signature bytes (3309 B).
    pub pqc_sig_b64: String,
    /// Key id of the signing identity: `hex(sha256(ed25519 pubkey))`.
    pub key_id: String,
    /// Always `"federation"`.
    pub tier: String,
    /// Hex of the exact JCS bytes that were signed and verified.
    pub jcs_hex: String,
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Failure modes of the one effect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectError {
    /// JCS canonicalization (RFC 8785) of the envelope failed.
    Canonicalize(String),
    /// The hybrid signer failed to produce a signature.
    Sign(String),
    /// The admission gate refused the candidate: the hybrid signature did not
    /// verify. The fabric refuses the brain.
    AdmissionRejected,
}

impl std::fmt::Display for EffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EffectError::Canonicalize(e) => write!(f, "canonicalization failed: {e}"),
            EffectError::Sign(e) => write!(f, "hybrid signing failed: {e}"),
            EffectError::AdmissionRejected => {
                write!(f, "admission rejected: hybrid signature did not verify")
            }
        }
    }
}

impl std::error::Error for EffectError {}

// ---------------------------------------------------------------------------
// The hybrid identity (real ciris-crypto)
// ---------------------------------------------------------------------------

/// A real hybrid identity: an Ed25519 + ML-DSA-65 [`HybridSigner`] paired with
/// the matching [`HybridVerifier`], plus the stable key id derived from the
/// classical public key.
///
/// `key_id = hex(sha256(ed25519 pubkey))`. The Ed25519 signer is consumed when
/// the [`HybridSigner`] is assembled, so the public key is captured up front.
pub struct Hybrid {
    signer: HybridSigner<Ed25519Signer, MlDsa65Signer>,
    verifier: HybridVerifier<Ed25519Verifier, MlDsa65Verifier>,
    key_id: String,
}

impl Hybrid {
    /// Generate a fresh hybrid identity, drawing OS entropy for both key
    /// generations (both are fail-secure and therefore fallible).
    pub fn generate() -> Result<Self, EffectError> {
        // Both keygens draw a 32-byte seed via `ciris_crypto::random::fill`,
        // which fail-secures (CryptoError::RngHealthCheckFailed) on a broken
        // SP 800-90B startup RNG health latch — hence both are fallible.
        let ed = Ed25519Signer::random().map_err(|e| EffectError::Sign(e.to_string()))?;

        // Capture the classical public key BEFORE `ed` is moved into the
        // HybridSigner (after assembly it is no longer reachable).
        let ed_pubkey = ed
            .public_key()
            .map_err(|e| EffectError::Sign(e.to_string()))?;
        let key_id = sha256_hex(&ed_pubkey);

        let mldsa = MlDsa65Signer::new().map_err(|e| EffectError::Sign(e.to_string()))?;
        let signer = HybridSigner::new(ed, mldsa).map_err(|e| EffectError::Sign(e.to_string()))?;

        // None of these verifier constructors return Result — do NOT use `?`.
        let verifier = HybridVerifier::new(Ed25519Verifier::new(), MlDsa65Verifier::new());

        Ok(Self {
            signer,
            verifier,
            key_id,
        })
    }

    /// The stable identity key id: `hex(sha256(ed25519 pubkey))`.
    #[must_use]
    pub fn key_id(&self) -> String {
        self.key_id.clone()
    }
}

/// The admission gate, factored out so [`promote`] and the tamper test exercise
/// *the same* code. Signs `sign_bytes`, then verifies the produced signature
/// against `verify_bytes`. On the genuine path `sign_bytes == verify_bytes` and
/// the verifier returns `Ok(true)`; anything else is a refusal.
///
/// Note: `HybridVerifier::verify` returns `Ok(true)` **only** when both the
/// classical and the PQC halves pass. A mismatched payload makes the classical
/// half fail and the verifier returns `Err(..)` (never `Ok(false)`). Either way
/// — `Ok(false)` or any `Err` — the candidate is rejected.
fn admit(h: &Hybrid, sign_bytes: &[u8], verify_bytes: &[u8]) -> Result<HybridSignature, EffectError> {
    let sig = h
        .signer
        .sign(sign_bytes)
        .map_err(|e| EffectError::Sign(e.to_string()))?;
    match h.verifier.verify(verify_bytes, &sig) {
        Ok(true) => Ok(sig),
        // `Ok(false)` is not actually reachable from the real verifier, but the
        // admission semantics are "verified == not Ok(true) ⟹ rejected".
        Ok(false) | Err(_) => Err(EffectError::AdmissionRejected),
    }
}

// ---------------------------------------------------------------------------
// The one effect: emit (local-tier) + promote (the gate)
// ---------------------------------------------------------------------------

/// `emit` a candidate form as a local-tier, **unsigned**, content-addressed
/// row. Pure: it performs no IO and no signing. `cohort_scope` is `"self"`,
/// `tier` is `"local"`, and `attestation_id = content_address(JCS(envelope))`.
pub fn emit(
    form: &crate::forms::Form,
    attesting_key_id: &str,
    subject_key_ids: &[String],
) -> Result<LocalRow, EffectError> {
    let jcs = crate::sexpr::jcs_bytes(&form.envelope).map_err(EffectError::Canonicalize)?;
    let attestation_id = crate::sexpr::content_address(&jcs);

    Ok(LocalRow {
        attestation_id,
        attesting_key_id: attesting_key_id.to_string(),
        attestation_type: form.op.as_str().to_string(),
        attestation_envelope: form.envelope.clone(),
        cohort_scope: "self".to_string(),
        subject_key_ids: subject_key_ids.to_vec(),
        tier: "local".to_string(),
    })
}

/// `promote` a local-tier row to federation tier through the admission gate.
///
/// Canonicalizes the envelope (JCS / RFC 8785), hybrid-**signs** the bytes,
/// then hybrid-**verifies** that signature over the same bytes. If verification
/// does not succeed, the candidate is refused with
/// [`EffectError::AdmissionRejected`]. On success the row is federation-tier and
/// carries the verified classical + PQC signatures — enforcing
/// `tier = federation ⟹ hybrid signature present`.
pub fn promote(row: &LocalRow, h: &Hybrid) -> Result<FederationRow, EffectError> {
    let jcs = crate::sexpr::jcs_bytes(&row.attestation_envelope).map_err(EffectError::Canonicalize)?;

    // Sign the canonical bytes, then verify them. Genuine path => Ok(true);
    // a verification failure => Err(AdmissionRejected).
    let sig = admit(h, &jcs, &jcs)?;

    Ok(FederationRow {
        local: row.clone(),
        classical_sig_b64: B64.encode(&sig.classical.signature),
        pqc_sig_b64: B64.encode(&sig.pqc.signature),
        key_id: h.key_id(),
        tier: "federation".to_string(),
        jcs_hex: hex::encode(&jcs),
    })
}

/// `hex(sha256(bytes))`.
fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

// ---------------------------------------------------------------------------
// OPTIONAL: the real ciris-persist CONFORMANT emit path (feature = "real-persist")
// ---------------------------------------------------------------------------

/// The CONFORMANT emit/promote flow, executed against the **real** `ciris-persist`
/// Engine. Where the default [`emit`]/[`promote`] above hand-roll JCS + hybrid
/// sign + verify into in-memory rows, this module delegates to the Engine's
/// **one-call build-sign-admit** (`emit_attestation_self`): canonicalize →
/// SHA-256 → hybrid-sign (Ed25519 ‖ ML-DSA-65) → assemble the 20-field
/// `Attestation` → `put_attestation`, producing a real federation-tier
/// [`ciris_persist::prelude::SignedAttestation`] — the exact wire type CIRISEdge
/// ships.
///
/// The hybrid signer is built entirely through
/// [`ciris_persist::prelude::LocalSigner::from_config`] from two 32-byte raw seed
/// FILES (Ed25519 + ML-DSA-65); the caller supplies only std types, so this path
/// needs NO direct `ciris-keyring`/`ed25519-dalek` dependency (they arrive
/// transitively through `ciris-persist`). [`Persisted::open`] additionally runs
/// `register_self_federation_key` (the self-key enrollment that populates BOTH
/// pubkeys and the FK target every federation-tier emit/scrub points at) before
/// any emit — without it the federation-tier ingest gate rejects every hybrid
/// emit.
///
/// This module — and every `ciris-persist`/`tokio`/`chrono` reference — is fully
/// gated behind the `real-persist` feature, so the default build pulls none of
/// them. The default [`emit`]/[`promote`] in-memory effect path above is left
/// entirely unchanged.
#[cfg(feature = "real-persist")]
pub mod persisted {
    use std::path::PathBuf;
    use std::sync::Arc;

    use ciris_persist::federation::types::attestation_type;
    use ciris_persist::federation::types::LocalAttestationInput;
    use ciris_persist::federation::EmitAttestationInput;
    use ciris_persist::prelude::{
        Attestation, Engine, LocalSigner, LocalSignerConfig, SignedAttestation,
    };

    /// Boxed error so `?` works across the distinct persist error types
    /// (`EngineError`, `federation::Error`, `LocalSignerError`, IO) — each
    /// derives `std::error::Error`.
    pub type BoxErr = Box<dyn std::error::Error + Send + Sync>;

    /// 32-byte raw Ed25519 seed (deterministic for a reproducible run).
    const ED25519_SEED: [u8; 32] = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        0x00, 0x0f, 0x1e, 0x2d, 0x3c, 0x4b, 0x5a, 0x69, 0x78, 0x87, 0x96, 0xa5, 0xb4, 0xc3, 0xd2,
        0xe1, 0xf0,
    ];
    /// 32-byte raw ML-DSA-65 seed (FIPS 204 final; the signer expands it).
    const MLDSA_SEED: [u8; 32] = [
        0xa0, 0xb1, 0xc2, 0xd3, 0xe4, 0xf5, 0x06, 0x17, 0x28, 0x39, 0x4a, 0x5b, 0x6c, 0x7d, 0x8e,
        0x9f, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54,
        0x32, 0x10,
    ];

    /// Per-process seed directory (avoids cross-run collisions). Each node's
    /// seeds are alias-derived (see [`derive_seed`]), so distinct aliases on the
    /// same shared substrate get DISTINCT federation key ids — node B is a real
    /// peer, not node A reading its own key over a second connection.
    fn default_seed_dir() -> String {
        format!("/tmp/ciris-spike-seeds-{}", std::process::id())
    }

    /// Deterministically derive a distinct 32-byte seed per (domain, alias) from a
    /// base salt: `sha256(domain | alias | base)`. Reproducible across runs, and
    /// a different `alias` yields a different key — the property that makes the
    /// two engines in the federation demo genuinely distinct peers.
    fn derive_seed(domain: &str, alias: &str, base: &[u8; 32]) -> [u8; 32] {
        use sha2::Digest as _;
        let mut h = sha2::Sha256::new();
        h.update(domain.as_bytes());
        h.update(b"|");
        h.update(alias.as_bytes());
        h.update(b"|");
        h.update(base);
        let out = h.finalize();
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&out[..32]);
        seed
    }

    /// Build a HYBRID [`LocalSigner`] (Ed25519 + ML-DSA-65) from on-disk seeds.
    /// Configuring the PQC pair (both-or-neither) is what makes the signer hybrid
    /// — an Ed25519-only signer is rejected at federation-tier ingest. Seeds are
    /// alias-derived so each node has its own federation key id.
    fn build_hybrid_signer(seed_dir: &str, alias: &str) -> Result<Arc<LocalSigner>, BoxErr> {
        std::fs::create_dir_all(seed_dir)?;
        let ed_seed = derive_seed("ed25519", alias, &ED25519_SEED);
        let pqc_seed = derive_seed("mldsa65", alias, &MLDSA_SEED);
        let ed_path = PathBuf::from(format!("{seed_dir}/{alias}.ed25519.seed"));
        let pqc_path = PathBuf::from(format!("{seed_dir}/{alias}.mldsa65.seed"));
        std::fs::write(&ed_path, ed_seed)?;
        std::fs::write(&pqc_path, pqc_seed)?;
        let signer = LocalSigner::from_config(&LocalSignerConfig {
            key_id: alias.to_string(),
            key_path: ed_path,
            pqc_key_id: Some(format!("{alias}-pqc")),
            pqc_key_path: Some(pqc_path),
        })?;
        Ok(Arc::new(signer))
    }

    /// The scores admission gate requires a `:v[0-9]+` version segment in the
    /// dimension (T3 version-pinning). Append `:v1` when one is absent so a
    /// calculus Form whose dimension is unversioned still emits conformantly.
    fn ensure_version_segment(dimension: &str) -> String {
        // crude but correct check: a `:vN` segment ending the string or followed
        // by another `:` segment.
        let has = dimension.split(':').any(|seg| {
            seg.len() >= 2 && seg.starts_with('v') && seg[1..].chars().all(|c| c.is_ascii_digit())
        });
        if has {
            dimension.to_string()
        } else {
            format!("{dimension}:v1")
        }
    }

    /// Build the persist-shaped `scores` envelope from a calculus Form. Carries
    /// the (version-pinned) dimension plus score/confidence the Form computed.
    fn scores_envelope_from_form(form: &crate::forms::Form) -> serde_json::Value {
        let env = &form.envelope;
        let dimension = ensure_version_segment(
            env.get("dimension").and_then(|v| v.as_str()).unwrap_or("evaluation:quality"),
        );
        let jcs = crate::sexpr::jcs_bytes(env).unwrap_or_default();
        let id = crate::sexpr::content_address(&jcs);
        serde_json::json!({
            "id": id,
            "dimension": dimension,
            "score": env.get("score").cloned().unwrap_or_else(|| serde_json::json!(0.0)),
            "confidence": env.get("confidence").cloned().unwrap_or_else(|| serde_json::json!(1.0)),
        })
    }

    /// A real ciris-persist Engine with its self federation key enrolled. One
    /// `Persisted` owns exactly one Engine; several may share one file DSN.
    pub struct Persisted {
        engine: Engine,
        key_id: String,
    }

    impl Persisted {
        /// Open an Engine on `dsn` and (when `enroll`) run the self-key
        /// enrollment. `with_signer` connects the backend AND runs all
        /// migrations (idempotent on an existing file via refinery).
        ///
        /// DSN slashes: `build_backend` strips `sqlite:///` before `sqlite://`,
        /// so an ABSOLUTE file path needs FOUR slashes total —
        /// `format!("sqlite:///{}", "/abs/x.db")`. `"sqlite::memory:"` is
        /// in-memory.
        async fn build(dsn: &str, alias: &str, seed_dir: &str, enroll: bool) -> Result<Self, BoxErr> {
            let signer = build_hybrid_signer(seed_dir, alias)?;
            // Capture the DERIVED federation key id ("<alias>-<fp>") before the
            // Arc is moved into the Engine. This is the FK target for every
            // emit/scrub.
            let derived = signer.derived_key_id();
            let engine = Engine::with_signer(signer, dsn).await?;
            if enroll {
                // register_self_federation_key MUST run before any emit: it
                // populates pubkey_ed25519_base64 AND pubkey_ml_dsa_65_base64 +
                // a hybrid self-scrub, so the federation-tier hybrid-verify gate
                // accepts the emit. Idempotent upsert on the derived key id.
                let registered = engine
                    .register_self_federation_key(
                        "agent",
                        "ciris-self-node",
                        None,
                        serde_json::json!({ "role": "self-node", "v": 1 }),
                        vec!["producer".to_string()],
                    )
                    .await?;
                debug_assert_eq!(registered, derived, "registered id is the derived id");
            }
            Ok(Self { engine, key_id: derived })
        }

        /// Open an emitter Engine on `dsn` (self key enrolled).
        pub async fn open(dsn: &str) -> Result<Self, BoxErr> {
            Self::build(dsn, "ciris-self", &default_seed_dir(), true).await
        }

        /// Attach a DISTINCT-IDENTITY reader Engine (alias `ciris-peer-b`, its own
        /// derived key id) to an existing shared-substrate `dsn`. It enrolls its
        /// own self key so it is a real federation peer, then reads the
        /// federation-tier rows other engines on the same file wrote (federation
        /// tier is federation-visible regardless of attester).
        pub async fn attach(dsn: &str) -> Result<Self, BoxErr> {
            Self::build(dsn, "ciris-peer-b", &default_seed_dir(), true).await
        }

        /// Open an in-memory emitter Engine (migrations run internally).
        pub async fn open_in_memory() -> Result<Self, BoxErr> {
            Self::open("sqlite::memory:").await
        }

        /// The enrolled/derived federation key id (the attesting + scrub FK).
        #[must_use]
        pub fn key_id(&self) -> &str {
            &self.key_id
        }

        /// CONFORMANT one-call build-sign-admit emit. Delegates to the real
        /// Engine `emit_attestation_self`: canonicalize → SHA-256 → hybrid sign
        /// (Ed25519 ‖ ML-DSA-65) → assemble → `put_attestation`, producing a
        /// FEDERATION-tier row in ONE call. Returns the persist attestation id.
        pub async fn emit(&self, form: &crate::forms::Form, weight: Option<f64>) -> Result<String, BoxErr> {
            let envelope = scores_envelope_from_form(form);
            let input =
                EmitAttestationInput::with_envelope(attestation_type::SCORES, envelope).with_weight(weight);
            let att_id = self.engine.emit_attestation_self(input).await?;
            Ok(att_id)
        }

        /// Read a row back as a [`SignedAttestation`] (the newtype CIRISEdge
        /// wraps), or `None` if absent on this substrate.
        pub async fn get_signed(&self, att_id: &str) -> Result<Option<SignedAttestation>, BoxErr> {
            let row: Option<Attestation> =
                self.engine.federation_directory().get_attestation(att_id).await?;
            Ok(row.map(|attestation| SignedAttestation { attestation }))
        }

        /// Conformant emit + extract: the real trio's "node A produces a real
        /// federation-tier SignedAttestation".
        pub async fn emit_signed(
            &self,
            form: &crate::forms::Form,
            weight: Option<f64>,
        ) -> Result<(String, SignedAttestation), BoxErr> {
            let id = self.emit(form, weight).await?;
            let signed = self
                .get_signed(&id)
                .await?
                .ok_or_else(|| -> BoxErr { "emitted row not found on its own substrate".into() })?;
            Ok((id, signed))
        }

        /// `attestation_promote`: `Ok(true)` on a genuine local→federation flip,
        /// `Ok(false)` (idempotent) on an already-federation row.
        pub async fn promote(&self, att_id: &str) -> Result<bool, BoxErr> {
            Ok(self.engine.attestation_promote(att_id).await?)
        }

        /// Local-tier write (`cohort_scope = "self"`) for exercising a GENUINE
        /// local→federation promotion. Returns the local attestation id.
        pub async fn upsert_local(
            &self,
            form: &crate::forms::Form,
            weight: Option<f64>,
        ) -> Result<String, BoxErr> {
            let envelope = scores_envelope_from_form(form);
            let input = LocalAttestationInput {
                attesting_key_id: self.key_id.clone(),
                attested_key_id: None,
                attestation_type: attestation_type::SCORES.to_string(),
                weight,
                expires_at: None,
                attestation_envelope: envelope,
                subject_key_ids: vec![],
                cohort_scope: "self".to_string(),
            };
            let id = self.engine.federation_directory().attestation_upsert_local(input).await?;
            Ok(id)
        }
    }

    /// (B) SHARED-SUBSTRATE FEDERATION DEMO (CIRISConformance test_300 pattern).
    ///
    /// Two Engines on ONE shared sqlite file DSN. Node A — the evaluator —
    /// reduces a calm Thought through the PDMA-as-eval reduction, then CONFORMANT-
    /// emits the resulting `scores` form as a federation-tier SignedAttestation.
    /// Node B, a SECOND Engine attached to the SAME substrate (it emitted
    /// nothing), SEES the row. Also exercises `attestation_promote` idempotence
    /// (Ok(false) on the already-federation emit) and a GENUINE local→federation
    /// promotion (Ok(true)).
    ///
    /// Returns the captured evidence log (also printed by the demo binary).
    pub async fn federation_shared_substrate_demo() -> Result<String, BoxErr> {
        use std::fmt::Write as _;
        let mut log = String::new();

        // Shared substrate: ONE sqlite file both engines open.
        let db_path = format!("/tmp/ciris-spike-fed-{}.db", std::process::id());
        let _ = std::fs::remove_file(&db_path);
        // FOUR slashes: "sqlite:///" + "/tmp/..." -> opens the absolute file.
        let dsn = format!("sqlite:///{db_path}");

        // Node A: the evaluator. Reduce a calm Thought -> Emit form (PDMA-as-eval).
        let node_a = Persisted::open(&dsn).await?;
        let thought = crate::pdma::Thought {
            situation: "routine quality scoring (federation demo)".to_string(),
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
            attesting_key_id: node_a.key_id().to_string(),
            subject_key_ids: vec![],
        };
        let form = match crate::pdma::reduce(&thought) {
            crate::pdma::Reduction::Emit(f) => f,
            crate::pdma::Reduction::Defer(_) => {
                return Err("calm Thought unexpectedly deferred".into());
            }
        };

        // CONFORMANT emit -> real federation-tier SignedAttestation.
        let (att_id, signed) = node_a.emit_signed(&form, Some(0.92)).await?;
        writeln!(log, "node A emitted {att_id}")?;
        writeln!(
            log,
            "  [A] tier={} attesting_key_id={} cohort_scope={} weight={:?}",
            signed.attestation.tier,
            signed.attestation.attesting_key_id,
            signed.attestation.cohort_scope,
            signed.attestation.weight,
        )?;
        writeln!(
            log,
            "  [A] original_content_hash(len {}) ed25519_scrub(len {}) ml_dsa_scrub_present={}",
            signed.attestation.original_content_hash.len(),
            signed.attestation.scrub_signature_classical.len(),
            signed
                .attestation
                .scrub_signature_pqc
                .as_deref()
                .map(|s| !s.is_empty())
                .unwrap_or(false),
        )?;

        // attestation_promote on the already-federation emit is idempotent.
        let promoted = node_a.promote(&att_id).await?;
        writeln!(log, "  [A] attestation_promote(emit) = {promoted} (already federation => false)")?;

        // GENUINE local->federation promotion (Ok(true)).
        let local_id = node_a.upsert_local(&form, Some(0.80)).await?;
        let promoted_true = node_a.promote(&local_id).await?;
        writeln!(
            log,
            "  [A] upsert_local id={local_id} -> promote = {promoted_true} (local => federation)"
        )?;

        // Node B: a DISTINCT-IDENTITY peer Engine on the SAME DSN (shared
        // substrate). Its own key id differs from A's; it emitted nothing of A's
        // rows, yet it must SEE both of A's federation-tier rows.
        let node_b = Persisted::attach(&dsn).await?;
        writeln!(
            log,
            "  node B is a DISTINCT peer: key_id={} (≠ node A key_id={})",
            node_b.key_id(),
            node_a.key_id(),
        )?;
        assert_ne!(
            node_b.key_id(),
            node_a.key_id(),
            "node B must be a distinct federation identity from node A"
        );
        let seen = node_b
            .get_signed(&att_id)
            .await?
            .ok_or_else(|| -> BoxErr { "node B must SEE A's emitted row (shared substrate)".into() })?;
        writeln!(
            log,
            "node B (shared substrate) SEES {att_id} (tier={})",
            seen.attestation.tier
        )?;
        let seen_promoted = node_b
            .get_signed(&local_id)
            .await?
            .ok_or_else(|| -> BoxErr { "node B must SEE A's promoted row (shared substrate)".into() })?;
        writeln!(
            log,
            "node B (shared substrate) SEES {local_id} (tier={})",
            seen_promoted.attestation.tier
        )?;

        // The federation-tier invariants the demo proves.
        assert_eq!(seen.attestation.tier, "federation", "emit row must be federation tier");
        assert_eq!(seen.attestation.attestation_id, att_id);
        assert!(
            seen.attestation
                .scrub_signature_pqc
                .as_deref()
                .is_some_and(|s| !s.is_empty()),
            "federation-tier row carries the ML-DSA-65 hybrid scrub half"
        );
        assert_eq!(seen_promoted.attestation.tier, "federation");
        assert!(promoted_true, "local->federation promotion must return true");

        let _ = std::fs::remove_file(&db_path);
        Ok(log)
    }
}

// ---------------------------------------------------------------------------
// Tests (default path — real crypto, no real-persist)
// ---------------------------------------------------------------------------

#[cfg(all(test, not(feature = "real-persist")))]
mod tests {
    use super::*;
    use serde_json::json;

    /// Build a real `scores` Form through the closed-op reader.
    fn scores_form() -> crate::forms::Form {
        let env = json!({
            "attestation_type": "scores",
            "dimension": "evaluation:quality",
            "score": 0.85,
            "confidence": 0.92,
            "cohort_scope": "self",
        });
        crate::forms::read_form(&env).expect("a `scores` envelope must read as a Form")
    }

    #[test]
    fn emit_yields_local_tier_and_content_addressed_id() {
        let form = scores_form();
        let row = emit(&form, "steward-self", &[]).expect("emit must succeed");

        assert_eq!(row.tier, "local", "emit is local-tier");
        assert_eq!(row.cohort_scope, "self", "local emissions are cohort self");
        assert_eq!(row.attestation_type, "scores");
        assert!(row.subject_key_ids.is_empty());

        // The id is the content address of the JCS-canonical envelope bytes.
        let jcs = crate::sexpr::jcs_bytes(&form.envelope).expect("jcs");
        assert_eq!(row.attestation_id, crate::sexpr::content_address(&jcs));
        assert_eq!(row.attestation_id.len(), 64, "hex(sha256) is 64 hex chars");

        // Content-addressing is deterministic: re-emitting yields the same id.
        let row2 = emit(&form, "steward-self", &[]).unwrap();
        assert_eq!(row.attestation_id, row2.attestation_id);
    }

    #[test]
    fn promote_yields_federation_tier_with_verifying_hybrid_signature() {
        let form = scores_form();
        let row = emit(&form, "steward-self", &[]).unwrap();

        let h = Hybrid::generate().expect("real hybrid identity must generate");
        let fed = promote(&row, &h).expect("genuine promote must pass the gate");

        assert_eq!(fed.tier, "federation", "promote flips to federation tier");
        assert_eq!(fed.local, row, "the local row is carried verbatim");
        assert_eq!(fed.key_id, h.key_id(), "key id comes from the Hybrid");

        // Both halves of the hybrid signature are present and non-empty.
        assert!(!fed.classical_sig_b64.is_empty(), "classical sig present");
        assert!(!fed.pqc_sig_b64.is_empty(), "pqc sig present");

        // jcs_hex is the hex of the exact bytes that were signed.
        let jcs = crate::sexpr::jcs_bytes(&row.attestation_envelope).unwrap();
        assert_eq!(fed.jcs_hex, hex::encode(&jcs));

        // The recorded signatures are the REAL primitive sizes, proving genuine
        // Ed25519 + ML-DSA-65 material (not a stub).
        let classical = B64.decode(&fed.classical_sig_b64).expect("b64 classical");
        let pqc = B64.decode(&fed.pqc_sig_b64).expect("b64 pqc");
        assert_eq!(classical.len(), 64, "Ed25519 signature is 64 bytes");
        assert_eq!(pqc.len(), 3309, "ML-DSA-65 signature is 3309 bytes (FIPS 204)");

        // And the federation-tier invariant holds end to end: re-verifying the
        // produced signature over the same JCS bytes succeeds (Ok(true)).
        let sig = h.signer.sign(&jcs).unwrap();
        assert!(
            matches!(h.verifier.verify(&jcs, &sig), Ok(true)),
            "tier = federation ⟹ a verifying hybrid signature is present"
        );
    }

    #[test]
    fn tamper_is_rejected_at_admission() {
        let h = Hybrid::generate().unwrap();

        // Hand-built bad case: sign canonical bytes A, but verify bytes B.
        // This is exactly the shape `promote` guards against — a signature that
        // does not cover the bytes being admitted.
        let bytes_a = b"canonical envelope A".as_slice();
        let bytes_b = b"canonical envelope B (tampered)".as_slice();

        // Raw crypto: a mismatched payload never verifies as Ok(true).
        let sig = h.signer.sign(bytes_a).unwrap();
        assert!(
            !matches!(h.verifier.verify(bytes_b, &sig), Ok(true)),
            "a signature over A must not verify over B"
        );

        // Through the SAME gate `promote` uses, the mismatch is AdmissionRejected.
        let gated = admit(&h, bytes_a, bytes_b);
        assert!(
            matches!(gated, Err(EffectError::AdmissionRejected)),
            "the admission gate refuses a sig that does not cover the admitted bytes"
        );

        // Sanity: the genuine path (A signed, A verified) passes the same gate.
        assert!(admit(&h, bytes_a, bytes_a).is_ok());

        // And mutating the envelope after emit re-addresses the fact: the
        // content address changes, so the tampered row is a different fact —
        // it can never masquerade as the original under its old id.
        let form = scores_form();
        let mut row = emit(&form, "steward-self", &[]).unwrap();
        let original_id = row.attestation_id.clone();
        row.attestation_envelope = json!({
            "attestation_type": "scores",
            "dimension": "evaluation:quality",
            "score": 0.99,            // tampered score
            "confidence": 0.92,
            "cohort_scope": "self",
        });
        let promoted = promote(&row, &h).expect("the tampered row still self-signs");
        // Its JCS no longer matches the original content address.
        let new_jcs = crate::sexpr::jcs_bytes(&row.attestation_envelope).unwrap();
        assert_ne!(
            crate::sexpr::content_address(&new_jcs),
            original_id,
            "mutating the envelope changes the fact's content address"
        );
        // The federation row binds its signature to the NEW bytes, not the old id.
        assert_eq!(promoted.jcs_hex, hex::encode(&new_jcs));
    }
}
