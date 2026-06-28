# CC 1.7 — Fair Exchange is (mostly) in-grammar: a clarification proposal

> **Finding:** the Constitution's one named "out-of-grammar" boundary — **atomic fair-exchange /
> bilateral simultaneity (CC 1.7)** — is *over-broadly phrased*. Fair exchange / barter is **not an
> ethical refusal** (it serves M-1; it is nowhere on the apophatic floor), and the *accountable
> ("optimistic") form* of fair exchange is **expressible in-grammar by composition with no new
> structural primitive**. Only the **trustless, third-party-free atomic swap** (HTLC-class) genuinely
> remains out-of-grammar. CC 1.7 should be **narrowed, not removed.**
> **Status:** verified against CC 0.6; a candidate amendment via the CC 4.5.1 process (CC 1.7 is
> **not** entrenched — only CC 1.2, CC 4.2, and the amendment process are). Pre-maturity (<100k
> nodes): founder / accord-holder sign-off.

## The category error

A *principled refusal* is "the fabric **won't**, because it's wrong" — the apophatic
`prohibited:*` floor (CC 3.1.5.4): `surveillance_mass`, `biometric_inference`,
`infrastructure_control`, `deception_fraud`, … **"Exchange," "barter," "commerce" appear nowhere on
it.** Voluntary exchange is the foundation of cooperation, which M-1 (CC 1.1) exists to protect.
So fair-exchange can be at most a *technical* boundary, never an ethical "no." CC 1.7 itself frames it
as a *1+4-adequacy* bet ("a bridge, not a primitive") — not an ethics bound. Citing CC 1.7 as "CIRIS
refuses commerce" miscategorizes a grammar-expressiveness boundary as a moral one.

## Why EGL does not bind CIRIS

CC 1.7 invokes Even–Goldreich–Lempel: fair exchange between mutually-distrusting parties is impossible
**without a trusted third party (TTP) or a totally-ordered ledger.** That premise — *no TTP* — is the
*trustless-crypto* world. It is the opposite of CIRIS, which **guarantees an accountable third party by
design**: the named-moderator existence invariant (CC 4.5.4), Wise Authorities + WBD (CC 4.3 / 1.9),
the fire-floor-of-1 (CC 4.5.4). EGL's impossibility assumes away exactly what CIRIS provides — and
**CC 1.7's own citation is to "Optimistic fair exchange"** (Asokan–Schunter–Waidner), the TTP-invoked-
on-dispute construction CIRIS can run natively. CC 1.7 undersells its own capability.

## The in-grammar barter recipe (no new primitive)

| Leg | Mechanism (CC) | Status |
|---|---|---|
| offer / accept | **bilateral ratification** — two `consent_record` halves under one `bilateral_pair_id`, both `stance: granted` (CC 3.3.5 / 2.3.2.4). "The structural primitives close the bilateral shape — no new attestation_type, no new field." | **IN-GRAMMAR** |
| atomic release of a **digital** good | `delegates_to` (bounded scope, CC 2.4.1.2) → a steward-bound **escrow custodian** emitting two `key_grant`s (CC 3.3.2). Precedent in the CC: `archive_custody` is "an institutional archive-key **custodian/escrow** … rides the key-grant/escrow cascade" (CC 4.4.3.2). | **IN-GRAMMAR by composition** (atomicity = escrow-trust, not cryptographic) |
| dispute resolution | named-moderator / WA adjudication, **always present by design** (CC 4.5.4 / 4.3 / 1.9) — optimistic fair exchange's TTP-on-dispute. This is **WBD applied to exchange**. | **IN-GRAMMAR / by-design** |
| defection → consequence | `commitment_fulfillment:{prior}` (CC 3.1.9.2) + `slashing:{outcome}` (CC 3.1.9.3, WA-quorum gated, never sole-evidence) + the per-attestation `stake` field (CC 2.4.2). | **IN-GRAMMAR** |

## What genuinely remains out-of-grammar (the narrowed target)

1. **Trustless, TTP-free, atomic swap (HTLC-class)** — commit-or-abort against **all** parties
   *including the escrow*, without a TTP **and** without a totally-ordered ledger. CEG attestations
   are "unilateral, monotonic graph claims" (CC 1.7) with no two-phase-commit primitive; CEG is
   deliberately not a totally-ordered ledger (CC 3.3.10). **This, and only this, is the boundary.**
2. **The value leg** — "value transfer is **not** a CEG primitive — it rides external rails … CEG
   never reverses value" (CC 3.3.10). Not fair-exchange-specific (true of any payment).
3. **Physical delivery** — outside any wire format. Not fair-exchange-specific.

## The honest counter (where the in-grammar claim stops)

**1+4 buys accountability, not atomicity** — and that gap is the boundary:
- a malicious/colluding escrow can still defect; CEG gives *after-the-fact* redress (WA ruling +
  `slashing`), which is **accountability, not prevention**;
- a revealed `key_grant` leaks the bytes irreversibly — "the wire cannot un-send bytes a peer already
  holds" (CC 3.3.5); an adjudicator cannot *un-reveal* a leaked secret;
- griefing/timeout has no native HTLC refund;
- the value leg can't be made atomic with the digital leg without a rail.

So: fair exchange is in-grammar for the **optimistic/accountable** version, out-of-grammar for the
**trustless-atomic** version — which is precisely the version CC 1.7 *named*.

## Proposed CC 1.7 clarification (one paragraph, for the CC 4.5.1 process)

> *The standing falsification target is the **trustless, third-party-free atomic swap** — bilateral
> simultaneity with commit-or-abort against **all** parties **including any escrow**, achieved
> **without** a trusted third party **and without** a totally-ordered ledger (the HTLC / atomic-swap
> class). This, and only this, is out-of-grammar: CEG attestations are unilateral and monotonic, with
> no two-phase-commit primitive, and value transfer rides external rails (CC 3.3.10). **Fair exchange
> via accountable adjudication ("optimistic fair exchange") is expressible in-grammar by composition**
> — bilateral ratification (CC 3.3.5) for offer/accept; a steward-bound escrow custodian (the
> CC 4.4.3.2 `archive_custody` pattern) authorized by `delegates_to` (CC 2.4.1.2) emitting `key_grant`s
> (CC 3.3.2) for the digital leg; the always-present named-moderator/WA adjudicator (CC 4.5.4 / 4.3)
> for disputes; and `commitment_fulfillment` + `slashing` + `stake` (CC 3.1.9.2 / 3.1.9.3 / 2.4.2) for
> defection. The residual bridges are the value leg (CC 3.3.10) and physical delivery — neither
> fair-exchange-specific. 1+4 buys **accountability**, not cryptographic **atomicity**; the one domain
> that reaches outside itself is atomic simultaneity, not commerce.*

## Downstream effect on the gap analysis

The number of *named structural boundaries* is unchanged in kind (atomic simultaneity remains the one
target) but its **scope collapses** from "fair exchange / barter" (a domain) to "trustless atomic
swap" (one construction). Most of **Commerce & Money** moves to **in-grammar by composition**:
contracts/offers (bilateral ratification), paid/licensed content access + subscriptions (`key_grant`
`SingleContent` / `SubscriptionTier`), escrowed delivery of digital goods (`delegates_to` + custodian),
receipts/auditability (`settlement`, CC 3.3.10), and reputation/penalty (`commitment_fulfillment` +
`slashing`). The residual out-of-grammar set for Commerce becomes exactly **{trustless atomic swap,
the value-movement rail, physical delivery}** — and **only the apophatic-floor items remain genuine
*ethical* refusals** (surveillance/biometric-inference/infrastructure-control). Net: the boundary is
preserved but made **sharper and harder to accidentally "refute"** — pointing at escrow-mediated
commerce no longer counts as a counterexample, because that was never the target.
