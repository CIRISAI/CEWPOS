//! (C) REAL EDGE WIRE PATH — integration test (feature = "real-edge").
//!
//! The full real trio: node A CONFORMANT-emits a real federation-tier
//! `SignedAttestation` via the persist Engine, then ships that exact attestation
//! A→B over a real `ciris-edge` transport-http wire, where B's inbound pipeline
//! verifies the wrapping envelope and republishes it on `subscribe_verified_feed`.
//!
//! Gated on `real-edge` (implies `real-persist`); zero tests under the default /
//! `real-persist` builds, and references no `ciris-edge` crate there.
#![cfg(feature = "real-edge")]

use attestation_calculus_spike::transport;

/// End-to-end: a real conformant attestation crosses a real HTTP wire and is
/// verified by node B's inbound pipeline. The demo asserts envelope identity and
/// inner-attestation-id survival internally; here we re-check the evidence log.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn promoted_attestation_crosses_real_edge_wire_and_verifies_on_b() {
    let log = transport::wire_demo().await.expect("real-edge wire demo");
    assert!(log.contains("node A emitted"), "evidence must report A's conformant emit:\n{log}");
    assert!(
        log.contains("node B RECEIVED+VERIFIED"),
        "evidence must report B receiving+verifying over ciris-edge:\n{log}"
    );
    assert!(log.contains("over ciris-edge"), "evidence must name the ciris-edge medium:\n{log}");
}

/// Directly exercise `run_wire_transfer` with a conformant attestation and assert
/// on the structured `WireOutcome` (envelope routed node-a -> node-b; inner
/// attestation is the federation-tier `scores` row A emitted).
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn wire_outcome_carries_envelope_and_inner_identity() {
    use attestation_calculus_spike::effect::persisted::Persisted;
    use attestation_calculus_spike::pdma;

    // Conformant emit on node A's substrate.
    let producer = Persisted::open_in_memory().await.expect("persist engine");
    let thought = pdma::Thought {
        situation: "wire outcome test".to_string(),
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
        attesting_key_id: producer.key_id().to_string(),
        subject_key_ids: vec![],
    };
    let form = match pdma::reduce(&thought) {
        pdma::Reduction::Emit(f) => f,
        pdma::Reduction::Defer(_) => panic!("calm Thought must Emit"),
    };
    let (att_id, signed) = producer.emit_signed(&form, Some(0.92)).await.expect("conformant emit");

    let outcome = transport::run_wire_transfer(signed).await.expect("wire transfer");
    assert_eq!(outcome.received_attestation_id, att_id, "B receives A's exact attestation id");
    assert_eq!(outcome.envelope_signing_key_id, "node-a");
    assert_eq!(outcome.envelope_destination_key_id, "node-b");
    assert_eq!(outcome.inner_attestation_type, "scores");
    assert_eq!(outcome.inner_tier, "federation");
}
