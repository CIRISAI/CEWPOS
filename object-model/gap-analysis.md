# Gap Analysis — adversarial challenge of the civilizational-surface gaps

> Each "gap" from `civilizational-surface.md` / `civilizational-top-gaps.json` re-tried against
> **MISSION.md** and **CC 0.6**. "Principled" is granted **only** when a specific clause makes the
> fabric *refuse by design* (out-of-grammar-by-construction or an apophatic prohibition); everything
> else is demoted to its true, more actionable category. The headline finding: the "~27 principled
> boundaries" claim **does not survive** — most of the residue is clean unbuilt builds, honest
> bridges, deferred-to-agent compute, or simply mislabeled (already in-grammar).

Categories: **PRINCIPLED-NON-GOAL** (the fabric shouldn't do it — leave it) · **BUILD-IT-WE-SHOULD**
(unbuilt vocabulary extension, no new primitive — the real roadmap) · **BRIDGE-TO-RAIL** (external
settlement/legal/physical rail) · **HARD-RESEARCH** (needs crypto/design that doesn't exist cleanly
yet) · **DEFERRED-TO-AGENT** (the excluded brain) · **MISLABELED** (already in-grammar).

## A — the 12 "top" gaps, re-tried

| # | Gap | Old | Challenged | CC / MISSION | Recommendation |
|---|---|---|---|---|---|
| 1 | Atomic fair-exchange / bilateral simultaneity (DvP, swap, escrow) | BR | **PRINCIPLED → BRIDGE** | CC 1.7 (unilateral+monotonic ⇒ impossible w/o trusted 3rd-party/ordered ledger, EGL); CC 3.3.10 "CEG never reverses value" | Leave it — the one fully-earned refusal, named by name |
| 2 | One-shot physical actuation (unlock door, shut valve) | BR/GAP | **BRIDGE-TO-RAIL** (+ effect-rail BUILD) | §5 bridge table: authz in-grammar, attestation can't *be* the effect | Mislabeled as gap; build receipt-back effect-rail (mirrors `settlement_ref`) |
| 3 | High-volume sensor/telemetry `subject_kind` | GAP | **MISLABELED (transport) + BUILD-IT (record)** | CC 5.3.3.2.2: high-rate telemetry rides the same realtime transport; fine-GPS limit = CC 2.6.6.1 ≤res7 + CC 1.13.3 (deliberate) | Transport in-grammar; build durable-telemetry subject_kind via CC 4.5.1; GPS limit is principled |
| 4 | Realtime convergent-merge (CRDT/OT/LWW) | GAP | **MISLABELED (transport) + DEFERRED (merge)** | CC 5.3.3.2.2 ops-transport IN; merge "application-layer" per CC 1.13.4 | Not a fabric gap; merge is consumer-layer by the same discipline that excludes the agent |
| 5 | Secret / receipt-free / coercion-resistant voting | GAP | **HARD-RESEARCH** | Non-repudiation structural; CC 1.13.3.1 names unlinkability vs global passive adversary a non-goal | Needs new ZK composition; not forbidden, not buildable today |
| 6 | Proof-of-unique-personhood / biometric uniqueness | GAP | **SPLIT: PRINCIPLED (biometric) + HARD-RESEARCH (privacy-preserving)** | CC 3.1.5.4 / §3.4: `biometric_inference`, `surveillance_mass` apophatic (score ≤0) | Biometric route = refusal (leave); privacy-preserving personhood = research |
| 7 | Hardware-rooted attestation chain (TPM/FIDO verify) | GAP (R5) | **BUILD-IT-WE-SHOULD** | `attestation:hardware_rooted` in-grammar; surface calls it "roadmap" | CIRISVerify verification module consuming external HW root; no new primitive |
| 8 | Native property/title/deed + chattel `subject_kind` | IG/GAP | **BUILD-IT-WE-SHOULD** (flagship) | Surface §4.A "pure 1+4 vocab, mirrors `partner_record`"; new subject_kind via CC 4.5.1 | Cleanest build, not a gap; legal-force is a separate bridge |
| 9 | Conditional / event-triggered execution (will-on-death, dead-man's-switch) | GAP | **BUILD-IT-WE-SHOULD** (composition) | `delegates_to` immediate (CC 2.4.1); `health:liveness` exists (CC 3.4.3); trigger = composition via CC 4.5.1 | Build liveness-triggered conditional-delegation; oracle is bounded design, not new crypto |
| 10 | Real-time safety-critical control (SCADA, AV control, infusion) | GAP | **PRINCIPLED-NON-GOAL** | CC 1.7 monotonic ≠ sub-second loop; + CC 3.1.5.4 `infrastructure_control` apophatic | Leave it — refused on two independent grounds; attest post-hoc, never drive the loop |
| 11 | Authoritative balance / proof-of-reserves / finality / order-book / bearer value | BR/GAP | **PRINCIPLED → BRIDGE-TO-RAIL** | CC 1.7 (no ordered ledger by design) + CC 3.3.10 (value transfer not a CEG primitive) | Earned refusal realized as chain/exchange bridge; several mis-tagged "GAP" are really BRIDGE |
| 12 | Ranked relevance / semantic search / recommendation | GAP | **DEFERRED-TO-AGENT** | Thesis "agency-free first"; CC 1.13.4 consumer-composes | The excluded brain that plugs in last; calling it a gap is a category error |

## B — the rest of the ~27 GAP rows

| Gap-family | Old | Challenged | Citation |
|---|---|---|---|
| Hard DRM / copy-prevention | GAP | **PRINCIPLED-NON-GOAL** | CC 4.4.3.4 Policy L Option A "once shared, always shared"; honest-holder discipline |
| Anchoring record to physical object/dirt (oracle) | GAP | **PRINCIPLED-NON-GOAL** | §5 "never claims a fact it can only receive"; CC 1.7 composes claims, doesn't adjudicate reality |
| Bearer / cash-like anonymous value | GAP | **PRINCIPLED → BRIDGE** | CC 3.3.10 Identity=Wallet; non-repudiation structural |
| Patent/trademark registry; easements/leases | IG/GAP | **BUILD-IT** (+ legal bridge) | CC 4.5.1; `transparency_log:inclusion` in-grammar (folds into #8) |
| Canonical `email` sub_kind | IG | **MISLABELED** (doc fix) | composes as `chat_message` + transport today |
| Recurring events / RRULE | GAP | **MISLABELED** | repeated `event_listings` today; RRULE = optional envelope field via CC 4.5.1 |
| Ephemeral live presence / typing-rate | GAP | **MISLABELED → PRINCIPLED** | D6: presence never attested/replicated/logged; aligns CC 1.13.3.1 |
| Tax calculation / withholding | GAP | **DEFERRED-TO-AGENT/operator** | obligation/`tax_id`/settlement in-grammar; arithmetic is consumer compute (CC 1.13.4) |
| Dynamic pricing / market-making | GAP | **DEFERRED-TO-AGENT/operator** | published prices = `scores` (IG); optimization = excluded brain (CC 1.13.4) |
| Biometric access / live proctoring | GAP | **PRINCIPLED-NON-GOAL** | CC 3.1.5.4 `biometric_inference` apophatic |

## Overall verdict

The "all ~27 are principled boundaries" claim **does not survive**. The distinct gap-families sort:

- **Genuinely PRINCIPLED constitutional refusals (~5–6):** atomic fair-exchange (CC 1.7); the ordered-ledger family — finality/clearing/order-book/bearer (CC 1.7 + CC 3.3.10); real-time control loops (CC 1.7 + `infrastructure_control`); biometric/mass-surveillance personhood (CC 3.1.5.4); hard DRM (Option A); physical-truth oracle. **Each cites a clause that makes the fabric refuse by design.**
- **BUILD-IT-WE-SHOULD (~5):** property/title/chattel + patent/trademark + easement `subject_kind`s; hardware-attestation verification module; conditional/event-triggered execution composition; durable-telemetry `subject_kind`. **All via CC 4.5.1 — no new primitive. This is the real roadmap, currently mis-banked as "principled."**
- **HARD-RESEARCH (~2):** receipt-free coercion-resistant voting; privacy-preserving proof-of-personhood.
- **BRIDGE-TO-RAIL, misfiled as gaps (~2):** one-shot actuation effect-rail; chain/exchange settlement functions tagged "GAP (rail)."
- **DEFERRED-TO-AGENT (~3):** recommendation/ranking, tax calculation, dynamic pricing.
- **MISLABELED — already in-grammar (~4–5):** high-rate telemetry + CRDT transport (both CC 5.3.3.2.2), RRULE, `email` sub_kind, ephemeral presence (principled-ephemeral).

Of the **12 "top" gaps**, only **~3.5 are genuinely principled** (#1, #10, #11, biometric-half of #6). The other **~8.5 deflate** into builds, bridges, research, deferred-agent, or mislabeled-transport. Most valuable single catch: **CC 5.3.3.2.2 makes #3 and #4 in-grammar at the transport layer.**

## Corrected headline framing

Of 550 civilizational functions, **~82% are fully in-grammar and ~95% are expressible** (in-grammar
or honestly bridged). Of the ~27 "needs-design" residue, only about **5–6 are true constitutional
refusals** the fabric *should* never cross (atomic fair-exchange + the totally-ordered-ledger family,
CC 1.7 / CC 3.3.10; real-time safety-critical control, CC 1.7 + `infrastructure_control`;
biometric/mass-surveillance personhood, CC 3.1.5.4; hard DRM; physical-truth anchoring). The rest are
**not** principled: about **five are clean unbuilt in-grammar builds** (property/chattel/patent/easement
`subject_kind`s, a hardware-attestation module, a conditional-execution composition, a durable-telemetry
`subject_kind` — all via CC 4.5.1, no new primitive); **two are genuine hard-research** (receipt-free
voting, privacy-preserving personhood); **two are honest bridges misfiled as gaps**; **three are
deferred-to-agent compute** (recommendation, tax-calc, pricing); and **four-to-five are mislabeled —
already in-grammar**. The honest story is *~95% expressible, roughly five clean builds remaining, and
only about five-to-six genuine constitutional refusals — not "twenty-seven principled boundaries."*

*Sources: `civilizational-surface.md`, `civilizational-top-gaps.json`, `CIRISRegistry/MISSION.md`,
`CIRISRegistry/FSD/CIRIS_Constitution/` (CC 1.7, 1.13.3.1, 1.13.5, 3.1.5.4, 3.3.10, 3.4, 4.5.1, 5.3.3.2.2).*

## Post-challenge correction — fair exchange is NOT a principled refusal

Gap #1 (and the ledger family, #11) above were filed as **PRINCIPLED constitutional refusals**. A
follow-up adversarial verification against CC 0.6 ([`CC1.7-fair-exchange.md`](CC1.7-fair-exchange.md))
**overturns that classification**:

- **Not ethical.** "Exchange/barter" is nowhere on the apophatic `prohibited:*` floor (CC 3.1.5.4);
  voluntary exchange *serves* M-1 (CC 1.1). CC 1.7 itself frames it as a grammar-adequacy bet, not an
  ethics bound.
- **Mostly IN-GRAMMAR by composition.** CC 1.7's EGL citation assumes *no trusted third party* — but
  CIRIS guarantees an accountable one (named-moderator invariant CC 4.5.4, WAs CC 4.3). The
  *optimistic* fair-exchange recipe composes from existing primitives with **no new primitive**:
  bilateral ratification (CC 3.3.5) + a steward-bound escrow custodian (the CC 4.4.3.2 `archive_custody`
  pattern via `delegates_to` + `key_grant`) + WA dispute resolution + `slashing`/`stake`.
- **Only the trustless atomic swap survives** as out-of-grammar (HTLC-class — atomicity against *all*
  parties incl. the escrow). The value leg (CC 3.3.10) and physical delivery bridge, as they do for
  *any* commerce.

**Net correction to the verdict:** the genuine **ethical** refusals reduce to the **apophatic-floor**
items (mass-surveillance, biometric-inference, infrastructure-control) + hard DRM; **fair-exchange and
the ledger family are *technical* bridges, not principled refusals**, and most of Commerce/Money moves
to in-grammar-by-composition. CC 1.7's standing target should be **narrowed** to "trustless atomic
swap" — a candidate CC 4.5.1 amendment (CC 1.7 is not entrenched).
