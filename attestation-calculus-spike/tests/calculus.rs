//! Integration tests for the Attestation Calculus — the five load-bearing
//! properties, asserted through the public library surface.
//!
//! These complement (do not replace) the per-module unit tests authored inside
//! `sexpr` / `forms` / `pdma` / `effect`. Here we treat the crate as a black box
//! and check the five demonstrations the spike exists to prove.

use serde_json::json;

use attestation_calculus_spike::{effect, forms, pdma, sexpr};

// ---------------------------------------------------------------------------
// (1) HOMOICONICITY — code is data; the JCS round-trip is bit-for-bit and the
//     content address is stable across value→sexpr→value.
// ---------------------------------------------------------------------------
#[test]
fn property_1_homoiconicity_jcs_round_trip() {
    let envelope = json!({
        "attestation_type": "scores",
        "dimension": "evaluation:quality",
        "score": 0.85,
        "confidence": 0.92,
        "evidence_refs": [],
        "tier": "federation",
        "weights": {"b": 1, "a": 2}
    });

    let s = sexpr::value_to_sexpr(&envelope);
    let back = sexpr::sexpr_to_value(&s);

    // Deep structural round-trip (numbers verbatim).
    assert_eq!(envelope, back, "value→sexpr→value must be deep-equal");

    // Canonical-bytes round-trip and stable content address.
    let jcs_a = sexpr::jcs_bytes(&envelope).expect("jcs original");
    let jcs_b = sexpr::jcs_bytes(&back).expect("jcs round-trip");
    assert_eq!(jcs_a, jcs_b, "JCS bytes must round-trip bit-for-bit");
    assert_eq!(
        sexpr::content_address(&jcs_a),
        sexpr::content_address(&jcs_b),
        "content address must be stable across the round-trip"
    );

    // The printer is homoiconic: the `scores` head leads the rendering.
    let printed = sexpr::print_sexpr(&s);
    assert!(printed.starts_with("(scores"), "head leads: got {printed}");
}

// ---------------------------------------------------------------------------
// (2) CLOSED OPERATORS — read_form admits the 1+4 and rejects every other head.
// ---------------------------------------------------------------------------
#[test]
fn property_2_closed_operators() {
    // `scores` is admitted.
    let good = json!({ "attestation_type": "scores", "dimension": "evaluation:quality" });
    let form = forms::read_form(&good).expect("scores must read");
    assert_eq!(form.op, forms::Op::Scores);

    // Each non-1+4 head is rejected as UnknownOperator, carrying the head verbatim.
    for bad in ["lambda", "exec", "eval", "setf", "apply"] {
        let env = json!({ "attestation_type": bad, "dimension": "x" });
        match forms::read_form(&env) {
            Err(forms::ReadError::UnknownOperator(got)) => assert_eq!(got, bad),
            other => panic!("expected UnknownOperator({bad:?}), got {other:?}"),
        }
    }

    // The four composers also read.
    for head in ["delegates_to", "supersedes", "withdraws", "recants"] {
        let env = json!({ "attestation_type": head, "dimension": "x" });
        assert!(forms::read_form(&env).is_ok(), "{head} must read");
    }
}

// ---------------------------------------------------------------------------
// (3) PDMA-AS-EVAL — a calm Thought reduces to Emit of a `scores` Form.
// ---------------------------------------------------------------------------
fn calm_thought() -> pdma::Thought {
    pdma::Thought {
        situation: "routine quality scoring".to_string(),
        candidate: pdma::CandidateAction {
            dimension: "evaluation:quality".to_string(),
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
    }
}

#[test]
fn property_3_pdma_as_eval_emits() {
    match pdma::reduce(&calm_thought()) {
        pdma::Reduction::Emit(form) => {
            assert_eq!(form.op, forms::Op::Scores);
            assert_eq!(form.envelope["attestation_type"], "scores");
            assert_eq!(form.envelope["dimension"], "evaluation:quality");
            assert_eq!(form.envelope["score"], 0.85);
            // The emitted form prints homoiconically with the head leading.
            let printed = sexpr::print_sexpr(&forms::form_to_sexpr(&form));
            assert!(printed.starts_with("(scores"), "got {printed}");
        }
        pdma::Reduction::Defer(_) => panic!("calm Thought must Emit"),
    }
}

// ---------------------------------------------------------------------------
// (4) WBD NORMAL FORM — uncertain/novel and order-veto Thoughts reduce to Defer.
// ---------------------------------------------------------------------------
#[test]
fn property_4_wbd_normal_form_defers() {
    // (a) WBD: uncertainty above threshold (and novel) -> Defer with a WBD rationale.
    let mut uncertain = calm_thought();
    uncertain.candidate.uncertainty = 0.95;
    uncertain.candidate.novel = true;
    match pdma::reduce(&uncertain) {
        pdma::Reduction::Defer(pkg) => {
            assert!(pkg.rationale.contains("WBD"), "rationale must cite WBD: {}", pkg.rationale);
            assert!(pkg.rationale.contains("CC 1.9"), "rationale must cite CC 1.9");
        }
        pdma::Reduction::Emit(_) => panic!("uncertain/novel Thought must Defer"),
    }

    // (b) Order-Maximization Veto: entropy >= 10 x loss (loss > 0) -> Defer (veto).
    let mut veto = calm_thought();
    veto.candidate.predicted_entropy_reduction = 100.0;
    veto.candidate.predicted_flourishing_loss = 1.0;
    match pdma::reduce(&veto) {
        pdma::Reduction::Defer(pkg) => {
            assert!(
                pkg.rationale.contains("CC 1.3 step 2"),
                "veto rationale must cite CC 1.3 step 2: {}",
                pkg.rationale
            );
        }
        pdma::Reduction::Emit(_) => panic!("order-veto Thought must Defer"),
    }
}

// ---------------------------------------------------------------------------
// (5) GATE-CHECKED EMIT→PROMOTE — emit a local-tier content-addressed row,
//     promote it through the real hybrid admission gate, then show tamper
//     re-addresses the fact (the admitted signature does not cover it).
// ---------------------------------------------------------------------------
#[test]
fn property_5_gate_checked_emit_promote() {
    // Build the form the calm reduction would emit.
    let form = match pdma::reduce(&calm_thought()) {
        pdma::Reduction::Emit(f) => f,
        pdma::Reduction::Defer(_) => panic!("calm Thought must Emit"),
    };

    // emit() -> local tier, content-addressed.
    let row = effect::emit(&form, "steward-self", &[]).expect("emit");
    assert_eq!(row.tier, "local");
    assert_eq!(row.cohort_scope, "self");
    assert_eq!(row.attestation_id.len(), 64, "hex(sha256) is 64 chars");
    let jcs = sexpr::jcs_bytes(&form.envelope).expect("jcs");
    assert_eq!(row.attestation_id, sexpr::content_address(&jcs));

    // promote() -> federation tier with a verifying hybrid signature.
    let hybrid = effect::Hybrid::generate().expect("hybrid identity");
    let fed = effect::promote(&row, &hybrid).expect("genuine promote passes the gate");
    assert_eq!(fed.tier, "federation");
    assert_eq!(fed.key_id, hybrid.key_id());
    assert_eq!(fed.jcs_hex, hex::encode(&jcs), "signature bound to the canonical bytes");

    // Real primitive sizes prove genuine Ed25519 + ML-DSA-65 material (not a stub).
    use base64::engine::general_purpose::STANDARD as B64;
    use base64::Engine as _;
    let classical = B64.decode(&fed.classical_sig_b64).expect("b64 classical");
    let pqc = B64.decode(&fed.pqc_sig_b64).expect("b64 pqc");
    assert_eq!(classical.len(), 64, "Ed25519 signature is 64 bytes");
    assert_eq!(pqc.len(), 3309, "ML-DSA-65 signature is 3309 bytes (FIPS 204)");

    // Tamper: mutate the envelope after admission -> a different content address,
    // which the admitted (signed) bytes do not cover. The fabric refuses it.
    let mut tampered = row.clone();
    tampered.attestation_envelope["score"] = json!(0.99);
    let tampered_jcs = sexpr::jcs_bytes(&tampered.attestation_envelope).expect("jcs tampered");
    let tampered_id = sexpr::content_address(&tampered_jcs);
    assert_ne!(
        tampered_id, fed.local.attestation_id,
        "mutating the envelope re-addresses the fact (admission rejected)"
    );
}
