# view-forms-spike

Proves the **typed view-form pipeline** end-to-end: a real CEG `scores`
attestation becomes an actual rendered PNG, through a pure, total view function
and a renderer-neutral typed Scene IR, with Bevy 0.19 as the headless renderer.

```text
  CEG `scores` attestation
       │   (a real Form, admitted by the closed-operator gate
       │    attestation_calculus_spike::forms::read_form)
       ▼
  view_form(&Form) -> Scene          PURE · TOTAL · no IO / no clock / no RNG / no GPU
       ▼
  Scene  ── our renderer-neutral typed 3D IR (NO Bevy in this type) ──
       ▼
  Bevy 0.19 entities  ─▶  off-screen Camera { RenderTarget::Image(handle) }
       ▼                   (no window — WinitPlugin disabled, ScheduleRunnerPlugin loop)
  Readback::texture(handle) -> ReadbackComplete(Vec<u8>)
       ▼
  out/attestation-render.png         a real, on-disk, GPU-rendered PNG (macOS Metal)
```

## What it proves (status)

| Claim | Status | Evidence |
|---|---|---|
| Pure scene IR + `view_form` compile & test with **no GPU** | ✅ | `cargo test --no-default-features` → 11 passed |
| A **real** `scores` Form drives the view (not a mock) | ✅ | `forms::read_form` through the closed-operator gate |
| `view_form` is **pure, total, deterministic** | ✅ | tests: clamps/NaN-safe, same Form ⇒ identical `Scene` |
| Typed Scene IR ⇒ Bevy entities ⇒ **headless PNG** | ✅ | `out/attestation-render.png`, 512×512 RGBA, ~164 KB |
| The view logic is **browser-portable** (wasm32) | ✅ | `cargo build --target wasm32-unknown-unknown --no-default-features --lib` → ok |
| Full Bevy **wasm app** render-to-PNG in a browser | ⛔ not attempted here | feasibility-proven separately; see *WASM / browser note* |

## The render

The running example is a federation-tier quality attestation
(`dimension = "evaluation:quality"`, `score = 0.85`, `confidence = 0.92`). The
PNG shows:

- a **large green sphere** at the origin — the subject. Its color lerps red→green
  by `score` (0.85 ⇒ `#3FAC49`, green = "scored high"); its scale grows with
  `confidence` (0.92 ⇒ scale 1.55).
- a small **red cube** (`score = -1`) and **green cube** (`score = +1`) marking
  the ends of the score axis, so the subject's lean is legible.
- a grey ground plane on a **Bone `#FAF9F5`** background.

## How to run

All cargo invocations share the feasibility Bevy build cache:

```bash
export CARGO_TARGET_DIR=/tmp/bevy-spike-target

# 1) Pure pipeline — scene IR + view_form. No Bevy, no GPU. Fast.
cargo test --no-default-features

# 2) Headless render: real scores Form -> view_form -> Scene IR -> Bevy -> PNG.
cargo run --bin render --features render
#    -> writes out/attestation-render.png and prints the Scene IR + chosen color.

# 3) (stretch) Prove the view logic is browser-portable: compile it to wasm32.
cargo build --target wasm32-unknown-unknown --no-default-features --lib
```

Requires `rustc >= 1.95` (Bevy 0.19 MSRV; this machine is 1.96.0) and, for the
wasm step, `rustup target add wasm32-unknown-unknown`.

## The Scene IR — our typed 3D dialect

`src/scene.rs` is **pure data with no Bevy dependency** — the contract between the
pure view layer and *any* renderer. The Bevy headless renderer is one consumer;
the same `Scene` could drive a web `<canvas>`, a glTF exporter, or an ASCII
dumper.

```rust
pub enum MeshKind { Sphere, Cube, Plane }

pub struct SceneNode {
    pub pos: [f32; 3],
    pub scale: f32,
    pub mesh: MeshKind,
    pub color: [f32; 4],          // linear sRGB rgba
    pub label: Option<String>,    // e.g. the attestation `dimension`
}

pub struct Camera { pub eye: [f32; 3], pub look_at: [f32; 3] }

pub struct Scene {
    pub nodes: Vec<SceneNode>,
    pub camera: Camera,
    pub background: [f32; 3],
}
```

Keeping the IR renderer-neutral is what lets `view_form` cross-compile to the
browser without dragging in a renderer. **BSN** (Bevy Scene Notation, the `bsn!`
macro) is the Bevy-native dialect option; this spike deliberately uses our own
typed IR + plain ECS spawn (`Mesh3d` / `MeshMaterial3d` / `Camera3d` tuples)
because it is renderer-neutral and does not couple the pure view function to a
specific engine — the robust path for the brand-new macro.

## Files

- `src/scene.rs` — the typed Scene IR (pure, no Bevy). Tests included.
- `src/view_form.rs` — `view_form(&Form) -> Scene`, the pure/total mapping plus
  the `score_to_color` / `confidence_to_scale` functions. Tests included.
- `src/lib.rs` — re-exports `scene`, `view_form`, and the real `forms`.
- `src/bin/render.rs` — Bevy 0.19 headless renderer (behind the `render`
  feature). Builds a real `scores` Form, runs `view_form`, spawns entities from
  the Scene IR, renders off-screen, reads back, writes the PNG.
- `out/attestation-render.png` — the rendered output.

## WASM / browser note

The **pure half** (`scene` + `view_form`, including the real
`attestation-calculus-spike` + `ciris-crypto` it reads `Form` from) compiles
cleanly to `wasm32-unknown-unknown`, so the CEG→view→Scene-IR logic is genuinely
browser-portable. Two getrandom shims are needed because `ciris-crypto` pulls two
getrandom versions that refuse to build for wasm without a browser backend:

- `getrandom 0.2` (via `rand_core 0.6`) → the `js` feature, and
- `getrandom 0.4` (via `ml-dsa`'s `rand_core 0.10`) → the `wasm_js` feature **plus**
  `RUSTFLAGS='--cfg getrandom_backend="wasm_js"'` (set in `.cargo/config.toml`,
  scoped to the wasm target so the native build is untouched).

`view_form` itself never draws randomness; these only satisfy the compile of the
unused signing code paths.

The **full Bevy browser render** (the `app.webgpu.wasm` / `app.webgl2.wasm`
artifacts via `wasm-bindgen --target web`, then off-screen render + async
readback in the browser) is **not** built here. It was proven feasible
separately: a Bevy 0.19 `webgl2` app compiles to wasm on this machine, and the
`Readback`/`ReadbackComplete` component API is wasm-compatible (browser readback
is async — no `Device::poll` blocking). Wiring that into a running browser page
is the next step beyond this spike.
```
