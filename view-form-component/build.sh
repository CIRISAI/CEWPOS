#!/usr/bin/env bash
# Build + run the sandboxed WASM Component view-form spike, end to end.
#
# Proven toolchain (all darwin/arm64):
#   rustc 1.96.0, cargo-component 0.20.0, wasm-tools 1.252.0,
#   wit-bindgen 0.58.0 (guest), wasmtime 46.0.1 (host),
#   target wasm32-unknown-unknown  (the zero-import target — NOT wasip1/wasip2).
set -euo pipefail

ROOT="/Users/macmini/CEWPOS/view-form-component"
export CARGO_TARGET_DIR="/tmp/wasm-component-target"
WT="/tmp/wasmtools-bin/wasm-tools-1.252.0-aarch64-macos/wasm-tools"   # wasm-tools CLI
CC="/tmp/wasmtools-bin/cargo-component"                                # cargo-component CLI
TGT="$CARGO_TARGET_DIR/wasm32-unknown-unknown/release"
DIST="$ROOT/dist"; mkdir -p "$DIST" "$ROOT/out"

# === PATH A: wit-bindgen + `wasm-tools component new` (guarantees 0 imports) ==
echo ">> guest: cargo build -> core module (wasm32-unknown-unknown)"
( cd "$ROOT/guest" && cargo build --release --target wasm32-unknown-unknown )
echo ">> guest: wasm-tools component new -> COMPONENT"
"$WT" component new "$TGT/view_form_guest.wasm" -o "$DIST/view_form_component.wasm"
"$WT" validate --features all "$DIST/view_form_component.wasm"
echo ">> view-form component WIT (note: zero import lines):"
"$WT" component wit "$DIST/view_form_component.wasm"

echo ">> malicious guest (sandbox-breaker): imports a host capability"
( cd "$ROOT/malicious-guest" && cargo build --release --target wasm32-unknown-unknown )
"$WT" component new "$TGT/malicious_guest.wasm" -o "$DIST/malicious_component.wasm"

# === PATH B: cargo-component (blessed; produces the component in one step) ====
echo ">> cc-guest: cargo component build -> COMPONENT"
( cd "$ROOT/cc-guest" && "$CC" component build --release --target wasm32-unknown-unknown )
cp "$TGT/cc_guest.wasm" "$DIST/cc_guest_component.wasm"

# === HOST: wasmtime, EMPTY linker (zero host imports) ========================
echo ">> host: cargo test (typed-scene round-trip + sandbox-reject)"
( cd "$ROOT/host" && cargo test --release )
echo ">> host: cargo run -> instantiate + call across WIT boundary"
( cd "$ROOT/host" && cargo run --release -- \
    "$DIST/view_form_component.wasm" "$DIST/malicious_component.wasm" )
echo ">> host vs cargo-component-built component (Path B)"
( cd "$ROOT/host" && cargo run --release -- "$DIST/cc_guest_component.wasm" )

echo ">> OK"
