# CEWPOS — Deployment & Bare-Metal Requirements

CEWPOS is a native OS interface built on top of a **running CIRISServer node**. CIRISServer and
CIRISAgent are the production software; CEWPOS replaces the KMP/Compose client with a native layer
that embeds the substrate crates directly. This document covers what you need to run a node.

## Node topologies

### Full node (recommended)

CIRISServer and the CEWPOS interface co-locate on the same host. The client communicates over a
local Unix socket — no HTTP round-trip on the hot path.

```
┌──────────────────────────────────────────────────────┐
│  Host                                                │
│                                                      │
│  ┌────────────────┐    Unix socket    ┌────────────┐ │
│  │  CIRISServer   │ ◄──────────────► │   CEWPOS   │ │
│  │  v0.5.58       │                  │  (native)  │ │
│  │  + substrate   │                  │  Bevy GPU  │ │
│  │    crates      │                  │  WASM view │ │
│  └────────────────┘                  └────────────┘ │
│          │                                           │
│   ciris-persist · ciris-verify · ciris-edge          │
└──────────│───────────────────────────────────────────┘
           │ ciris-edge (Reticulum / HTTP)
           ▼
    federation peers
```

### Thin client

CEWPOS client only; connects to a remote CIRISServer over the network. Suitable for workstations
that do not need to host a corpus or participate in federation directly.

```
[CEWPOS client]  ──── TCP / ciris-edge ────►  [CIRISServer node]
```

### Headless federation node

CIRISServer only, no CEWPOS interface. Stores corpus, participates in federation, runs conformance.
This is the existing CIRISServer deployment model.

---

## Hardware requirements

### CPU

| | Minimum | Recommended |
|---|---|---|
| Architecture | x86_64 or ARM64 | ARM64 (Apple M-series) or x86_64 |
| Cores | 4 | 8+ |
| Note | PQC operations (ML-DSA-65 sign/verify, ML-KEM-768 encap/decap) are CPU-bound | More cores isolate substrate from renderer under load |

### GPU

Required for the Bevy renderer (view-forms → Scene IR → GPU pixels). Not required for headless
federation nodes.

| Backend | Platform | Minimum |
|---|---|---|
| Metal | macOS 12+ | any Apple GPU (M1 / A12 or newer) |
| Vulkan 1.1+ | Linux, Windows | NVIDIA Kepler+, AMD GCN2+, Intel Arc / Iris Xe |
| WebGPU | browser / WASM target | any browser with WebGPU enabled |

Integrated GPUs are sufficient for most view-form rendering. Discrete GPU recommended for
high-density scene IR workloads (many concurrent attestation views, media streams).

### RAM

| Deployment | Minimum | Recommended |
|---|---|---|
| Full node | 4 GB | 8 GB |
| Thin client | 2 GB | 4 GB |
| Headless federation node | 2 GB | 4 GB |

Breakdown (full node): ~1 GB substrate crates + ~1 GB ciris-persist working set + ~1 GB Bevy GPU
staging + ~512 MB WASM runtime + headroom. `ciris-edge` AV1/Opus streaming under load can spike an
additional 1–2 GB.

### Storage

Content-addressed and append-only — the corpus grows without bound. Plan accordingly.

| Tier | Minimum | Recommended |
|---|---|---|
| Development / spike | 10 GB SSD | 50 GB SSD |
| Single-user node | 50 GB SSD | 250 GB NVMe |
| Federation / community node | 250 GB NVMe | 1 TB+ NVMe |

Rule of thumb: 1–2 GB per active user per year at moderate attestation + media volume.

**Reference storage layout:**

```
/var/lib/cewpos/
  corpus/           # ciris-persist chunk-DAG + stream-STH (append-only)
  keyring/          # ciris-keyring encrypted key material
  federation/       # peer registry, trust anchors, federation state
  view-forms/       # cached WASM view-form components (content-addressed)
  renderer-cache/   # wasmtime AOT-compiled view-forms (can be rebuilt)
```

### Network

| Scenario | Minimum | Recommended |
|---|---|---|
| Attestation fabric only | 1 Mbps | 10 Mbps |
| Streaming media (ciris-edge AV1/Opus) | 10 Mbps | 100 Mbps |
| Community federation node | 25 Mbps | 1 Gbps |

**Ports (defaults, configurable):**

| Port | Protocol | Use |
|---|---|---|
| 7777/TCP | ciris-edge HTTP transport | federation peer connections |
| configurable | Reticulum RF/LoRa/serial | mesh / air-gapped deployment |

No NAT traversal is required — ciris-edge handles the transport layer. IPv4 and IPv6 both
supported. For air-gapped or austere environments, Reticulum supports LoRa, packet radio, and
serial links with no IP dependency.

---

## Software dependencies

### Host OS

| OS | Minimum version | Notes |
|---|---|---|
| Linux | kernel 5.15+ | Debian 12, Ubuntu 22.04, Fedora 38+ tested |
| macOS | 12 (Monterey) | Apple Silicon and Intel both supported |
| Windows | 11 | Vulkan or DirectX 12 required for Bevy |

### Rust toolchain

MSRV: **Rust 1.75+**. Install via `rustup`. The WASM renderer target also requires:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack            # for browser WASM builds
```

### WASM runtime

**wasmtime 14+** — embedded in the CEWPOS binary; not separately installed. View-forms are
instantiated with zero imports; the runtime enforces capability isolation at instantiation.

### Display server (full-node / thin-client only)

| Platform | Required |
|---|---|
| Linux | Wayland (preferred) or X11 |
| macOS | Metal display pipeline (included in macOS 12+) |
| Windows | DirectX 12 |

Headless nodes (no Bevy renderer) require no display server.

---

## Cryptographic requirements

### Key material

No dedicated HSM required. Key material is stored via **ciris-keyring**, which integrates with the
OS secure enclave:

| Platform | Backend |
|---|---|
| macOS | Keychain Services |
| Linux | Secret Service (GNOME Keyring / KWallet) or DPAPI fallback |
| Windows | DPAPI / Windows Credential Store |

Entropy source: OS `OsRng` (`/dev/urandom` on Linux, `SecRandomCopyBytes` on macOS). Seed
generation uses SHA-256 over `(domain | alias | base_seed)` for deterministic peer identity
derivation.

### Signature schemes (in use today)

| Purpose | Scheme |
|---|---|
| Attestation signing (local tier) | Ed25519 |
| Attestation signing (federation promotion) | Ed25519 + ML-DSA-65 (hybrid) |
| Content encryption cascade | X25519 + ML-KEM-768 (hybrid) |

Harvest-now-decrypt-later is in the threat model — the hybrid PQC scheme is not a future
aspiration, it is on the wire today.

### TPM 2.0 (optional)

Hardware-rooted attestation chain (TPM/FIDO verify) is on the BUILD-IT roadmap (see
[`object-model/gap-analysis.md`](object-model/gap-analysis.md), gap #7). Not required today —
CIRISVerify provides the software verification module; the hardware root is a planned extension via
CC 4.5.1 and the `attestation:hardware_rooted` subject kind.

---

## Structural invariants enforced at deployment

These are not configuration options — they are admission-gate invariants the fabric enforces
regardless of local node configuration.

### Named-moderator existence invariant (CC 4.5.4)

A community cannot operate at moderated capability without a live, accountable, named moderator on
record. A fresh CEWPOS node starts at `ConfigScope::Local` (node-tier) and cannot self-moderate
at community tier. This is by design.

### Four-test admission gate

Every `emit` from the CEWPOS interface (and from CIRISAgent) clears:

1. **Rules hash-pinned** — the rule set used is content-addressed; a changed rule set produces a
   different gate identity.
2. **Mechanism-not-quality** — the gate checks structure, not sentiment or meaning.
3. **Re-checkable** — any node can re-run the gate on any stored attestation.
4. **Never sole evidence** — no attestation is the only basis for a consequential action; corroboration
   is required.

### Reverse-quorum halt (fire-on-1)

A single accountable signer (a named Wise Authority) can halt the system. This is a day-one
structural invariant, not a governance future-work item.

### Structural invisibility

Self and family content never emits a `holds_bytes:sha256:*` discovery attestation. It cannot be
found by federation peers — undiscoverable, not merely access-controlled. No deployment option
weakens this.

---

## Minimum viable full-node deployment (step sketch)

1. **Install CIRISServer v0.5.58** per the [CIRISServer docs](https://cirisai.github.io/CIRISServer).
   Verify the node is running and the CEG wire (1.0-RC29) is active.
2. **Build CEWPOS** against the same workspace (substrate crates pinned to matching versions):
   ```sh
   cargo build --release --features real-persist,real-edge
   ```
3. **Initialize key material:**
   ```sh
   cewpos init --domain <your-domain> --alias <node-alias>
   ```
   This calls `register_self_federation_key` via ciris-persist and stores the keypair in the OS
   keyring. The `LocalSigner` is available immediately; the `SignedAttestation` promotion path
   requires a running CIRISServer node to be reachable.
4. **Register a named moderator** before enabling community-tier capabilities (CC 4.5.4). The
   node will refuse to open community-scoped attestation paths until this invariant is satisfied.
5. **Start the CEWPOS interface:**
   ```sh
   cewpos start --connect unix:///var/run/cirisserver/fabric.sock
   ```
   The Bevy renderer initializes, view-forms are loaded from the content-addressed cache, and the
   node enters the federation.

---

## Deployment topology reference

```
                     ┌─────────────────────────────┐
                     │  CIRIS Federation            │
                     │  (named-moderator guaranteed │
                     │   reverse-quorum halt: 1)    │
                     └──────────────┬──────────────┘
                                    │ ciris-edge (Reticulum / HTTP)
              ┌─────────────────────┼──────────────────────┐
              │                     │                      │
   ┌──────────▼──────────┐ ┌───────▼───────────┐ ┌────────▼──────────┐
   │  Full node A        │ │  Full node B       │ │  Headless node C  │
   │  CIRISServer        │ │  CIRISServer       │ │  CIRISServer only │
   │  + CEWPOS client    │ │  + CEWPOS client   │ │  (corpus + relay) │
   │  + ciris-persist    │ │  + ciris-persist   │ │  + ciris-persist  │
   │  + Bevy renderer    │ │  + Bevy renderer   │ └───────────────────┘
   └─────────┬───────────┘ └───────────────────┘
             │ optional
   ┌─────────▼───────────┐
   │  CIRISAgent         │
   │  (app store)        │
   │  emits gate-checked │
   │  attestations only  │
   └─────────────────────┘
```

CIRISAgent is optional at any node — it plugs in as one gate-checked emitter and does not require
co-location. The fabric operates without it; the agent simply adds the moral-reasoning loop as an
emitting participant subject to the same admission gate as any other source.
