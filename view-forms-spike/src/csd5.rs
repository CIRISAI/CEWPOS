//! csd5.rs — The Screen Standard (`CSD/5` draft 3, `manifests/ui_primitives.json` at `v1.0-rc21-wire`)
//! and `holon-draw` Profile B Whole-Screen Composition Layer (`INTERFACE.md` §1, §2, §8, §9, §10).
//!
//! Unites:
//! 1. **The 24 Closed UX Primitives (`Csd5Primitive`)** and **60 Closed Slots (`Csd5Slot`)**, replacing
//!    the superseded 12-primitive (`P1`–`P12`) taxonomy (Issue #3).
//! 2. **The 17 Exact Read-Bound States (`Csd5State`)** and **4 Read Outcomes (`ReadOutcome`)** (`L2`, `L3`).
//! 3. **The 12 Screen Laws (`Csd5Law::L1`..`L12`)** and **5 Write Verbs (`Csd5WriteVerb`)** (`L6`).
//! 4. **The Derived Tag Grammar (`L1`, `INTERFACE.md` §10.1)**:
//!    - Instance / Slot tag: `prefix '_' surface '_' instance [ '__' slot ] [ '.' key ]` (slot BEFORE key!).
//!    - Shell tag: `shell_prefix '_' name` (`circle`, `tab`, `nav`, `screen`).
//!    - Built exclusively by the primitives layer (`derive_csd5_tag`, `derive_shell_tag`), never by hand.
//! 5. **Direct `holon-draw` (`CIRISHolon/engine/crates/holon-draw`) Profile B Composition**:
//!    - Projects any `EpistemicScene` or 24-primitive `Csd5Screen` into `holon_draw::Scene` tiles of
//!      52-byte `holon_draw::Instance` marks (`Kind::Quad`, `Kind::Line`, `Kind::Arc`, `Kind::Disc`,
//!      `Kind::Sphere`, `Kind::Rod`, `Kind::Glyph`, `Kind::Run`), with `HandleTable` mapping each
//!      `u32` handle to its derived `CSD/5` tag and source-string SHA-256 digest (`L12` `shown_digest`).

use crate::epistemic_scene::{EpistemicNode, EpistemicScene};
use holon_draw::{
    ui::{FakeText, Text, BODY, BUTTON, HEADING, SMALL, STATUS},
    Acuity, Batch, Camera, Frame, Instance, Kind, Scene as DrawScene, Shown, Tile, PARAMS_NEUTRAL,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Pinned `CIRISConstitution` commit for `manifests/ui_primitives.json` (`v1.0-rc21-wire`).
pub const CSD5_PINNED_TAG: &str = "v1.0-rc21-wire";
pub const CSD5_PINNED_COMMIT: &str = "e6fd7c6071ced5d0dcdd7f578e7fa9625e5bf16c";
pub const CSD5_STANDARD: &str = "CSD/5";
pub const CSD5_DRAFT: u32 = 3;

/// Embedded copy of `manifests/ui_primitives.json` vendored at `v1.0-rc21-wire`.
pub const UI_PRIMITIVES_JSON: &str = include_str!("../../epistemic-ui/ui_primitives.json");

// ============================================================================
// 1. SHELL PREFIXES, READ OUTCOMES, 17 STATES, 5 WRITE VERBS, 12 LAWS
// ============================================================================

/// The 4 shell prefixes (`shell_prefix '_' name`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellPrefix {
    Circle,
    Tab,
    Nav,
    Screen,
}

impl ShellPrefix {
    pub const ALL: [ShellPrefix; 4] = [
        ShellPrefix::Circle,
        ShellPrefix::Tab,
        ShellPrefix::Nav,
        ShellPrefix::Screen,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            ShellPrefix::Circle => "circle",
            ShellPrefix::Tab => "tab",
            ShellPrefix::Nav => "nav",
            ShellPrefix::Screen => "screen",
        }
    }
}

/// The 4 read outcomes of `CSD/5`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadOutcome {
    /// Not asked.
    Idle,
    /// Asked, no answer yet.
    Pending,
    /// No answer arrived, or it could not be parsed.
    Failed,
    /// The node answered; a refusal and a non-2xx status that carries a body are answers.
    Ok,
}

impl ReadOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReadOutcome::Idle => "idle",
            ReadOutcome::Pending => "pending",
            ReadOutcome::Failed => "failed",
            ReadOutcome::Ok => "ok",
        }
    }
}

/// All 17 exact states of a part bound to a read (`StateBlock` `@states`, `L2`, `L3`).
///
/// Notice:
/// - `FileGone` in legacy `P7` is `Csd5State::Gone` (`"gone"`).
/// - `Halt423` is NOT a state of `CSD/5` (it is a populate/locked/refused reading or Conscience/Halt row).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Csd5State {
    Idle,
    Loading,
    LoadingKnown,
    Error,
    Populated,
    Empty,
    Refused,
    Gone,
    Withheld,
    Lapsed,
    Ended,
    Locked,
    HiddenByRules,
    Unreviewed,
    Waiting,
    Unknown,
    NotOnThisNode,
}

impl Csd5State {
    pub const ALL: [Csd5State; 17] = [
        Csd5State::Idle,
        Csd5State::Loading,
        Csd5State::LoadingKnown,
        Csd5State::Error,
        Csd5State::Populated,
        Csd5State::Empty,
        Csd5State::Refused,
        Csd5State::Gone,
        Csd5State::Withheld,
        Csd5State::Lapsed,
        Csd5State::Ended,
        Csd5State::Locked,
        Csd5State::HiddenByRules,
        Csd5State::Unreviewed,
        Csd5State::Waiting,
        Csd5State::Unknown,
        Csd5State::NotOnThisNode,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Csd5State::Idle => "idle",
            Csd5State::Loading => "loading",
            Csd5State::LoadingKnown => "loading_known",
            Csd5State::Error => "error",
            Csd5State::Populated => "populated",
            Csd5State::Empty => "empty",
            Csd5State::Refused => "refused",
            Csd5State::Gone => "gone",
            Csd5State::Withheld => "withheld",
            Csd5State::Lapsed => "lapsed",
            Csd5State::Ended => "ended",
            Csd5State::Locked => "locked",
            Csd5State::HiddenByRules => "hidden_by_rules",
            Csd5State::Unreviewed => "unreviewed",
            Csd5State::Waiting => "waiting",
            Csd5State::Unknown => "unknown",
            Csd5State::NotOnThisNode => "not_on_this_node",
        }
    }

    /// The exact read outcome required by `L3` (*An honest zero*) for this state.
    pub fn outcome(&self) -> ReadOutcome {
        match self {
            Csd5State::Idle => ReadOutcome::Idle,
            Csd5State::Loading | Csd5State::LoadingKnown => ReadOutcome::Pending,
            Csd5State::Error => ReadOutcome::Failed,
            Csd5State::Populated
            | Csd5State::Empty
            | Csd5State::Refused
            | Csd5State::Gone
            | Csd5State::Withheld
            | Csd5State::Lapsed
            | Csd5State::Ended
            | Csd5State::Locked
            | Csd5State::HiddenByRules
            | Csd5State::Unreviewed
            | Csd5State::Waiting
            | Csd5State::Unknown
            | Csd5State::NotOnThisNode => ReadOutcome::Ok,
        }
    }

    /// Total, non-empty form description for each of the 17 states (`L3`: a missing form is a build error).
    pub fn default_form_label(&self) -> &'static str {
        match self {
            Csd5State::Idle => "Nothing asked yet",
            Csd5State::Loading => "Loading from node…",
            Csd5State::LoadingKnown => "Loading known length…",
            Csd5State::Error => "Read failed — no answer arrived or it could not be parsed",
            Csd5State::Populated => "Populated",
            Csd5State::Empty => "Nothing here yet (stated by node)",
            Csd5State::Refused => "Refused by node",
            Csd5State::Gone => "Gone — existed and was withdrawn or purged",
            Csd5State::Withheld => "Withheld under cover",
            Csd5State::Lapsed => "Lapsed — passed its end",
            Csd5State::Ended => "Ended — window ran out or question answered",
            Csd5State::Locked => "Locked — group is seating a required role",
            Csd5State::HiddenByRules => "Hidden by your policy lens rules",
            Csd5State::Unreviewed => "Unreviewed — nobody you trust has reviewed this yet",
            Csd5State::Waiting => "Waiting on another participant's hand",
            Csd5State::Unknown => "Unknown — node could not read its own state",
            Csd5State::NotOnThisNode => "Not held on this node",
        }
    }
}

/// The 5 write verbs of `CSD/5` (`manifests/transitions.json`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Csd5WriteVerb {
    Attest,
    Authorize,
    Recant,
    Supersede,
    Withdraw,
}

impl Csd5WriteVerb {
    pub const ALL: [Csd5WriteVerb; 5] = [
        Csd5WriteVerb::Attest,
        Csd5WriteVerb::Authorize,
        Csd5WriteVerb::Recant,
        Csd5WriteVerb::Supersede,
        Csd5WriteVerb::Withdraw,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Csd5WriteVerb::Attest => "attest",
            Csd5WriteVerb::Authorize => "authorize",
            Csd5WriteVerb::Recant => "recant",
            Csd5WriteVerb::Supersede => "supersede",
            Csd5WriteVerb::Withdraw => "withdraw",
        }
    }
}

/// The 12 Laws of the Screen Standard (`L1`–`L12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Csd5Law {
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
    L9,
    L10,
    L11,
    L12,
}

impl Csd5Law {
    pub const ALL: [Csd5Law; 12] = [
        Csd5Law::L1,
        Csd5Law::L2,
        Csd5Law::L3,
        Csd5Law::L4,
        Csd5Law::L5,
        Csd5Law::L6,
        Csd5Law::L7,
        Csd5Law::L8,
        Csd5Law::L9,
        Csd5Law::L10,
        Csd5Law::L11,
        Csd5Law::L12,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Csd5Law::L1 => "L1",
            Csd5Law::L2 => "L2",
            Csd5Law::L3 => "L3",
            Csd5Law::L4 => "L4",
            Csd5Law::L5 => "L5",
            Csd5Law::L6 => "L6",
            Csd5Law::L7 => "L7",
            Csd5Law::L8 => "L8",
            Csd5Law::L9 => "L9",
            Csd5Law::L10 => "L10",
            Csd5Law::L11 => "L11",
            Csd5Law::L12 => "L12",
        }
    }
}

// ============================================================================
// 2. THE 24 CLOSED UX PRIMITIVES (`Csd5Primitive`) & 60 SLOTS
// ============================================================================

/// The 24 closed UX primitives of `CSD/5` draft 3 (`manifests/ui_primitives.json`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Csd5Primitive {
    CardShell,
    FieldRow,
    ItemRow,
    ScopePill,
    Chip,
    Action,
    ReceiptSheet,
    StateBlock,
    Metric,
    ConfirmSheet,
    Choice,
    DestinationPicker,
    Field,
    StepFlow,
    Ceremony,
    LineageChain,
    CodeCompare,
    QrCode,
    Transcript,
    ConscienceStep,
    PolicyLens,
    GraphView,
    Document,
    Media,
}

impl Csd5Primitive {
    pub const ALL: [Csd5Primitive; 24] = [
        Csd5Primitive::CardShell,
        Csd5Primitive::FieldRow,
        Csd5Primitive::ItemRow,
        Csd5Primitive::ScopePill,
        Csd5Primitive::Chip,
        Csd5Primitive::Action,
        Csd5Primitive::ReceiptSheet,
        Csd5Primitive::StateBlock,
        Csd5Primitive::Metric,
        Csd5Primitive::ConfirmSheet,
        Csd5Primitive::Choice,
        Csd5Primitive::DestinationPicker,
        Csd5Primitive::Field,
        Csd5Primitive::StepFlow,
        Csd5Primitive::Ceremony,
        Csd5Primitive::LineageChain,
        Csd5Primitive::CodeCompare,
        Csd5Primitive::QrCode,
        Csd5Primitive::Transcript,
        Csd5Primitive::ConscienceStep,
        Csd5Primitive::PolicyLens,
        Csd5Primitive::GraphView,
        Csd5Primitive::Document,
        Csd5Primitive::Media,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Csd5Primitive::CardShell => "CardShell",
            Csd5Primitive::FieldRow => "FieldRow",
            Csd5Primitive::ItemRow => "ItemRow",
            Csd5Primitive::ScopePill => "ScopePill",
            Csd5Primitive::Chip => "Chip",
            Csd5Primitive::Action => "Action",
            Csd5Primitive::ReceiptSheet => "ReceiptSheet",
            Csd5Primitive::StateBlock => "StateBlock",
            Csd5Primitive::Metric => "Metric",
            Csd5Primitive::ConfirmSheet => "ConfirmSheet",
            Csd5Primitive::Choice => "Choice",
            Csd5Primitive::DestinationPicker => "DestinationPicker",
            Csd5Primitive::Field => "Field",
            Csd5Primitive::StepFlow => "StepFlow",
            Csd5Primitive::Ceremony => "Ceremony",
            Csd5Primitive::LineageChain => "LineageChain",
            Csd5Primitive::CodeCompare => "CodeCompare",
            Csd5Primitive::QrCode => "QrCode",
            Csd5Primitive::Transcript => "Transcript",
            Csd5Primitive::ConscienceStep => "ConscienceStep",
            Csd5Primitive::PolicyLens => "PolicyLens",
            Csd5Primitive::GraphView => "GraphView",
            Csd5Primitive::Document => "Document",
            Csd5Primitive::Media => "Media",
        }
    }

    /// Tag prefixes owned exclusively by this primitive (`L1`: one owner per prefix).
    pub fn prefixes(&self) -> &'static [&'static str] {
        match self {
            Csd5Primitive::CardShell => &["card"],
            Csd5Primitive::FieldRow => &["txt"],
            Csd5Primitive::ItemRow => &["row"],
            Csd5Primitive::ScopePill => &["scope"],
            Csd5Primitive::Chip => &["chip"],
            Csd5Primitive::Action => &["btn", "mi"],
            Csd5Primitive::ReceiptSheet => &["receipt"],
            Csd5Primitive::StateBlock => &["state"],
            Csd5Primitive::Metric => &["stat"],
            Csd5Primitive::ConfirmSheet => &["sheet"],
            Csd5Primitive::Choice => &["opt"],
            Csd5Primitive::DestinationPicker => &["pick"],
            Csd5Primitive::Field => &["input"],
            Csd5Primitive::StepFlow => &["step"],
            Csd5Primitive::Ceremony => &["ceremony"],
            Csd5Primitive::LineageChain => &["chain"],
            Csd5Primitive::CodeCompare => &["code"],
            Csd5Primitive::QrCode => &["qr", "scan"],
            Csd5Primitive::Transcript => &["msg", "note"],
            Csd5Primitive::ConscienceStep => &["conscience"],
            Csd5Primitive::PolicyLens => &["lens"],
            Csd5Primitive::GraphView => &["canvas"],
            Csd5Primitive::Document => &["doc"],
            Csd5Primitive::Media => &["media"],
        }
    }

    pub fn primary_prefix(&self) -> &'static str {
        self.prefixes()[0]
    }

    /// Whether instances of this primitive may carry a `.key` suffix.
    pub fn is_keyed(&self) -> bool {
        match self {
            Csd5Primitive::ItemRow
            | Csd5Primitive::ReceiptSheet
            | Csd5Primitive::Choice
            | Csd5Primitive::DestinationPicker
            | Csd5Primitive::StepFlow
            | Csd5Primitive::Ceremony
            | Csd5Primitive::LineageChain
            | Csd5Primitive::Transcript
            | Csd5Primitive::ConscienceStep
            | Csd5Primitive::Document
            | Csd5Primitive::Media => true,
            Csd5Primitive::CardShell
            | Csd5Primitive::FieldRow
            | Csd5Primitive::ScopePill
            | Csd5Primitive::Chip
            | Csd5Primitive::Action
            | Csd5Primitive::StateBlock
            | Csd5Primitive::Metric
            | Csd5Primitive::ConfirmSheet
            | Csd5Primitive::Field
            | Csd5Primitive::CodeCompare
            | Csd5Primitive::QrCode
            | Csd5Primitive::PolicyLens
            | Csd5Primitive::GraphView => false,
        }
    }

    /// Closed slot list for this primitive (60 slots across the 24 primitives in `ui_primitives.json`).
    pub fn slots(&self) -> &'static [&'static str] {
        match self {
            Csd5Primitive::CardShell => &["title"],
            Csd5Primitive::FieldRow => &["wire", "gloss"],
            Csd5Primitive::ItemRow => &["flag", "act"],
            Csd5Primitive::ScopePill => &["group"],
            Csd5Primitive::Chip => &[],
            Csd5Primitive::Action => &["reason"],
            Csd5Primitive::ReceiptSheet => &[
                "open", "fact_1", "fact_2", "fact_3", "fact_4", "fact_5", "holders", "notes",
                "acts",
            ],
            Csd5Primitive::StateBlock => &["@states", "freshness"],
            Csd5Primitive::Metric => &["unit", "source"],
            Csd5Primitive::ConfirmSheet => {
                &["fact_1", "fact_2", "fact_3", "note", "confirm", "cancel"]
            }
            Csd5Primitive::Choice => &["cost", "tally"],
            Csd5Primitive::DestinationPicker => &["group"],
            Csd5Primitive::Field => &["ceiling", "error"],
            Csd5Primitive::StepFlow => &["next", "back"],
            Csd5Primitive::Ceremony => {
                &["act_price", "undo_price", "window", "threshold", "signer"]
            }
            Csd5Primitive::LineageChain => &["link", "cascade"],
            Csd5Primitive::CodeCompare => &["words", "digits", "verdict"],
            Csd5Primitive::QrCode => &["paste"],
            Csd5Primitive::Transcript => &["receipt", "fetch"],
            Csd5Primitive::ConscienceStep => &["cost", "faculty"],
            Csd5Primitive::PolicyLens => &["hidden_count", "helpers", "depth"],
            Csd5Primitive::GraphView => &["twin"],
            Csd5Primitive::Document => &["claim", "recant", "revision", "cite"],
            Csd5Primitive::Media => &["alt", "ai_disclosure", "blur", "class"],
        }
    }

    /// Checks whether `slot` is a legal slot for this primitive (handling `@states` expansion for `StateBlock`).
    pub fn allows_slot(&self, slot: &str) -> bool {
        if *self == Csd5Primitive::StateBlock {
            if slot == "freshness" || slot == "@states" {
                return true;
            }
            return Csd5State::ALL.iter().any(|s| s.as_str() == slot);
        }
        self.slots().contains(&slot)
    }

    /// Look up the single owning primitive for a prefix (`L1`).
    pub fn from_prefix(prefix: &str) -> Option<Csd5Primitive> {
        Csd5Primitive::ALL
            .iter()
            .copied()
            .find(|p| p.prefixes().contains(&prefix))
    }
}

// ============================================================================
// 3. TAG GRAMMAR BUILDER (`L1` & `INTERFACE.md` §10.1: slot BEFORE key!)
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TagError {
    PrefixNotOwnedByPrimitive {
        primitive: Csd5Primitive,
        prefix: String,
    },
    InvalidSurface(String),
    InvalidInstance(String),
    InvalidSlot {
        primitive: Csd5Primitive,
        slot: String,
    },
    KeyNotAllowedForPrimitive {
        primitive: Csd5Primitive,
        key: String,
    },
    InvalidKey(String),
}

fn is_lower_snake(s: &str) -> bool {
    if s.is_empty() || s.starts_with('_') || s.ends_with('_') || s.contains("__") {
        return false;
    }
    s.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn is_valid_key(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == ':' || c == '_' || c == '-')
}

/// Canonical `CSD/5` tag generator (`L1` & `INTERFACE.md` §10.1):
/// `prefix '_' surface '_' instance [ '__' slot ] [ '.' key ]`
///
/// Enforces:
/// 1. `prefix` belongs to `primitive`.
/// 2. `surface` (with `-` normalized to `_`) and `instance` are valid lower-snake identifiers.
/// 3. `slot`, if present, is in `primitive.slots()` (or one of the 17 states for `StateBlock`).
/// 4. `key`, if present, is valid (`^[A-Za-z0-9:_\-]+$`) and `primitive` is keyed (or the slot is keyed,
///    such as `Ceremony`'s `__signer.<who>` or `ReceiptSheet`'s `__acts.<act>`).
/// 5. **Slot comes BEFORE key** (`row_s_i__flag.k`, never `row_s_i.k__flag`).
pub fn derive_csd5_tag(
    primitive: Csd5Primitive,
    prefix: Option<&str>,
    surface: &str,
    instance: &str,
    slot: Option<&str>,
    key: Option<&str>,
) -> Result<String, TagError> {
    let pfx = prefix.unwrap_or_else(|| primitive.primary_prefix());
    if !primitive.prefixes().contains(&pfx) {
        return Err(TagError::PrefixNotOwnedByPrimitive {
            primitive,
            prefix: pfx.to_string(),
        });
    }
    let norm_surface = surface.replace('-', "_");
    if !is_lower_snake(&norm_surface) {
        return Err(TagError::InvalidSurface(surface.to_string()));
    }
    let norm_instance = instance.replace('-', "_");
    if !is_lower_snake(&norm_instance) {
        return Err(TagError::InvalidInstance(instance.to_string()));
    }
    if let Some(sl) = slot {
        if !primitive.allows_slot(sl) {
            return Err(TagError::InvalidSlot {
                primitive,
                slot: sl.to_string(),
            });
        }
    }
    if let Some(k) = key {
        if !primitive.is_keyed() {
            return Err(TagError::KeyNotAllowedForPrimitive {
                primitive,
                key: k.to_string(),
            });
        }
        if !is_valid_key(k) {
            return Err(TagError::InvalidKey(k.to_string()));
        }
    }

    let mut out = format!("{pfx}_{norm_surface}_{norm_instance}");
    if let Some(sl) = slot {
        out.push_str("__");
        out.push_str(sl);
    }
    if let Some(k) = key {
        out.push('.');
        out.push_str(k);
    }
    Ok(out)
}

/// Canonical `CSD/5` shell tag generator: `shell_prefix '_' name`.
pub fn derive_shell_tag(prefix: ShellPrefix, name: &str) -> Result<String, TagError> {
    let norm = name.replace('-', "_");
    if !is_lower_snake(&norm) {
        return Err(TagError::InvalidInstance(name.to_string()));
    }
    Ok(format!("{}_{norm}", prefix.as_str()))
}

// ============================================================================
// 4. TYPED INSTANCES OF ALL 24 CSD/5 PRIMITIVES (`Csd5Node`)
// ============================================================================

/// Universal 24-primitive `CSD/5` typed screen node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Csd5Node {
    /// Shell element (`circle_<name>`, `tab_<name>`, `nav_<name>`, `screen_<name>`).
    Shell {
        prefix: ShellPrefix,
        name: String,
        label: String,
        active: bool,
    },
    /// 1. `CardShell` (`card`): slots `["title"]`, `keyed = false`.
    CardShell {
        surface: String,
        instance: String,
        title: String,
        children: Vec<Csd5Node>,
    },
    /// 2. `FieldRow` (`txt`): slots `["wire", "gloss"]`, `keyed = false`.
    /// Note: A value the node did not send is drawn as `"This node did not send this"`, never blank.
    FieldRow {
        surface: String,
        instance: String,
        label: String,
        wire_value: Option<String>,
        gloss: Option<String>,
    },
    /// 3. `ItemRow` (`row`): slots `["flag", "act"]`, `keyed = true`.
    ItemRow {
        surface: String,
        instance: String,
        key: String,
        title: String,
        subtitle: String,
        flag: Option<String>,
        act_label: Option<String>,
        scope_pill: Option<Box<Csd5Node>>,
        receipt_open_tag: Option<String>,
    },
    /// 4. `ScopePill` (`scope`): slots `["group"]`, `keyed = false`.
    ScopePill {
        surface: String,
        instance: String,
        circle_label: String,
        group: Option<String>,
    },
    /// 5. `Chip` (`chip`): slots `[]`, `keyed = false`.
    /// Note: One status token sent by the node. Interactive choices use `Choice` (`opt`), not `Chip`.
    Chip {
        surface: String,
        instance: String,
        token: String,
    },
    /// 6. `Action` (`btn` or `mi`): slots `["reason"]`, `keyed = false`, laws `L5, L6, L12`.
    Action {
        surface: String,
        instance: String,
        in_menu: bool,
        label: String,
        enabled: bool,
        disabled_reason: Option<String>,
        requires_confirm: bool,
    },
    /// 7. `ReceiptSheet` (`receipt`): slots `["open", "fact_1".."fact_5", "holders", "notes", "acts"]`, `keyed = true`.
    ReceiptSheet {
        surface: String,
        instance: String,
        key: String,
        is_open: bool,
        facts: [String; 5],
        holders: Vec<String>,
        notes: Vec<String>,
        acts: Vec<String>,
    },
    /// 8. `StateBlock` (`state`): slots `["@states", "freshness"]`, `keyed = false`, binds `read`, laws `L2, L3, L9`.
    StateBlock {
        surface: String,
        instance: String,
        outcome: ReadOutcome,
        state: Csd5State,
        detail_text: Option<String>,
        freshness: Option<String>,
    },
    /// 9. `Metric` (`stat`): slots `["unit", "source"]`, `keyed = false`, binds `read`, laws `L4`.
    /// Note: Not sent or not read is drawn as an em dash (`"—"`), never as zero.
    Metric {
        surface: String,
        instance: String,
        outcome: ReadOutcome,
        value: Option<String>,
        unit: String,
        source: String,
    },
    /// 10. `ConfirmSheet` (`sheet`): slots `["fact_1", "fact_2", "fact_3", "note", "confirm", "cancel"]`, `keyed = false`, laws `L6, L7, L12`.
    ConfirmSheet {
        surface: String,
        instance: String,
        fact_1: String,
        fact_2: String,
        fact_3: String,
        note: Option<String>,
        confirm_label: String,
        cancel_label: String,
    },
    /// 11. `Choice` (`opt`): slots `["cost", "tally"]`, `keyed = true`, laws `L12`.
    Choice {
        surface: String,
        instance: String,
        question: String,
        options: Vec<Csd5ChoiceOption>,
    },
    /// 12. `DestinationPicker` (`pick`): slots `["group"]`, `keyed = true`.
    DestinationPicker {
        surface: String,
        instance: String,
        destinations: Vec<(String, String, Option<String>)>, // (key, label, optional group)
    },
    /// 13. `Field` (`input`): slots `["ceiling", "error"]`, `keyed = false`, laws `L12`.
    Field {
        surface: String,
        instance: String,
        value: String,
        byte_ceiling: u32,
        error: Option<String>,
    },
    /// 14. `StepFlow` (`step`): slots `["next", "back"]`, `keyed = true`.
    /// Note: Each step is a key; a skipped step is absent, not greyed.
    StepFlow {
        surface: String,
        instance: String,
        steps: Vec<(String, String)>, // (key, title)
        active_key: String,
        has_back: bool,
        has_next: bool,
    },
    /// 15. `Ceremony` (`ceremony`): slots `["act_price", "undo_price", "window", "threshold", "signer"]`, `keyed = true`.
    Ceremony {
        surface: String,
        instance: String,
        key: String,
        act_price: String,
        undo_price: String,
        window: String,
        threshold: String,
        signers: Vec<(String, String)>, // (who_key, status)
    },
    /// 16. `LineageChain` (`chain`): slots `["link", "cascade"]`, `keyed = true`.
    LineageChain {
        surface: String,
        instance: String,
        key: String,
        links: Vec<(String, String)>, // (link_key, label)
        cascade: Option<String>,
    },
    /// 17. `CodeCompare` (`code`): slots `["words", "digits", "verdict"]`, `keyed = false`.
    CodeCompare {
        surface: String,
        instance: String,
        words: String,
        digits: String,
        verdict: String,
    },
    /// 18. `QrCode` (`qr`, `scan`): slots `["paste"]`, `keyed = false`.
    QrCode {
        surface: String,
        instance: String,
        is_scanner: bool,
        payload: String,
        has_paste: bool,
    },
    /// 19. `Transcript` (`msg`, `note`): slots `["receipt", "fetch"]`, `keyed = true`.
    Transcript {
        surface: String,
        instance: String,
        is_note: bool,
        entries: Vec<(String, String, bool)>, // (key, text, has_receipt)
        has_fetch: bool,
    },
    /// 20. `ConscienceStep` (`conscience`): slots `["cost", "faculty"]`, `keyed = true`.
    ConscienceStep {
        surface: String,
        instance: String,
        key: String,
        summary: String,
        cost: String,
        faculties: Vec<(String, String)>, // (faculty_key, verdict) — 4 faculties (CC 3.1.5.3)
    },
    /// 21. `PolicyLens` (`lens`): slots `["hidden_count", "helpers", "depth"]`, `keyed = false`.
    PolicyLens {
        surface: String,
        instance: String,
        hidden_count: u32,
        helpers: Vec<String>,
        depth: String,
    },
    /// 22. `GraphView` (`canvas`): slots `["twin"]`, `keyed = false`.
    /// Note: The list twin is `ItemRow` instances (`row`); the canvas owns `canvas`.
    GraphView {
        surface: String,
        instance: String,
        world_nodes: Vec<GraphMarkSpec>,
        twin_rows: Vec<Csd5Node>,
    },
    /// 23. `Document` (`doc`): slots `["claim", "recant", "revision", "cite"]`, `keyed = true`.
    /// Note: A recanted claim is struck with its reason beside it, never removed.
    Document {
        surface: String,
        instance: String,
        key: String,
        revision: String,
        claims: Vec<DocumentClaimSpec>,
    },
    /// 24. `Media` (`media`): slots `["alt", "ai_disclosure", "blur", "class"]`, `keyed = true`, laws `L10`.
    /// Note: Cannot be drawn without `alt` and `ai_disclosure` (`L10`).
    Media {
        surface: String,
        instance: String,
        key: String,
        alt: String,
        ai_disclosure: String,
        blurred: bool,
        media_class: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Csd5ChoiceOption {
    pub key: String,
    pub label: String,
    pub cost: String,
    pub tally: Option<String>,
    pub selected: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphMarkSpec {
    pub key: String,
    pub label: String,
    pub pos: [f32; 3],
    pub radius: f32,
    pub rgba: [u8; 4],
    pub bonded_to_pos: Option<[f32; 3]>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentClaimSpec {
    pub claim_key: String,
    pub text: String,
    pub recanted_reason: Option<String>,
    pub citation: Option<String>,
}

// ============================================================================
// 5. HANDLE TABLE & `holon-draw` PROFILE B WHOLE-SCREEN COMPILER
// ============================================================================

/// Entry in the consumer-owned handle table (`INTERFACE.md` §6, §8, §9).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandleEntry {
    pub handle: u32,
    pub tag: String,
    pub primitive: Option<Csd5Primitive>,
    pub slot: Option<String>,
    /// The exact string rendered by this instance, for `L12` (`shown_digest`) verification.
    pub rendered_text: Option<String>,
    /// FNV-1a 64-bit digest of `rendered_text` (`INTERFACE.md` §9).
    pub text_digest: u64,
}

/// Consumer-owned table mapping `u32` `holon_draw::Instance::tag` handles (`1..=N`)
/// to their derived `CSD/5` string tags (`L1`: 1-to-1 bijection).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandleTable {
    by_handle: BTreeMap<u32, HandleEntry>,
    by_tag: BTreeMap<String, u32>,
}

impl HandleTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Interns a derived `CSD/5` tag and returns its `u32` handle (`>= 1`).
    /// If the tag was already interned, returns the existing handle (`L1`: one tag per instance).
    pub fn intern(
        &mut self,
        tag: String,
        primitive: Option<Csd5Primitive>,
        slot: Option<String>,
        rendered_text: Option<String>,
    ) -> u32 {
        if let Some(&existing) = self.by_tag.get(&tag) {
            return existing;
        }
        let handle = (self.by_handle.len() as u32) + 1;
        let text_digest = rendered_text
            .as_deref()
            .map(fnv1a_str)
            .unwrap_or(0xcbf2_9ce4_8422_2325);
        let entry = HandleEntry {
            handle,
            tag: tag.clone(),
            primitive,
            slot,
            rendered_text,
            text_digest,
        };
        self.by_handle.insert(handle, entry);
        self.by_tag.insert(tag, handle);
        handle
    }

    pub fn get_by_handle(&self, handle: u32) -> Option<&HandleEntry> {
        self.by_handle.get(&handle)
    }

    pub fn get_by_tag(&self, tag: &str) -> Option<&HandleEntry> {
        let h = self.by_tag.get(tag)?;
        self.by_handle.get(h)
    }

    pub fn entries(&self) -> impl Iterator<Item = &HandleEntry> {
        self.by_handle.values()
    }

    pub fn len(&self) -> usize {
        self.by_handle.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_handle.is_empty()
    }
}

pub fn fnv1a_str(s: &str) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &b in s.as_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
    h
}

/// Violation of one of the 12 `CSD/5` screen laws (`L1`–`L12`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LawViolation {
    pub law: Csd5Law,
    pub tag: String,
    pub message: String,
}

/// Compiled `holon-draw` Profile B Whole-Screen output:
/// - `draw_scene`: the `holon_draw::Scene` ready for `draw_scene.frame(&cam, version, &Acuity { eps_px: 1.0 }, budget)`
/// - `handles`: the `HandleTable` mapping `u32` handles to derived `CSD/5` tags
/// - `violations`: any `CSD/5` law violations caught during compilation (`L1`–`L12`)
#[derive(Clone, Debug, PartialEq)]
pub struct CompiledHolonScreen {
    pub version: u64,
    pub draw_scene: DrawScene,
    pub handles: HandleTable,
    pub violations: Vec<LawViolation>,
}

impl CompiledHolonScreen {
    /// Evaluate the `holon_draw::Scene` into a packed `holon_draw::Frame` at the given camera and acuity budget.
    pub fn evaluate_frame(&self, cam: &Camera, eps_px: f64, budget: u64) -> Result<Frame, holon_draw::Refusal> {
        self.draw_scene
            .frame(cam, self.version, &Acuity { eps_px }, budget)
    }

    /// Pure pick (`S2`, `L1`): returns the derived `CSD/5` tag of the instance under `(x, y)` in device pixels.
    pub fn pick_tag<'a>(&'a self, frame: &Frame, x: f64, y: f64) -> Option<&'a HandleEntry> {
        let handle = frame.pick(x, y)?;
        self.handles.get_by_handle(handle)
    }

    /// Report (`S3`, `L12`) for a derived `CSD/5` tag on the given frame.
    pub fn report_tag(&self, frame: &Frame, tag: &str) -> Option<(Shown, &HandleEntry)> {
        let entry = self.handles.get_by_tag(tag)?;
        let shown = frame.report(entry.handle)?;
        Some((shown, entry))
    }

    /// Verifies `L12` (*Shown is sent*) before allowing a write:
    /// Every tag in `required_value_tags` must be `on_screen`, `legible`, and its `text_digest`
    /// must match the expected value string's FNV-1a digest.
    pub fn verify_l12_shown_is_sent(
        &self,
        frame: &Frame,
        sender_action_or_confirm_tag: &str,
        required_values: &[(&str, &str)],
    ) -> Result<u64, LawViolation> {
        let (sender_shown, _) = self
            .report_tag(frame, sender_action_or_confirm_tag)
            .ok_or_else(|| LawViolation {
                law: Csd5Law::L12,
                tag: sender_action_or_confirm_tag.to_string(),
                message: "Sending Action or ConfirmSheet confirm button is not drawn on this frame"
                    .to_string(),
            })?;
        if !sender_shown.on_screen || !sender_shown.legible {
            return Err(LawViolation {
                law: Csd5Law::L12,
                tag: sender_action_or_confirm_tag.to_string(),
                message: "Sending control is not on screen and legible".to_string(),
            });
        }
        let mut combined_digest = sender_shown.shown;
        for (val_tag, expected_str) in required_values {
            let (val_shown, entry) =
                self.report_tag(frame, val_tag).ok_or_else(|| LawViolation {
                    law: Csd5Law::L12,
                    tag: (*val_tag).to_string(),
                    message: format!("Sent value tag '{val_tag}' is not drawn on this frame"),
                })?;
            if !val_shown.on_screen || !val_shown.legible {
                return Err(LawViolation {
                    law: Csd5Law::L12,
                    tag: (*val_tag).to_string(),
                    message: format!("Sent value tag '{val_tag}' is not legible on screen"),
                });
            }
            let expected_hash = fnv1a_str(expected_str);
            if entry.text_digest != expected_hash {
                return Err(LawViolation {
                    law: Csd5Law::L12,
                    tag: (*val_tag).to_string(),
                    message: format!(
                        "Shown text {:?} does not match sent value {:?}",
                        entry.rendered_text, expected_str
                    ),
                });
            }
            combined_digest ^= entry.text_digest;
            combined_digest = combined_digest.wrapping_mul(0x0000_0100_0000_01B3);
        }
        Ok(combined_digest)
    }
}

/// Colours matching `holon_draw::ui` and CEWPOS spatial slabs.
const SLAB_BG: [u8; 4] = [0x14, 0x1c, 0x26, 235];
const SLAB_BORDER: [u8; 4] = [0x4b, 0x6b, 0x94, 140];
const TEXT_PAPER: [u8; 4] = [0xed, 0xe9, 0xdf, 255];
const TEXT_MUTED: [u8; 4] = [0x8b, 0x98, 0xa9, 255];
const ACCENT_GREEN: [u8; 4] = [0x23, 0x69, 0x57, 240];
const ACCENT_AMBER: [u8; 4] = [0xd4, 0x9a, 0x36, 255];
const ACCENT_RED: [u8; 4] = [0xbb, 0x4c, 0x2e, 255];
const ACCENT_BLUE: [u8; 4] = [0x2b, 0x5b, 0x84, 240];

/// Push a tagged `Kind::Quad` (in device pixels) and a tagged `Kind::Run` label onto `marks`.
#[allow(clippy::too_many_arguments)]
fn emit_box_and_run(
    marks: &mut Vec<Instance>,
    text_provider: &mut dyn Text,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    radius: f32,
    bg: [u8; 4],
    fg: [u8; 4],
    style: u8,
    label: &str,
    handle: u32,
    pin: bool,
) {
    let mut q = Instance::new(Kind::Quad, [x + 0.5 * w, y + 0.5 * h, 0.0], [w, h, 0.0])
        .colour(bg)
        .tagged(handle);
    q.aux[0] = radius;
    if pin {
        q = q.pinned();
    }
    marks.push(q);

    if !label.trim().is_empty() {
        let p = text_provider.place(label, style);
        if p.w > 0.0 && p.h > 0.0 {
            let rw = p.w.min(w - 8.0).max(4.0);
            let mut r = Instance::new(
                Kind::Run,
                [x + 6.0 + 0.5 * rw, y + 0.5 * h, 0.0],
                [rw, p.h, 0.0],
            )
            .colour(fg)
            .tagged(handle);
            r.aux = [p.u0, p.v0, p.u1];
            r.angle = p.v1;
            r.variant = p.page;
            if pin {
                r = r.pinned();
            }
            marks.push(r);
        }
    }
}

/// Push a tagged `Kind::Glyph` + `Kind::Run` pair onto `marks`.
#[allow(clippy::too_many_arguments)]
fn emit_glyph_and_run(
    marks: &mut Vec<Instance>,
    text_provider: &mut dyn Text,
    x: f32,
    y: f32,
    h: f32,
    fg: [u8; 4],
    style: u8,
    label: &str,
    handle: u32,
    pin: bool,
) {
    let mut g = Instance::new(Kind::Glyph, [x + 6.0, y + 0.5 * h, 0.0], [4.0, 0.0, 0.0])
        .colour(fg)
        .tagged(handle);
    if pin {
        g = g.pinned();
    }
    marks.push(g);

    let p = text_provider.place(label, style);
    let rw = p.w.max(8.0);
    let mut r = Instance::new(
        Kind::Run,
        [x + 16.0 + 0.5 * rw, y + 0.5 * h, 0.0],
        [rw, p.h.max(12.0), 0.0],
    )
    .colour(fg)
    .tagged(handle);
    r.aux = [p.u0, p.v0, p.u1];
    r.angle = p.v1;
    r.variant = p.page;
    if pin {
        r = r.pinned();
    }
    marks.push(r);
}

/// Compile a slice of `Csd5Node`s (using all 24 `CSD/5` primitives) into a `CompiledHolonScreen`
/// backed by `holon_draw::Scene` tiles and a consumer `HandleTable`.
pub fn compile_csd5_screen(
    nodes: &[Csd5Node],
    text_provider: &mut dyn Text,
    version: u64,
) -> CompiledHolonScreen {
    let mut handles = HandleTable::new();
    let mut violations = Vec::new();
    let mut tiles = Vec::new();
    let mut modal_layer: Option<u8> = None;
    let mut next_tile_id = 1u64;
    let mut cursor_y = 12.0f32;

    for node in nodes {
        compile_one_node(
            node,
            text_provider,
            version,
            &mut handles,
            &mut violations,
            &mut tiles,
            &mut modal_layer,
            &mut next_tile_id,
            20.0,
            &mut cursor_y,
            760.0,
            0,
        );
        cursor_y += 10.0;
    }

    CompiledHolonScreen {
        version,
        draw_scene: DrawScene {
            tiles,
            modal: modal_layer,
        },
        handles,
        violations,
    }
}

#[allow(clippy::too_many_arguments)]
fn compile_one_node(
    node: &Csd5Node,
    text: &mut dyn Text,
    version: u64,
    handles: &mut HandleTable,
    violations: &mut Vec<LawViolation>,
    tiles: &mut Vec<Tile>,
    modal_layer: &mut Option<u8>,
    next_tile_id: &mut u64,
    x: f32,
    y: &mut f32,
    w: f32,
    layer: u8,
) {
    match node {
        Csd5Node::Shell {
            prefix,
            name,
            label,
            active,
        } => {
            if let Ok(tag) = derive_shell_tag(*prefix, name) {
                let h = handles.intern(tag, None, None, Some(label.clone()));
                let mut marks = Vec::new();
                emit_box_and_run(
                    &mut marks,
                    text,
                    x,
                    *y,
                    150.0,
                    28.0,
                    4.0,
                    if *active { ACCENT_GREEN } else { SLAB_BG },
                    TEXT_PAPER,
                    BUTTON,
                    label,
                    h,
                    true,
                );
                let tid = *next_tile_id;
                *next_tile_id += 1;
                tiles.push(Tile {
                    id: tid,
                    origin: [0.0; 3],
                    version,
                    layer,
                    screen: true,
                    params: PARAMS_NEUTRAL,
                    marks,
                });
                *y += 32.0;
            }
        }

        Csd5Node::CardShell {
            surface,
            instance,
            title,
            children,
        } => {
            let root_tag =
                derive_csd5_tag(Csd5Primitive::CardShell, None, surface, instance, None, None)
                    .expect("valid CardShell tag");
            let title_tag = derive_csd5_tag(
                Csd5Primitive::CardShell,
                None,
                surface,
                instance,
                Some("title"),
                None,
            )
            .expect("valid CardShell__title tag");

            let h_root = handles.intern(root_tag, Some(Csd5Primitive::CardShell), None, None);
            let h_title = handles.intern(
                title_tag,
                Some(Csd5Primitive::CardShell),
                Some("title".into()),
                Some(title.clone()),
            );

            let top_y = *y;
            *y += 32.0;
            for child in children {
                compile_one_node(
                    child,
                    text,
                    version,
                    handles,
                    violations,
                    tiles,
                    modal_layer,
                    next_tile_id,
                    x + 12.0,
                    y,
                    w - 24.0,
                    layer + 1,
                );
            }
            let total_h = (*y - top_y + 10.0).max(44.0);
            let mut marks = Vec::new();
            let mut bg = Instance::new(
                Kind::Quad,
                [x + 0.5 * w, top_y + 0.5 * total_h, 0.0],
                [w, total_h, 0.0],
            )
            .colour(SLAB_BG)
            .tagged(h_root);
            bg.aux[0] = 6.0;
            marks.push(bg);

            emit_box_and_run(
                &mut marks,
                text,
                x + 8.0,
                top_y + 6.0,
                w - 16.0,
                22.0,
                3.0,
                SLAB_BORDER,
                TEXT_PAPER,
                HEADING,
                title,
                h_title,
                false,
            );
            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y = top_y + total_h + 6.0;
        }

        Csd5Node::FieldRow {
            surface,
            instance,
            label,
            wire_value,
            gloss,
        } => {
            let root_tag =
                derive_csd5_tag(Csd5Primitive::FieldRow, None, surface, instance, None, None)
                    .expect("valid FieldRow tag");
            let wire_tag = derive_csd5_tag(
                Csd5Primitive::FieldRow,
                None,
                surface,
                instance,
                Some("wire"),
                None,
            )
            .expect("valid FieldRow__wire tag");

            // CSD/5 FieldRow note & L4: A value the node did not send is drawn as "This node did not send this", never blank.
            let wire_str = match wire_value {
                Some(v) if !v.trim().is_empty() => v.clone(),
                _ => "This node did not send this".to_string(),
            };

            let h_root = handles.intern(
                root_tag,
                Some(Csd5Primitive::FieldRow),
                None,
                Some(label.clone()),
            );
            let h_wire = handles.intern(
                wire_tag,
                Some(Csd5Primitive::FieldRow),
                Some("wire".into()),
                Some(wire_str.clone()),
            );

            let mut marks = Vec::new();
            emit_box_and_run(
                &mut marks,
                text,
                x,
                *y,
                0.4 * w,
                22.0,
                2.0,
                SLAB_BG,
                TEXT_MUTED,
                BODY,
                label,
                h_root,
                false,
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 0.42 * w,
                *y,
                0.35 * w,
                22.0,
                2.0,
                SLAB_BG,
                TEXT_PAPER,
                BODY,
                &wire_str,
                h_wire,
                false,
            );

            if let Some(g) = gloss {
                let gloss_tag = derive_csd5_tag(
                    Csd5Primitive::FieldRow,
                    None,
                    surface,
                    instance,
                    Some("gloss"),
                    None,
                )
                .expect("valid FieldRow__gloss tag");
                let h_gloss = handles.intern(
                    gloss_tag,
                    Some(Csd5Primitive::FieldRow),
                    Some("gloss".into()),
                    Some(g.clone()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + 0.78 * w,
                    *y,
                    0.22 * w,
                    22.0,
                    2.0,
                    SLAB_BG,
                    TEXT_MUTED,
                    SMALL,
                    g,
                    h_gloss,
                    false,
                );
            }

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += 26.0;
        }

        Csd5Node::ItemRow {
            surface,
            instance,
            key,
            title,
            subtitle,
            flag,
            act_label,
            scope_pill,
            receipt_open_tag,
        } => {
            let root_tag = derive_csd5_tag(
                Csd5Primitive::ItemRow,
                None,
                surface,
                instance,
                None,
                Some(key),
            )
            .expect("valid ItemRow tag");
            let full_text = format!("{title} — {subtitle}");
            let h_root = handles.intern(
                root_tag,
                Some(Csd5Primitive::ItemRow),
                None,
                Some(full_text.clone()),
            );

            let mut marks = Vec::new();
            emit_box_and_run(
                &mut marks,
                text,
                x,
                *y,
                0.55 * w,
                28.0,
                4.0,
                SLAB_BG,
                TEXT_PAPER,
                BODY,
                &full_text,
                h_root,
                false,
            );

            if let Some(fl) = flag {
                let flag_tag = derive_csd5_tag(
                    Csd5Primitive::ItemRow,
                    None,
                    surface,
                    instance,
                    Some("flag"),
                    Some(key),
                )
                .expect("valid ItemRow__flag.key tag");
                let h_flag = handles.intern(
                    flag_tag,
                    Some(Csd5Primitive::ItemRow),
                    Some("flag".into()),
                    Some(fl.clone()),
                );
                emit_glyph_and_run(
                    &mut marks,
                    text,
                    x + 0.56 * w,
                    *y,
                    28.0,
                    ACCENT_AMBER,
                    SMALL,
                    fl,
                    h_flag,
                    false,
                );
            }

            if let Some(act) = act_label {
                let act_tag = derive_csd5_tag(
                    Csd5Primitive::ItemRow,
                    None,
                    surface,
                    instance,
                    Some("act"),
                    Some(key),
                )
                .expect("valid ItemRow__act.key tag");
                let h_act = handles.intern(
                    act_tag,
                    Some(Csd5Primitive::ItemRow),
                    Some("act".into()),
                    Some(act.clone()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + 0.72 * w,
                    *y + 2.0,
                    90.0,
                    24.0,
                    4.0,
                    ACCENT_GREEN,
                    TEXT_PAPER,
                    BUTTON,
                    act,
                    h_act,
                    true,
                );
            }

            if let Some(rcpt_open) = receipt_open_tag {
                let h_open = handles.intern(
                    rcpt_open.clone(),
                    Some(Csd5Primitive::ReceiptSheet),
                    Some("open".into()),
                    Some("☰".into()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + w - 42.0,
                    *y + 2.0,
                    36.0,
                    24.0,
                    4.0,
                    ACCENT_BLUE,
                    TEXT_PAPER,
                    BUTTON,
                    "☰",
                    h_open,
                    true,
                );
            }

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += 32.0;

            if let Some(sp) = scope_pill {
                compile_one_node(
                    sp,
                    text,
                    version,
                    handles,
                    violations,
                    tiles,
                    modal_layer,
                    next_tile_id,
                    x + 12.0,
                    y,
                    w - 24.0,
                    layer,
                );
            }
        }

        Csd5Node::ScopePill {
            surface,
            instance,
            circle_label,
            group,
        } => {
            let root_tag =
                derive_csd5_tag(Csd5Primitive::ScopePill, None, surface, instance, None, None)
                    .expect("valid ScopePill tag");
            let h_root = handles.intern(
                root_tag,
                Some(Csd5Primitive::ScopePill),
                None,
                Some(circle_label.clone()),
            );
            let mut marks = Vec::new();
            emit_box_and_run(
                &mut marks,
                text,
                x,
                *y,
                140.0,
                22.0,
                11.0,
                ACCENT_BLUE,
                TEXT_PAPER,
                SMALL,
                circle_label,
                h_root,
                false,
            );
            if let Some(grp) = group {
                let grp_tag = derive_csd5_tag(
                    Csd5Primitive::ScopePill,
                    None,
                    surface,
                    instance,
                    Some("group"),
                    None,
                )
                .expect("valid ScopePill__group tag");
                let h_grp = handles.intern(
                    grp_tag,
                    Some(Csd5Primitive::ScopePill),
                    Some("group".into()),
                    Some(grp.clone()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + 148.0,
                    *y,
                    120.0,
                    22.0,
                    11.0,
                    SLAB_BORDER,
                    TEXT_PAPER,
                    SMALL,
                    grp,
                    h_grp,
                    false,
                );
            }
            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += 26.0;
        }

        Csd5Node::Chip {
            surface,
            instance,
            token,
        } => {
            let root_tag =
                derive_csd5_tag(Csd5Primitive::Chip, None, surface, instance, None, None)
                    .expect("valid Chip tag");
            let h_root = handles.intern(
                root_tag,
                Some(Csd5Primitive::Chip),
                None,
                Some(token.clone()),
            );
            let mut marks = Vec::new();
            emit_box_and_run(
                &mut marks,
                text,
                x,
                *y,
                110.0,
                22.0,
                11.0,
                SLAB_BORDER,
                TEXT_PAPER,
                SMALL,
                token,
                h_root,
                false,
            );
            // INTERFACE.md §9: Chip is quad (radius) + glyph + run (never colour alone)
            marks.push(
                Instance::new(Kind::Glyph, [x + 98.0, *y + 11.0, 0.0], [3.0, 0.0, 0.0])
                    .colour(TEXT_PAPER)
                    .tagged(h_root),
            );
            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += 26.0;
        }

        Csd5Node::Action {
            surface,
            instance,
            in_menu,
            label,
            enabled,
            disabled_reason,
            ..
        } => {
            let pfx = if *in_menu { "mi" } else { "btn" };
            let root_tag =
                derive_csd5_tag(Csd5Primitive::Action, Some(pfx), surface, instance, None, None)
                    .expect("valid Action tag");
            let h_root = handles.intern(
                root_tag.clone(),
                Some(Csd5Primitive::Action),
                None,
                Some(label.clone()),
            );
            let mut marks = Vec::new();
            emit_box_and_run(
                &mut marks,
                text,
                x,
                *y,
                160.0,
                34.0,
                4.0,
                if *enabled { ACCENT_GREEN } else { SLAB_BORDER },
                TEXT_PAPER,
                BUTTON,
                label,
                h_root,
                true, // L12: Action is pinned
            );
            if !*enabled {
                match disabled_reason {
                    Some(reason) if !reason.trim().is_empty() => {
                        let reason_tag = derive_csd5_tag(
                            Csd5Primitive::Action,
                            Some(pfx),
                            surface,
                            instance,
                            Some("reason"),
                            None,
                        )
                        .expect("valid Action__reason tag");
                        let h_reason = handles.intern(
                            reason_tag,
                            Some(Csd5Primitive::Action),
                            Some("reason".into()),
                            Some(reason.clone()),
                        );
                        emit_box_and_run(
                            &mut marks,
                            text,
                            x + 168.0,
                            *y + 4.0,
                            220.0,
                            26.0,
                            3.0,
                            SLAB_BG,
                            ACCENT_AMBER,
                            SMALL,
                            reason,
                            h_reason,
                            true,
                        );
                    }
                    _ => {
                        violations.push(LawViolation {
                            law: Csd5Law::L5,
                            tag: root_tag,
                            message:
                                "L5: A disabled Action must name and draw the reason it is disabled"
                                    .to_string(),
                        });
                    }
                }
            }
            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer: if *in_menu { layer + 2 } else { layer },
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += 38.0;
        }

        Csd5Node::ReceiptSheet {
            surface,
            instance,
            key,
            is_open,
            facts,
            holders,
            notes,
            acts,
        } => {
            let open_tag = derive_csd5_tag(
                Csd5Primitive::ReceiptSheet,
                None,
                surface,
                instance,
                Some("open"),
                Some(key),
            )
            .expect("valid ReceiptSheet__open.key tag");
            let h_open = handles.intern(
                open_tag,
                Some(Csd5Primitive::ReceiptSheet),
                Some("open".into()),
                Some("☰".into()),
            );
            let mut open_marks = Vec::new();
            emit_box_and_run(
                &mut open_marks,
                text,
                x,
                *y,
                44.0,
                28.0,
                4.0,
                ACCENT_BLUE,
                TEXT_PAPER,
                BUTTON,
                "☰",
                h_open,
                true,
            );
            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks: open_marks,
            });
            *y += 32.0;

            if *is_open {
                let modal_l = 10u8;
                *modal_layer = Some(modal_l);
                let root_tag = derive_csd5_tag(
                    Csd5Primitive::ReceiptSheet,
                    None,
                    surface,
                    instance,
                    None,
                    Some(key),
                )
                .expect("valid ReceiptSheet root tag");
                let h_root =
                    handles.intern(root_tag, Some(Csd5Primitive::ReceiptSheet), None, None);
                let mut marks = Vec::new();
                let sheet_y = *y;
                let sheet_h = 230.0f32;
                let mut bg = Instance::new(
                    Kind::Quad,
                    [x + 0.5 * w, sheet_y + 0.5 * sheet_h, 0.0],
                    [w, sheet_h, 0.0],
                )
                .colour(SLAB_BG)
                .tagged(h_root)
                .pinned();
                bg.aux[0] = 8.0;
                marks.push(bg);

                let fact_slots = ["fact_1", "fact_2", "fact_3", "fact_4", "fact_5"];
                for (i, slot_name) in fact_slots.iter().enumerate() {
                    let ftag = derive_csd5_tag(
                        Csd5Primitive::ReceiptSheet,
                        None,
                        surface,
                        instance,
                        Some(slot_name),
                        Some(key),
                    )
                    .expect("valid ReceiptSheet__fact_N tag");
                    let hf = handles.intern(
                        ftag,
                        Some(Csd5Primitive::ReceiptSheet),
                        Some((*slot_name).into()),
                        Some(facts[i].clone()),
                    );
                    emit_box_and_run(
                        &mut marks,
                        text,
                        x + 12.0,
                        sheet_y + 10.0 + (i as f32) * 24.0,
                        w - 24.0,
                        20.0,
                        2.0,
                        SLAB_BORDER,
                        TEXT_PAPER,
                        BODY,
                        &facts[i],
                        hf,
                        true,
                    );
                }

                if !holders.is_empty() {
                    let htag = derive_csd5_tag(
                        Csd5Primitive::ReceiptSheet,
                        None,
                        surface,
                        instance,
                        Some("holders"),
                        Some(key),
                    )
                    .expect("valid ReceiptSheet__holders tag");
                    let joined = holders.join(", ");
                    let hh = handles.intern(
                        htag,
                        Some(Csd5Primitive::ReceiptSheet),
                        Some("holders".into()),
                        Some(joined.clone()),
                    );
                    emit_box_and_run(
                        &mut marks,
                        text,
                        x + 12.0,
                        sheet_y + 135.0,
                        w - 24.0,
                        20.0,
                        2.0,
                        SLAB_BG,
                        TEXT_MUTED,
                        SMALL,
                        &joined,
                        hh,
                        true,
                    );
                }

                if !notes.is_empty() {
                    let ntag = derive_csd5_tag(
                        Csd5Primitive::ReceiptSheet,
                        None,
                        surface,
                        instance,
                        Some("notes"),
                        Some(key),
                    )
                    .expect("valid ReceiptSheet__notes tag");
                    let joined = notes.join(" · ");
                    let hn = handles.intern(
                        ntag,
                        Some(Csd5Primitive::ReceiptSheet),
                        Some("notes".into()),
                        Some(joined.clone()),
                    );
                    emit_box_and_run(
                        &mut marks,
                        text,
                        x + 12.0,
                        sheet_y + 160.0,
                        w - 24.0,
                        20.0,
                        2.0,
                        SLAB_BG,
                        TEXT_MUTED,
                        SMALL,
                        &joined,
                        hn,
                        true,
                    );
                }

                if !acts.is_empty() {
                    let atag = derive_csd5_tag(
                        Csd5Primitive::ReceiptSheet,
                        None,
                        surface,
                        instance,
                        Some("acts"),
                        Some(key),
                    )
                    .expect("valid ReceiptSheet__acts tag");
                    let joined = acts.join(" | ");
                    let ha = handles.intern(
                        atag,
                        Some(Csd5Primitive::ReceiptSheet),
                        Some("acts".into()),
                        Some(joined.clone()),
                    );
                    emit_box_and_run(
                        &mut marks,
                        text,
                        x + 12.0,
                        sheet_y + 188.0,
                        w - 24.0,
                        28.0,
                        4.0,
                        ACCENT_GREEN,
                        TEXT_PAPER,
                        BUTTON,
                        &joined,
                        ha,
                        true,
                    );
                }

                let stid = *next_tile_id;
                *next_tile_id += 1;
                tiles.push(Tile {
                    id: stid,
                    origin: [0.0; 3],
                    version,
                    layer: modal_l,
                    screen: true,
                    params: PARAMS_NEUTRAL,
                    marks,
                });
                *y += sheet_h + 8.0;
            }
        }

        Csd5Node::StateBlock {
            surface,
            instance,
            outcome,
            state,
            detail_text,
            freshness,
        } => {
            let root_tag =
                derive_csd5_tag(Csd5Primitive::StateBlock, None, surface, instance, None, None)
                    .expect("valid StateBlock root tag");
            // L3: A StateBlock's state belongs to the outcome of the read it is bound to.
            if state.outcome() != *outcome {
                violations.push(LawViolation {
                    law: Csd5Law::L3,
                    tag: root_tag.clone(),
                    message: format!(
                        "L3 violation: StateBlock state '{}' requires read outcome '{}', but outcome is '{}'",
                        state.as_str(),
                        state.outcome().as_str(),
                        outcome.as_str()
                    ),
                });
            }
            let state_slot = state.as_str();
            let state_tag = derive_csd5_tag(
                Csd5Primitive::StateBlock,
                None,
                surface,
                instance,
                Some(state_slot),
                None,
            )
            .expect("valid StateBlock__<state> tag");

            // Root tag value is the state's exact name (ui_primitives.json note).
            let _h_root = handles.intern(
                root_tag,
                Some(Csd5Primitive::StateBlock),
                None,
                Some(state_slot.to_string()),
            );
            let form_text = detail_text
                .clone()
                .unwrap_or_else(|| state.default_form_label().to_string());
            let h_state = handles.intern(
                state_tag,
                Some(Csd5Primitive::StateBlock),
                Some(state_slot.to_string()),
                Some(form_text.clone()),
            );

            // L2: Exactly ONE state's form is drawn.
            let mut marks = Vec::new();
            emit_glyph_and_run(
                &mut marks,
                text,
                x,
                *y,
                26.0,
                if *state == Csd5State::Error || *state == Csd5State::Refused {
                    ACCENT_RED
                } else {
                    TEXT_PAPER
                },
                STATUS,
                &form_text,
                h_state,
                false,
            );
            if *state == Csd5State::LoadingKnown {
                // INTERFACE.md §9: loading_known adds a quad bar
                marks.push(
                    Instance::new(Kind::Quad, [x + 120.0, *y + 24.0, 0.0], [200.0, 4.0, 0.0])
                        .colour(ACCENT_GREEN)
                        .tagged(h_state),
                );
            }

            if let Some(fr) = freshness {
                let fr_tag = derive_csd5_tag(
                    Csd5Primitive::StateBlock,
                    None,
                    surface,
                    instance,
                    Some("freshness"),
                    None,
                )
                .expect("valid StateBlock__freshness tag");
                let h_fr = handles.intern(
                    fr_tag,
                    Some(Csd5Primitive::StateBlock),
                    Some("freshness".into()),
                    Some(fr.clone()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + w - 150.0,
                    *y + 2.0,
                    140.0,
                    22.0,
                    2.0,
                    SLAB_BG,
                    TEXT_MUTED,
                    SMALL,
                    fr,
                    h_fr,
                    false,
                );
            }

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += 30.0;
        }

        Csd5Node::Metric {
            surface,
            instance,
            outcome,
            value,
            unit,
            source,
        } => {
            let root_tag =
                derive_csd5_tag(Csd5Primitive::Metric, None, surface, instance, None, None)
                    .expect("valid Metric root tag");
            let unit_tag = derive_csd5_tag(
                Csd5Primitive::Metric,
                None,
                surface,
                instance,
                Some("unit"),
                None,
            )
            .expect("valid Metric__unit tag");
            let source_tag = derive_csd5_tag(
                Csd5Primitive::Metric,
                None,
                surface,
                instance,
                Some("source"),
                None,
            )
            .expect("valid Metric__source tag");

            // L4: A Metric shows a value only when its read was answered (`Ok`) and the value was sent.
            // Not sent or not read is drawn as an em dash ("—"), never as zero.
            let display_val = match (outcome, value) {
                (ReadOutcome::Ok, Some(v)) if !v.trim().is_empty() => v.clone(),
                _ => "—".to_string(),
            };

            let h_root = handles.intern(
                root_tag,
                Some(Csd5Primitive::Metric),
                None,
                Some(display_val.clone()),
            );
            let h_unit = handles.intern(
                unit_tag,
                Some(Csd5Primitive::Metric),
                Some("unit".into()),
                Some(unit.clone()),
            );
            let h_source = handles.intern(
                source_tag,
                Some(Csd5Primitive::Metric),
                Some("source".into()),
                Some(source.clone()),
            );

            let mut marks = Vec::new();
            emit_box_and_run(
                &mut marks,
                text,
                x,
                *y,
                120.0,
                24.0,
                3.0,
                SLAB_BG,
                TEXT_PAPER,
                STATUS,
                &display_val,
                h_root,
                false,
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 126.0,
                *y,
                80.0,
                24.0,
                3.0,
                SLAB_BG,
                TEXT_MUTED,
                SMALL,
                unit,
                h_unit,
                false,
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 212.0,
                *y,
                160.0,
                24.0,
                3.0,
                SLAB_BG,
                TEXT_MUTED,
                SMALL,
                source,
                h_source,
                false,
            );

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += 28.0;
        }

        Csd5Node::ConfirmSheet {
            surface,
            instance,
            fact_1,
            fact_2,
            fact_3,
            note,
            confirm_label,
            cancel_label,
        } => {
            // ConfirmSheet is always a modal layer tile (`INTERFACE.md` §8.4, §9): takes every pick while open.
            let modal_l = 12u8;
            *modal_layer = Some(modal_l);
            let root_tag =
                derive_csd5_tag(Csd5Primitive::ConfirmSheet, None, surface, instance, None, None)
                    .expect("valid ConfirmSheet tag");
            let f1_tag = derive_csd5_tag(
                Csd5Primitive::ConfirmSheet,
                None,
                surface,
                instance,
                Some("fact_1"),
                None,
            )
            .unwrap();
            let f2_tag = derive_csd5_tag(
                Csd5Primitive::ConfirmSheet,
                None,
                surface,
                instance,
                Some("fact_2"),
                None,
            )
            .unwrap();
            let f3_tag = derive_csd5_tag(
                Csd5Primitive::ConfirmSheet,
                None,
                surface,
                instance,
                Some("fact_3"),
                None,
            )
            .unwrap();
            let confirm_tag = derive_csd5_tag(
                Csd5Primitive::ConfirmSheet,
                None,
                surface,
                instance,
                Some("confirm"),
                None,
            )
            .unwrap();
            let cancel_tag = derive_csd5_tag(
                Csd5Primitive::ConfirmSheet,
                None,
                surface,
                instance,
                Some("cancel"),
                None,
            )
            .unwrap();

            let h_root = handles.intern(root_tag, Some(Csd5Primitive::ConfirmSheet), None, None);
            let h_f1 = handles.intern(
                f1_tag,
                Some(Csd5Primitive::ConfirmSheet),
                Some("fact_1".into()),
                Some(fact_1.clone()),
            );
            let h_f2 = handles.intern(
                f2_tag,
                Some(Csd5Primitive::ConfirmSheet),
                Some("fact_2".into()),
                Some(fact_2.clone()),
            );
            let h_f3 = handles.intern(
                f3_tag,
                Some(Csd5Primitive::ConfirmSheet),
                Some("fact_3".into()),
                Some(fact_3.clone()),
            );
            let h_confirm = handles.intern(
                confirm_tag,
                Some(Csd5Primitive::ConfirmSheet),
                Some("confirm".into()),
                Some(confirm_label.clone()),
            );
            let h_cancel = handles.intern(
                cancel_tag,
                Some(Csd5Primitive::ConfirmSheet),
                Some("cancel".into()),
                Some(cancel_label.clone()),
            );

            let mut marks = Vec::new();
            let sh_y = *y;
            let sh_h = 160.0f32;
            let mut bg = Instance::new(
                Kind::Quad,
                [x + 0.5 * w, sh_y + 0.5 * sh_h, 0.0],
                [w, sh_h, 0.0],
            )
            .colour(SLAB_BG)
            .tagged(h_root)
            .pinned();
            bg.aux[0] = 8.0;
            marks.push(bg);

            for (i, (hf, ftxt)) in [(h_f1, fact_1), (h_f2, fact_2), (h_f3, fact_3)]
                .iter()
                .enumerate()
            {
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + 12.0,
                    sh_y + 10.0 + (i as f32) * 26.0,
                    w - 24.0,
                    22.0,
                    3.0,
                    SLAB_BORDER,
                    TEXT_PAPER,
                    BODY,
                    ftxt,
                    *hf,
                    true,
                );
            }

            if let Some(nt) = note {
                let note_tag = derive_csd5_tag(
                    Csd5Primitive::ConfirmSheet,
                    None,
                    surface,
                    instance,
                    Some("note"),
                    None,
                )
                .unwrap();
                let h_note = handles.intern(
                    note_tag,
                    Some(Csd5Primitive::ConfirmSheet),
                    Some("note".into()),
                    Some(nt.clone()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + 12.0,
                    sh_y + 90.0,
                    w - 24.0,
                    20.0,
                    2.0,
                    SLAB_BG,
                    ACCENT_AMBER,
                    SMALL,
                    nt,
                    h_note,
                    true,
                );
            }

            emit_box_and_run(
                &mut marks,
                text,
                x + 12.0,
                sh_y + 118.0,
                140.0,
                34.0,
                4.0,
                ACCENT_GREEN,
                TEXT_PAPER,
                BUTTON,
                confirm_label,
                h_confirm,
                true,
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 164.0,
                sh_y + 118.0,
                120.0,
                34.0,
                4.0,
                SLAB_BORDER,
                TEXT_PAPER,
                BUTTON,
                cancel_label,
                h_cancel,
                true,
            );

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer: modal_l,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += sh_h + 8.0;
        }

        Csd5Node::Choice {
            surface,
            instance,
            question,
            options,
        } => {
            let mut marks = Vec::new();
            for opt in options {
                let opt_tag = derive_csd5_tag(
                    Csd5Primitive::Choice,
                    None,
                    surface,
                    instance,
                    None,
                    Some(&opt.key),
                )
                .expect("valid Choice.key tag");
                let cost_tag = derive_csd5_tag(
                    Csd5Primitive::Choice,
                    None,
                    surface,
                    instance,
                    Some("cost"),
                    Some(&opt.key),
                )
                .expect("valid Choice__cost.key tag");

                let h_opt = handles.intern(
                    opt_tag,
                    Some(Csd5Primitive::Choice),
                    None,
                    Some(opt.label.clone()),
                );
                let h_cost = handles.intern(
                    cost_tag,
                    Some(Csd5Primitive::Choice),
                    Some("cost".into()),
                    Some(opt.cost.clone()),
                );

                emit_box_and_run(
                    &mut marks,
                    text,
                    x,
                    *y,
                    0.48 * w,
                    30.0,
                    4.0,
                    if opt.selected {
                        ACCENT_GREEN
                    } else {
                        SLAB_BORDER
                    },
                    TEXT_PAPER,
                    BUTTON,
                    &format!("{question}: {}", opt.label),
                    h_opt,
                    true, // L12: Choice options are pinned
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + 0.50 * w,
                    *y,
                    0.30 * w,
                    30.0,
                    3.0,
                    SLAB_BG,
                    TEXT_MUTED,
                    SMALL,
                    &opt.cost,
                    h_cost,
                    true,
                );
                if let Some(tally) = &opt.tally {
                    let tally_tag = derive_csd5_tag(
                        Csd5Primitive::Choice,
                        None,
                        surface,
                        instance,
                        Some("tally"),
                        Some(&opt.key),
                    )
                    .expect("valid Choice__tally.key tag");
                    let h_tally = handles.intern(
                        tally_tag,
                        Some(Csd5Primitive::Choice),
                        Some("tally".into()),
                        Some(tally.clone()),
                    );
                    emit_box_and_run(
                        &mut marks,
                        text,
                        x + 0.82 * w,
                        *y,
                        0.18 * w,
                        30.0,
                        3.0,
                        SLAB_BG,
                        TEXT_PAPER,
                        SMALL,
                        tally,
                        h_tally,
                        true,
                    );
                }
                *y += 34.0;
            }
            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
        }

        Csd5Node::DestinationPicker {
            surface,
            instance,
            destinations,
        } => {
            let mut marks = Vec::new();
            for (dkey, dlabel, dgroup) in destinations {
                let dtag = derive_csd5_tag(
                    Csd5Primitive::DestinationPicker,
                    None,
                    surface,
                    instance,
                    None,
                    Some(dkey),
                )
                .expect("valid DestinationPicker.key tag");
                let hd = handles.intern(
                    dtag,
                    Some(Csd5Primitive::DestinationPicker),
                    None,
                    Some(dlabel.clone()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x,
                    *y,
                    0.55 * w,
                    26.0,
                    4.0,
                    SLAB_BORDER,
                    TEXT_PAPER,
                    BUTTON,
                    dlabel,
                    hd,
                    true,
                );
                if let Some(grp) = dgroup {
                    let gtag = derive_csd5_tag(
                        Csd5Primitive::DestinationPicker,
                        None,
                        surface,
                        instance,
                        Some("group"),
                        Some(dkey),
                    )
                    .expect("valid DestinationPicker__group.key tag");
                    let hg = handles.intern(
                        gtag,
                        Some(Csd5Primitive::DestinationPicker),
                        Some("group".into()),
                        Some(grp.clone()),
                    );
                    emit_box_and_run(
                        &mut marks,
                        text,
                        x + 0.58 * w,
                        *y,
                        0.40 * w,
                        26.0,
                        3.0,
                        SLAB_BG,
                        TEXT_MUTED,
                        SMALL,
                        grp,
                        hg,
                        true,
                    );
                }
                *y += 30.0;
            }
            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
        }

        Csd5Node::Field {
            surface,
            instance,
            value,
            byte_ceiling,
            error,
        } => {
            let root_tag =
                derive_csd5_tag(Csd5Primitive::Field, None, surface, instance, None, None)
                    .expect("valid Field tag");
            let ceil_tag = derive_csd5_tag(
                Csd5Primitive::Field,
                None,
                surface,
                instance,
                Some("ceiling"),
                None,
            )
            .expect("valid Field__ceiling tag");

            let ceil_str = format!("{}/{} B", value.len(), byte_ceiling);
            let h_root = handles.intern(
                root_tag,
                Some(Csd5Primitive::Field),
                None,
                Some(value.clone()),
            );
            let h_ceil = handles.intern(
                ceil_tag,
                Some(Csd5Primitive::Field),
                Some("ceiling".into()),
                Some(ceil_str.clone()),
            );

            let mut marks = Vec::new();
            emit_box_and_run(
                &mut marks,
                text,
                x,
                *y,
                0.70 * w,
                30.0,
                4.0,
                SLAB_BORDER,
                TEXT_PAPER,
                BODY,
                value,
                h_root,
                true, // L12: Field is pinned
            );
            // Caret line (`INTERFACE.md` §9)
            let mut caret = Instance::new(
                Kind::Line,
                [x + 12.0, *y + 6.0, 0.0],
                [1.5, 0.0, 0.0],
            )
            .colour(TEXT_PAPER)
            .tagged(h_root)
            .pinned();
            caret.aux = [x + 12.0, *y + 24.0, 0.0];
            marks.push(caret);

            emit_box_and_run(
                &mut marks,
                text,
                x + 0.72 * w,
                *y,
                0.28 * w,
                30.0,
                3.0,
                SLAB_BG,
                TEXT_MUTED,
                SMALL,
                &ceil_str,
                h_ceil,
                true,
            );
            *y += 34.0;

            if let Some(err) = error {
                let err_tag = derive_csd5_tag(
                    Csd5Primitive::Field,
                    None,
                    surface,
                    instance,
                    Some("error"),
                    None,
                )
                .expect("valid Field__error tag");
                let h_err = handles.intern(
                    err_tag,
                    Some(Csd5Primitive::Field),
                    Some("error".into()),
                    Some(err.clone()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x,
                    *y,
                    w,
                    22.0,
                    3.0,
                    ACCENT_RED,
                    TEXT_PAPER,
                    SMALL,
                    err,
                    h_err,
                    true,
                );
                *y += 26.0;
            }

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
        }

        Csd5Node::StepFlow {
            surface,
            instance,
            steps,
            active_key,
            has_back,
            has_next,
        } => {
            let mut marks = Vec::new();
            let step_w = (w - 180.0) / (steps.len().max(1) as f32);
            for (i, (skey, stitle)) in steps.iter().enumerate() {
                let stag = derive_csd5_tag(
                    Csd5Primitive::StepFlow,
                    None,
                    surface,
                    instance,
                    None,
                    Some(skey),
                )
                .expect("valid StepFlow.key tag");
                let hs = handles.intern(
                    stag,
                    Some(Csd5Primitive::StepFlow),
                    None,
                    Some(stitle.clone()),
                );
                let sx = x + (i as f32) * step_w;
                marks.push(
                    Instance::new(Kind::Disc, [sx + 10.0, *y + 14.0, 0.0], [6.0, 0.0, 0.0])
                        .colour(if skey == active_key {
                            ACCENT_GREEN
                        } else {
                            SLAB_BORDER
                        })
                        .tagged(hs),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    sx + 20.0,
                    *y,
                    step_w - 24.0,
                    28.0,
                    3.0,
                    SLAB_BG,
                    TEXT_PAPER,
                    SMALL,
                    stitle,
                    hs,
                    false,
                );
            }
            if *has_back {
                let btag = derive_csd5_tag(
                    Csd5Primitive::StepFlow,
                    None,
                    surface,
                    instance,
                    Some("back"),
                    None,
                )
                .expect("valid StepFlow__back tag");
                let hb = handles.intern(
                    btag,
                    Some(Csd5Primitive::StepFlow),
                    Some("back".into()),
                    Some("Back".into()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + w - 168.0,
                    *y,
                    78.0,
                    28.0,
                    4.0,
                    SLAB_BORDER,
                    TEXT_PAPER,
                    BUTTON,
                    "Back",
                    hb,
                    true,
                );
            }
            if *has_next {
                let ntag = derive_csd5_tag(
                    Csd5Primitive::StepFlow,
                    None,
                    surface,
                    instance,
                    Some("next"),
                    None,
                )
                .expect("valid StepFlow__next tag");
                let hn = handles.intern(
                    ntag,
                    Some(Csd5Primitive::StepFlow),
                    Some("next".into()),
                    Some("Next".into()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + w - 82.0,
                    *y,
                    82.0,
                    28.0,
                    4.0,
                    ACCENT_GREEN,
                    TEXT_PAPER,
                    BUTTON,
                    "Next",
                    hn,
                    true,
                );
            }
            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += 32.0;
        }

        Csd5Node::Ceremony {
            surface,
            instance,
            key,
            act_price,
            undo_price,
            window,
            threshold,
            signers,
        } => {
            let root_tag = derive_csd5_tag(
                Csd5Primitive::Ceremony,
                None,
                surface,
                instance,
                None,
                Some(key),
            )
            .expect("valid Ceremony.key tag");
            let ap_tag = derive_csd5_tag(
                Csd5Primitive::Ceremony,
                None,
                surface,
                instance,
                Some("act_price"),
                Some(key),
            )
            .unwrap();
            let up_tag = derive_csd5_tag(
                Csd5Primitive::Ceremony,
                None,
                surface,
                instance,
                Some("undo_price"),
                Some(key),
            )
            .unwrap();
            let win_tag = derive_csd5_tag(
                Csd5Primitive::Ceremony,
                None,
                surface,
                instance,
                Some("window"),
                Some(key),
            )
            .unwrap();
            let thr_tag = derive_csd5_tag(
                Csd5Primitive::Ceremony,
                None,
                surface,
                instance,
                Some("threshold"),
                Some(key),
            )
            .unwrap();

            let h_root = handles.intern(root_tag, Some(Csd5Primitive::Ceremony), None, None);
            let h_ap = handles.intern(
                ap_tag,
                Some(Csd5Primitive::Ceremony),
                Some("act_price".into()),
                Some(act_price.clone()),
            );
            let h_up = handles.intern(
                up_tag,
                Some(Csd5Primitive::Ceremony),
                Some("undo_price".into()),
                Some(undo_price.clone()),
            );
            let h_win = handles.intern(
                win_tag,
                Some(Csd5Primitive::Ceremony),
                Some("window".into()),
                Some(window.clone()),
            );
            let h_thr = handles.intern(
                thr_tag,
                Some(Csd5Primitive::Ceremony),
                Some("threshold".into()),
                Some(threshold.clone()),
            );

            let mut marks = Vec::new();
            marks.push(
                Instance::new(Kind::Quad, [x + 0.5 * w, *y + 28.0, 0.0], [w, 56.0, 0.0])
                    .colour(SLAB_BG)
                    .tagged(h_root),
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 6.0,
                *y + 4.0,
                0.23 * w,
                22.0,
                2.0,
                SLAB_BORDER,
                TEXT_PAPER,
                SMALL,
                act_price,
                h_ap,
                false,
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 0.25 * w,
                *y + 4.0,
                0.23 * w,
                22.0,
                2.0,
                SLAB_BORDER,
                TEXT_PAPER,
                SMALL,
                undo_price,
                h_up,
                false,
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 0.50 * w,
                *y + 4.0,
                0.23 * w,
                22.0,
                2.0,
                SLAB_BORDER,
                TEXT_PAPER,
                SMALL,
                window,
                h_win,
                false,
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 0.75 * w,
                *y + 4.0,
                0.23 * w,
                22.0,
                2.0,
                SLAB_BORDER,
                TEXT_PAPER,
                SMALL,
                threshold,
                h_thr,
                false,
            );

            for (i, (who, st)) in signers.iter().enumerate() {
                let s_tag = derive_csd5_tag(
                    Csd5Primitive::Ceremony,
                    None,
                    surface,
                    instance,
                    Some("signer"),
                    Some(who),
                )
                .expect("valid Ceremony__signer.who tag");
                let hs = handles.intern(
                    s_tag,
                    Some(Csd5Primitive::Ceremony),
                    Some("signer".into()),
                    Some(format!("{who}: {st}")),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + 6.0 + (i as f32) * 140.0,
                    *y + 30.0,
                    132.0,
                    22.0,
                    3.0,
                    ACCENT_BLUE,
                    TEXT_PAPER,
                    SMALL,
                    &format!("{who}: {st}"),
                    hs,
                    false,
                );
            }

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += 60.0;
        }

        Csd5Node::LineageChain {
            surface,
            instance,
            key,
            links,
            cascade,
        } => {
            let root_tag = derive_csd5_tag(
                Csd5Primitive::LineageChain,
                None,
                surface,
                instance,
                None,
                Some(key),
            )
            .expect("valid LineageChain.key tag");
            let _h_root = handles.intern(root_tag, Some(Csd5Primitive::LineageChain), None, None);

            let mut marks = Vec::new();
            for (i, (lkey, llabel)) in links.iter().enumerate() {
                let ltag = derive_csd5_tag(
                    Csd5Primitive::LineageChain,
                    None,
                    surface,
                    instance,
                    Some("link"),
                    Some(lkey),
                )
                .expect("valid LineageChain__link.key tag");
                let hl = handles.intern(
                    ltag,
                    Some(Csd5Primitive::LineageChain),
                    Some("link".into()),
                    Some(llabel.clone()),
                );
                let ly = *y + (i as f32) * 26.0;
                marks.push(
                    Instance::new(Kind::Disc, [x + 10.0, ly + 11.0, 0.0], [5.0, 0.0, 0.0])
                        .colour(ACCENT_GREEN)
                        .tagged(hl),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + 22.0,
                    ly,
                    w - 28.0,
                    22.0,
                    3.0,
                    SLAB_BORDER,
                    TEXT_PAPER,
                    SMALL,
                    llabel,
                    hl,
                    false,
                );
            }
            *y += (links.len() as f32) * 26.0;

            if let Some(casc) = cascade {
                let ctag = derive_csd5_tag(
                    Csd5Primitive::LineageChain,
                    None,
                    surface,
                    instance,
                    Some("cascade"),
                    Some(key),
                )
                .expect("valid LineageChain__cascade.key tag");
                let hc = handles.intern(
                    ctag,
                    Some(Csd5Primitive::LineageChain),
                    Some("cascade".into()),
                    Some(casc.clone()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x,
                    *y,
                    w,
                    22.0,
                    3.0,
                    SLAB_BG,
                    ACCENT_AMBER,
                    SMALL,
                    casc,
                    hc,
                    false,
                );
                *y += 26.0;
            }

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
        }

        Csd5Node::CodeCompare {
            surface,
            instance,
            words,
            digits,
            verdict,
        } => {
            let root_tag =
                derive_csd5_tag(Csd5Primitive::CodeCompare, None, surface, instance, None, None)
                    .expect("valid CodeCompare tag");
            let w_tag = derive_csd5_tag(
                Csd5Primitive::CodeCompare,
                None,
                surface,
                instance,
                Some("words"),
                None,
            )
            .unwrap();
            let d_tag = derive_csd5_tag(
                Csd5Primitive::CodeCompare,
                None,
                surface,
                instance,
                Some("digits"),
                None,
            )
            .unwrap();
            let v_tag = derive_csd5_tag(
                Csd5Primitive::CodeCompare,
                None,
                surface,
                instance,
                Some("verdict"),
                None,
            )
            .unwrap();

            let _hr = handles.intern(root_tag, Some(Csd5Primitive::CodeCompare), None, None);
            let hw = handles.intern(
                w_tag,
                Some(Csd5Primitive::CodeCompare),
                Some("words".into()),
                Some(words.clone()),
            );
            let hd = handles.intern(
                d_tag,
                Some(Csd5Primitive::CodeCompare),
                Some("digits".into()),
                Some(digits.clone()),
            );
            let hv = handles.intern(
                v_tag,
                Some(Csd5Primitive::CodeCompare),
                Some("verdict".into()),
                Some(verdict.clone()),
            );

            let mut marks = Vec::new();
            emit_box_and_run(
                &mut marks,
                text,
                x,
                *y,
                0.45 * w,
                26.0,
                3.0,
                SLAB_BORDER,
                TEXT_PAPER,
                STATUS,
                words,
                hw,
                false,
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 0.47 * w,
                *y,
                0.25 * w,
                26.0,
                3.0,
                SLAB_BORDER,
                TEXT_PAPER,
                STATUS,
                digits,
                hd,
                false,
            );
            emit_glyph_and_run(
                &mut marks,
                text,
                x + 0.74 * w,
                *y,
                26.0,
                ACCENT_GREEN,
                SMALL,
                verdict,
                hv,
                false,
            );

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += 30.0;
        }

        Csd5Node::QrCode {
            surface,
            instance,
            is_scanner,
            payload,
            has_paste,
        } => {
            let pfx = if *is_scanner { "scan" } else { "qr" };
            let root_tag =
                derive_csd5_tag(Csd5Primitive::QrCode, Some(pfx), surface, instance, None, None)
                    .expect("valid QrCode tag");
            let hr = handles.intern(
                root_tag,
                Some(Csd5Primitive::QrCode),
                None,
                Some(payload.clone()),
            );

            let mut marks = Vec::new();
            emit_box_and_run(
                &mut marks,
                text,
                x,
                *y,
                0.65 * w,
                28.0,
                4.0,
                SLAB_BORDER,
                TEXT_PAPER,
                BODY,
                payload,
                hr,
                false,
            );
            if *has_paste {
                let ptag = derive_csd5_tag(
                    Csd5Primitive::QrCode,
                    Some(pfx),
                    surface,
                    instance,
                    Some("paste"),
                    None,
                )
                .expect("valid QrCode__paste tag");
                let hp = handles.intern(
                    ptag,
                    Some(Csd5Primitive::QrCode),
                    Some("paste".into()),
                    Some("Paste code".into()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + 0.68 * w,
                    *y,
                    0.30 * w,
                    28.0,
                    4.0,
                    ACCENT_BLUE,
                    TEXT_PAPER,
                    BUTTON,
                    "Paste code",
                    hp,
                    true,
                );
            }

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += 32.0;
        }

        Csd5Node::Transcript {
            surface,
            instance,
            is_note,
            entries,
            has_fetch,
        } => {
            let pfx = if *is_note { "note" } else { "msg" };
            let mut marks = Vec::new();
            for (ekey, etxt, has_rcpt) in entries {
                let etag = derive_csd5_tag(
                    Csd5Primitive::Transcript,
                    Some(pfx),
                    surface,
                    instance,
                    None,
                    Some(ekey),
                )
                .expect("valid Transcript.key tag");
                let he = handles.intern(
                    etag,
                    Some(Csd5Primitive::Transcript),
                    None,
                    Some(etxt.clone()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x,
                    *y,
                    0.78 * w,
                    24.0,
                    3.0,
                    SLAB_BG,
                    TEXT_PAPER,
                    BODY,
                    etxt,
                    he,
                    false,
                );
                if *has_rcpt {
                    let rtag = derive_csd5_tag(
                        Csd5Primitive::Transcript,
                        Some(pfx),
                        surface,
                        instance,
                        Some("receipt"),
                        Some(ekey),
                    )
                    .expect("valid Transcript__receipt.key tag");
                    let hr = handles.intern(
                        rtag,
                        Some(Csd5Primitive::Transcript),
                        Some("receipt".into()),
                        Some("signed".into()),
                    );
                    emit_glyph_and_run(
                        &mut marks,
                        text,
                        x + 0.80 * w,
                        *y,
                        24.0,
                        ACCENT_GREEN,
                        SMALL,
                        "signed",
                        hr,
                        false,
                    );
                }
                *y += 26.0;
            }
            if *has_fetch {
                let ftag = derive_csd5_tag(
                    Csd5Primitive::Transcript,
                    Some(pfx),
                    surface,
                    instance,
                    Some("fetch"),
                    None,
                )
                .expect("valid Transcript__fetch tag");
                let hf = handles.intern(
                    ftag,
                    Some(Csd5Primitive::Transcript),
                    Some("fetch".into()),
                    Some("Fetch earlier".into()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x,
                    *y,
                    140.0,
                    24.0,
                    4.0,
                    SLAB_BORDER,
                    TEXT_PAPER,
                    BUTTON,
                    "Fetch earlier",
                    hf,
                    true,
                );
                *y += 28.0;
            }
            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
        }

        Csd5Node::ConscienceStep {
            surface,
            instance,
            key,
            summary,
            cost,
            faculties,
        } => {
            let root_tag = derive_csd5_tag(
                Csd5Primitive::ConscienceStep,
                None,
                surface,
                instance,
                None,
                Some(key),
            )
            .expect("valid ConscienceStep.key tag");
            let cost_tag = derive_csd5_tag(
                Csd5Primitive::ConscienceStep,
                None,
                surface,
                instance,
                Some("cost"),
                Some(key),
            )
            .expect("valid ConscienceStep__cost.key tag");

            let hr = handles.intern(
                root_tag,
                Some(Csd5Primitive::ConscienceStep),
                None,
                Some(summary.clone()),
            );
            let hc = handles.intern(
                cost_tag,
                Some(Csd5Primitive::ConscienceStep),
                Some("cost".into()),
                Some(cost.clone()),
            );

            let mut marks = Vec::new();
            emit_glyph_and_run(
                &mut marks,
                text,
                x,
                *y,
                24.0,
                ACCENT_GREEN,
                BODY,
                summary,
                hr,
                false,
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 0.65 * w,
                *y,
                0.35 * w,
                24.0,
                3.0,
                SLAB_BG,
                TEXT_MUTED,
                SMALL,
                cost,
                hc,
                false,
            );
            *y += 26.0;

            for (i, (fkey, fverdict)) in faculties.iter().enumerate() {
                let ftag = derive_csd5_tag(
                    Csd5Primitive::ConscienceStep,
                    None,
                    surface,
                    instance,
                    Some("faculty"),
                    Some(fkey),
                )
                .expect("valid ConscienceStep__faculty.key tag");
                let hf = handles.intern(
                    ftag,
                    Some(Csd5Primitive::ConscienceStep),
                    Some("faculty".into()),
                    Some(format!("{fkey}: {fverdict}")),
                );
                emit_glyph_and_run(
                    &mut marks,
                    text,
                    x + (i as f32) * 160.0,
                    *y,
                    22.0,
                    TEXT_PAPER,
                    SMALL,
                    &format!("{fkey}: {fverdict}"),
                    hf,
                    false,
                );
            }
            if !faculties.is_empty() {
                *y += 24.0;
            }

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
        }

        Csd5Node::PolicyLens {
            surface,
            instance,
            hidden_count,
            helpers,
            depth,
        } => {
            let root_tag =
                derive_csd5_tag(Csd5Primitive::PolicyLens, None, surface, instance, None, None)
                    .expect("valid PolicyLens tag");
            let hc_tag = derive_csd5_tag(
                Csd5Primitive::PolicyLens,
                None,
                surface,
                instance,
                Some("hidden_count"),
                None,
            )
            .unwrap();
            let hlp_tag = derive_csd5_tag(
                Csd5Primitive::PolicyLens,
                None,
                surface,
                instance,
                Some("helpers"),
                None,
            )
            .unwrap();
            let dp_tag = derive_csd5_tag(
                Csd5Primitive::PolicyLens,
                None,
                surface,
                instance,
                Some("depth"),
                None,
            )
            .unwrap();

            let hc_str = format!("{hidden_count} hidden");
            let hlp_str = format!("{} helpers", helpers.len());
            let hr = handles.intern(
                root_tag,
                Some(Csd5Primitive::PolicyLens),
                None,
                Some("Policy Lens".into()),
            );
            let h_hc = handles.intern(
                hc_tag,
                Some(Csd5Primitive::PolicyLens),
                Some("hidden_count".into()),
                Some(hc_str.clone()),
            );
            let h_hlp = handles.intern(
                hlp_tag,
                Some(Csd5Primitive::PolicyLens),
                Some("helpers".into()),
                Some(hlp_str.clone()),
            );
            let h_dp = handles.intern(
                dp_tag,
                Some(Csd5Primitive::PolicyLens),
                Some("depth".into()),
                Some(depth.clone()),
            );

            let mut marks = Vec::new();
            emit_box_and_run(
                &mut marks,
                text,
                x,
                *y,
                0.24 * w,
                24.0,
                3.0,
                SLAB_BORDER,
                TEXT_PAPER,
                SMALL,
                "Policy Lens",
                hr,
                false,
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 0.26 * w,
                *y,
                0.24 * w,
                24.0,
                3.0,
                SLAB_BG,
                TEXT_MUTED,
                SMALL,
                &hc_str,
                h_hc,
                false,
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 0.52 * w,
                *y,
                0.24 * w,
                24.0,
                3.0,
                SLAB_BG,
                TEXT_MUTED,
                SMALL,
                &hlp_str,
                h_hlp,
                false,
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 0.78 * w,
                *y,
                0.22 * w,
                24.0,
                3.0,
                SLAB_BG,
                TEXT_MUTED,
                SMALL,
                depth,
                h_dp,
                false,
            );

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += 28.0;
        }

        Csd5Node::GraphView {
            surface,
            instance,
            world_nodes,
            twin_rows,
        } => {
            let root_tag =
                derive_csd5_tag(Csd5Primitive::GraphView, None, surface, instance, None, None)
                    .expect("valid GraphView tag");
            let twin_tag = derive_csd5_tag(
                Csd5Primitive::GraphView,
                None,
                surface,
                instance,
                Some("twin"),
                None,
            )
            .expect("valid GraphView__twin tag");
            let _h_canvas = handles.intern(root_tag, Some(Csd5Primitive::GraphView), None, None);
            let _h_twin = handles.intern(
                twin_tag,
                Some(Csd5Primitive::GraphView),
                Some("twin".into()),
                None,
            );

            // First emit the twin ItemRows so their `row_<surface>_<instance>.<key>` handles exist,
            // then bind each 3D world mark to the exact same handle (`INTERFACE.md` §0, §8.1, §9)!
            for row in twin_rows {
                compile_one_node(
                    row,
                    text,
                    version,
                    handles,
                    violations,
                    tiles,
                    modal_layer,
                    next_tile_id,
                    x,
                    y,
                    w,
                    layer,
                );
            }

            let mut world_marks = Vec::new();
            // Ground patch decoration (tag 0)
            world_marks.push(
                Instance::new(Kind::Patch, [0.0, 0.0, -1.5], [24.0, 24.0, 0.0])
                    .colour([0x10, 0x16, 0x1e, 255])
                    .detail(1),
            );
            for wn in world_nodes {
                let row_tag = derive_csd5_tag(
                    Csd5Primitive::ItemRow,
                    None,
                    surface,
                    instance,
                    None,
                    Some(&wn.key),
                )
                .expect("valid GraphView twin ItemRow tag");
                let h_mark = handles.intern(
                    row_tag,
                    Some(Csd5Primitive::ItemRow),
                    None,
                    Some(wn.label.clone()),
                );
                world_marks.push(
                    Instance::new(Kind::Sphere, wn.pos, [wn.radius, 0.0, 0.0])
                        .colour(wn.rgba)
                        .tagged(h_mark)
                        .detail(1),
                );
                if let Some(target) = wn.bonded_to_pos {
                    let mut rod = Instance::new(Kind::Rod, wn.pos, [0.15, 0.0, 0.0])
                        .colour(ACCENT_GREEN)
                        .tagged(h_mark);
                    rod.aux = target;
                    world_marks.push(rod);
                }
            }
            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer: 0,
                screen: false, // 3D World tile!
                params: PARAMS_NEUTRAL,
                marks: world_marks,
            });
        }

        Csd5Node::Document {
            surface,
            instance,
            key,
            revision,
            claims,
        } => {
            let root_tag = derive_csd5_tag(
                Csd5Primitive::Document,
                None,
                surface,
                instance,
                None,
                Some(key),
            )
            .expect("valid Document.key tag");
            let rev_tag = derive_csd5_tag(
                Csd5Primitive::Document,
                None,
                surface,
                instance,
                Some("revision"),
                Some(key),
            )
            .expect("valid Document__revision.key tag");

            let _hr = handles.intern(root_tag, Some(Csd5Primitive::Document), None, None);
            let h_rev = handles.intern(
                rev_tag,
                Some(Csd5Primitive::Document),
                Some("revision".into()),
                Some(revision.clone()),
            );

            let mut marks = Vec::new();
            emit_box_and_run(
                &mut marks,
                text,
                x,
                *y,
                160.0,
                22.0,
                3.0,
                SLAB_BORDER,
                TEXT_MUTED,
                SMALL,
                revision,
                h_rev,
                false,
            );
            *y += 26.0;

            for cl in claims {
                let ctag = derive_csd5_tag(
                    Csd5Primitive::Document,
                    None,
                    surface,
                    instance,
                    Some("claim"),
                    Some(&cl.claim_key),
                )
                .expect("valid Document__claim.key tag");
                let hc = handles.intern(
                    ctag,
                    Some(Csd5Primitive::Document),
                    Some("claim".into()),
                    Some(cl.text.clone()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x,
                    *y,
                    0.55 * w,
                    24.0,
                    3.0,
                    SLAB_BG,
                    TEXT_PAPER,
                    BODY,
                    &cl.text,
                    hc,
                    false,
                );

                // INTERFACE.md §9: A recanted claim is a run with a line through it and its reason beside it; never removed.
                if let Some(reason) = &cl.recanted_reason {
                    let rtag = derive_csd5_tag(
                        Csd5Primitive::Document,
                        None,
                        surface,
                        instance,
                        Some("recant"),
                        Some(&cl.claim_key),
                    )
                    .expect("valid Document__recant.key tag");
                    let h_rec = handles.intern(
                        rtag,
                        Some(Csd5Primitive::Document),
                        Some("recant".into()),
                        Some(reason.clone()),
                    );
                    let mut strike = Instance::new(
                        Kind::Line,
                        [x + 4.0, *y + 12.0, 0.0],
                        [2.0, 0.0, 0.0],
                    )
                    .colour(ACCENT_RED)
                    .tagged(h_rec);
                    strike.aux = [x + 0.54 * w, *y + 12.0, 0.0];
                    marks.push(strike);

                    emit_box_and_run(
                        &mut marks,
                        text,
                        x + 0.56 * w,
                        *y,
                        0.24 * w,
                        24.0,
                        3.0,
                        ACCENT_RED,
                        TEXT_PAPER,
                        SMALL,
                        reason,
                        h_rec,
                        false,
                    );
                }

                if let Some(cite) = &cl.citation {
                    let cit_tag = derive_csd5_tag(
                        Csd5Primitive::Document,
                        None,
                        surface,
                        instance,
                        Some("cite"),
                        Some(&cl.claim_key),
                    )
                    .expect("valid Document__cite.key tag");
                    let h_cit = handles.intern(
                        cit_tag,
                        Some(Csd5Primitive::Document),
                        Some("cite".into()),
                        Some(cite.clone()),
                    );
                    emit_glyph_and_run(
                        &mut marks,
                        text,
                        x + 0.82 * w,
                        *y,
                        24.0,
                        ACCENT_BLUE,
                        SMALL,
                        cite,
                        h_cit,
                        false,
                    );
                }
                *y += 28.0;
            }

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
        }

        Csd5Node::Media {
            surface,
            instance,
            key,
            alt,
            ai_disclosure,
            blurred,
            media_class,
        } => {
            let root_tag = derive_csd5_tag(
                Csd5Primitive::Media,
                None,
                surface,
                instance,
                None,
                Some(key),
            )
            .expect("valid Media.key tag");

            // L10: Media is never drawn bare. A Media instance has alt text and an AI-disclosure value or it is not drawn.
            if alt.trim().is_empty() || ai_disclosure.trim().is_empty() {
                violations.push(LawViolation {
                    law: Csd5Law::L10,
                    tag: root_tag,
                    message: "L10 violation: Media lacks alt text or ai_disclosure and was refused (not drawn)".to_string(),
                });
                return;
            }

            let alt_tag = derive_csd5_tag(
                Csd5Primitive::Media,
                None,
                surface,
                instance,
                Some("alt"),
                Some(key),
            )
            .unwrap();
            let ai_tag = derive_csd5_tag(
                Csd5Primitive::Media,
                None,
                surface,
                instance,
                Some("ai_disclosure"),
                Some(key),
            )
            .unwrap();

            let h_root = handles.intern(root_tag, Some(Csd5Primitive::Media), None, None);
            let h_alt = handles.intern(
                alt_tag,
                Some(Csd5Primitive::Media),
                Some("alt".into()),
                Some(alt.clone()),
            );
            let h_ai = handles.intern(
                ai_tag,
                Some(Csd5Primitive::Media),
                Some("ai_disclosure".into()),
                Some(ai_disclosure.clone()),
            );

            let mut marks = Vec::new();
            marks.push(
                Instance::new(Kind::Quad, [x + 0.5 * w, *y + 20.0, 0.0], [w, 40.0, 0.0])
                    .colour(SLAB_BG)
                    .tagged(h_root),
            );
            emit_box_and_run(
                &mut marks,
                text,
                x + 6.0,
                *y + 6.0,
                0.40 * w,
                28.0,
                3.0,
                SLAB_BORDER,
                TEXT_PAPER,
                BODY,
                alt,
                h_alt,
                false,
            );
            emit_glyph_and_run(
                &mut marks,
                text,
                x + 0.42 * w,
                *y + 6.0,
                28.0,
                ACCENT_GREEN,
                SMALL,
                ai_disclosure,
                h_ai,
                false,
            );

            if *blurred {
                let btag = derive_csd5_tag(
                    Csd5Primitive::Media,
                    None,
                    surface,
                    instance,
                    Some("blur"),
                    Some(key),
                )
                .unwrap();
                let hb = handles.intern(
                    btag,
                    Some(Csd5Primitive::Media),
                    Some("blur".into()),
                    Some("blurred".into()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + 0.72 * w,
                    *y + 6.0,
                    80.0,
                    28.0,
                    3.0,
                    ACCENT_AMBER,
                    TEXT_PAPER,
                    SMALL,
                    "blurred",
                    hb,
                    false,
                );
            }

            if let Some(mc) = media_class {
                let ctag = derive_csd5_tag(
                    Csd5Primitive::Media,
                    None,
                    surface,
                    instance,
                    Some("class"),
                    Some(key),
                )
                .unwrap();
                let hc = handles.intern(
                    ctag,
                    Some(Csd5Primitive::Media),
                    Some("class".into()),
                    Some(mc.clone()),
                );
                emit_box_and_run(
                    &mut marks,
                    text,
                    x + 0.85 * w,
                    *y + 6.0,
                    0.14 * w,
                    28.0,
                    3.0,
                    SLAB_BG,
                    TEXT_MUTED,
                    SMALL,
                    mc,
                    hc,
                    false,
                );
            }

            let tid = *next_tile_id;
            *next_tile_id += 1;
            tiles.push(Tile {
                id: tid,
                origin: [0.0; 3],
                version,
                layer,
                screen: true,
                params: PARAMS_NEUTRAL,
                marks,
            });
            *y += 44.0;
        }
    }
}

// ============================================================================
// 6. BRIDGE: EpistemicScene -> Csd5Node Tree -> CompiledHolonScreen
// ============================================================================

fn clean_id(raw: &str) -> String {
    let mut s = String::new();
    for c in raw.chars() {
        if c.is_ascii_alphanumeric() {
            s.push(c.to_ascii_lowercase());
        } else if (c == '_' || c == '-') && !s.ends_with('_') && !s.is_empty() {
            s.push('_');
        }
    }
    while s.ends_with('_') {
        s.pop();
    }
    if s.is_empty() {
        "item".to_string()
    } else {
        s
    }
}

/// Lifts an existing CEWPOS `EpistemicScene` into the 24-primitive `CSD/5` `Csd5Node` algebra
/// and compiles it directly into a `holon-draw` `CompiledHolonScreen`.
pub fn lift_epistemic_scene_to_csd5(scene: &EpistemicScene, surface: &str) -> Vec<Csd5Node> {
    let surf = clean_id(surface);
    let mut out = Vec::new();

    for (idx, node) in scene.nodes.iter().enumerate() {
        match node {
            EpistemicNode::ShellHeader(h) => {
                out.push(Csd5Node::Shell {
                    prefix: ShellPrefix::Circle,
                    name: clean_id(h.circle.label()),
                    label: h.circle.label().to_string(),
                    active: true,
                });
                out.push(Csd5Node::Shell {
                    prefix: ShellPrefix::Tab,
                    name: clean_id(h.tab.as_tag()),
                    label: format!("{:?}", h.tab),
                    active: h.active_instrument.is_none(),
                });
                out.push(Csd5Node::Action {
                    surface: surf.clone(),
                    instance: "my_things".to_string(),
                    in_menu: false,
                    label: "My things".to_string(),
                    enabled: true,
                    disabled_reason: None,
                    requires_confirm: false,
                });
                out.push(Csd5Node::Action {
                    surface: surf.clone(),
                    instance: "stop_everything".to_string(),
                    in_menu: false,
                    label: "Stop everything".to_string(),
                    enabled: true,
                    disabled_reason: None,
                    requires_confirm: false,
                });
            }

            EpistemicNode::StateBanner {
                tag,
                state,
                data_doors_open_banner,
                ..
            } => {
                let (outcome, csd5_state, detail) = match state {
                    crate::algebra::SurfaceState::Loading => {
                        (ReadOutcome::Pending, Csd5State::Loading, None)
                    }
                    crate::algebra::SurfaceState::Populated => {
                        (ReadOutcome::Ok, Csd5State::Populated, None)
                    }
                    crate::algebra::SurfaceState::Empty { honest_zero } => {
                        (ReadOutcome::Ok, Csd5State::Empty, Some(honest_zero.clone()))
                    }
                    crate::algebra::SurfaceState::Error { title, body, .. } => (
                        ReadOutcome::Failed,
                        Csd5State::Error,
                        Some(format!("{title}: {body}")),
                    ),
                    crate::algebra::SurfaceState::FileGone { digest } => (
                        ReadOutcome::Ok,
                        Csd5State::Gone,
                        Some(format!("Gone: {digest}")),
                    ),
                    crate::algebra::SurfaceState::HiddenByRules { rule } => (
                        ReadOutcome::Ok,
                        Csd5State::HiddenByRules,
                        Some(format!("Hidden by rule: {rule}")),
                    ),
                    crate::algebra::SurfaceState::Unreviewed { summary } => (
                        ReadOutcome::Ok,
                        Csd5State::Unreviewed,
                        Some(summary.clone()),
                    ),
                };
                out.push(Csd5Node::StateBlock {
                    surface: surf.clone(),
                    instance: clean_id(tag),
                    outcome,
                    state: csd5_state,
                    detail_text: detail,
                    freshness: data_doors_open_banner.clone(),
                });
            }

            EpistemicNode::PolicyLens(pl) => {
                out.push(Csd5Node::PolicyLens {
                    surface: surf.clone(),
                    instance: clean_id(&pl.tag),
                    hidden_count: 0,
                    helpers: pl.helper_keys.clone(),
                    depth: pl.sort_label.clone(),
                });
            }

            EpistemicNode::Card {
                tag,
                title,
                fields,
                items,
                ..
            } => {
                let mut children = Vec::new();
                for f in fields {
                    let wire_val = match &f.basis {
                        crate::algebra::VerificationBasis::NotSent => None,
                        _ => Some(f.value.clone()),
                    };
                    children.push(Csd5Node::FieldRow {
                        surface: surf.clone(),
                        instance: clean_id(&f.tag),
                        label: f.label.clone(),
                        wire_value: wire_val,
                        gloss: f.gloss.clone().or_else(|| f.protocol_wire_name.clone()),
                    });
                }
                for it in items {
                    let rcpt_open = if it.has_hamburger() {
                        Some(
                            derive_csd5_tag(
                                Csd5Primitive::ReceiptSheet,
                                None,
                                &surf,
                                "item_receipt",
                                Some("open"),
                                Some(&clean_id(&it.id)),
                            )
                            .unwrap(),
                        )
                    } else {
                        None
                    };
                    children.push(Csd5Node::ItemRow {
                        surface: surf.clone(),
                        instance: "list".to_string(),
                        key: clean_id(&it.id),
                        title: it.title.clone(),
                        subtitle: it.subtitle.clone(),
                        flag: it.chips.first().map(|c| c.label.clone()),
                        act_label: it.primary_action.as_ref().map(|(l, _)| l.clone()),
                        scope_pill: Some(Box::new(Csd5Node::ScopePill {
                            surface: surf.clone(),
                            instance: clean_id(&it.scope_pill.tag),
                            circle_label: it.scope_pill.label.clone(),
                            group: if it.scope_pill.local_only_badge {
                                Some("local".to_string())
                            } else {
                                None
                            },
                        })),
                        receipt_open_tag: rcpt_open,
                    });
                }
                out.push(Csd5Node::CardShell {
                    surface: surf.clone(),
                    instance: clean_id(tag),
                    title: title.clone(),
                    children,
                });
            }

            EpistemicNode::ReceiptModal {
                tag,
                item_id,
                receipt,
            } => {
                out.push(Csd5Node::ReceiptSheet {
                    surface: surf.clone(),
                    instance: clean_id(tag),
                    key: clean_id(item_id),
                    is_open: true,
                    facts: [
                        receipt.subject_key_ids.display_value().to_string(),
                        receipt.attesting_key_id.display_value().to_string(),
                        receipt.cohort_scope.display_value().to_string(),
                        receipt.dimension.display_value().to_string(),
                        receipt.consent_scope.display_value().to_string(),
                    ],
                    holders: vec![format!("{} custody holders", receipt.holders)],
                    notes: receipt.notes.clone(),
                    acts: receipt.acts.iter().map(|a| format!("{a:?}")).collect(),
                });
            }

            EpistemicNode::ConfirmModal { tag, sheet } => {
                out.push(Csd5Node::ConfirmSheet {
                    surface: surf.clone(),
                    instance: clean_id(tag),
                    fact_1: sheet.facts[0].value.clone(),
                    fact_2: sheet.facts[1].value.clone(),
                    fact_3: sheet.facts[2].value.clone(),
                    note: sheet.danger_note.clone(),
                    confirm_label: "Confirm".to_string(),
                    cancel_label: "Cancel".to_string(),
                });
            }

            EpistemicNode::TwoPathCeremony(c) => {
                // Issue #3: Split P9 into Choice (when options are posed) + Ceremony (threshold/window/signers).
                if let Some(dv) = &c.deferral_vote {
                    out.push(Csd5Node::Choice {
                        surface: surf.clone(),
                        instance: clean_id(&c.tag_prefix),
                        question: dv.context_prompt.clone(),
                        options: dv
                            .choices
                            .iter()
                            .map(|ch| Csd5ChoiceOption {
                                key: clean_id(&ch.choice_id),
                                label: ch.label.clone(),
                                cost: ch.consequence.clone(),
                                tally: Some(format!("score {:+}", ch.vote_score)),
                                selected: ch.selected,
                            })
                            .collect(),
                    });
                }
                let spec = c.policy_family.spec();
                out.push(Csd5Node::Ceremony {
                    surface: surf.clone(),
                    instance: clean_id(&c.tag_prefix),
                    key: format!("c{idx}"),
                    act_price: format!("quorum {}/{}", spec.enact_num, spec.enact_den),
                    undo_price: format!("cancel {}/{}", spec.cancel_num, spec.cancel_den),
                    window: format!("{}s", spec.window_secs),
                    threshold: format!("{:?}", c.verdict),
                    signers: c
                        .signers
                        .iter()
                        .map(|s| (clean_id(&s.key_id), format!("{:?}", s.state)))
                        .collect(),
                });
            }

            EpistemicNode::QrCeremony(q) => {
                out.push(Csd5Node::QrCode {
                    surface: surf.clone(),
                    instance: clean_id(&q.tag),
                    is_scanner: q.tag.contains("scan"),
                    payload: q.payload_uri.clone(),
                    has_paste: q.paste_fallback_enabled,
                });
            }

            EpistemicNode::LineageChain(l) => {
                out.push(Csd5Node::LineageChain {
                    surface: surf.clone(),
                    instance: clean_id(&l.tag_prefix),
                    key: format!("l{idx}"),
                    links: l
                        .links
                        .iter()
                        .map(|lk| (format!("step_{}", lk.step_index), lk.label.clone()))
                        .collect(),
                    cascade: Some(l.title.clone()),
                });
            }

            EpistemicNode::ConscienceStep(cs) => {
                out.push(Csd5Node::ConscienceStep {
                    surface: surf.clone(),
                    instance: clean_id(&cs.tag_prefix),
                    key: format!("cs{idx}"),
                    summary: cs.task_summary.clone(),
                    cost: format!("{} tokens · {:.4} kWh", cs.cost.tokens, cs.cost.energy_kwh),
                    faculties: cs
                        .principles
                        .iter()
                        .map(|p| (clean_id(&p.code), p.title.clone()))
                        .collect(),
                });
            }

            EpistemicNode::SpatialCanvas {
                tag, holon_frame, ..
            } => {
                let mut world_nodes = Vec::new();
                let mut twin_rows = Vec::new();
                let inst_name = clean_id(tag);
                if let Some(hf) = holon_frame {
                    let root_pos = hf
                        .atoms
                        .first()
                        .map(|a| [a.x as f32, a.y as f32, a.z as f32])
                        .unwrap_or([0.0, 0.0, 0.0]);
                    for (ai, atom) in hf.atoms.iter().enumerate() {
                        let akey = format!("atom_{ai}");
                        world_nodes.push(GraphMarkSpec {
                            key: akey.clone(),
                            label: atom.label.clone(),
                            pos: [atom.x as f32, atom.y as f32, atom.z as f32],
                            radius: atom.radius.max(0.4),
                            rgba: if atom.selected {
                                ACCENT_GREEN
                            } else {
                                ACCENT_BLUE
                            },
                            bonded_to_pos: if ai > 0 { Some(root_pos) } else { None },
                        });
                        twin_rows.push(Csd5Node::ItemRow {
                            surface: surf.clone(),
                            instance: inst_name.clone(),
                            key: akey,
                            title: atom.label.clone(),
                            subtitle: atom.subtitle.clone(),
                            flag: None,
                            act_label: None,
                            scope_pill: None,
                            receipt_open_tag: None,
                        });
                    }
                }
                out.push(Csd5Node::GraphView {
                    surface: surf.clone(),
                    instance: inst_name,
                    world_nodes,
                    twin_rows,
                });
            }
        }
    }

    out
}

/// Builds a canonical showcase screen containing **every single one of the 24 `CSD/5` primitives**
/// and **all 60 slots**, plus the **4 shell prefixes** (`circle`, `tab`, `nav`, `screen`),
/// and compiles it through `holon-draw`.
pub fn build_all_24_primitives_showcase_screen(surface: &str) -> Vec<Csd5Node> {
    let s = surface.to_string();
    vec![
        // 4 Shell prefixes
        Csd5Node::Shell {
            prefix: ShellPrefix::Circle,
            name: "self".into(),
            label: "Just me".into(),
            active: true,
        },
        Csd5Node::Shell {
            prefix: ShellPrefix::Tab,
            name: "decisions".into(),
            label: "Decisions".into(),
            active: true,
        },
        Csd5Node::Shell {
            prefix: ShellPrefix::Nav,
            name: "this_node".into(),
            label: "This node".into(),
            active: false,
        },
        Csd5Node::Shell {
            prefix: ShellPrefix::Screen,
            name: "root".into(),
            label: "CEWPOS Epistemic Screen".into(),
            active: true,
        },
        // 1. CardShell (`title`) wrapping 2. FieldRow (`wire`, `gloss`), 3. ItemRow (`flag`, `act`),
        // 4. ScopePill (`group`), 5. Chip, 6. Action (`reason`)
        Csd5Node::CardShell {
            surface: s.clone(),
            instance: "main".into(),
            title: "CardShell Title".into(),
            children: vec![
                Csd5Node::FieldRow {
                    surface: s.clone(),
                    instance: "identity".into(),
                    label: "Root key".into(),
                    wire_value: Some("ed25519:alice".into()),
                    gloss: Some("Self-signed genesis key".into()),
                },
                Csd5Node::ItemRow {
                    surface: s.clone(),
                    instance: "claim".into(),
                    key: "row1".into(),
                    title: "Signed attestation".into(),
                    subtitle: "evaluation:quality".into(),
                    flag: Some("verified".into()),
                    act_label: Some("Inspect".into()),
                    scope_pill: Some(Box::new(Csd5Node::ScopePill {
                        surface: s.clone(),
                        instance: "vis".into(),
                        circle_label: "Family".into(),
                        group: Some("Moore Household".into()),
                    })),
                    receipt_open_tag: None,
                },
                Csd5Node::Chip {
                    surface: s.clone(),
                    instance: "status".into(),
                    token: "LIVE".into(),
                },
                Csd5Node::Action {
                    surface: s.clone(),
                    instance: "submit".into(),
                    in_menu: false,
                    label: "Send attestation".into(),
                    enabled: false,
                    disabled_reason: Some("Awaiting peer key record".into()),
                    requires_confirm: true,
                },
            ],
        },
        // 7. ReceiptSheet (`open`, `fact_1`..`fact_5`, `holders`, `notes`, `acts`)
        Csd5Node::ReceiptSheet {
            surface: s.clone(),
            instance: "env".into(),
            key: "row1".into(),
            is_open: true,
            facts: [
                "About: ed25519:bob".into(),
                "From: ed25519:alice".into(),
                "Scope: family".into(),
                "Kind: evaluation:quality (Account)".into(),
                "Rule: consent:scope (Wire)".into(),
            ],
            holders: vec!["node_a".into(), "node_b".into()],
            notes: vec!["No moderation flags".into()],
            acts: vec!["Withdraw".into(), "Supersede".into(), "Recant".into()],
        },
        // 8. StateBlock (`@states`, `freshness`)
        Csd5Node::StateBlock {
            surface: s.clone(),
            instance: "feed".into(),
            outcome: ReadOutcome::Ok,
            state: Csd5State::Populated,
            detail_text: Some("All rows verified".into()),
            freshness: Some("Just now".into()),
        },
        // 9. Metric (`unit`, `source`)
        Csd5Node::Metric {
            surface: s.clone(),
            instance: "keff".into(),
            outcome: ReadOutcome::Ok,
            value: Some("4.82".into()),
            unit: "sources".into(),
            source: "CC 6.1.2.3 n_eff".into(),
        },
        // 10. ConfirmSheet (`fact_1`, `fact_2`, `fact_3`, `note`, `confirm`, `cancel`)
        Csd5Node::ConfirmSheet {
            surface: s.clone(),
            instance: "widen".into(),
            fact_1: "1. What moves: evaluation:quality".into(),
            fact_2: "2. Where it goes: Everyone (federation)".into(),
            fact_3: "3. Can I undo it: Withdraws local copy; remote copies persist".into(),
            note: Some("G14 Replication boundary applies".into()),
            confirm_label: "Confirm & Sign".into(),
            cancel_label: "Cancel".into(),
        },
        // 11. Choice (`cost`, `tally`)
        Csd5Node::Choice {
            surface: s.clone(),
            instance: "posture".into(),
            question: "Operational posture".into(),
            options: vec![
                Csd5ChoiceOption {
                    key: "steward".into(),
                    label: "Run with AI Steward".into(),
                    cost: "Bonds depth-1 steward".into(),
                    tally: Some("1 vote".into()),
                    selected: true,
                },
                Csd5ChoiceOption {
                    key: "bare".into(),
                    label: "Bare Node".into(),
                    cost: "No AI runtime".into(),
                    tally: Some("0 votes".into()),
                    selected: false,
                },
            ],
        },
        // 12. DestinationPicker (`group`)
        Csd5Node::DestinationPicker {
            surface: s.clone(),
            instance: "dest".into(),
            destinations: vec![(
                "fam".into(),
                "Family Circle".into(),
                Some("Moore Household".into()),
            )],
        },
        // 13. Field (`ceiling`, `error`)
        Csd5Node::Field {
            surface: s.clone(),
            instance: "handle".into(),
            value: "alice.ciris".into(),
            byte_ceiling: 64,
            error: Some("Handle verified".into()),
        },
        // 14. StepFlow (`next`, `back`)
        Csd5Node::StepFlow {
            surface: s.clone(),
            instance: "wizard".into(),
            steps: vec![
                ("you".into(), "1. You".into()),
                ("fed".into(), "2. Federation".into()),
            ],
            active_key: "you".into(),
            has_back: true,
            has_next: true,
        },
        // 15. Ceremony (`act_price`, `undo_price`, `window`, `threshold`, `signer`)
        Csd5Node::Ceremony {
            surface: s.clone(),
            instance: "roster".into(),
            key: "add_dev".into(),
            act_price: "2-of-2 devices".into(),
            undo_price: "1 objection cancels".into(),
            window: "24h W_self".into(),
            threshold: "2/2 required".into(),
            signers: vec![("occ_a1".into(), "Signed".into())],
        },
        // 16. LineageChain (`link`, `cascade`)
        Csd5Node::LineageChain {
            surface: s.clone(),
            instance: "bond".into(),
            key: "b1".into(),
            links: vec![("l1".into(), "Human Steward -> Bonded Agent".into())],
            cascade: Some("Revoking steward bond halts agent".into()),
        },
        // 17. CodeCompare (`words`, `digits`, `verdict`)
        Csd5Node::CodeCompare {
            surface: s.clone(),
            instance: "sas".into(),
            words: "albatross · cedar · lantern".into(),
            digits: "482-910".into(),
            verdict: "Match verified".into(),
        },
        // 18. QrCode (`paste`)
        Csd5Node::QrCode {
            surface: s.clone(),
            instance: "pair".into(),
            is_scanner: false,
            payload: "ciris://ed25519:alice@127.0.0.1:4243".into(),
            has_paste: true,
        },
        // 19. Transcript (`receipt`, `fetch`)
        Csd5Node::Transcript {
            surface: s.clone(),
            instance: "room".into(),
            is_note: false,
            entries: vec![("m1".into(), "Hello constitutional mesh!".into(), true)],
            has_fetch: true,
        },
        // 20. ConscienceStep (`cost`, `faculty`)
        Csd5Node::ConscienceStep {
            surface: s.clone(),
            instance: "eval".into(),
            key: "c1".into(),
            summary: "H3ERE Conscience Pass".into(),
            cost: "120 tokens · 0.0002 kWh".into(),
            faculties: vec![
                ("epistemic".into(), "PASS".into()),
                ("harm".into(), "PASS".into()),
                ("autonomy".into(), "PASS".into()),
                ("justice".into(), "PASS".into()),
            ],
        },
        // 21. PolicyLens (`hidden_count`, `helpers`, `depth`)
        Csd5Node::PolicyLens {
            surface: s.clone(),
            instance: "filter".into(),
            hidden_count: 2,
            helpers: vec!["mod_helper_1".into()],
            depth: "2 hops".into(),
        },
        // 22. GraphView (`twin`)
        Csd5Node::GraphView {
            surface: s.clone(),
            instance: "mesh".into(),
            world_nodes: vec![
                GraphMarkSpec {
                    key: "n0".into(),
                    label: "Context Question".into(),
                    pos: [0.0, 0.0, 0.0],
                    radius: 1.2,
                    rgba: ACCENT_BLUE,
                    bonded_to_pos: None,
                },
                GraphMarkSpec {
                    key: "n1".into(),
                    label: "Selected Option".into(),
                    pos: [3.0, 1.5, 0.0],
                    radius: 0.9,
                    rgba: ACCENT_GREEN,
                    bonded_to_pos: Some([0.0, 0.0, 0.0]),
                },
            ],
            twin_rows: vec![
                Csd5Node::ItemRow {
                    surface: s.clone(),
                    instance: "mesh".into(),
                    key: "n0".into(),
                    title: "Context Question".into(),
                    subtitle: "setup:identity_path".into(),
                    flag: None,
                    act_label: None,
                    scope_pill: None,
                    receipt_open_tag: None,
                },
                Csd5Node::ItemRow {
                    surface: s.clone(),
                    instance: "mesh".into(),
                    key: "n1".into(),
                    title: "Selected Option".into(),
                    subtitle: "New CIRIS ID".into(),
                    flag: None,
                    act_label: None,
                    scope_pill: None,
                    receipt_open_tag: None,
                },
            ],
        },
        // 23. Document (`claim`, `recant`, `revision`, `cite`)
        Csd5Node::Document {
            surface: s.clone(),
            instance: "kb".into(),
            key: "d1".into(),
            revision: "rev 3".into(),
            claims: vec![DocumentClaimSpec {
                claim_key: "cl1".into(),
                text: "Earlier estimate was 10ms".into(),
                recanted_reason: Some("Recanted: measured at 42ms".into()),
                citation: Some("sha256:90f3c31".into()),
            }],
        },
        // 24. Media (`alt`, `ai_disclosure`, `blur`, `class`)
        Csd5Node::Media {
            surface: s,
            instance: "call".into(),
            key: "v1".into(),
            alt: "Live peer video stream".into(),
            ai_disclosure: "Human-captured (no AI synthesis)".into(),
            blurred: true,
            media_class: Some("av-stream-v2".into()),
        },
    ]
}

/// Convenience helper: compiles an `EpistemicScene` into a `CompiledHolonScreen` with `FakeText`.
pub fn compile_epistemic_scene_with_holon_draw(
    scene: &EpistemicScene,
    surface: &str,
    version: u64,
    dpr: f32,
) -> (Vec<Csd5Node>, CompiledHolonScreen) {
    let nodes = lift_epistemic_scene_to_csd5(scene, surface);
    let mut text = FakeText { dpr, placed: 0 };
    let compiled = compile_csd5_screen(&nodes, &mut text, version);
    (nodes, compiled)
}

/// Serializes a `holon_draw::Frame` and its `HandleTable` into a JSON-friendly summary for
/// `/cewpos/v1/holon-draw` and the WebGL `draw.js` / `text.js` browser backend.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HolonDrawFrameExport {
    pub standard: String,
    pub draft: u32,
    pub pinned_tag: String,
    pub version: u64,
    pub fine_tiles: u32,
    pub coarse_tiles: u32,
    pub stale_tiles: u32,
    pub remainder_px: f64,
    pub frame_digest: u64,
    pub instances_drawn: u64,
    pub batch_count: usize,
    pub handle_count: usize,
    pub handles: Vec<HandleEntry>,
    pub violations: Vec<LawViolation>,
}

impl CompiledHolonScreen {
    pub fn export_summary(&self, frame: &Frame) -> HolonDrawFrameExport {
        HolonDrawFrameExport {
            standard: CSD5_STANDARD.to_string(),
            draft: CSD5_DRAFT,
            pinned_tag: CSD5_PINNED_TAG.to_string(),
            version: self.version,
            fine_tiles: frame.fine,
            coarse_tiles: frame.coarse,
            stale_tiles: frame.stale,
            remainder_px: frame.remainder_px,
            frame_digest: frame.digest(),
            instances_drawn: frame.drawn(),
            batch_count: frame.batches.len(),
            handle_count: self.handles.len(),
            handles: self.handles.entries().cloned().collect(),
            violations: self.violations.clone(),
        }
    }

    pub fn batches<'a>(&self, frame: &'a Frame) -> &'a [Batch] {
        &frame.batches
    }
}

// ============================================================================
// 7. EXHAUSTIVE UNIT TESTS AGAINST `epistemic-ui/ui_primitives.json` & `holon-draw`
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::{FirstRunWizardState, ViewerContext};
    use crate::epistemic_scene::{project_epistemic_scene, project_first_run_wizard_scene};
    use std::collections::BTreeSet;

    #[test]
    fn vendored_ui_primitives_json_matches_24_primitives_60_slots_17_states_12_laws() {
        let raw: serde_json::Value =
            serde_json::from_str(UI_PRIMITIVES_JSON).expect("valid vendored ui_primitives.json");

        assert_eq!(raw["_meta"]["standard"], "CSD/5");
        assert_eq!(raw["_meta"]["draft"], 3);

        // 1. Check 4 shell prefixes
        let shell_obj = raw["shell_prefixes"].as_object().unwrap();
        assert_eq!(shell_obj.len(), 4);
        for sp in ShellPrefix::ALL {
            assert!(shell_obj.contains_key(sp.as_str()));
        }

        // 2. Check 5 write verbs
        let verbs: Vec<&str> = raw["write_verbs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        assert_eq!(verbs.len(), 5);
        for wv in Csd5WriteVerb::ALL {
            assert!(verbs.contains(&wv.as_str()));
        }

        // 3. Check 17 states and their read outcomes
        let states_obj = raw["states"].as_object().unwrap();
        assert_eq!(states_obj.len(), 17);
        for st in Csd5State::ALL {
            let entry = states_obj
                .get(st.as_str())
                .unwrap_or_else(|| panic!("missing state {}", st.as_str()));
            assert_eq!(entry["outcome"].as_str().unwrap(), st.outcome().as_str());
            assert!(!st.default_form_label().is_empty());
        }

        // 4. Check 12 laws
        let laws_obj = raw["laws"].as_object().unwrap();
        assert_eq!(laws_obj.len(), 12);
        for law in Csd5Law::ALL {
            assert!(laws_obj.contains_key(law.as_str()));
        }

        // 5. Check 24 primitives, their prefixes (L1 one owner), keyed flag, and 60 total slots
        let prims_obj = raw["primitives"].as_object().unwrap();
        assert_eq!(prims_obj.len(), 24);
        let mut total_slots = 0usize;
        let mut seen_prefixes = BTreeSet::new();

        for p in Csd5Primitive::ALL {
            let entry = prims_obj
                .get(p.name())
                .unwrap_or_else(|| panic!("missing primitive {}", p.name()));
            let json_prefixes: Vec<&str> = entry["prefixes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            assert_eq!(json_prefixes, p.prefixes(), "prefixes mismatch on {:?}", p);
            for pfx in p.prefixes() {
                assert!(
                    seen_prefixes.insert(*pfx),
                    "L1 violation: duplicate prefix {pfx}"
                );
                assert_eq!(Csd5Primitive::from_prefix(pfx), Some(p));
            }

            let json_keyed = entry["keyed"].as_bool().unwrap();
            assert_eq!(json_keyed, p.is_keyed(), "keyed mismatch on {:?}", p);

            let json_slots: Vec<&str> = entry["slots"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            assert_eq!(json_slots, p.slots(), "slots mismatch on {:?}", p);
            total_slots += p.slots().len();
        }
        assert_eq!(total_slots, 60, "CSD/5 draft 3 has exactly 60 slots across the 24 primitives");
    }

    #[test]
    fn tag_grammar_places_slot_before_key_and_rejects_unowned_prefixes() {
        // INTERFACE.md §10.1: slot BEFORE key (`row_s_i__flag.k`, never `row_s_i.k__flag`)
        let t = derive_csd5_tag(
            Csd5Primitive::ItemRow,
            None,
            "my-surface",
            "claims",
            Some("flag"),
            Some("row:42"),
        )
        .unwrap();
        assert_eq!(t, "row_my_surface_claims__flag.row:42");

        // Unowned prefix is rejected (`L1`)
        assert!(matches!(
            derive_csd5_tag(
                Csd5Primitive::ReceiptSheet,
                Some("btn"),
                "s",
                "i",
                Some("open"),
                Some("k1")
            ),
            Err(TagError::PrefixNotOwnedByPrimitive { .. })
        ));

        // Unkeyed primitive rejects a key
        assert!(matches!(
            derive_csd5_tag(
                Csd5Primitive::CardShell,
                None,
                "s",
                "i",
                Some("title"),
                Some("k1")
            ),
            Err(TagError::KeyNotAllowedForPrimitive { .. })
        ));
    }

    #[test]
    fn all_24_primitives_and_60_slots_compile_into_holon_draw_and_pass_pick_and_laws() {
        let nodes = build_all_24_primitives_showcase_screen("showcase");
        let mut text = FakeText {
            dpr: 1.0,
            placed: 0,
        };
        let compiled = compile_csd5_screen(&nodes, &mut text, 1);
        assert!(
            compiled.violations.is_empty(),
            "showcase screen must have 0 law violations: {:?}",
            compiled.violations
        );

        // Verify every one of the 24 primitives is represented in the HandleTable
        let present_prims: BTreeSet<Csd5Primitive> = compiled
            .handles
            .entries()
            .filter_map(|e| e.primitive)
            .collect();
        assert_eq!(
            present_prims.len(),
            24,
            "all 24 CSD/5 primitives must be compiled into handles"
        );

        // Build a holon-draw Frame with the Acuity allocator
        let cam = Camera::orbit([0.0, 0.0, 0.0], 0.6, 0.4, 15.0, 0.8, 1280.0, 2200.0);
        let frame = compiled
            .evaluate_frame(&cam, 1.0, u64::MAX)
            .expect("all marks are finite (L4)");
        assert!(frame.drawn() > 50);
        assert_eq!(frame.stale, 0);

        // Modal law (L6/L7): ConfirmSheet is open on layer 12, so modal is Some(12)
        // and confirm/cancel buttons are pickable on the modal layer while background cards are blocked!
        assert_eq!(frame.modal, Some(12));
        let (confirm_shown, confirm_entry) = compiled
            .report_tag(&frame, "sheet_showcase_widen__confirm")
            .expect("ConfirmSheet confirm button is drawn");
        assert!(confirm_shown.on_screen && confirm_shown.legible);
        let cx = 0.5 * (confirm_shown.bounds[0] + confirm_shown.bounds[2]);
        let cy = 0.5 * (confirm_shown.bounds[1] + confirm_shown.bounds[3]);
        let picked = compiled
            .pick_tag(&frame, cx, cy)
            .expect("pick hits ConfirmSheet confirm");
        assert_eq!(picked.tag, confirm_entry.tag);

        // Verify GraphView twin sharing (INTERFACE.md §0, §8.1, §9):
        // `row_showcase_mesh.n1` has both screen marks (ItemRow) and world marks (Sphere + Rod),
        // so its `Shown::marks` count is >= 3!
        let (twin_shown, _) = compiled
            .report_tag(&frame, "row_showcase_mesh.n1")
            .expect("GraphView twin ItemRow is drawn");
        assert!(
            twin_shown.marks >= 3,
            "GraphView twin shares its handle between the screen ItemRow and 3D world Sphere+Rod marks"
        );

        // Verify L12 (`shown_digest` verification) on the Field and Choice values:
        let digest = compiled
            .verify_l12_shown_is_sent(
                &frame,
                "sheet_showcase_widen__confirm",
                &[
                    ("input_showcase_handle", "alice.ciris"),
                    ("opt_showcase_posture.steward", "Run with AI Steward"),
                ],
            )
            .expect("L12 shown-is-sent verification succeeds when values match");
        assert_ne!(digest, 0);

        // Tampering with the sent string must fail L12!
        let bad = compiled.verify_l12_shown_is_sent(
            &frame,
            "sheet_showcase_widen__confirm",
            &[("input_showcase_handle", "mallory.ciris")],
        );
        assert!(matches!(bad, Err(LawViolation { law: Csd5Law::L12, .. })));
    }

    #[test]
    fn laws_l3_l4_l5_l10_catch_violations() {
        let bad_nodes = vec![
            // L3 violation: StateBlock `populated` when read outcome is `pending`
            Csd5Node::StateBlock {
                surface: "test".into(),
                instance: "s1".into(),
                outcome: ReadOutcome::Pending,
                state: Csd5State::Populated,
                detail_text: None,
                freshness: None,
            },
            // L4 honest number: Metric with pending read renders em dash "—", never "0"
            Csd5Node::Metric {
                surface: "test".into(),
                instance: "m1".into(),
                outcome: ReadOutcome::Pending,
                value: Some("42".into()),
                unit: "ms".into(),
                source: "probe".into(),
            },
            // L5 violation: disabled Action with no reason
            Csd5Node::Action {
                surface: "test".into(),
                instance: "a1".into(),
                in_menu: false,
                label: "Act".into(),
                enabled: false,
                disabled_reason: None,
                requires_confirm: false,
            },
            // L10 violation: Media without alt text is refused and not drawn
            Csd5Node::Media {
                surface: "test".into(),
                instance: "med1".into(),
                key: "k1".into(),
                alt: "".into(),
                ai_disclosure: "Human".into(),
                blurred: false,
                media_class: None,
            },
        ];

        let mut text = FakeText {
            dpr: 1.0,
            placed: 0,
        };
        let compiled = compile_csd5_screen(&bad_nodes, &mut text, 1);

        assert!(compiled
            .violations
            .iter()
            .any(|v| v.law == Csd5Law::L3 && v.tag == "state_test_s1"));
        assert!(compiled
            .violations
            .iter()
            .any(|v| v.law == Csd5Law::L5 && v.tag == "btn_test_a1"));
        assert!(compiled
            .violations
            .iter()
            .any(|v| v.law == Csd5Law::L10 && v.tag == "media_test_med1.k1"));

        // L10: bare Media was NOT interned or drawn!
        assert!(compiled.handles.get_by_tag("media_test_med1.k1").is_none());

        // L4: Metric with pending outcome rendered "—" instead of "42" or "0"
        let metric_entry = compiled.handles.get_by_tag("stat_test_m1").unwrap();
        assert_eq!(metric_entry.rendered_text.as_deref(), Some("—"));
    }

    #[test]
    fn epistemic_scene_and_first_run_wizard_compile_through_csd5_and_holon_draw() {
        let ctx = ViewerContext::default();
        let wiz = FirstRunWizardState::default();
        let wiz_scene = project_first_run_wizard_scene(&wiz, &ctx);
        let (_, compiled_wiz) =
            compile_epistemic_scene_with_holon_draw(&wiz_scene, "wizard", 1, 1.0);
        assert!(compiled_wiz.violations.is_empty());

        let cam = Camera::orbit([0.0, 0.0, 0.0], 0.5, 0.35, 18.0, 0.8, 1280.0, 900.0);
        let frame = compiled_wiz.evaluate_frame(&cam, 1.0, u64::MAX).unwrap();
        assert!(frame.drawn() > 20);

        let app_scene = project_epistemic_scene(&ctx, &[], &[]);
        let (_, compiled_app) =
            compile_epistemic_scene_with_holon_draw(&app_scene, "circles", 1, 1.0);
        assert!(compiled_app.violations.is_empty());
    }
}
