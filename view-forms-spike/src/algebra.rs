//! algebra.rs — The Formal `CC rc9` UI Algebra: Claims (C) × Contexts (X) × Actions (A) × Conclusions (V).
//!
//! Implements the closed mathematical structures derived from `CIRISConstitution` (`rc9` @ `017360e8`):
//!
//! 1. **Claims (`C`)**:
//!    - The closed `1+4` CEG operators (`Op`) + derived `carries` (`Verb`).
//!    - The `12 × 6` `(ChangeKind × Verb)` occupancy grid (`48 / 72` legal cells across
//!      `78 THING`, `76 ACCOUNT`, `15 RECORD` families).
//!    - The 5 Universal Receipt Facts (`CSD-006`) with epistemic `VerificationBasis`
//!      (`Wire | ByRule(cc_ref) | NotSent`).
//! 2. **Contexts (`X`)**:
//!    - The 7→5 `CohortScope` fold (`CircleScope`: *Just me · Family · Neighbours ·
//!      Communities & Businesses · Everyone*).
//!    - The 7 Circle Tabs + 6 *My things* Instruments + 2 Top-Bar Invariants (`My things`, `Stop everything`).
//!    - The 20 closed `CSD/4` topologies (`TopologyId::T0..T19`).
//!    - The 9-plane normalized state vector (`StatePlaneVector`, `formal/STATE_TABLE.tsv`).
//! 3. **Actions (`A`)**:
//!    - The 8 canonical transition archetypes (`ActionArchetype`).
//!    - The **Directional Asymmetry Law** (`GateRequirement`): outward/widening/irreversible
//!      actions require `ConfirmFacts3` (exactly 3 facts), whereas inward/narrowing/exit/halt/refuse
//!      execute in 1 step (`ImmediateOneStep`).
//!    - The **10 Governance Decision Families** (`manifests/governance_policy.json`,
//!      `formal/two_path/TwoPath.tla`, `PolicyCorridor.lean`).
//! 4. **Conclusions (`V`)**:
//!    - Closed formal verdicts per plane (`RootVerdict`, `HaltVerdict`, `AgentGateVerdict`,
//!      `TwoPathVerdict`, `RowStanding`, `PolicyLensVerdict`, `RetirementStage`, `SurfaceState`).

use attestation_calculus_spike::forms::{Form, Op};
use serde::{Deserialize, Serialize};

// ============================================================================
// 1. CLAIMS (C) — 12 Change Kinds × 6 Verbs × 3 Ontologies + 5 Receipt Facts
// ============================================================================

/// The 3 ontological classes of the 169 namespace families (`review/derivation/family_kind_grid.md`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OntologyClass {
    /// 78 families: files, chat messages, keys, group structures, charters.
    Thing,
    /// 76 families: scores, votes, consent grants, delegations, confidence ratings.
    Account,
    /// 15 families: audit chain entries, transparency STHs, tombstones, delivery receipts.
    Record,
}

/// The 12 Change Kinds partitioning all 169 namespace families (`CC rc9` `family_kind_grid.md`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangeKind {
    /// 1. Votes, credits, need weightings (5 families; accrue, never delegated or superseded).
    Priorities,
    /// 2. Accord, consent, duty, moderation, quarantine, watchlist, mesh config (33 families).
    Rules,
    /// 3. Content warnings (`cw_class:*`), register, tone (3 families).
    Manner,
    /// 4. Keys, ownership, occurrences, hardware custody, age assurance (16 families; never recanted).
    Identity,
    /// 5. Capacity, coherence standing, trust, reputation (15 families; no-self-emit rule).
    Confidence,
    /// 6. Chat messages, files (`holds_bytes:*`), notes, contacts, observations (46 families).
    Facts,
    /// 7. Health, transport, peer reachability, resource telemetry (12 families; never recanted).
    Circumstances,
    /// 8. H3ERE DMA steps, deferral tickets, retirement ladder, membership ceremony (9 families).
    Process,
    /// 9. Calibration, detectors, ratchet flags (5 families).
    Model,
    /// 10. Households, communities, affiliations, rooms, authority trees (8 families).
    Structure,
    /// 11. Charter, creator intent, constitutional foundations (2 families).
    Premises,
    /// 12. Audit chain, transparency STH, tombstones, delivery receipts (15 families).
    Record,
}

/// The 6 verbs of the `CC rc9` derivation grid: the 1+4 CEG wire operators plus derived `Carries`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Verb {
    /// `scores` — assert a claim.
    Attest,
    /// `delegates_to` — grant bounded authority (<= 5 hops, <= 365 days).
    Authorize,
    /// `supersedes` — replace a prior row by the same attester.
    Replace,
    /// `withdraws` — retract forward-only under CC 2.4.1.1 Rules 0-5.
    Withdraw,
    /// `recants` — producer confesses a prior assertion was false/erroneous on the record.
    Recant,
    /// Derived cross-kind effect edge.
    Carries,
}

impl From<Op> for Verb {
    fn from(op: Op) -> Self {
        match op {
            Op::Scores => Verb::Attest,
            Op::DelegatesTo => Verb::Authorize,
            Op::Supersedes => Verb::Replace,
            Op::Withdraws => Verb::Withdraw,
            Op::Recants => Verb::Recant,
        }
    }
}

impl ChangeKind {
    /// Classify a wire `dimension` into one of the 12 Change Kinds.
    pub fn from_dimension(dim: &str) -> Self {
        let prefix = dim.split(':').next().unwrap_or(dim);
        match prefix {
            "priority" | "vote" | "credit" | "need" => ChangeKind::Priorities,
            "accord" | "consent" | "duty" | "moderation" | "quarantine" | "watchlist" | "rule"
            | "prohibited" | "licensure" | "key_grant" => ChangeKind::Rules,
            "cw_class" | "manner" | "register" | "tone" => ChangeKind::Manner,
            "key" | "identity" | "ownership" | "occurrence" | "custody_hw" | "age" | "device"
            | "self_roster" => ChangeKind::Identity,
            "evaluation" | "capacity" | "coherence" | "trust" | "reputation" | "confidence" => {
                ChangeKind::Confidence
            }
            "circumstance" | "health" | "transport" | "reachability" | "telemetry" | "resource" => {
                ChangeKind::Circumstances
            }
            "process" | "dma" | "wbd" | "retirement" | "membership" | "objection"
            | "wa_adjudication" => ChangeKind::Process,
            "model" | "calibration" | "detector" | "ratchet" => ChangeKind::Model,
            "structure" | "household" | "community" | "affiliation" | "room" | "authority"
            | "bond" => ChangeKind::Structure,
            "premise" | "charter" | "intent" | "principle" => ChangeKind::Premises,
            "record" | "audit" | "sth" | "tombstone" | "receipt" | "revocation" | "slashing" => {
                ChangeKind::Record
            }
            _ => ChangeKind::Facts,
        }
    }

    /// Map this Change Kind to its primary Ontological Class (`THING`, `ACCOUNT`, `RECORD`).
    pub fn ontology_class(self) -> OntologyClass {
        match self {
            ChangeKind::Identity
            | ChangeKind::Facts
            | ChangeKind::Structure
            | ChangeKind::Premises => OntologyClass::Thing,
            ChangeKind::Priorities
            | ChangeKind::Rules
            | ChangeKind::Manner
            | ChangeKind::Confidence
            | ChangeKind::Circumstances
            | ChangeKind::Process
            | ChangeKind::Model => OntologyClass::Account,
            ChangeKind::Record => OntologyClass::Record,
        }
    }

    /// The exact `12 × 6` (`48 / 72`) occupancy law from `review/derivation/family_kind_grid.md`.
    ///
    /// For example:
    /// - `Identity`, `Circumstances`, and `Process` cannot be `Recant`ed (`recants = 0`).
    /// - Only `Rules` and `Identity` admit `Authorize` (`delegates_to`).
    /// - `Priorities` and `Model` cannot be `Replace`d (`supersedes = 0`).
    pub fn admits_verb(self, verb: Verb) -> bool {
        match (self, verb) {
            // Every kind admits Attest (`scores`) and Withdraw (`withdraws`).
            (_, Verb::Attest) | (_, Verb::Withdraw) => true,
            // Only Rules and Identity admit Authorize (`delegates_to`).
            (ChangeKind::Rules | ChangeKind::Identity, Verb::Authorize) => true,
            (_, Verb::Authorize) => false,
            // Priorities and Model do not admit Replace (`supersedes`).
            (ChangeKind::Priorities | ChangeKind::Model, Verb::Replace) => false,
            (_, Verb::Replace) => true,
            // Identity, Circumstances, and Process do not admit Recant (`recants`).
            (
                ChangeKind::Identity | ChangeKind::Circumstances | ChangeKind::Process,
                Verb::Recant,
            ) => false,
            (_, Verb::Recant) => true,
            // Carries is occupied for Rules, Confidence, and Record.
            (ChangeKind::Rules | ChangeKind::Confidence | ChangeKind::Record, Verb::Carries) => {
                true
            }
            (_, Verb::Carries) => false,
        }
    }
}

/// Epistemic provenance of a Receipt fact (`CSD-006`, `Receipt.kt`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationBasis {
    /// Fact was sent directly on the wire by the node.
    Wire { value: String, gloss: Option<String> },
    /// Fact is fixed by a constitutional rule (`cc_ref`, e.g. `"CC 2.3.3"`).
    ByRule { value: String, cc_ref: String },
    /// Fact was omitted by the node — must be rendered in `Tone::Danger`, never guessed or hidden.
    NotSent,
}

impl VerificationBasis {
    pub fn display_value(&self) -> &str {
        match self {
            VerificationBasis::Wire { value, .. } => value.as_str(),
            VerificationBasis::ByRule { value, .. } => value.as_str(),
            VerificationBasis::NotSent => "NOT SENT BY NODE",
        }
    }

    pub fn is_not_sent(&self) -> bool {
        matches!(self, VerificationBasis::NotSent)
    }
}

/// Actions that can be launched directly from a Receipt (`Truth 3`: Receipt as Object Inspector + Action Launcher).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReceiptAct {
    /// 1-step inward withdrawal (`CC 2.4.1.1`) — available to attester or named subject.
    WithdrawTakeBack,
    /// Outward scope widening (`enter_mesh` / `widen_audience`) — gated by 3-fact `ConfirmSheet`.
    WidenScope,
    /// Producer recantation (`CC 2.4.1.3`) — confesses prior assertion was erroneous.
    RecantAssertion,
    /// Named-subject contest/objection (`G4`, `CC 4.5.5`).
    ContestOrObject,
}

/// The 5 Universal Receipt Facts (`CSD-006`) present on every wire-backed `ItemRow`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt5 {
    /// Fact 1: Who it is about (`subject_key_ids`).
    pub subject_key_ids: VerificationBasis,
    /// Fact 2: Who sent it (`attesting_key_id`).
    pub attesting_key_id: VerificationBasis,
    /// Fact 3: Who can see it (`cohort_scope`).
    pub cohort_scope: VerificationBasis,
    /// Fact 4: What it is (`dimension`).
    pub dimension: VerificationBasis,
    /// Fact 5: The rule it follows (`consent:scope`).
    pub consent_scope: VerificationBasis,
    /// Optional beneficiary (`for_key_id`).
    pub for_key_id: Option<String>,
    /// Number of custody holders (`holders`).
    pub holders: u32,
    /// Attached constitutional or verification notes.
    pub notes: Vec<String>,
    /// Allowable actions computed deterministically from `(Form, viewer_key, standing)`.
    pub acts: Vec<ReceiptAct>,
}

impl Receipt5 {
    /// Derive a 5-fact `Receipt5` and its legal `ReceiptAct`s purely from a signed CEG `Form`.
    pub fn from_form(form: &Form, viewer_key: Option<&str>, standing: RowStanding) -> Self {
        let obj = form.envelope.as_object();

        let subjects_vec: Vec<String> = obj
            .and_then(|m| m.get("subject_key_ids"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|x| x.as_str().map(ToString::to_string))
                    .collect()
            })
            .unwrap_or_default();

        let subject_basis = if !subjects_vec.is_empty() {
            VerificationBasis::Wire {
                value: subjects_vec.join(", "),
                gloss: Some("Named subject(s) with Take-Back & Contest standing".to_string()),
            }
        } else {
            VerificationBasis::ByRule {
                value: "self (producer-scoped)".to_string(),
                cc_ref: "CC 2.1.2".to_string(),
            }
        };

        let attester_str = obj
            .and_then(|m| m.get("attesting_key_id"))
            .and_then(|v| v.as_str())
            .map(ToString::to_string);

        let attesting_basis = match &attester_str {
            Some(k) if !k.is_empty() => VerificationBasis::Wire {
                value: k.clone(),
                gloss: Some("Hybrid Ed25519 + ML-DSA-65 signer".to_string()),
            },
            _ => VerificationBasis::ByRule {
                value: "local-node-self".to_string(),
                cc_ref: "CC 2.3.1".to_string(),
            },
        };

        let scope_basis = match obj
            .and_then(|m| m.get("cohort_scope"))
            .and_then(|v| v.as_str())
        {
            Some(s) if !s.is_empty() => {
                let circle = CircleScope::from_wire_scope(s).unwrap_or(CircleScope::JustMe);
                VerificationBasis::Wire {
                    value: s.to_string(),
                    gloss: Some(circle.label().to_string()),
                }
            }
            _ => VerificationBasis::NotSent,
        };

        let dim_str = obj
            .and_then(|m| m.get("dimension"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let kind = ChangeKind::from_dimension(dim_str);
        let dimension_basis = if !dim_str.is_empty() {
            VerificationBasis::Wire {
                value: dim_str.to_string(),
                gloss: Some(format!("{:?} ({:?})", kind, kind.ontology_class())),
            }
        } else {
            VerificationBasis::NotSent
        };

        let consent_basis = match obj
            .and_then(|m| m.get("consent_scope").or_else(|| m.get("consent:scope")))
            .and_then(|v| v.as_str())
        {
            Some(c) if !c.is_empty() => VerificationBasis::Wire {
                value: c.to_string(),
                gloss: None,
            },
            _ => VerificationBasis::ByRule {
                value: format!("op:{} admission rule", form.op.as_str()),
                cc_ref: "CC 2.1 / CC 2.4".to_string(),
            },
        };

        let for_key_id = obj
            .and_then(|m| m.get("for_key_id"))
            .and_then(|v| v.as_str())
            .map(ToString::to_string);

        let holders = obj
            .and_then(|m| m.get("holders"))
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as u32;

        // Compute allowable ReceiptActs (Truth 3)
        let mut acts = Vec::new();
        if standing == RowStanding::Admitted {
            let is_attester = match (viewer_key, attester_str.as_deref()) {
                (Some(vk), Some(ak)) => vk == ak,
                (Some(_), None) => true, // local-node-self
                _ => false,
            };
            let is_subject = viewer_key
                .map(|vk| subjects_vec.iter().any(|s| s == vk))
                .unwrap_or(false);

            if is_attester || is_subject {
                acts.push(ReceiptAct::WithdrawTakeBack);
            }
            if is_attester {
                acts.push(ReceiptAct::WidenScope);
                if kind.admits_verb(Verb::Recant) {
                    acts.push(ReceiptAct::RecantAssertion);
                }
            }
            if is_subject
                && matches!(
                    kind,
                    ChangeKind::Confidence | ChangeKind::Rules | ChangeKind::Facts
                )
            {
                acts.push(ReceiptAct::ContestOrObject);
            }
        }

        Receipt5 {
            subject_key_ids: subject_basis,
            attesting_key_id: attesting_basis,
            cohort_scope: scope_basis,
            dimension: dimension_basis,
            consent_scope: consent_basis,
            for_key_id,
            holders,
            notes: Vec::new(),
            acts,
        }
    }
}

// ============================================================================
// 2. CONTEXTS (X) — 5 Circles × 7 Tabs + 6 Instruments × 9 State Planes × T0..T19
// ============================================================================

/// The 5 human-facing Circles folded from the 7 wire `CohortScope` values (`CirclesNav.kt`, `circles.html`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CircleScope {
    /// `self` — *Just me* (`#22C0E8`)
    JustMe,
    /// `family` — *Family* (`#419CA0`)
    Family,
    /// `community` — *Neighbours* (`#7A6FD6`)
    Neighbours,
    /// `affiliations` — *Communities & Businesses* (`#C96A38`)
    CommunitiesAndBusinesses,
    /// `species | biosphere | federation` — *Everyone* (`#B08A3E`)
    Everyone,
}

impl CircleScope {
    /// Fold any of the 7 wire `CohortScope` strings into the 5 UI Circles.
    pub fn from_wire_scope(s: &str) -> Option<Self> {
        match s {
            "self" | "agent" => Some(CircleScope::JustMe),
            "family" => Some(CircleScope::Family),
            "community" | "local_community" => Some(CircleScope::Neighbours),
            "affiliations" | "global_communities" => Some(CircleScope::CommunitiesAndBusinesses),
            "species" | "biosphere" | "federation" | "global_commons" => {
                Some(CircleScope::Everyone)
            }
            _ => None,
        }
    }

    /// Plain-language Circle title (`CirclesNav.kt` & `circles.html`).
    pub fn label(self) -> &'static str {
        match self {
            CircleScope::JustMe => "Just me",
            CircleScope::Family => "Family",
            CircleScope::Neighbours => "Neighbours",
            CircleScope::CommunitiesAndBusinesses => "Communities & Businesses",
            CircleScope::Everyone => "Everyone",
        }
    }

    /// Canonical wire `cohort_scope` emitted when creating in this circle.
    pub fn wire_canonical(self) -> &'static str {
        match self {
            CircleScope::JustMe => "self",
            CircleScope::Family => "family",
            CircleScope::Neighbours => "community",
            CircleScope::CommunitiesAndBusinesses => "affiliations",
            CircleScope::Everyone => "federation",
        }
    }

    /// Circle color token hex (`ScopePill.kt` — the sole place circle color + name is rendered).
    pub fn accent_hex(self) -> &'static str {
        match self {
            CircleScope::JustMe => "#22C0E8",
            CircleScope::Family => "#419CA0",
            CircleScope::Neighbours => "#7A6FD6",
            CircleScope::CommunitiesAndBusinesses => "#C96A38",
            CircleScope::Everyone => "#B08A3E",
        }
    }

    /// Numeric rank for directional asymmetry: `target.rank() > source.rank()` means widening outward.
    pub fn rank(self) -> u8 {
        match self {
            CircleScope::JustMe => 0,
            CircleScope::Family => 1,
            CircleScope::Neighbours => 2,
            CircleScope::CommunitiesAndBusinesses => 3,
            CircleScope::Everyone => 4,
        }
    }

    /// Whether this scope is structurally invisible to non-members (`Inv_Invisible`, `MeshNet.tla`).
    pub fn is_private(self) -> bool {
        matches!(
            self,
            CircleScope::JustMe | CircleScope::Family | CircleScope::Neighbours
        )
    }
}

/// The 7 Tabs present inside each Circle (`CirclesNav.kt`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CircleTab {
    Files,
    Chats,
    People,
    Safety,
    Rules,
    Decisions,
    Record,
}

impl CircleTab {
    pub fn as_tag(self) -> &'static str {
        match self {
            CircleTab::Files => "tab_files",
            CircleTab::Chats => "tab_chats",
            CircleTab::People => "tab_people",
            CircleTab::Safety => "tab_safety",
            CircleTab::Rules => "tab_rules",
            CircleTab::Decisions => "tab_decisions",
            CircleTab::Record => "tab_record",
        }
    }
}

/// The 6 non-circle Instruments under *My things* (`MyThings.kt`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstrumentId {
    DevicesAndKeys,
    EverythingIShared,
    ThisAgent,
    ThisNode,
    SomeoneITrust,
    Help,
}

impl InstrumentId {
    /// Runtime narrowing (`CC 4.4.3.4.3`): `ThisAgent` and `SomeoneITrust` require an attached agent brain.
    pub fn requires_agent(self) -> bool {
        matches!(
            self,
            InstrumentId::ThisAgent | InstrumentId::SomeoneITrust
        )
    }

    pub fn as_tag(self) -> &'static str {
        match self {
            InstrumentId::DevicesAndKeys => "inst_devices_keys",
            InstrumentId::EverythingIShared => "inst_everything_i_shared",
            InstrumentId::ThisAgent => "inst_this_agent",
            InstrumentId::ThisNode => "inst_this_node",
            InstrumentId::SomeoneITrust => "inst_someone_i_trust",
            InstrumentId::Help => "inst_help",
        }
    }
}

/// The 20 closed CSD/4 topologies (`T0`–`T19`, `manifests/topology_vocabulary.json`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TopologyId {
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16,
    T17,
    T18,
    T19,
}

/// Normalized slice of the 9 formal state planes (`formal/STATE_TABLE.tsv`: 157 states over 51 variables).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatePlaneVector {
    /// Plane 1 (`node`): whether the node is claimed, serving, and whether an AI agent is attached.
    pub node_claimed: bool,
    pub node_serving: bool,
    pub has_agent: bool,
    pub halt_status: HaltVerdict,
    /// Plane 2 (`root`): trust-root standing verdict.
    pub root_standing: RootVerdict,
    pub root_count: u8,
    /// Plane 3 (`self`): self-key roster count and post-quantum status.
    pub self_roster_size: u32,
    pub pqc_complete: bool,
    /// Plane 4 (`membership`): active group membership stage and federation flag.
    pub member_stage: MemberStage,
    pub group_federating: bool,
    /// Plane 5 (`consent`): active consent and replication status.
    pub consent_granted: bool,
    pub replication_granted: bool,
    /// Plane 7 (`authority`): stewardship and guardian bond status.
    pub steward_bonded: bool,
    pub guardian_bonded: bool,
    pub delegation_depth: u8,
    /// Plane 8 (`agent`): agent conscience gate and optional retirement stage.
    pub agent_gate: AgentGateVerdict,
    pub retirement_stage: Option<RetirementStage>,
    pub wa_present: bool,
    /// Plane 9 (`governance`): capacity and quarantine status.
    pub capacitated: bool,
}

impl Default for StatePlaneVector {
    fn default() -> Self {
        Self {
            node_claimed: true,
            node_serving: true,
            has_agent: true,
            halt_status: HaltVerdict::Clear,
            root_standing: RootVerdict::Rooted,
            root_count: 1,
            self_roster_size: 1,
            pqc_complete: true,
            member_stage: MemberStage::Member,
            group_federating: true,
            consent_granted: true,
            replication_granted: false,
            steward_bonded: true,
            guardian_bonded: false,
            delegation_depth: 1,
            agent_gate: AgentGateVerdict::Acting,
            retirement_stage: None,
            wa_present: true,
            capacitated: true,
        }
    }
}

impl StatePlaneVector {
    /// Enforce theorem `Inv_X1_LatchDoesNotBlockServing` (`cross_model_composition.tla`):
    /// When the halt latch is active (`LatchedLone` or `Confirmed`), the agent act gate MUST refuse
    /// with `423 Locked`, while the node's data/serving doors (`node_serving`) remain open!
    pub fn with_halt_applied(mut self, halt: HaltVerdict) -> Self {
        self.halt_status = halt.clone();
        match halt {
            HaltVerdict::Clear => {}
            HaltVerdict::LatchedLone { .. } | HaltVerdict::Confirmed { .. } => {
                // Node keeps serving data doors (read/export/withdraw), agent gate refuses 423.
                self.node_serving = true;
                self.agent_gate = AgentGateVerdict::Refusing423 {
                    data_doors_open: true,
                    terminal_result: "HALT_LATCHED_423".to_string(),
                };
            }
        }
        self
    }
}

/// Complete viewer evaluation context `x in X`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerContext {
    pub viewer_key_id: String,
    pub circle: CircleScope,
    pub tab: CircleTab,
    pub active_instrument: Option<InstrumentId>,
    pub topology: TopologyId,
    pub planes: StatePlaneVector,
    pub active_lens_rules: Vec<String>,
    pub helper_keys: Vec<String>,
}

impl Default for ViewerContext {
    fn default() -> Self {
        Self {
            viewer_key_id: "ed25519:viewer_self".to_string(),
            circle: CircleScope::JustMe,
            tab: CircleTab::Files,
            active_instrument: None,
            topology: TopologyId::T0,
            planes: StatePlaneVector::default(),
            active_lens_rules: vec!["my_rules".to_string()],
            helper_keys: Vec::new(),
        }
    }
}

// ============================================================================
// 3. ACTIONS (A) — Directional Asymmetry Law + 10 Governance Policy Families
// ============================================================================

/// The 8 canonical transition archetypes (`UI_FORMAL_PLAN.md §2.3`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionArchetype {
    A1CreateLocal,
    A2ShareOutward,
    A3TakeBackOrWithdraw,
    A4MembershipCeremony,
    A5GovernanceVote,
    A6DelegateOrBond,
    A7HaltOrRefuse,
    A8RetireAndSeal,
}

/// One fact row inside a `ConfirmSheet` (`ConfirmSheet.kt`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfirmFact {
    pub label: String,
    pub value: String,
}

/// Compile-time and runtime proof that a `ConfirmSheet` carries **exactly 3 facts**
/// (*What is moving/changing*, *To whom / scope*, *Can I undo it?*), plus an optional danger note.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfirmFacts3 {
    pub title: String,
    pub facts: [ConfirmFact; 3],
    pub danger_note: Option<String>,
}

impl ConfirmFacts3 {
    pub fn new(
        title: impl Into<String>,
        fact1: (impl Into<String>, impl Into<String>),
        fact2: (impl Into<String>, impl Into<String>),
        fact3: (impl Into<String>, impl Into<String>),
        danger_note: Option<String>,
    ) -> Self {
        Self {
            title: title.into(),
            facts: [
                ConfirmFact {
                    label: fact1.0.into(),
                    value: fact1.1.into(),
                },
                ConfirmFact {
                    label: fact2.0.into(),
                    value: fact2.1.into(),
                },
                ConfirmFact {
                    label: fact3.0.into(),
                    value: fact3.1.into(),
                },
            ],
            danger_note,
        }
    }
}

/// Gate requirement computed by the Directional Asymmetry Law.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateRequirement {
    /// Inward / narrowing / exit / halt / refuse: 1-step immediate execution, zero confirmation friction.
    ImmediateOneStep,
    /// Outward / widening / delegating / irreversible: requires the 3-fact `ConfirmSheet`.
    ConfirmSheet3(ConfirmFacts3),
    /// Multi-party ceremony governed by one of the 10 rows of `manifests/governance_policy.json`.
    TwoPathCeremony(GovernancePolicyFamily),
}

/// Typed user or agent action intent returned from an `EpistemicScene`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionIntent {
    /// Born in *Just me* (`self` scope, `Inv_SmallestCircle`).
    CreateLocal { dimension: String },
    /// Move/widen an existing claim from `from` to `to`.
    MoveScope {
        item_label: String,
        from: CircleScope,
        to: CircleScope,
    },
    /// Pull back / withdraw a claim (`CC 2.4.1.1`).
    WithdrawClaim { claim_id: String },
    /// Producer recants an assertion (`CC 2.4.1.3`).
    RecantClaim { claim_id: String },
    /// Leave a household, community, or trust root (`Inv_ExitOneStep`).
    LeaveGroupOrRoot { target_id: String },
    /// Fire 1-of-N emergency Accord halt (`AccordHaltFuse.tla`).
    FireAccordHalt { root_id: String },
    /// Grant authority delegation (`<= 5` hops, `<= 365` days).
    GrantDelegation {
        delegate_key: String,
        scope: String,
        valid_days: u16,
    },
    /// Revoke an existing delegation link.
    RevokeDelegation { link_id: String },
    /// Cancel a planned agent retirement before the `Frozen` state (`Retirement.tla`).
    CancelRetirementBeforeFrozen { agent_id: String },
    /// Agent conscience `REFUSE` (`G17`) or `WBD` deferral to Wise Authority (`G8`).
    AgentRefuseOrDefer { reason: String },
    /// Start or vote in a multi-party governance ceremony.
    CeremonyAction {
        family: GovernancePolicyFamily,
        approve: bool,
    },
}

impl ActionIntent {
    /// **The Directional Asymmetry Law (`circles.html` Scene 11 & `ConfirmSheet.kt`):**
    /// *"Going out asks. Coming back never does."*
    pub fn gate_requirement(&self) -> GateRequirement {
        match self {
            ActionIntent::CreateLocal { .. }
            | ActionIntent::WithdrawClaim { .. }
            | ActionIntent::RecantClaim { .. }
            | ActionIntent::LeaveGroupOrRoot { .. }
            | ActionIntent::FireAccordHalt { .. }
            | ActionIntent::RevokeDelegation { .. }
            | ActionIntent::CancelRetirementBeforeFrozen { .. }
            | ActionIntent::AgentRefuseOrDefer { .. } => GateRequirement::ImmediateOneStep,

            ActionIntent::MoveScope {
                item_label,
                from,
                to,
            } => {
                // Narrowing inward is 1-step; widening outward requires a 3-fact ConfirmSheet.
                if to.rank() <= from.rank() {
                    GateRequirement::ImmediateOneStep
                } else {
                    let danger = if *to == CircleScope::Everyone {
                        Some("Federation copies cannot be forcibly erased from remote peers once replicated.".to_string())
                    } else {
                        None
                    };
                    GateRequirement::ConfirmSheet3(ConfirmFacts3::new(
                        format!("Share {} to {}", item_label, to.label()),
                        ("Moving", item_label.clone()),
                        ("To", format!("{} ({})", to.label(), to.wire_canonical())),
                        (
                            "Can I undo it?",
                            "You can withdraw your claim in 1 step at any time (CC 2.4.1.1).",
                        ),
                        danger,
                    ))
                }
            }

            ActionIntent::GrantDelegation {
                delegate_key,
                scope,
                valid_days,
            } => GateRequirement::ConfirmSheet3(ConfirmFacts3::new(
                "Grant authority delegation",
                ("Delegating to", delegate_key.clone()),
                ("Scope & term", format!("{} ({} days, <= 365d max)", scope, valid_days)),
                (
                    "Can I undo it?",
                    "1-step withdraw revokes this link and cascades to sub-delegations (authority_tree).",
                ),
                None,
            )),

            ActionIntent::CeremonyAction { family, .. } => {
                GateRequirement::TwoPathCeremony(*family)
            }
        }
    }
}

/// The 10 Governance Decision Families in `CC rc9` (`manifests/governance_policy.json`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GovernancePolicyFamily {
    DefaultContent,
    DefaultStanding,
    DefaultIrreversible,
    CommunityInvite,
    CommunityRemove,
    SelfRosterAdd,
    SelfRosterEvict,
    ModerationAction,
    AccordHalt,
    CustodianReplace,
}

/// Decision mechanism archetype (`manifests/governance_policy.json`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PolicyMechanismKind {
    TwoPath,
    Mirror,
    Cosigned,
}

/// Formal specification of a row in `manifests/governance_policy.json` (`TwoPath.tla`, `PolicyCorridor.lean`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernancePolicySpec {
    pub family: GovernancePolicyFamily,
    pub kind: PolicyMechanismKind,
    pub enact_num: u32,
    pub enact_den: u32,
    pub cancel_num: u32,
    pub cancel_den: u32,
    pub cancel_min: u32,
    pub floor_num: u32,
    pub floor_den: u32,
    pub floor_min: u32,
    pub window_secs: u64,
    pub irreversible: bool,
    pub has_reverse_path: bool,
}

impl GovernancePolicyFamily {
    /// Return the exact `CC rc9` `manifests/governance_policy.json` specification for this family.
    pub fn spec(self) -> GovernancePolicySpec {
        match self {
            GovernancePolicyFamily::DefaultContent => GovernancePolicySpec {
                family: self,
                kind: PolicyMechanismKind::TwoPath,
                enact_num: 2,
                enact_den: 3,
                cancel_num: 1,
                cancel_den: 3,
                cancel_min: 2,
                floor_num: 1,
                floor_den: 3,
                floor_min: 2,
                window_secs: 86_400, // 24h
                irreversible: false,
                has_reverse_path: true,
            },
            GovernancePolicyFamily::DefaultStanding => GovernancePolicySpec {
                family: self,
                kind: PolicyMechanismKind::TwoPath,
                enact_num: 2,
                enact_den: 3,
                cancel_num: 1,
                cancel_den: 3,
                cancel_min: 2,
                floor_num: 1,
                floor_den: 3,
                floor_min: 2,
                window_secs: 259_200, // 72h
                irreversible: false,
                has_reverse_path: true,
            },
            GovernancePolicyFamily::DefaultIrreversible => GovernancePolicySpec {
                family: self,
                kind: PolicyMechanismKind::TwoPath,
                enact_num: 2,
                enact_den: 3,
                cancel_num: 1,
                cancel_den: 3,
                cancel_min: 2,
                floor_num: 1,
                floor_den: 3,
                floor_min: 2,
                window_secs: 604_800, // 7d
                irreversible: true,
                has_reverse_path: true,
            },
            GovernancePolicyFamily::CommunityInvite => GovernancePolicySpec {
                family: self,
                kind: PolicyMechanismKind::TwoPath,
                enact_num: 2,
                enact_den: 3,
                cancel_num: 1,
                cancel_den: 3,
                cancel_min: 2,
                floor_num: 1,
                floor_den: 3,
                floor_min: 2,
                window_secs: 259_200, // 72h
                irreversible: false,
                has_reverse_path: true,
            },
            GovernancePolicyFamily::CommunityRemove => GovernancePolicySpec {
                family: self,
                kind: PolicyMechanismKind::TwoPath,
                enact_num: 2,
                enact_den: 3,
                cancel_num: 1,
                cancel_den: 1,
                cancel_min: u32::MAX, // forward-only cosigned; expires cancelled
                floor_num: 2,
                floor_den: 3,
                floor_min: 2,
                window_secs: 259_200, // 72h
                irreversible: false,
                has_reverse_path: false,
            },
            GovernancePolicyFamily::SelfRosterAdd => GovernancePolicySpec {
                family: self,
                kind: PolicyMechanismKind::TwoPath,
                enact_num: 1,
                enact_den: 2,
                cancel_num: 1,
                cancel_den: 2,
                cancel_min: 1,
                floor_num: 0,
                floor_den: 1,
                floor_min: 1, // 1 lone signer floor
                window_secs: 86_400, // 24h
                irreversible: false,
                has_reverse_path: true,
            },
            GovernancePolicyFamily::SelfRosterEvict => GovernancePolicySpec {
                family: self,
                kind: PolicyMechanismKind::TwoPath,
                enact_num: 1,
                enact_den: 2,
                cancel_num: 1,
                cancel_den: 2,
                cancel_min: 1,
                floor_num: 0,
                floor_den: 1,
                floor_min: 1,
                window_secs: 86_400, // 24h
                irreversible: false,
                has_reverse_path: true,
            },
            GovernancePolicyFamily::ModerationAction => GovernancePolicySpec {
                family: self,
                kind: PolicyMechanismKind::Mirror,
                enact_num: 0,
                enact_den: 1, // 1 moderator/steward acts immediately in window
                cancel_num: 1,
                cancel_den: 2,
                cancel_min: 1,
                floor_num: 1,
                floor_den: 2,
                floor_min: 1,
                window_secs: 172_800, // 48h
                irreversible: false,
                has_reverse_path: true,
            },
            GovernancePolicyFamily::AccordHalt => GovernancePolicySpec {
                family: self,
                kind: PolicyMechanismKind::Mirror,
                enact_num: 0,
                enact_den: 1, // 1-of-N fires latch immediately; >1/2 confirms
                cancel_num: 1,
                cancel_den: 2,
                cancel_min: 1,
                floor_num: 1,
                floor_den: 2,
                floor_min: 2,
                window_secs: 86_400, // 24h fuse
                irreversible: false,
                has_reverse_path: false,
            },
            GovernancePolicyFamily::CustodianReplace => GovernancePolicySpec {
                family: self,
                kind: PolicyMechanismKind::Cosigned,
                enact_num: 1,
                enact_den: 1, // 2-of-2 cosigned (strict >1/2 of 2 = 2)
                cancel_num: 1,
                cancel_den: 2,
                cancel_min: 1,
                floor_num: 1,
                floor_den: 1,
                floor_min: 2,
                window_secs: 0,
                irreversible: false,
                has_reverse_path: false,
            },
        }
    }
}

impl GovernancePolicySpec {
    /// Forward enact threshold (`TwoPath.tla` / `PolicyCorridor.lean`):
    /// `floor(enact_num * base / enact_den) + 1`, clamped to `1..=base`.
    pub fn enact_votes(&self, base: u32) -> u32 {
        if base == 0 {
            return 1;
        }
        if matches!(self.family, GovernancePolicyFamily::CustodianReplace) {
            return 2.min(base);
        }
        if matches!(
            self.family,
            GovernancePolicyFamily::ModerationAction | GovernancePolicyFamily::AccordHalt
        ) {
            return 1;
        }
        let v = (self.enact_num * base) / self.enact_den + 1;
        v.clamp(1, base)
    }

    /// Objection brake threshold (`TwoPath.tla`):
    /// `max(cancel_min, ceil(cancel_num * denom_pool / cancel_den))`.
    /// For `SelfRosterAdd` / `SelfRosterEvict`, the objection pool is `others = base - 1`
    /// with strict majority `> 1/2` of others.
    pub fn cancel_votes(&self, base: u32) -> u32 {
        if !self.has_reverse_path
            && matches!(self.family, GovernancePolicyFamily::CommunityRemove)
        {
            return u32::MAX;
        }
        if matches!(
            self.family,
            GovernancePolicyFamily::SelfRosterAdd | GovernancePolicyFamily::SelfRosterEvict
        ) {
            let others = base.saturating_sub(1);
            if others == 0 {
                return 1;
            }
            return (others / 2) + 1;
        }
        let ceil_div = (self.cancel_num * base + self.cancel_den - 1) / self.cancel_den;
        ceil_div.max(self.cancel_min).min(base.max(1))
    }

    /// Silence floor threshold for the reverse path (`TwoPath.tla`):
    /// `max(floor_min, ceil(floor_num * base / floor_den))`.
    pub fn floor_votes(&self, base: u32) -> u32 {
        if self.floor_num == 0 {
            return self.floor_min;
        }
        let ceil_div = (self.floor_num * base + self.floor_den - 1) / self.floor_den;
        ceil_div.max(self.floor_min).min(base.max(1))
    }

    /// Evaluate the exact `TwoPathVerdict` (`TwoPath.tla` & `SelfRoster.tla`).
    pub fn evaluate(
        &self,
        base: u32,
        approvals: u32,
        objections: u32,
        window_rem_secs: u64,
    ) -> TwoPathVerdict {
        // Special case from SelfRoster.tla (CSD-095):
        // - 1 key: unilateral immediate forward landing
        // - 2 keys: 1-of-2 with 0 objections stays pending (stalemate; lone path closed for 2 keys)
        if matches!(
            self.family,
            GovernancePolicyFamily::SelfRosterAdd | GovernancePolicyFamily::SelfRosterEvict
        ) {
            if base == 1 && approvals >= 1 {
                return TwoPathVerdict::LandsForward;
            }
            if base == 2 {
                if approvals >= 2 {
                    return TwoPathVerdict::LandsForward;
                }
                if objections >= 1 {
                    return TwoPathVerdict::Cancelled {
                        reason: "Objected by other key in 2-key roster".to_string(),
                    };
                }
                return TwoPathVerdict::Pending {
                    approvals,
                    enact_needed: 2,
                    objections,
                    cancel_needed: 1,
                    floor_needed: 2, // lone path disabled at N=2 per SelfRoster.tla
                    window_rem_secs,
                };
            }
        }

        let enact = self.enact_votes(base);
        let cancel = self.cancel_votes(base);
        let floor = self.floor_votes(base);

        // 1. Check objection brake first.
        if objections >= cancel {
            return TwoPathVerdict::Cancelled {
                reason: format!("Objection brake reached ({objections}/{cancel})"),
            };
        }

        // 2. Check forward path (instant unless `irreversible: true`, which must wait out the window).
        if approvals >= enact && (!self.irreversible || window_rem_secs == 0) {
            return TwoPathVerdict::LandsForward;
        }

        // 3. At window expiry (`window_rem_secs == 0`), check reverse silence path.
        if window_rem_secs == 0 {
            if self.has_reverse_path && approvals >= floor && objections == 0 {
                return TwoPathVerdict::LandsReverse;
            } else {
                return TwoPathVerdict::Cancelled {
                    reason: "Window expired without meeting silence floor or unopposed requirement"
                        .to_string(),
                };
            }
        }

        TwoPathVerdict::Pending {
            approvals,
            enact_needed: enact,
            objections,
            cancel_needed: cancel,
            floor_needed: floor,
            window_rem_secs,
        }
    }
}

// ============================================================================
// 4. CONCLUSIONS (V) — Closed Formal Verdicts per Plane
// ============================================================================

/// Trust-Root Standing Verdict (`TrustRootVerdict.tla`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RootVerdict {
    Rooted,
    Stalled,
    NotRooted,
}

/// Node Halt Latch Verdict (`AccordHaltFuse.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HaltVerdict {
    Clear,
    LatchedLone { fuse_rem_secs: u64 },
    Confirmed { signers: u32 },
}

/// Agent Act-Gate Verdict (`Guarantees.tla`, `Inv_X1_LatchDoesNotBlockServing`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentGateVerdict {
    Acting,
    Refusing423 {
        /// Must always be `true` per `Inv_X1_LatchDoesNotBlockServing`: data/serving doors stay open!
        data_doors_open: bool,
        terminal_result: String,
    },
    DeferredToWa {
        ticket_id: String,
    },
    RefusedG17 {
        reason: String,
    },
}

/// Two-Path / Governance Decision Verdict (`TwoPath.tla`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TwoPathVerdict {
    LandsForward,
    LandsReverse,
    Pending {
        approvals: u32,
        enact_needed: u32,
        objections: u32,
        cancel_needed: u32,
        floor_needed: u32,
        window_rem_secs: u64,
    },
    Cancelled {
        reason: String,
    },
}

/// Content Row Standing (`MeshNet.tla`, `WithdrawsAdmission.tla`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RowStanding {
    Admitted,
    Withdrawn,
    Recanted,
    Superseded,
    Tombstoned,
}

/// Group Membership Stage (`Membership.tla`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemberStage {
    Proposed,
    AcceptedAwaitingGroup,
    Declined,
    Member,
    Removed,
    Resigned,
}

/// Local Policy-Lens Rendering Verdict (`circles.html` Scene 12–13: *"A note is an opinion, not an order."*).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PolicyLensVerdict {
    Clear,
    Blurred { opinion_by: String, rule: String },
    HiddenByRules { rule: String },
    FileGone,
}

/// Evaluate a stream of opinion/moderation `Form`s (`cw_class:*`, `moderation:*`, `evaluation:*`)
/// under the viewer's chosen policy lens (`active_rules`, `helper_keys`).
pub fn evaluate_policy_lens(
    opinions: &[Form],
    active_rules: &[String],
    helper_keys: &[String],
) -> PolicyLensVerdict {
    let mut blur_match: Option<(String, String)> = None;
    for form in opinions {
        if form.op != Op::Scores {
            continue;
        }
        let dim = form
            .envelope
            .get("dimension")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let score = form
            .envelope
            .get("score")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let attester = form
            .envelope
            .get("attesting_key_id")
            .and_then(|v| v.as_str())
            .unwrap_or("neighbour");

        let trusted_helper = helper_keys.is_empty() || helper_keys.iter().any(|k| k == attester);
        if !trusted_helper {
            continue;
        }

        // If viewer rule says hide strong negative moderation scores:
        if dim.starts_with("moderation:hide")
            && score <= -0.5
            && active_rules.iter().any(|r| r == "my_rules")
        {
            return PolicyLensVerdict::HiddenByRules {
                rule: format!("Hidden by my_rules ({dim} by {attester})"),
            };
        }

        // Content warning or negative score -> blur under viewer's rules (`circles.html` Scene 12).
        if (dim.starts_with("cw_class:") || score < 0.0)
            && active_rules.iter().any(|r| r == "my_rules")
        {
            blur_match = Some((attester.to_string(), dim.to_string()));
        }
    }

    if let Some((by, rule)) = blur_match {
        PolicyLensVerdict::Blurred {
            opinion_by: by,
            rule,
        }
    } else {
        PolicyLensVerdict::Clear
    }
}

/// The 7-State Forward-Only Retirement Ladder (`Retirement.tla`, `CSD-113`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RetirementStage {
    Decided = 1,
    Frozen = 2,
    SecretsDestroyed = 3,
    HandedOver = 4,
    Notified = 5,
    CustodianMinted = 6,
    Retired = 7,
}

impl RetirementStage {
    /// Per `Retirement.tla` (`Inv_FreezeSealsCancel`), `withdraws` can cancel retirement
    /// ONLY before `Frozen` (i.e. while in `Decided`).
    pub fn can_withdraw_cancel(self) -> bool {
        self == RetirementStage::Decided
    }

    pub fn all_steps() -> [RetirementStage; 7] {
        [
            RetirementStage::Decided,
            RetirementStage::Frozen,
            RetirementStage::SecretsDestroyed,
            RetirementStage::HandedOver,
            RetirementStage::Notified,
            RetirementStage::CustodianMinted,
            RetirementStage::Retired,
        ]
    }
}

/// Semantic UI Tone (`StateBlock.kt`, `ConfirmSheet.kt`, `Chip.kt`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tone {
    Mute,
    Brand,
    Ok,
    Warn,
    Danger,
}

/// The 7-way Surface / List Epistemic Verdict (`StateBlock.kt`, `CSD/3 §2.2`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SurfaceState {
    Populated,
    Loading,
    Empty { honest_zero: String },
    Error {
        title: String,
        body: String,
        detail: Option<String>,
    },
    FileGone { digest: String },
    HiddenByRules { rule: String },
    Unreviewed { summary: String },
}

impl SurfaceState {
    /// Enforce the constitutional tone invariant (`StateBlock.kt` & CC Part 8):
    /// - `Error` is ALWAYS `Tone::Danger` (never looks like `Empty`).
    /// - `Unreviewed` is ALWAYS `Tone::Brand` (never `Tone::Ok`/green).
    /// - `Empty` is `Tone::Mute`.
    pub fn tone(&self) -> Tone {
        match self {
            SurfaceState::Populated => Tone::Ok,
            SurfaceState::Loading | SurfaceState::Empty { .. } => Tone::Mute,
            SurfaceState::Error { .. } => Tone::Danger,
            SurfaceState::FileGone { .. } | SurfaceState::HiddenByRules { .. } => Tone::Warn,
            SurfaceState::Unreviewed { .. } => Tone::Brand,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use attestation_calculus_spike::forms::read_form;
    use serde_json::json;

    #[test]
    fn kind_verb_occupancy_enforces_rc9_laws() {
        // Identity and Circumstances never admit Recant.
        assert!(!ChangeKind::Identity.admits_verb(Verb::Recant));
        assert!(!ChangeKind::Circumstances.admits_verb(Verb::Recant));
        assert!(!ChangeKind::Process.admits_verb(Verb::Recant));
        // Facts and Confidence DO admit Recant.
        assert!(ChangeKind::Facts.admits_verb(Verb::Recant));
        assert!(ChangeKind::Confidence.admits_verb(Verb::Recant));
        // Only Rules and Identity admit Authorize (`delegates_to`).
        assert!(ChangeKind::Rules.admits_verb(Verb::Authorize));
        assert!(ChangeKind::Identity.admits_verb(Verb::Authorize));
        assert!(!ChangeKind::Facts.admits_verb(Verb::Authorize));
        // Count total occupied cells across all 12 kinds × 6 verbs = 48.
        let kinds = [
            ChangeKind::Priorities,
            ChangeKind::Rules,
            ChangeKind::Manner,
            ChangeKind::Identity,
            ChangeKind::Confidence,
            ChangeKind::Facts,
            ChangeKind::Circumstances,
            ChangeKind::Process,
            ChangeKind::Model,
            ChangeKind::Structure,
            ChangeKind::Premises,
            ChangeKind::Record,
        ];
        let verbs = [
            Verb::Attest,
            Verb::Authorize,
            Verb::Replace,
            Verb::Withdraw,
            Verb::Recant,
            Verb::Carries,
        ];
        let occupied: usize = kinds
            .iter()
            .flat_map(|k| verbs.iter().filter(move |v| k.admits_verb(**v)))
            .count();
        assert_eq!(occupied, 48, "CC rc9 grid has exactly 48 occupied (Kind × Verb) cells");
    }

    #[test]
    fn directional_asymmetry_law_holds() {
        // Outward move (JustMe -> Everyone) requires 3-fact ConfirmSheet with federation danger note.
        let outward = ActionIntent::MoveScope {
            item_label: "beach.jpg".to_string(),
            from: CircleScope::JustMe,
            to: CircleScope::Everyone,
        };
        match outward.gate_requirement() {
            GateRequirement::ConfirmSheet3(sheet) => {
                assert_eq!(sheet.facts.len(), 3);
                assert!(sheet.danger_note.is_some());
            }
            other => panic!("expected ConfirmSheet3 for outward move, got {other:?}"),
        }

        // Inward move (Everyone -> JustMe), Withdraw, Leave, Halt, Refuse are all 1-step immediate!
        let inward_actions = [
            ActionIntent::MoveScope {
                item_label: "beach.jpg".to_string(),
                from: CircleScope::Everyone,
                to: CircleScope::JustMe,
            },
            ActionIntent::WithdrawClaim {
                claim_id: "row-1".to_string(),
            },
            ActionIntent::LeaveGroupOrRoot {
                target_id: "root-1".to_string(),
            },
            ActionIntent::FireAccordHalt {
                root_id: "root-1".to_string(),
            },
            ActionIntent::AgentRefuseOrDefer {
                reason: "G17 refusal".to_string(),
            },
        ];
        for act in inward_actions {
            assert_eq!(
                act.gate_requirement(),
                GateRequirement::ImmediateOneStep,
                "protective/inward action {act:?} must be 1-step"
            );
        }
    }

    #[test]
    fn two_path_governance_policy_matches_rc9_table() {
        // Self-roster add: N=1 -> immediate forward; N=2 with 1 vote -> stalemate Pending; N=3 with 2 votes -> LandsForward; N=3 with 1 vote at window=0 -> LandsReverse (lone path).
        let spec = GovernancePolicyFamily::SelfRosterAdd.spec();
        assert_eq!(spec.evaluate(1, 1, 0, 86_400), TwoPathVerdict::LandsForward);
        assert!(matches!(
            spec.evaluate(2, 1, 0, 0),
            TwoPathVerdict::Pending { .. }
        ));
        assert_eq!(spec.evaluate(3, 2, 0, 86_400), TwoPathVerdict::LandsForward);
        assert_eq!(spec.evaluate(3, 1, 0, 0), TwoPathVerdict::LandsReverse);
        assert!(matches!(
            spec.evaluate(3, 1, 1, 86_400),
            TwoPathVerdict::Cancelled { .. }
        ));

        // Irreversible policy: even with 100% approvals, stays Pending while window_rem_secs > 0!
        let irrev = GovernancePolicyFamily::DefaultIrreversible.spec();
        assert!(matches!(
            irrev.evaluate(6, 6, 0, 3600),
            TwoPathVerdict::Pending { .. }
        ));
        assert_eq!(irrev.evaluate(6, 5, 0, 0), TwoPathVerdict::LandsForward);
    }

    #[test]
    fn halt_latch_preserves_data_doors_inv_x1() {
        let planes = StatePlaneVector::default().with_halt_applied(HaltVerdict::LatchedLone {
            fuse_rem_secs: 86_400,
        });
        assert!(
            planes.node_serving,
            "Inv_X1_LatchDoesNotBlockServing: node must keep serving data doors during halt"
        );
        assert_eq!(
            planes.agent_gate,
            AgentGateVerdict::Refusing423 {
                data_doors_open: true,
                terminal_result: "HALT_LATCHED_423".to_string(),
            }
        );
    }

    #[test]
    fn receipt5_derives_facts_and_acts_from_signed_form() {
        let env = json!({
            "attestation_type": "scores",
            "dimension": "evaluation:quality",
            "score": 0.85,
            "confidence": 0.92,
            "subject_key_ids": ["ed25519:alice"],
            "attesting_key_id": "ed25519:bob",
            "cohort_scope": "community",
            "evidence_refs": []
        });
        let form = read_form(&env).unwrap();

        // When viewed by the subject ("ed25519:alice"), she gets 1-step WithdrawTakeBack + ContestOrObject.
        let r_alice = Receipt5::from_form(&form, Some("ed25519:alice"), RowStanding::Admitted);
        assert!(r_alice.acts.contains(&ReceiptAct::WithdrawTakeBack));
        assert!(r_alice.acts.contains(&ReceiptAct::ContestOrObject));
        assert!(!r_alice.acts.contains(&ReceiptAct::RecantAssertion));

        // When viewed by the attester ("ed25519:bob"), he gets WithdrawTakeBack + WidenScope + RecantAssertion.
        let r_bob = Receipt5::from_form(&form, Some("ed25519:bob"), RowStanding::Admitted);
        assert!(r_bob.acts.contains(&ReceiptAct::WithdrawTakeBack));
        assert!(r_bob.acts.contains(&ReceiptAct::WidenScope));
        assert!(r_bob.acts.contains(&ReceiptAct::RecantAssertion));
    }

    #[test]
    fn policy_lens_blurs_opinions_without_deleting() {
        let cw_env = json!({
            "attestation_type": "scores",
            "dimension": "cw_class:spoiler",
            "score": -0.6,
            "confidence": 0.9,
            "attesting_key_id": "ed25519:neighbour_dan",
            "cohort_scope": "community"
        });
        let form = read_form(&cw_env).unwrap();
        let verdict = evaluate_policy_lens(
            &[form],
            &["my_rules".to_string()],
            &["ed25519:neighbour_dan".to_string()],
        );
        assert_eq!(
            verdict,
            PolicyLensVerdict::Blurred {
                opinion_by: "ed25519:neighbour_dan".to_string(),
                rule: "cw_class:spoiler".to_string(),
            }
        );
    }
}
