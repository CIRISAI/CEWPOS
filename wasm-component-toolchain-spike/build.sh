#!/usr/bin/env bash
# Known-good build/run for the sandboxed WASM Component spike.
# Proven on: rustc 1.96.0, cargo-component 0.20.0, wasm-tools 1.252.0,
#            wit-bindgen 0.58.0, wasmtime 46.0.1, target wasm32-unknown-unknown.
set -euo pipefail

ROOT="/Users/macmini/CEWPOS/wasm-component-toolchain-spike"
export CARGO_TARGET_DIR="/tmp/wasm-component-target"
WT="/tmp/wasmtools-bin/wasm-tools-1.252.0-aarch64-macos/wasm-tools"   # wasm-tools CLI
CC="/tmp/wasmtools-bin/cargo-component"                                # cargo-component CLI
TGT="$CARGO_TARGET_DIR/wasm32-unknown-unknown/release"
DIST="$ROOT/dist"; mkdir -p "$DIST"

# --- prerequisites (one-time) ------------------------------------------------
# rustup target add wasm32-unknown-unknown
# wasm-tools + cargo-component: prebuilt binaries from bytecodealliance GitHub
#   releases (aarch64), OR `cargo install wasm-tools cargo-component`.

# === PATH A: wit-bindgen + wasm-tools component new (guarantees zero imports) =
echo ">> guest: cargo build -> core module (wasm32-unknown-unknown)"
( cd "$ROOT/guest" && cargo build --release --target wasm32-unknown-unknown )
echo ">> guest: wasm-tools component new -> COMPONENT"
"$WT" component new "$TGT/view_form_guest.wasm" -o "$DIST/view_form_component.wasm"
"$WT" validate --features all "$DIST/view_form_component.wasm"
"$WT" component wit "$DIST/view_form_component.wasm"   # imports: NONE

echo ">> malicious guest (sandbox-breaker): imports a host capability"
( cd "$ROOT/malicious-guest" && cargo build --release --target wasm32-unknown-unknown )
"$WT" component new "$TGT/malicious_guest.wasm" -o "$DIST/malicious_component.wasm"

# === PATH B: cargo-component (blessed; produces the component in one step) ====
echo ">> cc-guest: cargo component build -> COMPONENT"
( cd "$ROOT/cc-guest" && "$CC" component build --release --target wasm32-unknown-unknown )
cp "$TGT/cc_guest.wasm" "$DIST/cc_guest_component.wasm"

# === HOST: wasmtime, EMPTY linker (zero host imports) ========================
echo ">> host: cargo run -> instantiate + call across WIT boundary"
( cd "$ROOT/host" && cargo run --release -- \
    "$DIST/view_form_component.wasm" "$DIST/malicious_component.wasm" )
echo ">> host vs cargo-component-built component"
( cd "$ROOT/host" && cargo run --release -- "$DIST/cc_guest_component.wasm" )
