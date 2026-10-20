//! # view-forms-spike — typed view-forms over the Attestation Calculus
//!
//! Proves the pipeline:
//!
//! ```text
//!   CEG `scores` attestation                 (a real Form, read by the
//!        |                                     closed-operator gate
//!        |  attestation_calculus_spike::forms::read_form)
//!        v
//!   view_form(&Form) -> Scene                 (PURE, TOTAL, no IO/GPU)
//!        |
//!        v
//!   Scene  (our renderer-neutral typed 3D IR; NO Bevy here)
//!        |
//!        v
//!   Bevy 0.19 entities  ->  off-screen RenderTarget::Image  ->  Readback
//!        |
//!        v
//!   an actual PNG on disk         (src/bin/render.rs, native macOS Metal)
//! ```
//!
//! [`scene`] and [`view_form`] are the **pure** half: no renderer dependency, so
//! they compile to `wasm32-unknown-unknown` and the view logic is browser-
//! portable. The Bevy renderer lives in `src/bin/render.rs` behind the `render`
//! feature.

pub mod algebra;
pub mod epistemic_scene;
pub mod scene;
pub mod view_form;

// Re-export the real calculus types so a consumer (and the render bin) can read
// a Form and view it through one crate.
pub use algebra::{
    evaluate_policy_lens, ActionArchetype, ActionIntent, AgentGateVerdict, ChangeKind, CircleScope,
    CircleTab, ConfirmFact, ConfirmFacts3, GateRequirement, GovernancePolicyFamily,
    GovernancePolicySpec, HaltVerdict, InstrumentId, MemberStage, OntologyClass, PolicyLensVerdict,
    Receipt5, ReceiptAct, RetirementStage, RootVerdict, RowStanding, StatePlaneVector,
    SurfaceState, Tone, TopologyId, TwoPathVerdict, Verb, VerificationBasis, ViewerContext,
};
pub use attestation_calculus_spike::forms;
pub use epistemic_scene::{
    project_epistemic_scene, project_promise_card, Csd4TreeNode, EpistemicNode, EpistemicScene,
};
pub use view_form::view_form;
