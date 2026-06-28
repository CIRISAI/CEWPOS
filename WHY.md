# Why CEWPOS?

*A 5-minute orientation. No prior CIRIS knowledge required.*

## In one sentence

CEWPOS is an operating system built not as a scheduler of processes but as an **evaluator over
immutable, signed records** — computation is deterministic, history is append-only, and **authority is
separated from execution**, so no participant (human or AI) gains power merely by running.

## The inversion

| | Conventional OS | CEWPOS |
|---|---|---|
| State | mutable | **immutable, append-only signed history** |
| Execution | privileged | **deterministic evaluation, no ambient authority** |
| Audit | afterward | **the record *is* the state — auditable by construction** |

Every meaningful thing — an identity, a config value, a consent, a moderation action, a payment claim,
a media stream — is a **signed attestation**. You never mutate one; you **supersede** it (the old one
stays, for provenance). An agent doesn't "control" anything: it **emits an attestation**, and an
independent **admission gate** admits or rejects it. The platform never has to trust the agent — it
verifies the constraint.

```
agent → emit attestation → admission gate → federation        (not:  agent → kernel)
```

## The small kernel (and why it matters)

Semantics are a **closed set of five operators** (`scores` + `delegates_to` / `supersedes` /
`withdraws` / `recants`). The system grows by **vocabulary, not new primitives** — a land title, a
vaccination record, and a group chat are the same five operators wearing different names. That's the
discipline behind Git's object model, content-addressed storage, capability systems, and Lisp. A small
kernel is an **auditable** kernel: most ambitious infrastructure fails because the trusted core grows
without bound; this one is designed to do the opposite.

## Three layers — keep them separate as you read

This is the key to reading the whole project:

| Layer | What it is | Status |
|---|---|---|
| **Mathematics** | the Attestation Calculus — the closed operators + the canonical (content-addressed, hashable) form | **provably invariant** (frozen) |
| **Mechanism** | the deterministic, total, effect-typed evaluator + the WASM-sandboxed renderer | **implemented + verified** |
| **Policy** | the Constitution, the vocabulary, federation rules, admission/scoring/revocation criteria | **governance-defined** (and amendable) |

The math is fixed and provable. The mechanism is code you can verify and re-run. The policy is **data
you can argue about and amend** — not opaque code. Almost everything people call "the hard questions"
lives in the Policy layer **on purpose**: CEWPOS pushes judgment *out* of the kernel into
externally-visible, adjudicated data, so it can be inspected, contested, and changed without touching
the math.

## What makes it different: moderation is first-class, day one

Most systems treat moderation and governance as an *edge* concern (ATProto keeps the protocol amoral
and moderates downstream), as *optional*, or as *absent* ("code is law"). CEWPOS is **moderation-first**:
governance is a **structural invariant**, not a bolt-on.

- A community **cannot operate at moderated capability without a live, accountable, named moderator**
  (the *named-moderator existence invariant*) — and the system guarantees one is always present.
- **Every** state transition passes the admission gate (rules hash-pinned · mechanism-not-quality ·
  re-checkable · never sole evidence).
- Conflicts **defer to accountable adjudicators** (Wise Authorities); bad actors are **slashable**;
  a single accountable signer can **halt** (reverse-quorum).

So the questions that sink other federated systems — *who resolves conflicting vocabularies? who
handles semantic drift? who governs evolution?* — are not an afterthought here. They are the **core
primitive**. If you're going to build a shared substrate for many parties (and many AI agents), the
moderation layer is the hard part, and it's the part CEWPOS builds first.

## The claims are checked, not aspirational

This matters because architecture diagrams are cheap. The CIRIS stack follows a discipline of
**claim → invariant → implement → verify → prove**:

- **[ciris.ai/verification](https://ciris.ai/verification/)** — a conformance suite (16,000+ tests
  across the separately-shipped components) that *proves the system enforces its constitution* rather
  than asserting it in prose.
- **[ciris.ai/proof](https://ciris.ai/proof)** — the underlying evidence artifacts.

And the substrate ships: `ciris-persist` v11.5, `ciris-verify` v8.3, `ciris-edge` v7.4,
`ciris-server` v0.5.58 (with KMP/Compose clients), the agent on the app store, the CEG wire at
1.0-RC29 with the 1+4 surface frozen.

## Then read

1. [`README.md`](README.md) — the engineering overview (architecture, data model, evaluator, security).
2. [`CEWPOS_ATTESTATION_CALCULUS.md`](CEWPOS_ATTESTATION_CALCULUS.md) — the design memo (why CEG *is*
   the language and just needed an evaluator).
3. [`object-model/`](object-model/) — the whole agency-free fabric as one schema-validated dataset.
