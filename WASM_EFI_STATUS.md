# CEWPOS Substrate & Architecture Status: WASM & EFI Native

*Status Date: November 9, 2026*  
*Related Repositories: [CIRISServer](https://github.com/CIRISAI/CIRISServer), [CIRISVerify](https://github.com/CIRISAI/CIRISVerify), [CIRISPersist](https://github.com/CIRISAI/CIRISPersist), [CIRISEdge](https://github.com/CIRISAI/CIRISEdge), [CEWPOS](https://github.com/CIRISAI/CEWPOS)*

---

## 1. Executive Summary

CEWPOS (CIRIS Epistemic Web Platform OS) projects user interfaces and epistemic choices strictly from a closed algebra of **12 UX Primitives (`P1`–`P12`)** over an immutable, content-addressed **Attestation Calculus (`1+4` CEG operators)** rendered directly in a **full-screen Bevy / `CIRISHolon` 3D scene**.

To enable CEWPOS to run everywhere from sovereign browser sandboxes to bare-metal hardware without a general-purpose host OS underneath, the substrate architecture is progressing across two formal profiles:
1. **Pure WebAssembly / WASI Node Profile (`CSD-037`)**: Sandboxed, zero-ambient-authority execution in modern browsers and edge hardware runtimes.
2. **Bare-Metal Native Profile (UEFI Stage A + WASI Node B)**: Hardware appliance boot verification sealed to TPM 2.0 PCRs with no general-purpose OS.

---

## 2. Active Upstream Issues & Substrate Roadmap

### 2.1 [CIRISServer#855](https://github.com/CIRISAI/CIRISServer/issues/855): Pure WASM / WASI Node Build Profile
- **Status:** Open (Design / Specification converged with CC 4.5.13 ChoiceItem CDDL schema).
- **Core Requirements:**
  - **Abstract Transport (`ciris-edge`)**: Pluggable Reticulum transport replacing raw POSIX sockets with WebSocket relays and WebRTC DataChannels in browser WASM, and WASI component sockets in edge runtimes.
  - **Abstract Storage Engine (`ciris-persist`)**: Swappable SQLite/KV VFS backend supporting SQLite Wasm over Origin Private File System (`OPFS`) or IndexedDB.
  - **External `HardwareSigner` Trait**: Delegated signing across WebUSB/WebHID/Web NFC or TPM 2.0 chips, keeping hybrid `Ed25519 + ML-DSA-65` private keys out of WASM heap memory.
  - **Zero Ambient Authority**: Eliminating OS-specific daemon spawns, signal handling, and raw file permission dependencies.

### 2.2 [CIRISVerify#325](https://github.com/CIRISAI/CIRISVerify/issues/325): `ciris-verify-core` Pure/Host Split & Key Custody
- **Status:** Open (Active coordination with CEWPOS & CIRISServer#870; trial cut compiled in scratch branch).
- **Core Requirements:**
  - **Strict `no_std` + `alloc` Core**: Moving 12 host-only modules behind a default-on feature flag. The pure verification crate must be strictly `#![no_std]` + `alloc` so it can compile cleanly for both `wasm32-unknown-unknown` and `x86_64-unknown-uefi`.
  - **Pure Inputs for Clocks & RNG**: Ambient clocks (`chrono::Utc::now()`) and ambient RNGs are removed from the core verifier; `now` is passed as a parameter and signing takes an injected RNG.
  - **Hybrid Key Custody Matrix**: Documented custody boundaries across TPM 2.0, USB drives, OS keyrings, PKCS#11 tokens, WebAuthn, sealed software, and browser OPFS/IndexedDB.
  - **SQLite & Migration Floor**: Aligned with `ciris-persist` v61.2.0 adoption of `rusqlite 0.41.1` and `refinery 0.10` (requiring Rust 1.92+ for persist, while pure verify-core preserves Rust 1.89+ compatibility).
  - **Revocation via Signed Snapshot**: Pure verifier operates offline without live network by validating against signed revocation bundle inputs.

### 2.3 [CIRISServer#870](https://github.com/CIRISAI/CIRISServer/issues/870): Bare-Metal Profile for CEWPOS
- **Status:** Open (Filed November 9, 2026).
- **Architecture (A + B, explicitly ruling out native UEFI server rewrite):**
  - **Stage A (UEFI Verification Stage)**: Firmware-level boot stage (`x86_64-unknown-uefi` / `aarch64-unknown-uefi`) verifying hybrid signatures on the node image and boot manifest using the pure half of `ciris-verify-core`, extending PCRs via `EFI_TCG2_PROTOCOL`.
  - **Stage B (WASI Node on Bare Metal)**: Minimal loader/unikernel boots the `wasm32-wasip2` node runtime from #855, sealing node keys to TPM measured boot PCRs. CEWPOS Bevy spatial scene renders directly to hardware framebuffer/GPU.

---

## 3. CEWPOS Local Portability Status

| Component | Target Architecture | Status |
|---|---|---|
| **Attestation Calculus Spike** (`attestation-calculus-spike/`) | `wasm32-unknown-unknown` / Native | **100% Portable** (30/30 unit & live router integration tests pass) |
| **View-Forms Component** (`view-forms-spike/`) | `wasm32-wasip2` Zero-Import Sandbox | **100% Portable** (28/28 tests pass; zero host imports enforced) |
| **Epistemic UI & 3D Spatial Scene** (`epistemic-ui/`) | Full-Screen Bevy / Canvas (`100vw × 100vh`) | **Live** (Pure P1–P12 algebra, 101/101 CSD/4 cards, automated guided tour) |
| **Full-Stack Node Binary** (`cewpos-node`) | Native Linux (Ubuntu x86_64) | **Verified** (`ciris-server v0.5.245` crate consumption, 10/12 fresh provisioning pass) |
| **Two-Instance Node Ceremony** (`two_instance_runner.py`) | Multi-Node Mesh / Localhost | **Verified** (First-Run Wizard, 0600 PIN claim, QR pairing, Drive FSD, 3D Holon scene) |
