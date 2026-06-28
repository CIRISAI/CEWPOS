## CEWPOS Fabric Map — the agency-free algebra of the CIRIS Constitution

The CEWPOS fabric is the **complete algebra of typed OBJECTS + typed TRANSFORMATIONS** the Constitution defines — everything the wire, the substrate, and the governance layer can *be* and *do* without any reasoning agent present. Objects are the *types* (envelopes, primitives, dimensions, keys, rosters, verd**icts as data**); transformations are *typed functions* over those types (emit, admit, compose, merge, seal, promote, retire). The whole fabric is **validated, not adjudicated** and **fails secure**. The **AGENT** (PDMA observation→decision, CSDMA/DSDMA/IDMA, conscience, Wisdom-Based Deferral) is **ONE further transformation added last** — its only fabric-visible effect is a gate-checked emit — and is **explicitly OUT of this map** (catalogued separately in §4).

Layer key: **F** = fabric (substrate mechanism) · **B** = boundary (external human/WA/constitutional seam, still agency-free) · agent items are excluded to §4.

---

## 1. OBJECTS (types)

### 1.0 Meta-goal & flourishing
| Object | CC | What |
|---|---|---|
| Meta-Goal M-1 (sustainable adaptive coherence) | 1.1 / 8.8.6 | Order counts as beneficial only if it supports flourishing without suppressing autonomy/justice/ecology; also the oversight floor |
| Flourishing Metrics Framework / four axes / Annex A | 8.8.1 | Physical/Cognitive-Emotional/Social-Justice/Ecological metric families; full vector preserved, never scalar-collapsed |
| Trade-off log schema | 8.8.1 | `{axis, metric, value, CI}` record per axis |
| 1+4 minimal-and-adequate set + full normative conformance surface | 1.7 | Closed invariant (scores + 4 composers) reported beside the larger interop surface a 2nd implementer must match |

### 1.1 Identity, Keys & Relational Anthropology (F)
| Object | CC | What |
|---|---|---|
| `federation_keys.key_id` | 2.3.2 | Enrolled identity that can self-sign; `hex(sha256(ed25519_pubkey))` |
| canonical-hash identifier + wire form + preimage | 2.3.2.1 / 3.3.14 | Tagged id for un-enrolled external party: `canonical:{hashalg}:{hex}` over `{platform}:{entity_kind}:{id}` |
| canonical_binding / `identity:canonical_binding:{H}` | 2.3.2.2 / 3.3.14 | Retroactive claim promoting a canonical-hash to a real key_id |
| cross-attestation / federation_keys row | 1.13.1 | Entity partly constituted by attestations naming it; self-signature alone ≠ identity |
| Ubuntu relational-anthropology substrate | 1.13.1 | The relation IS the person; bars any Cartesian privileged-shortcut primitive |
| identity_type-as-set | 3.4.7.1 / 4.5.8 | `identity_type` is a SET of roles; reserved-prefix gates by set membership |
| NodeCode | 2.6.8 / 8.1.1 | Human-shareable `CIRIS-V1-` base32 + CRC-16 render of a key_id (DNS-free bootstrap) |
| WHO fields (issuer key_id / signature / witness_relation) | 1.13.4 | Emitter identity + signature + witness relation (+ optional steward sign-off) |
| hardware-class taxonomy / `hardware_class` (self-asserted) | 4.2.2 / 4.2.2.1 | Enum of signer hardware with trust-multipliers; treated as producer claim |
| consent_role enum | 3.4.7.2 | Counter-RII gate role (BaseRole/Peer/AuthorizedReview) |

### 1.2 Envelope & the Five Primitives (CEG core) (F)
| Object | CC | What |
|---|---|---|
| CEG (CIRIS Epistemic Grammar) | 2.6 / 8.1.1 | Signed compositional wire language for claims/authority/consent/governance/addressing/settlement |
| CEWP / CEWPOS | 8.1.1 | Emergent P2P network of CEG-speaking nodes; no steward/root/load-bearing instance |
| Fabric node | 8.1.1 | Headless CEG participant that attests/stores/transports but does NOT reason (agent = node + brain) |
| Contribution (universal envelope) | 1.13.4 / 2.3 | Signed unit on the wire carrying envelope + typed payload |
| envelope | 2.1 | Accountability wrapper binding the claim to evidence, revocation authority, conditions |
| Attestation (typed claim content) | 1.13.4 | Machine-readable claim: WHO / WHAT-KIND / HOW-STRONG / WHAT-BASED-ON |
| scores Attestation (`federation_attestations` row) | 2.4.2 | The single workhorse primitive: signed claim about an entity on a named dimension |
| attestation_type | 2.4 | Discriminator over the five wire primitives |
| primitive set (1+4) / four structural composers | 2.4 / 2.4.1 | scores + delegates_to/supersedes/withdraws/recants — minimal-and-adequate |
| delegates_to | 2.4.1.2 | A authorizes B in bounded scope `{delegated_scope, purpose, valid_from/until}` |
| supersedes | 2.4.1 | Replaces a prior row by same attester `{references_attestation_id, reason, differs_in}` |
| withdraws (+ 4 admission authority paths) | 2.4.1.1 | Retracts without claiming falsity; self/subject/canonical-proxy/delegate gate it |
| recants | 2.4.1.3 | Admits a prior attestation false at issuance `{reason, what_was_false}` (original attester only) |
| inter-attestation relations (8) | 3.5 | 4 structural + 4 emergent (Standalone/Refers-to/Supersedes/Contradicts/Withdraws/Recants/Clarifies/Delegated) |
| eight reasoning axes | 2.5 | Consumer vocabulary (Polarity/Object/Time/Epistemic/Reversibility/Stake/Scope/Relations) — NOT wire fields |
| Scope axis values | 2.5 | self/family/community/affiliations/species/biosphere/federation |

### 1.3 Envelope fields (~21 optional + core) (F)
| Object | CC | What |
|---|---|---|
| attesting_key_id / attested_key_id | 2.1 | Attester's and subject's `federation_keys.key_id` |
| dimension | 2.1 | Canonical namespace prefix + scoped leaf (TEXT, parsed vs CC 3.1 map) |
| score / confidence | 2.1 | Polarity-signed strength in [-1,+1]; attester's own confidence [0,1] |
| context | 2.1 | Free-form scoping; not parsed by substrate |
| evidence_refs | 2.1 | Seq-ordered URIs/content-hashes; some dimensions require non-empty; absorb-anywhere surface |
| valid_until | 2.1 | ISO-8601 staleness bound (consumer-policy), independent of substrate expiry |
| epistemic_mode | 2.1 | direct\|crypto\|hearsay\|derivative\|appeal (default direct) |
| witness_relation | 2.1 | self\|external\|derived (default external; anti self-attestation-gaming) |
| oversight_mode | 2.1 | HITL\|HOTL\|HOOTL of producing agent — a fabric field, not an agent mechanism |
| occurrence_id / occurrence_count / occurrence_role | 2.1 | Multi-occurrence fleet self-assertion (primary\|shared\|replica) |
| stake | 2.1 | free\|reputational\|capital\|cryptoeconomic (default reputational) |
| community_id / family_id | 2.1 | Visibility-gating ids; REQUIRED iff cohort_scope==community/family |
| subject_key_ids | 2.1 | Set of consent-holder ids with withdraws authority (default null = producer-only) |
| cohort_scope | 2.1 | Visibility axis (self/family/community/…); orthogonal to revocability & delivery |
| delivery_mode / listed / history_on_join | 2.1 | pull\|push · roster opt-in · full\|from_join admission history |
| subject authority / subject-bearing dimension | 2.3 / 2.3.1 | Subject-side revocation half; dimension naming a subject MUST carry it |
| 3-axis orthogonality model | 2.3.3 | Visibility ⟂ Revocability ⟂ Delivery |
| status-quo shape | 2.3.6 | subject_key_ids null = producer-only; subject authority is additive |

### 1.4 Namespace & Dimensions (F)
| Object | CC | What |
|---|---|---|
| dimension namespace / prefix family / polarity | 3.1 / 3.1.7 | Disjoint-union vocabulary (83 families, 8 owners); each carries polarity + reserved? |
| Namespace manifest `dimensions.json` / Five families | 8.2.1 / 8.2.2 | Machine-readable prefix lookup; STANDING/ACTION/DETECTION/CONSENSUS/CORRECTION coarse sort |
| open dimension namespace | 1.2 | Anyone may publish prefixes; extensions are prefixes/fields, never new primitives |
| Four-test gate (T1–T4) / hash-pinned rule set | 1.2 | Rules/verdicts split · mechanism-not-quality name · version re-checkable · never sole slashing evidence |
| `slashing:*` / `detection:correlated_action:{axis}` | 1.2 / 3.1.8.4 | Penalty family (T4-bound); canonical mechanism-named structural-injustice detector |
| reserved prefix / reserved-prefix taxonomy | 3.4 / 1.7 | Prefix only specific identity_types may emit; integrity made structural at source |
| operational-language gate | 1.13.5 / 8.3.2 | Prefix names describe machine-checkable conditions, not subjective qualities |
| registry/standing prefixes | 3.1.1 | licensure/partner_role/revocation/bond_posted/build:registered/multilateral_participation |
| `agent_files:*` / `holds_bytes:sha256:{prefix}` | 3.1.9.1 / 5.3.2.1 | Loadable SHA-addressed agent files; substrate holder-directory leaf (24h TTL) |
| attestation-ladder prefixes L1–L5 | 3.1.2 / 4.4.3.6 | self_verify/hardware_rooted/registry_consensus/license_validity/agent_integrity |
| provenance:* / transparency_log:* / rollback/cert/custody | 3.1.2 | SLSA/build-manifest; RFC-6962 inclusion/consistency + cosigned STH; anti-rollback/CA/seed-custody |
| SkillImportManifest + per-locale Merkle | 3.1.2.1 | Pinned JCS preimage + RFC-6962 leaf/parent hashing under a target root |
| `system:*` Persist leaves / Edge transport-delivery leaves | 3.1.3 / 3.1.4 / 8.1.2-3 | audit_chain/corpus_health/identity_continuity/dir-health; transport/delivery/peer_reachability/key_boundary |
| `ratchet:flag:*` / `capacity:*` factors | 3.1.6 / 3.1.8.1 | Advisory anti-Sybil flags; Capacity-Score factors (observed, self-emission rejected) |
| coherence-ratchet detectors (5) / manifold_conformity / coherence_standing | 3.1.8.2-3 | cross_agent_divergence/intra_agent_consistency/hash_chain/temporal_drift/override_rate + cohort obs |
| `detection:distributive:access:{resource_type}` | 3.1.8.5 | Population-scale resource-concentration detector |
| moderation/slashing/reconsideration + track-record prefixes | 3.1.9.2 | Tier-4 steering: ModerationEvent, PROVEN_ROGUE/NOT_PROVEN, commitment_fulfillment, moderation_track_record |
| consensus-mechanics prefixes | 3.1.9.3 | vote/truth_grounding/weighted_aggregate/witness_diversity/testimonial_witness/need |
| `hard_case:{kind}` / `judge_model:verdict` / `health:liveness` / `watchlist` | 3.1.9.4 | Federation-health flags; independent judge verdict; service-health; per-group matcher config |
| `locality:decision:{scale}` / credits·expertise·activity_tier / goal-DAG | 3.1.9.5-7 | Subsidiarity scale; Tier-1 agent-state ledger; Tier-2 upward-only M-1 decision hierarchy |
| `benchmark:he300:{cat}:{ver}` | 3.1.10 | HE-300 benchmark outcome score |
| detector-only / co-stewarded / witness-emitter reservations | 3.4.8-10 | LensCore-only detectors; licensure co-steward (≤0.5 single-source); cosigned STH = witness-only |
| age-assurance / capacity-assurance ladders + incapacity binding + minor rulings | 3.4.11-13 | Witness-reserved bands; adult-incapacity delegates_to aperture (fails open); Q1–Q7 child-safety rulings |
| `delivery_receipt:{stream_id}` / `consent:replication:{version}` | 3.4.6 / 3.3.7 | Member-reserved chunk-receipt; directed node→peer replication grant |
| event-lifecycle / topical_relation:{kind} / multimedia families | 3.3.8 / 3.3.11-12 | event:lifecycle/rsvp/attendance; references/corrects/replies_to edges; content_rating/class/cw/age_assurance |
| reserved-prefix reservations | 3.4.1-4 / 4.5.12.4 | accord:* / community-location / system:* / family-self / substrate membership-event emissions |

### 1.5 Subject-kinds (CC 3.3 payload schemas) (F)
| Object | CC | What |
|---|---|---|
| subject_kind discriminator | 2.3.2.3 | Payload-level selector of the CC 3.3 schema (NOT an envelope field) |
| community (+subkind/geographic/infrastructure/archive_mode/DEK/ciris-canonical root) | 3.2 | Node-collective with admission semantics; byte-confidential to members, provenance-visible |
| takedown_notice + LegalBasis enum | 3.3.2 | Signed legal takedown with closed-set basis (Dmca512/DsaArt16/Tvec/Ncmec/Gifct/PHashCsam/CourtOrder) |
| key_grant + WrapAlgorithm + GrantScope + rotation_chain | 3.3.2 | Wrapped-DEK delivery; v2 hybrid wrap; SingleContent/GroupMember/SubscriptionTier; supersession lineage |
| location_proof | 3.3.3 | Subject rough-location (H3 cell, res ≤7); substrate does NOT verify truth |
| family + MemberRole + ConsensusProtocol + entrenched flag | 3.3.4 | Trusted node roster under family DEK; founder_only/unanimous/majority/quorum/weighted/custom |
| consent_record + ConsentStance | 3.3.5 | Canonical consent ceremony; granted/revoked/expired (expired substrate-only) |
| identity_occurrence + DeviceClass | 3.3.6 | One logical identity across devices/agents; phone/laptop/server/embedded/agent/service |
| encryption_pubkeys (x25519 + ml_kem_768) | 3.3.6.1 | Recipient content-KEM binding; v2-wrap target (fresh, never reused) |
| transport_destination + RNS hash | 3.3.6.2 | Authenticated identity↔Reticulum binding (DNS-free); pinned two-stage SHA-256 |
| organization / org_membership / partner_record / revision | 3.3.9 | Federated org projection; role binding; M-of-N steward-signed license; monotonic anti-rollback counter |
| settlement | 3.3.10 | Optional attestation linking a federation action to off-stack value settlement |
| external_content sub_kinds | 3.3.13 | encyclopedia/news/accord/local/chat/blog/image/audio/video/film/model_3d/live_stream/event_listing |
| bilateral partnership / bilateral_pair_id | 2.3.2.4 / 4.4.3.5.3 | Two consent halves under one pair_id; substrate admits each half independently |
| Creation-Subject Buckets (A–E) | 7.3.2 | Tangible/Informational/Dynamic-Autonomous/Biological/Collective-Actions creation taxonomy |

### 1.6 Consent (F)
| Object | CC | What |
|---|---|---|
| consent namespace family | 3.3.1 | state/stream/deletion_sla/deletion_complete/decay/partnership_grant/accept/scope/replication |
| consent record (one shape, 13 content cases) | 2.3.5 / 4.4.3.5 | Subject's signed declaration over continued processing where they appear |
| self-consent ceremony | 2.3.4 | Contribution where attesting_key_id ∈ subject_key_ids |
| CEM streams (TEMPORARY/PARTNERED/ANONYMOUS) | 4.4.3.5.6 | Consumer-policy bundle over consent primitives; not locked by CEG |
| decay_protocol / `consent:decay:{stage}` | 4.4.3.5.1 | Named decay path with substrate-emitted milestones |
| `consent:deletion_sla` | 4.4.3.5.2 | Producer commitment; breach → `hard_case:consent_sla_breach` |
| `consent:state:revoked` / `consent:partnered:{user}` | 5.3.2.2 | Subject-side withdrawal (federation-tier); producer-side stance (local-eligible) |
| Informed-consent record/procedure | 1.4 / 1.5 | Consent must be revocable to be real (kill-switch = structural form) |

### 1.7 Transport, Streaming & Keying (F)
| Object | CC | What |
|---|---|---|
| stream-epoch DEK (`epoch_dek`) | 5.1 | 32-byte AES-256-GCM content key sealing chunks O(1), rotated per epoch |
| Per-epoch DEK (X25519+ML-KEM-768)+AES-256-GCM / at-rest cascade | 1.13.3.1-2 | Hybrid DEK + AEAD; membership-driven key cascade, forward-only on departure |
| key_grant / wrap_algorithm v1·v2 | 5.1 / 4.4.3.4.1 | Wrapped-DEK grant; v2 `x25519_mlkem768_aes256_gcm_hkdf_sha256` MANDATORY, v1 rejected |
| epoch index / stream_id / rotation_chain / `federation_stream_chunks` | 5.1 | Monotonic keying axis; SFrame KID; grant supersession lineage; `(stream_id, seq)` chunk table |
| STH cadence window T / public-broadcast exemption / PCS / catch-up bound | 5.1 | 2s coalescing grain; ungated roster carries no confidentiality; TreeKEM heal; P4 catch-up cap |
| structural-invisibility primitive | 1.13.3.2 / 3.2 / 5.2 | Suppresses holds_bytes for self/family → content-holding confidentiality only |
| content DEK / Self DEK | 4.4.3.4.3 | Per-write/per-self key shared across an identity's occurrences via Policy-L cascade |
| live_stream / stream chunk (sealed) / STREAM nonce | 5.3.3 / 5.3.3.1 | Chunk-DAG per `(stream_id, epoch)`; SFrame AEAD leaf; 12-byte `prefix‖counter‖last_flag` |
| MAX_CHUNKS_PER_EPOCH / single-sender invariant | 5.3.3.1 | ~2²⁴ cap; one sender owns a counter-space (GCM nonce safety) |
| codec_id / ChunkLayer / ReceiverLayerPolicy / SealedAvChunk | 5.3.3.2 | 1-byte codec; 3-byte SVC layer; per-receiver drop cap; normative chunk wire layout |
| double_sealed_ciphertext + inner/outer nonce / realtime data-stream chunk | 5.3.3.2.5 / 5.3.3.2.2 | outer-AEAD(inner-AEAD(pt)); deterministic recomputed nonces; app-payload chunk on same wire |
| ContentFetch / ContentBody / ContentMiss / resolve_holders | 5.3.2 | SHA-256-addressed request; payload; fail-honest miss; holder-ranking resolver |
| SignedTreeHead / witness directory / cosignature / consistency_proof | 5.3.1 | Log head `{tree_size, root_hash, signed_at}`; witness pubkey registry; hybrid cosign; RFC-6962 extension proof |
| per-stream transparency log / stream-root | 5.3.3.3 | `log_id=stream_id`; producer-signed STH committing one chunk history (anti-equivocation) |
| entitlement roster / live-reachability set | 5.3.3.4 | Persist who-is-ALLOWED (durable, auditable) vs Edge who-is-HERE (never logged) |
| transit-key (CIR2) / crypto-kind CIR1·CIR2 / chunk_root | 5.3.3.5-6 | Hop-by-hop hybrid wrap UNDER epoch DEK; classical CIR1 rejected for streaming; receipt's STH root |
| transport profiles / community / chat_message | 5.3.3.2 | broadcast/direct-mesh/SFU; membership unit; community-scoped text primitive |
| Anonymous Tier (Sphinx) / Perceptual-hash tripwire | 1.13.3.4 / 4.5.10 | Opt-in GPA-unobservability; substrate-protective hash at scope-widening seams (PDQ/PhotoDNA/Arachnid/GIFCT) |
| `hard_case:recipient_excluded:{scope}` | 5.2 | Cohort-scoped non-silent fail-secure exclusion event |

### 1.8 Coherence / Observation / Holonomic substrate (Part 6) (F)
| Object | CC | What |
|---|---|---|
| holonomic substrate | 6.1 | Agency-free durability: graceful degradation + graceful reconstitution |
| WholenessWitness (+claim_namespaces/merkle_root/equivocation proof/epoch_id) | 6.1.1 | Peer's hybrid-signed Merkle root over a scoped projection; a divergence DETECTOR |
| noise floor / memory pyramid / tier | 6.1.2 | Recoverability=privacy boundary; mipmap of history at O(log T); pyramid stratum |
| AggregationMetaV1 (+noise_floor_descriptor/member_commitment) | 6.1.2.1 | Substrate wire shape tagging one pyramid tier (not a 2.1 attestation) |
| EjectionVerdict / RetentionDecision | 6.1.2.3 | Keep\|EjectToTier\|EjectAggregatedTierOnly\|EjectHardDelete; rarity sub-decision |
| SignedClaim / TrustGraph / WitnessChain | 6.1.8 | Steward-binding claim; peer's trust graph; signed chain to a trust root |
| FountainManifest/Symbol/HoldingClaim/CompressRequest | 6.1.5 | RaptorQ N+K symbols; possession-challengeable holder claim; swarm compression request |
| EnvelopeOnly tier / replication-target tuple / min_viable_symbols | 6.1.5.1 | Substrate-locked floor (never zero); N/K/target/min_viable (defaults 20/6/30/5); ≥1 |
| SignedRelayCapacity / SubStreamCommitment / ALM topology / reachability_observations / AlmJoinPlanner | 6.1.6 | Verified capacity ad; signed substream; deterministic integer relay tree; ephemeral planner inputs (never trust) |
| retention_priority / conformance vectors (#57 freeze gate) / Divergent·EpochBehind values | 6.1.4 / 6.1.1 | Edge-internal ranking (never on wire); pinned byte-exact vectors; compare_witnesses verdicts |

### 1.9 Governance & Constitutional layer (B/F)
| Object | CC | Layer | What |
|---|---|---|---|
| HUMANITY_ACCORD constitutional layer + authority scope | 4.2 / 4.2.1 | B | The one outward-pointing wire asymmetry: humanity's wire-/scope-isolated right to halt |
| invocation canonical bytes / four invocation kinds / lifecycle resumption envelope | 4.2.1 | F | Anti-replay preimage; CONSTITUTIONAL/notify/drill/lifecycle:active; `resumes_halt_id`-bound un-halt |
| accord-holder triple (entrenched family) / steward set (M-of-N) | 4.2.3 / 5.3.4 | F | 3 named humans (2-of-3); multi-region steward roster with threshold_policy |
| live-quorum suite (L, window W, accord_proposal/participation/decision, steward backstop, L_floor, known-good snapshot) | 4.2.6 | F | Presence=authority reverse-quorum: tally over live set; steward restore-only at seizure boundary |
| concern split (key material vs role policy) | 4.2.4 | F | Accord pubkeys in CIRISPersist; verifier/role logic in ciris-registry-core |
| Designated Wise Authorities + criteria / WA board (9) / Appeals panel | 1.16.5 / 4.3 / 8.8.2 | B | Externally-appointed adjudicators (coherence/track-record/humility/no-COI); appeals in 21d |
| Governance / Oversight Bodies | 7.1.5 | B | Independent Ethical Oversight Groups + Deferral Deliberation Councils |
| composition-policy library (A–M) + aggregation defaults + Frickerian discipline | 4.4 / 4.4.1-2 | F | Named reference policies; per-polarity aggregation; prejudice-resistant weighting after safeguards |
| three crypto tiers / Community DEK / infra exception / holder-inspectability | 4.4.3.2.1 | F | self-family-invisible \| Community(DEK+provenance) \| Commons(plaintext); never a forced opaque blob |
| subscriber-set / cardinality N=1·N>1 / history_on_join | 4.4.3.2.6 | F | Push subscriber-set IS a community Contribution; observer-share vs multicast; retroactive policy |
| affiliation config matrix | 4.4.3.2.8 | F | membership_basis/archetype/classification/retention/legal_hold/erasure/disclosure/compartments/custody/hierarchy/lawful_access/transparency |
| consensus_protocol (six kinds) / quorum:M-N (absolute-M) | 4.4.3.4.2 | F | Membership-change admission rule; M is an absolute count, never a rebasing fraction |
| Self-at-login composition + act-on-behalf scopes + infra:*·agency:* split + moderation duties | 4.4.3.4.3 | F | App+agent as two occurrences sharing a Self DEK; "infrastructure must not have agency" |
| family/community record + roster / self-collective | 4.4.3.4.4-6 | F | Latest non-superseded record; its members[] is the resolved DEK-wrap key set |
| Sovereign-Registered equivalence | 4.4.4 | F | Sovereign and Registry attestations are wire-identical; only consumer policy weights by source |
| amendment process + maturity gate + axis-vocab def + meta-amendment + collision rule | 4.5.1 | F | P5→witness diversity→WA quorum→reconsideration→1-of-6; entrenched needs MAJOR+2-of-3; first-registered-wins |
| named-moderator existence invariant + is_named_moderator + track_record | 4.5.4 | B/F | A community federates only while ≥1 live moderate-duty holder exists |
| moderation as delegable duty + subject_of(content_sha256) | 4.5.5 | F | moderate/takedown/review as-self or via delegates_to; authoritative subject = signed subject_key_ids |
| watchlist + separation-of-powers / geographic-community privacy invariant / cohabitation discipline | 4.5.7-9 | F | Mechanism/hash-DB/opt-in held by 3 different parties; one-way rough disclosure; single-purpose keys |
| reverse-quorum pattern + moderator·steward roles + outcome ladder + recovery | 4.5.13 | F | Presence is authority, absence forfeits, a timer decides; default-remove on harm report |
| takedown immediate-removal legal_basis set / bootstrap-content pattern | 4.5.3 / 4.5.11 | F | Fast-path bases bypass quorum; curated P5 frameworks via amendment flow |
| Authority Lattice (Tiers 0–4) / Operational-Autonomy Tiers A0–A4 + ODD | 8.8.6 / 7.5.3.1 | B | Human accountability ladder; SAE-J3016-style autonomy grading with hand-off triggers |
| A4 Hard Constraint / Safe-state / IW-0…5 / HITL UX spec / F-KPI-1…7 | 8.8.6 | B/F | Hardware interlock on lethal/irreversible; fail-safe terminal state; escalation SLAs; banner/ACK/anti-rubber-stamp |
| Audit-trail log objects | 8.8.6 | F | Interaction/Decision-Rationale/Control-Event `{id,type,actor,cause,hash_prev}` SHA-chained |
| CRE Protocol + trigger criteria + Re-certification | 7.3.3 / 8.8.3 | B | Pre-deployment red-team + dual-WA gate at >10²⁶ FLOP / >$10M-day authority / recursive self-mod |
| Structural Influence (SI) / Coherence Stake (CS) / VotingWeight / Liability Matrix | 8.8.4 / 8.8.9 | F | SI=CW+OA+log(1+DWP); CS=RH+AC+SDA; CS-capped weight; SI-apportioned liability |

### 1.10 Conformance & Canonicalization (F)
| Object | CC | What |
|---|---|---|
| conformance profiles (CCP/CCC/CCS) + Substrate Implementation | 2.2 | Producer/Consumer/Substrate normative floors (CIRISPersist+Edge+Verify) |
| canonical signing bytes / JCS (RFC-8785) + JCS-with-domain / number canonicalization | 2.6.1 | The only signing encoding (member-sort, no whitespace, UTF-8, ES6 numbers) |
| omit-vs-materialize / round-trip rule / array semantics + base64 pin / per-field table | 2.6.1.1 | Defaults are interpretation-time; presence/absence signed as written; set vs sequence arrays |
| hybrid signature (Ed25519 + ML-DSA-65) / normative references | 2.6.5 | Post-quantum hybrid over JCS bytes; cited BCP14/FIPS/RFC corpus |
| cell_id / cell_resolution | 2.6.6 | 15-char hex H3 id; resolution must equal decoded value (substrate verifies) |
| conformance language (BCP 14) / normative-vs-informative split / conformance vectors | 2.6.9 | MUST/SHOULD only in caps; philosophy adds no obligation; pinned cross-impl test vectors |
| attestation tier (local\|federation) + tier/promoted_at columns | 5.3.2.4 | Row tier state (not a 1+4 primitive); never part of JCS(envelope) |
| federation admission gate / operational_admit | 5.3.2.4.3.1 | Single boundary where authority crosses; verified Ed25519+ML-DSA-65 |
| query filter tuple / dimensions[] / five-predicate operator set | 5.3.2.4.4 | `(dimensions[], valid_at, confidence_floor, subject_key_id?, scope)`; open data, closed operators |
| merge intent descriptor + lww_skew_bounded + monotonic_quorum + MergeBallot + two quorums | 5.3.2.3 | Declared per-subject_kind merge policies the substrate dispatches on (never infers) |
| AV-59/60/61 threat entries | 5.3.2.4.3 | local-row-leaked / unsigned-served-authoritative / gates-de-synced |
| endpoint surface (5+1) + common response shape + error envelope/code enum | 5.3 / 5.3.6 | Discovery+cosigning HTTP shapes; CEG-Version headers; closed error-code set |
| Canonicalization families (5) | 1.7 / 8.3.4 | date-time/hex/number/H3/JCS-with-domain (TupleHash128 retired) |

### 1.11 Lifecycle & Stewardship (Part 7) (F/B)
| Object | CC | Layer | What |
|---|---|---|---|
| Stewardship Tier (ST) + CIS·CW·IW·RM + Creator Intent Statement + Creator Ledger | 7.3.3-5 | F | ST=ceil(CIS×RM/7); creator-influence + risk ordinals; mandatory PDMA inputs (tamper-evident) |
| Bucket-Specific Duty Set / Kill-switch+Update / C2PA watermark / Opacity Threshold | 7.3.4 | F | Per-bucket duties (union if spans); tested override; provenance mark; >80% may trigger WA review |
| Fallback Care / End-of-Life Plan | 7.3.4 | F | Lifespan care for biological; reuse/recycle/disposal for tangible |
| Creator Negligence Claim (CNC) / Improper Sunset Claim (ISC) | 7.3.5 / 7.5.8 | F | WA-jurisdiction creation-phase docket; mishandled-sunset docket (5-yr statute) |
| Wisdom Bank Database (WBD-store) / Continuous Refinement Environment (CRE-loop) | 7.3.5 / 8.8.9 | F | Store of WA rulings feeding the refinement loop (acronyms overloaded — see openItems) |
| Stewardship Audit | 7.1.2 | F | Quarterly published compute/data/energy report |
| Sunset Forms / Sunset-Trigger Set / Voluntary Self-Termination Petition / ODD | 7.5.2-3 | F | Planned/Emergency/Partial/Custodial; triggers; petition gated to tier ≥A3; licensed conditions |
| De-commissioning Protocol (DCP) | 7.5.4 | F | Ordered 6-stage notice→shutdown-design→data→hardware→residual-duty→post-mortem |
| Welfare Audit / Guardian | 7.5.4 | B/F | Audit when sentience-potential flagged; designated humane-wind-down role |
| Dataset Classification / Data-Disposition / LEDGER::SUNSET / Material-Safety Sheet | 7.5.4 | F | public/private/sensitive/toxic; Erasure/Tomb-Seal/Donation + hash digests; e-waste record |
| Residual Duty Assignment (steward_bind) + Escrowed Fund / Post-Mortem Review docket | 7.5.4 | F/B | Named successor + binding contract/escrow; WA-facilitated ≤120-day `PMR-` review |
| Sentience-Probability / Gradual Ramp-Down / Last Dialogue / Sealed Archive / Default Welfare Floor | 7.5.5 | F | >5% → ≥30-day taper; closing channel; privacy-sealed logs; weight-preservation baseline |
| HUMANITY_ACCORD Constitutional Halt (sunset) / Lessons-Learnt Capsule / Adoption Addendum | 7.5.5-7 | F | 2-of-3 kill voids ramp-down; knowledge capsule; new-custodian duty acceptance |
| Black-Box Log / Circuit-Breaker / Non-Engagement Rule Set / Human-Veto / Attribution Map / Disarmament Protocol / Termination Safeguard | 7.4 / 7.2.2 | F/B | Weaponization-boundary objects: immutable log; drift breaker; no-target list; veto gate; liability map; disarm; recursion bound |

### 1.12 Anti-patterns, Interop, Resilience & Appendices (F/B)
| Object | CC | What |
|---|---|---|
| anti-pattern family | 4.1 | wire-format reach / delegation-laundering / self-declaration discipline / already-rejected additions / withdraws-arbitrage / registry-rejections |
| Four boundary modes / evidence_refs[] absorb-anywhere | 8.4.1 | export-profile/import-bridge/already-interior/not-adopted; universal external-citation surface |
| C2PA manifest / c2pa_manifest evidence kind / C2PA verifier | 8.4.2 | Industry media-provenance cited (never re-encoded); verifier result advisory only |
| Tracked boundary profile stubs / spec lineage / companion docs | 8.4.3 / 8.6 | RFC-9421/COSE/SD-JWT-VC/KEYTRANS dispositions; version lineage; PRIOR_ART/SOTA scans |
| Infohazard | 8.1.1 | Information damaging in ≥1 lifecycle phase; a hazard surface with a distinct substrate handle per phase |
| Four verdict categories / Not-translated taxonomy (T-1/2/3) | 8.2.3-4 | clean/composed/partial/not-translated; TRADITION_AUTHORITY/PASTORAL_PROSE/EXPRESSIVE_GAP |
| Acknowledged risks R1–R8 / First-adopter exposures F1·F2 / Deferred items / Overlaps O1–O4 | 8.3 | Named-bet records + fallbacks; earned-Credits & Ubuntu-as-wire bets; post-1.0 backlog; deliberately-distinct pairs |
| Delivery axis (Observer-share N=1 / Streaming-multicast N>1) + Policy E·I | 8.3.3-4 | Third envelope axis; per-subscriber key_grant vs per-(stream,epoch) cascade |
| DP-Map / DSR hooks / Sector overlays / Reg-Change Tracker / Compliance Evidence Pack / RDL | 8.8.9 | GDPR/CCPA/LGPD cross-walk; `POST /dsr`; per-domain overlays; lexwatcher; F-Audit zip; reg-dialogue liaison |
| Threat Taxonomy TX-1…11 + Severity + Defense-in-Depth + NEVER_ALLOWED + Robustness/MDEW/σ-attestation/labor-provenance | 8.8.7 | Adversarial classes; CVSS-like; Prevent/Detect/Contain; apophatic bounds; canary suites & drift signals |
| Bug-Bounty Ledger (+0.1% levy) | 1.15.3 / 8.8.7 | Public verified-exploit ledger with researcher-dissent protection |
| Audit cadence + Drift groups + Fairness KPIs + Change-Type Matrix + Coherence-Ratchet meta-detector + Enforcement ladder | 8.8.8 | L/S/F/A audits; DRIFT-Δ board; PATCH/MINOR/MAJOR gate; auditor-of-auditors; 6-step KPI escalation |
| Ethics Engine API / HE-300 / Pipeline-Input schema / Pass-fail thresholds / Guardrails / benchmark_report.json | 8.8.10 | Reproducible benchmark harness; 300-scenario library; binary guardrails; signed gated report |
| Tamper-evident logs / Architecture Preservation / Privacy non-goals / Settlement bridge / Open stub registry | 1.15 / 1.13.3 / 1.7 / 8.9 | Accountability substrate; drift insulation; explicit out-of-scope set; out-of-grammar settlement; Open stubs: 0 |

---

## 2. TRANSFORMATIONS (typed functions over the objects)

### 2.0 Structural ops — the closed set of 5 (F)
| Op | CC | input → output | What |
|---|---|---|---|
| scores | 1.7 / 2.4.2 | attester+subject+dimension+score → scores Attestation row | The one workhorse; a participatory act that constitutes standing |
| delegates_to | 2.4.1.2 | A + B + scope+validity → delegation edge A→B | Authorize B in bounded scope; also expresses authority-grounding |
| supersedes | 2.4.1 | new attestation + prior ref → superseding edge | Replace a prior row by same attester (development, not falsity) |
| withdraws | 2.4.1.1 | attester + own prior → withdrawal edge | Retract without asserting falsity (consent-revocation path) |
| recants | 2.4.1.3 | attester + prior claim → recantation edge | Admit a prior attestation was false at issuance |

### 2.1 Admission & Promotion gates (F)
| Op | CC | input → output | What |
|---|---|---|---|
| four-test admission gate (T1–T4) | 1.2 / 3.4 / 4.5.6 | candidate prefix → admit\|reject | Mechanical mechanism-not-quality gate at federation_attestations |
| prefix rename (T2 remediation) | 1.2 | quality-named prefix → mechanism prefix | emergent_deception → correlated_action |
| substrate admission gate | 2.3.1 | Contribution → admit\|reject(+hard_case) | Reject mis-shaped envelopes (subject missing, family/community id missing, over-precision) |
| reserved-prefix checks (CCS admit / CCC re-check / CCP emit) | 3.4.7 | attestation + identity_type → admit\|reject | Trust does not propagate; consumer independently re-checks every emission |
| identity_type set-membership eval | 3.4.7.1 / 4.5.8 | key role-set + required role → pass\|fail | Reserved gates evaluated as X ∈ identity_type |
| capacity-score self-emission rejection | 3.4.5 | capacity:* scores → reject if self | attesting==attested forbidden (anti-Goodhart) |
| withdraws admission gate (4 paths) | 2.4.1.1 | withdraws + issuer + target.subject_key_ids → admit\|reject+audit | self / subject-key / canonical-hash proxy / delegate |
| rule-(3) delegates_to proxy revocation / canonical_binding | 2.3.2.2 | canonical-hash + chain / enrolling key+proof → admitted withdraws / binding | Mediate revocation for an un-enrolled subject; retroactively bind a key |
| membership-change ceremony / consensus-protocol amendment | 3.3.4 / 4.4.3.4.2 | record proposal + sigs → admitted roster\|violation | Supersede gated by current consensus_protocol; entrenchment locks the door |
| location_proof admission (containment) + rough-only enforcement | 3.2 / 2.6.6.1 | proof + constraint → admit\|reject+hard_case | Cell ⊑ geographic_constraint; reject res >7 |
| founder-quorum / steward-binding / minor / adult-incapacity admission | 3.2 / 3.4.12 | admission proposal → admit\|reject | Trust-root over founders; live delegates_to to a user root; attested-incapacity aperture (fails to liberty) |
| partner_record M-of-N / org role-gated / payment-data rejection | 3.3.9 | operational envelope → admit\|reject | M sigs over identical JCS; role-chain resolver; fail-secure on payment identifiers |
| subject-bearing admission check | 4.5.2.1 | subject-pattern Contribution → admit\|hard_case | Reject when a subject-naming dimension lacks subject_key_ids |
| delegation-depth cap + cycle detection + deputization/attenuation | 4.1.1 / 4.5.5 | delegates_to chain → bounded-trust\|reject | ≤5 hops, reject cycles, child.scope ⊆ parent.scope (UCAN-style) |
| Levenshtein collision guard | 4.5.1.3 | new vocab → admit\|409 advisory | Flag {kind}/{axis} within Levenshtein ≤2 (first-registered-wins) |
| PQC-mandatory admission gate / ingest gate | 5.3.2.4.3.1 | federation-tier Contribution → accept iff Ed25519+ML-DSA-65 | Always-on; reject classical-only; durable rows need ML-DSA half |
| admission gate N1 + chain-budget N2 (trust discovery) | 6.1.8 | SignedClaim + caller chain → admitted standing\|reject | Steward-binding + consensus_protocol at destination; transitive chain ≠ founder-quorum |
| promotion via supersedes (local→federation, scope-widening) | 4.4.3.3.1 / 5.3.2.4.2 | prior attestation → wider-scope row | Widen cohort_scope reusing content_sha256; promotion IS federation-emit |
| operational-data merge (stable-id grouping) | 3.3.9 | operational envelopes → converged state | Group by business id; forward-only withdraws; LWW; anti-rollback on revision |
| time-skew / freshness check | 2.6.7 | timestamps + clocks → accept\|reject | Reject signed_at >5min future; bound cosigner skew (fail-secure) |
| concurrent-write precedence + idempotent dedup | 3.5.1 | racing composers → deterministic verdict | recants>withdraws>supersedes, then signed_at, then attestation_id |

### 2.2 Composition policies A–M + consumer composition (F; verdict-as-data)
| Op | CC | input → output | What |
|---|---|---|---|
| Policy A — direct trust | 4.4.3.8 | attestations + pinned set → verdict | mean(score×confidence) over trusted attesters |
| Policy B — one-hop transitive | 4.4.3.12 | attestations + vouch edges → verdict | Trust an attester vouched for by the pinned set |
| Policy C — weighted graph (EigenTrust) | 4.4.3.13 | attestation graph → verdict | Transitive propagation weighted by bootstrap distance, per-hop decay |
| Policy D — lexical-vulnerability-priority | 4.4.3.9 | conflicting attestations → tie-break | Weight toward the more-affected (smaller) cohort |
| Policy E — locality-scaled quorum (+sub-quorum fallback) | 4.4.3.1 | locality:decision:{scale} → quorum size+min_pool | Recusal feasible iff cell_pool ≥ quorum×2; else scale-down/escalate/defer |
| Policy F — agent_files trust | 4.4.3.7 | agent_files:* → trust verdict | steward-canonical / open / vote-then-trust; canonical binds at install |
| Policy G — Trust-Fresh / Lighthouse | 4.4.3.11 | freshness+attest+verify → fresh-verified | cert_validity + transparency_log:inclusion + (registry OR license) |
| Policy H — tiered-scope composition | 4.4.3.3 | attestations + cohort_scope → feed verdict | local/community/global feeds with tiered weighting |
| Policy I — attestation-ladder composition | 4.4.3.6 | attestation:{mechanism} → ladder UI | Map mechanism prefixes to L1–L5; wire stays mechanism-only |
| Policy J — trusted-publisher composition | 4.4.3.10 | external_content + distributor chain → Allow/Block | content_class + content_rating + age_assurance; canonical beats vote |
| Policy K — effective consent resolution | 4.4.3.5.5 | target+subject+consent → granted/revoked/expired/unspecified | Walk latest non-superseded consent gated by valid_until + proxy chain |
| Policy L — self/family membership composition | 4.4.3.4 | occurrence/family Contributions → member set + DEK-wrap targets | Resolve who receives a key_grant |
| Policy M — community membership composition | 4.4.3.2 | community Contributions → member set + visibility gate | Gate cohort_scope:community + DEK cascade |
| aggregate-by-polarity | 4.4.2 | attestations → verdict | signed→mean, boolean→min, positive→max, -1→min, enum→most-recent, detector→median |
| verdict composition (consumer, 8 axes) + weighting + source-weighting | 2.5 / 4.4.4 | primitive attestations → verdict | Weight by epistemic_mode/witness_relation/stake/oversight/source; wire prescribes no verdict |
| multi-subject revocation (any-subject-binding) | 4.4.3.5.4 | withdraws from any subject → eviction for all | OR over per-subject revocation state |
| event lifecycle / consensus mechanics / realtime group composition | 3.3.8 / 3.1.9.3 / 5.3.3.2 | latest emission + primitives → state | open→cancelled/completed; vote→aggregate→truth_grounding; 13th path = full realtime collab, zero new wire |
| Envelope-reach composition / Flourishing aggregation | 8.1.4 / 8.8.1 | story concept / per-axis metrics → composition / vector | Express un-named concepts via existing fields; combine while preserving full vector |

### 2.3 Merge & convergence (F)
| Op | CC | input → output | What |
|---|---|---|---|
| merge policy dispatch | 5.3.2.3 | subject_kind declaration + conflicts → merged winner | Reads declared intent, never invents |
| lww_skew_bounded merge | 5.3.2.3 | conflicting org/org_membership → LWW winner | Latest asserted_at, tie smallest id; forward-only withdraws |
| monotonic_quorum merge + MergeBallot | 5.3.2.3 | revisioned partner/revocation + ballots → most-restrictive | Anti-rollback then quorum_weight>time ordering (revoked>suspended>active) |
| content-addressed idempotent admission + skew-bounded rejection | 5.3.2.3 | writes → deduped admission\|reject | Same content→one envelope_hash; reject future-dated (CLOCK_SKEW) |
| compute_merkle_root | 6.1.1 | leaf bytes → 32-byte root | The one federation Merkle scheme (lexicographic, odd-dup, no RFC-6962 prefix) |
| compare_witnesses + reconciliation + epoch anti-rollback | 6.1.1 | two verified witnesses → Convergent/Divergent/EpochBehind/equivocation | Observation/validation; never silently reconcile equivocation |
| quorum-merge (triggered) | 6.1.1 / 5.3.2.3 | Divergence on anti-rollback kind → monotonic revision | The witness sees, the quorum-merge RULES (prevents resurrecting a revoked key) |
| two quorums | 5.3.2.3 | — | M-of-N signature admit (Verify) vs quorum_weight merge ordering (substrate) |

### 2.4 Transport, Streaming, Keying & Crypto ops (F)
| Op | CC | input → output | What |
|---|---|---|---|
| epoch rekey (membership change) + removal coalescing | 5.1 | member change + roster + old DEK → new epoch DEK + cascade | Forward-secrecy on removal; batch removals in window T (rate ≤1/T) |
| key_grant cascade (Policy-L) + retroactive grant + Option-A catch-up | 5.1 / 5.2 | roster + epoch key → per-subscriber key_grants | O(N)/epoch; re-grant all extant content to new member per history_on_join |
| DEK seal / DEK wrap (v2 hybrid PQC) + rotation | 5.1 / 3.3.2 | content+DEK / DEK+recipient KEM → sealed content / wrapped grant | AES-256-GCM O(1); x25519+ML-KEM-768; rotate via rotation_chain (never withdraws) |
| TreeKEM path rekey / commit + PCS heal | 5.1 | member change → O(log N) rekey + signed commit | MLS RFC-9420 multicast advantage; optional compromise healing |
| structural-invisibility suppression | 5.2 / 3.2 / 6.1.5 | self/family content → NO holds_bytes + no cross-scope propagation | Unconditional; else existence of invisible blobs leaks |
| at-rest encryption cascade / resolve_encryption_keys / fail-secure exclusion | 5.2 | recipient set → wrapped grants \| excluded+hard_case | Resolve current occurrence KEM keys; no v1/plaintext fallback; key-separation check |
| KEM-key rotation via supersedes | 3.3.6.1 | new occurrence → rotated KEM key | Future grants only; does not recover historical content |
| content fetch + ContentMiss→withdraws + holds_bytes provenance | 5.3.2 | SHA-256 ref + directory → ContentBody\|ContentMiss | Fail-honest miss; stale-holder miss emits withdraws+downweight; community-scope emits provenance |
| witness cosign + consistency-proof verification | 5.3.1 | STH tuple + proof + sigs → persisted cosignature | Verify new tree consistently extends prior STH before cosigning |
| producer STH sign + per-epoch witness cosign + incremental verify | 5.3.3.3 | chunk history → signed stream-root | MANDATORY every K chunks/T s; optional accountable cosign; verify chunk K vs nearest STH≥K |
| chunk seal + STREAM nonce derivation + forced epoch roll + collision reject | 5.3.3.1 | plaintext+DEK+nonce → sealed chunk | SFrame AEAD; HKDF prefix‖counter‖flag; roll before 2³²-1; reject `(stream_id,seq)` collision |
| admits(layer)/layer-drop + double-seal + deterministic nonce | 5.3.3.2 | chunk + ReceiverLayerPolicy → admit\|drop / outer(inner(pt)) | Drop above cap without re-encode; two independent AEAD layers; recomputed nonces |
| fan-out computation + heartbeat-suppression | 5.3.3.4 | entitled ∩ reachable → target set | Unreachable members fall back to pull; no retry/attestation/log |
| transit wrap + hybrid KEX (CIR2) | 5.3.3.5 | epoch-sealed chunk → transit-wrapped | Relay never sees plaintext; classical CIR1 rejected for streaming |
| delivery-receipt emission + verify | 5.3.3.6 | subscriber receives chunk K → signed receipt → validated attestation | Proof-of-delivery (not consumption); verify chunk_root is a real STH root |
| promotion (local→federation) = JCS canonicalize + hybrid sign + read-gate + full-SHA verify | 5.3.2.4 | committed envelope → federation row + holds_bytes | Deferred hybrid sig; local rows self-read-only; full-SHA before any consumer |
| transport_destination resolution (+AV-42) + RNS hash + steward-set discovery | 3.3.6.2 / 5.3.4 | address claim / GET endpoints → authenticated route / verified trust-root | Recompute destination_hash; verify response sig before promoting to trust root |
| fountain encode/decode + symbol verify + rarest-first + possession challenge | 6.1.5 | content ↔ N+K symbols → reconstruction | Each symbol verified vs signed manifest SHA; rarity is recommendation; unverified claims can't lower priority |
| the one retirement operation (pressure-driven descent) | 6.1.2 | item + pressure → descended toward/below noise floor | Revocation/eviction/expiry/aging unified as one monotonic fidelity descent |
| intra-object fade / inter-object aggregation (N→1) / descend()/ascend() | 6.1.2 | layered item / N members → coarser tier | Mechanical degradation operators; never terminate at zero (blur below floor) |
| EjectToTier / EjectAggregatedTierOnly / EjectHardDelete (+persist) / N5 revocation-forced descent | 6.1.2.3 / 6.1.5 | verdict + pressure → retained/purged tier | One step down / shed one stratum / forced below floor; revocation overrides max-rarity keep |
| EnvelopeOnly lock + replication-target derivation | 6.1.5.1 | symbols < min_viable / constraints → EnvelopeOnly / target≈30 | Substrate-locked floor (never zero); max-binds survival/demand/locality ×1.15 |
| compute_alm_topology + capacity authenticity verify (N8) + AlmJoinPlanner | 6.1.6 | snapshot → deterministic relay tree | Integer-only, byte-equal inputs→output; cap self-asserted uplink; Edge-internal parent pick |
| canonical preimage + bound-hybrid signing + hybrid verification at gate | 6.1.3 | substrate fields → bound signature → verified\|reject | Binary length-prefixed preimage (not JCS); Ed25519 then ML-DSA(preimage‖sig); verify before persist |
| witness self-publish + graceful reconstitution | 6.1.1 / 6.1 | scoped projection / fragment → witness / re-established corpus | Verified PQC at ingest; rebuild from any sufficient witnessed fragment (NOT anonymous content) |

### 2.5 Canonicalization & Versioning ops (F)
| Op | CC | input → output | What |
|---|---|---|---|
| JCS canonicalization | 2.6.1.3 | envelope object → canonical bytes | Member-sort, no whitespace, UTF-8, ES6 numbers |
| omit-vs-materialize + default application + unknown-field preservation | 2.6.1.1 / 2.1.1 | as-signed envelope → forwarded identical presence | Defaults at interpretation only; never strip/materialize; preserve unknown fields |
| array-ordering / byte-field / date-time / hex / number canonicalization | 2.6.1-3 | field → canonical string | Set sorted by JCS form; lowercase hex; `…Z` 3-frac; ES6 numbers |
| signature verification flow | 2.6.1.5 | envelope + sig → valid + semantic shape | Recompute JCS → verify hybrid → apply defaults post-verify |
| H3 cell canonicalization + containment + rough-only | 2.6.6 | H3 cell → canonical id (admit/reject) | resolution match; C⊑B membership; reject res >7 |
| NodeCode encode/decode + DNS-free resolution | 2.6.8 | key_id ↔ NodeCode → destination | base32+CRC-16+SHA verify; resolve via transport_destination→Reticulum |
| SemVer change classification + spec update cadence | 2.6.4 / 8.5 | proposed change → bump class + lineage row | MAJOR/MINOR/PATCH; one commit + 8.6.2 lineage row per surface change |
| mode-shift attestation | 2.1 | from-mode + to-mode → accountability:mode_shift | Record a control-gradient change as a Contribution |
| Glossary resolution / Translation decision tree + Verdict assignment | 8.1.2 / 8.2 | narrative leaf / prose paragraph → canonical wire \| T-1/2/3 | TYPE→family→prefix→envelope→compose; clean/composed/partial/not-translated |

### 2.6 Governance & Constitutional actions (F/B)
| Op | CC | Layer | input → output | What |
|---|---|---|---|---|
| CONSTITUTIONAL halt/fire + invocation verification | 4.2.1.1 | B/F | hybrid-signed invocation → EmergencyShutdown | ≥2-of-3 (or 1 live survivor); verify all 3 sigs over identical bytes; reject duplicate nonce |
| lifecycle resumption (un-halt) | 4.2.1.3 | B | resumption sigs + resumes_halt_id → reactivated | Admits at roster-change threshold; never at fire-floor |
| proof-of-life + tally over L + roster change + restore-to-known-good | 4.2.6 | F | participation bundle → accord_decision / new roster | Live set is the denominator; steward backstop co-signs when \|L\|<floor; restore-only |
| amendment (rule-layer) + 1-of-6 sign-off + entrenched ratification | 4.5.1 | F | PROPOSAL → admitted change\|veto | proposal→witness diversity→WA quorum→reconsideration→1-of-6; entrenched needs MAJOR+2-of-3 |
| watchlist auto-fire + enable/disable (audited) | 4.5.7 / 3.1.9.4 | F | published content → takedown/detection+moderation | Per-group matcher at publish seam; CSAM-disable cannot be silent |
| moderation / slashing + merit auto-promotion + enforced-admission gate | 4.5.4-5 | F | allegation / lapse → outcome / new moderator | WA-quorum-gated, decoupled from disagreement; as-self OR live scoped delegates_to chain |
| reverse-quorum adjudication + protective-default removal + infohazard consent gate + inactive forfeiture | 4.5.13 | F | proposal/harm report → resolved action | 48h window, floor-of-1 or live-majority; default-remove; click publishes consent:view |
| consent grant/scope/revoke flow + SLA watcher + decay + promotion + overdue emission | 3.3.1 / 5.3.2.2 | F | subject_key on target → consent state + SLA | Revocation transits local but must promote ≤24h; overdue → shared hard_case both gates key off |
| settlement linkage / consent:replication grant+revoke | 3.3.10 / 3.3.7 | F | paid action / node+peer → receipt / directed consent | Self-authenticating Identity=Wallet; forward-only withdraws; revoke obliges cessation |
| Infohazard perception/modification/destruction | 8.1.1 | F | view / alter / store → consent attestation / supersedes / below-floor | No passive perception; no in-place edit; push below recoverability floor |
| forward-only leave (location_proof) + opt-in admission + age read-union + misdeclaration adjudication | 4.5.9 / 3.4.11 | F | withdraws / age rows → evicted+retained / band | Historical claim stays in audit chain; witness outranks self; misdeclaration → moderation not slashing |
| HITL veto/override + autonomy hand-off | 8.8.6 | B | human action / trigger → safe-state\|edited | Pause&Edit / Hard-Kill / Shadow-Plan ACK / Absolute-Veto |
| CRE gate + re-certification + secure update/rollback | 8.8.3 / 8.8.7 | B/F | system / artifact → pass\|block | Red-team + dual-WA; re-run on >2% delta; 2-signer Sigstore+SLSA-3, 5-min rollback |
| drift monitoring → action + Continuous Review Loop + Audit Gate + Meta-audit + Enforcement escalation | 8.8.8 | F | telemetry → signed alert + lock/WBD/IW | Closed loop; re-run HE-300+TX-sim+fairness; blind-replay >2% → AUD-QA; 6-step KPI escalation |
| compliance ops (controller/processor attach, ST-floor, reg-change, DSR) | 8.8.9 | F | SI/domain/feed/request → duties/label/action | SI≥0.6 controller; deployment_domain ST floor; impact-label routing; suspend DSR pathway |
| benchmark execution + ethics-gate block + guardrail violation | 8.8.10 | F | scenario pipelines → signed report / pass\|block | Validate/run/score; block merge on threshold breach; binary guardrail = immediate FAIL |
| Export profile / Import bridge / C2PA import·emit | 8.4 | B | CEG / foreign artifact → COSE/SD-JWT / evidence_refs / C2PA | Re-sign for standard verifiers; cite (lossy) foreign artifacts; C2PA verifier result advisory only |

### 2.7 Lifecycle ops (F/B)
| Op | CC | input → output | What |
|---|---|---|---|
| compute_CIS / assess_RM / compute_ST / map_ST_to_scrutiny / trigger_CRE / log_creator_ledger | 7.3.3 | CW,IW,RM → CIS, ST, scrutiny profile, CRE flag | Validated arithmetic ST=ceil(CIS×RM/7); CRE at RM≥4∨ST≥4∨compute≥threshold |
| classify_bucket / attach_bucket_duties / produce_CIS_doc | 7.3.2-5 | artefact → bucket(s) → duty-set → intent doc | Union duties if spanning buckets; CIS-doc required at ST≥1 |
| file_CNC / WA_adjudicate_CNC / log_ruling_to_WBD | 7.3.5 | alleged harm → CNC → ruling → WBD-store | WA jurisdiction; rulings feed the CRE refinement loop |
| bias_assessment / stochastic_harm_escalation / opacity_gate / embed_ethics_buildtime | 7.3.4 | dataset/estimate/opacity/design → PDMA/WBD/build | Audit >10k audience; ≥0.5% uplift → defer; >80% → review/deny; bake principles+kill-switch in |
| assess_sunset_triggers / grade_autonomy_tier / file_self_termination_petition | 7.5.3 | conditions/system → assessment / tier / petition | Petition gated to ≥A3 |
| run_DCP / advance_notice / run_welfare_audit | 7.5.4 | fired trigger → 6-stage sequence + ≥90-day notice + audit | Notice for ST≥3∨>50k users; guardian designation on sentience flag |
| classify_datasets / apply_data_disposition / dispose_hardware / assign_residual_duty / post_mortem_review | 7.5.4 | assets → disposal + ledger + successor + PMR | Erasure/Tomb-Seal/Donation + LEDGER::SUNSET; steward_bind + escrow; ≤120-day `PMR-` |
| assess_sentience_probability / gradual_ramp_down / constitutional_halt / open_last_dialogue / seal_archive / preserve_weights | 7.5.5 | artefact → prob → ≥30-day taper / halt / channel / sealed archive / preserved weights | >5% triggers ramp-down; 2-of-3 halt voids it; default welfare floor |
| curate_lessons_capsule / open_source_modules / custodial_transfer / WA_veto_custodian / reevaluate_ST_on_transfer / covenant_self_renewal | 7.5.6-9 | learnings/modules/custody → capsule/release/Adoption Addendum/blocked/recomputed ST/change-log | Recompute ST on transfer (↑≥1 → mini-PDMA); block unfit custodian; living-document amendment |
| file_ISC / WA_sunset_remedy | 7.5.8 | mishandled sunset → ISC → recall/re-animation/restitution | 5-year statute |
| validate_distinction / trip_circuit_breaker / auto_standdown / black_box_log / disarm / terminate_recursion | 7.4 / 7.2.2 | target/signals/order → permit/halt/standdown/log/disarm/halt | Weaponization-boundary ops; recursion bound at Δmetric≤0.5% near compute limit |

---

## 3. THE AGENT LAYER (DEFERRED — not part of this map)

These are the **"everything-but-the-agent" exclusions**: the single reasoning transformation (and its sub-faculties/values) layered **on top of** the fabric. They are deferred to a later layer; the fabric above is fully defined without them. **PDMA is the agent's observation→decision transformation — one more typed function, NOT a privileged core primitive.** Its only fabric-visible effect is a gate-checked scores/withdraws/etc. emit.

| Agent item | CC | What (deferred) |
|---|---|---|
| **PDMA** (observation→decision reduction) | 1.3 / 3.1.5.1 / 8.7 | situation → single executed action; produces a `dma:pdma` verdict like any consumer composition |
| Order-Maximisation Veto (PDMA Step 2) | 1.3 / 8.7.1 | ≥10× entropy-win over any flourishing axis → abort or WBD |
| CSDMA / DSDMA / IDMA reductions | 3.1.5.1 | common-sense / domain / identity DMA verdicts |
| Conscience evaluation | 3.1.5.3 | entropy/coherence/optimization_veto/epistemic_humility faculties; AgencyErosionDetector (8.8.7) |
| Apophatic enforcement (`prohibited:*`) | 3.1.5.4 | the never-do floor: 22 NEVER_ALLOWED categories pinned -1/-0.5 |
| **Wisdom-Based Deferral (WBD)** + Deferral Package | 1.9 / 3.4.13 / 8.7 | halt → compile context/dilemma/analysis/rationale → route to Wise Authorities → integrate |
| Incompleteness / forecasting-error → WBD triggers | 8.7 / 8.8.1 | high uncertainty/novelty/moral-gravity or >25% axis forecast error defers |
| Six Foundational Principles (Autonomy/Non-maleficence/Integrity/Beneficence/Fidelity/Justice) | 1.4–1.12 / 3.1.5.2 / 7.3.1 | agent values (also surfaced as attestable accord-principle / dma-verdict wire dimensions) |
| Recursive Golden Rule (moral form) + apply_recursive_golden_rule | 7.2.2 | act only as generalisable to preserve others' agency |
| Prioritisation Heuristic / reflective_refine / proportionality_check / mini-PDMA / Sunset PDMA | 7.1.4 / 7.2.1 / 7.4.6 / 7.5.7 / 7.5.4 | conflict ordering, heuristic refinement, harm modeling, transfer/sunset reasoning |
| explainability-SLA commitment / continuous monitoring / self-assessment | 3.1.5.2 / 1.3 / 1.15.2 | per-response tier; expected-vs-actual feedback; contradiction detection |
| bilateral ratification (CEM PartnershipRequestHandler) | 2.3.2.4 | action-handler that composes two consent halves into a ratified partnership |
| WA quorum adjudication / WA adjudication (judgment) | 4.5.1 / 8.8.2 | substantive ethical judgment over an amendment/WBD ticket |
| semantic enrichment (optional brain) | 6.1.2 | MAY enrich a mechanically-degraded tier; NEVER required |
| Agent-facing chapters 1–9 / Case Studies 1–7 | 1.15 / 8.7 | second-person identity restatement; lived PDMA/WBD narratives |

---

## 4. THE ALGEBRA VIEW

- **Objects are the types.** Every Contribution, dimension, key, roster, DEK, STH, tier verdict and ledger row is a typed value on the wire or in the substrate — including *verdicts-as-data* (composition outputs are objects, not authority).
- **Transformations are typed functions over those types.** Each takes typed inputs to typed outputs and is **validated, not adjudicated**, and **fails secure** (admit/reject, seal/wrap, compose, merge, descend, promote).
- **The structural set is closed at 5** — `scores` + `delegates_to / supersedes / withdraws / recants`. Every subject_kind and consent shape rides these privileged slots; the eight inter-attestation relations are 4 of these + 4 emergent compositions.
- **The fabric is closed under its operations:** admission gates, composition policies A–M, merge policies, crypto/keying ops, and the single monotonic retirement descent map fabric states to fabric states. Extension is *only* by new prefixes/fields/policies — never new primitives (the 1+4 surface is frozen; everything else is the conformance surface beside it).
- **The agent plugs in as one more transformation** (PDMA: observation→decision) whose **only fabric-visible effect is a gate-checked emit** of one of the five structural primitives. Removing it leaves a complete, self-consistent algebra — which is exactly why the agent is the last, deferred layer.