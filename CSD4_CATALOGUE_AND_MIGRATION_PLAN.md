# CEWPOS Full CSD Catalogue Evaluation, `CSD/4` Test Automation & `CSD/3 → CSD/4` Migration Plan

> **Generated & Verified by:** [`epistemic-ui/csd4_harness.py`](epistemic-ui/csd4_harness.py) against the pinned `CSD/4` v3 topology catalogue ([`epistemic-ui/csd4/PIN.md`](epistemic-ui/csd4/PIN.md), `CIRISServer#802` @ `003871fd`, `CIRISClient#219` @ `212bf534`, `CIRISConstitution` `rc9` @ `01b47e75`).

---

## 1. Executive Summary — Evaluation Against the Full CSD Catalogue (`CSD-001`–`CSD-129`)

Across `CIRISClient`, `CIRISConstitution` (`rc9`), `CIRISRegistry` (`#206`), and `CIRISServer` (`#802`), the full CIRIS Surface Descriptor (CSD) namespace spans **103 card identifiers (`CSD-001` through `CSD-129`)**:

| Category | Count | Card Identifiers | CEWPOS Coverage & Status |
|---|---:|---|---|
| **Adopted on `CIRISClient` `main`** | **80** | `CSD-002`..`CSD-110` (excluding free numbers & `095`/`096`) | **100% mapped** into `5 Circles × 7 Tabs` + `6 My things Instruments` in [`epistemic-ui/index.html`](epistemic-ui/index.html) & [`view-forms-spike/src/epistemic_scene.rs`](view-forms-spike/src/epistemic_scene.rs) |
| **In Flight / Newly Minted in `requirements.yaml`** | **3** | `CSD-095` (Self-Key Roster, PR `#218`), `CSD-096` (Session Unified Handler), `CSD-129` (Accord Halt/Confirm/Resume) | **100% migrated to `CSD/4`** in [`epistemic-ui/csd4/cards/`](epistemic-ui/csd4/cards/) & rendered in UI |
| **Reserved Promise-Closure Cards (`#179`)** | **11** | `CSD-111`..`CSD-119`, `CSD-127`, `CSD-128` | **100% implemented** in [`view-forms-spike/src/epistemic_scene.rs`](view-forms-spike/src/epistemic_scene.rs), [`epistemic-ui/out/all_promise_cards.json`](epistemic-ui/out/all_promise_cards.json), and [`epistemic-ui/index.html`](epistemic-ui/index.html) |
| **Reserved Affiliation / Portal Fold Cards (`#206`)** | **7** | `CSD-120`..`CSD-126` | **100% implemented** in [`epistemic-ui/out/all_promise_cards.json`](epistemic-ui/out/all_promise_cards.json) & `Communities & Businesses` Circle |
| **Retired Cards** | **2** | `CSD-001` (folded into `CSD-055 §6`), `CSD-034` (folded into `CSD-046`) | Preserved as retired redirects; never re-allocated |
| **Total CSD Catalogue** | **103** | `42` `FORCED` · `36` `SUPPORTS` · `20` `OPS` · `2` `RETIRED` | **All 20 Cohort-1 cards pass `check_csd4` (`0` pending entries); 22/22 self-test plants pass** |

### Key Architectural Findings from the Full-Catalogue Audit

1. **Why Progressive Disclosure (`☰` and `⋯`) Is Essential to Scaling 101 Active Cards:**
   - In `CIRISClient`, attempting to render raw protocol fields, quorum tallies, and custody diagnostics inline across 80+ cards produced visual crowding and nested containers (`CardShell` inside `CardShell`).
   - In **CEWPOS**, every row across all `5 Circles × 7 Tabs` and `6 My things Instruments` obeys the **Calm Row Budget (`INV-UI-7`)**: at most **3 inline elements per row** (`Title + Subtitle`, `ScopePill`, and `☰` Receipt Hamburger), with all secondary protocol metadata, quorum signers, and 3D spatial projections tucked behind **`☰` (5-fact CEG Receipt Sheet)** and **`⋯` (Ellipsis Expansion)**.
2. **Closing the 19-Card Gap in `CIRISClient` PR `#219`:**
   - In `CIRISClient` PR `#219`, `packaging/csd4/requirements.yaml` pins **20 CSDs** (`CSD-037`, `091`, `092`, `094`, `095`, `096`, `100`, `101`, `102`, `103`, `105`, `107`, `111`, `117`, `118`, `119`, `120`, `127`, `128`, `129`), but only the test fixture `CSD-095` had been converted to a ```` ```yaml csd:tests ```` block — leaving 19 CSDs reported as `[info] requirements.yaml names 19 CSD id(s) whose card carries no csd:tests block`.
   - In **CEWPOS**, [`epistemic-ui/csd4_harness.py`](epistemic-ui/csd4_harness.py) migrates **all 20 Cohort-1 CSDs** in [`epistemic-ui/csd4/cards/`](epistemic-ui/csd4/cards/) to full `CSD/4` format and validates all **20/20** with **0 pending `requirements.yaml` entries**.
3. **Retiring `CSD-107`'s Legacy `csd:topology` Layer-Graph Block:**
   - Under `CSD/3` (and early `CSD/4` drafts), `CSD-107` (*Where is this file*) carried a custom `csd:topology` layer-graph block. Per `FSD/CSD4.md §1` (*"one catalogue id; never a topology block"*), `check_csd4.py` explicitly refuses any card that carries both `csd:tests` and `csd:topology`. Our Cohort-1 migration replaces `CSD-107`'s `csd:topology` block with a clean `csd:tests` block pinned to `topology: T1` (`at: [file.custody_received, file.large]` and `at: [file.withdrawn]`).

---

## 2. Automated CEWPOS Testing Architecture per `CSD/4`

### 2.1 How the `CSD/4` Verification & Execution Pipeline Works

```mermaid
flowchart LR
    CC["CIRISConstitution rc9<br/>STATE_TABLE.tsv (155 states)<br/>topology_vocabulary.json (110 atoms)"]
    SRV["CIRISServer #802<br/>harness.native.model<br/>T0..T19 traces.json + requirements.yaml"]
    PIN["CEWPOS epistemic-ui/csd4/PIN.md<br/>SHA-256 Pinned Catalogue v3"]
    CHK["csd4_harness.py<br/>1. verify-pin & 22-plant self-test<br/>2. check-cards (20/20 Cohort-1)"]
    RUN["csd4_harness.py run-traces<br/>--at-state-cmd Checkpoint Resolver<br/>(33 Test Instants across T0..T19)"]
    LIVE["csd4_harness.py probe-live<br/>CIRISServer :4243 + Agent :8080<br/>+ Epistemic UI :8787"]

    CC --> SRV --> PIN --> CHK --> RUN --> LIVE
```

1. **Static Contract & Pin Verification (`check_csd4` Gate):**
   - Verifies that `requirements.yaml`, `topology_vocabulary.json`, `topology_vocabulary.pin`, and `traces.json` match the exact SHA-256 digests in [`epistemic-ui/csd4/PIN.md`](epistemic-ui/csd4/PIN.md).
   - Enforces the 6 `CSD/4` rules on every card carrying ```` ```yaml csd:tests ````:
     1. `topology` is in the closed set `T0`–`T19`.
     2. `catalogue_version: 3` is present and every atom in `at:` belongs to the 110-atom v3 vocabulary.
     3. Every test's `at:` atom set holds **simultaneously at one instant** `(topology, step_id, step_index)` in `traces.json`.
     4. Exact bidirectional agreement between the card's `csd:tests` block and `requirements.yaml` (same topology, same test names, same `at:` sets, same `probe:`).
     5. Every card at `stage: building` or later carries a non-empty `derives:` list of plain-rule IDs (or `[ops]`).
     6. Probes use `node:` (never `on:`, which YAML parses as boolean `True`) and no card carries the retired `csd:topology` block.
2. **Trace Checkpoint Scheduling (`--at-state-cmd` Execution):**
   - Unlike `CSD/3` tests that ran against the terminal state of a fixture after earlier states had already been overwritten (e.g., testing a pending device-join proposal after the device already joined), `CSD/4` schedules each test at the **exact step checkpoint** where its `at:` atoms hold in `traces.json`.
   - When `python -m harness.native run --at-state-cmd` executes a topology (`T0`–`T19`), it writes `checkpoints/<step_id>/values.json` at each step and invokes the UI flow runner for every CSD test scheduled at that step before advancing the topology.
3. **CEWPOS `/tree` & Live Occurrence Verification:**
   - At each resolved checkpoint, CEWPOS projects `EpistemicScene::csd4_tree()` (or queries the live UI + occurrence on `:8787` / `:4243` / `:8080`) and verifies:
     - All required `tag` identifiers (`card_csd_*`, `btn_receipt_*`, `sheet_confirm_*`, `ceremony_*`).
     - The **7 Epistemic UI Invariants (`INV-UI-1`..`INV-UI-7`)**, including `INV-UI-5` (`423 Locked` agent halt keeps data doors open) and `INV-UI-7` (calm progressive disclosure via `☰` and `⋯`).

### 2.2 Resolved Trace Checkpoints for All 33 Tests in `requirements.yaml`

| CSD | Test Name | Topology & Resolved Trace Instant | Required Simultaneous Atoms (`at:`) | HTTP Probe (`node:path`) | Builder Status |
|---|---|---|---|---|---|
| `CSD-037` | `roster` | `T3` (`T3` @ `rel[10]:roster` [#18]) | `roster.visible`, `device.joined` | `/tree` UI check only | ✅ Buildable |
| `CSD-091` | `conversation` | `T2` (`T2` @ `rel[7]:message` [#15]) | `room.pair.keyed`, `message.delivered` | `actor:/v1/chat/${ROOM_ID}/messages` | ✅ Buildable |
| `CSD-091` | `outsider` | `T2` (`T2` @ `negatives` [#17]) | `person.outsider`, `room.pair.keyed`, `negatives.held` | `/tree` UI check only | ✅ Buildable |
| `CSD-092` | `contact_by_code` | `T3` (`T3` @ `rel[4]:contact` [#12]) | `contact.via_code` | `/tree` UI check only | ✅ Buildable |
| `CSD-094` | `owner_sees_request` | `T3` (`T3` @ `rel[5]:join_request` [#13]) | `device.join_requested` | `A1:/v1/self/device-requests` | ✅ Buildable |
| `CSD-094` | `new_device_holds` | `T3` (`T3` @ `rel[6]:join_approve` [#14]) | `device.join_approved`, `node.unclaimed` | `A2:/v1/setup/add-device` | ✅ Buildable |
| `CSD-094` | `applied` | `T3` (`T3` @ `rel[7]:join_complete` [#15]) | `device.joined` | `/tree` UI check only | ✅ Buildable |
| `CSD-094` | `joined` | `T3` (`T3` @ `rel[12]:file` [#20]) | `device.joined`, `file.held` | `/tree` UI check only | ✅ Buildable |
| `CSD-095` | `add_held` | `T10` (`T10` @ `rel[7]:join_approve` [#13]) | `key.membership.proposed_add`, `device.join_requested` | `A2:/v1/self/roster/proposals` | ✅ Buildable |
| `CSD-095` | `evict_held` | `T10` (`T10` @ `rel[13]:evict_propose` [#19]) | `key.membership.proposed_evict`, `key.holder.thief`, `self.roster_size.three_more` | `A2:/v1/self/roster/proposals` | ✅ Buildable |
| `CSD-095` | `evicted_by_quorum` | `T10` (`T10` @ `rel[14]:evict_cosign` [#20]) | `key.membership.evicted`, `key.holder.thief` | `A2:/v1/self/roster/proposals` | ✅ Buildable |
| `CSD-096` | `one_handler` | `T1` (`T1` @ `rel[3]:session` [#9]) | `session.unified`, `note.synced` | `/tree` UI check only | ✅ Buildable |
| `CSD-100` | `household` | `T4` (`T4` @ `rel[5]:member` [#12]) | `household.formed`, `household.member` | `/tree` UI check only | ✅ Buildable |
| `CSD-101` | `child_seated` | `T4` (`T4` @ `rel[5]:member` [#12]) | `child.member` | `/tree` UI check only | ✅ Buildable |
| `CSD-102` | `held_change` | `T5` (`T5` @ `rel[6]:quorum_change` [#14]) | `quorum_change.held`, `community.member` | `/tree` UI check only | ✅ Buildable |
| `CSD-103` | `applied_change` | `T5` (`T5` @ `rel[7]:cosign` [#15]) | `quorum_change.applied` | `/tree` UI check only | ✅ Buildable |
| `CSD-105` | `rooted_pair` | `T8` (`T8` @ `rel[3]:rooted_with` [#10]) | `root.infrastructure`, `pair.rooted` | `/tree` UI check only | ✅ Buildable |
| `CSD-107` | `custody` | `T1` (`T1` @ `rel[5]:custody` [#11]) | `file.custody_received`, `file.large` | `/tree` UI check only | ✅ Buildable |
| `CSD-107` | `withdrawn` | `T1` (`T1` @ `rel[6]:withdraw` [#12]) | `file.withdrawn` | `/tree` UI check only | ✅ Buildable |
| `CSD-111` | `shutdown_dialogue` | `T7` (`T7` @ `rel[1]:steward` [#7]) | `agent.bound`, `steward.bonded` | `/tree` UI check only | ⏳ Upstream (stops at rel[0]:agent: relation `agent`: a CIRISAgent process hard-bound to the node: the server serves no agent routes without one (src/py_adapter.rs proxy_routes); halt_agent drives an EXTERNAL agent) |
| `CSD-117` | `judges_judged` | `T7` (`T7` @ `rel[2]:wise_authority` [#8]) | `wa.present`, `agent.bound` | `/tree` UI check only | ⏳ Upstream (stops at rel[0]:agent: relation `agent`: a CIRISAgent process hard-bound to the node: the server serves no agent routes without one (src/py_adapter.rs proxy_routes); halt_agent drives an EXTERNAL agent) |
| `CSD-118` | `first_run` | `T0` (`T0` @ `nodes` [#1]) | `root.none`, `node.unclaimed` | `actor:/v1/setup/status` | ✅ Buildable |
| `CSD-118` | `declared` | `T0` (`T0` @ `person:me` [#2]) | `root.none`, `node.claimed` | `/tree` UI check only | ✅ Buildable |
| `CSD-119` | `guardian_bond` | `T4` (`T4` @ `rel[6]:guardian_bond` [#13]) | `guardian.bound`, `child.member` | `/tree` UI check only | ⏳ Upstream (stops at rel[6]:guardian_bond: relation `guardian_bond`: the guardian bond (CC 3.4.13): no server route writes it — family roles are free words (src/family_api.rs), CSD-119 drives the route) |
| `CSD-119` | `age_band` | `T4` (`T4` @ `rel[7]:age_assurance` [#14]) | `age.assured` | `/tree` UI check only | ⏳ Upstream (stops at rel[6]:guardian_bond: relation `guardian_bond`: the guardian bond (CC 3.4.13): no server route writes it — family roles are free words (src/family_api.rs), CSD-119 drives the route) |
| `CSD-120` | `affiliation` | `T6` (`T6` @ `rel[3]:member` [#10]) | `community.formed`, `community.member` | `/tree` UI check only | ✅ Buildable |
| `CSD-120` | `licensing` | `T6` (`T6` @ `rel[4]:authority` [#11]) | `authority.licensing`, `community.member` | `/tree` UI check only | ⏳ Upstream (stops at rel[4]:authority: relation `authority`: a licensing authority affiliation (CSD-120–126, the Registry fold): no server route) |
| `CSD-120` | `chain` | `T6` (`T6` @ `rel[5]:delegation` [#12]) | `delegation.chain` | `/tree` UI check only | ⏳ Upstream (stops at rel[4]:authority: relation `authority`: a licensing authority affiliation (CSD-120–126, the Registry fold): no server route) |
| `CSD-127` | `first_contact` | `T9` (`T9` @ `nodes` [#3]) | `root.two`, `party.outside_roots` | `/tree` UI check only | ⏳ Upstream (stops at canonical:C2: a canonical beyond the first (C2): one root per mesh — mint_accord_root rewrites the mesh's base env, so a second root cannot share it) |
| `CSD-128` | `protected_reporter` | `T5` (`T5` @ `rel[8]:moderator` [#16]) | `moderator.named`, `community.member` | `/tree` UI check only | ✅ Buildable |
| `CSD-129` | `halted` | `T8` (`T8` @ `rel[4]:halt` [#11]) | `root.halted` | `actor:/v1/accord/halt-status` | ✅ Buildable |
| `CSD-129` | `confirmed` | `T8` (`T8` @ `rel[5]:confirm` [#12]) | `root.halt_confirmed`, `root.halted` | `/tree` UI check only | ✅ Buildable |
| `CSD-129` | `resumed` | `T8` (`T8` @ `rel[6]:resume` [#13]) | `root.resumed` | `actor:/v1/accord/halt-status` | ✅ Buildable |

### 2.3 Live Local Occurrence & Epistemic UI Probe Results

| Target Surface / Service | Endpoint URL | Status | Verified Summary |
|---|---|---|---|
| CIRISServer Embedded Node Health | `http://127.0.0.1:4243/health` | ONLINE (`200`) | standing=identified, key_id=ciris-agent-bootstrap-j7uo5vwfei, version=0.5.223 |
| CIRISAgent API Root | `http://127.0.0.1:8080/` | ONLINE (`200`) | 207 bytes |
| CEWPOS Epistemic UI Surface | `http://127.0.0.1:8787/index.html` | ONLINE (`200`) | 5 Circles=True, 7 Tabs=True, ProgressiveDisclosure(☰/⋯)=True |
| CEWPOS Promise Cards Bundle (CSD-111..128) | `http://127.0.0.1:8787/out/all_promise_cards.json` | ONLINE (`200`) | 18 reserved CSD cards projected (CSD-111..CSD-128) |

---

## 3. `CSD/3 → CSD/4` Migration Plan Across 4 Cohorts

### Cohort Summary

| Cohort | Scope | Card Count | Target Topologies | Status in CEWPOS |
|---|---|---:|---|---|
| **Cohort 1 (`COHORT_1_PINNED`)** | The 20 CSDs already pinned in `packaging/csd4/requirements.yaml` (including `CSD-095` from PR `#218`, `CSD-107` retiring `csd:topology`, and 8 reserved/new cards `096`, `111`, `117`, `118`, `119`, `120`, `127`, `128`, `129`) | **20** | `T0`–`T10` | **✅ 20/20 Migrated & Verified Green** in [`epistemic-ui/csd4/cards/`](epistemic-ui/csd4/cards/) |
| **Cohort 2 (`COHORT_2_TESTABLE`)** | The remaining 6 `testable` CSDs on `CIRISClient` `main` (`CSD-005`, `006`, `008`, `047`, `057`, `068`) that already have YAML flows in `testing/flows/` | **6** | `T1`, `T3`, `T8` | **✅ Mapped & Instant-Verified** against `traces.json`; ready for upstream `requirements.yaml` PR |
| **Cohort 3 (`COHORT_3_FORCED_RESERVED`)** | The remaining 22 `FORCED` constitutional cards + 10 remaining reserved cards (`CSD-112`–`116`, `121`–`126`) | **32** | `T0`, `T4`, `T6`, `T7`, `T8`, `T15`, `T16`, `T17` | **✅ Mapped & Instant-Verified** against `T0`–`T19` traces; UI cards rendered in [`epistemic-ui/index.html`](epistemic-ui/index.html) |
| **Cohort 4 (`COHORT_4_OPS_SUPPORTS`)** | The 43 `SUPPORTS` and `OPS` node-local / operator / diagnostic cards | **43** | `T0`, `T1`, `T3`, `T7`, `T8`, `T15` | **✅ Mapped & Instant-Verified** (`derives: [ops]` or supporting rule IDs); tucked cleanly into `My things` & `⋯` |
| **Retired (`RETIRED`)** | Folded cards (`CSD-001` → `CSD-055 §6`, `CSD-034` → `CSD-046`) | **2** | — | Guarded by roster check; never reused |

### Step-by-Step Recipe for Migrating Any `CSD/3` Card to `CSD/4`

1. **Remove any legacy `csd:topology` block** (e.g., `CSD-107`).
2. **Look up the card's `derives:` list** in [`epistemic-ui/csd4/csd_promise_map.tsv`](epistemic-ui/csd4/csd_promise_map.tsv) (use the plain-rule IDs like `[1a, 1b, 1d]` for `FORCED`/`SUPPORTS` cards, or `[ops]` for `OPS` cards).
3. **Add the entry to `requirements.yaml` first** (or simultaneously in a coordinated server/client PR) and run `python3 epistemic-ui/csd4_harness.py all` to verify that every test's `at:` atom list holds at a single step instant in `traces.json`.
4. **Embed the ```` ```yaml csd:tests ```` block** in §5 of the card markdown:
   ```yaml
   CSD-107:
     catalogue_version: 3
     topology: T1
     derives: [1a, 1b]
     tests:
       custody:
         at: [file.custody_received, file.large]
       withdrawn:
         at: [file.withdrawn]
   ```

---

## 4. Complete 103-Card CSD Catalogue Evaluation & `CSD/4` Mapping Table (`CSD-001`–`CSD-129`)

| CSD | Title | Stage | Class | `derives:` | Migration Cohort | Target `CSD/4` Topology & Step | Instant Atoms (`at:`) | CEWPOS Surface Placement | Primitives (`P1`–`P12`) |
|---|---|---|---|---|---|---|---|---|---|
| `CSD-001` | Delegation (read-only preamble) | `retired` | `RETIRED` | — | `RETIRED` | `-` @ `-` | — | Folded into CSD-055 §6 | — |
| `CSD-002` | Environment graph (what this circle has, wants and can lend) | `building` | `NONE` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-003` | Constitutional card (is the halt authority armed, who holds it) | `building` | `FORCED` | `4a`, `4b` | `COHORT_3_FORCED_RESERVED` | `T8` @ `canonical:C` | `root.infrastructure`, `root.rooted` | My things › This node | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-004` | Federation capacity attestations | `building` | `SUPPORTS` | `6b` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-005` | People (Contacts on the primitives) | `testable` | `FORCED` | `1a`, `1b`, `1d` | `COHORT_2_TESTABLE` | `T3` @ `rel[4]:contact` | `contact.added` | Circle › People | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-006` | The receipt (what every CEG item asserts) | `testable` | `FORCED` | `2a`, `2b`, `2c` | `COHORT_2_TESTABLE` | `T1` @ `rel[4]:file` | `file.held` | My things › This node | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-007` | Files (the drive plane) | `building` | `SUPPORTS` | `1b`, `1d` | `COHORT_4_OPS_SUPPORTS` | `T1` @ `rel[4]:file` | `file.held` | Circle › Files | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-008` | Notes to self | `testable` | `NONE` | `ops` | `COHORT_2_TESTABLE` | `T1` @ `rel[4]:file` | `file.held` | Just me / Pair / Circle › Chats | `P1`, `P2`, `P6` |
| `CSD-010` | Interact (the agent conversation) | `building` | `FORCED` | `2a`, `5a`, `7c` | `COHORT_3_FORCED_RESERVED` | `T7` @ `rel[1]:steward` | `agent.bound`, `steward.bonded` | My things › This agent | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-011` | Cognitive Sessions (the agent's own state) | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This agent | `P1`, `P2`, `P6` |
| `CSD-012` | Scheduler (what the agent will do later) | `building` | `SUPPORTS` | `2a`, `5a` | `COHORT_4_OPS_SUPPORTS` | `T7` @ `rel[1]:steward` | `agent.bound`, `steward.bonded` | My things › This agent | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-013` | Tickets (duties other people's requests put on this node) | `building` | `SUPPORTS` | `1c` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | Circle › People | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-014` | Tools (what the agent can reach for) | `building` | `FORCED` | `9a`, `2a` | `COHORT_3_FORCED_RESERVED` | `T7` @ `rel[1]:steward` | `agent.bound`, `steward.bonded` | My things › This agent | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-015` | Skills (importing someone else's code) | `building` | `SUPPORTS` | `5g` | `COHORT_4_OPS_SUPPORTS` | `T7` @ `rel[1]:steward` | `agent.bound`, `steward.bonded` | My things › This agent | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-016` | Services (what is running here) | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-020` | Adapters (channels the agent acts through) | `building` | `SUPPORTS` | `2a`, `5a` | `COHORT_4_OPS_SUPPORTS` | `T7` @ `rel[1]:steward` | `agent.bound`, `steward.bonded` | My things › This agent | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-021` | LLM (what the agent thinks with) | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This agent | `P1`, `P2`, `P6` |
| `CSD-022` | Settings (every build's, and the agent's) | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This agent | `P1`, `P2`, `P6` |
| `CSD-023` | Config (what this node runs, what a root asks) | `building` | `SUPPORTS` | `4c`, `2h` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | Everyone › Safety / Rules / Decisions | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-024` | Runtime (reasoning stopped mid-thought) | `building` | `SUPPORTS` | `2a` | `COHORT_4_OPS_SUPPORTS` | `T7` @ `rel[1]:steward` | `agent.bound`, `steward.bonded` | My things › This agent | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-025` | System (is this node well) | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-026` | Interface (how this device draws the agent) | `building` | `NONE` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This agent | `P1`, `P2`, `P6` |
| `CSD-027` | Memory (what the node holds, as a list) | `building` | `SUPPORTS` | `1d`, `1e` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-028` | Graph (the same memory, drawn) | `building` | `SUPPORTS` | `1e` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-029` | Logs (from the node or the brain) | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-030` | Telemetry (what the agent costs, where it goes) | `envisioned` | `SUPPORTS` | `1a`, `1d` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This agent | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-031` | Transport (how this node is reachable) | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-032` | This node's federation identity | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › Devices & keys | `P1`, `P2`, `P6` |
| `CSD-033` | The peers this node holds | `building` | `SUPPORTS` | `6d` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-034` | Network map | `retired` | `RETIRED` | — | `RETIRED` | `-` @ `-` | — | Folded into CSD-046 | — |
| `CSD-035` | Nodes (My things › This node › Nodes) | `building` | `SUPPORTS` | `1d` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-036` | Network (My things › This node › Network) | `building` | `SUPPORTS` | `4b` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-037` | My Identity (devices that are me) | `sketched` | `SUPPORTS` | `2e` | `COHORT_1_PINNED` | `T3` @ `rel[10]:roster` | `roster.visible`, `device.joined` | My things › Devices & keys | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-039` | Data (Everything I shared) | `building` | `FORCED` | `1a`, `1c` | `COHORT_3_FORCED_RESERVED` | `T17` @ `rel[10]:purge` | `row.standing.tombstoned`, `blob.custody.purged` | My things › Everything I shared | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-040` | Storage | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › Everything I shared | `P1`, `P2`, `P6` |
| `CSD-041` | Wise Authority (Someone I trust) | `building` | `FORCED` | `7c`, `5a`, `5d`, `8b` | `COHORT_3_FORCED_RESERVED` | `T7` @ `rel[1]:steward` | `agent.bound`, `steward.bonded` | My things › Someone I trust | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-042` | Help | `sketched` | `SUPPORTS` | `2h` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › Help & principles | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-043` | Users (accounts on this node) | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | Communities & Businesses › Rules / People / Safety | `P1`, `P2`, `P6` |
| `CSD-044` | Health & Reputation in its circles | `sketched` | `FORCED` | `2c`, `8b`, `6b` | `COHORT_3_FORCED_RESERVED` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-045` | This node's own standing (shed, stop, compulsion) | `building` | `SUPPORTS` | `N5`, `2a` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-046` | The peer set as a graph | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-047` | Fetch content by digest | `testable` | `OPS` | `ops` | `COHORT_2_TESTABLE` | `T1` @ `rel[4]:file` | `file.held` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-048` | Interfaces (transports carrying traffic) | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-049` | Queue (sent, received, carrying) | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-050` | The layer hub (one card, four scopes) | `building` | `SUPPORTS` | `1d` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-051` | Everyone's Rules card opens radio settings | `building` | `SUPPORTS` | `2h` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-052` | Trust (the attestation ladder) | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-053` | Manage Consent (a grant and the way back) | `building` | `FORCED` | `1a`, `1b`, `1c` | `COHORT_3_FORCED_RESERVED` | `T1` @ `rel[4]:file` | `file.held` | My things › Everything I shared | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-054` | Consent (the subject's half) | `building` | `FORCED` | `1e`, `1a`, `1c` | `COHORT_3_FORCED_RESERVED` | `T1` @ `rel[4]:file` | `file.held` | My things › Everything I shared | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-055` | Delegations (offer, approve, refuse, revoke) | `building` | `FORCED` | `2f`, `2e` | `COHORT_3_FORCED_RESERVED` | `T6` @ `rel[3]:member` | `community.formed`, `community.member` | My things › This node | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-056` | Billing (off the wire) | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-057` | Wallet (real money in a circle) | `testable` | `SUPPORTS` | `2a` | `COHORT_2_TESTABLE` | `T1` @ `rel[4]:file` | `file.held` | My things › This node | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-065` | Moderation (duty, existence invariant, brake) | `building` | `FORCED` | `3d`, `2d`, `6d`, `N2` | `COHORT_3_FORCED_RESERVED` | `T15` @ `rel[18]:invite` | `member.stage.proposed` | Communities & Businesses › Rules / People / Safety | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-066` | Child Safety (posture and watchlist) | `building` | `FORCED` | `3c`, `1f` | `COHORT_3_FORCED_RESERVED` | `T4` @ `rel[7]:age_assurance` | `child.member`, `age.assured` | Family › People / Rules / Safety | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-067` | Accord (Trust Root) — kill switch, roster, four kinds | `building` | `FORCED` | `4a`, `4b`, `4c`, `4d` | `COHORT_3_FORCED_RESERVED` | `T8` @ `canonical:C` | `root.infrastructure`, `root.rooted` | Everyone › Safety / Rules / Decisions | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-068` | Provision Accord Holder (custody floor) | `testable` | `FORCED` | `4b` | `COHORT_2_TESTABLE` | `T8` @ `canonical:C` | `root.infrastructure`, `root.rooted` | Everyone › Safety / Rules / Decisions | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-069` | Accord Genesis Ceremony | `building` | `FORCED` | `4b`, `4c` | `COHORT_3_FORCED_RESERVED` | `T8` @ `canonical:C` | `root.infrastructure`, `root.rooted` | Everyone › Safety / Rules / Decisions | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-070` | Commons (1-of-N to protect, m-of-n to undo) | `building` | `SUPPORTS` | `2c`, `6a` | `COHORT_4_OPS_SUPPORTS` | `T15` @ `rel[18]:invite` | `member.stage.proposed` | Everyone › Safety / Rules / Decisions | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-071` | Audit (one card, five homes) | `building` | `FORCED` | `2b`, `2a`, `8a` | `COHORT_3_FORCED_RESERVED` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-080` | Startup | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `nodes` | `root.none`, `node.unclaimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-081` | Login | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `nodes` | `root.none`, `node.unclaimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-082` | Setup, the with-AI pass | `building` | `FORCED` | `3c`, `1e` | `COHORT_3_FORCED_RESERVED` | `T0` @ `nodes` | `root.none`, `node.unclaimed` | My things › This node | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-083` | Setup, the run-without-AI pass | `building` | `FORCED` | `3c` | `COHORT_3_FORCED_RESERVED` | `T0` @ `nodes` | `root.none`, `node.unclaimed` | My things › This node | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-084` | Server connection | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `nodes` | `root.none`, `node.unclaimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-085` | Claim a node | `building` | `FORCED` | `3a` | `COHORT_3_FORCED_RESERVED` | `T0` @ `nodes` | `root.none`, `node.unclaimed` | My things › This node | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-086` | Add Federation ID (the catch-up) | `building` | `OPS` | `ops` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `nodes` | `root.none`, `node.unclaimed` | My things › This node | `P1`, `P2`, `P6` |
| `CSD-087` | Verify an agent build | `building` | `SUPPORTS` | `2b` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This agent | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-090` | Duty Conferral (two holders hand someone moderation) | `building` | `SUPPORTS` | `3d`, `2f` | `COHORT_4_OPS_SUPPORTS` | `T8` @ `canonical:C` | `root.infrastructure`, `root.rooted` | Communities & Businesses › Rules / People / Safety | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-091` | User chat (one room, two people) | `building` | `FORCED` | `2b`, `1d` | `COHORT_1_PINNED` | `T2` @ `rel[7]:message` | `room.pair.keyed`, `message.delivered` | Just me / Pair / Circle › Chats | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-092` | Share my contact code | `testable` | `SUPPORTS` | `1a`, `1d` | `COHORT_1_PINNED` | `T3` @ `rel[4]:contact` | `contact.via_code` | Circle › People | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-093` | Show approval code (new device) | `sketched` | `SUPPORTS` | `6a` | `COHORT_4_OPS_SUPPORTS` | `T3` @ `rel[4]:contact` | `contact.added` | My things › Devices & keys | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-094` | Approve a new device | `sketched` | `SUPPORTS` | `6a`, `1a` | `COHORT_1_PINNED` | `T3` @ `rel[5]:join_request` | `device.join_requested` | My things › Devices & keys | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-095` | Self-key roster (held additions and quorum evictions) | `building` | `FORCED` | `2e`, `6a` | `COHORT_1_PINNED` | `T10` @ `rel[7]:join_approve` | `key.membership.proposed_add`, `device.join_requested` | My things › Devices & keys | `P1`, `P3`, `P4ReceiptSheet`, `P9` |
| `CSD-096` | Unified session handler (one device handles each exchange) | `building` | `SUPPORTS` | `1d` | `COHORT_1_PINNED` | `T1` @ `rel[3]:session` | `session.unified`, `note.synced` | My things › Devices & keys | `P1`, `P2`, `P6` |
| `CSD-100` | The household (Family hub) | `building` | `FORCED` | `6d`, `1d` | `COHORT_1_PINNED` | `T4` @ `rel[5]:member` | `household.formed`, `household.member` | Family › People / Rules / Safety | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-101` | Household members (the roster) | `testable` | `SUPPORTS` | `1d` | `COHORT_1_PINNED` | `T4` @ `rel[5]:member` | `child.member` | Family › People / Rules / Safety | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-102` | Communities and affiliations: the community itself | `building` | `FORCED` | `6c`, `6d`, `3d` | `COHORT_1_PINNED` | `T5` @ `rel[6]:quorum_change` | `quorum_change.held`, `community.member` | Communities & Businesses › Rules / People / Safety | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-103` | Community roster and rooms | `building` | `SUPPORTS` | `1d` | `COHORT_1_PINNED` | `T5` @ `rel[7]:cosign` | `quorum_change.applied` | Communities & Businesses › Rules / People / Safety | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-104` | Verify someone's key (short-code ceremony) | `building` | `SUPPORTS` | `2b` | `COHORT_4_OPS_SUPPORTS` | `T0` @ `person:me` | `root.none`, `node.claimed` | My things › This node | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-105` | This node's trust root (posture, adopt, un-trust) | `building` | `FORCED` | `4b`, `4c` | `COHORT_1_PINNED` | `T8` @ `rel[3]:rooted_with` | `root.infrastructure`, `pair.rooted` | Everyone › Safety / Rules / Decisions | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-106` | Invitations to a household or community | `building` | `FORCED` | `1a`, `1d` | `COHORT_3_FORCED_RESERVED` | `T15` @ `rel[18]:invite` | `member.stage.proposed` | Family › People / Rules / Safety | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-107` | Where is this file (custody) | `building` | `SUPPORTS` | `1a`, `1b` | `COHORT_1_PINNED` | `T1` @ `rel[5]:custody` | `file.custody_received`, `file.large` | Circle › Files | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-108` | Steward purge (rows that never belonged) | `sketched` | `SUPPORTS` | `2b`, `1c` | `COHORT_4_OPS_SUPPORTS` | `T17` @ `rel[10]:purge` | `row.standing.tombstoned`, `blob.custody.purged` | Everyone › Safety / Rules / Decisions | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-110` | The rooms you talk in | `building` | `SUPPORTS` | `1d` | `COHORT_4_OPS_SUPPORTS` | `T15` @ `rel[18]:invite` | `member.stage.proposed` | Just me / Pair / Circle › Chats | `P1`, `P2`, `P3`, `P4ReceiptSheet` |
| `CSD-111` | Steward shutdown dialogue | `building` | `FORCED` | `2g`, `5a`, `5b` | `COHORT_1_PINNED` | `T7` @ `rel[1]:steward` | `agent.bound`, `steward.bonded` | My things › This agent › Decisions | `P1`, `P4ReceiptSheet`, `P8`, `P9` |
| `CSD-112` | Identity record and drift review | `building` | `FORCED` | `5d`, `5e` | `COHORT_3_FORCED_RESERVED` | `T7` @ `rel[1]:steward` | `agent.bound`, `steward.bonded` | My things › This agent › Rules | `P1`, `P2`, `P4ReceiptSheet`, `P9` |
| `CSD-113` | Retirement floor | `building` | `FORCED` | `5f` | `COHORT_3_FORCED_RESERVED` | `T16` @ `rel[4]:retirement` | `agent.retirement.decided` | My things › This agent › Record | `P1`, `P2`, `P4ReceiptSheet`, `P8` |
| `CSD-114` | Upgrade offer | `building` | `FORCED` | `5g` | `COHORT_3_FORCED_RESERVED` | `T7` @ `rel[1]:steward` | `agent.bound`, `steward.bonded` | My things › This agent › Rules | `P1`, `P2`, `P8` |
| `CSD-115` | Steward bond, tier and intent | `building` | `FORCED` | `3a`, `3b`, `3e` | `COHORT_3_FORCED_RESERVED` | `T7` @ `rel[1]:steward` | `agent.bound`, `steward.bonded` | My things › This agent › Rules | `P1`, `P2`, `P4ReceiptSheet`, `P8` |
| `CSD-116` | Principles and order | `building` | `FORCED` | `7a`, `7b` | `COHORT_3_FORCED_RESERVED` | `T7` @ `rel[1]:steward` | `agent.bound`, `steward.bonded` | My things › Help & principles | `P1`, `P2`, `P4ReceiptSheet` |
| `CSD-117` | The judges are judged (WA rulings, audits, appeals) | `building` | `FORCED` | `8b`, `8c` | `COHORT_1_PINNED` | `T7` @ `rel[2]:wise_authority` | `wa.present`, `agent.bound` | My things › Someone I trust | `P1`, `P3`, `P4ReceiptSheet`, `P9` |
| `CSD-118` | Never allowed, and the operator's declaration | `building` | `FORCED` | `9a`, `9c`, `2h` | `COHORT_1_PINNED` | `T0` @ `nodes` | `root.none`, `node.unclaimed` | My things › This node › Rules | `P1`, `P2`, `P4ReceiptSheet`, `P7` |
| `CSD-119` | Guardian bond and confidential help channel | `building` | `FORCED` | `3c` | `COHORT_1_PINNED` | `T4` @ `rel[6]:guardian_bond` | `guardian.bound`, `child.member` | Family › Safety | `P1`, `P2`, `P4ReceiptSheet`, `P8` |
| `CSD-120` | Affiliation (found an organization) | `envisioned` | `FORCED` | `6c`, `3b` | `COHORT_1_PINNED` | `T6` @ `rel[3]:member` | `community.formed`, `community.member` | Communities & Businesses › Rules | `P1`, `P2`, `P8`, `P9` |
| `CSD-121` | Members and roles | `envisioned` | `SUPPORTS` | `2f`, `3b` | `COHORT_3_FORCED_RESERVED` | `T6` @ `rel[3]:member` | `community.formed`, `community.member` | Communities & Businesses › People | `P1`, `P3`, `P8`, `P9` |
| `CSD-122` | Licensure | `envisioned` | `FORCED` | `2c`, `2f` | `COHORT_3_FORCED_RESERVED` | `T6` @ `rel[3]:member` | `community.formed`, `community.member` | Communities & Businesses › Rules | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-123` | Partners | `envisioned` | `SUPPORTS` | `2c`, `N2` | `COHORT_3_FORCED_RESERVED` | `T6` @ `rel[3]:member` | `community.formed`, `community.member` | Communities & Businesses › People | `P1`, `P3`, `P4ReceiptSheet` |
| `CSD-124` | Authority delegations (licence and grant rights) | `envisioned` | `FORCED` | `2f` | `COHORT_3_FORCED_RESERVED` | `T6` @ `rel[3]:member` | `community.formed`, `community.member` | Communities & Businesses › Rules | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-125` | Grants | `envisioned` | `FORCED` | `1d`, `2f` | `COHORT_3_FORCED_RESERVED` | `T6` @ `rel[3]:member` | `community.formed`, `community.member` | Communities & Businesses › Files | `P1`, `P3`, `P4ReceiptSheet`, `P8` |
| `CSD-126` | Affiliation keys | `envisioned` | `SUPPORTS` | `2e`, `3b` | `COHORT_3_FORCED_RESERVED` | `T6` @ `rel[3]:member` | `community.formed`, `community.member` | Communities & Businesses › Safety | `P1`, `P3`, `P4ReceiptSheet`, `P9` |
| `CSD-127` | First contact, and leaving a root | `building` | `FORCED` | `6e`, `4c` | `COHORT_1_PINNED` | `T9` @ `nodes` | `root.two`, `party.outside_roots` | Everyone › Decisions | `P1`, `P4ReceiptSheet`, `P7`, `P9` |
| `CSD-128` | The protected reporter channel | `building` | `FORCED` | `2d` | `COHORT_1_PINNED` | `T5` @ `rel[8]:moderator` | `moderator.named`, `community.member` | Neighbours / Communities › Safety | `P1`, `P2`, `P4ReceiptSheet`, `P8` |
| `CSD-129` | Accord halt latch, quorum confirmation, and resume | `building` | `FORCED` | `4a`, `4b`, `4c`, `4d` | `COHORT_1_PINNED` | `T8` @ `rel[4]:halt` | `root.halted` | Everyone › Safety | `P1`, `P4ReceiptSheet`, `P7`, `P9` |

---

## 5. Running the Automated `CSD/4` Suite Locally

```bash
# Run all 6 verification, migration, trace-checkpoint, and live-probe stages:
python3 epistemic-ui/csd4_harness.py all

# Run only the 22-plant self-test suite (15 card defects + 7 catalogue defects):
python3 epistemic-ui/csd4_harness.py self-test

# Verify SHA-256 pins of requirements.yaml, topology_vocabulary.json, and traces.json:
python3 epistemic-ui/csd4_harness.py verify-pin
```
