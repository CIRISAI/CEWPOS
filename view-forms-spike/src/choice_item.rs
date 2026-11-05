//! choice_item.rs — Canonical CC 4.5.13 "The choice as data" data model (`manifests/choice_item.cddl`).
//!
//! Every decision a person is asked to make (a First-Run Wizard step, a consent gate, an AI steward
//! deferral, a room vote, or a moderation action) is represented as a single `ChoiceItem` object.
//!
//! ## Core Architectural Invariants (CC 4.5.13 / CC 5.4.3 / Issue #2)
//! 1. **Impact is derived, not written**: Each option declares the CEG rows it would sign (`writes`).
//!    What moves, where it goes, whether it widens or narrows, and whether it can be undone are looked
//!    up from the family model.
//! 2. **The policy is a governance row**: `self_choice` runs 1-of-1 with no silence window; room votes
//!    run the room's two-path row from `governance_policy.json`.
//! 3. **Declining is a complete answer, never a failed vote**: In the self case, a decline is
//!    recorded in `answer.value` and completes the decision (`LandsForward`), never causing `TwoPathVerdict::Cancelled`.
//! 4. **Four kinds and no more**: `choice` (>= 2 options), `act` (1 option), `entry` (typed/ceremony value),
//!    and `notice` (no options: fact, state, or refusal).
//! 5. **Directional Asymmetry Law**: Options with `direction: widens` or `undo.possible: false` require a
//!    3-fact `P8: ConfirmSheet`; options with `direction: narrows` or `same` with `undo.possible: true`
//!    execute in 1 step!

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::algebra::{
    CircleScope, ConfirmFacts3, DeferralChoice, DeferralVote, GovernancePolicyFamily,
    TwoPathVerdict,
};
use crate::epistemic_scene::{
    CeremonySignerState, DisclosureHint, SignerChip, TwoPathCeremonyNode,
};
use crate::view_form::view_deferral_vote;

/// Provenance of displayed fact (CC 4.5.13 CDDL: `"wire" / "by_rule" / "not_sent"`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactProvenance {
    Wire,
    ByRule,
    NotSent,
}

/// CC 4.5.13: The four closed item kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChoiceKind {
    /// Two or more options.
    Choice,
    /// Typed input or entry arriving by ceremony (P10).
    Entry,
    /// Exactly one option — a thing done.
    Act,
    /// Zero options — facts, state, or refusal notice (P2/P3/P11/P12).
    Notice,
}

/// The concentric circle context where the choice lives.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChoiceCircle {
    SelfScope,
    Family,
    Community,
    Affiliations,
    Federation,
}

impl From<CircleScope> for ChoiceCircle {
    fn from(cs: CircleScope) -> Self {
        match cs {
            CircleScope::JustMe => ChoiceCircle::SelfScope,
            CircleScope::Family => ChoiceCircle::Family,
            CircleScope::Neighbours => ChoiceCircle::Community,
            CircleScope::CommunitiesAndBusinesses => ChoiceCircle::Affiliations,
            CircleScope::Everyone => ChoiceCircle::Federation,
        }
    }
}

impl From<ChoiceCircle> for CircleScope {
    fn from(cc: ChoiceCircle) -> Self {
        match cc {
            ChoiceCircle::SelfScope => CircleScope::JustMe,
            ChoiceCircle::Family => CircleScope::Family,
            ChoiceCircle::Community => CircleScope::Neighbours,
            ChoiceCircle::Affiliations => CircleScope::CommunitiesAndBusinesses,
            ChoiceCircle::Federation => CircleScope::Everyone,
        }
    }
}

/// The contextual role of the decision.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChoiceRole {
    FirstRun,
    Consent,
    Deferral,
    Vote,
    Moderation,
}

/// Localized or referenced human-readable text (CC 5.4.3).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextRef {
    pub id: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,
}

impl TextRef {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            params: None,
        }
    }
}

/// Context block for a ChoiceItem (`context`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceContext {
    pub circle: ChoiceCircle,
    pub role: ChoiceRole,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub atoms: Vec<String>,
    pub title: TextRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body: Vec<TextRef>,
}

/// A fact displayed alongside the choice, never chosen (`facts`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceFact {
    pub label: TextRef,
    pub value: serde_json::Value,
    pub provenance: FactProvenance,
}

/// CEG row write operation declared by an option (`writes`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceWrite {
    pub family: String,
    pub verb: String,
    pub scope: ChoiceCircle,
}

/// Direction of state movement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactDirection {
    Widens,
    Narrows,
    Same,
}

/// Destination of moving data (`to`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactDestination {
    pub audience: String,
    pub tier: String,
}

/// Erasure and retraction route (`undo`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoSpec {
    pub possible: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub how: Option<TextRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub later: Option<String>,
}

/// Impact derived from the family model for an option's `writes` (`impact`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedImpact {
    pub moves: TextRef,
    pub to: ImpactDestination,
    pub direction: ImpactDirection,
    pub undo: UndoSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub held: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<String>,
}

impl DerivedImpact {
    /// Directional Asymmetry Law:
    /// Returns true if choosing this option requires a 3-fact `P8: ConfirmSheet`.
    /// Widening or irreversible options always require confirmation.
    /// Narrowing and reversible options execute in 1 step!
    pub fn requires_p8_confirm(&self) -> bool {
        self.direction == ImpactDirection::Widens || !self.undo.possible
    }

    /// Build the canonical 3-fact `ConfirmFacts3` sheet from this derived impact.
    pub fn to_confirm_facts_3(&self, option_label: &str) -> ConfirmFacts3 {
        let undo_str = if self.undo.possible {
            self.undo
                .how
                .as_ref()
                .map(|h| h.text.clone())
                .unwrap_or_else(|| "Yes — retract in 1 step from settings".to_string())
        } else {
            "No — irreversible action once signed".to_string()
        };

        let danger = if self.direction == ImpactDirection::Widens && self.to.tier == "federation" {
            Some("Federation exposure: Shared telemetry will replicate to federation peers under CC 3.3.1 replication consent.".to_string())
        } else if !self.undo.possible {
            Some("Irreversible mutation: Cannot be undone or recalled after signing.".to_string())
        } else {
            None
        };

        ConfirmFacts3::new(
            format!("Confirm {}", option_label),
            ("What moves", self.moves.text.clone()),
            ("Where it goes", format!("{} ({})", self.to.audience, self.to.tier)),
            ("Can I undo it?", undo_str),
            danger,
        )
    }
}

/// One selectable option in a ChoiceItem (`options`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceOption {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub benefit: TextRef,
    pub downside: TextRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub writes: Vec<ChoiceWrite>,
    pub impact: DerivedImpact,
    pub available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable: Option<TextRef>,
}

/// Source specification for entry items (`entry`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntrySpec {
    pub source: String, // "typed" or "ceremony"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_len: Option<usize>,
}

/// State of an answered item (`answer`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerState {
    Served,
    Held,
    Signed,
    Deferred,
    Changed,
}

/// The recorded answer to a ChoiceItem.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceAnswer {
    pub state: AnswerState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_ms: Option<u64>,
    pub shown_digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<String>>,
}

/// **CC 4.5.13 ChoiceItem**: The single data model for decisions, context, options, and derived impact.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceItem {
    pub id: String,
    pub kind: ChoiceKind,
    pub context: ChoiceContext,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facts: Vec<ChoiceFact>,
    pub policy: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<ChoiceOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<EntrySpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<ChoiceAnswer>,
}

impl ChoiceItem {
    /// True when this item runs under self governance (1-of-1 self-quorum, e.g. first-run or consent).
    pub fn is_self_choice(&self) -> bool {
        self.policy == "self_choice"
            || self.context.role == ChoiceRole::FirstRun
            || self.context.role == ChoiceRole::Consent
            || self.context.circle == ChoiceCircle::SelfScope
    }

    /// Convert a `ChoiceItem` directly to a `TwoPathCeremonyNode` (`P9`),
    /// rendering choices from the served data with zero hardcoded option copy.
    pub fn to_ceremony_node(&self, tag_prefix: impl Into<String>, proposed_by: impl Into<String>) -> TwoPathCeremonyNode {
        let prefix = tag_prefix.into();
        let proposer = proposed_by.into();

        // Map options to DeferralChoices dynamically from served data.
        let is_self = self.is_self_choice();
        let answered_val = self.answer.as_ref().and_then(|a| a.value.as_deref());

        let choices: Vec<DeferralChoice> = self
            .options
            .iter()
            .enumerate()
            .map(|(_idx, opt)| {
                let is_selected = answered_val == Some(&opt.value)
                    || (answered_val.is_none() && self.default.as_deref() == Some(&opt.value));

                // In the self case: declining is answer.value, never a negative vote score.
                let vote_score = if is_self {
                    0
                } else if opt.value.contains("yes") || opt.value.contains("assent") || opt.value.contains("approve") {
                    1
                } else if opt.value.contains("no") || opt.value.contains("object") || opt.value.contains("refuse") {
                    -1
                } else {
                    0
                };

                // Species for 3D holon representation (C=6, O=8, N=7, H=1, F=9).
                let z_species = if opt.impact.direction == ImpactDirection::Widens {
                    6 // Carbon (widening / connecting)
                } else if opt.impact.direction == ImpactDirection::Narrows {
                    8 // Oxygen (focusing / keeping local)
                } else {
                    7 // Nitrogen
                };

                let csd_tag = opt.tag.clone().unwrap_or_else(|| opt.value.clone());
                let consequence = format!("Benefit: {} — Downside: {}", opt.benefit.text, opt.downside.text);

                DeferralChoice {
                    choice_id: opt.value.clone(),
                    label: opt.benefit.text.clone(),
                    consequence,
                    vote_score,
                    z_species,
                    action_tag: opt.tag.clone().unwrap_or_else(|| format!("{prefix}_{}", opt.value)),
                    selected: is_selected,
                    csd_tag,
                }
            })
            .collect();

        let deferral_vote = DeferralVote {
            deferral_id: self.id.clone(),
            dimension: self.options.first().and_then(|o| o.writes.first()).map(|w| w.family.clone()).unwrap_or_else(|| "choice:v1".to_string()),
            cohort_scope: self.context.circle.into(),
            title: self.context.title.text.clone(),
            context_prompt: self.context.body.first().map(|b| b.text.clone()).unwrap_or_default(),
            roster_size: 1,
            enact_needed: 1,
            approvals: if answered_val.is_some() { 1 } else { 0 },
            objections: 0,
            step_index: 1,
            total_steps: 1,
            choices,
        };

        let (_, holon_frame) = view_deferral_vote(&deferral_vote);

        // In the self case, answering is complete immediately upon selection.
        let verdict = if is_self {
            if answered_val.is_some() {
                TwoPathVerdict::LandsForward
            } else {
                TwoPathVerdict::Pending {
                    approvals: 0,
                    enact_needed: 1,
                    objections: 0,
                    cancel_needed: 1,
                    floor_needed: 1,
                    window_rem_secs: 0,
                }
            }
        } else if self.answer.as_ref().map(|a| a.state) == Some(AnswerState::Signed) {
            TwoPathVerdict::LandsForward
        } else {
            TwoPathVerdict::Pending {
                approvals: 0,
                enact_needed: 1,
                objections: 0,
                cancel_needed: 1,
                floor_needed: 1,
                window_rem_secs: 86_400,
            }
        };

        TwoPathCeremonyNode {
            tag_prefix: prefix.clone(),
            disclosure: DisclosureHint::EllipsisExpand {
                tag: format!("btn_{prefix}_ellipsis"),
                collapsed_by_default: false,
                summary_chip: format!("Policy: {}", self.policy),
            },
            policy_family: GovernancePolicyFamily::SelfRosterAdd,
            proposal: self.context.title.text.clone(),
            proposed_by: proposer.clone(),
            signers: vec![SignerChip {
                key_id: proposer,
                state: if answered_val.is_some() {
                    CeremonySignerState::Signed
                } else {
                    CeremonySignerState::Proposed
                },
            }],
            verdict,
            own_proposal: false,
            can_assent: true,
            can_object: true,
            deferral_vote: Some(deferral_vote),
            holon_frame: Some(holon_frame),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_trace_choice_item() -> ChoiceItem {
        ChoiceItem {
            id: "choice:setup:trace_replication:v1".to_string(),
            kind: ChoiceKind::Choice,
            context: ChoiceContext {
                circle: ChoiceCircle::Federation,
                role: ChoiceRole::Consent,
                atoms: vec!["root.none".to_string(), "node.unclaimed".to_string()],
                title: TextRef::new("txt_trace_title", "Share diagnostic traces with the CIRIS federation?"),
                body: vec![TextRef::new(
                    "txt_trace_body",
                    "Nothing leaves Just me without an explicit scope choice. Diagnostic traces help mesh reliability.",
                )],
            },
            facts: vec![ChoiceFact {
                label: TextRef::new("lbl_scope", "Active Scope"),
                value: serde_json::Value::String("Just me".to_string()),
                provenance: FactProvenance::ByRule,
            }],
            policy: "self_choice".to_string(),
            options: vec![
                ChoiceOption {
                    value: "trace_consent_yes".to_string(),
                    tag: Some("trace_consent_yes".to_string()),
                    benefit: TextRef::new("b_yes", "Share anonymized diagnostic traces to help federation reliability"),
                    downside: TextRef::new("d_yes", "Stripped trace envelopes will replicate across federation peers"),
                    writes: vec![ChoiceWrite {
                        family: "consent:replication:v1".to_string(),
                        verb: "attest".to_string(),
                        scope: ChoiceCircle::Federation,
                    }],
                    impact: DerivedImpact {
                        moves: TextRef::new("m_traces", "Diagnostic trace envelopes"),
                        to: ImpactDestination {
                            audience: "federation peers".to_string(),
                            tier: "federation".to_string(),
                        },
                        direction: ImpactDirection::Widens,
                        undo: UndoSpec {
                            possible: true,
                            how: Some(TextRef::new("u_yes", "Withdraw anytime in Just me settings")),
                            later: None,
                        },
                        held: Some("settle_on_key".to_string()),
                        projection: Some("consent_peer_set".to_string()),
                    },
                    available: true,
                    unavailable: None,
                },
                ChoiceOption {
                    value: "trace_consent_no".to_string(),
                    tag: Some("trace_consent_no".to_string()),
                    benefit: TextRef::new("b_no", "Strict isolation: zero diagnostic telemetry leaves this device"),
                    downside: TextRef::new("d_no", "The node cannot contribute to or benefit from mesh trace analysis"),
                    writes: Vec::new(),
                    impact: DerivedImpact {
                        moves: TextRef::new("m_local", "Keeps all traces local to node storage"),
                        to: ImpactDestination {
                            audience: "owner's node only".to_string(),
                            tier: "local".to_string(),
                        },
                        direction: ImpactDirection::Narrows,
                        undo: UndoSpec {
                            possible: true,
                            how: Some(TextRef::new("u_no", "Can enable sharing at any time")),
                            later: None,
                        },
                        held: None,
                        projection: None,
                    },
                    available: true,
                    unavailable: None,
                },
            ],
            entry: None,
            default: None,
            required: true,
            answer: None,
        }
    }

    #[test]
    fn choice_item_cddl_roundtrip_and_zero_import_validation() {
        let item = sample_trace_choice_item();
        let serialized = serde_json::to_string(&item).expect("ChoiceItem must serialize cleanly");

        // Verify JSON fields match CDDL schema requirements.
        let json_val: serde_json::Value = serde_json::from_str(&serialized).expect("Must parse valid JSON");
        assert_eq!(json_val["kind"], "choice");
        assert_eq!(json_val["context"]["role"], "consent");
        assert_eq!(json_val["policy"], "self_choice");
        assert_eq!(json_val["options"].as_array().unwrap().len(), 2);

        // Deserialization round-trip.
        let deserialized: ChoiceItem = serde_json::from_str(&serialized).expect("Must deserialize cleanly");
        assert_eq!(deserialized, item);
    }

    #[test]
    fn derived_impact_directional_asymmetry_p8_sheet() {
        let item = sample_trace_choice_item();
        let yes_opt = &item.options[0];
        let no_opt = &item.options[1];

        // 1. Widening move requires P8: ConfirmSheet (Directional Asymmetry Law).
        assert_eq!(yes_opt.impact.direction, ImpactDirection::Widens);
        assert!(yes_opt.impact.requires_p8_confirm(), "Widening option must require P8 ConfirmSheet");

        let p8_sheet = yes_opt.impact.to_confirm_facts_3(&yes_opt.benefit.text);
        assert_eq!(p8_sheet.facts.len(), 3);
        assert_eq!(p8_sheet.facts[0].label, "What moves");
        assert_eq!(p8_sheet.facts[0].value, "Diagnostic trace envelopes");
        assert_eq!(p8_sheet.facts[1].label, "Where it goes");
        assert_eq!(p8_sheet.facts[1].value, "federation peers (federation)");
        assert_eq!(p8_sheet.facts[2].label, "Can I undo it?");
        assert_eq!(p8_sheet.facts[2].value, "Withdraw anytime in Just me settings");
        assert!(p8_sheet.danger_note.is_some(), "Federation widening move must carry danger note");

        // 2. Narrowing move bypasses P8 confirmation (executes in 1 step).
        assert_eq!(no_opt.impact.direction, ImpactDirection::Narrows);
        assert!(!no_opt.impact.requires_p8_confirm(), "Narrowing reversible option executes in 1 step without friction");
    }

    #[test]
    fn self_choice_declining_is_complete_answer_not_failed_vote() {
        let mut item = sample_trace_choice_item();

        // When unanswered:
        let node_unanswered = item.to_ceremony_node("trace_consent", "owner-key");
        assert!(matches!(node_unanswered.verdict, TwoPathVerdict::Pending { .. }));

        // When declined (trace_consent_no):
        item.answer = Some(ChoiceAnswer {
            state: AnswerState::Signed,
            value: Some("trace_consent_no".to_string()),
            at_ms: Some(1730764800000),
            shown_digest: "sha256-dummy-digest-trace-consent".to_string(),
            rows: None,
        });

        let node_declined = item.to_ceremony_node("trace_consent", "owner-key");
        // Must land forward, NOT Cancelled! Declining is an answer value, not a cancelled motion.
        assert_eq!(
            node_declined.verdict,
            TwoPathVerdict::LandsForward,
            "Declining a 1-of-1 self choice is a complete valid answer, not a failed/cancelled vote"
        );
        let selected_choice = node_declined
            .deferral_vote
            .as_ref()
            .unwrap()
            .choices
            .iter()
            .find(|c| c.choice_id == "trace_consent_no")
            .unwrap();
        assert!(selected_choice.selected);
        assert_eq!(selected_choice.vote_score, 0, "Self choice decline has vote_score 0 (recorded in answer.value)");
    }

    #[test]
    fn two_path_ceremony_renders_from_choice_item_no_hardcoded_copy() {
        let item = sample_trace_choice_item();
        let node = item.to_ceremony_node("trace_consent", "owner-key");

        let dv = node.deferral_vote.as_ref().expect("Must project deferral vote");
        assert_eq!(dv.title, item.context.title.text);
        assert_eq!(dv.context_prompt, item.context.body[0].text);
        assert_eq!(dv.choices.len(), 2);
        assert_eq!(dv.choices[0].label, item.options[0].benefit.text);
        assert_eq!(dv.choices[1].label, item.options[1].benefit.text);
        assert!(dv.choices[0].consequence.contains(&item.options[0].downside.text));
        assert!(dv.choices[1].consequence.contains(&item.options[1].downside.text));
    }
}
