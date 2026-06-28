# CEWPOS — CIRIS Epistemic Web Platform OS

> The **agency-free fabric** for an everything-app operating system: a civilizational substrate
> where identity, data, trust, governance, money, media, and computation are native **CEG
> attestations** on a unified, content-addressed, post-quantum-signed corpus — evaluated by a
> deterministic Lisp, surfaced as **typed 3D view-forms**, deployable to the browser end-to-end
> in WebAssembly.
>
> **Status:** research + specification + working proof-of-concept spikes. Part of the
> [CIRIS](https://ciris.ai) ecosystem. AGPL-3.0-or-later.

## What it is, in one line

**CEWPOS is superalignment infrastructure in the shape of a civilizational operating system —
where the OS *is* the alignment strategy.**

It takes the **institutional / accountability** road to the superalignment problem, *not* the
singleton road. It does not try to align one god-model from the inside (RLHF, Constitutional-AI,
interpretability — the [road CIRIS explicitly rejects](https://ciris.ai/compare)). It bets that
safety through advanced AI comes from a **federation of externally-verifiable, accountable agents**
— and from **structurally preventing the unaccountable singleton** ("ρ→1 single-voice collapse") —
and it realizes that bet as the **everything-app OS the federation runs on**. The same fabric that
carries civilization's identity, data, money, governance, and media is exactly what makes every
agent operating inside it accountable. The civilizational OS is not a side-effect of the alignment
goal; it is the *body* that goal requires. A signature you cannot forge, a membership you cannot
fake, an audit you cannot escape — external, independently-verifiable constraints, not a model's
self-report.

## The thesis

CEG (the CIRIS Epistemic Grammar) already **is** a homoiconic, immutable, content-addressed Lisp —
it just needed an evaluator. CEWPOS gives it one, the **Attestation Calculus**:

- a **closed set of 5 structural operators** (`scores` + `delegates_to`/`supersedes`/`withdraws`/`recants`)
  over an **open namespace** (dimensions, subject_kinds, composition policies);
- **a new domain is new *vocabulary*, not a new primitive** — land title, a vaccination record, and a
  group chat are the same grammar wearing different prefixes;
- the fabric is **agency-free** — mechanism, not moral judgment (the admission gate is "mechanism,
  not quality"). The agent (CIRIS's PDMA observation→decision loop) plugs in **last**, as one
  deferred transformation whose only effect is a gate-checked `emit`;
- truth is the **append-only signed history** (supersession, never mutation), hybrid Ed25519 +
  ML-DSA-65 signed for posterity.

This is the "third path" that resolves the Lisp × CEG question: not a mutable Lisp-Machine image
(which collides with every CEG invariant), but **CEG-as-the-canonical-Lisp + a disciplined,
total, effect-typed evaluator**. Design rationale: [`CEWPOS_ATTESTATION_CALCULUS.md`](CEWPOS_ATTESTATION_CALCULUS.md).

## What's here

| Path | What it is |
|---|---|
| [`CEWPOS_ATTESTATION_CALCULUS.md`](CEWPOS_ATTESTATION_CALCULUS.md) | The design memo — the Lisp×CEG verdict and the third path. |
| [`object-model/`](object-model/) | **The agency-free fabric, mapped.** `schema.json` (the unified object-model schema) + three prose maps (Constitution algebra, the repo knob/capability surface, the civilizational surface) + the schema-conformant JSON dataset (`cewpos-object-model.json`) + the Object Model FSD. |
| [`attestation-calculus-spike/`](attestation-calculus-spike/) | The calculus + the **real substrate trio** (`ciris-persist` / `ciris-verify` / `ciris-edge`) wired end-to-end behind feature flags, following the cohabitation patterns proven by [CIRISConformance](https://github.com/CIRISAI/CIRISConformance). |
| [`view-forms-spike/`](view-forms-spike/) | A CEG `scores` attestation **rendered to pixels by Bevy** (`out/attestation-render.png`); also builds to `wasm32`. |
| [`view-form-component/`](view-form-component/) | A view-form as a **sandboxed WASM Component** (zero ambient authority — an agent-generated view can only return a scene). |
| `wasm-component-toolchain-spike/` | The verified known-good WASM Component Model toolchain. |

## What's proven (real toolchains, on disk)

- **Homoiconicity** — a CEG envelope reads to an s-expr and prints back byte-identically under JCS
  (RFC 8785); **closed operators** (the reader admits only the 1+4); **gate-checked emit → promote**
  with **real** hybrid Ed25519 + ML-DSA-65 signing and verification.
- **The real trio** — conformant `ciris-persist` Engine emit, distinct-peer shared-substrate
  federation, and a promoted `SignedAttestation` crossing a **real `ciris-edge` wire** A→B.
- **Typed view-forms = 3D renderings** — a `scores` attestation → pure `view_form()` → typed Scene
  IR → **Bevy GPU render** (Apple M4 Metal) → PNG; and a successful `wasm32-unknown-unknown` build
  (browser end-to-end).
- **Safe by construction** — a view-form is a **WASM Component instantiated with zero imports**; a
  malicious view-form that *demands* a host capability is **rejected at instantiation**.
- **Exhaustiveness** — of 550 civilizational functions, **~82% are in-grammar and ~95% expressible**
  (in-grammar or honestly bridged). An adversarial re-test of the residue against CC 0.6 + MISSION
  ([`object-model/gap-analysis.md`](object-model/gap-analysis.md)) found the "principled gaps" claim
  was over-stated: only **~5–6 are true constitutional refusals** (atomic fair-exchange + the
  totally-ordered-ledger family per CC 1.7 / 3.3.10; real-time safety-critical control; biometric /
  mass-surveillance personhood per CC 3.1.5.4; hard DRM; physical-truth anchoring). The rest are
  **~5 clean unbuilt builds** (property/chattel/patent/easement `subject_kind`s, a hardware-attestation
  module, a conditional-execution composition, a durable-telemetry `subject_kind` — all via CC 4.5.1,
  *no new primitive*), ~2 hard-research, ~2 honest bridges, ~3 deferred-to-agent compute, and ~4–5
  simply mislabeled (already in-grammar). Honest roadmap, not twenty-seven refusals.

## Honest framing

CEWPOS is the **governance-first / "the other road"** position in AI safety: accountability and
verification over value-internalization, and a *federation of accountable agents* over an
unaccountable singleton. The substrate is the part that's provably soundable; the open frontier —
as for every civilizational substrate before it — is **legitimacy, inclusion, and adoption**, not
architecture. The repo proves the fabric; it does not claim to have solved the human-governance
layer it deliberately defers to.

## Ecosystem & references

CEWPOS is the OS-ification of a shipped, continuously-verified CIRIS stack — not a greenfield:

- **CIRIS** — [ciris.ai](https://ciris.ai) · **Continuous verification** (the conformance suite that
  *proves* the system enforces its constitution, 16,000+ tests) — [ciris.ai/verification](https://ciris.ai/verification/)
- **[CIRISConformance](https://github.com/CIRISAI/CIRISConformance)** — the federation-cohabitation +
  cross-artifact conformance harness (persist + verify + edge + nodecore + lenscore + registry). The
  authoritative reference for how the trio is wired together, and the proof the separately-shipped
  components enforce the constitution as one system.
- **[CIRISServer](https://cirisai.github.io/CIRISServer)** — the base fabric node (the cohabitation
  runtime / substrate host); the CEWPOS spec FSDs (`CEWPOS.md`, `HYBRID_LISP_OS.md`,
  `CEWPOS_RENDERED_OBJECTS.md`) live in its `FSD/`.
- Substrate crates the spikes wire against: [CIRISPersist](https://github.com/CIRISAI/CIRISPersist) ·
  [CIRISVerify](https://github.com/CIRISAI/CIRISVerify) · [CIRISEdge](https://github.com/CIRISAI/CIRISEdge).

## License

AGPL-3.0-or-later. See [`LICENSE`](LICENSE).
