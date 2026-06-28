//! forms.rs — the closed 1+4 special-form set.
//!
//! In the Attestation Calculus (see `CEWPOS_ATTESTATION_CALCULUS.md` §3), a CEG
//! envelope is an s-expression and its head is its `attestation_type`. The
//! special forms of this Lisp are *closed*: exactly the structural op `scores`
//! plus the four composers `delegates_to` / `supersedes` / `withdraws` /
//! `recants` — and **nothing else** (CC 1.7, CC 5.3.2.4.4: "open data, closed
//! operators"). There is no general `eval`.
//!
//! The crux of this module is [`read_form`]: the **closed-operator gate**. It is
//! the reader, and it rejects — *at read time* — any envelope whose head is not
//! one of the five. A hallucinated operator (`eval`, `exec`, `lambda`, `setf`,
//! …) is not a form; it is a read error.

use crate::sexpr::{value_to_sexpr, Sexpr};

/// The closed set of special-form operators: `scores` (the structural op) plus
/// the four composers. This enum is the whole alphabet of the calculus; there is
/// deliberately no `Other`/`Unknown` variant — an unknown head cannot be
/// *represented* as an `Op`, only rejected by [`read_form`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    /// `scores` — the structural attestation op (CC 2.1).
    Scores,
    /// `delegates_to` — composer.
    DelegatesTo,
    /// `supersedes` — composer (append, never mutate; CC 5.3.2.3).
    Supersedes,
    /// `withdraws` — composer (forward-only revocation).
    Withdraws,
    /// `recants` — composer.
    Recants,
}

impl Op {
    /// Map a wire head string to its operator, or `None` if the head is outside
    /// the closed 1+4 set. This is the single source of truth for the gate.
    pub fn from_head(s: &str) -> Option<Op> {
        match s {
            "scores" => Some(Op::Scores),
            "delegates_to" => Some(Op::DelegatesTo),
            "supersedes" => Some(Op::Supersedes),
            "withdraws" => Some(Op::Withdraws),
            "recants" => Some(Op::Recants),
            _ => None,
        }
    }

    /// The canonical wire head for this operator. Inverse of [`Op::from_head`].
    pub fn as_str(&self) -> &'static str {
        match self {
            Op::Scores => "scores",
            Op::DelegatesTo => "delegates_to",
            Op::Supersedes => "supersedes",
            Op::Withdraws => "withdraws",
            Op::Recants => "recants",
        }
    }
}

/// A read, well-typed form: a recognized operator paired with the JSON envelope
/// it was read from. Invariant: `envelope` is a JSON object whose
/// `"attestation_type"` member equals `op.as_str()` and which carries a
/// `"dimension"` member.
#[derive(Debug, Clone, PartialEq)]
pub struct Form {
    pub op: Op,
    pub envelope: serde_json::Value,
}

/// Why an envelope failed to read as a [`Form`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadError {
    /// The head (`attestation_type`) was present and a string, but is not one of
    /// the closed 1+4 operators. Carries the offending head verbatim. This is the
    /// closed-operator invariant firing.
    UnknownOperator(String),
    /// The top-level value was not a JSON object.
    NotAnObject,
    /// A required member was absent.
    MissingField(&'static str),
}

/// The reader and the **closed-operator gate**.
///
/// Reads the JSON object member `"attestation_type"`, maps it through
/// [`Op::from_head`], and admits the envelope only if the head is one of the
/// five. Order of checks:
///
/// 1. not a JSON object            → [`ReadError::NotAnObject`]
/// 2. `attestation_type` absent / not a string → [`ReadError::MissingField`]`("attestation_type")`
/// 3. `attestation_type` is a string outside the 1+4 → [`ReadError::UnknownOperator`]
/// 4. `dimension` absent           → [`ReadError::MissingField`]`("dimension")`
///
/// On success the envelope is cloned into the returned [`Form`]; the reader does
/// not normalize or rewrite it (JCS canonicalization is the substrate's job).
pub fn read_form(envelope: &serde_json::Value) -> Result<Form, ReadError> {
    // (1) Must be a JSON object.
    let obj = envelope.as_object().ok_or(ReadError::NotAnObject)?;

    // (2) Head must be present and a string. A missing head — or a head that is
    // present but not a string (so it cannot name an operator) — means the
    // envelope carries no operator at all.
    let head = obj
        .get("attestation_type")
        .and_then(|v| v.as_str())
        .ok_or(ReadError::MissingField("attestation_type"))?;

    // (3) THE GATE. The head must name one of the closed 1+4 operators; any
    // other head (eval, exec, lambda, setf, …) is rejected here, verbatim.
    let op = Op::from_head(head).ok_or_else(|| ReadError::UnknownOperator(head.to_string()))?;

    // (4) Every form carries a dimension.
    if !obj.contains_key("dimension") {
        return Err(ReadError::MissingField("dimension"));
    }

    Ok(Form {
        op,
        envelope: envelope.clone(),
    })
}

/// Print a form as an s-expression by mirroring its envelope. Homoiconicity: a
/// form *is* its data (the FSD's real goal — a reflective, agent-legible system —
/// delivered without a mutable image).
pub fn form_to_sexpr(f: &Form) -> Sexpr {
    value_to_sexpr(&f.envelope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// (a) A well-formed `scores` envelope reads to `Op::Scores`.
    #[test]
    fn well_formed_scores_reads_to_scores() {
        let env = json!({
            "attestation_type": "scores",
            "dimension": "evaluation:quality",
            "score": 0.85,
            "confidence": 0.92,
            "evidence_refs": [],
            "cohort_scope": "self"
        });
        let form = read_form(&env).expect("well-formed scores envelope must read");
        assert_eq!(form.op, Op::Scores);
        // The reader does not mutate the envelope.
        assert_eq!(form.envelope, env);
    }

    /// (b) The closed-operator invariant: any head outside the 1+4 — including
    /// the headline `eval`, and the mutable-image ops `exec`/`lambda`/`setf`/
    /// `rplaca` — is rejected as `UnknownOperator`, carrying the head verbatim.
    #[test]
    fn non_closed_heads_are_rejected() {
        for bad in [
            "eval", "exec", "lambda", "setf", "rplaca", "apply", "scores ", "Scores", "",
        ] {
            let env = json!({ "attestation_type": bad, "dimension": "x" });
            match read_form(&env) {
                Err(ReadError::UnknownOperator(got)) => assert_eq!(got, bad),
                other => panic!("expected UnknownOperator({bad:?}), got {other:?}"),
            }
        }
    }

    /// (c) Each of the five valid heads round-trips through `from_head`/`as_str`
    /// in both directions.
    #[test]
    fn five_heads_round_trip() {
        let heads = [
            ("scores", Op::Scores),
            ("delegates_to", Op::DelegatesTo),
            ("supersedes", Op::Supersedes),
            ("withdraws", Op::Withdraws),
            ("recants", Op::Recants),
        ];
        for (head, op) in heads {
            // wire head -> Op
            assert_eq!(Op::from_head(head), Some(op));
            // Op -> wire head
            assert_eq!(op.as_str(), head);
            // and back again
            assert_eq!(Op::from_head(op.as_str()), Some(op));
        }
        // The set is closed at exactly five.
        assert_eq!(heads.len(), 5);
    }

    /// (d) A recognized head with no `dimension` is a `MissingField("dimension")`.
    #[test]
    fn missing_dimension_is_missing_field() {
        let env = json!({ "attestation_type": "scores" });
        assert_eq!(
            read_form(&env),
            Err(ReadError::MissingField("dimension"))
        );
    }

    /// Bonus: a missing head is `MissingField("attestation_type")`, and the head
    /// check precedes the dimension check.
    #[test]
    fn missing_attestation_type_is_missing_field() {
        let env = json!({ "dimension": "evaluation:quality" });
        assert_eq!(
            read_form(&env),
            Err(ReadError::MissingField("attestation_type"))
        );
    }

    /// Bonus: a non-object top-level value is `NotAnObject`.
    #[test]
    fn non_object_is_not_an_object() {
        for v in [json!(null), json!(true), json!(42), json!("scores"), json!([1, 2, 3])] {
            assert_eq!(read_form(&v), Err(ReadError::NotAnObject));
        }
    }

    /// Bonus: a non-string head cannot name an operator, so it reads as a missing
    /// `attestation_type` rather than an unknown operator.
    #[test]
    fn non_string_head_is_missing_field() {
        let env = json!({ "attestation_type": 7, "dimension": "x" });
        assert_eq!(
            read_form(&env),
            Err(ReadError::MissingField("attestation_type"))
        );
    }
}
