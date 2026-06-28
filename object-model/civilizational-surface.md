# CEWPOS — the Civilizational Surface

*The claim, tested: an agency-free CEG fabric — **1 emit + 4 composers** over an **open vocabulary** — can express what the internet and civilization require. Domain-by-domain, with honest gaps and bridges.*

---

## 1. THE THESIS

Civilization has an unbounded number of *domains* (medicine, land title, peer review, payroll, elections, CSAM removal, smart meters). CEWPOS asserts they all reduce to a **closed** structural core plus an **open** namespace:

- **The closed 5 ops** — `scores` (emit a signed, dated, dimensioned claim — the workhorse) + the four composers `supersedes` / `withdraws` / `recants` / `delegates_to`. Nothing else is ever a primitive.
- **The open vocabulary** — dimensions (`licensure:*`, `consent:*`, `provenance:*`, `detection:*`…), `subject_kind`s (`identity_occurrence`, `community`, `family`, `partner_record`, `settlement`, `takedown_notice`…), and **consumer-side composition** (eigentrust, attestation ladders, quorum tallies, thread traversal).
- **The move:** *a new domain is a new namespace, not a new primitive* (CC 1.7 / CC 4.5.1). Land title, a vaccination record, and a Discord channel are the same 1+4 grammar wearing different vocabulary.
- **Agency-free first:** the fabric only **records, composes, and revokes trust claims**. It never reasons, ranks-by-meaning, or acts. The agent (CIRISAgent) is excluded by construction and *plugs in last*; ranking/personalization/diagnosis are deferred to it or to consumer policy.

**The one honest out-of-grammar boundary.** CEG attestations are **unilateral and monotonic**. Therefore **atomic fair-exchange / bilateral simultaneity** (pay ⟺ release, DvP, atomic swap, escrow) is *classically impossible* in-grammar (Even–Goldreich–Lempel; CC 1.7). It **bridges** to an external settlement rail (USDC-on-Base / x402; `settlement` subject_kind, CC 3.3.10): **the fabric records the after-the-fact trust claim; the rail performs the atomic act.** Every "money moves," "physical thing actuates," "state compels," or "fiat anchors" function inherits this same honest boundary. *That it happened* is in-grammar and permanent; *the atomic/physical/legal effect itself* is the rail's job.

Three honest verdicts per function: **IG** = in-grammar · **BR** = bridges to an external rail · **GAP** = needs-design (no clean expression yet, or an unbuilt component/subject_kind).

---

## 2. THE MAP

### 2.1 The Internet's own substrate layers (55)

| Function | CEG objects | Transforms | Expr | Component |
|---|---|---|---|---|
| Root identity / account | `federation_keys`, Ed25519+ML-DSA-65, `identity_occurrence` | scores · delegates_to · supersedes/withdraws | IG | Verify+Persist+Registry |
| Naming / DNS-replacement | `community_key_id`, signed Contribution, `transport_destination`, RNS `destination_hash` | resolve_community/member; WHO=signed / WHERE=mesh split | IG | Persist/Server+Edge |
| Multi-device account / sync | `identity_occurrence`, DeviceClass, `encryption_pubkeys` | self/single-vouch admit; DEK auto-wrap to occurrences | IG | Verify+Persist |
| PKI / CA / trust root | `ciris-canonical` infra community, attestation ladder | founder-quorum 2/3; delegates_to chains (5-hop); **re-rootable** | IG | Verify+Registry |
| Key rotation / crypto-agility | supersedes occurrence, `wrap_algorithm` v1→v2→v3 | rotate off stable signing key; fail-secure on unknown alg | IG | Verify |
| Recovery / guardianship / stewardship | `delegates_to`, `is_steward_bound`, minor/incapacity rulings | steward binding; fail-secure: steward-less ⇒ no operate | IG | Registry+Verify |
| External-identity / OAuth binding | `identity:canonical_binding:{H}`, canonical-hash subjects | self-asserted scores; proof-of-control = consumer policy | **BR** | Verify+NodeCore (+IdP) |
| Addressing (authenticated IP-equiv) | `TransportDestination`, AV-17 layer-split | fed-key-signed binding; recompute hash; announce=advisory | IG | Edge |
| Routing / mesh transport | Reticulum/Leviculum, HF/NVIS store-fwd | NOT a primitive — "reachability is never trust" | **BR** | Edge (external mesh) |
| Liveness / presence / NAT | Edge-local reachable set, D6 invariant | fan_out = entitled ∩ reachable; never logged/replicated | IG | Edge |
| Content-addressed storage (IPFS-equiv) | SHA-256 `evidence_refs`, `holds_bytes:sha256:*` | auto-emit holds_bytes; full-SHA verify before consume | IG | Persist+Edge |
| Durable corpus / ledger | `federation_attestations`, two-tier local/fed | local O(1) → fed promotion pays one hybrid sig; idempotent dedup | IG | Persist+Server |
| Encryption at rest | `key_grant` wrapped DEK, 3 crypto tiers | substrate-wraps per recipient; fail-secure exclude (no plaintext fallback) | IG | Persist+Verify |
| Access control to data | `cohort_scope`, `GrantScope`, `consent:scope` | CC 5.2 target-gate ∧ tier-gate; key_grant cascade | IG | Persist+NodeCore |
| Deletion / erasure / GC | `consent:state:revoked`, `deletion_sla`, EvictionSweeper | withdraws (forward-only); 24h fed promote; SLA watcher | IG | Persist+NodeCore |
| Compute integrity / provenance | `provenance:slsa`, `attestation:agent_integrity` L1-L5 | boolean-via-score (min-agg); per-locale Merkle | IG | Verify+Registry |
| Compute execution | CIRISServer node, `health:liveness` | execution is node behavior; CEG attests *around* it | IG | Server+LensCore |
| Code/skill import & sandbox | `provenance:skill_import`, SkillImportManifest v2 | hybrid sig over canonical bytes; sandbox node-side | IG | Verify |
| Web-of-trust / reputation | eigentrust, `capacity:*`, `coherence_standing` | delegates_to (depth5/0.5×/cycle-reject); polarity-default agg | IG | LensCore+NodeCore |
| Transport security (TLS-equiv) | two-layer E1, X25519+ML-KEM-768 KEX, RNS Links | key material in-grammar; link crypto rides Reticulum | **BR** | Edge+Verify |
| Cert-transparency / public log | RFC 6962 Merkle, STH, `transparency_log:*` | witness cosign w/ consistency proof; per-stream STH | IG | Verify+Persist+Registry |
| Revocation (CRL/OCSP) | `revocation:{type}:{reason}` (-1), `rollback_detected` | withdraws vs recants; monotonic_quorum; restrictive-wins | IG | Registry+Verify |
| Licensing / authorization grants | `licensure:*`, `partner_record`, capabilities set | M-of-N steward quorum; single-source conf ≤0.5 | IG | Registry |
| Sybil resistance / bonding | `bond_posted`, `ratchet:flag:*`, `witness_diversity` | scores + advisory flags (never sole); capital on rail | **BR** | Registry+RATCHET (+chain) |
| Service / peer discovery | holds_bytes dir, steward endpoints, `need:*` | scores+subject_kind; cold-start = pin key + seed hash | IG | Registry+Persist |
| Content discovery / search | open-prefix `dimensions[]`, 5-op query, `topical_relation` | prefix match; **ranking = consumer-side** | IG | Persist+LensCore |
| Feeds / timelines | `external_content` sub_kinds, `topical_relation` | tiered-scope compose; thread traversal consumer-side | IG | NodeCore |
| Time / timestamps / freshness | `asserted_at`/`valid_until` RFC3339 | ±5min skew bound; no global clock; valid_at filter | IG | all (canonicalization) |
| Ordering / convergence | MergeBallot (quorum→time→hash) | recants>withdraws>supersedes; no global ledger | IG | Server+Persist |
| Causality / lineage | `references_attestation_id`, supersedes, `rotation_chain` | 8 inter-attestation relations; hash-chain continuity | IG | Persist+Verify |
| Anti-equivocation / fork detect | STH consistency, `detection:cross_agent_divergence` | witness verify vs prior STH; median-agg detectors | IG | Verify+LensCore |
| Value transfer / payments rail | `settlement`, `settlement_ref`, Identity=Wallet | **not a primitive** — rides USDC/x402; CEG never reverses | **BR** | Financial+Billing (+chain) |
| **Atomic fair-exchange** | `settlement` as bridge target, `settled_action_ref` | **the named CC 1.7 falsification target** — impossible in-grammar | **BR** | Financial (+chain) |
| Billing / subscriptions / metering | `settlement` receipt, SubscriptionTier | billing OFF-WIRE; substrate rejects processor IDs | **BR** | Billing (+Stripe) |
| Non-monetary governance weight | `credits:*` (non-transferable), `expertise:*` | truth-grounding accrual; vote = credits×expertise | IG | NodeCore |
| CDN / caching / replication | `holds_bytes:*` multi-holder, 24h TTL | auto-emit; ≤2 holders parallel; ContentMiss→downweight | IG | Edge+Persist |
| Cache eviction / storage policy | EvictionSweeper, `archive_mode` | informed keep/evict; rotate-forward = key deletion | IG | Persist |
| Cross-region sync / anti-entropy | replication wire, per-subject merge intents | dispatch on declared policy; partition-tolerant | IG | Edge+Persist+Server |
| Conflict resolution / merge | merge intents, MergeBallot, stable-id grouping | skew-bounded; quorum>time; withdrawal forward-only | IG | Server+Persist |
| Group key agreement (re-keying) | MLS TreeKEM (RFC 9420), epoch-DEK | O(log N) path rekey; ≤2s removal-coalescing; Option-A | IG | Persist+Verify |
| **Realtime convergent state (CRDT/OT)** | data-stream chunk, per-(stream,epoch) DEK | transport IG; **merge/convergence app-layer, NOT a primitive** | **GAP** | Edge (transport) + app |
| Messaging / pub-sub / streaming | `live_stream`, observer-share, SealedAvChunk | delivery axis; epoch DEK; pull-only RC1 | IG | Edge+NodeCore |
| Realtime comms (WebRTC/VoIP) | group-comms composition, ephemeral rosters | N↔N = N live_streams; Reticulum mesh; **zero new primitives** | IG | Edge |
| Delivery receipts | `delivery_receipt:{stream}`, per-stream STH | subscriber scores; validated-not-adjudicated | IG | Edge+Verify |
| Moderation / takedown | `takedown_notice` (LegalBasis enum), `watchlist` | fast-path eviction; per-group never global; legal force external | **BR** | NodeCore+Registry (+legal) |
| Child safety / CSAM | `PerceptualHashCsam`/`NcmecCsam`, apophatic floor | immediate eviction; cannot reach self/family; NCMEC external | **BR** | Registry+NodeCore (+NCMEC) |
| Age verification | `age_assurance:{self|provider|government}` | self rung IG; provider/gov rungs need external verifier | **BR** | NodeCore+Registry (+verifier) |
| Consent / GDPR-DSAR | `consent` namespace, `subject_key_ids` | subject scores; withdraws rule 2/3; 24h fed promote | IG | NodeCore+Persist |
| Geolocation | `location_proof`, H3 cell ≤res7 (rough-only) | cascade-by-containment; substrate doesn't verify truth | IG | NodeCore |
| Emergency broadcast | `news_article`+`emergency_authority`, `event:lifecycle` | pre-cross-attested authority bypass; cascade-by-containment | IG | NodeCore |
| Kill-switch / halt authority | `HUMANITY_ACCORD` entrenched, `accord:invoke:CONSTITUTIONAL` | 2/3 live-quorum; wire+scope isolated; fire-floor=1 | IG | Registry+Server+Persist |
| Governance / voting / deferral | `vote:*`, `weighted_aggregate`, `hard_case:*` | locality-scaled quorum; subsidiarity; WA quorum load-bearing | IG | NodeCore |
| Health / observability | `health:liveness`, `system:*` reserved | scores (witness=external); infra folds as evidence_refs | IG | LensCore+Persist/Edge |
| Software supply chain | `build:registered`, `agent_files`, BuildManifest | SHA-addressed; per-locale Merkle sub-manifests | IG | Registry+Verify+Edge |
| **Hardware-rooted device attestation** | `attestation:hardware_rooted`, `hardware_class` | binding IG, but **no TPM-quote/FIDO chain verification (R5)** | **GAP** | Verify (roadmap) |

### 2.2 Communication & Media (54)

| Function | CEG objects | Transforms | Expr | Component |
|---|---|---|---|---|
| Direct message 1:1 | `chat_message` self/family, N=1 observer-share key_grant | per-write DEK; structural invisibility; supersedes=edit | IG | Edge+NodeCore+Persist |
| Group messaging | `chat_message` community, epoch-DEK cascade | Policy M; O(1) seal/O(N) key_grant; Option-A rekey | IG | NodeCore+Edge+Persist |
| Threaded conversation | `topical_relation:replies_to/comments_on` | consumer-side thread-graph traversal | IG | NodeCore |
| Microblog post | `chat_message` (the slot microblog rides) | Policy H tiered-scope promotion | IG | NodeCore |
| Reactions / engagement | bare scores, `vote:*` analog | polarity agg; consumer tally; withdraws to un-react | IG | LensCore+NodeCore |
| Delivery/read receipts | `delivery_receipt`, D6 invariant | heartbeat-suppression; missed→pull-on-reconnect | IG | Edge |
| Presence / typing indicators | D6 reachable, Edge reachability tracker | **never attestation/replicated/logged** (ephemeral) | IG | Edge |
| Edit / recall / delete | supersedes/withdraws/recants | forward-only; can't un-send held bytes | IG | NodeCore |
| Ephemeral / disappearing | `valid_until`, `consent:stream:temporary` | rotate-forward key deletion (not crypto guarantee) | IG | Persist |
| Email async store-forward | `chat_message` long-form, MX-free addressing | pull-on-reconnect; *no canonical `email` sub_kind (vocab gap)* | IG | Edge+NodeCore+Bridge |
| Mailing list / newsletter | community roster, `news_article`, push | subscribe=join; fan-out entitled∧reachable | IG | Edge+NodeCore |
| Voice call 1:1 | `live_stream` audio, ephemeral community, epoch DEK | chunk-seal; direct Reticulum Link | IG | Edge *(Phase-2)* |
| Video call group | per-participant `live_stream`, channel community | N↔N streams; direct-link mesh ≤~50 | IG | Edge (SFU→1.x) |
| Conferencing / webinar | ephemeral community, SVC/MDC codec_id | per-receiver bandwidth drop to BLINKING_DOT | IG | Edge+NodeCore (SFU→1.x) |
| Screen sharing | `live_stream` from capture, ChunkLayer | relay drops chunks w/o decrypt (clear metadata) | IG | Edge |
| Voice channels / spaces | persistent + nested community | multi-level membership gates | IG | Edge+NodeCore |
| Livestream broadcast | `live_stream`, public-vs-gated | pull-only RC1; public-broadcast no-rotation exemption | IG | Edge+Persist *(Phase-2)* |
| Stream anti-equivocation | per-stream log (log_id=stream_id), STH | witness cosign per-epoch; locality quorum | IG | Verify+NodeCore *(pending)* |
| Social feed / timeline | local/community/global surfaces, Policy H | cohort-weighted compose; cross-cohort downweight | IG | NodeCore+LensCore |
| Feed ranking / "for you" | LensCore scores, eigentrust, manifold_conformity | **ranking algo = consumer/agent policy** | IG | LensCore |
| Follow / subscribe graph | community join, `consent:replication`, `need:*` | admission; withdraws to unfollow | IG | NodeCore |
| Hashtags / topics / channels | `topical_relation` open-vocab, community=channel | open-vocab edges; consumer aggregation | IG | NodeCore |
| Blog long-form | `blog_post`, `blog:*` | tiered-scope; comments via topical_relation | IG | NodeCore |
| Encyclopedia / wiki | `encyclopedia_article`, indefinite valid_until | editor-consensus; revision via supersedes | IG | NodeCore+LensCore |
| News / journalism | `news_article`, publisher partner_role | time-decay; corrections via recants | IG | NodeCore+Registry |
| Corrections / retractions | recants, `topical_relation:corrects` | recants penalty (≠ withdraws-arbitrage) | IG | NodeCore |
| Citizen journalism / whistleblower | `news_article`, empty subject_key_ids | substrate-protective; blocks bad-actor takedown | IG | NodeCore+Registry |
| Music / audio streaming | `audio` sub_kind, Source struct | ContentFetch; SubscriptionTier; paid→bridge | IG | NodeCore+Edge+Persist |
| Podcasts | `audio`, subscription feed, transcript | push/pull; episodes via supersedes | IG | NodeCore+Edge |
| Image / photo / art / 3D | `image`/`model_3d`, mandatory alt_text | cohort gradient keeps private at self/family | IG | NodeCore+Edge+Persist |
| AI-generation disclosure | `is_ai_generated`, `content_class:generated` | producer-declared; consent:train opt-out | IG | Verify+Proxy |
| Video-on-demand / film | `video`/`film`, distributor chain | key_grant; PPV→settlement bridge | IG | NodeCore+Edge |
| Content rating / classification | `content_rating:{scheme}`, `content_class` | multi-scheme; consumer gate; not slashing input | IG | NodeCore+Registry |
| Captions / subtitles / l10n | mandatory Source fields, per-locale Merkle | translation edges; locale-attack detection | IG | Verify+NodeCore |
| **Collaborative editing (Docs/Figma)** | data-stream chunks, epoch DEK, opaque codec | transport IG; **CRDT/OT merge app-layer** | **GAP** | Edge + Game/app |
| Whiteboard / live cursors / multiplayer | realtime non-A/V data-stream | "a chunk is just a chunk"; convergence app-layer | IG | Edge+Game |
| Push notifications | push mode, Policy M, D6 | fan-out entitled∧reachable; pull-fallback | IG | Edge |
| Notification prefs / muting | `consent:*`, withdraws | leave/withdraw → stop wrapping (forward-only) | IG | NodeCore+Edge |
| Content storage / CDN | SHA blobs, holds_bytes | PeerResolver; verify full SHA | IG | Persist+Edge |
| Moderation / takedown (DMCA/DSA) | `takedown_notice`, LegalBasis, counter-notice | fast-path; CourtOrder→legal-force bridge | **BR** | NodeCore+Registry+Legal |
| Child safety / CSAM | `NcmecCsam`/`PerceptualHashCsam`, hash-DB | immediate eviction; GIFCT; hash-DB external | **BR** | Registry+NodeCore (+DB) |
| Content watchlist matching | `watchlist:{id}` per-group | auto-fire at publish seam; never global/self-family | IG | NodeCore |
| Age-assurance gating | `age_assurance:{level}`, AvmsdAgeInappropriate | self IG; government rung→external verifier | **BR** | Verify+Registry |
| Content warnings / sensitivity | `cw_class:{class}` cohort CW | Frickerian non-downweight low-density | IG | NodeCore+LensCore |
| Speaker identity auth | `identity_occurrence`, canonical_binding, ladder | single-vouch admit; L1-L5 compose | IG | Verify+Registry |
| DNS-free addressing | `transport_destination`, RNS two-stage hash | fed-key-signed binding replaces DNS+IP | IG | Edge+Verify |
| E2E encryption / privacy scoping | epoch DEK, ML-KEM-768, 3 tiers | TreeKEM rekey; *no auto re-encrypt of past (CC 3.3.6.1 gap)* | IG | Verify+Persist+Edge |
| **Monetization / paywall / tips** | `settlement`, SubscriptionTier access | access IG; **atomic key-for-payment bridges off-wire** | **BR** | Billing/Financial+NodeCore |
| **DRM / copy-prevention** | `key_grant`, rotation_chain | access IG; **hard copy-prevention has no clean expression** (honest-holder) | **GAP** | NodeCore+Verify (access IG) |
| Events / RSVP / ticketing | `event_listing`, ticket_grant_policy | RSVP=scores; transfer=delegates_to; paid→bridge | IG | NodeCore |
| Spam / Sybil / abuse | `bond_posted`, `ratchet:flag:*`, track-record | advisory flags never sole; WA quorum gate | IG | RATCHET+NodeCore+Registry |
| Censorship-resistant offline | Reticulum, HF/NVIS store-forward | "signature is trust not path"; pull-on-reconnect | IG | Edge |
| Audit / transparency logs | RFC 6962, cosigned STH | inclusion/consistency; hash-chain continuity | IG | Verify+Persist |
| Search / indexing | open-vocab query, holds_bytes dir | prefix match; full-text = consumer concern | IG | LensCore+Persist |

### 2.3 Social, Relationships & Community (77 — deduped)

| Function | CEG objects | Transforms | Expr | Component |
|---|---|---|---|---|
| Personal identity & profile | `identity_occurrence`, profile sub_kinds | self-attest single-vouch; supersedes/withdraws | IG | Verify+Persist |
| Follow (asymmetric) / repost / mention | directed scores edge, `subject_key_ids`, `topical_relation` | withdraws to unfollow; named-subject retains consent | IG | NodeCore+LensCore |
| Friend (symmetric) / mutual match | `partnership_grant/accept`, `bilateral_pair` | ratify iff both live; recants to de-friend | IG | NodeCore+Persist |
| Unfollow/unfriend/leave | the prior edge | withdraws forward-only; recants if error | IG | Persist |
| Block | absence of `consent:replication`, revoked | honest-holder cessation (not crypto against determined peer) | IG | Server + consumer |
| Mute / hide / filter / label | consumer filters over scores label graph | stackable/swappable; may escalate to finding | IG | LensCore + consumer |
| Audience lists / close-friends | `family` (≤20), cohort_scope, per-cohort DEK | scoped scores; family suppresses holds_bytes | IG | Persist+NodeCore |
| Follower / social-proof aggregation | directed follow-edge set, published-aggregate | eigentrust; count-vs-edgeset divergence = anomaly | IG | LensCore |
| Family / household formation & change | `family` subject_kind, `hard_case:family_*` | consensus_protocol; Option-A rekey; structural invisibility | IG | Persist+Server |
| Guardianship / minor stewardship | `delegates_to(adult→minor)`, age band | steward-sig=agreement; fail-secure for unbound minor | IG | Registry+Verify+Persist |
| Shared household devices | family device identities | family admission; key_grant to family DEK | IG | Home+Persist |
| Community create / admit / govern / roles | `community`, consensus_protocol, MemberRole | supersedes ceremony; Policy M; delegates_to role-chain | IG | NodeCore+Server |
| Community moderation (required-mod invariant) | moderate duty, `community_unmoderated` | gate at admit+every fed step; merit auto-promote; fail-secure | IG | Server+NodeCore+Registry |
| Geographic / interest / professional community | cohort_subkind geographic/custom, location_proof | containment; partner_role for licensed | IG | NodeCore+Registry |
| Leaving community / forward secrecy | community DEK, `membership_change` | supersedes removal; Option-A rotation (no PCS) | IG | Persist |
| Community confidentiality + discovery | per-community DEK, holds_bytes provenance | wrap O(1); inspectable; infra→Commons-plaintext | IG | Persist+Edge |
| Formal orgs / affiliations / nested | `organization`/`org_membership`/`partner_record`, `multilateral_participation` | M-of-N quorum; delegates_to resolver; PII region-local | IG | Registry+Persist+NodeCore |
| Event create / RSVP / cancel / reschedule / attend | `event_listing`, `event:lifecycle`, `rsvps` | scores; withdraws=cancel; supersedes differs_in | IG | NodeCore |
| Ticketing / access / capacity | ticket_grant_policy, key_grant GroupMember | grant; capacity=consumer; **paid→settlement** | **BR** | NodeCore+Financial+Billing |
| Ticket transfer | the ticket-grant | **delegates_to** (named primitive) | IG | NodeCore |
| **Recurring events / RRULE** | repeated event_listings | **no native recurrence field** — operator/UI expand | **GAP** | operator/UI (external) |
| Event reminders | Edge delivery, valid_until | transport act, not attestation | IG | Edge |
| Dating profile / like-swipe / consent-to-contact | content sub_kinds, private directed scores | held at self until mutual; withdraws to un-like | IG | NodeCore+Persist |
| Matchmaking / recommendation ranking | edge corpus + coherence/eigentrust | substrate carries edges; **match brain=excluded agent** | IG | LensCore (match→Agent excl.) |
| Safety / age / identity verification | `age_assurance` (witness-reserved), `age_self_declared` | witness rung outranks self; L1-L5 ladder | IG | Verify+Registry |
| Endorsement / vouch / reviews / ratings | `expertise:*`, `testimonial_witness`, open-vocab scores | by OTHERS (never self-declared); recants to retract | IG | NodeCore+LensCore |
| Reputation / standing / verified badge / portability | `capacity:*`, `coherence_standing`, `licensure`, `credits` | never wire `score:trustworthiness` — composed downstream; credits non-transferable | IG | LensCore+NodeCore+Registry |
| Negative reputation / sanctions | `moderation:*`, `slashing:*`, `ratchet:flag:*` | WA quorum gate; flags never sole; reconsideration appeals | IG | NodeCore+RATCHET+Registry |
| Web-of-trust / transitive endorsement | `delegates_to` chains | depth-5, 0.5× cap, cycle-reject | IG | LensCore + consumer |
| Presence — reachability (DNS-free) | `transport_destination`, `peer_reachability` | signed binding; reachability ≠ trust | IG | Edge |
| **Presence — ephemeral live status** | transient Edge signal | **no durable attestation home** for typing-rate | **GAP** | Edge (no CEG object) |
| Read / delivery receipts / check-in | `delivery_receipt`, `location_proof` ≤res7 | member-emitter; validated-not-adjudicated | IG | Edge+NodeCore+Persist |
| Forum / threads / votes / polls / pinning / tags | community, `topical_relation`, `vote:*`, content_class | scores; consumer tally; moderator-signed curation | IG | NodeCore+Persist |
| Search / discovery (people/content/groups) | holds_bytes index, scores corpus | family scope structurally un-discoverable | IG | Persist+LensCore |
| Collective / group identity & reputation | community key, `coherence_standing`, partner_record | consensus authorizes group-signed acts; trust≠membership | IG | NodeCore+Verify+LensCore |
| Public-figure / official verification | identity_occurrence→service key, canonical_binding | consumer proof-of-control; competing→RATCHET | IG | Verify+Registry |
| Membership lifecycle / directory / onboarding / invites | community/family record, `resolve_community` | supersedes/withdraws; deterministic resolution; delegates_to invite | IG | NodeCore+Persist+Server |
| Membership tiers / paid subs | key_grant SubscriptionTier, settlement | **recurring payment bridges to rail** | **BR** | Billing+Financial+NodeCore |
| Gifting content / access | content scores + key_grant | no value transfer → IG | IG | NodeCore+Persist |
| Gifting value / tipping | `settlement`, `settlement:received` | self-auth via Identity=Wallet; **chain settles value** | **BR** | Financial (+rail) |
| Open call / mutual aid / need | `need:{domain}:{kind}` positive-only | supersedes/withdraws/recants lifecycle | IG | NodeCore |
| Gift ack / reciprocity | `commitment_fulfillment`, partnership | bilateral pair; truth_grounding track-record | IG | NodeCore+LensCore |
| DM / group channel messaging | `chat_message` self/family/community DEK | "community is a stream"; Option-A rekey | IG | Edge+NodeCore+Persist |
| Consent & data rights (DSAR/RTBF) | `consent_record`, `deletion_sla`, `decay` | subject revoke rules 2/3; multi-stage decay | IG | Persist+NodeCore |
| Reporting / abuse / child-safety | `takedown_notice`, `watchlist`, child-safety rulings | open-labeling IG; **legal takedowns→external regime** | **BR** | Registry+Server+Legal |
| Feeds / notifications ranking | edge corpus, Edge delivery | substrate carries edges; **personalization=excluded agent** | IG | LensCore+Edge |
| **Marriage / civil-partnership legal recognition** | family + bilateral partnership | social/crypto union IG; **legal recognition→registrar rail** | **BR** | Legal+Persist |
| Pseudonymous / anonymous participation | `consent:stream:anonymous`, decay, distinct KEM | decay severs; **full ZK unlinkability not yet shipped** | IG | Persist+Verify |
| **Live co-presence / shared rooms** | live_stream (Phase-2), durable via vod_of | **synchronous mutable room state lives outside append-only corpus** | **GAP** | Edge+Game (corpus=vod only) |

### 2.4 Commerce, Money & Markets (70)

| Function | CEG objects | Transforms | Expr | Component |
|---|---|---|---|---|
| Value-unit naming | open-vocab `settlement.rail`, currency tags | namespace declaration; no op consumed | IG | Registry+Financial |
| **Native value transfer (the rail)** | none on-wire; USDC/Base/x402, Identity=Wallet | **bridge** — CEG never holds/moves value | **BR** | GAP (rail) / Financial |
| Settlement receipt / proof-of-payment | `settlement` {settled_action_ref, rail, ref, amount_commitment} | self-auth (signer controls wallet); dispute=new settlement | IG | NodeCore+Persist+Verify |
| P2P payment / micropayment / tip | settlement vs paid Contribution | value bridges; receipt in-grammar | **BR** | Billing/rail + NodeCore |
| Remittance / cross-border | settlement {rail, ref}, jurisdiction fields | rail does FX+transfer; CEG records receipt | **BR** | GAP (rail) / Financial |
| Invoice / request-for-payment | consent_record / scores naming debtor, `topical_relation:settles` | claim IG; payment bridges | IG | NodeCore+Billing |
| Invoice↔payment reconciliation | settlement + settles edges | consumer-side traversal | IG | LensCore+Billing |
| Recurring payment / mandate / direct debit | `delegates_to(payer→payee, act_on_behalf)` | mandate IG; each pull bridges | **BR** | Billing+NodeCore |
| Refund / chargeback / reversal | new settlement vs original | supersedes/topical; rail reverses value | **BR** | Billing/rail+NodeCore |
| Disbursement / split / royalty fan-out | N settlement records | fan-out=consumer compose; each leg bridges | **BR** | Billing/rail+NodeCore |
| Account / wallet identity | fed signing key = wallet, identity_occurrence | key IS the account; no separate object | IG | Verify+Persist |
| Custody of assets / keys | `hardware_custody:*`, key_grant, archive_custody | key custody IG; asset value on rail | **BR** | Verify+NodeCore |
| Multi-sig / treasury control | family/affiliation `quorum:M/N`, accord triple | membership via supersedes; **co-signed move→multisig rail** | **BR** | NodeCore (+multisig rail) |
| **Balance / statement / solvency** | tally over settlement records | **no global ordered state — best-effort observation only** | **GAP** | GAP (rail ledger) / Billing |
| KYC / identity verification | attestation ladder L1-L5, partner_role, age_assurance | Policy I compose; canonical_binding | IG | Verify+Registry |
| Account recovery / inheritance / estate | incapacity steward-binding, capacity panels | M-of-N quorum; value still bridges | **BR** | Registry+NodeCore |
| Creditworthiness / scoring | scores, eigentrust, commitment_fulfillment | LensCore observe (advisory); agent reasoning excluded | IG | LensCore |
| Loan / debt instrument | consent_record terms, settlement series, bond escrow | terms IG; principal+interest+enforceability bridge | **BR** | NodeCore+Legal+rail |
| Repayment schedule tracking | settlement series + settles, commitment_fulfillment | each payment bridges; schedule IG | IG | NodeCore+LensCore |
| Default / delinquency / blacklist | negative scores, moderation, revocation | recants/withdraws on cure; quorum-gated | IG | LensCore+NodeCore+RATCHET |
| Collateral / lien / stake / bond | `bond_posted` (forfeit-on-revocation), escrow | revocation triggers forfeit; **value on rail** | **BR** | Registry+Verify |
| Marketplace / listing / catalog | event_listing/external_content, community | tiered feeds; promotion; holds_bytes discovery | IG | NodeCore+Edge |
| Order / purchase intent / quote / offer | consent_record/scores vs listing, bilateral_pair | bind offer↔acceptance; fulfillment bridges | IG | NodeCore |
| **Auction / bid-ask / price discovery** | scores as bids, ballot pattern | sealed/ascending tally IG; **continuous CLOB needs total-order+atomic match** | **GAP** | NodeCore (bids); GAP (engine) |
| Reviews / seller reputation | scores, testimonial_witness, eigentrust | Frickerian discipline; recants by witness | IG | LensCore+NodeCore |
| Dispute resolution / arbitration | moderation, reconsideration, witness_diversity | quorum+deferral; locality-scaled | IG | NodeCore+Legal |
| Bilateral / multi-party contract | consent_record `bilateral_pair_id`, consensus_protocol | two unilateral scores joined; ratify iff both live | IG | NodeCore+Legal |
| Terms / SLA / warranty | `deletion_sla` pattern, `explainability_sla` | SLA-breach watcher emits hard_case | IG | NodeCore+Persist |
| Signature / e-sig / notarization | hybrid-signed scores, witness_diversity, inclusion | RFC 6962 inclusion proof | IG | Verify |
| Contract amend / novate / terminate | the original contract | supersedes/withdraws/recants — entirely in 4 composers | IG | NodeCore |
| Supply-chain provenance / chain of custody | `provenance:*`, `chain_of_custody`, holds_bytes | each handoff via supersedes; processor_chain | IG | Verify+NodeCore+Persist |
| Lot/batch traceability & recall | chain_of_custody, takedown/recall notice | supersedes chain + consumer traversal | IG | NodeCore+Verify |
| Authenticity / anti-counterfeit | provenance hash-equality, ladder | boolean-via-score; consumer verdict | IG | Verify |
| Certification / labeling (organic, ISO) | `licensure:*`, certifier scores | revocation immediate; consumer trusts authority | IG | Registry+Verify |
| Subscription entitlement / paywall | `key_grant` {SubscriptionTier} | wrap DEK; rotation_chain; fail-secure exclude | IG | NodeCore+Edge+Verify |
| Metering / usage / pay-per-use | `delivery_receipt`, credit ledger off-wire | validated-not-adjudicated; billing off-wire | **BR** | Billing+Edge+Proxy |
| Subscriber-set management | community = subscriber set, Policy M | subscribe=join; forward-secrecy rekey | IG | NodeCore+Edge |
| Escrow custodian role | archive_custody, delegates_to scoped | relationship IG; **held value on rail** | **BR** | NodeCore+Verify |
| **Conditional / milestone release / atomic fair-exchange** | settlement + delivered Contribution | **CC 1.7 target** — bridges atomicity to rail | **BR** | GAP (rail) / NodeCore |
| **Atomic swap / DvP / PvP** | two cross-referenced settlements | bilateral simultaneity out-of-grammar → HTLC rail | **BR** | GAP (rail) / NodeCore |
| Insurance policy | consent_record + affiliation pool + settlement | terms IG; premium/payout bridge | **BR** | NodeCore+Financial+rail |
| Risk pool / mutual / captive | `affiliation` {membership_basis, compartments} | admission + consensus; DEK cascade | IG | NodeCore |
| Claim filing & adjudication | claim scores, testimonial, moderation | quorum + deferral; payout bridges | IG | NodeCore (+rail) |
| Underwriting / actuarial | scores, LensCore detectors (advisory) | observation only; pricing off-wire | IG | LensCore+Financial |
| Employment relationship | affiliation employment-contract, designated_officials | admission via consensus; term-bound delegations | IG | NodeCore |
| Salary / wage / payroll run | recurring settlements, delegates_to authorization | mandate+N receipts; **wage bridges** | **BR** | Billing/rail+NodeCore |
| Benefits / HR records | affiliation compartments, classification_scheme | sub-roster DEK; minimum-necessary; disclosure_accounting | IG | NodeCore+Verify |
| Work / time / deliverable attestation | scores, commitment_fulfillment, progress_measure | consumer compose; track-record accrues | IG | NodeCore+LensCore |
| Contractor / gig / labor matching | `need:*` open-call, settlement on completion | positive-only broadcast; payment bridges | IG | NodeCore |
| Tax identity / registration | `tax_id` region-local (never federates), partner_role | off-wire by design; status via scores | IG | Registry |
| **Tax calculation / withholding** | none — no compute primitive | **off-wire consumer policy** | **GAP** | GAP / operator |
| Tax reporting / mandated disclosure | `transparency_obligations`, lawful_access | declared, logged, never covert; filed data bridges | IG | NodeCore |
| Audit trail (tax / financial) | `audit_chain:hash_continuity`, inclusion/consistency, legal_hold | append-only; auditor verifies | IG | Persist+Verify |
| **Settlement finality** | settlement_ref (chain tx) | **finality = property of rail's ordered ledger** | **BR** | GAP (rail) / NodeCore |
| **Clearing / multilateral netting** | graph of obligation Contributions | **needs total-order + atomic multi-party settle** | **GAP** | GAP (rail) / NodeCore |
| Ownership / title registry & transfer | title scores, supersedes, provenance | record IG; **transferring exclusive bearer control bridges** | **BR** | NodeCore + rail |
| Crowdfunding / donations / pledges | consent_record pledges, public settlement | consumer tally; contributions bridge | IG | NodeCore + rail |
| Loyalty points / rewards / store credit | credits-shape (non-transferable) or key_grant | accrual IG; **bearer vouchers bridge** | IG | Billing+NodeCore |
| **Bearer / cash-like anonymous value** | none — every attestation identity-bound | **out-of-grammar by design** | **GAP** | GAP (rail/token) |
| Commons Credits (governance weight) | `credits:{domain}` positive-only | **non-transferable — explicitly NOT money** (why value bridges) | IG | NodeCore |
| Asset tokenization / RWA | provenance + chain_of_custody + token ref | trust-layer IG; **token+transfer on rail** | **BR** | Verify + rail |
| IP / content licensing & royalties | `licensure`, `consent:scope:[publish…]`, key_grant | license IG (revocable); royalty bridges | IG | Registry+NodeCore |
| Bankruptcy / insolvency / wind-down | `dissolves_at`, legal_hold, withdraws | auto-wind-down; **distribution+legal-force bridge** | **BR** | NodeCore+Legal |
| Anti-fraud / AML monitoring | `ratchet:flag:*`, `detection:correlated_action` | advisory never sole; WA quorum gate | IG | RATCHET+LensCore+NodeCore |
| Sanctions / compliance screening | `revocation` (-1), licensure revoke, lawful_access | anti-rollback; partner_role gates financial actors | IG | Registry+Verify |
| Market data / FIX/SWIFT/ISO 20022 | external_content/evidence_refs payloads, Tier-5 | scores over ingested data; Aureus = integration only | IG | Financial/Aureus |
| **Order-book / matching engine** | order Contributions | **continuous price-time match needs total-order + atomic match** | **GAP** | GAP (rail) / NodeCore |
| **Dynamic pricing / market-making** | scores as published prices | **optimization compute off-wire** | **GAP** | GAP / operator |
| Fees / commissions / spreads | settlement leg + topical to trade | receipt IG; value bridges | **BR** | Billing/rail+NodeCore |

### 2.5 Governance, Law & Collective Decision (75)

| Function | CEG objects | Transforms | Expr | Component |
|---|---|---|---|---|
| Legislation / rule-making (amendment) | P5 PROPOSAL scores, vote/aggregate, 1-of-6 sign-off | file+vote; supersedes; WA-quorum; 1-of-6 = single VETO | IG | NodeCore+Registry+accord |
| Entrenchment / meta-amendment | entrenched `humanity-accord` (2/3), MAJOR `:vN` | reject if entrenched; out-of-band replacement only | IG | Registry+NodeCore+accord |
| Open-vocab registration | open-prefix families, registry docs | axis-vocab discipline; first-wins, Levenshtein≤2→409 | IG | Registry+NodeCore |
| Transitional founder authority | maturity-gate config | founder scores; mechanized process supersedes at maturity | IG | Registry+accord |
| Subsidiarity / locality scaling | `locality:decision:{scale}` | quorum_size(scale); scale-down/escalate/defer fallbacks | IG | NodeCore |
| Courts / adjudication (WA quorum) | vote, weighted_aggregate, witness_diversity | Policy C/E; fresh-quorum recusal | IG | NodeCore+Legal |
| Dispute / allegation filing | `moderation:{allegation_type}` | scores filing; RATCHET feeds never sole | IG | NodeCore+RATCHET |
| Appeals / reconsideration | `reconsideration:{grounds}` reversed/partial/upheld | fresh-quorum recusal; review-scope duty | IG | NodeCore |
| Witness / testimony preservation | `testimonial_witness:{kind}` self-scope | never aggregated, never sole slashing evidence | IG | NodeCore |
| Evidence / chain of custody | `evidence_refs`, holds_bytes, references_attestation | ContentFetch; verify full SHA; supersedes amend | IG | Persist+Edge |
| Penalties / slashing / revocation | `slashing:{outcome}`, `revocation` (-1) | min-agg; WA quorum gate; flags never sole | IG | NodeCore+Registry |
| **Monetary penalties / fines / restitution** | settlement, judgment scores | judgment IG; **value movement bridges** | **BR** | Financial+Billing+rail |
| Precedent / case law / finality | `hard_case:novel_context`, corpus-as-case-law | supersedes finality; walk references for precedent | IG | Persist+NodeCore |
| **Court-ordered content removal** | `takedown_notice{CourtOrder}` | notice IG; **legal FORCE compelling compliance external** | **BR** | Server+Registry+legal |
| Voting on contributions | `vote:*`, weighted_aggregate, credits×expertise | mean of score×conf; withdraws to change | IG | NodeCore |
| Ballot initiatives / referenda | community/event_listing, consent_record ballots | tally consumer-side; supersedes to recast | IG | NodeCore+CEWP |
| Petitions / signatures | consent_record granted scope [share,publish] | aggregate like ballots; withdraws forward-only | IG | NodeCore |
| Electoral roll / voter registration | geographic community, location_proof, weighted rubric | supersedes ceremony; Policy M; containment | IG | NodeCore+Verify |
| **Voter uniqueness / 1p1v** | credits, bond_posted, ratchet, eigentrust | cost/reputation Sybil-resistance; **true personhood→external proofing** | **BR** | RATCHET+LensCore+Verify+ext |
| **Secret / coercion-resistant ballot** | (none clean) — attestations signed/attributable | **receipt-free unlinkable voting not yet expressible** | **GAP** | GAP |
| Vote integrity / fraud detection | `ratchet:flag:*`, `seed_holder_voting_alignment` | median-agg; advisory feeds WA; never sole | IG | RATCHET+NodeCore+LensCore |
| Delegated / liquid democracy | `delegates_to(voter→proxy, [vote])` | depth-5, 0.5× cap; withdraws severs subtree | IG | NodeCore |
| Quorum computation (locality-scaled) | locality, Policy E quorum_size+min_pool | recusal feasible iff cell_pool ≥ min_pool | IG | NodeCore |
| Sub-quorum fallback | `hard_case:locality_*` | scale-down / escalate / liveness-defer | IG | NodeCore |
| Reverse / live-quorum (presence=authority) | accord_proposal/participation/decision, live set L | proof-of-life floats L; fire-floor-1; Enoch-Arden | IG | NodeCore+Server+accord |
| Consensus protocols | ConsensusProtocol enum on family/community | evaluate at admit; entrenched blocks amend | IG | NodeCore+Persist |
| Moderation as delegable duty | `delegates_to` {moderate,takedown,review,sub_delegation} | enforced-admission walks chain; root steward-bound; depth5 | IG | Server+NodeCore |
| Named-moderator invariant + auto-promotion | `is_named_moderator`, track-record | gate at admit+every step; deterministic auto-promote | IG | Server+NodeCore |
| Open labeling / community notes | scores, `judge_model:verdict`, truth_grounding | consumer filters; may escalate to finding | IG | NodeCore+LensCore |
| **Watchlist auto-detection (CSAM/illegal)** | `watchlist:{id}`, auto-fired takedown | matcher IG; **licensed hash-DB operator-provisioned, off-wire** | **BR** | Server+Registry+ext DB |
| Content classification / age-gating | `content_class`, age_assurance/self_declared | witness outranks self; protective default | IG | Registry+NodeCore |
| Infohazard consent gate | content_class:infohazard + consent:view | affirmative signed act; rides existing families | IG | Server/CEWP |
| Professional licensing | `partner_record`, `licensure`, partner_role | M-of-N quorum over JCS bytes; conf ≤0.5 single-source | IG | Registry+Verify |
| License revocation / suspension | `revocation` (-1), monotonic revision | reject revision decrease; revoked beats stale active | IG | Registry |
| Permits / scoped grants | capabilities_granted, delegates_to, key_grant | attenuating, revocable; rotation_chain | IG | Registry+NodeCore |
| Credentialing / attestation ladder | `attestation:self/hw/registry/license/integrity` | min-agg fail-secure; Policy I L1-L5 | IG | Verify |
| **Bonding / surety (PoB)** | `bond_posted` | claim IG; **value lock+forfeiture bridges (escrow)** | **BR** | Registry + rail |
| Regulatory compliance mapping | consent family, delegates_to(parent→student) | mappings = operator config, NOT new wire | IG | Registry + verticals |
| Data-subject rights (DSAR/erasure/port) | consent revoke→sla→complete, decay, export | subject-revoke rule 2; SLA watcher; DSAR query | IG | Agent CEM+Persist |
| SLA enforcement / deadline watchers | deletion_sla, explainability_sla, hard_case | Policy K watcher clocks; emits on miss | IG | Persist+LensCore |
| Audit chain / immutable record | `audit_chain:hash_continuity`, accord_decision log | substrate-emitted; never silent | IG | Persist |
| Transparency log (public verify) | `transparency_log:inclusion/consistency/cosigned` | witness cosign; Merkle proofs | IG | Verify+Persist |
| Accountability attribution | delegates_to chains rooted at steward-bound key | takedown-isn't-a-coup made structural | IG | Verify+Server |
| Reputation / track-record | commitment_fulfillment, expertise, credits, conformity | EigenTrust Policy C; relational not self-declared | IG | LensCore+NodeCore |
| Drift / integrity monitoring (ratchet) | `detection:cross_agent_divergence/hash_chain/temporal_drift` | median-agg; detector-only gate | IG | LensCore+RATCHET |
| Structural-injustice / correlated-action | `detection:correlated_action:{axis}`, distributive:access | over signed traces; polarity verdict | IG | LensCore |
| Non-silent governance invariant | `hard_case:*`, outcome ladder | default-remove/attributable-keep/view; no passive survival | IG | Persist+NodeCore |
| Federation membership / admission | community infrastructure, ciris-canonical | founder-quorum supersedes; DNS-free resolution | IG | NodeCore+Registry |
| Treaties / inter-fed compacts | `multilateral_participation`, organization links | role-chain resolver for write authority | IG | Registry+Edge |
| Data-sharing / directed replication consent | `consent:replication:{version}` | granting-node scores; withdraws to revoke | IG | Edge+Persist |
| Sovereign equivalence / mutual recognition | Sovereign-Registered equivalence, identity_type | wire-symmetric; consumer differentiates weight | IG | Registry |
| Cross-region merge (federalism) | MergeBallot quorum_weight, precedence | recants>withdraws>supersedes; partition-tolerant | IG | Persist+Server |
| Delegation / power of attorney | `delegates_to(principal→agent, scope, constraints)` | UCAN-style attenuation; depth-5; withdraws severs | IG | NodeCore |
| Guardianship / minor stewardship | minor-protection rulings, steward-binding | never key handover; non-overridable floor; fail-secure | IG | Registry |
| Adult incapacity / conservatorship | `capacity_assurance:*` per-domain vector | prior-will-first; fails OPEN to liberty | IG | Registry+Medical |
| Sub-delegation / deputization | sub_delegation token | child ⊆ parent; revocable; depth ≤5 | IG | NodeCore |
| Constitutional kill-switch | `accord:invoke:CONSTITUTIONAL`, entrenched 2/3 | 2-of-3 over anti-replay bytes; wire+scope isolated; fire-floor-1 | IG | Registry+accord+Server |
| Halt resumption / un-halt | `accord:lifecycle:active`, resumes_halt_id | strict-majority of live set; never lone sig | IG | Registry |
| Halt recovery under decimation | live set L, steward backstop, restore-to-known-good | proof-of-life floats L; restore-only reversal | IG | Registry+Server+steward |
| Notify / drill | `accord:invoke:notify/drill` | 2-of-3; UI MUST distinguish from CONSTITUTIONAL | IG | Registry |
| Per-Contribution halt | `consent:state:revoked`, subject withdraws | M-1 revocability extended to per-record | IG | Agent CEM |
| Conflict-of-interest / recusal | fresh-quorum recusal, seed_holder_voting_alignment | consumer-side recusal; reconsideration if capture proven | IG | NodeCore |
| **WA appointment / rotation / recusal** | WA status via identity_type, Charter Annex B | fabric RECORDS status; **appointment external by design** | **BR** | Registry + ext Charter |
| Rights floor / prohibited bounds (apophatic) | `prohibited:{category}` (22 NEVER_ALLOWED) | min-agg; hard floor by polarity | IG | Registry/Agent namespace |
| Whistleblower disclosure protection | cohort_scope self→promote, empty subject_key_ids | supersedes widen; substrate-protective guards takedown | IG | Server/CEWP |
| FOIA / public records request | consent_record scope [publish], sla_breach | SLA watcher clocks agency window | IG | Registry/CEWP |
| Emergency / public-safety broadcast | `emergency_authority`, event:lifecycle, H3 cells | cascade-by-containment; pre-attested bypass | IG | Edge+Registry |
| **Public-official / civil identity auth** | identity_occurrence→service key, age_assurance:government | cross-binding IG; **gov attestation external** | **BR** | Verify+Registry+ext gov |
| Notarization / trusted timestamping | asserted_at/signed_at, inclusion proof | witness cosign anchors time | IG | Verify+Persist |
| Contract formation / multi-party | consent_record, bilateral_pair | bilateral pairing; performance value→settlement bridge | IG | Legal (+settlement) |
| **Treasury / public finance / budget** | settlement cohort_scope:public | receipt IG; **value movement off-wire on rail** | **BR** | Financial + rail |
| **Taxation / fee / assessment collection** | assessment scores, settlement link | obligation IG; **atomic pay⟺mark-paid bridges** | **BR** | Billing + rail |
| Quorum-compromise / capture recovery | `reconsideration:quorum_compromise`, restore snapshot | steward quorum restore-only rollback; append-only trail | IG | Registry+steward |
| Census / population stats (privacy-preserving) | location_proof rough-only, distributive:access | aggregate compose; bulk-scan structurally non-conformant | IG | LensCore |
| **Physical-world enforcement of judgments** | (none) — fabric holds no monopoly on force | **revoke digital standing IG; physical arm external** | **BR** | GAP / ext legal-force |

### 2.6 Knowledge, Science & Education (87 — deduped)

| Function | CEG objects | Transforms | Expr | Component |
|---|---|---|---|---|
| Course/curriculum/OER/lesson publishing | external_content `course`/`dataset`/media, content_class:educational | scores; supersedes; mandatory accessibility fields | IG | NodeCore+Persist |
| Enrollment / cohort roster | community classroom, affiliation, DEK cascade | supersedes; Policy M; key_grant rewrap | IG | NodeCore |
| Assignment submission / grading / progress | external_content vs assignment, `assessment:grade:{rubric}` | scores→mean; supersedes regrade; recants if error | IG | NodeCore+LensCore |
| Live lecture / streaming class | live_stream (Phase-2), epoch-DEK, delivery_receipt | Edge parallel transport; Policy M subscriber-set | IG | Edge |
| Discussion forum / Q&A / reputation | chat_message, topical_relation, vote, expertise, credits | weighted_aggregate; EigenTrust | IG | NodeCore+LensCore |
| Tutoring / mentoring / office hours | `need:*mentor`, bilateral_pair, event_listing | bilateral compose; delegates_to; RSVP/reschedule | IG | NodeCore |
| **Tuition / paid-course / APC / grants** | settlement vs enrollment/publication | **content-for-payment atomicity bridges** | **BR** | Billing/Aureus+Financial |
| **Exam / quiz proctoring** | judge_model:verdict, expertise_fraud; biometric→prohibited | non-biometric integrity IG; **biometric proctoring no agency-free expression** | **GAP** | RATCHET/NodeCore; GAP |
| Diploma / micro-credential / badge issuance | `credential:degree/badge:{field}`, partner_record | scores; delegates_to issuer; supersedes; withdraws rescind | IG | Verify+Registry+NodeCore |
| Credential verification / transcript / wallet | attestation ladder, cert_validity, key_grant | Policy I compose; consent-scoped release; Identity=key | IG | Verify+Persist |
| Credential revocation | withdraws/recants, revocation, monotonic revision | recants outranks withdraws; anti-rollback merge | IG | Registry |
| **Credential legal recognition / equivalency** | partner_record + Sovereign-Registered | attest IG; **legal-force recognition→external authority** | **BR** | Legal/Themis |
| Continuing-education credits | `credits:{domain}` positive-only, commitment_fulfillment | truth-grounding accrual | IG | NodeCore |
| Bibliographic / catalog / metadata (FAIR) | encyclopedia-shape, Source schema, topical_relation | scores; supersedes; axis-vocab discipline | IG | NodeCore+Persist |
| Content-addressed storage / preservation / PID / fixity | content_sha256, holds_bytes, archive_mode:retain, inclusion | retention operator; stable-id grouping=DOI; SHA verify | IG | Persist+Verify |
| Controlled access / lending / special collections | key_grant key_validity_window, community DEK | time-window=auto-return; withdraws=recall; **paid→bridge** | IG | NodeCore+Edge |
| Versioning / revision history / format migration | supersedes chain, differs_in, transparency_log:consistency | chain walk; migrated references original | IG | NodeCore+Persist |
| Interlibrary loan / catalog federation | holds_bytes dir, consent:replication, PeerResolver | transport; cross-region merge; withdraws revoke peering | IG | Server+Edge |
| Dataset publishing / versioning / data-DOI / citation | external_content `dataset`, supersedes, cites_source | stable-id resolution; topical edges; evidence_refs | IG | NodeCore+Persist |
| Dataset licensing / reuse / metadata / embargo | consent:scope, license, key_grant window | Policy K consent; supersedes lift = widen scope | IG | NodeCore |
| Datasets-as-evidence / provenance / lineage | evidence_refs, provenance:build_manifest, truth_grounding | Merkle composition; chain walk | IG | NodeCore+Verify |
| **Paid dataset / marketplace** | settlement, key_grant SubscriptionTier | **atomic data-for-payment bridges** | **BR** | Billing/Aureus |
| Manuscript submission / reviewer solicitation | external_content community pool, `need:*expertise`, witness_diversity | admission; Policy M; expertise-weighted selection | IG | NodeCore |
| Peer review / editorial decision / anonymity | `review:{aspect}` scores, weighted_aggregate, consent:anonymous | vote credits×expertise; locality quorum; decay de-anon | IG | NodeCore+LensCore+Verify |
| Reviewer reputation / post-pub review | moderation_track_record, commitment_fulfillment, comments_on | merit auto-promote; edge-walk | IG | NodeCore+LensCore |
| Review-ring / collusion detection | `ratchet:flag:coordinated/expertise_anomaly`, correlated_action | median-agg; quorum (ratchet never sole) | IG | RATCHET+NodeCore |
| Methods / build / pipeline provenance / pre-registration | external_content, provenance:slsa/build_manifest, inclusion | Merkle; transparency-log priority; lock-after | IG | NodeCore+Verify |
| Reproducibility / replication attestation | `reproduction:{claim}` scores, witness_diversity, truth_grounding | the CLAIM is in-grammar | IG | NodeCore |
| **Computational re-execution / verify-of-compute** | judge_model:verdict, agent_integrity | records verdict; **execution = external compute** | **BR** | Verify + ext compute |
| Citation graph / metrics / impact | cites_source/references edges, eigentrust | Policy C weighted-graph; h-index = consumer compose | IG | NodeCore+LensCore |
| Retraction / correction / erratum | recants, supersedes, corrects | recants outranks; propagate along cites_source | IG | NodeCore |
| Encyclopedia / editorial consensus / disambiguation | encyclopedia_article, vote, witness_diversity, disambiguates | supersedes revision; quorum for disputes | IG | NodeCore |
| Fact-checking / translation / authority files | truth_grounding, judge_model, translation_of, canonical_binding | per-locale Merkle; global_feed weighting | IG | NodeCore+LensCore+Verify |
| Expertise standing / skill endorsement / fraud detection | expertise, credits, witness_diversity, expertise_fraud | weighted_aggregate; median-agg; capacity rejects self-emit | IG | NodeCore+LensCore+RATCHET |
| Open-access / preprints / scholar identity / CRediT roles | external_content commons, asserted_at, canonical_binding, `contribution_role:{role}` | global_feed; inclusion priority; axis-vocab | IG | NodeCore+Verify |
| Reuse licensing (CC) | consent:scope:[share,train,analyze,publish] | Policy K consent resolution | IG | NodeCore |
| **Research grant disbursement** | `need:*evidence`, settlement, partner_record | need IG; **value transfer→external rail** | **BR** | Billing/Aureus+Financial |
| Conferences / societies / institutional repository | event_listing, organization, affiliation, community infra | quorum/role-chain; retention operator | IG | NodeCore+Registry+Persist |
| Standards / specification publishing | external_content + accord_data multi-sig | StewardTriple/WaQuorum/1-of-6 admission; transparency | IG | NodeCore+Verify |
| Curriculum / learning standards | external_content, goal/approach/method DAG | MetaGoalAlignment invariant; supersedes | IG | NodeCore |
| Age-appropriate content / minor protection / accessibility | age_assurance, content_rating, cw_class, watchlist; mandatory alt_text | protective-gate compose; auto-fire takedown; schema-required | IG | Registry (rulings)+NodeCore |
| Lab notebook → promotion | local_data self-DEK, supersedes differs_in | tiered-scope self→community→global | IG | NodeCore+Persist |
| Apprenticeship / practical skill demo | video, testimonial_witness, witness_diversity | witness composition; delegates_to mentor | IG | NodeCore |
| Academic-misconduct adjudication / plagiarism | moderation, slashing, reconsideration; content_sha256/perceptual_hash/watchlist | quorum (P8); exact/near-dup match; **semantic compute off-fabric** | IG | NodeCore (+ext semantic) |
| **Search ranking / recommendation paths** | holds_bytes dir, citation graph, manifold_conformity | directory+composition IG; **relevance/recommender = consumer/agent compute** | **GAP** | Edge/LensCore + consumer |
| Education-record DSR (FERPA/GDPR) | consent_record DSAR, deletion_sla | subject-revoke rule 2; SLA-breach watcher | IG | NodeCore+Registry |

### 2.7 Health, Safety & the Physical World (63)

| Function | CEG objects | Transforms | Expr | Component |
|---|---|---|---|---|
| Clinician licensure / credential verification | `licensure:{authority}:{key}`, partner_role PROFESSIONAL_MEDICAL, partner_record | issue/supersede/revoke; M-of-N quorum; Policy I/J ladder | IG | Registry+Verify |
| Medical record w/ patient authority | `agent_files:medical_record:{patient_hash}`, subject=[patient], consent | content-addressed; delegates_to(guardian); HIPAA Policy K | IG | Persist+Medical |
| Patient consent / DSAR / erasure | consent_record, deletion_sla, sla_breach watcher | single-subject authority no quorum; SLA clock | IG | Persist+Portal |
| Clinician-supervised diagnosis | `dma:dsdma:medical:*`, oversight_mode, prohibited:medical | WBD defer normal form; delegates_to(clinician); **reasoning EXCLUDED** | IG | Medical+NodeCore (agent excl) |
| Medical device / firmware integrity | provenance:build_manifest/slsa, hardware_rooted, device=occurrence | min-agg fail-secure; supersedes update; inclusion | IG | Verify+Server |
| Lab / diagnostic provenance | provenance, inclusion, hybrid sig | content-addressing; recants+corrects for amended | IG | Verify+Persist |
| Health credential / immunization | witness-reserved health dim, health_authority partner_role | witness scores; supersedes booster; read-union | IG | Verify+Registry |
| Telemedicine encounter transport | live_stream (Phase-2), cohort DEK, transport_destination | delegates_to(session); per-write DEK | IG | Edge *(Phase-2)* |
| **Mental-health crisis / suicide routing** | prohibited:crisis_escalation/protective_routing, need:expertise | WBD defer mandatory; **responder dispatch external** | **BR** | Agent WBD (+ext services) |
| Disease-outbreak alerting | news_article health_authority, geographic cohort, event:lifecycle | cascade-by-containment; pre-attested bypass | IG | Server+Registry |
| Contact tracing (opt-in) | consent:scope:[analyze], consent_record | subject-revoke rule 2 | IG | Persist |
| Population epidemiological surveillance | `detection:correlated_action/distributive:access` | median-agg; mass non-consented surveillance refused | IG | LensCore |
| Age verification (verified level) | age_assurance:{provider|government}, age_self_declared | witness outranks self; protective default | IG | Registry+Verify+consumer |
| Adult-content gating | content_class:adult, resolved band | consumer visibility compose; never slashing | IG | Server + consumer |
| CSAM detection & immediate takedown | watchlist, `takedown_notice{PerceptualHashCsam}`, PerceptualHashMatcher | auto-fire 1-hour no counter-notice; NCMEC; non-silent floor | IG | Server+Persist+Registry |
| Guardian stewardship of minor / minor-as-data-subject | delegates_to(parent→minor), minor_data_handling | fails-secure-to-locked; FERPA = custodian role-swap | IG | Server+Registry+Persist+Legal |
| No-unmoderated-space invariant | `is_named_moderator`, track-record, community_unmoderated | merit auto-promote; quiesce if no live moderator | IG | Server+NodeCore |
| Grooming / predatory-pattern flagging | watchlist non-CSAM→detection+moderation, prohibited:pattern_detection | per-group never global; quorum; never sole | IG | Server+NodeCore |
| Age-misdeclaration adjudication | moderation:age_assurance_misdeclaration | quorum; slashing decoupled | IG | NodeCore |
| Cryptographic identity & authentication | identity_occurrence, transport_destination, ladder, prohibited:identity_verification | self/single-vouch; supersedes rotate; withdraws revoke | IG | Verify+Server |
| Harassment / abuse reporting | testimonial_witness, ratchet:flag:harassment, moderation | RATCHET advisory; WA quorum load-bearing; Frickerian | IG | LensCore/RATCHET+NodeCore |
| Personal-safety check-in / proof-of-life | scores + valid_until freshness | absence detection; supersedes refresh | IG | Server |
| Whistleblower / protected disclosure | cohort_scope self→community, empty subject_key_ids | Policy H promote; bad-actor takedown→HUMANITY_ACCORD | IG | Server+HUMANITY_ACCORD |
| Home-security monitoring / alerting | family embedded devices, family DEK, prohibited:home_security | alert IG; **autonomous arm/disarm deferred (WBD)** | IG | Home/Server (agent excl) |
| Doxxing / PII takedown | takedown_notice, consent revoked | subject-revoke; forward-only cessation | IG | Server |
| Emergency broadcast / public alerting | emergency_authority, geographic cohort, event:lifecycle | cascade-by-containment; missing-person consent-override | IG | Server+Registry+Edge |
| Emergency ack / muster / evacuation ack | consent_record granted, rsvps | scores ack; consumer tallies muster (ack≠consent) | IG | Server |
| Mass-casualty / responder coordination | community incident_response, responder chat, event_listing | supersedes membership; need:* resource calls | IG | Server+NodeCore |
| Resource request / mutual-aid matching | `need:{domain}:{kind}` positive-only | supersedes/withdraws; consumer matches offers | IG | NodeCore |
| Federation-wide emergency halt | accord:invoke:CONSTITUTIONAL, entrenched, live-quorum L | 2-of-3; fire-floor-1; scope+wire isolated; restore backstop | IG | HUMANITY_ACCORD/Registry |
| Infrastructure-free survivor signaling | accord_participation proof-of-life, Reticulum/HF | signature=trust, path untrusted | IG | Edge |
| IoT device identity & enrollment | identity_occurrence embedded, family member, steward-binding | self/single-vouch admit; family supersedes | IG | Home/Server |
| Home / family cohort & shared-device scoping | family, family DEK, key_grant, structural invisibility | retroactive cascade; Option-A forward secrecy | IG | Home+Server |
| **Device command / physical actuation** | `delegates_to(owner→actor, control)`, prohibited:infrastructure_control | authorize IG; **actuation = external effect-rail** (physical analog of settlement bridge) | **BR** | GAP (effect-rail) |
| **Sensor telemetry ingestion** | open-vocab sensor/vital dim, evidence_refs, live_stream Phase-2 | point readings fit; **no high-rate continuous-stream subject_kind** | **GAP** | Home / GAP |
| Home-automation rules / scene triggers | signed rule-as-data, WASM-sandboxed eval | PDMA reduction to gate-checked emit; **actuation→effect-rail** | IG | Home + Attestation Calculus |
| **Smart-meter reading / energy attestation** | energy:* dim, evidence_refs, settlement link | reading IG; billing off-wire; **high-rate metering hits telemetry gap** | **BR** | Billing / GAP (volume) |
| **Demand-response / load-shedding signal** | utility/grid_authority news_article, geographic cohort | signal IG; **load-shed actuation→effect-rail** | **BR** | Server (signal); GAP (act) |
| **Grid / SCADA real-time control** | prohibited:infrastructure_control | **append-only latency/monotonic ≠ sub-second control loop** | **GAP** | GAP |
| **P2P energy / commodity trading** | settlement, provenance REC/carbon, Identity=Wallet | **atomic energy-for-payment = CC 1.7 boundary→rail** | **BR** | Financial/Aureus+settlement |
| Renewable / carbon provenance certification | provenance, partner_record certifier, content_rating | scores+inclusion; **certificate trading bridges** | IG | Verify+Registry |
| **Environmental sensor networks** | environment:* dim, location_proof, evidence_refs | aggregate via correlated_action; **high-rate volume hits telemetry gap** | **GAP** | LensCore / GAP |
| Structural-injustice / env-harm detection | `detection:correlated_action:{axis}`, aggregate_footprint | detector-only median-agg; advisory never sole | IG | LensCore |
| Geospatial location proof | location_proof, H3 ≤res7 | containment admit; resolution-violation hard_case | IG | Server |
| Food/goods supply-chain traceability / recall / certification | provenance, agent_files, revocation, content_rating | content-addressing; per-lot Merkle; cascade-by-containment | IG | Verify+Persist+Registry |
| Vehicle / asset identity / driver licensing | identity_occurrence embedded, licensure:{DMV}:{key}, partner_role | admission; supersedes renewal; revocation; Policy J | IG | Server+Registry |
| **Telematics / trip telemetry** | telematics:* dim, location_proof (rough-only tension), evidence_refs | **high-rate continuous stream + fine-GPS vs rough-only enforcement = gap** | **GAP** | GAP |
| **Mobility-as-a-service booking & payment** | event_listing ticket Paid, settlement, delegates_to(ticket) | **ride-for-payment atomicity = CC 1.7→rail** | **BR** | Billing+settlement |
| Traffic / transit advisory & routing | transit_authority news_article, event:lifecycle, prohibited:protective_routing | advisory IG; **autonomous protective re-routing prohibited (defer)** | IG | Server |
| **Autonomous-vehicle command & control** | prohibited:infrastructure_control | **same mismatch as SCADA — attest post-hoc, cannot actuate** | **GAP** | GAP |
| Customs / cargo manifest / cross-border | partner_record, organization, provenance, multilateral_participation | M-of-N quorum; **duty/tariff→settlement** | IG | Registry |
| **Physical access grant (door/gate)** | delegates_to(access), key_grant, valid_until, location_proof | grant IG; **unlock actuation→effect-rail** | **BR** | Server+Verify (grant); GAP (act) |
| Visitor credential issuance & revocation | key_grant + rotation_chain, revocation | supersedes rotation (not withdraws); immediate revoke | IG | Server |
| **Biometric access / matching** | prohibited:biometric_inference, special-category restriction | can attest match-occurred; **inference barred — no clean expression** | **GAP** | GAP / restricted |
| Tamper-evident safety-event audit | append-only corpus, audit_chain, inclusion/consistency | RFC 6962 proofs; supersedes never mutates | IG | Persist+Verify |
| Safety-system liveness / health monitoring | health:liveness external, system:* self-report | witness=external; deps as evidence_refs | IG | Server/Persist + ciris-status |
| Fail-secure halt for uncertainty | WBD defer, fail-secure posture, prohibited:* floor | PDMA→defer; no attestation on uncertainty; routes to WA | IG | Agent WBD + admission gate |
| Regulatory compliance mapping (HIPAA/FDA/COPPA) | Policy K CEM, vertical mappings | operator-pinned config; no new wire | IG | Legal/Themis+Persist |
| Recall / revocation propagation | revocation (-1), rollback_detected | immediate non-rollbackable; detect revision decrease | IG | Registry |

### 2.8 Identity, Records & Property (69)

| Function | CEG objects | Transforms | Expr | Component |
|---|---|---|---|---|
| Root personhood anchor (SSI) | federation_keys identity_type⊇{user}, hybrid keypair | self-signed; standing relational (anti-self-declare); **adult un-stewardable** | IG | Verify+Registry |
| One self across devices/agents | identity_occurrence, device_class, occurrence_id | binding scores; auto hard_case; self-DEK auto-wrap | IG | Verify+Persist |
| Claim external/legacy identifier | `identity:canonical_binding:{hash}`, canonical-hash subject | self-assert; admission widens withdraws; competing→RATCHET | IG | Registry+Verify |
| Hardware-rooted key custody | hardware_custody, attestation:hardware_rooted, hardware_class | boolean-via-score; Policy I ladder | IG | Verify |
| Key rotation / identity / social recovery | new occurrence, delegates_to recovery roots, rotation_chain | supersedes+withdraws+delegates_to; anti-rollback | IG | Verify+Persist |
| Pseudonymity / unlinkable personas | separate roots, NO occurrence edge, cohort isolation | absence of binding IS the mechanism | IG | Verify |
| Verifiable attribute credential | age_assurance witness-reserved, age_self_declared, partner_role | witness scores; band predicate downstream; selective disclosure | IG | Registry+Verify |
| Guardianship / minor steward / PoA | delegates_to(adult→minor), is_steward_bound | grant/transfer/revoke; un-stewardable adult rejected | IG | Registry+Verify |
| Identity deactivation / revocation | revocation (-1 immediate), identity_type emitter gate | min-agg; anti-rollback | IG | Registry |
| **Proof of unique personhood** | bond_posted, witness_diversity, ratchet flags | bounded by cost/reputation; **biometric uniqueness PROHIBITED (apophatic)** | **GAP** | RATCHET/NodeCore (bounded) |
| Citizenship / polity membership | community geographic, org_membership, locality | supersedes ceremony; location containment; resolve_community | IG | Registry+NodeCore |
| Residency / domicile | location_proof ≤res7 (rough-only) | withdraws forward-only; substrate doesn't verify truth | IG | NodeCore |
| Office-holding / official role | partner_role, org_membership, occurrence→service key | delegates_to role-chain; canonical_binding authenticates | IG | Registry |
| Voter registration / suffrage | community roll, consent_record ballots, weighted rubric | tally consumer-side; election_interference apophatic-bounded | IG | NodeCore |
| Household / civil-status record | family, members roster, family DEK, structural invisibility | supersedes ceremony; Option-A rekey; hard_case | IG | Persist+Verify |
| **Birth registration** | new user key + delegates_to guardian, registrar partner_record | registration IG; **legal force external** | **BR** | Registry |
| Death registration / decedent status | witness/registrar scores, revocation:death, relational_anchor | scores+withdraws+delegates_to executor | IG | Registry |
| Marriage / civil-union record | family roster OR partnership_grant/accept, bilateral_pair | ratify iff both present; supersedes amend | IG | Persist |
| Divorce / dissolution | prior roster / partnership pair | supersedes/withdraws forward-only; hard_case | IG | Persist |
| Name / gender / civil-status amendment | attribute scores, registrar attestation | supersedes (latest wins); recants if false; prior stays in lineage | IG | Registry |
| Adoption / guardianship transfer | delegates_to(adult→minor), steward-binding gate | supersedes replaces; withdraws ends; fail-secure | IG | Registry |
| **Real-property land-title registry** | **NEW title subject_kind** (rides scores+discriminator), provenance, inclusion | record SHAPE is pure 1+4 vocab; **dirt-binding + legal force external** | IG | GAP (unbuilt subject_kind) |
| **Title conveyance for consideration** | title + settlement, settled_action_ref | record IG; **atomic deed-for-payment = CC 1.7→rail** | **BR** | Financial + settlement |
| **Liens / mortgages / encumbrances** | scores via topical_relation, settlement, lender partner_record | lien IG; **loan value transfer→bridge** | **BR** | Financial/Aureus |
| Cadastral / boundary / geospatial registry | location_proof, H3 arbitrary res, geographic_constraint | containment; custom polygon predicate | IG | NodeCore |
| Easements / leases / usage rights | delegates_to scoped + valid_until, key_grant, consent:scope | grant/amend/terminate; **paid lease→settlement** | IG | GAP / Home |
| **Anchoring record to physical object/parcel** | evidence_refs (deeds, GNSS, surveys), witness scores | **CEG composes claims, cannot adjudicate physical truth** (oracle problem) | **GAP** | GAP |
| **Movable-asset / chattel registry** | **NEW asset_record subject_kind**, subject_key_ids, asset_id | record IG (mirrors partner_record); component unbuilt | IG | GAP / Financial |
| Provenance / chain of custody of asset | provenance:* family, build_manifest Merkle, inclusion | supersedes chain; RFC 6962 tamper-evident | IG | Verify |
| **Asset sale with consideration** | asset_record + settlement | **atomic swap = CC 1.7→rail** | **BR** | Financial + settlement |
| Fractional ownership / shares / cap table | asset_record/org multiple subjects, weighted rubric | supersedes cap-table; **dividends→settlement** | IG | Financial/Aureus |
| **Escrow / custody / collateral hold** | settlement, consent_record conditions | **conditional simultaneous release = CC 1.7→rail** | **BR** | Financial/Aureus |
| Authorship / first-publication / priority | content sha256, asserted_at, inclusion, skill_import | inclusion proof = tamper-evident priority; witness cosign | IG | Verify+Persist |
| Copyright ownership & assignment | external_content sub_kinds, subject_key_ids, provenance | supersedes assign; delegates_to exclusive; **paid→settlement** | IG | Persist |
| IP licensing (usage rights) | consent:scope, delegates_to, key_grant, licensure | grant+access; withdraws revoke; **paid license→settlement** | IG | Verify+NodeCore |
| **Patent / trademark registry & priority** | **NEW registry subject_kind**, inclusion, provenance | filing+priority IG; **legal grant of monopoly external** | IG | GAP / Registry-adjacent |
| **IP infringement takedown (DMCA/DSA)** | takedown_notice closed LegalBasis, perceptual_hash, counter_notice | notice+fast-path IG; **legal force/standing external** | **BR** | NodeCore |
| **Royalty / revenue distribution** | settlement, settlement:received counter-attest | binds payout to what it paid; **value transfer→rail** | **BR** | Billing/Financial |
| Attribution / moral rights / plagiarism detection | cites_source/translation_of, skill_import, correlated_action | recants for misattribution; median-agg detector | IG | LensCore |
| The permanent record (append-only) | CIRISPersist corpus, audit_chain, JCS bytes | supersedes/withdraws NEVER mutate; truth=signed history | IG | Persist |
| Tamper-evidence / record integrity | inclusion/consistency (RFC 6962), audit_chain, hybrid sig | Merkle proofs; rollback_detected (-1); chain break detectable | IG | Verify+Persist |
| Post-quantum durability of posterity | ML-DSA-65 on every fed row, ML-KEM-768 wrap (v2), v3 headroom | tier=fed ⟹ hybrid sig; forge-now/harvest-later threat; fail-secure | IG | Verify |
| Replication / preservation / partition tolerance | holds_bytes, replication_lag, anti-entropy, ContentFetch | PeerResolver; stable-id merge w/o chain completeness | IG | Edge+Persist |
| Witnessed timestamping (notarized time) | transparency_log:cosigned witness-reserved, STH cosign | witness scores; cosigned STH = notarized time anchor | IG | Verify (witness) |
| Archival readability / retention | archive_mode {rotate-forward|retain}, epoch keys, retirement | key-deletion-after-window = holder behavior; retain keeps above floor | IG | Persist |
| Generational continuity / relational anchor | identity_continuity:relational_anchor, credits/expertise | standing persists in corpus independent of any key | IG | Persist |
| Notarization (authority attestation) | scores itself, cert_validity, partner_role | the atomic primitive — every record fn reduces to it | IG | Verify+NodeCore |
| Multi-party witnessing | witness_diversity (N=3 bars), testimonial_witness | boolean-via-score when bar met; testimonial never sole | IG | NodeCore |
| Proof of existence / document sealing | sha256, evidence_refs, asserted_at, inclusion | hash-only attestation + inclusion = sealed PoE; bytes private | IG | Persist+Verify |
| Apostille / cross-jurisdiction auth | cross-attestation, cert_validity, delegates_to chains | depth-capped traversal; M-of-N steward quorum | IG | Verify+Registry |
| Non-repudiation / signature verification | hybrid sig over JCS, canonical_bytes_hash, bound-payload | recants is ONLY honest disavow (penalized); non-repudiation structural | IG | Verify |
| Heir / beneficiary designation | delegates_to(holder→heir) scope, consent_record | designate/revise/revoke scoped to delegated assets | IG | Legal/Themis |
| Executor / administrator authority | delegates_to(decedent→executor), role-chain resolver | activated on death attestation; depth+weight-capped | IG | Legal/Themis |
| **Estate asset transfer on death** | asset_record/title supersedes, settlement | record IG; **value transfer + death-conditioned trigger bridge** | **BR** | Financial/Themis |
| **Will execution / dead-man's-switch** | delegates_to + key_grant rotation, health:liveness | **delegates_to is immediate/unconditional — no native event-trigger** | **GAP** | GAP / Verify (liveness only) |
| Right to be forgotten / DSAR erasure | consent revoked→sla→complete, subject_key_ids | subject-revoke (no quorum); SLA watcher; EvictionSweeper | IG | Persist+NodeCore |
| Forward-only revocation semantics | withdraws (non-retroactive), permanent "was-withdrawn" leaf | withdraws vs recants asymmetry; consumer ratio tracking | IG | Persist |
| Cryptographic forgetting (noise-floor) | archive_mode rotate-forward, epoch DEK, retirement operator | honest-holder key deletion → info-theoretically unrecoverable | IG | Persist |
| Selective disclosure / data minimization | cohort_scope gradient, structural invisibility, amount_commitment | scope-gated; rough-only location; ZK composes later no wire change | IG | Edge+Persist |
| Forget-vs-preserve adjudication | permanent leaf vs evictable bytes, CourtOrder, consent SLA | MECHANISM IG; POLICY = quorum + court-order bridge | IG | NodeCore+Registry |
| Registrar / civil-authority institution | organization + partner_record, M-of-N quorum, partner_role | scores+supersedes+withdraws; identical-JCS quorum | IG | Registry |
| Professional licensure issuance & revocation | licensure, partner_record, revocation, license_validity | M-of-N quorum; anti-rollback; immediate revoke; Policy I | IG | Registry |
| Cross-region record portability | replication, transport_destination, indexed ids | anti-entropy merge; resolve_member_transport; never DNS | IG | Edge |
| Record dispute / contestation | competing scores, reconsideration, moderation | challenge+reconsideration+recants; WA quorum gate | IG | NodeCore |
| Anti-rollback / monotonic record state | revision:u64, rollback_detected (-1) | admission rejects decrease; revoke beats stale active | IG | Persist+Verify |
| Record correction: errata vs fraud | supersedes vs recants | supersedes=neutral; recants=acknowledged error penalty | IG | Persist |
| Trust-root / re-rootable authority | community infrastructure, ciris-canonical, Commons-plaintext | founder-quorum; consumers MUST be able to re-root | IG | Registry |
| Sealed / confidential vital records | cohort self/family + invisibility, community DEK, key_grant | sealed scope (no holds_bytes leak); selective unsealing | IG | Persist+Edge |
| **Identity = Wallet (economic identity)** | fed signing key ≡ Base wallet | proves "I am" + "I paid" no oracle; **value transfer→chain rail** | **BR** | Financial/Aureus |

---

## 3. THE COVERAGE LEDGER

| Domain | Functions | In-grammar | Bridge | Gap |
|---|---:|---:|---:|---:|
| Internet substrate | 55 | 43 | 10 | 2 |
| Communication & Media | 54 | 48 | 4 | 2 |
| Social & Community | 77 | 69 | 5 | 3 |
| Commerce & Money | 70 | 41 | 22 | 7 |
| Governance & Law | 75 | 64 | 10 | 1 |
| Knowledge / Science / Education | 87 | 78 | 6 | 3 |
| Health / Safety / Physical | 63 | 50 | 7 | 6 |
| Identity / Records / Property | 69 | 57 | 9 | 3 |
| **TOTAL** | **550** | **450** | **73** | **27** |

**Headline.** Of **550** civilizational functions tested, **450 (81.8%)** are **fully in-grammar** (closed 5 ops + open vocabulary, no external dependency); **73 (13.3%)** are **in-grammar but bridge** the atomic/physical/legal/fiat act to an external rail (the fabric still records the trust claim); only **27 (4.9%)** are genuine **gap-needs-design** — no clean CEG expression today.

**Built vs unbuilt.** Roughly **509 / 550 (≈93%)** are realized — at least for their in-grammar surface — by an **existing** CIRIS component (CIRISVerify, CIRISPersist, CIRISServer, CIRISEdge, CIRISNodeCore, CIRISRegistry, CIRISLensCore, RATCHET, plus verticals CIRISFinancial/Aureus, CIRISBilling, CIRISMedical, CIRISLegal/Themis, CIRISHome, CIRISProxy). The **≈41 unbuilt** comprise: the 27 design-gaps, ~4 impl-pending tiers (live_stream voice/video, livestream broadcast, accountable-stream tier — Phase-2), ~4 greenfield in-grammar `subject_kind`s (title/deed, movable-asset, patent/trademark registry, easements component), and ~6 pure external-rail bridges with no in-fabric component (native transfer, remittance, settlement finality, clearing, order-book, bearer value).

**The structural finding.** No domain required a **new structural primitive**. Realtime comms ("the 13th path"), emergency broadcast, kill-switch, marriage, land title, peer review, payroll, and CSAM removal all express as `scores` + the four composers + new vocabulary. The grammar's closure held.

---

## 4. THE GAP / ROADMAP — "to provide civilization, here is what is left"

Grouped by *what each gap actually needs*:

**A. Needs a new `subject_kind` / vocabulary (cleanest greenfield — pure in-grammar extensions):**
- **Property/title/deed + movable-asset (chattel) registry** — mirrors `organization`/`partner_record` exactly; record SHAPE is in-grammar, the owning component is unbuilt. *Highest-leverage extension.*
- **Patent/trademark registry** — filing + priority via `transparency_log:inclusion`; in-grammar, unbuilt.
- **Canonical `email` sub_kind** — composes today as `chat_message` + transport; documentation/vocabulary fix only.
- **Easements/leases component** — `delegates_to`-scoped grants exist; needs an owning module.

**B. Needs a composition policy / amendment (CC 4.5.1) only if demand pulls:**
- **Realtime convergent-merge primitive (CRDT/OT/LWW)** for collaborative editing / multiplayer / whiteboard — transport (sealed PQC chunks) is in-grammar; convergence is deliberately app-layer. Codify only on real demand.
- **Ranked relevance / semantic search & recommendation** — directory + citation-graph + eigentrust composition is in-grammar; the *ranking model* is consumer/agent-side by design (the deferred brain).

**C. Needs an external rail (acceptable bridge — see §5):**
- **Atomic fair-exchange / DvP / atomic swap / escrow / conditional-release** — the one CC 1.7 falsification target; settlement rail.
- **Authoritative balance / proof-of-reserves**, **settlement finality**, **clearing/netting**, **order-book matching**, **bearer/anonymous cash-like value** — all need a totally-ordered ledger / token property the fabric deliberately refuses; chain rails.
- **Tax calculation/withholding, dynamic pricing/market-making** — off-wire compute; operator policy.
- **Legal force / physical enforceability** (court compulsion, incarceration, asset seizure, monopoly grant of a patent) — external state.

**D. Needs new infrastructure design (real engineering gaps):**
- **High-volume continuous sensor/telemetry streaming `subject_kind`** — point readings fit; IoT/telematics/env/smart-meter high-rate, high-fan-in ingestion has no first-class shape (and fine-GPS telematics collides with rough-only ≤res7). *The biggest physical-world gap.*
- **Imperative physical-actuation effect-bridge** — authorization (`delegates_to`/`key_grant`) is in-grammar; CEG cannot *be* the imperative effect. Needs a designed "effect-rail": authorization attestation → off-fabric actuator → signed receipt back (mirroring `settlement_ref`).
- **Conditional / event-triggered execution** (will-on-death, dead-man's-switch, posthumous key release) — `delegates_to` is immediate/unconditional; needs a liveness-oracle + conditional-trigger design (`health:liveness` exists only as observation).
- **Hardware-rooted attestation chain** — `hardware_class` is self-asserted; needs normative TPM-quote / Apple / FIDO verification (R5).
- **Hardware-rooted impl tiers** — live_stream (voice/video/telemedicine/livestream), accountable-stream tier: Phase-2 implementation, not design.

**E. Bounded-by-design / constitutional gaps (NOT to be "fixed" — deliberate refusals):**
- **Real-time safety-critical control loops** (SCADA, autonomous-vehicle control, closed-loop infusion) — append-only/monotonic semantics fundamentally mismatch sub-second control; `infrastructure_control` is NEVER_ALLOWED. Attest post-hoc, never drive the loop.
- **Biometric uniqueness / proof-of-personhood / biometric access / live proctoring** — `biometric_inference` + `surveillance_mass` are prohibited apophatic categories. A deliberate constitutional gap; needs privacy-preserving design (local match → witness attestation only) or stays bridged to external identity-proofing.
- **Secret / coercion-resistant receipt-free ballots** — attestations are signed/attributable by design ("anonymity to outsiders, accountability to insiders"); needs future ZK composition.
- **Anchoring a record to physical dirt/object (oracle problem)** — CEG composes claims, never adjudicates physical truth. Outside the grammar by construction.
- **Hard DRM copy-prevention** — honest-holder model, "once shared always shared"; no tamper-proof client. No clean expression, by design.

---

## 5. THE BRIDGES — the honest boundary, and why it is acceptable

Every function that reaches outside the grammar does so for one of **four** principled reasons. In each, the fabric records the **trust claim**; an external rail performs the **atomic / physical / legal / fiat act**. The split is the design, not a deficiency.

| Bridge class | What crosses out | Why it must | What CEG keeps in-grammar | Why acceptable |
|---|---|---|---|---|
| **Settlement / atomic-exchange** | Value movement, atomic pay⟺release, DvP, escrow, finality, clearing, order-book, bearer value | Bilateral simultaneity is classically impossible for unilateral monotonic attestations (Even–Goldreich–Lempel; CC 1.7); needs a totally-ordered ledger | The `settlement` receipt, `settled_action_ref`, self-auth via Identity=Wallet, the obligation/invoice/dispute records | The rail does the one thing CEG provably can't; CEG self-authenticates the receipt (signer controls the wallet) and never has to reverse value |
| **Legal force / enforcement** | Court compulsion, incarceration, asset seizure, NCMEC reporting, patent/monopoly grant, marriage/birth/title legal recognition | The fabric holds no monopoly on violence; legal effect is the state's | The `takedown_notice{CourtOrder}` artifact, the authority chain, the immutable attributable audit leaf | CEG provides auditable evidence + accountable attribution (takedown-isn't-a-coup: coordinated + attributable + revocable); the coercion is the state's job |
| **Physical-world actuation** | Unlock door, shut valve, shed grid load, drive a vehicle, dispatch responders | Append-only attestation cannot *be* an imperative effect; monotonic semantics ≠ real-time control | The `delegates_to`/`key_grant` **authorization**, the rule-as-data, the post-hoc state attestation | Mirrors the settlement bridge: authorize in-grammar, act on an effect-rail, get a signed receipt back. Safety-critical control is a deliberate apophatic refusal |
| **Fiat / regulatory / identity anchoring** | Government age/identity credentials, KYC to legal level, biometric uniqueness, tax IDs, hardware-quote chains | Physical truth and sovereign attestation originate outside any digital fabric | The witness-reserved rungs (`age_assurance:government`, `attestation:hardware_rooted`), region-local operational fields, the ladder that composes them | CEG composes an honest L1-L5 trust ladder and marks single-source confidence ≤0.5; it never *claims* a fact it can only *receive* from an external authority |

**Why this is the right boundary.** The fabric's entire value is that it is **agency-free, monotonic, and append-only** — which is exactly what makes it a trustworthy *record and trust-composition layer*. The same properties make it structurally incapable of atomic exchange, imperative actuation, and physical adjudication. Rather than smuggle a trusted-third-party or a global ordered ledger into the grammar (which would break its guarantees), CEWPOS **names the boundary once** (CC 1.7) and bridges every value/physical/legal/fiat act to a rail that owns that capability. *That it happened* is permanent and in-grammar; *the act itself* is the rail's. The agent plugs in last, on top of a substrate that already expresses ~82% of civilization with no agency at all.