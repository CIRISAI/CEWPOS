# CEWPOS — CIRIS Epistemic Web Platform OS

> An **immutable, content-addressed object model** + a **deterministic, effect-typed evaluator** +
> **cryptographic provenance** + **federated trust** — the OS layer over a shipping CIRIS substrate.
> The only state is **signed attestations**; rendering is isolated from authority in sandboxed
> WebAssembly; **agents are optional participants the fabric can reject.**
>
> **Status:** the CIRIS substrate is **shipping** — `ciris-persist` v11.5 · `ciris-verify` v8.3 ·
> `ciris-edge` v7.4 · `ciris-server` v0.5.58, continuously verified by
> [CIRISConformance](https://github.com/CIRISAI/CIRISConformance) (16,000+ tests), with the agent on
> the app store and the **CEG wire at 1.0-RC29 (the 1+4 surface frozen)**. *This repo* is the OS layer
> over that substrate: the object model is complete and schema-validated, and the new
> evaluator / view-form / sandbox pieces are proven by working spikes against the real crates.
> AGPL-3.0-or-later · part of the [CIRIS](https://ciris.ai) ecosystem.

**New here?** Start with [`WHY.md`](WHY.md) — a 5-minute orientation (the inversion, the small kernel,
the three layers, and why CEWPOS is *moderation-first*), no prior CIRIS knowledge required.

## What this is

A small, auditable layer that turns the existing CIRIS data fabric into an operating environment:

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
  zero ambient authority** — a render can compute a scene and nothing else.
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
audit you cannot escape.

## Architecture

| Layer | Components | State |
|---|---|---|
| **Substrate** (shipping) | `ciris-persist` (the signed corpus / storage) · `ciris-verify` (hybrid Ed25519 + ML-DSA-65 crypto, identity, license) · `ciris-edge` (Reticulum/HTTP transport + streaming media) · `ciris-nodecore` (federation consensus) · `ciris-registry` (authority) · `ciris-server` (the fabric node + KMP/Compose clients) | production, conformance-verified |
| **Calculus** (this repo) | the deterministic evaluator · the typed view-form / Scene-IR layer · the WASM-component sandbox | object model complete; new pieces spike-proven |
| **Agent** (shipping, optional) | `CIRISAgent` — the moral-reasoning loop (PDMA/CSDMA/WBD) | on the app store; plugs in *last*, as one transformation whose only effect is a gate-checked `emit` |

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

This is **not a greenfield runtime**. It is a thin, auditable OS layer over a fabric that already ships:

- **Substrate (production):** persist / verify / edge / server at v7–11 (95k–246k LOC each),
  continuously verified by CIRISConformance ([ciris.ai/verification](https://ciris.ai/verification/),
  16k+ tests across 6 projects). `ciris-edge` includes the full streaming-media tier (realtime A/V with
  Opus/AV1 codecs, application-layer multicast, MLS/TreeKEM rekey, content-fetch byte-pull); `ciris-persist`
  the chunk-DAG + stream-STH store. The **CEG wire is 1.0-RC29 with the 1+4 surface frozen.**
- **Clients (shipping):** CIRISServer ships a Kotlin-Multiplatform / Compose client across Android, iOS,
  and desktop; **CIRISAgent is on the app store.**
- **This repo (the OS layer):** the **object model is complete and schema-validated** (see below); the
  **evaluator, view-form, and WASM-sandbox pieces are proven by working spikes** against the real
  crates. The spikes are proof-of-concept; the substrate beneath them is not.

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

- **CIRIS** — [ciris.ai](https://ciris.ai) · **continuous verification** — [ciris.ai/verification](https://ciris.ai/verification/) · **comparison / "the other road"** — [ciris.ai/compare](https://ciris.ai/compare)
- **[CIRISConformance](https://github.com/CIRISAI/CIRISConformance)** — the federation-cohabitation + cross-artifact conformance harness; the proof the separately-shipped components enforce the constitution as one system.
- **[CIRISServer](https://cirisai.github.io/CIRISServer)** — the base fabric node + clients; the CEWPOS spec FSDs (`CEWPOS.md`, `HYBRID_LISP_OS.md`, `CEWPOS_RENDERED_OBJECTS.md`) live in its `FSD/`.
- Substrate crates: [CIRISPersist](https://github.com/CIRISAI/CIRISPersist) · [CIRISVerify](https://github.com/CIRISAI/CIRISVerify) · [CIRISEdge](https://github.com/CIRISAI/CIRISEdge) · [CIRISNodeCore](https://github.com/CIRISAI/CIRISNodeCore) · [CIRISRegistry](https://github.com/CIRISAI/CIRISRegistry).

## License

AGPL-3.0-or-later. See [`LICENSE`](LICENSE).
