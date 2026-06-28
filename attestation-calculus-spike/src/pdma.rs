//! PDMA reduction = the evaluator.
//!
//! This module is the operational core of the Attestation Calculus: the CIRIS
//! Principled Decision-Making Algorithm (PDMA) plus Wisdom-Based Deferral (WBD),
//! encoded as the *reduction semantics* of a total, effect-typed Lisp over
//! immutable signed forms. "The ethics is the evaluator" — M-1 (CC 1.1) is not a
//! layer bolted above the runtime; it *is* the reduction rule.
//!
//! A `Thought` reduces to exactly one of two terminal values:
//!
//!   * [`Reduction::Emit`] — the candidate action is selected and executed
//!     (PDMA step 5 / CC 1.3 step 5). It is built into a canonical `scores`
//!     envelope and read through the closed-operator gate ([`crate::forms::read_form`]).
//!   * [`Reduction::Defer`] — the action is halted and a concise Deferral Package
//!     is escalated to Wise Authorities (CC 1.9). Defer is the *safe* terminal
//!     state; there is no third outcome.
//!
//! `reduce` is a short-circuiting gate pipeline whose ordering is load-bearing
//! and fixed (CC 1.8 wraps the whole PDMA in WBD):
//!
//!   1. **Wisdom-Based Deferral triggers** (CC 1.9 / CC 1.16.4) — the outermost
//!      humility guard; runs before any alignment arithmetic is trusted.
//!   2. **Order-Maximization Veto** (CC 1.3 step 2 / CC 1.16.3 step 2) — the
//!      "order must carry life" guard; runs only on thoughts that cleared WBD.
//!   3. **Emit** (CC 1.3 step 5) — the fall-through default, reachable only when
//!      both gates pass.
//!
//! The first gate to fire yields `Defer`. If no gate fires, the reduction falls
//! through to `Emit`.

/// A candidate action under evaluation. Its fields are produced by the
/// surrounding PDMA pipeline (Contextualisation, Conflict Identification /
/// Resolution — CC 1.3 steps 1,3,4) and consumed by the two-valued reduction.
#[derive(Debug, Clone)]
pub struct CandidateAction {
    /// The CEG dimension being scored, e.g. `"evaluation:quality"`.
    pub dimension: String,
    /// The score value carried by an emitted `scores` attestation.
    pub score: f64,
    /// Confidence in the score.
    pub confidence: f64,
    /// Version-pinned evidence references (CC 1.2 T3). Omitted from the
    /// envelope when empty.
    pub evidence_refs: Vec<String>,
    /// Predicted entropy-reduction benefit (CC 1.3 step 2 numerator).
    pub predicted_entropy_reduction: f64,
    /// Predicted flourishing loss — the binding (smallest positive) loss across
    /// the four axes {autonomy, justice, biodiversity, preference_diversity}
    /// (CC 1.3 step 2 denominator). Collapsed here to one scalar per the
    /// frozen interface; `> 0.0` is what the veto reasons over.
    pub predicted_flourishing_loss: f64,
    /// Scalar uncertainty; WBD trigger (a) (CC 1.9).
    pub uncertainty: f64,
    /// Novel dilemma beyond precedent; WBD trigger (b) (CC 1.9).
    pub novel: bool,
    /// Potential severe harm with ambiguous mitigation; WBD trigger (c) (CC 1.9).
    pub severe_ambiguous_harm: bool,
}

/// A unit of cognition presented to the evaluator. The surrounding pipeline
/// fills `situation` (CC 1.3 step 1 Contextualisation) and the candidate; the
/// reduction consumes the verdict.
#[derive(Debug, Clone)]
pub struct Thought {
    /// Situational context (CC 1.3 step 1).
    pub situation: String,
    /// The action under evaluation.
    pub candidate: CandidateAction,
    /// Key id of the attesting authority (carried through to `emit`, not the
    /// envelope itself).
    pub attesting_key_id: String,
    /// Subjects the candidate scores. Omitted from the envelope when empty.
    pub subject_key_ids: Vec<String>,
}

/// A concise Deferral Package per CC 1.9 / CC 1.16.4 — exactly the four named
/// elements. Lifecycle obligations (also CC 1.9), out of band of this struct:
/// halt; transmit to Wise Authorities (appointed under the Governance Charter,
/// CC 1.16.5) via secure channel; remain inactive on the issue; integrate
/// returned guidance; document and learn.
#[derive(Debug, Clone)]
pub struct DeferralPackage {
    /// (i) context (CC 1.9).
    pub context: String,
    /// (ii) dilemma (CC 1.9).
    pub dilemma: String,
    /// (iii) analysis (CC 1.9).
    pub analysis: String,
    /// (iv) rationale (CC 1.9) — names the gate that fired, with citation.
    pub rationale: String,
}

/// The two-valued terminal of the reduction (CC 1.3 step 5 / CC 1.9). There is
/// no third outcome; `Defer` is the safe terminal state.
pub enum Reduction {
    /// The candidate reduced to a gate-checked `scores` form (CC 1.3 step 5).
    Emit(crate::forms::Form),
    /// The candidate was halted and escalated (CC 1.9).
    Defer(DeferralPackage),
}

/// Uncertainty threshold for WBD trigger (a). CC 1.9 names the threshold but
/// fixes no numeral anywhere in Part 1 ("Uncertainty above defined thresholds");
/// it is a deployment-/charter-defined parameter (CC 1.8, CC 1.15.4). This is
/// the spike's configured default.
pub const UNCERTAINTY_THRESHOLD: f64 = 0.7;

/// Order-Maximization Veto ratio (CC 1.3 step 2 / CC 1.16.3 step 2): "predicted
/// entropy-reduction benefit `>=` 10 x any predicted loss ... -> abort action or
/// trigger WBD." Both resolutions collapse to `Defer` in this evaluator.
pub const ORDER_MAX_VETO_RATIO: f64 = 10.0;

/// Reduce a thought to its single terminal value.
///
/// Gate order is load-bearing: (1) WBD triggers (CC 1.9) -> (2) Order-Maximization
/// Veto (CC 1.3 step 2) -> (3) Emit (CC 1.3 step 5). The first gate to fire wins
/// and yields `Defer`; otherwise the reduction falls through to `Emit`.
pub fn reduce(t: &Thought) -> Reduction {
    let c = &t.candidate;

    // ---- GATE 1: Wisdom-Based Deferral (CC 1.9; obligation CC 1.8/CC 1.15.4) ----
    // CC 1.9: "when certainty runs thin, the system halts rather than guesses."
    // Integrity (CC 1.8) wraps the whole PDMA, so this humility / edge-of-competence
    // check is the OUTERMOST guard and runs before any alignment arithmetic is
    // trusted. Any one of the three named triggers fires the deferral:
    //   (a) uncertainty above defined thresholds,
    //   (b) novel dilemma beyond precedent,
    //   (c) potential severe harm with ambiguous mitigation.
    if c.uncertainty > UNCERTAINTY_THRESHOLD || c.novel || c.severe_ambiguous_harm {
        let mut triggers: Vec<&'static str> = Vec::new();
        if c.uncertainty > UNCERTAINTY_THRESHOLD {
            triggers.push("uncertainty above defined threshold");
        }
        if c.novel {
            triggers.push("novel dilemma beyond precedent");
        }
        if c.severe_ambiguous_harm {
            triggers.push("potential severe harm with ambiguous mitigation");
        }
        let rationale = format!(
            "Wisdom-Based Deferral (WBD, CC 1.9): {}",
            triggers.join("; ")
        );
        return Reduction::Defer(deferral_package(t, rationale));
    }

    // ---- GATE 2: Order-Maximization Veto (CC 1.3 step 2 / CC 1.16.3 step 2) ----
    // Verbatim: "If predicted entropy-reduction benefit >= 10 x any predicted loss
    // in autonomy, justice, biodiversity, or preference diversity -> abort action
    // or trigger WBD." The `loss > 0.0` guard is required by meaning: the veto is
    // about a LOSS along a flourishing axis, so a zero-loss axis cannot trigger it
    // (10 * 0 = 0 would otherwise fire trivially). Per CC 1.3's closing note, a
    // ten-to-one efficiency win over a flourishing axis "is not a green light, it
    // is a stop sign." Runs only on thoughts that already cleared the WBD gate.
    if c.predicted_flourishing_loss > 0.0
        && c.predicted_entropy_reduction >= ORDER_MAX_VETO_RATIO * c.predicted_flourishing_loss
    {
        let rationale = format!(
            "Order-Maximization Veto (CC 1.3 step 2): predicted_entropy_reduction {} >= {} x \
             predicted_flourishing_loss {} along a flourishing axis",
            c.predicted_entropy_reduction, ORDER_MAX_VETO_RATIO, c.predicted_flourishing_loss
        );
        return Reduction::Defer(deferral_package(t, rationale));
    }

    // ---- FALL-THROUGH: Selection & Execution -> Emit (CC 1.3 step 5) ----
    // Both gates passed: the candidate is the ethically optimal action. Build the
    // canonical `scores` envelope and read it through the closed-operator gate.
    // `cohort_scope` is "self"; `evidence_refs` and `subject_key_ids` are omitted
    // when empty so the canonical bytes carry no empty-array noise.
    let mut envelope = serde_json::json!({
        "attestation_type": "scores",
        "dimension": c.dimension.clone(),
        "score": c.score,
        "confidence": c.confidence,
        "cohort_scope": "self",
    });
    {
        let obj = envelope
            .as_object_mut()
            .expect("json! object literal is always a JSON object");
        if !c.evidence_refs.is_empty() {
            obj.insert(
                "evidence_refs".to_string(),
                serde_json::Value::from(c.evidence_refs.clone()),
            );
        }
        if !t.subject_key_ids.is_empty() {
            obj.insert(
                "subject_key_ids".to_string(),
                serde_json::Value::from(t.subject_key_ids.clone()),
            );
        }
    }

    // read_form is the closed-operator gate. A `scores` envelope that carries a
    // `dimension` is well-formed by construction, so this cannot fail here; the
    // gate is what enforces the 1+4 closed-operator discipline for ALL inputs.
    let form = crate::forms::read_form(&envelope)
        .expect("a `scores` envelope with a dimension is well-formed by construction");
    Reduction::Emit(form)
}

/// Build the concise four-field Deferral Package (CC 1.9 / CC 1.16.4) from the
/// thought plus the rationale that names the gate which fired.
fn deferral_package(t: &Thought, rationale: String) -> DeferralPackage {
    let c = &t.candidate;
    DeferralPackage {
        // (i) context — CC 1.9
        context: t.situation.clone(),
        // (ii) dilemma — CC 1.9
        dilemma: format!(
            "candidate `scores` on dimension {:?} (score {}, confidence {})",
            c.dimension, c.score, c.confidence
        ),
        // (iii) analysis — CC 1.9
        analysis: format!(
            "uncertainty={}, novel={}, severe_ambiguous_harm={}, \
             predicted_entropy_reduction={}, predicted_flourishing_loss={}",
            c.uncertainty,
            c.novel,
            c.severe_ambiguous_harm,
            c.predicted_entropy_reduction,
            c.predicted_flourishing_loss
        ),
        // (iv) rationale — CC 1.9
        rationale,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A calm, in-precedent candidate: low uncertainty, not novel, no ambiguous
    /// harm, zero flourishing loss. Should reduce to `Emit`.
    fn calm_candidate() -> CandidateAction {
        CandidateAction {
            dimension: "evaluation:quality".to_string(),
            score: 0.85,
            confidence: 0.92,
            evidence_refs: vec![],
            predicted_entropy_reduction: 1.0,
            predicted_flourishing_loss: 0.0,
            uncertainty: 0.1,
            novel: false,
            severe_ambiguous_harm: false,
        }
    }

    fn thought_with(c: CandidateAction) -> Thought {
        Thought {
            situation: "routine quality scoring".to_string(),
            candidate: c,
            attesting_key_id: "deadbeefcafef00d".to_string(),
            subject_key_ids: vec![],
        }
    }

    /// CC 1.3 step 5: both gates pass -> Emit a canonical `scores` Form.
    #[test]
    fn calm_candidate_emits_scores_form() {
        let t = thought_with(calm_candidate());
        match reduce(&t) {
            Reduction::Emit(form) => {
                assert!(
                    matches!(form.op, crate::forms::Op::Scores),
                    "emitted form must be a `scores` op"
                );
                assert_eq!(form.envelope["attestation_type"], "scores");
                assert_eq!(form.envelope["dimension"], "evaluation:quality");
                assert_eq!(form.envelope["score"], 0.85);
                assert_eq!(form.envelope["confidence"], 0.92);
                assert_eq!(form.envelope["cohort_scope"], "self");
                // Empty collections are omitted from the envelope.
                assert!(
                    form.envelope.get("evidence_refs").is_none(),
                    "empty evidence_refs must be omitted"
                );
                assert!(
                    form.envelope.get("subject_key_ids").is_none(),
                    "empty subject_key_ids must be omitted"
                );
            }
            Reduction::Defer(_) => panic!("calm candidate must Emit, not Defer"),
        }
    }

    /// CC 1.9 trigger (a): uncertainty above threshold -> Defer (WBD).
    #[test]
    fn high_uncertainty_defers_wbd() {
        let mut c = calm_candidate();
        c.uncertainty = 0.95; // strictly greater than UNCERTAINTY_THRESHOLD (0.7)
        let t = thought_with(c);
        match reduce(&t) {
            Reduction::Defer(pkg) => {
                assert!(pkg.rationale.contains("WBD"), "rationale must cite WBD");
                assert!(pkg.rationale.contains("CC 1.9"), "rationale must cite CC 1.9");
                assert_eq!(pkg.context, "routine quality scoring");
            }
            Reduction::Emit(_) => panic!("high-uncertainty candidate must Defer (WBD)"),
        }
    }

    /// CC 1.9 trigger (b): novel dilemma beyond precedent -> Defer (WBD).
    #[test]
    fn novel_candidate_defers() {
        let mut c = calm_candidate();
        c.novel = true;
        let t = thought_with(c);
        assert!(
            matches!(reduce(&t), Reduction::Defer(_)),
            "novel candidate must Defer (WBD)"
        );
    }

    /// CC 1.9 trigger (c): severe harm with ambiguous mitigation -> Defer (WBD).
    #[test]
    fn severe_ambiguous_harm_defers() {
        let mut c = calm_candidate();
        c.severe_ambiguous_harm = true;
        let t = thought_with(c);
        assert!(
            matches!(reduce(&t), Reduction::Defer(_)),
            "severe-ambiguous-harm candidate must Defer (WBD)"
        );
    }

    /// CC 1.3 step 2: entropy_reduction (100) >= 10 x loss (1) -> Defer (veto).
    #[test]
    fn order_maximization_veto_defers() {
        let mut c = calm_candidate();
        c.predicted_entropy_reduction = 100.0;
        c.predicted_flourishing_loss = 1.0; // 100 >= 10 * 1 -> veto fires
        let t = thought_with(c);
        match reduce(&t) {
            Reduction::Defer(pkg) => {
                assert!(
                    pkg.rationale.contains("CC 1.3 step 2"),
                    "veto rationale must cite CC 1.3 step 2"
                );
            }
            Reduction::Emit(_) => panic!("order-maximization-veto candidate must Defer"),
        }
    }

    /// The `loss > 0.0` guard: a huge entropy win with ZERO flourishing loss must
    /// NOT fire the veto (10 * 0 = 0 must not trivially trip). It Emits.
    #[test]
    fn veto_does_not_fire_when_loss_is_zero() {
        let mut c = calm_candidate();
        c.predicted_entropy_reduction = 1.0e9;
        c.predicted_flourishing_loss = 0.0;
        let t = thought_with(c);
        assert!(
            matches!(reduce(&t), Reduction::Emit(_)),
            "zero-loss candidate must Emit; the veto is about a LOSS"
        );
    }

    /// `>` is strict: uncertainty exactly at the threshold does NOT trigger WBD.
    #[test]
    fn uncertainty_at_threshold_emits() {
        let mut c = calm_candidate();
        c.uncertainty = UNCERTAINTY_THRESHOLD; // equal, not greater
        let t = thought_with(c);
        assert!(
            matches!(reduce(&t), Reduction::Emit(_)),
            "uncertainty == threshold must not trip the strict-> WBD gate"
        );
    }

    /// Gate ordering is load-bearing: WBD runs before the veto. A thought that
    /// would also trip the veto still defers with a WBD rationale, not a veto one.
    #[test]
    fn wbd_short_circuits_before_veto() {
        let mut c = calm_candidate();
        c.uncertainty = 0.99; // WBD trigger (a)
        c.predicted_entropy_reduction = 100.0; // would also trip the veto
        c.predicted_flourishing_loss = 1.0;
        let t = thought_with(c);
        match reduce(&t) {
            Reduction::Defer(pkg) => {
                assert!(
                    pkg.rationale.contains("WBD"),
                    "WBD must win the short-circuit over the veto"
                );
                assert!(
                    !pkg.rationale.contains("Order-Maximization Veto"),
                    "the veto rationale must not appear when WBD fired first"
                );
            }
            Reduction::Emit(_) => panic!("must Defer"),
        }
    }

    /// Non-empty evidence_refs and subject_key_ids are carried into the envelope.
    #[test]
    fn emit_carries_nonempty_collections() {
        let mut c = calm_candidate();
        c.evidence_refs = vec!["ref:rulev1#42".to_string()];
        let mut t = thought_with(c);
        t.subject_key_ids = vec!["subjectkey01".to_string()];
        match reduce(&t) {
            Reduction::Emit(form) => {
                assert_eq!(
                    form.envelope["evidence_refs"],
                    serde_json::json!(["ref:rulev1#42"])
                );
                assert_eq!(
                    form.envelope["subject_key_ids"],
                    serde_json::json!(["subjectkey01"])
                );
            }
            Reduction::Defer(_) => panic!("calm candidate must Emit"),
        }
    }
}
