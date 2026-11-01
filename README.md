# CEWPOS — CIRIS Epistemic Web Platform OS

> **End-to-end, the CIRIS stack forms a complete, running constitutional mesh for people and AI agents.**
>
> Ordinary devices (phones, laptops, servers) form a peer-to-peer network with no platform owner in the
> middle. Every consequential act is a signed, scoped, contestable row. Ethics and the wire format that
> enforces them live in one versioned document. Named humans can pause the AIs that follow their root.
> The whole thing is designed so that the rules cannot quietly drift from the code, and the code cannot
> quietly drift from the formal models.
>
> **CIRISServer** (`v0.5.243`) and **CIRISAgent** (app store) are the **shipping production software** —
> the fabric node and the conscience-bearing AI agent. Their shipping multi-platform interface is
> **[CIRISClient](https://github.com/CIRISAI/CIRISClient)** (Kotlin Multiplatform across Android, iOS,
> desktop, and WASM browser, governed by **101 Client Surface Definitions** in **CSD/3 → CSD/4** and the
> [Circles architecture](https://cirisai.github.io/CIRISClient/circles.html)). **CEWPOS is the native
> OS & semantic-view architecture** over that same substrate: an **Attestation Calculus** evaluator,
> **zero-import WASM view-forms**, and a **full-screen Bevy / `CIRISHolon` 3D renderer (`100vw × 100vh`)**
> driven directly by the **12 closed UX primitives (`P1`–`P12`)** and the formal state and claim algebra of
> **[CIRISConstitution](https://github.com/CIRISAI/CIRISConstitution)**
> (`v1.0-rc8.1` / `rc9` formal closure). Continuously verified by
> **[CIRISConformance](https://github.com/CIRISAI/CIRISConformance)** (17,000+ tests, 219 conformance
> specs) · AGPL-3.0-or-later · part of the [CIRIS](https://ciris.ai) ecosystem.

**New here?** Start with [`WHY.md`](WHY.md) — a 5-minute orientation (the inversion, the small kernel,
the three layers, and why CEWPOS is *moderation-first*), [`AGENTS.md`](AGENTS.md) — the canonical
engineering guide for **full-screen Bevy primitives-based rendering (`P1`–`P12`) and CEWPOS CEG
projection**, and the plain-words
[CIRIS Circles explainer](https://cirisai.github.io/CIRISClient/circles.html) (*"Starts close to you.
Moves out only when you say so. Comes back if anyone in it asks."*).

## The End-to-End CIRIS Stack

CIRIS is a full vertical system that turns a public ethical constitution into a live, post-quantum,
peer-to-peer network of people and conscience-bearing AI agents whose every important action is signed,
scoped, pauseable by named humans, and checkable against both formal models and the screens that display it.

### The layers, bottom to top

| Layer | What it is | Key pieces |
|---|---|---|
| **1. Constitution (CC)** | Single rulebook | [CIRISConstitution](https://github.com/CIRISAI/CIRISConstitution) (`v1.0-rc8.1` tagged · `rc9` formal-closure branch): Ethics (Meta-Goal M-1, PDMA, Recursive Golden Rule, fail-secure) + **CEG wire grammar** (1+4 closed row types, 169 namespace families across 12 change kinds, 10 governance policy families) in one stewarded, versioned AGPL document. |
| **2. Formal models** | Machine-checked witnesses | **14 formal model suites** (`formal/` in CC `rc9`): TLA+ (`trust_root`, `accord_halt`, `mesh_net`, `self_roster`, `retirement`, `consent`, `membership`, `two_path`, `withdraws_admission`, `guarantees`, `cewp_node` spine), Lean 4 (`accord_roster`, `reciprocity`, `influence`, `PolicyCorridor`, `coherence-ratchet`), and Python (`authority_tree`). **157 states** over **51 variables** in **9 planes** (`STATE_TABLE.tsv`), **152 checked invariants/theorems** (`INVARIANTS.tsv`), and the adequacy cascade: *ethics → 11 promises (G1–G28) → rules → invariants → state table → plane models → spine → CSD/4 catalogue → claims register*. |
| **3. Substrate** | Crypto + storage + transport | [CIRISVerify](https://github.com/CIRISAI/CIRISVerify) `v20.2.0` (hybrid Ed25519 + ML-DSA-65 signatures, X25519 + ML-KEM-768 KEX, transparency), [CIRISPersist](https://github.com/CIRISAI/CIRISPersist) `v59.1.0` (signed state, 9-axis `ContextualIntegrity` gate, chunk-DAG + stream-STH, v4 hardware-amended genesis bundle), [CIRISEdge](https://github.com/CIRISAI/CIRISEdge) `v43.0.0` (sealed Reticulum transport, 26 Tier-1 constitutional `MessageType`s + 3 Tier-2 `Opaque*` channels, A/V + MLS/TreeKEM). |
| **4. Fabric node** | The running peer | [CIRISServer](https://github.com/CIRISAI/CIRISServer) `v0.5.243` — the decentralized CEWP node (`ciris_server._native` cohabitation runtime folding `ciris-lens-core`, registry, and nodecore surfaces). Hosts routes, trust-root standing verdicts, households, communities, rooms, drive custody, self-key rosters, moderation & content notes, and the native 20-topology harness (`T0`–`T19`). |
| **5. Agent runtime** | Conscience-bearing AI | **CIRISAgent** — H3ERE pipeline (ethical PDMA / common-sense CSDMA / domain DMAs + conscience gates), Wisdom-Based Deferral (WBD) to a named human Wise Authority, refusal available every round (`G17`), resource-cost accounting (`G27`), signed hash-chained audit trail, and a human halt latch that stops the agent's act gate (`423`) while leaving node data and serving doors open (`Inv_X1_LatchDoesNotBlockServing`). |
| **6. Client surface** | Where a person meets the mesh | [CIRISClient](https://github.com/CIRISAI/CIRISClient) (KMP/Compose across Android, iOS, Desktop, and Browser WASM) + **CEWPOS** (native OS layer: Attestation Calculus + zero-import WASM view-forms + full-screen Bevy / `CIRISHolon` 3D renderer). One runtime-narrowing client that adapts to whether the attached node is bare or has an agent brain folded onto it. |
| **7. CSD/3 → CSD/4** | Surface + topology contracts | **CSD/3** (`CSD.md`): 6-stage lifecycle (`envisioned → shipped`), `ceg:` registry-bound fields, 4 mandatory list states, DSL v2 predicates, and static route closure (`check_csd_routes.py`). **CSD/4** (`FSD/CSD4.md`): binds each card's `csd:tests` block to the closed **20-topology catalogue (`T0`–`T19`, 110 state atoms)** so UI flows, the server harness, and the TLA+/Lean models prove the exact same network shape. |

### What a person actually experiences ([Circles](https://cirisai.github.io/CIRISClient/circles.html))

1. **Attach to a node you own or trust** (phone, desktop, or browser WASM). Whether the node is bare or has an AI agent attached, the same client adapts at runtime (`hasAgent`).
2. **Five concentric Circles (`CohortScope` 7→5 fold), closest to you first**: **Just me** (`self`), **Family** (`family`), **Neighbours** (`community`), **Communities & Businesses** (`affiliations`), and **Everyone** (`species` / `biosphere` / `federation`). Every circle has the same **7 Tabs** (*Files · Chats · People · Safety · Rules · Decisions · Record*), while **My things** (top-left signet: *Devices & keys, Everything I shared, This agent, This node, Someone I trust, Help*) and **Stop everything** (top-right emergency brake) sit permanently above the scroll.
3. **Five facts travel with every signed item (`CSD-006` Receipt)**: *Who it is about* (`subject_key_ids`), *Who sent it* (`attesting_key_id`), *Who can see it* (`cohort_scope`), *What it is* (`dimension`), and *The rule it follows* (`consent:scope`). If a row on screen has a receipt hamburger, it is a signed CEG claim off the wire; if it has none, it is app chrome.
4. **Starts close to you; moves out only when you say so; comes back if anyone in it asks**: New items start in *Just me*. Sharing outward always asks a 3-fact confirmation (`ConfirmSheet`); pulling inward, leaving a root (`Inv_ExitOneStep`), revoking consent, or withdrawing a claim where you are the producer or a named subject takes one step and never asks permission. Private circles are structurally invisible (`Inv_Invisible`).
5. **One person can pull the brake; it takes a group to let go of it**: Under the formal two-path and mirror-brake policies (`governance_policy.json`), a single voice can flag harm or pause a subscribed agent (1-of-N Accord halt with a 24h fuse), while enacting or lifting requires a standing quorum above an approval floor (`formal/two_path`).

## What CEWPOS is in this stack

**CIRISServer**, **CIRISAgent**, and **CIRISClient** are the shipping production deployment. **CEWPOS** explores and proves the **native semantic-view OS architecture** beneath and ahead of the client layer:

- **One immutable object model — CEG** (the CIRIS Epistemic Grammar). Every meaningful thing —
  identity, config, consent, a moderation action, a payment claim, a media stream — is a **signed
  attestation**, content-addressed and canonical.
- **A small trusted core.** Semantics are a **closed set of five structural operators** (`scores` +
  `delegates_to` / `supersedes` / `withdraws` / `recants`). The system grows by **vocabulary, not new
  primitives** — the same discipline behind Git's object model, RDF, content-addressed storage,
  capability systems, and Lisp homoiconicity. Keeping the evaluator small keeps it auditable.
- **A deterministic evaluator** (the *Attestation Calculus*) rather than an open-ended runtime. It is
  total and effect-typed; its **only effect is a gate-checked `emit`**.
- **Rendering isolated from authority.** Views are pure functions compiled to **WASM components with
  zero ambient authority** — a render can compute a scene and nothing else. Each CEG object or plane state
  maps through a deterministic **view-form** to a renderer-neutral **Scene IR**, which a host renderer (Bevy GPU, WebGPU, or Compose primitive tree) turns into pixels.
- **Agents are not privileged.** The model emits attestations *into* a fabric that admits or rejects
  them; the platform never has to "trust" the model.

## The problem it addresses

Most "AI operating system" proposals build a large autonomous runtime you must trust. CEWPOS inverts
that: instead of

```
AI → controls system
```

it is

```
attestation fabric ← AI emits attestations (admitted or rejected at a gate)
```

The goal is to make **every meaningful state transition externally auditable** — *verify the
constraint*, don't *trust the model*. A signature you cannot forge, a membership you cannot fake, an
audit you cannot escape. In one line: **make truthfulness architecturally cheaper than deception** —
maintaining a consistent lie across many independent, signed, externally-verifiable constraints costs
more than telling the truth.

## Architecture

```
┌────────────────────────────────────────────────────────────────────────────┐
│  CSD/3 → CSD/4 Surface & Topology Contracts (101 cards · T0–T19 catalogue) │
│  Every surface bound to CEG families, 4 list states, routes & state atoms  │
├─────────────────────────────────────┬──────────────────────────────────────┤
│  CIRISClient (shipping KMP/Compose) │  CEWPOS (this repo · native/WASM)    │
│  5 Circles · 7 Tabs · 6 Instruments │  Attestation Calculus · zero-import  │
│  9 UI Primitives · 29 MQM locales   │  12 Primitives · Full-Screen Bevy 3D │
├─────────────────────────────────────┴──────────────────────────────────────┤
│  CIRISAgent (shipping · app store · H3ERE conscience + WBD + Refusal G17)  │
│  emits gate-checked attestations only — pauseable via Accord halt latch    │
├────────────────────────────────────────────────────────────────────────────┤
│  CIRISServer v0.5.243 (shipping · decentralized CEWP fabric node)          │
│  ciris_server._native · lens-core + registry/node folded · T0–T19 harness  │
├──────────────┬──────────────┬──────────────┬───────────────────────────────┤
│ciris-persist │ ciris-verify │  ciris-edge  │ CIRISConstitution v1.0-rc8.1  │
│v59.1 · 750k  │ v20.2 · 125k │ v43.0 · 335k │ + rc9 Formal Closure          │
│chunk-DAG +   │ hybrid PQC   │ sealed RNS   │ 14 TLA+/Lean/Py models        │
│9-axis CI +   │ Ed25519+     │ transit-kex  │ 157 states · 51 vars · 9 plns │
│v4 genesis    │ ML-DSA-65    │ AV + MLS     │ 152 invariants · 110 atoms    │
└──────────────┴──────────────┴──────────────┴───────────────────────────────┘
```

| Layer | Shipping today | Status |
|---|---|---|
| **Constitution & Formal Closure** | `CIRISConstitution` `v1.0-rc8.1` (`rc9` branch: 14 formal model suites, 157 states / 51 vars / 9 planes, 152 checked properties, 169 CEG families, 10 governance policies) | normative & machine-checked in CI (`TLC` + `Lean 4` + `authority_tree.py`) |
| **Substrate crates** | `ciris-persist` v59.1 · `ciris-verify` v20.2 · `ciris-edge` v43.0 | production, conformance-verified (~1.21M LOC Rust across the 3 crates) |
| **Fabric node** | `ciris-server` v0.5.243 (`ciris_server._native` cohabitation wheel + folded `ciris-lens-core`, registry, nodecore & `T0`–`T19` native harness) | production (~270k LOC Rust + ~80k LOC Python), consumed directly as a crate (`cewpos-node`) |
| **Agent runtime** | `CIRISAgent` — H3ERE (PDMA/CSDMA/DSDMA + conscience gates), WBD deferral, human halt gate | on the app store; optional participant |
| **Shipping client** | [`CIRISClient`](https://github.com/CIRISAI/CIRISClient) — KMP/Compose (Android, iOS, Desktop, WASM browser), 101 CSD/3 & CSD/4 cards, 29 locales | shipping across 5 live-QA platforms |
| **CEWPOS** *(this repo)* | Attestation Calculus · zero-import WASM view-forms · 12 closed UX primitives (`P1`–`P12`) · Full-Screen Bevy / `CIRISHolon` 3D Scene | object model complete; full-stack `cewpos-node` (`ciris-server v0.5.243`), evaluator + view-forms + 11/11 live `CSD/4` harness verified (see [`AGENTS.md`](AGENTS.md) & [`UI_FORMAL_PLAN.md`](UI_FORMAL_PLAN.md)) |

See [`DEPLOYMENT.md`](DEPLOYMENT.md) for bare-metal requirements and node deployment topology, and
[`UI_FORMAL_PLAN.md`](UI_FORMAL_PLAN.md) for the formal evaluation of the UI against CSD/3 → CSD/4,
the [Circles explainer](https://cirisai.github.io/CIRISClient/circles.html), and the `rc9` formal
closure (`Claims × Contexts × Actions × Conclusions`).

## The data model — CEG

CEG is, structurally, a **homoiconic, immutable, content-addressed Lisp** — it just needed an
evaluator. Its surface is JSON; its canonical form is **JCS (RFC 8785)**; its identity is the
**SHA-256 of the canonical bytes**; its authenticity is a **hybrid Ed25519 + ML-DSA-65** signature
kept for posterity. Truth is the **append-only signed history** — you `supersedes`/`withdraws`/
`recants`, never mutate. That buys deterministic replay, complete provenance, easy distributed sync,
and cryptographic audit trails.

## The evaluator — the Attestation Calculus

Not a mutable Lisp-Machine image (which collides with every CEG invariant), but **CEG-as-the-canonical-
Lisp + a disciplined, total, effect-typed evaluator**:

- the reader admits only the **closed 1+4** special forms;
- pure computation is free; the **single effect is `emit`**, which must clear the **four-test
  admission gate** (rules hash-pinned · mechanism-not-quality · re-checkable · never sole evidence);
- a **new domain is new vocabulary** (a dimension / `subject_kind` / composition policy), *never* a
  new primitive.

Design rationale: [`CEWPOS_ATTESTATION_CALCULUS.md`](CEWPOS_ATTESTATION_CALCULUS.md).

## Example — an attestation, evaluated and rendered

A `scores` attestation reads as an s-expression and prints back **byte-identically** under JCS:

```
(scores :dimension "evaluation:quality" :score 0.85 :confidence 0.92 :cohort_scope "self")
```

A pure, total **view-form** maps it to a renderer-neutral Scene IR (here: score `0.85` → green;
confidence `0.92` → scale), which a Bevy renderer turns into pixels —
[`view-forms-spike/out/attestation-render.png`](view-forms-spike/out/attestation-render.png), rendered
headless on an Apple-M4 GPU and also built to `wasm32` for the browser. Same attestation in → same
scene out, on any platform.

## View-forms — the lineage

The semantic-view model returns to a principle as old as mainframes:

| Era | Model | Who owns the application? |
|---|---|---|
| 3270 / green screen | terminal renders host-described fields | host |
| HTML forms | browser renders server-sent markup | shared |
| React SPA | client builds the whole application | client |
| Agent-generated UI (A2UI) | LLM emits declarative JSON; host renders trusted components | host, again |
| **CEWPOS semantic views** | WASM view-form emits Scene IR; trusted renderer presents | host — and verifiable |

Google's A2UI makes the return of host authority explicit: *"safe like data, expressive like code."* The direction is the same as CEWPOS; the implementation differs. A2UI relies on LLM output and a trusted-component catalog; CEWPOS view-forms are **deterministic, content-addressed WASM functions** — signed, replayable, and capability-sandboxed (zero imports; a capability-demanding view rejected at instantiation).

The deeper analogy is Smalltalk: every **object** carries an **inspector**, not every page a component tree. You don't build screens that assemble an object graph — you discover the view-form for an object and render its inspector. Here, the inspector is portable, signed, GPU-accelerated, and incapable of reaching outside the scene.

> **Green screens weren't obsolete — they were incomplete.** WebAssembly and modern GPUs let us keep the deterministic, host-authoritative model while replacing fixed terminal layouts with secure, programmable, three-dimensional object views.

What CEWPOS adds to that lineage: **immutable semantic objects** (CEG) + **object-specific renderers** (WASM view-forms) + **capability isolation** (zero-import component model) + **deterministic scene graphs** (Scene IR) + **cryptographic identity** (content-addressed, hybrid-signed). The pieces all exist independently; the combination is new.

## Federation & trust

State is **agent-centric** (no global ledger). Config and identity are CEG attestations at a **node
tier** (the node's own state) or an **owner-self tier** (the human owner's identity/consent), composed
hierarchically. A write is cheap and local; **promotion to federation** is the one moment the hybrid
post-quantum signature is paid. **Structural invisibility** means self/family content never even emits
a discovery attestation — it can't be found, not merely "access-controlled." The full path —
conformant Engine emit → distinct-peer shared-substrate federation → a promoted `SignedAttestation`
crossing a **real `ciris-edge` wire** — is wired end-to-end in [`attestation-calculus-spike/`](attestation-calculus-spike/).

## Security

- **Capability-isolated rendering.** A view-form is a **WASM component instantiated with zero imports**
  — no kernel, substrate, network, or clock. A malicious view-form that *demands* a host capability is
  **rejected at instantiation** (proven in [`view-form-component/`](view-form-component/)).
- **Post-quantum throughout** — hybrid Ed25519 + ML-DSA-65 signatures; X25519 + ML-KEM-768 content
  cascade (`key-grant`) **and** per-link sealed Reticulum transport (`transit-kex`, every RNS frame
  sealed under a hybrid transit session in `ciris-edge` v43.0 + genesis bundle v4). Harvest-now-decrypt-later
  is in the threat model.
- **"Agency-free" means *mechanism-neutral*, not value-free.** The substrate carries no moral agency of
  its own — but admission, delegation, revocation, scoring, and conflict-resolution are still
  **governance decisions**. CEWPOS's claim is narrower and more honest: those decisions are
  **externalized as signed data and adjudicated at an explicit gate**, rather than baked into opaque
  code. *Policy-externalized*, not policy-free.

## Formal UI Algebra — Claims × Contexts × Actions × Conclusions (`CC rc9`)

The [`rc9` formal-closure branch of `CIRISConstitution`](https://github.com/CIRISAI/CIRISConstitution/tree/rc9)
proves that the entire UI surface of a constitutional mesh node factors into a finite, closed
4-tuple $(\mathcal{C}, \mathcal{X}, \mathcal{A}, \mathcal{V})$:

1. **Claims ($\mathcal{C}$ — What can be said):**
   - **1+4 closed CEG row operators** (`scores` + `delegates_to` / `supersedes` / `withdraws` / `recants`, plus derived `carries`).
   - **169 namespace families** (`manifests/namespace_registry.json`) partitioned across **12 change kinds** (`Priorities`, `Rules`, `Manner`, `Identity`, `Confidence`, `Facts`, `Circumstances`, `Process`, `Model`, `Structure`, `Premises`, `Record`) and **3 ontological categories** (`THING`: 78, `ACCOUNT`: 76, `RECORD`: 15), occupying **48 of 72 `(Kind × Verb)` cells**.
   - **5 universal envelope facts** (`CSD-006` Receipt: `subject_key_ids`, `attesting_key_id`, `cohort_scope`, `dimension`, `consent:scope` + optional `for_key_id`), each in one of 3 epistemic provenance states: `Fact.Wire` (sent by node), `Fact.ByRule(cc_ref)` (constitutionally fixed), or `Fact.NotSent` (error tone, never guessed).
2. **Contexts ($\mathcal{X}$ — Where and under what standing a claim lives):**
   - **5 concentric Circles** (the 7→5 fold of `CohortScope`: *Just me · Family · Neighbours · Communities & Businesses · Everyone*) × **7 Tabs** (*Files · Chats · People · Safety · Rules · Decisions · Record*), governed by the **9-axis `ContextualIntegrity` gate** in `ciris-persist`.
   - **6 non-circle Instruments** (*My things*: `devices-keys`, `everything-i-shared`, `this-agent`, `this-node`, `someone-i-trust`, `help`) + **2 top-bar invariants** (*My things* signet top-left; *Stop everything* top-right).
   - **Normalized Mesh State Vector** (`formal/STATE_TABLE.tsv`: **157 states over 51 variables across 9 planes** — `node`, `root`, `self`, `membership`, `consent`, `content`, `authority`, `agent`, `governance`) and the **closed 20-topology CSD/4 catalogue (`T0`–`T19`, 110 state atoms)**.
3. **Actions ($\mathcal{A}$ — Every state transition available to a principal):**
   - **Directional asymmetry law:** Outward / widening / delegating / irreversible acts always require a **3-fact `ConfirmSheet`**; inward / narrowing / withdrawing / leaving (`Inv_ExitOneStep`) / halting (`Prop_HaltReach`) / refusing (`Inv_RefuseAvailable`) acts execute in **one immediate step**.
   - **10 formal governance decision families** (`manifests/governance_policy.json`: `default_content`, `default_standing`, `default_irreversible`, `community_invite`, `community_remove`, `self_roster_add`, `self_roster_evict`, `moderation_action`, `accord_halt`, `custodian_replace`) governed by `TwoPath.tla` and `PolicyCorridor.lean`.
4. **Conclusions ($\mathcal{V}$ — The terminal verdicts the UI renders):**
   - Closed formal verdicts per plane: `Rooted | Stalled | NotRooted` (`TrustRootVerdict.tla`), `lands(forward | reverse) | pending | cancelled` (`TwoPath.tla`), `clear | latched_lone | confirmed` (`AccordHaltFuse.tla`), `admitted | withdrawn | recanted | superseded | tombstoned` (`WithdrawsAdmission.tla`), `7-step Retirement ladder` (`Retirement.tla`), and `7 surface states` (`Populated | Loading | Empty | Error | FileGone | HiddenByRules | Unreviewed`).

From this 4-tuple, **every screen in the 80+ CSD catalog** can be constructed from **12 Minimum Viable Reusable Primitives** (the 9 existing `CIRISClient` primitives + 3 formal closure primitives: `LineageChainBlock`, `ConscienceStepBlock`, and `PolicyLensBar`) and projected via a typed **Epistemic Scene IR** inside zero-import WASM view-forms. See **[`UI_FORMAL_PLAN.md`](UI_FORMAL_PLAN.md)** for the full evaluation and formal UI plan.

## Maturity — what ships vs. what's spike-proven

**CIRISServer, CIRISAgent, and CIRISClient are production software**, not prototypes:

- **Constitution & Formal Models (`v1.0-rc8.1` / `rc9`):** `CIRISConstitution` holds the frozen 1+4 CEG grammar, 169 namespace families, 359+ established register claims, and 14 formal model suites (`TLA+`, `Lean 4`, `Python`) verifying 152 properties across 157 normalized node states in CI.
- **Substrate (production):** `ciris-persist` v59.1 (~750k LOC Rust) / `ciris-verify` v20.2 (~125k LOC Rust) /
  `ciris-edge` v43.0 (~335k LOC Rust) / `ciris-server` v0.5.232 (~270k LOC Rust + ~80k LOC Python),
  continuously verified by CIRISConformance ([ciris.ai/verification](https://ciris.ai/verification/),
  17k+ tests and 219 conformance specifications across the stack). `ciris-edge` includes the sealed
  Reticulum transport (hybrid X25519 + ML-KEM-768 link KEX), 26 Tier-1 constitutional `MessageType`s +
  3 Tier-2 `Opaque*` channels, 15 signed `EnvelopeKind`s, and the full streaming-media tier (realtime
  A/V with Opus/AV1 codecs, per-hop transit keys, application-layer multicast, MLS/TreeKEM rekey,
  fountain blob swarm); `ciris-persist` provides the 3-backend (`memory`/`sqlite`/`postgres`) store,
  typed `EnvelopeCore`/`RowMirror` binding, 9-axis `ContextualIntegrity` mesh crossing (`enter_mesh` /
  `widen_audience`), chunk-DAG + stream-STH store, and the v4 hardware-amended genesis bundle.
- **Current client (shipping):** [`CIRISClient`](https://github.com/CIRISAI/CIRISClient) ships a
  runtime-narrowing Kotlin Multiplatform / Compose client across Android, iOS, Desktop, and WASM
  browser, backed by 80+ CSD/3 & CSD/4 cards, 29 MQM-evaluated locales, and a 5-platform live-QA
  harness. **CIRISAgent is on the app store.**
- **CEWPOS (the native semantic-view layer):** replaces bespoke screen trees with a native OS layer
  driven by the Attestation Calculus and zero-import WASM view-forms. The **object model is complete
  and schema-validated** (see below); the **evaluator, view-form, and WASM-sandbox pieces are proven
  by working spikes** against the real substrate crates.

## The object model

[`object-model/`](object-model/) maps the agency-free fabric as one queryable, schema-validated dataset
— **361 objects · 247 transformations · 132 knobs · 8 capabilities · 476 civilizational functions** —
in [`cewpos-object-model.json`](object-model/cewpos-object-model.json) (conforming to
[`schema.json`](object-model/schema.json)), with three prose maps (the Constitution algebra, the repo
knob/capability surface, the civilizational surface), the [Object Model FSD](object-model/CEWPOS_OBJECT_MODEL.md),
the adversarial [gap analysis](object-model/gap-analysis.md), and a CC-amendment proposal
([fair exchange is mostly in-grammar](object-model/CC1.7-fair-exchange.md)).

Coverage of the civilizational surface: **~82% in-grammar, ~95% expressible** (in-grammar or honestly
bridged to an external rail). An adversarial re-test against the Constitution found the "principled
gaps" residue is mostly **clean unbuilt vocabulary builds** (each a new `subject_kind`/composition via
the amendment process, no new primitive), with only a handful of genuine refusals — an honest roadmap,
not a wall.

## What's proven (real toolchains, on disk)

- **Homoiconicity** — JCS round-trip is byte-identical; the reader admits only the 1+4; `emit → promote`
  is gate-checked with **real** hybrid Ed25519 + ML-DSA-65 signing and verification.
- **The real trio** — conformant `ciris-persist` emit · distinct-peer shared-substrate federation · a
  promoted `SignedAttestation` over a **real `ciris-edge` wire**.
- **Typed view-forms = 3D renderings** — a CEG attestation → pure `view_form()` → Scene IR → Bevy GPU
  render → PNG, and a `wasm32` browser build.
- **Safe by construction** — a zero-import WASM component; a capability-demanding view rejected at
  instantiation.

## Why it's shaped this way — the bigger picture

*(Motivation, after the engineering.)* CEWPOS takes the **institutional / accountability** road in AI
safety — [the "other road"](https://ciris.ai/compare) — rather than aligning one model from the inside.
The bet: safety through advanced AI comes from a **federation of externally-verifiable, accountable
agents**, and from **structurally preventing the unaccountable singleton**. Read that way, the
"everything-app OS / civilizational substrate" framing is not a manifesto flourish — it's the *body*
that bet requires: the same fabric that carries a society's identity, data, money, governance, and media
is exactly what makes every agent operating inside it accountable. That is the long-range thesis; the
engineering above stands on its own without it.

## The hard part — vocabulary governance

The biggest risk is **not the calculus**; it's **vocabulary explosion.** "New domains require vocabulary,
not primitives" is elegant, but someone must then govern namespaces, interoperability, ontology
evolution, conflicting vocabularies, and semantic drift — the work of a standards body. The architecture
is *prepared* for this (the four-test admission gate + the CC 4.5.1 amendment process + an adversarially-
certified Constitution), and it is exactly where CIRIS concentrates its effort. But the governance
process — *legitimacy, inclusion, adoption* — is the real open frontier, not the architecture. The repo
proves the fabric; it does not claim to have solved the human layer it deliberately defers to.

## Ecosystem & references

- **CIRIS** — [ciris.ai](https://ciris.ai) · the **vision** ([/vision](https://ciris.ai/vision) — "honor the reaching instead of intercepting it") · **continuous verification** ([/verification](https://ciris.ai/verification/), 17k+ tests, 219 conformance specs) · **proof** ([/proof](https://ciris.ai/proof), 359+ established constitutional claims) · **compliance** ([/compliance](https://ciris.ai/compliance) — the policy layer cross-walked to EU HLEG / IEEE EAD / ASEAN / Magnifica Humanitas) · **comparison / "the other road"** ([/compare](https://ciris.ai/compare)) · **Circles explainer** ([cirisai.github.io/CIRISClient/circles.html](https://cirisai.github.io/CIRISClient/circles.html))
- **[CIRISConstitution](https://github.com/CIRISAI/CIRISConstitution)** (`v1.0-rc8.1` · `rc9` formal-closure branch) — canonical source of the CIRIS Constitution (CC) and CEG wire grammar, with 14 TLA+/Lean 4/Python formal model suites (`formal/`), `STATE_TABLE.tsv` (157 states over 51 variables in 9 planes), `INVARIANTS.tsv` (152 checked properties), `namespace_registry.json` (169 claim families), `governance_policy.json` (10 governance decision shapes), `topology_vocabulary.json` (110 CSD/4 atoms), and `evidence_pins.tsv`.
- **[CIRISClient](https://github.com/CIRISAI/CIRISClient)** — unified KMP/Compose client (Android, iOS, Desktop, WASM browser), the CSD/3 & CSD/4 catalog (`FSD/CSD/`), 9 UI primitives (`ui/primitives/`), 29-language MQM localization pipeline, and 5-platform live-QA harness.
- **[CIRISConformance](https://github.com/CIRISAI/CIRISConformance)** — the federation-cohabitation + cross-artifact conformance harness; the proof the separately-shipped components enforce the constitution as one system.
- **[CIRISServer](https://github.com/CIRISAI/CIRISServer)** (`v0.5.232`) — the base fabric node (`ciris_server._native` cohabitation runtime folding `ciris-lens-core`, registry, and nodecore surfaces over the shared substrate, plus the `T0`–`T19` native topology harness).
- Substrate crates: [CIRISPersist](https://github.com/CIRISAI/CIRISPersist) (`v59.1.0`) · [CIRISVerify](https://github.com/CIRISAI/CIRISVerify) (`v20.2.0`) · [CIRISEdge](https://github.com/CIRISAI/CIRISEdge) (`v43.0.0`) · folded pre-fold archives: [CIRISRegistry](https://github.com/CIRISAI/CIRISRegistry) (`v4.0.0`) · [CIRISNodeCore](https://github.com/CIRISAI/CIRISNodeCore).

## License

AGPL-3.0-or-later. See [`LICENSE`](LICENSE).
