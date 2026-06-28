# attestation-calculus-spike

The **minimal-and-adequate base** of the Attestation Calculus — the "third path" for
CEWPOS: instead of porting a mutable Lisp-Machine *image* (System 100) and running the agent
inside it (which collides with every CEG safety invariant), recognize that **CEG already *is* a
homoiconic, immutable, content-addressed Lisp** and give it a disciplined evaluator.

Design rationale: [`../CEWPOS_ATTESTATION_CALCULUS.md`](../CEWPOS_ATTESTATION_CALCULUS.md).
Grounded in the CIRIS Constitution CC 0.6 (CIRISRegistry `FSD/CIRIS_Constitution/`) and built
against the real substrate crates: `ciris-crypto` (CIRISVerify) and `ciris-persist` (CIRISPersist).

## Run it

```sh
# default — the pure calculus (no persist/edge crates pulled)
cargo run                              # the five demonstrations
cargo test                             # 30 tests (25 unit + 5 integration properties)

# real-persist — wire the REAL ciris-persist Engine (conformant emit + federation demo)
export CARGO_TARGET_DIR=/tmp/ciris-spike-target          # reuses the research compiles
export SQLITE3_LIB_DIR=/opt/homebrew/opt/sqlite/lib      # macOS: brew sqlite for the link
export DYLD_LIBRARY_PATH=$SQLITE3_LIB_DIR                 # and at runtime
cargo run  --features real-persist     # demos 1-5 + (6) shared-substrate federation
cargo test --features real-persist     # 30 + 3 federation tests

# real-edge — implies real-persist; stands up two real ciris-edge nodes
cargo build --features real-edge
cargo run   --features real-edge       # demos 1-7: (7) ships A->B over a real transport-http wire
cargo test  --features real-edge       # 30 + 3 federation + 2 wire tests
```

Requires a Rust toolchain ≥ 1.86, and (for `real-persist` / `real-edge`) a system `libsqlite3`.
The heavy `real-persist` / `real-edge` builds use git-pinned `ciris-persist v11.2.0` +
`CIRISVerify v8.3.0` and the path `ciris-edge` — set `CARGO_TARGET_DIR=/tmp/ciris-spike-target`
so the compiles are shared with the research probes.

## The three feature tiers (the real trio)

| Feature | Pulls | Adds |
|---|---|---|
| `default` | nothing beyond `ciris-crypto` (path) | the pure calculus + the in-memory effect path (real hybrid sign/verify, no DB) |
| `real-persist` | `ciris-persist` (git v11.2.0, `sqlite`) + `tokio` | **(A)** `effect::persisted` — `emit`/`promote` delegate to the REAL Engine **one-call build-sign-admit** (`register_self_federation_key` → `emit_attestation_self`), producing a real federation-tier `SignedAttestation`. **(B)** `federation_shared_substrate_demo` — two Engines on one shared sqlite DSN; node A emits, node B (same substrate) **sees** it (CIRISConformance test_300). |
| `real-edge` | `real-persist` + `ciris-edge` (path, `transport-http`) + `ciris-keyring` (git v8.3.0, `software`) | **(C)** `transport` — two real `ciris-edge` nodes; node A ships the promoted `SignedAttestation` A→B via `send_durable(AttestationGossip(..))` over a real HTTP wire, verified on B via `subscribe_verified_feed`. |

`default` and `real-persist` pull **neither** `ciris-edge` nor its git-dep tree (Reticulum /
openmls / axum); `ciris-edge` arrives only with `real-edge`. The default in-memory effect path
(`effect::emit`/`effect::promote`/`effect::Hybrid`) is unchanged by all of this.

## What each module proves

| Module | Lisp role | CEG realization | Property demonstrated |
|---|---|---|---|
| `sexpr.rs` | `read`/`print`, hash-consing | JCS (RFC 8785) + content-addressing | **Homoiconicity** — `jcs_bytes(v) == jcs_bytes(sexpr↔value(v))`, byte-exact |
| `forms.rs` | special forms (closed) | the 1+4 (`scores` + `delegates_to`/`supersedes`/`withdraws`/`recants`) | **Closed operators** — `read_form` admits only the 1+4; any other head ⇒ `UnknownOperator`. No general `eval` exists. |
| `pdma.rs` | `eval` strategy | PDMA reduction (CC 1.3) + WBD (CC 1.9) | **Reduction = the evaluator** — a `Thought` reduces to `Emit` or, when uncertain/novel or the Order-Maximization Veto fires, to a `Defer` normal form. |
| `effect.rs` | the only effect | gate-checked `emit` → `promote` | **The fabric refuses the brain** — `emit` writes an unsigned, content-addressed local-tier row; `promote` computes a **real** Ed25519+ML-DSA-65 hybrid signature and **verifies** it, enforcing `tier = federation ⟹ verified hybrid signature` (CC 5.3.2.4.3). Tamper ⇒ rejected. Under `real-persist`, `effect::persisted` delegates the same emit/promote to the REAL `ciris-persist` Engine's conformant one-call build-sign-admit. |
| `transport.rs` | the wire | `ciris-edge` 2-node `AttestationGossip` | **(real-edge)** the promoted `SignedAttestation` crosses a **real** transport-http wire A→B and B's inbound pipeline verifies the envelope, republishing it on the verified feed. |

## The one invariant that matters

There is **no general `eval`**. Pure computation is free; the sole side effect is `emit`, and
every emission is content-addressed and must clear the admission gate (hybrid sign + verify)
before it can become federation-visible. That is what lets a homoiconic, agent-legible system
keep CEG's fail-secure, closed-operator, separation-of-powers guarantees intact.

## Honest caveats (see design memo §5.1)

- `jcs_bytes` is total-or-honest-error in **both** feature configs: it coerces numbers to the
  RFC-8785 double model and rejects any number with no finite double (the `arbitrary_precision`
  hazard that `real-persist` would otherwise introduce).
- `print_sexpr` is a **display** rendering (non-injective: it sugars `attestation_type` into a
  bare head). The canonical reversible form is `value_to_sexpr`/`sexpr_to_value` + `jcs_bytes`.
  A string *reader* (parse → `Sexpr`) is deferred future work.
- `default` build links a clean `serde_json`; `real-persist` additionally pulls
  `ciris-persist` (git v11.2.0) + `tokio`, and `real-edge` adds `ciris-edge` + `ciris-keyring`
  (git v8.3.0) on top (all feature-gated; the default build pulls none of them). `ciris-persist`
  is git-pinned (not a path dep) so it is the SAME crate instance `ciris-edge` links — a path/git
  source split would fork `SignedAttestation` into two incompatible types and the wire would not
  typecheck.
- `real-edge` uses `HttpTransport` with `HybridPolicy::Ed25519Fallback` (edge's own e2e-test
  posture): the **envelope** is genuinely Ed25519-signed and verified through persist's
  `verify_hybrid_via_directory`, but classical-only on the wire. The **inner** `SignedAttestation`
  is the full conformant hybrid (Ed25519 + ML-DSA-65) row the Engine emitted; it rides the verified
  envelope verbatim and is not independently re-verified by the verified-feed path. `AttestationGossip`
  is `Delivery::Durable`, so the send is `send_durable` (not `send_federation`, which is for
  `Delivery::Federation`). Reticulum remains the production medium; transport-http suffices for the
  localhost 2-node loopback.

## Not in scope (next milestones)

A string reader (true read/print inversion), typed **view-forms** (pure, total,
WASM-sandboxed `data → 3D scene` functions — see [`../CEWPOS_VIEW_FORMS.md`](../CEWPOS_VIEW_FORMS.md)),
and embedding the evaluator in `ciris-server` (the corrected HYBRID_LISP_OS Step 1).
System 100 / juner_os are **reference/archival** only — not port targets.
