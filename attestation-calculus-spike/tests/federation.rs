//! (B) SHARED-SUBSTRATE FEDERATION — integration tests (feature = "real-persist").
//!
//! Two real `ciris-persist` Engines on ONE shared sqlite file DSN: the evaluator
//! node A CONFORMANT-emits a federation-tier `SignedAttestation` (via the Engine
//! one-call build-sign-admit), and node B — a SECOND Engine on the SAME
//! substrate — SEES it (CIRISConformance test_300 pattern). Also exercises the
//! conformant in-memory emit + extraction directly.
//!
//! The whole file is gated on `real-persist`; under the default build it
//! compiles to zero tests and references no persist crate.
#![cfg(feature = "real-persist")]

use attestation_calculus_spike::effect::persisted::{federation_shared_substrate_demo, Persisted};
use attestation_calculus_spike::{forms, pdma};

/// Build the `scores` Form a calm reduction emits (dimension version-pinned so
/// the conformant `scores` admission gate accepts it).
fn calm_scores_form() -> forms::Form {
    let thought = pdma::Thought {
        situation: "routine quality scoring".to_string(),
        candidate: pdma::CandidateAction {
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
        attesting_key_id: "steward-self".to_string(),
        subject_key_ids: vec![],
    };
    match pdma::reduce(&thought) {
        pdma::Reduction::Emit(f) => f,
        pdma::Reduction::Defer(_) => panic!("calm Thought must Emit"),
    }
}

/// The conformant Engine emit produces a REAL federation-tier SignedAttestation:
/// it is `tier = "federation"` and carries the ML-DSA-65 (hybrid) scrub half —
/// the substrate's own build-sign-admit, not a hand-rolled sign+write.
#[tokio::test]
async fn conformant_emit_produces_federation_tier_hybrid_signed_attestation() {
    let node = Persisted::open_in_memory().await.expect("open in-memory engine");
    let form = calm_scores_form();
    let (att_id, signed) = node.emit_signed(&form, Some(0.92)).await.expect("conformant emit");

    assert_eq!(signed.attestation.attestation_id, att_id);
    assert_eq!(signed.attestation.tier, "federation", "emit_attestation_self is federation tier");
    assert_eq!(signed.attestation.attestation_type, "scores");
    assert_eq!(signed.attestation.attesting_key_id, node.key_id(), "attester is the enrolled self key");

    // The federation-tier hybrid invariant: a non-empty ML-DSA-65 scrub half.
    assert!(
        signed
            .attestation
            .scrub_signature_pqc
            .as_deref()
            .is_some_and(|s| !s.is_empty()),
        "federation-tier row must carry the ML-DSA-65 hybrid scrub"
    );
    // SHA-256 content hash is 64 hex chars; classical scrub is present.
    assert_eq!(signed.attestation.original_content_hash.len(), 64);
    assert!(!signed.attestation.scrub_signature_classical.is_empty());

    // attestation_promote on an already-federation row is idempotent (false).
    let promoted = node.promote(&att_id).await.expect("promote emitted row");
    assert!(!promoted, "promoting an already-federation row returns Ok(false)");
}

/// A GENUINE local→federation promotion (Ok(true)) through `attestation_promote`.
#[tokio::test]
async fn genuine_local_to_federation_promotion_returns_true() {
    let node = Persisted::open_in_memory().await.expect("open in-memory engine");
    let form = calm_scores_form();
    let local_id = node.upsert_local(&form, Some(0.80)).await.expect("local upsert");
    let promoted = node.promote(&local_id).await.expect("promote local row");
    assert!(promoted, "local->federation promotion must return Ok(true)");

    let signed = node.get_signed(&local_id).await.expect("read").expect("row present");
    assert_eq!(signed.attestation.tier, "federation");
}

/// The headline (B) demo: two Engines, one shared file substrate; node A emits a
/// promoted attestation and node B SEES it. The demo asserts the federation-tier
/// invariants internally; here we additionally check its evidence log.
#[tokio::test]
async fn shared_substrate_node_b_sees_node_a_emit() {
    let log = federation_shared_substrate_demo().await.expect("federation shared-substrate demo");
    assert!(log.contains("node A emitted"), "evidence must report A's emit:\n{log}");
    assert!(
        log.contains("node B (shared substrate) SEES"),
        "evidence must report B's view of A's row:\n{log}"
    );
}
