# CEWPOS Object Model — Functional Specification (FSD)

| | |
|---|---|
| **Document** | CEWPOS Object Model — Synthesis FSD |
| **Status** | COMPLETE (object-model v1.0) |
| **Constitution** | CC 0.6 |
| **Sources** | constitution-algebra · repo-knob-inventory · civilizational-surface |
| **Generated** | 2026-06-27 |
| **Machine form** | [`cewpos-object-model.json`](./cewpos-object-model.json) conforming to [`schema.json`](./schema.json) |
| **Schema status** | VALID — 0 violations across 856 records (361 objects · 247 transformations · 132 knobs · 8 capabilities · 476 functions · 22 agent-deferred · 10 open-items) |

> This FSD is the prose synthesis that ties the three legs together. The arrays are authoritative
> in the machine form; this document is the reading order, the ledger, and the honesty record.

---

## 1. Thesis

**A closed kernel under an open vocabulary, with no agency in the substrate.**

1. **Closed five operations.** Every act on the wire is one of five primitives: `scores` (the one
   workhorse attestation) plus four structural composers — `delegates_to`, `supersedes`,
   `withdraws`, `recants` (CC 1.7 / 2.4.1). This set is *minimal-and-adequate*: nothing in the
   552-element grammar adds a sixth primitive. The 247 transformations are all **compositions and
   admission gates over these five** — admission, merge, transport/crypto, retirement, canonicalization
   — never a new verb.

2. **Open vocabulary.** Expressive growth happens entirely in the **namespace** — dimensions,
   subject-kinds, reserved prefixes, composition policies (Policy A–M) — gated by the mechanical
   four-test gate (T1–T4). "Extensions are prefixes/fields, never new primitives" (CC 1.2). This is
   why a 5-op kernel can chase a 476-function civilizational surface without growing the kernel.

3. **Agency-free fabric.** The substrate **attests, stores, transports, and composes by declared
   policy — it never reasons, judges, or decides what is good.** Of 361 objects, **329 are `fabric`
   and 32 `boundary` (0 agent)**; of 247 transformations, **231 `fabric` / 16 `boundary` (0 agent)**;
   of 132 knobs, **97 `fabric` / 34 `boundary` / exactly 1 `agent`** — the lone agent-layer knob is
   `knob.server.py_adapter` (the pluggable LLM provider), which is precisely the seam where a brain
   bolts on. A fabric node = "a node + no brain"; an agent = "a node + a brain."

4. **Agent layer deferred.** The whole moral-reasoning loop — PDMA/CSDMA/DSDMA/IDMA reductions, the
   conscience faculties, the six foundational principles, Wisdom-Based Deferral — is **recorded but
   explicitly NOT part of the agency-free fabric** (22 deferred items, §6). The substrate carries the
   *fields* an agent reads (`oversight_mode`, `epistemic_mode`, `stake`) but performs none of the
   reasoning.

---

## 2. The Three Legs (GRAMMAR ∩ REALIZATION ∩ CIVILIZATIONAL TARGET)

The model is the intersection of three independently-derived inventories. The object model is the
*join*: each civilizational function is mapped to the grammar objects/transformations that express it
and to the repo component that realizes it.

| | **LEG A — GRAMMAR** | **LEG B — REALIZATION** | **LEG C — CIVILIZATIONAL TARGET** |
|---|---|---|---|
| **Source** | Constitution algebra (CC 0.6) | Repo knob/capability surface | The civilizational-surface map |
| **What it is** | The closed kernel + open namespace | What is actually built & tunable | What the internet/civilization require |
| **Records** | **361 objects + 247 transformations** | **132 knobs + 8 capabilities** | **476 functions** |
| **Spanning** | Parts I–VIII | 9 repos (persist 26 · edge 23 · verify 18 · nodecore 16 · registry 16 · agent 13 · server 12 · lens 8) | 8 super-domain clusters |
| **Invariant** | 1 + 4 primitives, never a 6th | All 8 capabilities `covered: true` today | Mapped to grammar + component |

**LEG A (Grammar).** Identity & keys, the envelope and its fields, the five primitives, the 8-owner /
83-family dimension namespace, the subject-kinds, consent, transport & streaming, governance, the
coherence/retirement calculus, and the conformance surface. The transformations are the closed op-set
plus every admission/merge/crypto/retirement composition that the Constitution defines over it.

**LEG B (Realization).** The eight named cross-cutting capabilities — *moderation, quorum (M-of-N to
allow), reverse-quorum (M-of-N to block/halt), child-safety floor, voting/ratification, media/blobs,
social-feed/live-streaming, EigenTrust* — each fully wired across the repos today (all `covered:true`),
tuned by the 132 knobs. Each capability record carries its CEG objects, transformations, knobs, and a
CEWPOS-native 3D view-form.

**LEG C (Target).** 476 enumerated functions across the internet's own substrate, communication &
media, social/relationships, commerce & money, governance & law, identity/records/property,
knowledge/science/education, and health/safety/physical. Each tagged `in-grammar | bridge |
gap-needs-design`.

---

## 3. Coverage Ledger

**Honest, JSON-derived counts.** Computed directly from `civilizationalFunctions.json` (476 rows);
these are the numbers the machine form carries in `meta.counts`.

| Cluster | Functions | In-grammar | Bridge | Gap-needs-design |
|---|---:|---:|---:|---:|
| The Internet's own substrate layers | 55 | 43 | 10 | 2 |
| Communication & Media | 54 | 48 | 4 | 2 |
| Social, Relationships & Community | 49 | 41 | 5 | 3 |
| Commerce, Money & Markets | 69 | 40 | 22 | 7 |
| Governance, Law & Collective Decision | 75 | 64 | 10 | 1 |
| Identity, Records & Property | 69 | 57 | 9 | 3 |
| Knowledge, Science & Education | 46 | 39 | 5 | 2 |
| Health, Safety & the Physical World | 59 | 46 | 7 | 6 |
| **TOTAL** | **476** | **378** | **72** | **26** |

- **In-grammar: 378 / 476 (79.4%)** — expressible in pure 1+4 + open namespace.
- **Bridge: 72 / 476 (15.1%)** — needs an external rail (settlement / legal force / physical actuation).
- **Gap-needs-design: 26 / 476 (5.5%)** — no clean CEG expression yet.

**Covered-by-existing-component — read carefully.** "In-grammar" means *the record shape is
expressible*, **not** *the function is built and delivered*. **31 of 476 rows carry `GAP` in the
`cirisComponent` column** (the unbuilt seam), including **4 rows that are tagged `in-grammar` while
their own component column says GAP** (land-title, easements/leases, chattel registry,
patent/trademark — see §7). Treat IG as "the grammar can hold it," BR as "the grammar can hold the
record but a rail must close it," GAP as "the grammar itself has no clean form yet."

> **Do not use 550 / 450 / 73 / 27.** That headline (and the "≈509/550 ≈93% realized" claim) is the
> inflated synthesis figure corrected in §7. The authoritative base is **476 = 378 / 72 / 26.**

---

## 4. Gap Roadmap (the principled 26)

The gaps are not random holes — they cluster into six principled classes, each a place where the
agency-free / record-only design *deliberately* stops. Commerce (7) and Health/Physical (6) carry the
load.

| # | Gap class | Representative gap-needs-design functions | Why it is a genuine gap |
|---|---|---|---|
| **A** | **Fair-exchange / atomic settlement** | Clearing & multilateral netting · order-book / matching engine · auction / bid-ask price discovery · balance/solvency · bearer/cash-like value · dynamic pricing | The CC 1.7 *atomic-exchange boundary*: holding funds and conditionally releasing them (escrow / DvP) is the one thing a record-only substrate cannot self-perform. Pledges/bids are IG; the **custody + conditional-release engine is the gap.** |
| **B** | **Physical actuation** | Grid / SCADA real-time control · autonomous-vehicle command & control · (home-automation actuation, the actuation half of scene triggers) | The substrate can attest a command; it cannot *move the actuator*. The unbuilt **effect-rail** (§4.D of the design notes) and physical actuation are out of grammar. |
| **C** | **Sensor / real-time streaming ingest** | Sensor telemetry ingestion · environmental sensor networks · telematics / trip telemetry · grid SCADA | High-rate, signed-but-unattested sensor streams have **no CEG object** for the stream-of-readings; the oracle problem (is the reading true?) is structural. |
| **D** | **Secret / coercion-resistant voting** | Secret / coercion-resistant ballot | The grammar is signed-and-attributable by construction; **ballot secrecy + receipt-freeness** is the antithesis of an attestation and needs dedicated cryptographic design. |
| **E** | **Proof-of-personhood** | Proof of unique personhood · biometric access/matching · (exam/quiz proctoring) | RATCHET/NodeCore give a *bounded* Sybil signal; a **hard one-human-one-key** guarantee is not derivable from attestations alone. |
| **F** | **Native property / title (record↔dirt binding)** | Anchoring record to physical object/parcel · will execution / dead-man's-switch · (land-title, easements, chattel, patent/trademark — IG record shapes whose binding+legal force is a gap/bridge) | The record shape is expressible; **binding it to the actual object and conferring legal title (state force) is the gap+bridge.** The oracle problem again, plus §5. |

The remaining gaps (realtime convergent state / CRDT-OT, collaborative editing, live co-presence,
presence status, DRM/copy-prevention, recurring-event RRULE, search ranking) are **synchronous
convergent shared-state** design gaps — only the sealed-chunk *transport* is in-grammar; the shared
mutable convergence layer is app-level and unbuilt.

> **Correction (gap class A — fair exchange).** A follow-up verification against CC 0.6
> ([`CC1.7-fair-exchange.md`](CC1.7-fair-exchange.md)) **demotes class A out of "principled refusal."**
> Fair exchange / barter is **not** on the apophatic floor (CC 3.1.5.4) and *serves* M-1; the EGL
> impossibility CC 1.7 cites assumes *no trusted third party*, which CIRIS guarantees by design (named-
> moderator invariant CC 4.5.4, WAs CC 4.3). The **optimistic** form composes in-grammar with no new
> primitive — bilateral ratification (CC 3.3.5) + a steward-bound escrow custodian (the CC 4.4.3.2
> `archive_custody` pattern) + WA dispute resolution + `slashing`/`stake`. **Only the trustless atomic
> swap (HTLC-class) stays out-of-grammar**; the value leg (CC 3.3.10) and physical delivery bridge as
> for any commerce. Most of Commerce/Money is therefore **in-grammar by composition**, and CC 1.7's
> target should be narrowed (a candidate CC 4.5.1 amendment — CC 1.7 is not entrenched).

---

## 5. Honest Bridges (the 72)

Bridges are functions whose **record is in-grammar but whose force requires an external rail.** Three
rail families, concentrated in Commerce (22 bridges):

1. **Settlement rail (money actually moving).** Payments, payouts, refunds, settlement
   receipt/proof-of-payment, crowdfunding custody, loyalty/store-credit, IP-royalties. The substrate
   records the *claim* of payment (self-authenticated: the signer controls the wallet) but **does not
   verify the transaction cleared** — that needs the rail. A signer can emit a false "I paid."

2. **Legal-force / fiat-identity rail (state or authority recognition).** Birth/death registration,
   name/gender amendment, adoption/guardianship transfer, KYC to a legally-sufficient level, age
   verification at the verified/government rung, external-identity/OAuth binding, real-property title,
   patent/trademark grant of monopoly. The substrate holds the **record**; an external verifier or
   state confers the **recognition**.

3. **Physical / hardware-anchor rail.** Routing & mesh transport ("reachability is never trust"),
   hardware-quote attestation chains (TPM-quote / FIDO), physical access grant, demand-response.
   Reachability and hardware class are *self-asserted producer claims* until an external anchor
   verifies them.

**Bridge honesty rule:** wherever the doc elsewhere admits a rail (settlement, KYC-to-legal,
hardware-quote), the sibling "self-auth / no-oracle / verified-level" claim for the *same* capability
must be read as **record-IG, force-BR** — see the §7 corrections.

---

## 6. The Deferred Agent Layer (22)

Recorded for completeness; **explicitly excluded from the agency-free fabric.** The fabric provides the
substrate these consume; it performs none of the reasoning.

- **Decision reductions:** PDMA (observation→decision), CSDMA / DSDMA / IDMA, Sunset/mini-PDMA.
- **Conscience faculties:** entropy / coherence / optimization-veto / epistemic-humility, the
  AgencyErosionDetector, apophatic `prohibited:*` NEVER_ALLOWED floor, Order-Maximisation Veto.
- **Deferral:** Wisdom-Based Deferral + Deferral Package, incompleteness-awareness /
  forecasting-error → WBD triggers.
- **Values:** the Six Foundational Principles (Autonomy, Non-maleficence, Integrity, Beneficence,
  Fidelity & Transparency, Justice); `accord-principle` / `dma-verdict` / `conscience-verdict`
  prefixes (wire-attestable as objects, *agent* in meaning).
- **Heuristics & process:** Recursive Golden Rule, prioritisation heuristic, reflective refinement,
  proportionality check, explainability-SLA, continuous monitoring/self-assessment, bilateral
  ratification (CEM handler), WA quorum adjudication (substantive judgment), semantic enrichment over
  degraded tiers, agent-facing chapters 1–9, Case Studies 1–7.

**Boundary seams (10 open-items, for a human call):** the same *object* sits in two layers depending on
*who composes it* — e.g. `delivery_receipt` verdict (fabric-validated receipt vs consumer-policy
"owes N"), `verdict composition` 2.5 (policy=fabric vs PDMA=agent over identical inputs), bilateral
ratification (substrate admits each consent half; an agent handler enforces ratification), moderation
keep/remove (delegation+gate fabric; the harm judgment agent), WA quorum adjudication, the T-3
expressive-gap test, the Order-Maximisation-Veto fabric-vs-agent ambiguity, and two acronym collisions
(WBD = Wisdom Bank DB vs Wisdom-Based Deferral; CRE = Catastrophic-Risk Eval vs Continuous-Refinement
Env).

---

## 7. Honesty-Audit Caveats

This synthesis previously shipped an **inflated headline** that this object model corrects. Audit
verdict: **`passed: false`.** The machine form (`meta.counts`) carries only the honest JSON-derived
numbers (**476 = 378 / 72 / 26**). Each issue below is logged as a binding correction.

**C-0 — Headline inflation (the load-bearing one).** Claimed *"550 functions, 450 in-grammar (81.8%),
73 bridge, 27 gap."* **Reality:** `civilizationalFunctions.json` and the markdown Expr column both
enumerate **476 rows = 378 IG / 72 BR / 26 GAP.** The headline adds **~72 phantom in-grammar
functions.** Inflation concentrates in **Social** (ledger 77/69 vs actual **49/41**) and **Knowledge**
(ledger 87/78 vs actual **46/39**), plus **Health** (IG 50 vs **46**) and **Commerce** (IG 41 vs
**40**). The §2.3/§2.6 headers literally read "77 — deduped" / "87 — deduped" yet the ledger used the
**pre-dedup** Functions+IG counts while leaving BR/GAP at the **post-dedup** row level (Social BR/GAP
5/3 matches actual exactly) — selective accounting that inflates the in-grammar percentage.

**C-1 — "Realized" silently means "expressible," not "built."** Claimed *"≈509/550 (≈93%) realized by
an existing CIRIS component."* **Reality:** computed on the inflated 550 base; **31 rows carry `GAP` in
the component column** and **~7 IG rows** (voice/video call, conferencing, livestream broadcast,
telemedicine, live lecture, stream anti-equivocation) depend on **Phase-2 / unbuilt CIRISEdge
streaming** yet are counted as realized.

**C-2 — Real-property land-title registry (§2.8) tagged IG.** Its own component column says
*"GAP (unbuilt subject_kind)."* Only the record *shape* is in-grammar; binding the record to actual
dirt (the oracle problem — separately tagged GAP at "Anchoring record to physical object/parcel") and
conferring legal title (state force) is a **GAP + a legal bridge.** Should not be IG.

**C-3 — Movable-asset/chattel registry · Patent/trademark registry & priority · Easements/leases
(§2.8/§2.4) tagged IG with `component = GAP` (unbuilt).** Patent/trademark even admits *"legal grant of
monopoly external"* yet is IG not BR. IG here conflates "record shape is expressible" with "function
delivered." Aspirational subject-kinds, not in-grammar coverage.

**C-4 — Whiteboard / live cursors / multiplayer (§2.2) tagged IG** "convergence app-layer" — the exact
reason the adjacent **Collaborative editing (Docs/Figma)** and **Realtime convergent state (CRDT/OT)**
are GAP and **Live co-presence / shared rooms** is GAP. Synchronous convergent shared state is a
genuine design gap; only the sealed-chunk transport is IG. **Inconsistent — should be GAP.**

**C-5 — KYC / identity verification (§2.4) tagged IG.** §5's own bridge table lists "KYC to legal
level" as a fiat/identity-anchoring bridge, and siblings (External-identity/OAuth binding, Age
verification) are BR. The L1–L5 ladder composition is IG, but **legally-sufficient KYC bridges to an
external verifier.** Over-claim.

**C-6 — Age verification (verified level) (§2.7) and Safety/age/identity verification (§2.3) tagged
IG** — directly contradict "Age verification" and "Age-assurance gating" (§2.1/§2.2), which are BR
because the provider/government rung needs an external verifier. **The verified/government level IS the
bridge case.**

**C-7 — Hardware-rooted key custody (§2.8) tagged IG.** Relies on
`attestation:hardware_rooted` / `hardware_class`, which "Hardware-rooted device attestation" marks GAP
("no TPM-quote/FIDO chain verification, R5"); §5 lists hardware-quote chains as a fiat-anchor bridge.
The class is **self-asserted today** — should be GAP/BR.

**C-8 — Home-automation rules / scene triggers (§2.7) tagged IG.** The function exists to trigger
**physical scenes**; the row itself says "actuation→effect-rail," the effect-rail is an unbuilt design
gap, and physical actuation is a bridge class. Siblings (Device command, Physical access grant,
Demand-response) cross out as BR/GAP. Only rule-as-data evaluation is IG; **the actuation is GAP/BR.**

**C-9 — Birth registration = BR but Death registration / Name-gender amendment / Adoption-guardianship
transfer = IG (§2.8).** All four are civil-registry records whose legal force is external. Birth is
correctly BR ("legal force external"); the other three carry **identical external-legal-force
structure** (succession/estate, legal name change, custody transfer) yet are IG. **Should match birth:
record IG, legal recognition bridges.**

**C-10 — Settlement receipt / proof-of-payment (§2.4), and "Identity = Wallet … proves I paid, no
oracle" (§2.8), tagged IG "self-auth (signer controls wallet)."** Controlling the wallet authenticates
**identity, not that the referenced tx exists or cleared** — a signer can emit a false "I paid."
Recording the receipt-claim is IG; **"proof-of-payment / no-oracle" overstates** — verifying the actual
payment needs the rail.

**C-11 — Crowdfunding / donations / pledges (§2.4) tagged IG ("contributions bridge").** Hedge label.
All-or-nothing crowdfunding requires **holding funds and conditional release = escrow = the CC 1.7
atomic-exchange boundary.** Pledge record is IG; **the defining custody/conditional-release function is
a bridge.** Loyalty/store-credit and IP-royalties carry the same "IG with value-leg bridges" hedge.

**Net effect of the corrections:** the in-grammar surface is **~14% smaller** than the inflated
headline and several IG tags should be demoted (≥10 functions toward BR/GAP). The object model ships the
**conservative** numbers; this caveat record is the audit trail.

---

## 8. Conformance

- **Schema:** all 856 records validate against `schema.json` `$defs` — required fields present;
  `layer ∈ {fabric, agent, boundary}`; `tier ∈ {node, owner-self, community, federation,
  constitutional, meta, multiple}`; `repo ∈ {persist, verify, edge, nodecore, registry, server, agent,
  lens, constitution}`; `expressibility ∈ {in-grammar, bridge, gap-needs-design}`; no unknown
  properties; `kind` consts correct; no duplicate ids. **0 violations; 0 unfixable.**
- **Authoritative numbers** live in `cewpos-object-model.json → meta.counts`. Any prose figure that
  disagrees (notably the deprecated 550/450/73/27 headline) is wrong by construction; see §7.
- **Layer invariant:** 0 agent-layer objects, 0 agent-layer transformations, exactly 1 agent-layer
  knob (the pluggable LLM provider) — the fabric is agency-free as designed.
