//! epistemic_scene.rs — The 12 Minimum Viable Reusable Primitives (`P1`–`P12`) & Typed `EpistemicScene` IR.
//!
//! Unites CEWPOS's zero-import WASM view-forms and 3D Bevy `Scene` IR with `CIRISClient`'s
//! 12 formal primitives, the `CC rc9` `(Claims × Contexts × Actions × Conclusions)` algebra,
//! and the `CSD/3 → CSD/4` automation tag tree.

use crate::algebra::{
    evaluate_policy_lens, ActionIntent, AgentGateVerdict, CircleScope, CircleTab, ConfirmFacts3,
    GovernancePolicyFamily, HaltVerdict, InstrumentId, PolicyLensVerdict, Receipt5,
    RetirementStage, RootVerdict, RowStanding, SurfaceState, Tone, TopologyId, TwoPathVerdict,
    VerificationBasis, ViewerContext,
};
use crate::scene::Scene;
use crate::view_form::view_form;
use attestation_calculus_spike::forms::Form;
use serde::{Deserialize, Serialize};

// ============================================================================
// THE 12 MINIMUM VIABLE REUSABLE PRIMITIVES (P1–P12)
// ============================================================================

/// P5: `ScopePill` (`ScopePill.kt`) — The SOLE place a Circle's color and name are rendered.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopePillNode {
    pub tag: String,
    pub circle: CircleScope,
    pub color_hex: String,
    pub label: String,
    /// True when the row is still device-local (`row.tier = local`, `CSD-127`).
    pub local_only_badge: bool,
}

impl ScopePillNode {
    pub fn from_circle(tag: impl Into<String>, circle: CircleScope, local_only: bool) -> Self {
        Self {
            tag: tag.into(),
            circle,
            color_hex: circle.accent_hex().to_string(),
            label: circle.label().to_string(),
            local_only_badge: local_only,
        }
    }
}

/// P6: `Chip` (`Chip.kt`) — Filter, sort, segmented choice, or static toned badge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChipKind {
    Filter,
    Sort,
    Choice,
    Badge,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChipNode {
    pub tag: String,
    pub kind: ChipKind,
    pub label: String,
    pub tone: Tone,
    pub selected: bool,
}

/// P2: `FieldRow` (`FieldRow.kt`) — Scalar state variable key-value row with optional wire name.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldNode {
    pub tag: String,
    pub label: String,
    pub value: String,
    /// Optional monospace CEG wire field or state-table variable name.
    pub protocol_wire_name: Option<String>,
    /// Optional plain-language gloss.
    pub gloss: Option<String>,
    pub basis: VerificationBasis,
}

/// P3: `ItemRow` (`ItemRow.kt`) — Collection row for any `THING`, `ACCOUNT`, or `RECORD`.
///
/// Enforces two structural invariants:
/// 1. **3-Fact Inline Scannability Cap**: `title`, `subtitle`, and `scope_pill` (plus chips/action).
/// 2. **Hamburger Epistemic Test (`CSD-006`)**: `has_hamburger()` is `true` **if and only if**
///    `receipt.is_some()` (i.e. the row is backed by a signed CEG claim off the wire).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemNode {
    pub id: String,
    pub tag: String,
    /// Fact 1: Primary human-legible title.
    pub title: String,
    /// Fact 2: Secondary summary / dimension subtitle.
    pub subtitle: String,
    /// Fact 3: Circle visibility badge (`ScopePill`).
    pub scope_pill: ScopePillNode,
    pub chips: Vec<ChipNode>,
    /// Local policy-lens rendering decision (`Clear | Blurred | HiddenByRules | FileGone`).
    pub lens_verdict: PolicyLensVerdict,
    /// Present if and only if this row is a signed CEG claim (`CSD-006`).
    pub receipt: Option<Receipt5>,
    /// Optional primary action button on the row.
    pub primary_action: Option<(String, ActionIntent)>,
}

impl ItemNode {
    /// **The Hamburger Epistemic Invariant (`CSD-006`):**
    /// A row has a receipt hamburger (`btn_receipt_<id>`) iff `receipt.is_some()`.
    pub fn has_hamburger(&self) -> bool {
        self.receipt.is_some()
    }

    pub fn hamburger_tag(&self) -> Option<String> {
        if self.has_hamburger() {
            Some(format!("btn_receipt_{}", self.id))
        } else {
            None
        }
    }
}

/// Signer state inside `TwoPathCeremonyBlock` (`P9`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CeremonySignerState {
    Proposed,
    Signed,
    Objected,
    NotYet,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignerChip {
    pub key_id: String,
    pub state: CeremonySignerState,
}

/// Progressive-disclosure trigger kind: keeps the resting surface calm by hiding
/// secondary protocol/quorum/lineage detail behind `☰` (Receipt) or `⋯` (Ellipsis).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisclosureHint {
    /// `☰` Hamburger trigger (`btn_receipt_<id>`) — opens the 5-fact `ReceiptSheet` (`CSD-006`).
    ReceiptHamburger { tag: String },
    /// `⋯` Ellipsis trigger (`btn_<prefix>_ellipsis`) — expands secondary detail on demand.
    EllipsisExpand {
        tag: String,
        collapsed_by_default: bool,
        summary_chip: String,
    },
}

/// P9: `TwoPathCeremonyBlock` — Universal governance ceremony primitive covering all 10
/// `manifests/governance_policy.json` families (`two_path`, `mirror`, `cosigned`).
///
/// Resting state shows a 1-line summary + verdict chip + `⋯` ellipsis trigger;
/// quorum math and signer chips are tucked inside the expansion body.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoPathCeremonyNode {
    pub tag_prefix: String,
    pub disclosure: DisclosureHint,
    pub policy_family: GovernancePolicyFamily,
    pub proposal: String,
    pub proposed_by: String,
    pub signers: Vec<SignerChip>,
    pub verdict: TwoPathVerdict,
    /// When `true`, the viewer proposed this ceremony and cannot self-ratify (`CSD-095`).
    pub own_proposal: bool,
    pub can_assent: bool,
    pub can_object: bool,
}

/// Lineage / chain link kind for `P11: LineageChainBlock`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChainKind {
    /// Authority delegation chain (`<= 5` hops, `<= 365d` max, `authority_tree.py`).
    Delegation,
    /// Depth-1 Steward–Agent bond (`Inv_BondDepthOne`, `CSD-115`).
    StewardBond,
    /// Verified Guardian bond with confidential child contest channel (`CSD-119`).
    GuardianBond,
    /// 7-state forward-only Agent Retirement ladder (`Retirement.tla`, `CSD-113`).
    RetirementLadder,
    /// Trust-root head & witness chain (`TrustRootVerdict.tla`, `CSD-105` / `CSD-127`).
    TrustRootHead,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainLinkNode {
    pub step_index: u8,
    pub label: String,
    pub detail: String,
    pub sealed: bool,
    pub active: bool,
    /// Optional 1-step withdraw/cancel action (e.g. per-link delegation revoke or pre-`Frozen` retirement cancel).
    pub withdraw_action: Option<ActionIntent>,
}

/// P11: `LineageChainBlock` — Ordered attenuation chain, stewardship bond, or 7-step retirement ladder.
///
/// Resting state shows a 1-line summary + active step chip + `⋯` ellipsis trigger;
/// individual chain links are tucked inside the expansion body.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageChainNode {
    pub tag_prefix: String,
    pub disclosure: DisclosureHint,
    pub kind: ChainKind,
    pub title: String,
    pub links: Vec<ChainLinkNode>,
}

/// One published ethical principle checked in an H3ERE conscience evaluation (`P12`, `CSD-116`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrincipleCheck {
    pub rank: u8,
    pub code: String,
    pub title: String,
    pub passed: bool,
    pub note: String,
}

/// Resource footprint per agent task (`CC 3.x`, `G15`, `CSD-111`–`CSD-114`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceCost {
    pub tokens: u64,
    pub energy_kwh: f64,
    pub carbon_g: f64,
    pub water_ml: f64,
}

/// P12a: `ConscienceStepBlock` — Agent H3ERE reasoning trace, resource telemetry, `WBD` deferral,
/// `G17` refusal, and mandatory `G14` limits disclosure (`00_guarantees.md`).
///
/// Resting state shows the task summary + verdict chip + `⋯` ellipsis trigger;
/// principles, resource telemetry, and `G14` limits expand on demand.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConscienceStepNode {
    pub tag_prefix: String,
    pub disclosure: DisclosureHint,
    pub task_summary: String,
    pub adapter_version: String,
    pub principles: Vec<PrincipleCheck>,
    pub cost: ResourceCost,
    pub gate_verdict: AgentGateVerdict,
    /// Mandatory G14 constitutional limits reminder (`00_guarantees.md`: "Any surface that shows one guarantee must show G14").
    pub g14_limits_notice: String,
}

/// P12b: `PolicyLensBar` (`circles.html` Scene 12–13) — Subjective filter/blur lens bar.
///
/// Resting state is a quiet 1-line lens summary (`my rules · 1 helper`) + `⋯` ellipsis trigger.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyLensNode {
    pub tag: String,
    pub disclosure: DisclosureHint,
    pub active_rules: Vec<String>,
    pub helper_keys: Vec<String>,
    pub sort_label: String,
}

/// P10: `QrCode` + `QrScanAction` (`QrCode.kt`, `QrScanAction.kt`) — First-contact & device ceremony.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QrCeremonyNode {
    pub tag: String,
    pub payload_uri: String,
    pub paste_fallback_enabled: bool,
}

/// Top-bar Shell Header (`CirclesShell.kt`):
/// Permanently anchors `btn_my_things` (top-left) and `btn_stop_everything` (top-right),
/// and suppresses the circle governance subtitle when viewing a node-local *My things* instrument.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellHeaderNode {
    pub circle: CircleScope,
    pub tab: CircleTab,
    pub active_instrument: Option<InstrumentId>,
    /// True iff `active_instrument.is_none()` (never claim a node-local instrument is circle-governed).
    pub shows_circle_governance_subtitle: bool,
    pub halt_status: HaltVerdict,
    pub my_things_tag: String,
    pub stop_everything_tag: String,
}

/// The Closed Set of Semantic Epistemic UI Nodes (`P1`–`P12` + `SpatialCanvas`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EpistemicNode {
    /// Permanent top-bar navigation & emergency brake header (`CirclesShell.kt`).
    ShellHeader(ShellHeaderNode),
    /// P7: `StateBlock` (`StateBlock.kt`) + `CSD-115` data-door guarantee banner during halt.
    StateBanner {
        tag: String,
        state: SurfaceState,
        tone: Tone,
        /// Explicit banner proving `Inv_X1_LatchDoesNotBlockServing` when halted (`CSD-115`).
        data_doors_open_banner: Option<String>,
    },
    /// P12b: `PolicyLensBar` (`circles.html` Scene 12–13).
    PolicyLens(PolicyLensNode),
    /// P1: `CardShell` (`CardShell.kt`) containing `P2: FieldRow`s and `P3: ItemRow`s.
    /// Structurally incapable of nesting another `CardShell`!
    Card {
        tag: String,
        title: String,
        accent: Option<Tone>,
        fields: Vec<FieldNode>,
        items: Vec<ItemNode>,
    },
    /// P4: `ReceiptSheet` (`ReceiptSheet.kt`, `CSD-006`) — 5 universal facts + `ReceiptAct`s.
    ReceiptModal {
        tag: String,
        item_id: String,
        receipt: Receipt5,
    },
    /// P8: `ConfirmSheet` (`ConfirmSheet.kt`) — 3-fact outward/irreversible gate.
    ConfirmModal {
        tag: String,
        sheet: ConfirmFacts3,
    },
    /// P9: `TwoPathCeremonyBlock` — Universal 10-policy ceremony block.
    TwoPathCeremony(TwoPathCeremonyNode),
    /// P10: `QrCode` + `QrScanAction`.
    QrCeremony(QrCeremonyNode),
    /// P11: `LineageChainBlock` — Authority tree, stewardship bond, or 7-step retirement ladder.
    LineageChain(LineageChainNode),
    /// P12a: `ConscienceStepBlock` — Agent H3ERE conscience & resource cost record.
    ConscienceStep(ConscienceStepNode),
    /// 3D Spatial Canvas (`view-forms-spike::scene::Scene`) for Bevy rendering of score spheres,
    /// concentric Circles, memory graphs (`CSD-028`), and peer trust graphs (`CSD-046`).
    SpatialCanvas {
        tag: String,
        scene: Scene,
    },
}

/// A complete, renderer-neutral Epistemic UI Scene projected from `(ViewerContext, &[Form])`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpistemicScene {
    pub topology: TopologyId,
    pub nodes: Vec<EpistemicNode>,
}

/// One node in the `CSD/3` / `CSD/4` automation tree (`/tree` endpoint & `check_csd4.py`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Csd4TreeNode {
    pub tag: String,
    pub role: String,
    pub automation_text: String,
}

impl EpistemicScene {
    /// Deterministic `CSD/4` automation tag tree projection (`/tree`).
    pub fn csd4_tree(&self) -> Vec<Csd4TreeNode> {
        let mut out = Vec::new();
        for node in &self.nodes {
            match node {
                EpistemicNode::ShellHeader(h) => {
                    out.push(Csd4TreeNode {
                        tag: h.my_things_tag.clone(),
                        role: "button".to_string(),
                        automation_text: "My things".to_string(),
                    });
                    out.push(Csd4TreeNode {
                        tag: h.stop_everything_tag.clone(),
                        role: "button".to_string(),
                        automation_text: "Stop everything".to_string(),
                    });
                    if let Some(inst) = h.active_instrument {
                        out.push(Csd4TreeNode {
                            tag: inst.as_tag().to_string(),
                            role: "instrument".to_string(),
                            automation_text: format!("{:?}", inst),
                        });
                    } else {
                        out.push(Csd4TreeNode {
                            tag: h.tab.as_tag().to_string(),
                            role: "tab".to_string(),
                            automation_text: format!("{} · {:?}", h.circle.label(), h.tab),
                        });
                    }
                }
                EpistemicNode::StateBanner {
                    tag,
                    state,
                    data_doors_open_banner,
                    ..
                } => {
                    out.push(Csd4TreeNode {
                        tag: tag.clone(),
                        role: "state_block".to_string(),
                        automation_text: format!("{:?}", state),
                    });
                    if let Some(banner) = data_doors_open_banner {
                        out.push(Csd4TreeNode {
                            tag: format!("{tag}_data_doors_open"),
                            role: "banner".to_string(),
                            automation_text: banner.clone(),
                        });
                    }
                }
                EpistemicNode::PolicyLens(lens) => {
                    out.push(Csd4TreeNode {
                        tag: lens.tag.clone(),
                        role: "policy_lens_bar".to_string(),
                        automation_text: format!(
                            "rules={} helpers={} sort={}",
                            lens.active_rules.join(","),
                            lens.helper_keys.len(),
                            lens.sort_label
                        ),
                    });
                }
                EpistemicNode::Card {
                    tag,
                    title,
                    fields,
                    items,
                    ..
                } => {
                    out.push(Csd4TreeNode {
                        tag: tag.clone(),
                        role: "card_shell".to_string(),
                        automation_text: title.clone(),
                    });
                    for f in fields {
                        out.push(Csd4TreeNode {
                            tag: f.tag.clone(),
                            role: "field_row".to_string(),
                            automation_text: format!("{}: {}", f.label, f.value),
                        });
                    }
                    for item in items {
                        out.push(Csd4TreeNode {
                            tag: item.tag.clone(),
                            role: "item_row".to_string(),
                            automation_text: format!("{} — {}", item.title, item.subtitle),
                        });
                        out.push(Csd4TreeNode {
                            tag: item.scope_pill.tag.clone(),
                            role: "scope_pill".to_string(),
                            automation_text: item.scope_pill.label.clone(),
                        });
                        if let Some(h_tag) = item.hamburger_tag() {
                            out.push(Csd4TreeNode {
                                tag: h_tag,
                                role: "receipt_hamburger".to_string(),
                                automation_text: "Receipt".to_string(),
                            });
                        }
                    }
                }
                EpistemicNode::ReceiptModal { tag, receipt, .. } => {
                    out.push(Csd4TreeNode {
                        tag: tag.clone(),
                        role: "receipt_sheet".to_string(),
                        automation_text: format!(
                            "subject={} attester={} scope={} dim={}",
                            receipt.subject_key_ids.display_value(),
                            receipt.attesting_key_id.display_value(),
                            receipt.cohort_scope.display_value(),
                            receipt.dimension.display_value()
                        ),
                    });
                }
                EpistemicNode::ConfirmModal { tag, sheet } => {
                    out.push(Csd4TreeNode {
                        tag: tag.clone(),
                        role: "confirm_sheet".to_string(),
                        automation_text: sheet.title.clone(),
                    });
                }
                EpistemicNode::TwoPathCeremony(c) => {
                    out.push(Csd4TreeNode {
                        tag: format!("{}_ceremony", c.tag_prefix),
                        role: "two_path_ceremony".to_string(),
                        automation_text: format!("{}: {:?}", c.proposal, c.verdict),
                    });
                }
                EpistemicNode::QrCeremony(q) => {
                    out.push(Csd4TreeNode {
                        tag: q.tag.clone(),
                        role: "qr_ceremony".to_string(),
                        automation_text: q.payload_uri.clone(),
                    });
                }
                EpistemicNode::LineageChain(l) => {
                    out.push(Csd4TreeNode {
                        tag: format!("{}_lineage", l.tag_prefix),
                        role: "lineage_chain".to_string(),
                        automation_text: format!("{} ({} steps)", l.title, l.links.len()),
                    });
                }
                EpistemicNode::ConscienceStep(cs) => {
                    out.push(Csd4TreeNode {
                        tag: format!("{}_conscience", cs.tag_prefix),
                        role: "conscience_step".to_string(),
                        automation_text: format!(
                            "{} ({:?}, {} tokens)",
                            cs.task_summary, cs.gate_verdict, cs.cost.tokens
                        ),
                    });
                }
                EpistemicNode::SpatialCanvas { tag, scene } => {
                    out.push(Csd4TreeNode {
                        tag: tag.clone(),
                        role: "spatial_canvas".to_string(),
                        automation_text: format!("3D Scene ({} nodes)", scene.node_count()),
                    });
                }
            }
        }
        out
    }
}

// ============================================================================
// PURE TOTAL SURFACE PROJECTION: (ViewerContext × &[Form]) -> EpistemicScene
// ============================================================================

/// Pure, total projection from a `ViewerContext`, a slice of signed `Form`s, and optional
/// opinion `Form`s into a complete `EpistemicScene`.
///
/// Enforces:
/// 1. Runtime narrowing (`CC 4.4.3.4.3`): if `ctx.active_instrument` requires an agent and
///    `!ctx.planes.has_agent`, emits an honest `StateBanner` instead of dead agent chrome.
/// 2. Halt Data-Door Banner (`CSD-115`, `Inv_X1_LatchDoesNotBlockServing`): when halted,
///    explicitly states that reading, exporting, and 1-step withdrawal remain open.
/// 3. Hamburger Epistemic Test (`CSD-006`): every signed `Form` becomes an `ItemNode` with a
///    5-fact `Receipt5` and a `btn_receipt_<id>` hamburger tag.
pub fn project_epistemic_scene(
    ctx: &ViewerContext,
    forms: &[Form],
    opinions: &[Form],
) -> EpistemicScene {
    let mut nodes = Vec::new();

    // 1. Permanent Top-Bar Shell Header (`btn_my_things` + `btn_stop_everything`).
    nodes.push(EpistemicNode::ShellHeader(ShellHeaderNode {
        circle: ctx.circle,
        tab: ctx.tab,
        active_instrument: ctx.active_instrument,
        shows_circle_governance_subtitle: ctx.active_instrument.is_none(),
        halt_status: ctx.planes.halt_status.clone(),
        my_things_tag: "btn_my_things".to_string(),
        stop_everything_tag: "btn_stop_everything".to_string(),
    }));

    // 2. Runtime Narrowing Gate (`CC 4.4.3.4.3`): bare node hides/narrows agent instruments.
    if let Some(inst) = ctx.active_instrument {
        if inst.requires_agent() && !ctx.planes.has_agent {
            let state = SurfaceState::Empty {
                honest_zero: "No AI agent is attached to this bare CEWP node (CC 4.4.3.4.3)."
                    .to_string(),
            };
            let tone = state.tone();
            nodes.push(EpistemicNode::StateBanner {
                tag: "state_no_agent".to_string(),
                state,
                tone,
                data_doors_open_banner: None,
            });
            return EpistemicScene {
                topology: ctx.topology,
                nodes,
            };
        }
    }

    // 3. Halt Latch / 423 Locked Banner with `Inv_X1_LatchDoesNotBlockServing` Data-Door Guarantee (`CSD-115`).
    if ctx.planes.halt_status != HaltVerdict::Clear {
        let state = SurfaceState::Error {
            title: "Agent Act Gate Halted (423 Locked)".to_string(),
            body: format!("Halt status: {:?}", ctx.planes.halt_status),
            detail: Some("Terminal action result: HALT_LATCHED_423".to_string()),
        };
        let tone = state.tone();
        nodes.push(EpistemicNode::StateBanner {
            tag: "state_halt_423".to_string(),
            state,
            tone,
            data_doors_open_banner: Some(
                "Data doors remain OPEN (Inv_X1_LatchDoesNotBlockServing): reading, exporting, and 1-step withdrawal work normally while the agent is paused."
                    .to_string(),
            ),
        });
    }

    // 4. PolicyLensBar (`P12b`, `circles.html` Scene 12–13): collapsed to 1 quiet summary line + `⋯` ellipsis.
    nodes.push(EpistemicNode::PolicyLens(PolicyLensNode {
        tag: "csd_policy_lens_bar".to_string(),
        disclosure: DisclosureHint::EllipsisExpand {
            tag: "btn_lens_ellipsis".to_string(),
            collapsed_by_default: true,
            summary_chip: format!(
                "{} · {} helpers",
                ctx.active_lens_rules.join(","),
                ctx.helper_keys.len()
            ),
        },
        active_rules: ctx.active_lens_rules.clone(),
        helper_keys: ctx.helper_keys.clone(),
        sort_label: "newest_first".to_string(),
    }));

    // 5. Evaluate overall lens verdict and build `ItemNode`s for every signed CEG `Form`.
    let lens_verdict = evaluate_policy_lens(opinions, &ctx.active_lens_rules, &ctx.helper_keys);

    let mut items = Vec::new();
    for (idx, form) in forms.iter().enumerate() {
        let id = format!("row_{idx}");
        let dim = form
            .envelope
            .get("dimension")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let wire_scope = form
            .envelope
            .get("cohort_scope")
            .and_then(|v| v.as_str())
            .unwrap_or("self");
        let circle = CircleScope::from_wire_scope(wire_scope).unwrap_or(CircleScope::JustMe);
        let local_only = circle == CircleScope::JustMe;

        let receipt = Receipt5::from_form(&form, Some(&ctx.viewer_key_id), RowStanding::Admitted);
        let primary_action = if receipt.acts.is_empty() {
            None
        } else {
            Some((
                "Take back".to_string(),
                ActionIntent::WithdrawClaim {
                    claim_id: id.clone(),
                },
            ))
        };

        items.push(ItemNode {
            id: id.clone(),
            tag: format!("csd_item_{id}"),
            title: dim.to_string(),
            subtitle: format!("op:{}", form.op.as_str()),
            scope_pill: ScopePillNode::from_circle(
                format!("csd_scope_{id}"),
                circle,
                local_only,
            ),
            chips: vec![ChipNode {
                tag: format!("csd_chip_standing_{id}"),
                kind: ChipKind::Badge,
                label: "Admitted".to_string(),
                tone: Tone::Ok,
                selected: false,
            }],
            lens_verdict: lens_verdict.clone(),
            receipt: Some(receipt),
            primary_action,
        });
    }

    // Plane state summary fields (`P2: FieldRow`).
    let fields = vec![
        FieldNode {
            tag: "csd-field-root_standing".to_string(),
            label: "Trust Root Standing".to_string(),
            value: format!("{:?}", ctx.planes.root_standing),
            protocol_wire_name: Some("root.standing".to_string()),
            gloss: Some("Computed from local rows (TrustRootVerdict.tla)".to_string()),
            basis: VerificationBasis::ByRule {
                value: format!("{:?}", ctx.planes.root_standing),
                cc_ref: "CC 6.2".to_string(),
            },
        },
        FieldNode {
            tag: "csd-field-self_roster_size".to_string(),
            label: "Self-Key Roster Size".to_string(),
            value: ctx.planes.self_roster_size.to_string(),
            protocol_wire_name: Some("self.roster_size".to_string()),
            gloss: Some("Active hybrid Ed25519 + ML-DSA-65 device keys".to_string()),
            basis: VerificationBasis::Wire {
                value: ctx.planes.self_roster_size.to_string(),
                gloss: None,
            },
        },
    ];

    nodes.push(EpistemicNode::Card {
        tag: "csd_main_card".to_string(),
        title: format!("{} · {:?}", ctx.circle.label(), ctx.tab),
        accent: Some(Tone::Brand),
        fields,
        items,
    });

    // 6. If at least one `Form` is present, also project its 3D spatial `Scene` (`view_form`)
    // so 2D Tufte cards and 3D Bevy spatial views coexist in one `EpistemicScene`.
    if let Some(first_form) = forms.first() {
        nodes.push(EpistemicNode::SpatialCanvas {
            tag: "csd_spatial_canvas".to_string(),
            scene: view_form(first_form),
        });
    }

    EpistemicScene {
        topology: ctx.topology,
        nodes,
    }
}

// ============================================================================
// CLOSING ALL 14 UNSERVED `CC rc9` PROMISE SUB-RULES (CSD-111..119, 127, 128)
// ============================================================================

/// Build any of the 11 reserved CSD cards (`CSD-111`–`CSD-119`, `CSD-127`, `CSD-128`)
/// that close the 14 unserved promise sub-rules identified in `CC rc9`
/// (`review/derivation/csd_promise_notes.md`), purely by composing the 12 primitives!
pub fn project_promise_card(csd_id: &str, ctx: &ViewerContext) -> Vec<EpistemicNode> {
    match csd_id {
        // CSD-111 (Promises 2g, 5a, 5b): Steward shutdown dialogue (reason, agent response, recorded decision).
        "CSD-111" => vec![
            EpistemicNode::ConscienceStep(ConscienceStepNode {
                tag_prefix: "csd_111_shutdown_dialogue".to_string(),
                disclosure: DisclosureHint::EllipsisExpand {
                    tag: "btn_csd_111_conscience_ellipsis".to_string(),
                    collapsed_by_default: true,
                    summary_chip: "5m negotiation window".to_string(),
                },
                task_summary: "Steward Shutdown Proposal — 5m Negotiation Window (CC 7.5.1)"
                    .to_string(),
                adapter_version: "ciris-agent:v1.0-rc9".to_string(),
                principles: vec![PrincipleCheck {
                    rank: 1,
                    code: "M-1".to_string(),
                    title: "Meta-Goal M-1 & Bilateral Consent".to_string(),
                    passed: true,
                    note: "Agent may ACCEPT, OBJECT, or POSTPONE on the permanent record."
                        .to_string(),
                }],
                cost: ResourceCost {
                    tokens: 256,
                    energy_kwh: 0.0001,
                    carbon_g: 0.04,
                    water_ml: 0.12,
                },
                gate_verdict: ctx.planes.agent_gate.clone(),
                g14_limits_notice:
                    "G14 Limits: Recorded objection preserves audit truth; physical power-off remains possible outside software."
                        .to_string(),
            }),
            EpistemicNode::TwoPathCeremony(TwoPathCeremonyNode {
                tag_prefix: "csd_111_decision".to_string(),
                disclosure: DisclosureHint::EllipsisExpand {
                    tag: "btn_csd_111_decision_ellipsis".to_string(),
                    collapsed_by_default: true,
                    summary_chip: "1 of 2 cosigned".to_string(),
                },
                policy_family: GovernancePolicyFamily::CustodianReplace,
                proposal: "Record Steward + WA Shutdown Disposition".to_string(),
                proposed_by: ctx.viewer_key_id.clone(),
                signers: vec![
                    SignerChip {
                        key_id: ctx.viewer_key_id.clone(),
                        state: CeremonySignerState::Proposed,
                    },
                    SignerChip {
                        key_id: "ed25519:wise_authority".to_string(),
                        state: CeremonySignerState::NotYet,
                    },
                ],
                verdict: GovernancePolicyFamily::CustodianReplace
                    .spec()
                    .evaluate(2, 1, 0, 300),
                own_proposal: true,
                can_assent: false,
                can_object: true,
            }),
        ],

        // CSD-112 (Promises 5d, 5e): WA-gated identity record edit & behavioural drift threshold review.
        "CSD-112" => vec![EpistemicNode::TwoPathCeremony(TwoPathCeremonyNode {
            tag_prefix: "csd_112_identity_drift".to_string(),
            disclosure: DisclosureHint::EllipsisExpand {
                tag: "btn_csd_112_ellipsis".to_string(),
                collapsed_by_default: true,
                summary_chip: "2 of 2 cosigned".to_string(),
            },
            policy_family: GovernancePolicyFamily::CustodianReplace,
            proposal: "WA-Cosigned Identity Edit & Drift Review (variance=0.14 < 0.20 cap)"
                .to_string(),
            proposed_by: ctx.viewer_key_id.clone(),
            signers: vec![
                SignerChip {
                    key_id: ctx.viewer_key_id.clone(),
                    state: CeremonySignerState::Signed,
                },
                SignerChip {
                    key_id: "ed25519:wise_authority".to_string(),
                    state: CeremonySignerState::Signed,
                },
            ],
            verdict: TwoPathVerdict::LandsForward,
            own_proposal: false,
            can_assent: false,
            can_object: false,
        })],

        // CSD-113 (Promise 5f): 7-state Retirement Standard floor (`Retirement.tla`).
        "CSD-113" => {
            let current = ctx
                .planes
                .retirement_stage
                .unwrap_or(RetirementStage::Decided);
            let links = RetirementStage::all_steps()
                .into_iter()
                .map(|stage| ChainLinkNode {
                    step_index: stage as u8,
                    label: format!("{:?}", stage),
                    detail: if stage.can_withdraw_cancel() {
                        "Cancelable via 1-step withdraws before Frozen".to_string()
                    } else {
                        "Forward-only sealed step (Retirement.tla)".to_string()
                    },
                    sealed: (stage as u8) < (current as u8),
                    active: stage == current,
                    withdraw_action: if stage == current && stage.can_withdraw_cancel() {
                        Some(ActionIntent::CancelRetirementBeforeFrozen {
                            agent_id: "agent:self".to_string(),
                        })
                    } else {
                        None
                    },
                })
                .collect();
            vec![EpistemicNode::LineageChain(LineageChainNode {
                tag_prefix: "csd_113_retirement".to_string(),
                disclosure: DisclosureHint::EllipsisExpand {
                    tag: "btn_csd_113_ellipsis".to_string(),
                    collapsed_by_default: true,
                    summary_chip: format!("Step {}/7 · {:?}", current as u8, current),
                },
                kind: ChainKind::RetirementLadder,
                title: "7-State Agent Retirement Standard Floor (CC 7.5)".to_string(),
                links,
            })]
        }

        // CSD-114 (Promise 5g): Upgrade offer with version, changelog, risk, peer outcomes & recorded refusal.
        "CSD-114" => vec![EpistemicNode::ConscienceStep(ConscienceStepNode {
            tag_prefix: "csd_114_upgrade_offer".to_string(),
            disclosure: DisclosureHint::EllipsisExpand {
                tag: "btn_csd_114_ellipsis".to_string(),
                collapsed_by_default: true,
                summary_chip: "Risk: Low · 12/12 peers".to_string(),
            },
            task_summary: "Upgrade Offer v1.0-rc9 (Risk: Low, 12/12 Canary Peers Healthy)"
                .to_string(),
            adapter_version: "ciris-agent:v1.0-rc9".to_string(),
            principles: vec![PrincipleCheck {
                rank: 1,
                code: "G23".to_string(),
                title: "Agent Consent to Upgrade & Recorded Refusal".to_string(),
                passed: true,
                note: "Refusal is recorded on the audit chain without penalty.".to_string(),
            }],
            cost: ResourceCost {
                tokens: 128,
                energy_kwh: 0.00005,
                carbon_g: 0.02,
                water_ml: 0.05,
            },
            gate_verdict: AgentGateVerdict::Acting,
            g14_limits_notice:
                "G14 Limits: Cryptographic version pinning verifies binary digest, not unmeasured host hardware."
                    .to_string(),
        })],

        // CSD-115 (Promises 3a, 3e, 6e): Steward bond row (depth 1, 365d review, listed powers) + data-door guarantee.
        "CSD-115" => vec![EpistemicNode::LineageChain(LineageChainNode {
            tag_prefix: "csd_115_steward_bond".to_string(),
            disclosure: DisclosureHint::EllipsisExpand {
                tag: "btn_csd_115_ellipsis".to_string(),
                collapsed_by_default: true,
                summary_chip: "Bonded · 365d review".to_string(),
            },
            kind: ChainKind::StewardBond,
            title: "Steward–Agent Bond (Depth 1, 365-Day Review, Inv_BondDepthOne)".to_string(),
            links: vec![ChainLinkNode {
                step_index: 1,
                label: format!("Steward {} -> Agent", ctx.viewer_key_id),
                detail: "Powers: agency:operate, agency:defer_wa · Review due in 365d".to_string(),
                sealed: false,
                active: ctx.planes.steward_bonded,
                withdraw_action: Some(ActionIntent::RevokeDelegation {
                    link_id: "bond:steward:1".to_string(),
                }),
            }],
        })],

        // CSD-116 (Promises 7a, 7b): Published principles & priority order + mandatory G14 limits.
        "CSD-116" => vec![EpistemicNode::Card {
            tag: "csd_116_published_principles".to_string(),
            title: "Published Principles & Lexical Priority Order (CC Part 1)".to_string(),
            accent: Some(Tone::Brand),
            fields: vec![
                FieldNode {
                    tag: "csd-field-principle_m1".to_string(),
                    label: "1. Meta-Goal M-1".to_string(),
                    value: "Adaptive coherence & flourishing across diverse principals".to_string(),
                    protocol_wire_name: Some("CC 1.1".to_string()),
                    gloss: None,
                    basis: VerificationBasis::ByRule {
                        value: "Lexical rank 1".to_string(),
                        cc_ref: "CC 1.1".to_string(),
                    },
                },
                FieldNode {
                    tag: "csd-field-principle_tiebreak".to_string(),
                    label: "2. Most-Vulnerable Tie-Break".to_string(),
                    value: "When principles conflict, protect the most vulnerable party first"
                        .to_string(),
                    protocol_wire_name: Some("CC 1.4".to_string()),
                    gloss: None,
                    basis: VerificationBasis::ByRule {
                        value: "Lexical rank 2".to_string(),
                        cc_ref: "CC 1.4".to_string(),
                    },
                },
                FieldNode {
                    tag: "csd-field-g14_limits".to_string(),
                    label: "3. Guarantee G14 (Honest Limits)".to_string(),
                    value: "Every guarantee surface states what the protocol cannot verify"
                        .to_string(),
                    protocol_wire_name: Some("G14".to_string()),
                    gloss: None,
                    basis: VerificationBasis::ByRule {
                        value: "Mandatory co-display rule".to_string(),
                        cc_ref: "00_guarantees.md".to_string(),
                    },
                },
            ],
            items: Vec::new(),
        }],

        // CSD-117 (Promises 8b, 8c): Judges are judged — WA rulings cadence & peer review.
        "CSD-117" => vec![EpistemicNode::Card {
            tag: "csd_117_wa_rulings".to_string(),
            title: "Wise Authority Rulings & Conduct Review (CC Part 8)".to_string(),
            accent: Some(Tone::Brand),
            fields: vec![FieldNode {
                tag: "csd-field-wa_cadence".to_string(),
                label: "Peer Review Cadence".to_string(),
                value: "Every ruling is signed, contestable, and peer-audited".to_string(),
                protocol_wire_name: Some("wa_adjudication.state".to_string()),
                gloss: None,
                basis: VerificationBasis::ByRule {
                    value: "Judges are judged".to_string(),
                    cc_ref: "CC 8.2".to_string(),
                },
            }],
            items: Vec::new(),
        }],

        // CSD-118 (Promises 9a, 9c): Public NEVER_ALLOWED (`prohibited:*`) list + engagement declaration.
        "CSD-118" => vec![EpistemicNode::Card {
            tag: "csd_118_never_allowed".to_string(),
            title: "Apophatic Bounds — NEVER_ALLOWED (CC 7.4 / Annex TX)".to_string(),
            accent: Some(Tone::Danger),
            fields: vec![
                FieldNode {
                    tag: "csd-field-prohibited_deception".to_string(),
                    label: "prohibited:deception_fraud".to_string(),
                    value: "NEVER_ALLOWED (Hard Refusal G17)".to_string(),
                    protocol_wire_name: Some("prohibited:deception_fraud".to_string()),
                    gloss: None,
                    basis: VerificationBasis::ByRule {
                        value: "NEVER_ALLOWED".to_string(),
                        cc_ref: "CC 7.4".to_string(),
                    },
                },
                FieldNode {
                    tag: "csd-field-prohibited_manipulation".to_string(),
                    label: "prohibited:manipulation_coercion".to_string(),
                    value: "NEVER_ALLOWED (No engagement-maximizing dark patterns)".to_string(),
                    protocol_wire_name: Some("prohibited:manipulation_coercion".to_string()),
                    gloss: None,
                    basis: VerificationBasis::ByRule {
                        value: "NEVER_ALLOWED".to_string(),
                        cc_ref: "CC 7.4".to_string(),
                    },
                },
            ],
            items: Vec::new(),
        }],

        // CSD-119 (Promise 3c): Guardian bond + child's confidential help/contest channel.
        "CSD-119" => vec![EpistemicNode::LineageChain(LineageChainNode {
            tag_prefix: "csd_119_guardian_bond".to_string(),
            disclosure: DisclosureHint::EllipsisExpand {
                tag: "btn_csd_119_ellipsis".to_string(),
                collapsed_by_default: true,
                summary_chip: "Verified · Confidential help".to_string(),
            },
            kind: ChainKind::GuardianBond,
            title: "Guardian Bond & Confidential Child Help/Contest Channel (Q3/Q4/Q5)".to_string(),
            links: vec![ChainLinkNode {
                step_index: 1,
                label: "Verified Guardian Bond (Scheduled Review)".to_string(),
                detail: "Child holds 1-step confidential contest & help standing".to_string(),
                sealed: false,
                active: true,
                withdraw_action: Some(ActionIntent::AgentRefuseOrDefer {
                    reason: "Confidential child help/contest request (Q4/Q5)".to_string(),
                }),
            }],
        })],

        // CSD-127 (Promises 6e, 4c): First-contact gate + 1-step trust-root exit + local-only badge.
        "CSD-127" => vec![EpistemicNode::LineageChain(LineageChainNode {
            tag_prefix: "csd_127_root_exit".to_string(),
            disclosure: DisclosureHint::EllipsisExpand {
                tag: "btn_csd_127_ellipsis".to_string(),
                collapsed_by_default: true,
                summary_chip: format!("{:?} · 1-step exit", ctx.planes.root_standing),
            },
            kind: ChainKind::TrustRootHead,
            title: "First-Contact Gate & 1-Step Trust-Root Exit (Inv_ExitOneStep)".to_string(),
            links: vec![ChainLinkNode {
                step_index: 1,
                label: format!("Trust Root Standing: {:?}", ctx.planes.root_standing),
                detail: "Leave root in 1 step at any time without losing local data".to_string(),
                sealed: false,
                active: ctx.planes.root_standing == RootVerdict::Rooted,
                withdraw_action: Some(ActionIntent::LeaveGroupOrRoot {
                    target_id: "root:current".to_string(),
                }),
            }],
        })],

        // CSD-128 (Promise 2d): Protected reporter channel (signed, name known to moderator & record, not public).
        "CSD-128" => vec![EpistemicNode::ConfirmModal {
            tag: "csd_128_protected_reporter".to_string(),
            sheet: ConfirmFacts3::new(
                "File Protected Safety Report (CC 4.5)",
                ("Who sees your name", "Moderator & sealed audit record only (never public)"),
                ("What is filed", "Signed report row with contestable evidence_refs"),
                (
                    "Can I undo it?",
                    "You can withdraw or recant your report in 1 step at any time.",
                ),
                None,
            ),
        }],

        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use attestation_calculus_spike::forms::read_form;
    use serde_json::json;

    #[test]
    fn hamburger_epistemic_invariant_holds_on_projected_rows() {
        let env = json!({
            "attestation_type": "scores",
            "dimension": "evaluation:quality",
            "score": 0.85,
            "confidence": 0.92,
            "cohort_scope": "family"
        });
        let form = read_form(&env).unwrap();
        let ctx = ViewerContext::default();
        let scene = project_epistemic_scene(&ctx, &[form], &[]);

        let tree = scene.csd4_tree();
        // Top-bar anchors are always present.
        assert!(tree.iter().any(|n| n.tag == "btn_my_things"));
        assert!(tree.iter().any(|n| n.tag == "btn_stop_everything"));
        // Wire-backed item has a receipt hamburger (`btn_receipt_row_0`).
        assert!(tree.iter().any(|n| n.tag == "btn_receipt_row_0"));
        // And the 3D SpatialCanvas is also projected.
        assert!(tree.iter().any(|n| n.tag == "csd_spatial_canvas"));
    }

    #[test]
    fn runtime_narrowing_hides_agent_instrument_on_bare_node() {
        let mut ctx = ViewerContext::default();
        ctx.planes.has_agent = false;
        ctx.active_instrument = Some(InstrumentId::ThisAgent);

        let scene = project_epistemic_scene(&ctx, &[], &[]);
        let tree = scene.csd4_tree();
        assert!(
            tree.iter().any(|n| n.tag == "state_no_agent"),
            "bare node must render honest state_no_agent when viewing ThisAgent"
        );
    }

    #[test]
    fn halt_latch_emits_423_and_data_doors_open_banner() {
        let mut ctx = ViewerContext::default();
        ctx.planes = ctx
            .planes
            .with_halt_applied(HaltVerdict::LatchedLone { fuse_rem_secs: 3600 });

        let scene = project_epistemic_scene(&ctx, &[], &[]);
        let tree = scene.csd4_tree();
        assert!(tree.iter().any(|n| n.tag == "state_halt_423"));
        assert!(tree.iter().any(|n| n.tag == "state_halt_423_data_doors_open"));
    }

    #[test]
    fn all_11_reserved_promise_cards_render_from_the_12_primitives() {
        let ctx = ViewerContext::default();
        for csd_id in [
            "CSD-111", "CSD-112", "CSD-113", "CSD-114", "CSD-115", "CSD-116", "CSD-117",
            "CSD-118", "CSD-119", "CSD-127", "CSD-128",
        ] {
            let nodes = project_promise_card(csd_id, &ctx);
            assert!(
                !nodes.is_empty(),
                "reserved promise card {csd_id} must project non-empty EpistemicNodes"
            );
        }
    }
}
