# CEWPOS — CIRIS Epistemic Web Platform OS

> An **immutable, content-addressed object model** + a **deterministic, effect-typed evaluator** +
> **cryptographic provenance** + **federated trust** — the OS layer over a shipping CIRIS substrate.
> The only state is **signed attestations**; rendering is isolated from authority in sandboxed
> WebAssembly; **agents are optional participants the fabric can reject.**
>
> **CIRISServer** (v0.5.58) and **CIRISAgent** (app store) are the **shipping production software** —
> the fabric node and the AI agent. Their current interface is a KMP/Compose client (Android, iOS,
> desktop, in the CIRISServer repo). **CEWPOS is the native-interface rewrite of that client**: a
> native OS layer built on the Attestation Calculus, WASM-sandboxed view-forms, and a Bevy renderer,
> running directly against the substrate crates rather than through a mobile framework.
> Continuously verified by [CIRISConformance](https://github.com/CIRISAI/CIRISConformance) (16,000+
> tests) · CEG wire 1.0-RC29 (1+4 frozen) · AGPL-3.0-or-later · part of the [CIRIS](https://ciris.ai) ecosystem.

**New here?** Start with [`WHY.md`](WHY.md) — a 5-minute orientation (the inversion, the small kernel,
the three layers, and why CEWPOS is *moderation-first*), no prior CIRIS knowledge required.

## What this is

**CIRISServer** and **CIRISAgent** are the existing production deployment — the fabric node and the
AI agent, continuously shipped and conformance-verified. Their current interface is a KMP/Compose
app (Android, iOS, desktop) that lives in the CIRISServer repository.

**CEWPOS is the native-interface rewrite of that client.** It replaces the KMP/Compose layer with a
native OS interface built directly on the substrate crates — no mobile framework in the middle. The
key architectural additions over the KMP client:

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
  zero ambient authority** — a render can compute a scene and nothing else. This replaces the KMP
  Compose UI with typed, sandboxed **view-forms** that output a renderer-neutral Scene IR.
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
┌─────────────────────────────────────────────────────────────────┐
│  CIRISAgent  (shipping · app store · moral-reasoning loop)      │
│  emits gate-checked attestations only — not privileged          │
├─────────────────────────────────────────────────────────────────┤
│  CEWPOS  (this repo · replaces KMP/Compose client)              │
│  Attestation Calculus · WASM view-forms · Bevy renderer         │
├─────────────────────────────────────────────────────────────────┤
│  CIRISServer v0.5.58  (shipping · production fabric node)       │
│  CEG wire 1.0-RC29 (1+4 frozen) · conformance-verified          │
├──────────────┬──────────────┬──────────────┬────────────────────┤
│ciris-persist │ ciris-verify │  ciris-edge  │ ciris-nodecore /   │
│v11.5 · 246k  │ v8.3 · 104k  │ v7.4 · 95k   │ ciris-registry     │
│chunk-DAG     │ hybrid PQC   │ transport +  │ federation         │
│stream-STH    │ Ed25519+     │ streaming AV │ consensus +        │
│append-only   │ ML-DSA-65    │ MLS/TreeKEM  │ authority          │
└──────────────┴──────────────┴──────────────┴────────────────────┘
```

| Layer | Shipping today | Status |
|---|---|---|
| **Substrate crates** | `ciris-persist` v11.5 · `ciris-verify` v8.3 · `ciris-edge` v7.4 | production, conformance-verified |
| **Fabric node** | `ciris-server` v0.5.58 + `ciris-nodecore` + `ciris-registry` | production |
| **Current client** | KMP/Compose — Android, iOS, desktop (in CIRISServer repo) | shipping; CEWPOS replaces this |
| **Agent** | `CIRISAgent` — PDMA/CSDMA/WBD moral-reasoning loop | on the app store; optional participant |
| **CEWPOS** *(this repo)* | Attestation Calculus · WASM view-forms · Bevy renderer · native OS interface | object model complete; evaluator + view-forms spike-proven |

See [`DEPLOYMENT.md`](DEPLOYMENT.md) for bare-metal requirements and node deployment topology.

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
  cascade. Harvest-now-decrypt-later is in the threat model.
- **"Agency-free" means *mechanism-neutral*, not value-free.** The substrate carries no moral agency of
  its own — but admission, delegation, revocation, scoring, and conflict-resolution are still
  **governance decisions**. CEWPOS's claim is narrower and more honest: those decisions are
  **externalized as signed data and adjudicated at an explicit gate**, rather than baked into opaque
  code. *Policy-externalized*, not policy-free.

## Maturity — what ships vs. what's spike-proven

**CIRISServer and CIRISAgent are production software**, not prototypes:

- **Substrate (production):** persist / verify / edge / server at v7–11 (95k–246k LOC each),
  continuously verified by CIRISConformance ([ciris.ai/verification](https://ciris.ai/verification/),
  16k+ tests across 6 projects). `ciris-edge` includes the full streaming-media tier (realtime A/V with
  Opus/AV1 codecs, application-layer multicast, MLS/TreeKEM rekey, content-fetch byte-pull); `ciris-persist`
  the chunk-DAG + stream-STH store. The **CEG wire is 1.0-RC29 with the 1+4 surface frozen.**
- **Current client (shipping):** CIRISServer ships a Kotlin-Multiplatform / Compose client across
  Android, iOS, and desktop. **CIRISAgent is on the app store.**
- **CEWPOS (the native-interface rewrite):** replaces the KMP/Compose client with a native OS layer.
  The **object model is complete and schema-validated** (see below); the **evaluator, view-form, and
  WASM-sandbox pieces are proven by working spikes** against the real substrate crates. The spikes are
  proof-of-concept; the production substrate beneath them is not.

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

- **CIRIS** — [ciris.ai](https://ciris.ai) · the **vision** ([/vision](https://ciris.ai/vision) — "honor the reaching instead of intercepting it") · **continuous verification** ([/verification](https://ciris.ai/verification/), 16k+ tests) · **proof** ([/proof](https://ciris.ai/proof)) · **compliance** ([/compliance](https://ciris.ai/compliance) — the policy layer cross-walked to EU HLEG / IEEE EAD / ASEAN / Magnifica Humanitas) · **comparison / "the other road"** ([/compare](https://ciris.ai/compare))
- **[CIRISConformance](https://github.com/CIRISAI/CIRISConformance)** — the federation-cohabitation + cross-artifact conformance harness; the proof the separately-shipped components enforce the constitution as one system.
- **[CIRISServer](https://cirisai.github.io/CIRISServer)** — the base fabric node + clients; the CEWPOS spec FSDs (`CEWPOS.md`, `HYBRID_LISP_OS.md`, `CEWPOS_RENDERED_OBJECTS.md`) live in its `FSD/`.
- Substrate crates: [CIRISPersist](https://github.com/CIRISAI/CIRISPersist) · [CIRISVerify](https://github.com/CIRISAI/CIRISVerify) · [CIRISEdge](https://github.com/CIRISAI/CIRISEdge) · [CIRISNodeCore](https://github.com/CIRISAI/CIRISNodeCore) · [CIRISRegistry](https://github.com/CIRISAI/CIRISRegistry).

## License

AGPL-3.0-or-later. See [`LICENSE`](LICENSE).
