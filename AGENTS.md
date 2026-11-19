# AGENTS.md — CEWPOS Architecture, Primitives-Based Rendering, and CEG Projection Guide

This file is the canonical engineering guide for coding agents and contributors working in **CEWPOS** (CIRIS Epistemic Web Platform OS). Read this before modifying any UI surface, view-form, evaluator rule, or server route binding.

---

## 1. Core Architectural Law: Full-Screen Bevy Scene & Primitives-Based Rendering

### 1.1 The Bevy Scene IS the Entire Screen (`100vw × 100vh`)
- **Never embed a 3D sub-canvas inside a 2D web page.** In CEWPOS, the Bevy / `CIRISHolon` 3D scene (`#bevyViewport` + `<canvas id="bevySceneCanvas">` in `epistemic-ui/index.html`, backed by `view-forms-spike` and `CIRISHolon` `holon-render-3d`) **is the entire viewport (`100vw × 100vh`)**.
- **Modern Spatial Interface with Typography & Selections — Not Molecular Spheres, Not HTML Web Forms:**
  - Do **not** render user-facing epistemic states as floating molecular spheres (`Z=8 Oxygen`, `STO-3G` atoms) and do **not** render them as traditional 2D HTML web forms.
  - Instead, every surface in the full-screen Bevy scene is projected as a **3D Spatial Slab** floating in architectural perspective space (`perspective: 1600px`, `transform-style: preserve-3d`), featuring crisp editorial typography (`IBM Plex Sans` / `Newsreader` / `IBM Plex Mono`), tactile 3D selection tiles (`transform: translate3d(0, -3px, 22px)` when selected), and live 3D selection splines drawn on the full-screen `#bevySceneCanvas` connecting each **Question Context Slab** (`[data-bevy-question="1"]`) to its **Selection Slabs** (`[data-bevy-choice="1"]`).

### 1.2 Primitive P9 is ChoiceBlock: Context, Options, and Consequences
- **The Primitive is not "deferral" — it is Choice (`P9: ChoiceBlock`)**:
  - Every decision, setup step, consent gate, moderation action, deferral, or governance vote in CEWPOS is a unified **`P9` ChoiceBlock** consisting of:
    1. **Context**: Poses the situation, question, or inquiry clearly (`.p9-question-hero` with `[data-bevy-question="1"]`, protocol dimension eyebrow, plain-language question title, explanatory subtitle, and scope pill `P5`).
    2. **Options**: The available choices posed as tactile 3D Selection Tiles (`.p9-selection-grid` -> `.p9-selection-tile` with `[data-bevy-choice="1"]`, selection state indicator, clear option title, and optional `☰` CEG receipt trigger).
    3. **Consequences (What would change based on each option)**: Explicit, plain-language explanation rendered directly inside each option tile indicating the precise state mutation, cryptographic attestation, or policy impact before selection.
- **Context Determines the Role of the Choice**:
  - **First-Run Wizard Step**: Setup paths where context is device initialization (`setup:identity_path`, `mode:operational_posture`, `consent:trace_replication`, `steward:bond:v1`, `setup:claim_remote`).
  - **Consent Gate**: Boundary decisions where context is telemetry sharing or federation reachability.
  - **Agent Deferral**: Where context is an AI steward deferring a high-stakes decision to human wisdom (Wise Authority ruling).
  - **Moderation Decision**: Where context is a flagged item or safety boundary (`blur`, `warn`, `remove`, `reveal`).
  - **Circle Governance Vote**: Where context is a multi-party quorum motion across the 5 Circles.
- **The First-Run Setup Wizard (`CSD-080`–`CSD-085`, `CSD-093`) is a sequence of 4 single-person `P9` ChoiceBlocks:**
  1. **Step 1 (`YOU` — `setup:identity_path` & `mode:operational_posture`)**:
     - *"How do you want to establish your root identity on this device?"* (`Set up a new CIRIS ID` vs. `Add this device to an existing CIRIS ID`).
     - *"What operational mode should this node initialize in?"* (`Run with AI Steward` [CSD-082] vs. `Run without AI (Bare Node)` [CSD-083]).
     - Modern spatial inputs for Handle and Device Name; 3D selection tiles for Key Custody and Age Band (`CC 3.2`).
  2. **Step 2 (`JOIN_FEDERATION` — `consent:trace_replication`)**: *"Do you want to share anonymized diagnostic traces with the CIRIS federation?"* (`Share anonymized traces` vs. `Keep traces local only`).
  3. **Step 3 (`AI` — `steward:bond:v1`)**: *"Which inference engine should your bonded AI steward use?"* (`Local Ollama (Keyless)` vs. `CIRIS Steward Portal` vs. `Bring Your Own API Key`).
  4. **Step 4 (`COMPLETE` — `setup:claim_remote`)**: *"Are you ready to seal your 0600 claim_pin and claim ownership of this node?"* (`Claim node with 0600 PIN` vs. `Leave Unclaimed`).
- Exactly the same `P9` Question + Selection primitive renders:
  - **Single-person (`1-of-1`) node owner deferrals** (`GET /v1/self/owner-requests`, `POST /v1/self/owner-requests/{id}/decide`);
  - **Wise Authority (`1-of-1` or panel) WBD deferrals** (`CSD-117`);
  - **Multi-device (`2-of-2` + `24h W_self`) self-key roster ceremonies** (`CSD-093`, `CSD-094`, `CSD-095`);
  - **Multi-party (`m-of-n` + silence window) Circle governance ceremonies** (`Decisions` tab across all 5 Circles).

---

## 2. The 24 Closed UX Primitives (`CSD/5` Draft 3 at `v1.0-rc21-wire` & `holon-draw` Profile B)

Per Issue #3 and the lead's decision (2026-11-15), CEWPOS is re-based on **`CSD/5` draft 3** (`manifests/ui_primitives.json` in `CIRISConstitution` at `v1.0-rc21-wire`, vendored at `epistemic-ui/ui_primitives.json` and implemented in `view-forms-spike/src/csd5.rs` over `CIRISHolon`'s `holon-draw` crate): **24 primitives, 60 closed slots, 17 read-bound states, 12 laws (`L1`–`L12`), 4 shell prefixes (`circle`, `tab`, `nav`, `screen`), and the derived tag grammar**.

### 2.1 The 24 `CSD/5` Primitives, Prefixes, and 60 Closed Slots (`csd5.rs` & `ui_primitives.json`)

| # | `CSD/5` Primitive | Owned Prefix(es) (`L1`) | Keyed? | Closed Slots (60 total) | Legacy `P1`–`P12` Mapping & `holon-draw` Profile B Composition |
| :-: | :--- | :--- | :-: | :--- | :--- |
| 1 | **`CardShell`** | `card` | No | `title` | `P1` (1-to-1) — `Kind::Quad` (rounded) + `Kind::Run` title in its own screen tile |
| 2 | **`FieldRow`** | `txt` | No | `wire`, `gloss` | `P2` (1-to-1) — `Kind::Run` label + `Kind::Run` wire value (`"This node did not send this"` when not sent, never blank) |
| 3 | **`ItemRow`** | `row` | Yes | `flag`, `act` | `P3` (1-to-1) — `Kind::Quad` + `Kind::Run`s + `flag` + `act`; shares its handle with its 3D `GraphView` world mark (`twin`) |
| 4 | **`ScopePill`** | `scope` | No | `group` | `P5` (1-to-1) — `Kind::Quad` pill + `Kind::Run` circle & optional `group` |
| 5 | **`Chip`** | `chip` | No | *(none)* | Split from `P6` — Static status token (`Kind::Quad` + `Kind::Glyph` + `Kind::Run`, never color alone) |
| 6 | **`Action`** | `btn`, `mi` | No | `reason` | Added (`btn` on surface, `mi` in menu) — Pinned (`L12`); disabled draws `reason` (`L5`) |
| 7 | **`ReceiptSheet`** | `receipt` | Yes | `open`, `fact_1`..`fact_5`, `holders`, `notes`, `acts` | `P4` (1-to-1) — Modal tile (`__open` replaces `btn_receipt_<id>`) |
| 8 | **`StateBlock`** | `state` | No | `@states` (17), `freshness` | Expanded from `P7` — Bound to a read (`L2`, `L3`, `L9`); draws exactly 1 of the 17 state forms |
| 9 | **`Metric`** | `stat` | No | `unit`, `source` | Added — Bound to a read (`L4`); shows `"—"` (em dash) when not sent or not read, never `0` |
| 10 | **`ConfirmSheet`** | `sheet` | No | `fact_1`, `fact_2`, `fact_3`, `note`, `confirm`, `cancel` | `P8` (1-to-1) — Modal layer tile (`L6`, `L7`, `L12`); takes every pick while open |
| 11 | **`Choice`** | `opt` | Yes | `cost`, `tally` | Split from `P6`/`P9` — One question, each option is a `.key` (`L12`, pinned) |
| 12 | **`DestinationPicker`** | `pick` | Yes | `group` | Split from `P9` — Keyed destination rows with `group` slot |
| 13 | **`Field`** | `input` | No | `ceiling`, `error` | Added — Text input + caret `Kind::Line` + byte `ceiling` (`L12`, pinned) |
| 14 | **`StepFlow`** | `step` | Yes | `next`, `back` | Split from `P9` — Keyed step `Kind::Disc`s + `Kind::Line`s + `next`/`back` (skipped step is absent) |
| 15 | **`Ceremony`** | `ceremony` | Yes | `act_price`, `undo_price`, `window`, `threshold`, `signer` | Split from `P9` — Governance ceremony prices/window/threshold + keyed `__signer.<who>` |
| 16 | **`LineageChain`** | `chain` | Yes | `link`, `cascade` | `P11` (1-to-1) — Keyed `__link.<k>` `Kind::Disc`s + `Kind::Arc`s + `cascade` |
| 17 | **`CodeCompare`** | `code` | No | `words`, `digits`, `verdict` | Split from `P10` — SAS words + digits + verdict glyph |
| 18 | **`QrCode`** | `qr`, `scan` | No | `paste` | Split from `P10` — `qr` displays, `scan` reads into a `Field` and never submits |
| 19 | **`Transcript`** | `msg`, `note` | Yes | `receipt`, `fetch` | Added — Keyed message/note runs in a scrolling tile with `receipt` and `fetch` |
| 20 | **`ConscienceStep`** | `conscience` | Yes | `cost`, `faculty` | Split from `P12` — H3ERE evaluation + `cost` + 4 keyed faculties (CC 3.1.5.3) |
| 21 | **`PolicyLens`** | `lens` | No | `hidden_count`, `helpers`, `depth` | Split from `P12` — Subjective filter bar; hidden items are absent with `hidden_count` stated |
| 22 | **`GraphView`** | `canvas` | No | `twin` | Brought inside the 24 from `SpatialCanvas` — 3D world tiles whose pickable marks share handles with `twin` `ItemRow`s |
| 23 | **`Document`** | `doc` | Yes | `claim`, `recant`, `revision`, `cite` | Added — Keyed claims; a recanted claim is struck with a `Kind::Line` and its reason beside it, never removed |
| 24 | **`Media`** | `media` | Yes | `alt`, `ai_disclosure`, `blur`, `class` | Added — Enforces `L10`: never drawn without non-empty `alt` and `ai_disclosure` |

### 2.2 Tag Grammar (`L1` & `INTERFACE.md` §10.1), 17 Read-Bound States (`L2`, `L3`), and 12 Laws
- **Tag Grammar (`derive_csd5_tag` & `derive_shell_tag` in `view-forms-spike/src/csd5.rs`)**:
  - Primitive tag: `prefix '_' surface '_' instance [ '__' slot ] [ '.' key ]` (**slot comes BEFORE `.key`**, e.g. `row_s_i__flag.k`).
  - Shell tag: `shell_prefix '_' name` (`circle_<name>`, `tab_<name>`, `nav_<name>`, `screen_<name>`).
  - Tags are derived exclusively by the primitives layer and mapped 1-to-1 to `u32` `holon_draw::Instance::tag` handles via `HandleTable`.
- **All 17 Exact Read-Bound States (`Csd5State`)**:
  - `idle` (outcome `idle`); `loading`, `loading_known` (outcome `pending`); `error` (outcome `failed`); and under outcome `ok`: `populated`, `empty`, `refused`, `gone` (replaces `FileGone`), `withheld`, `lapsed`, `ended`, `locked`, `hidden_by_rules`, `unreviewed`, `waiting`, `unknown`, `not_on_this_node`.
- **Directional Asymmetry Law (`L6`: `Inv_ExitOneStep` vs. `ConfirmSheet`)**:
  - **Outward / widening / delegating / irreversible acts** (`confirm: true` in `manifests/transitions.json`) require a 3-fact `ConfirmSheet` (`sheet_<surface>_<instance>__confirm`).
  - **Inward / narrowing / withdrawing / leaving (`Inv_ExitOneStep`) / halting (`423` Stop everything) / refusing (`G17`) acts** execute in **one immediate step** via an `Action` (`btn_<surface>_<instance>`).

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
   - `Cosign`, `Withdraw`, `Supersede`, `Recant`, `Verify`, `History`, `Evidence`, `Details`, `Copy` — gated per `ChangeKind` by `ATTOP_MATRIX` (`algebra_engine.py`) and wired to live `CIRISServer v0.5.245` endpoints (`/v1/chat/{comm}/messages/{id}/{withdraw|supersede|recant}`).
4. **Blob & Referent Custody Acts (`Files` / `Blob` referents)**:
   - `CustodyHolders` (`GET /v1/files/{id}/custody`), `RenameBlob` (`POST /v1/files/{id}/rename`), and `MoveCircle` (`POST /v1/files/{id}/move`).
5. **`CC 4.5.4` Moderation & Content Note Acts**:
   - `FlagContent` (`POST /v1/safety/flag`), `RevealBlurred` (`POST /v1/chat/{comm}/items/{id}/reveal`), and Named Moderator actions (`POST /v1/safety/moderation`: `blur`, `warn`, `remove`).

---

## 4. Full-Stack `ciris-server` (`v0.5.250` on `release/0.5.250`) Crate Consumption & First-Run Entry

CEWPOS consumes [`CIRISServer`](https://github.com/CIRISAI/CIRISServer) directly as a Rust crate at **`branch = "release/0.5.250"`** (`v0.5.250`, bundling `ciris-persist v61.4.2`, `ciris-edge v45.1.7`, and `ciris-verify v22.2.0` in `view-forms-spike/Cargo.toml` and `attestation-calculus-spike/Cargo.toml`).

### 4.1 Starting CEWPOS Directly into First-Run (`T0`)
```bash
# Single-command First-Run entry point (wipes/initializes fresh CIRIS_HOME, boots cewpos-node :4243 & UI :8787 into First-Run Wizard)
python3 start_first_run.py

# Verify First-Run boot, 0600 claim_pin, is_first_run=true, and CSD/5 holon-draw endpoints non-interactively
python3 start_first_run.py --check
```

### 4.2 Building the Full-Stack `cewpos-node` Binary & Running Rust Tests
```bash
# 1. Run the 30 Attestation Calculus tests (with ciris-verify v22.2.0)
cargo test --manifest-path attestation-calculus-spike/Cargo.toml

# 2. Run the 33 View-Forms, CSD/5 (24 Primitives) & holon-draw Profile B tests (GPU-free & wasm-portable)
cargo test --manifest-path view-forms-spike/Cargo.toml --no-default-features

# 3. Build the unified full-stack cewpos-node binary (CIRISServer v0.5.250 + holon-draw CSD/5 Spatial Scene)
cargo build --manifest-path view-forms-spike/Cargo.toml --no-default-features --features fullstack --bin cewpos-node
```

### 4.2 Running the Formal UI Algebra & Live `CSD/4` Conformance Harness
```bash
# Verify the 72-cell (12 ChangeKind x 6 Verb) RC9 matrix, 12x9 AttOp matrix, and 101/101 CSD/4 cards
python3 epistemic-ui/algebra_engine.py

# Run the 11-step live QA conformance harness against CIRISServer v0.5.245 (:4243) and CEWPOS (:8787)
python3 epistemic-ui/csd4_harness.py all
```

### 4.3 Non-Negotiable Engineering Rules for Agents
1. **No Untyped Dicts**: Use explicit structs/enums in Rust and typed schema records in Python/JS.
2. **Full-Screen Bevy Scene Only**: Never reintroduce embedded `<canvas>` widgets inside cards or molecular sphere visualizations (`Z=8 Oxygen`) for user-facing decisions. Keep `#bevySceneCanvas` full-screen (`100vw × 100vh`) and render context as questions (`.p9-question-hero`) and choices as 3D selection slabs (`.p9-selection-tile`).
3. **Preserve `data-csd-tag` Attributes**: Every interactive element and surface block must retain its canonical `CSD/4` tag (`data-csd-tag`) so `/tree` and `csd4_harness.py all` pass `11/11` checks.
