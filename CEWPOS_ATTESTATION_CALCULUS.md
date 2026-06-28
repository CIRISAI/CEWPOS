# The Attestation Calculus — resolving Lisp × CEG for CEWPOS

> **Status:** Design memo + working spike. Answers the open question raised by
> `FSD/CEWPOS.md` and `FSD/HYBRID_LISP_OS.md` (CIRISServer PR #124).
> **Verdict:** The merge as those FSDs frame it (port System 100's mutable Lisp-Machine
> image, run CIRISAgent inside it with live `eval`) **should not be so**. There *is* an
> organic merge, but it requires inverting the proposition. That inversion — CEG *is* the
> canonical immutable Lisp; give it a disciplined evaluator — is the **third path**, named
> here the **Attestation Calculus**.
> **Grounding:** CIRIS Constitution CC 0.6 (CIRISRegistry `FSD/CIRIS_Constitution/`);
> CIRISVerify `9e6f295` (ciris-crypto 8.3.0); CIRISPersist `8c70147` (ciris-persist 11.2.0).
> Reference projects downloaded under `referenced-projects/` (juner_os; Tumbleweed System 100.0).

## 0. The question

`HYBRID_LISP_OS.md` proposes building CEWPOS on a Rust-Lisp hybrid: a `no_std` Rust
substrate (already realized as `ciris-persist` / `ciris-edge` / `ciris-verify`) under a
*Lisp Machine image* (System 100 / CADR, with `juner_os` as the embed proof). Its Step 3 is
"port the `CIRISAgent` reasoning loops to Lisp, utilizing the deep reflection capabilities to
build the dynamic 'Everything App' surface" — i.e. the agent lives inside a live, mutable,
homoiconic image and generates UI by evaluating new forms into kernel state.

The substrate layer is settled and is not in question. The open question is the layer above:
**can the homoiconic Lisp image and CEG (the CIRIS Epistemic Grammar — the wire format the
federation speaks) become one organic thing, or are they in tension?**

## 1. Verdict: the FSD's merge is a category error

The Lisp-Machine *image* model and CEG are not two layers that stack. They are two opposite
answers to a single question — *what is truth in this system?* — and they disagree on every
axis that matters for safety.

| Axis | Lisp-Machine image (System 100 / juner_os MAL) | CEG (CC 0.6) |
|---|---|---|
| **State** | mutable in place (`setf`, `rplaca`); truth = current image | append-only; `supersedes` / `withdraws` / forward-only revocation; truth = the signed history |
| **Identity of a fact** | a live object — no stable bytes | `JCS(envelope)` canonical bytes (RFC 8785), hybrid-signed, byte-indistinguishable on the wire (CC 5.3.2.4.2) |
| **Operators** | general `eval` — the *maximally open* operator | "open data, **closed operators**": 5 structural ops, 5 query predicates, no caller SQL (CC 5.3.2.4.4) |
| **Posture** | pause / inspect / **modify live**; agent reaches into kernel state | fail-secure (CC 1.5), WBD — *halt rather than guess* (CC 1.9), kill-switch |
| **Authority** | one self-contained image; "the OS *is* the agent" | separation of powers; agency-free quorum-bound fabric that can **refuse** the brain (CEWPOS §5) |
| **History** | working set, GC'd | kept for posterity; every federation-tier row carries ML-DSA-65; forge-now / harvest-later is the threat (CC 5.3.2.4.3.1) |

The FSD's headline feature — *"the LLM evaluates new Lisp forms into the live environment …
hallucinated interfaces are trivial"* — is **general `eval` into kernel state**. That is the
exact negation of:

- the **four-test prefix-admission gate** (CC 1.2: rules hash-pinned; mechanism-not-quality;
  re-checkable; never sole evidence),
- the **closed-operator discipline** (CC 5.3.2.4.4: *open data, closed operators*), and
- the **kill-switch / fail-secure** posture (CC 1.5).

An agent that can patch the image has already defeated everything the substrate exists to
enforce. And *"the OS is the agent"* collapses the **separation-of-powers invariant** that
`CEWPOS.md` §5 names as its own culmination: the fabric must be able to refuse the brain;
an agent fused into the image has nothing left to be refused by.

**So a literal System 100 → juner_os port, with CIRISAgent living in the image, is not
integration. It is the substrate surrendering to the brain. As framed, it should not be so.**

## 2. The inversion: CEG already *is* a Lisp

The resolution falls out of an observation the FSDs missed: **CEG is already a homoiconic,
immutable, content-addressed Lisp. It simply has no evaluator yet.** The correspondence is
1:1, not analogy:

| Lisp concept | CEG realization (already exists) |
|---|---|
| `read` / `print` | JCS canonicalization (RFC 8785) — total, deterministic (CC 2.6.1) |
| cons cell / s-expression | the envelope — the "1" surface (CC 2.1) |
| special forms (a closed set) | the 5 structural ops `scores` + `delegates_to` / `supersedes` / `withdraws` / `recants` — *closed at five, exactly as CEG demands* (CC 1.7) |
| symbol / package + `intern` guard | dimension prefix / open namespace + the four-test admission gate (CC 1.2, CC 4.5.1.1) |
| hash-consing / interning | content-addressing (sha256) (CC 5.3.2) |
| persistent (immutable) update | `supersedes` / `withdraws` — append, never mutate (CC 5.3.2.3) |
| `eval` | **the PDMA reduction** — *not* general eval (CC 1.3) |
| effects / IO | exactly **one** effect: `emit` an attestation, gate-checked at admission |
| "stuck term" / no normal form | **WBD** — a deferral is a normal form that emits `defer` and waits for a Wise Authority (CC 1.9) |
| GC / the image | the persist corpus — but append-only, signed, federated; *not* a mutable image |

So the move is not to put CEG *inside* a Lisp image. It is to recognize that **CEG is the
canonical immutable Lisp** and give it the one thing it lacks: a disciplined evaluator.

## 3. The third path — the Attestation Calculus

A homoiconic, immutable, content-addressed, **total, effect-typed** Lisp:

1. **Reader = JCS.** An attestation reads as an s-expression —
   `(scores :dimension "evaluation:quality" :score 0.85 :confidence 0.92 :evidence-refs ())`
   — and prints back **byte-identically** under RFC 8785. Homoiconicity (the FSD's real goal:
   a reflective, agent-legible system) is **gained**, not lost.
2. **Special forms = the 1+4, and nothing else.** The reader rejects any head outside the
   five structural ops. The closed-operator invariant is enforced *at read time*. There is no
   general `eval`.
3. **One effect: `emit`.** Pure computation runs freely; the only way to affect the world is
   to emit a candidate envelope, which is then subject to the substrate's admission gate
   (hybrid-sign + verify, the federation-tier invariant `tier = federation ⟹ hybrid signature
   present`, CC 5.3.2.4.3). The LLM *generates* candidate forms; a form becomes real only by
   reducing to a gate-checked `emit`.
4. **Evaluation strategy = PDMA.** The Order-Maximization Veto, conflict resolution, and WBD
   (CC 1.3, CC 1.9) become the *reduction semantics*. The ethics is no longer a Python loop
   bolted above an unrelated runtime — **the ethics is the evaluator.** That is the organic
   part: M-1 is not a layer on the Lisp; M-1 is the reduction rule.
5. **Typed view-forms — the UI surface, safe by type + sandbox, not by trust (no "hallucinated
   UI").** The agent *generates* view-forms (they are data — CEG s-exprs), but a view-form is a
   **pure, total, WASM-sandboxed** function `data → 3D scene` with a typed (WIT) interface: it
   computes a rendering and **nothing else** — no kernel, no substrate, no network. Dynamic and
   generative, but structurally incapable of the failure mode. For CEWPOS a typed form's *output
   type is a 3D scene* (Rust + Bevy + WebGPU, browser-deployable end-to-end in WASM).

### 3.1 What it preserves (vs. the naive merge, which breaks all of these)

- **Homoiconicity / reflection** — the agent reasons over its own emissions *as data*; the
  system is inspectable. (The FSD's actual goal — delivered.)
- **Fail-secure + closed operators + four-test gate** — the only effect is gate-checked
  `emit`; no general eval exists.
- **Separation of powers** — the evaluator (brain) can only *propose*; the Rust substrate
  (fabric) admits or refuses. The agent is **not** the OS; the agent is a reduction strategy
  whose effects are adjudicated.
- **PQC-signed posterity** — forms are content-addressed; federation-emit signs them; nothing
  is mutated in place.
- **M-1 / PDMA** — becomes the operational semantics instead of a bolt-on.

### 3.2 What it requires dropping

- **The mutable single-image model** (System 100's core). System 100 → *archival + UX
  inspiration*, **not** a port target. Its inspectability aesthetic is the thing to learn from;
  its mutable-image truth-model is the thing to refuse.
- **juner_os's MAL evaluator** (mutable environment, general `eval`). juner_os's *architecture*
  (a `no_std` Rust host carrying a Lisp near kernel state) is a useful scaffold; its
  *semantics* must be replaced by the total / effect-typed evaluator.
- **"Lisp all the way down"** → **"CEG all the way down, with a Lisp evaluator."** The
  substrate stays Rust (`persist` / `edge` / `verify`). Lisp is the cognition/agency layer's
  *language*, compiled to `emit`-calls — not the kernel.

## 4. Correction to the HYBRID_LISP_OS roadmap

| FSD step | Disposition |
|---|---|
| **Step 1** — embed a minimal Lisp in `ciris-server` to run agent logic inside the fabric process | **Keep, redefined.** Embed the *total, effect-typed* interpreter whose `emit` is wired to `ciris-verify` (sign/admit) + `ciris-persist` (local-tier write) — **not** a general REPL. |
| **Step 2** — bare-metal `no_std` port of `edge` / `verify` | **Deprioritize.** CEG's safety is cryptographic + protocol, not ring-0. "Zero POSIX baggage" is aesthetics that do not serve M-1. |
| **Step 3** — port PDMA to Lisp via a reflective mutable image | **Reframe.** PDMA becomes the evaluation strategy over *immutable signed forms*; reflection is over signed attestations, not live kernel objects. |

## 5. First milestone — the spike (in this repo)

Not "port System 100." A cheap spike that validates the thesis end-to-end against the **real**
crates. See `attestation-calculus-spike/`. It demonstrates, with real `ciris-crypto` signing
and real RFC-8785 (`serde_jcs`) canonicalization (and the real `ciris-persist` local-tier
write under `--features real-persist`):

1. **Homoiconicity** — a CEG envelope reads to an s-expr and prints back **byte-identically**
   under JCS (the `read`/`print` bijection over the real canonicalizer).
2. **Closed operators** — the reader admits only the 1+4 heads; any other head is a read error.
3. **PDMA-as-eval** — a "thought" reduces via the Order-Maximization Veto to either an `emit`
   or a `defer`.
4. **WBD normal form** — an uncertain thought reduces to `(defer …)`; no attestation is emitted.
5. **Gate-checked emit → promote** — `emit` writes an unsigned **local-tier** form; `promote`
   computes the **real** hybrid Ed25519 + ML-DSA-65 signature and **verifies** it — enforcing
   `tier = federation ⟹ hybrid signature present` with the actual crypto, not a stub.

If the spike holds, the organic merge is real and the Attestation Calculus is the road to CEWPOS.

### 5.1 As-built status (verified)

The spike is built and green. `cd attestation-calculus-spike && cargo run` prints all five
demonstrations; `cargo test` passes **30/30** (default) and the optional
`cargo build --features real-persist` compiles green against the real `ciris-persist`
(in-memory SQLite local-tier write + promote). Modules: `sexpr` (homoiconic JCS reader/printer
+ content-addressing), `forms` (the closed 1+4 gate), `pdma` (reduction = evaluator, WBD =
normal form), `effect` (the one gate-checked emit→promote, real `ciris-crypto` Ed25519+ML-DSA-65).

Two findings from the adversarial verification pass, both resolved:

- **`jcs_bytes` totality under `arbitrary_precision`.** The `real-persist` feature transitively
  unifies `serde_json/arbitrary_precision` into the graph, under which a string-backed `Number`
  (e.g. `1e1000`) can bypass the finite-double model and make canonicalization non-canonical or
  non-total. Fixed: `jcs_bytes` now coerces every number to the RFC-8785 double model *before*
  canonicalizing and returns an honest `Err` on any number with no finite double — so a content
  address is always either the spec-canonical hash or a hard error, in **either** feature config.
  Verified live: under `real-persist` the guard fires on `1e1000`; under default the literal is
  rejected at parse and the guard is a no-op. The default canonical forms are byte-unchanged.
- **`print_sexpr` is a display rendering, not a reversible reader.** Documented as non-injective
  (it sugars `attestation_type` into a bare operator head) and hardened to quote symbol-unsafe
  keys. The canonical, reversible homoiconic representation is `value_to_sexpr`/`sexpr_to_value`
  + `jcs_bytes` (proven byte-exact). A string *reader* (parse → `Sexpr`) is deferred future work.

## 6. Open questions / falsification targets

- **Fair exchange (CC 1.7).** The constitution's own standing falsification target — atomic
  content-for-payment — is out-of-grammar for 1+4 and bridges to an external settlement rail.
  The calculus inherits exactly this boundary: it is a *unilateral, monotonic* reduction; it
  cannot express bilateral simultaneity in-grammar. Honest non-goal, not a defect.
- **Totality vs. expressiveness.** A total evaluator cannot be Turing-complete. The bet is that
  agent cognition decomposes into (pure total computation) + (gate-checked emit) + (deferral
  for everything that doesn't reduce) — i.e. WBD *is* the escape hatch for non-termination.
  This is the thesis most worth attacking.
- **View-forms.** "Pure view-forms with zero kernel access" needs a concrete typed effect
  boundary before any UI work; out of scope for the spike, on the critical path for the
  "Everything App" surface.
