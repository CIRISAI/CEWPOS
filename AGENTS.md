# AGENTS.md — CEWPOS Architecture, Primitives-Based Rendering, and CEG Projection Guide

This file is the canonical engineering guide for coding agents and contributors working in **CEWPOS** (CIRIS Epistemic Web Platform OS). Read this before modifying any UI surface, view-form, evaluator rule, or server route binding.

---

## 1. Core Architectural Law: Full-Screen Bevy Scene & Primitives-Based Rendering

### 1.1 The Bevy Scene IS the Entire Screen (`100vw × 100vh`)
- **Never embed a 3D sub-canvas inside a 2D web page.** In CEWPOS, the Bevy / `CIRISHolon` 3D scene (`#bevyViewport` + `<canvas id="bevySceneCanvas">` in `epistemic-ui/index.html`, backed by `view-forms-spike` and `CIRISHolon` `holon-render-3d`) **is the entire viewport (`100vw × 100vh`)**.
- **Modern Spatial Interface with Typography & Selections — Not Molecular Spheres, Not HTML Web Forms:**
  - Do **not** render user-facing epistemic states as floating molecular spheres (`Z=8 Oxygen`, `STO-3G` atoms) and do **not** render them as traditional 2D HTML web forms.
  - Instead, every surface in the full-screen Bevy scene is projected as a **3D Spatial Slab** floating in architectural perspective space (`perspective: 1600px`, `transform-style: preserve-3d`), featuring crisp editorial typography (`IBM Plex Sans` / `Newsreader` / `IBM Plex Mono`), tactile 3D selection tiles (`transform: translate3d(0, -3px, 22px)` when selected), and live 3D selection splines drawn on the full-screen `#bevySceneCanvas` connecting each **Question Context Slab** (`[data-bevy-question="1"]`) to its **Selection Slabs** (`[data-bevy-choice="1"]`).

### 1.2 Context is the Question; Choices are 3D Selections (`P9: TwoPathCeremonyBlock` / `DeferralVote`)
- Every decision, deferral, or setup step in CEWPOS is a **signed epistemic vote** rendered through **Primitive `P9` (`TwoPathCeremonyBlock` / `DeferralVote`)**:
  - **The Context is posed as a clear Question** at the top of the 3D slab (`.p9-question-hero` with `[data-bevy-question="1"]`, protocol dimension eyebrow, plain-language question title, consequence subtitle, scope pill `P5`, and quorum badge `P6`).
  - **The Choices are posed as modern 3D Selection Tiles** below the question (`.p9-selection-grid` -> `.p9-selection-tile` with `[data-bevy-choice="1"]`, check indicator, clear title, consequence subtitle, and `☰` CEG hamburger trigger).
- **The First-Run Setup Wizard (`CSD-080`–`CSD-085`, `CSD-093`) is a sequence of 4 single-person (`1-of-1`) `P9` DeferralVotes:**
  1. **Step 1 (`YOU` — `setup:identity_path`)**: *"How do you want to establish your root identity on this device?"* (`Set up a new CIRIS ID` vs. `Add this device to an existing CIRIS ID`).
  2. **Step 2 (`JOIN_FEDERATION` — `consent:trace_replication`)**: *"Do you want to share anonymized diagnostic traces with the CIRIS federation?"* (`Share anonymized traces` vs. `Keep traces local only`).
  3. **Step 3 (`AI` — `steward:bond:v1`)**: *"Which inference engine should your bonded AI steward use?"* (`Local Ollama (Keyless)` vs. `CIRIS Steward Portal` vs. `Bring Your Own API Key`).
  4. **Step 4 (`COMPLETE` — `setup:claim_remote`)**: *"Are you ready to seal your 0600 claim_pin and claim ownership of this node?"* (`Claim node with 0600 PIN` vs. `Leave Unclaimed`).
- Exactly the same `P9` Question + Selection primitive renders:
  - **Single-person (`1-of-1`) node owner deferrals** (`GET /v1/self/owner-requests`, `POST /v1/self/owner-requests/{id}/decide`);
  - **Wise Authority (`1-of-1` or panel) WBD deferrals** (`CSD-117`);
  - **Multi-device (`2-of-2` + `24h W_self`) self-key roster ceremonies** (`CSD-093`, `CSD-094`, `CSD-095`);
  - **Multi-party (`m-of-n` + silence window) Circle governance ceremonies** (`Decisions` tab across all 5 Circles).

---

## 2. The 12 Closed UX Primitives (`P1`–`P12`)

Every epistemic web state across all **5 Circles × 7 Tabs × 6 Instruments × 101 `CSD/4` cards** is projected exclusively from a **closed algebra of 12 UX Primitives** (`view-forms-spike/src/epistemic_scene.rs` and `epistemic-ui/index.html`). Adding a new feature or domain **never** introduces a 13th primitive; it composes `P1`–`P12` over new CEG vocabulary.

| ID | Primitive Name | Rust Scene IR (`epistemic_scene.rs`) | Role in Full-Screen Bevy Scene |
| :--- | :--- | :--- | :--- |
| **`P1`** | `CardShell` | `UxPrimitive::CardShell` | 3D spatial slab container with title, status `P6` chip, and `⋯` progressive disclosure drawer. |
| **`P2`** | `FieldRow` | `UxPrimitive::FieldRow` | Key/value row binding a plain-language label to a wire field (`FactProvenance::Wire`, `ByRule`, or `NotSent`). |
| **`P3`** | `ItemRow` | `UxPrimitive::ItemRow` | Signed CEG claim row carrying a `P5` ScopePill and the **Universal CEG Hamburger (`☰`)** that opens `P4`. |
| **`P4`** | `ReceiptSheet` (`☰`) | `UxPrimitive::ReceiptSheet` | Universal CEG Hamburger sheet exposing the 5 envelope facts, 3D `view_form` slab preview, 9 uniform `AttOp`s, blob/custody acts, and `CC 4.5.4` moderation acts. |
| **`P5`** | `ScopePill` | `UxPrimitive::ScopePill` | Circle visibility badge (`Just me`, `Family`, `Neighbours`, `Communities & Businesses`, `Everyone`) + local-only indicator. |
| **`P6`** | `Chip` | `UxPrimitive::Chip` | Status or interactive choice badge (`Status | Choice`). |
| **`P7`** | `StateBlock` | `UxPrimitive::StateBlock` | Mandatory surface state banner (`Populated | Loading | Empty | Error | FileGone | HiddenByRules | Unreviewed | Halt423`). |
| **`P8`** | `ConfirmSheet` | `UxPrimitive::ConfirmSheet` | 3-fact outward-action confirmation gate (*1. What moves · 2. Where it goes · 3. Can I undo it?* + `G14` replication notice). |
| **`P9`** | `TwoPathCeremonyBlock` (`DeferralVote`) | `UxPrimitive::TwoPathCeremonyBlock` + `DeferralVoteNode` | Unified **Question Context + 3D Selection Tiles** primitive for all `1-of-1` deferrals (First-Run Wizard, OwnerRequests, WA rulings) and `m-of-n` two-path ceremonies. |
| **`P10`** | `QrCeremony` | `UxPrimitive::QrCeremony` | Multi-device pairing & key-exchange slab (`ciris://ed25519:...@host:4243` + 6-digit SAS match code). |
| **`P11`** | `LineageChainBlock` | `UxPrimitive::LineageChainBlock` | Cryptographic delegation & bond chain (`Human Steward -> Bonded Agent`, `Inv_BondDepthOne`, or `Genesis -> Supersedes -> Active Head`). |
| **`P12`** | `ConscienceStepBlock` (+ `PolicyLensBar`) | `UxPrimitive::ConscienceStepBlock` / `PolicyLensBar` | H3ERE conscience invariant evaluations, `CSD-118` Never-Allowed hard refusals (`G17`), `G14` non-verifiable boundary notice, and quiet policy lens bar. |

### Directional Asymmetry Law (`Inv_ExitOneStep` vs. `P8: ConfirmSheet`)
- **Outward / widening / delegating / irreversible acts** always require a 3-fact `P8: ConfirmSheet`.
- **Inward / narrowing / withdrawing / leaving (`Inv_ExitOneStep`) / halting (`423` Stop everything) / refusing (`G17`) acts** execute in **one immediate step** without confirmation friction.

---

## 3. CEWPOS CEG Rendering & Attestation Calculus

### 3.1 The Closed `1+4` CEG Grammar
Every consequential object in CEWPOS — an identity root, device enrollment, file custody manifest, chat message, community membership, moderation flag, or AI conscience deferral — is an immutable, JCS-canonical (RFC 8785), content-addressed (SHA-256), hybrid post-quantum signed (`Ed25519 + ML-DSA-65`) **CEG attestation** (`Form` in `attestation-calculus-spike/src/lib.rs`) using one of the **1+4 closed operators**:
1. **`scores`** (`op:scores`): Evaluates a `subject` along a `dimension` with `score ∈ [-1.0, +1.0]`, `confidence ∈ [0.0, 1.0]`, and `cohort_scope`.
2. **`delegates_to`** (`op:delegates_to`): Grants scoped authority (`agency:operate`, `agency:defer_wa`, etc.) to a delegate key (`Inv_BondDepthOne`).
3. **`supersedes`** (`op:supersedes`): Replaces an earlier attestation ID with a new revision while preserving append-only lineage.
4. **`withdraws`** (`op:withdraws`): Producer or named-subject retraction (`Inv_ExitOneStep`).
5. **`recants`** (`op:recants`): Producer fault-admission retraction (`score = -1.0` on record).

### 3.2 Deterministic View-Form Projection (`view_form(&Form)` & `project_epistemic_scene`)
- **Zero-Import Capability Sandbox (`view-form-component/`)**: View-forms compile to `wasm32-wasip2` components instantiated with **zero host imports** (no network, no filesystem, no clock, no ambient authority). Any component demanding a host import is rejected at instantiation.
- **From CEG `Form` to 3D Bevy `Scene` (`view-forms-spike/src/lib.rs` & `epistemic_scene.rs`)**:
  - `view_form(f: &Form) -> Scene` maps a single CEG attestation deterministically to a 3D `Scene` (camera, directional light, floor plane, and operator-specific 3D node geometry).
  - `project_epistemic_scene(input: &EpistemicViewInput) -> EpistemicProjection` projects any `(Circle, Tab, Instrument, HaltState, Claims, DeferralVote)` tuple into:
    1. A complete 3D Bevy `Scene` (`SceneNode`s positioned across the 5 concentric Circle rings and 3D Question/Selection slabs); and
    2. A deterministic tree of `UxNode`s (`P1`–`P12`) carrying exact `CSD/4` tags (`data-csd-tag`).

### 3.3 The Universal CEG Hamburger (`☰` / `P4: ReceiptSheet`)
If an item on screen has a `☰` hamburger button, it is a signed CEG attestation (or referent such as a file blob or community room); if it does not, it is app chrome. Opening `☰` renders **`P4: ReceiptSheet`** (`openReceiptSheet(claimId)` in `epistemic-ui/index.html`), which provides 5 unified sections on every CEG object:
1. **The 5 Plain-Language Envelope Facts (`CSD-006`)**:
   - *Who it is about* (`subject_key_ids`)
   - *Who sent it* (`attesting_key_id`)
   - *Who can see it* (`cohort_scope` via `P5: ScopePill`)
   - *What it is* (`dimension` + `ChangeKind` ontological badge: `THING | ACCOUNT | RECORD`)
   - *The rule it follows* (`consent:scope` + `FactProvenance` badge: `Wire | ByRule | NotSent`)
   - Plus **1-step Take back to Just me** (`Inv_ExitOneStep`) when the viewer is the producer or a named subject.
2. **3D Spatial Slab Preview (`view_form(&Form)`)**:
   - Interactive 3D slab projection showing the **Context Question Slab (`P9`)** connected via a 3D selection spline to the **Signed Selection / Envelope Slab (`op:${claim.op}`)**.
3. **The 9 Uniform `AttOp`s (`RC9` `12 × 9` Occupancy Matrix)**:
   - `Cosign`, `Withdraw`, `Supersede`, `Recant`, `Verify`, `History`, `Evidence`, `Details`, `Copy` — gated per `ChangeKind` by `ATTOP_MATRIX` (`algebra_engine.py`) and wired to live `CIRISServer v0.5.243` endpoints (`/v1/chat/{comm}/messages/{id}/{withdraw|supersede|recant}`).
4. **Blob & Referent Custody Acts (`Files` / `Blob` referents)**:
   - `CustodyHolders` (`GET /v1/files/{id}/custody`), `RenameBlob` (`POST /v1/files/{id}/rename`), and `MoveCircle` (`POST /v1/files/{id}/move`).
5. **`CC 4.5.4` Moderation & Content Note Acts**:
   - `FlagContent` (`POST /v1/safety/flag`), `RevealBlurred` (`POST /v1/chat/{comm}/items/{id}/reveal`), and Named Moderator actions (`POST /v1/safety/moderation`: `blur`, `warn`, `remove`).

---

## 4. Full-Stack `ciris-server` (`v0.5.243`) Crate Consumption & Verification

CEWPOS consumes [`CIRISServer`](https://github.com/CIRISAI/CIRISServer) directly as a Rust crate at **`tag = "v0.5.243"`** (`ciris-server` and `ciris-Select-core` in `view-forms-spike/Cargo.toml` and `attestation-calculus-spike/Cargo.toml`).

### 4.1 Building the Full-Stack `cewpos-node` Binary & Running Rust Tests
```bash
# 1. Run the 30 Attestation Calculus tests (including live ciris-server v0.5.243 router integration)
cargo test --manifest-path attestation-calculus-spike/Cargo.toml

# 2. Run the 24 View-Forms & P1-P12 Epistemic Scene tests (GPU-free & wasm-portable)
cargo test --manifest-path view-forms-spike/Cargo.toml --no-default-features

# 3. Build the unified full-stack cewpos-node binary (CIRISServer v0.5.243 + Bevy P1-P12 Spatial Scene)
cargo build --manifest-path view-forms-spike/Cargo.toml --no-default-features --features fullstack --bin cewpos-node
```

### 4.2 Running the Formal UI Algebra & Live `CSD/4` Conformance Harness
```bash
# Verify the 72-cell (12 ChangeKind x 6 Verb) RC9 matrix, 12x9 AttOp matrix, and 101/101 CSD/4 cards
python3 epistemic-ui/algebra_engine.py

# Run the 11-step live QA conformance harness against CIRISServer v0.5.243 (:4243) and CEWPOS (:8787)
python3 epistemic-ui/csd4_harness.py all
```

### 4.3 Non-Negotiable Engineering Rules for Agents
1. **No Untyped Dicts**: Use explicit structs/enums in Rust and typed schema records in Python/JS.
2. **Full-Screen Bevy Scene Only**: Never reintroduce embedded `<canvas>` widgets inside cards or molecular sphere visualizations (`Z=8 Oxygen`) for user-facing decisions. Keep `#bevySceneCanvas` full-screen (`100vw × 100vh`) and render context as questions (`.p9-question-hero`) and choices as 3D selection slabs (`.p9-selection-tile`).
3. **Preserve `data-csd-tag` Attributes**: Every interactive element and surface block must retain its canonical `CSD/4` tag (`data-csd-tag`) so `/tree` and `csd4_harness.py all` pass `11/11` checks.
