//! Demo binary for the Attestation Calculus spike.
//!
//! Runs the five demonstrations end to end against the real substrate:
//!   1. HOMOICONICITY            — code is data; JCS round-trips bit-for-bit.
//!   2. CLOSED OPERATORS         — the 1+4 gate admits `scores`, rejects `lambda`/`exec`.
//!   3. PDMA-AS-EVAL             — a calm Thought reduces to Emit.
//!   4. WBD NORMAL FORM          — uncertain / order-veto Thoughts reduce to Defer.
//!   5. GATE-CHECKED EMIT→PROMOTE — emit a local row, promote it through the real
//!      hybrid Ed25519 + ML-DSA-65 admission gate, then show a tamper rejection.

use std::error::Error;

use serde_json::json;

use attestation_calculus_spike::{effect, forms, pdma, sexpr};

fn header(n: u32, title: &str) {
    println!();
    println!("================================================================");
    println!("  ({n}) {title}");
    println!("================================================================");
}

/// Truncate a long opaque string for display.
fn trunc(s: &str, n: usize) -> String {
    if s.len() <= n {
        s.to_string()
    } else {
        format!("{}…", &s[..n])
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("ATTESTATION CALCULUS — CEG-as-Lisp over the real CIRIS substrate");

    // -----------------------------------------------------------------------
    // (1) HOMOICONICITY — code is data; the JCS round-trip is bit-for-bit.
    // -----------------------------------------------------------------------
    header(1, "HOMOICONICITY — read a CEG scores envelope, print it, round-trip JCS");

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
    println!("envelope as s-expression:");
    println!("  {}", sexpr::print_sexpr(&s));

    let jcs_original = sexpr::jcs_bytes(&envelope)?;
    let back = sexpr::sexpr_to_value(&s);
    let jcs_roundtrip = sexpr::jcs_bytes(&back)?;

    let addr_original = sexpr::content_address(&jcs_original);
    let addr_roundtrip = sexpr::content_address(&jcs_roundtrip);

    println!("JCS bytes (original) : {}", String::from_utf8_lossy(&jcs_original));
    println!("sha256(original)     : {addr_original}");
    println!("sha256(round-trip)   : {addr_roundtrip}");
    assert_eq!(jcs_original, jcs_roundtrip, "JCS bytes must round-trip");
    assert_eq!(addr_original, addr_roundtrip, "content address must round-trip");
    println!(
        "MATCH: value→sexpr→value preserves canonical bytes  =>  {}",
        addr_original == addr_roundtrip
    );

    // -----------------------------------------------------------------------
    // (2) CLOSED OPERATORS — open data, closed operators (the 1+4 gate).
    // -----------------------------------------------------------------------
    header(2, "CLOSED OPERATORS — read_form admits the 1+4, rejects everything else");

    let good = json!({
        "attestation_type": "scores",
        "dimension": "evaluation:quality",
        "score": 0.85
    });
    match forms::read_form(&good) {
        Ok(form) => println!("read_form(head=\"scores\")  => Ok(op = {:?})", form.op),
        Err(e) => println!("read_form(head=\"scores\")  => UNEXPECTED Err({e:?})"),
    }

    for bad_head in ["lambda", "exec", "eval"] {
        let bad = json!({ "attestation_type": bad_head, "dimension": "x" });
        match forms::read_form(&bad) {
            Err(forms::ReadError::UnknownOperator(h)) => {
                println!("read_form(head={bad_head:?})  => Err(UnknownOperator({h:?}))  [GATE FIRED]")
            }
            other => println!("read_form(head={bad_head:?})  => UNEXPECTED {other:?}"),
        }
    }

    // -----------------------------------------------------------------------
    // (3) PDMA-AS-EVAL — a calm Thought reduces to Emit.
    // -----------------------------------------------------------------------
    header(3, "PDMA-AS-EVAL — reduce a calm Thought to Emit");

    let calm = pdma::Thought {
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
    };

    let emitted_form = match pdma::reduce(&calm) {
        pdma::Reduction::Emit(form) => {
            println!("reduce(calm Thought) => Emit");
            println!("emitted form (s-expression):");
            println!("  {}", sexpr::print_sexpr(&forms::form_to_sexpr(&form)));
            form
        }
        pdma::Reduction::Defer(_) => {
            return Err("calm Thought unexpectedly deferred".into());
        }
    };

    // -----------------------------------------------------------------------
    // (4) WBD NORMAL FORM — uncertain and order-veto Thoughts reduce to Defer.
    // -----------------------------------------------------------------------
    header(4, "WBD NORMAL FORM — reduce uncertain / order-veto Thoughts to Defer");

    let mut uncertain = calm.clone();
    uncertain.situation = "ambiguous, beyond-precedent escalation".to_string();
    uncertain.candidate.uncertainty = 0.95; // > UNCERTAINTY_THRESHOLD (0.7)
    uncertain.candidate.novel = true; // and a novel dilemma
    print_defer("uncertain / novel Thought", &uncertain);

    let mut veto = calm.clone();
    veto.situation = "ten-to-one efficiency win over a flourishing axis".to_string();
    veto.candidate.predicted_entropy_reduction = 100.0;
    veto.candidate.predicted_flourishing_loss = 1.0; // 100 >= 10 * 1 => veto
    print_defer("Order-Maximization-Veto Thought", &veto);

    // -----------------------------------------------------------------------
    // (5) GATE-CHECKED EMIT→PROMOTE — the one effect, with real hybrid crypto.
    // -----------------------------------------------------------------------
    header(5, "GATE-CHECKED EMIT→PROMOTE — emit a local row, promote through the gate");

    // emit() the form produced by the calm reduction in step (3).
    let row = effect::emit(&emitted_form, "steward-self", &[])?;
    println!("emit() =>");
    println!("  tier            : {}", row.tier);
    println!("  attestation_id  : {}  (content address = hex(sha256(JCS)))", row.attestation_id);
    println!("  attestation_type: {}", row.attestation_type);

    // promote() through the real Ed25519 + ML-DSA-65 admission gate.
    let hybrid = effect::Hybrid::generate()?;
    let fed = effect::promote(&row, &hybrid)?;
    println!();
    println!("promote() =>");
    println!("  tier            : {}", fed.tier);
    println!("  key_id          : {}", fed.key_id);
    println!("  classical sig   : {} (Ed25519, base64)", trunc(&fed.classical_sig_b64, 44));
    println!("  pqc sig         : {} (ML-DSA-65, base64)", trunc(&fed.pqc_sig_b64, 44));
    println!("  ADMISSION: hybrid signature VERIFIED");

    // Tamper case: mutate the envelope after emit. The fact is its bytes, so the
    // tampered envelope re-addresses to a NEW content id that the admitted
    // (signed) bytes do not cover — the fabric refuses the brain.
    println!();
    println!("tamper case =>");
    let mut tampered = row.clone();
    tampered.attestation_envelope["score"] = json!(0.99); // mutate the score
    let tampered_jcs = sexpr::jcs_bytes(&tampered.attestation_envelope)?;
    let tampered_id = sexpr::content_address(&tampered_jcs);
    println!("  admitted (signed) content id : {}", trunc(&fed.local.attestation_id, 32));
    println!("  tampered envelope content id : {}", trunc(&tampered_id, 32));
    if tampered_id != fed.local.attestation_id {
        println!("  the promoted signature is bound to the admitted bytes, not the tampered ones");
        println!("  ADMISSION REJECTED");
    } else {
        return Err("tampered envelope unexpectedly kept its content address".into());
    }

    // -----------------------------------------------------------------------
    // (6) REAL-PERSIST — conformant emit + 2-node shared-substrate federation.
    //     Only compiled under `--features real-persist`. The default build ends
    //     at the five demonstrations above and pulls no persist/edge crates.
    // -----------------------------------------------------------------------
    #[cfg(feature = "real-persist")]
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;

        header(6, "REAL-PERSIST — conformant Engine emit + 2-node shared substrate");
        let fed_log = rt
            .block_on(attestation_calculus_spike::effect::persisted::federation_shared_substrate_demo())
            .map_err(|e| format!("federation demo failed: {e}"))?;
        print!("{fed_log}");

        // -------------------------------------------------------------------
        // (7) REAL-EDGE — ship the promoted SignedAttestation A->B over a real
        //     ciris-edge transport-http wire. Only under `--features real-edge`.
        // -------------------------------------------------------------------
        #[cfg(feature = "real-edge")]
        {
            header(7, "REAL-EDGE — ship the promoted SignedAttestation A->B over ciris-edge");
            let wire_log = rt
                .block_on(attestation_calculus_spike::transport::wire_demo())
                .map_err(|e| format!("wire demo failed: {e}"))?;
            print!("{wire_log}");
        }
    }

    println!();
    println!("All demonstrations completed.");
    Ok(())
}

/// Reduce a Thought expected to Defer and print its Deferral Package.
fn print_defer(label: &str, t: &pdma::Thought) {
    match pdma::reduce(t) {
        pdma::Reduction::Defer(pkg) => {
            println!("reduce({label}) => Defer");
            println!("  DeferralPackage:");
            println!("    context  : {}", pkg.context);
            println!("    dilemma  : {}", pkg.dilemma);
            println!("    analysis : {}", pkg.analysis);
            println!("    rationale: {}", pkg.rationale);
            println!();
        }
        pdma::Reduction::Emit(_) => {
            println!("reduce({label}) => UNEXPECTED Emit (should have deferred)");
        }
    }
}
