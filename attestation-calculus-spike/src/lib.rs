//! # attestation-calculus-spike — the Attestation Calculus
//!
//! A minimal-and-adequate spike of CEG-as-Lisp: a homoiconic, immutable,
//! content-addressed, total, effect-typed evaluator over the real CIRIS
//! substrate. The whole calculus is four small modules:
//!
//! * [`sexpr`] — the **homoiconic** layer. `Sexpr` is a lossless mirror of
//!   `serde_json::Value` (numbers kept verbatim), with an RFC 8785 (JCS) reader
//!   ([`sexpr::jcs_bytes`]), a homoiconic printer ([`sexpr::print_sexpr`]), and
//!   a SHA-256 content address ([`sexpr::content_address`]). Code *is* data: an
//!   envelope and its s-expression are the same object.
//!
//! * [`forms`] — the **closed 1+4 special forms**. The reader
//!   ([`forms::read_form`]) is the closed-operator gate: it admits exactly the
//!   structural op `scores` plus the four composers `delegates_to` /
//!   `supersedes` / `withdraws` / `recants`, and rejects every other head
//!   (`eval`, `exec`, `lambda`, …) at read time. Open data, closed operators.
//!
//! * [`pdma`] — the **evaluator**. The CIRIS Principled Decision-Making
//!   Algorithm + Wisdom-Based Deferral *is* the reduction rule:
//!   [`pdma::reduce`] takes a `Thought` to exactly one of two terminals —
//!   `Emit` a gate-checked `scores` form, or `Defer` (the WBD normal form, the
//!   safe terminal). The ethics is the evaluator.
//!
//! * [`effect`] — the **one effect**. Pure computation runs freely; the only
//!   way to touch the world is [`effect::emit`] (a local-tier, unsigned,
//!   content-addressed row) followed by [`effect::promote`], the admission gate
//!   that computes and verifies a real hybrid Ed25519 + ML-DSA-65 signature
//!   over the canonical bytes and refuses anything that does not verify.
//!
//! Together these give a reflective, agent-legible system with no mutable
//! image: every fact is its bytes, every operator is one of five, and the only
//! way to act is to sign and be admitted.

pub mod sexpr;
pub mod forms;
pub mod pdma;
pub mod effect;

/// (C) The real `ciris-edge` 2-node wire path — gated behind `real-edge` (which
/// implies `real-persist`). Ships a promoted [`effect::persisted`]-emitted
/// `SignedAttestation` A→B over a real transport-http medium, verified on B.
#[cfg(feature = "real-edge")]
pub mod transport;
