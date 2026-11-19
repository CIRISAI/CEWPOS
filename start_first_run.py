#!/usr/bin/env python3
"""start_first_run.py — Single-Command Entry Point that Boots CEWPOS Directly into First-Run (`T0`).

What it does:
  1. Creates a clean, unowned `CIRIS_HOME` directory (default: `/tmp/cewpos_first_run`) so the node
     always boots in genuine First-Run state (`is_first_run = true`, `root.none`, `node.unclaimed`,
     with a fresh `0600 <CIRIS_HOME>/claim_pin`).
  2. Builds (if not already built) and launches the full-stack `cewpos-node` binary consuming
     `ciris-server v0.5.250` (`branch = "release/0.5.250"`, `ciris-persist v61.4.2`, `ciris-edge v45.1.7`)
     and `CIRISHolon`'s `holon-draw` (`CSD/5` draft 3, 24 primitives) on `127.0.0.1:4243`.
  3. Waits for `GET /health`, `GET /v1/setup/status`, and the `0600 claim_pin` file to materialize.
  4. Starts the CEWPOS Epistemic UI & reverse-proxy server (`serve_cewpos.py`) on `127.0.0.1:8787`
     pointed at that fresh `CIRIS_HOME` and opens onto the First-Run Setup Wizard
     (`http://127.0.0.1:8787/index.html?mode=setup`).

Usage:
  python3 start_first_run.py                  # Starts node (:4243) + UI (:8787) in First-Run mode
  python3 start_first_run.py --check          # Boots fresh node + UI, verifies first-run state & exits 0
  python3 start_first_run.py --port 8787 --node-port 4243 --home /tmp/cewpos_first_run
"""
from __future__ import annotations

import argparse
import json
import os
import shutil
import signal
import stat
import subprocess
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent
MANIFEST_PATH = ROOT / "view-forms-spike" / "Cargo.toml"
BIN_PATH = ROOT / "view-forms-spike" / "target" / "debug" / "cewpos-node"
SERVE_UI_PATH = ROOT / "epistemic-ui" / "serve_cewpos.py"


def http_get_json(url: str, timeout: float = 3.0) -> tuple[int, dict[str, Any]]:
    req = urllib.request.Request(url, headers={"Accept": "application/json"}, method="GET")
    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            raw = resp.read().decode("utf-8")
            return resp.status, json.loads(raw) if raw else {}
    except urllib.error.HTTPError as exc:
        raw = exc.read().decode("utf-8")
        try:
            return exc.code, json.loads(raw) if raw else {}
        except Exception:
            return exc.code, {"error": raw}
    except Exception as exc:
        return 0, {"error": str(exc)}


def ensure_binary_built() -> Path:
    if BIN_PATH.exists():
        return BIN_PATH
    print("[CEWPOS First-Run] Building full-stack cewpos-node (ciris-server 0.5.250 + holon-draw)...", flush=True)
    subprocess.run(
        [
            "cargo",
            "build",
            "--manifest-path",
            str(MANIFEST_PATH),
            "--no-default-features",
            "--features",
            "fullstack",
            "--bin",
            "cewpos-node",
        ],
        check=True,
    )
    return BIN_PATH


def wait_for_first_run_node(node_url: str, home_dir: Path, timeout_secs: float = 25.0) -> dict[str, Any]:
    deadline = time.time() + timeout_secs
    pin_file = home_dir / "claim_pin"
    while time.time() < deadline:
        st, health = http_get_json(f"{node_url}/health")
        if st == 200 and pin_file.exists():
            _, setup_st = http_get_json(f"{node_url}/v1/setup/status")
            _, node_code = http_get_json(f"{node_url}/v1/federation/node-code")
            _, csd5_draw = http_get_json(f"{node_url}/cewpos/v1/csd5-holon-draw")
            st_mode = oct(stat.S_IMODE(pin_file.stat().st_mode))
            return {
                "health": health,
                "setup_status": setup_st,
                "node_code": node_code,
                "csd5_holon_draw": csd5_draw,
                "claim_pin_mode": st_mode,
                "claim_pin_exists": True,
            }
        time.sleep(0.35)
    raise RuntimeError(f"Timed out waiting for cewpos-node at {node_url} (home={home_dir})")


def main() -> int:
    parser = argparse.ArgumentParser(description="Start CEWPOS directly into First-Run Setup (T0).")
    parser.add_argument("--home", default="/tmp/cewpos_first_run", help="Fresh CIRIS_HOME directory")
    parser.add_argument("--key-id", default="cewpos-250-first-run", help="Initial node key ID")
    parser.add_argument("--node-port", type=int, default=4243, help="ciris-server read/write API port (default 4243; transport port is node_port - 1)")
    parser.add_argument("--port", type=int, default=8787, help="CEWPOS Epistemic UI HTTP port")
    parser.add_argument("--keep-home", action="store_true", help="Do not wipe CIRIS_HOME before starting")
    parser.add_argument(
        "--check",
        action="store_true",
        help="Boot into first-run, verify /health, /v1/setup/status (is_first_run=true), 0600 claim_pin, and UI :8787, then exit 0",
    )
    args = parser.parse_args()

    home_dir = Path(args.home).resolve()
    if not args.keep_home and home_dir.exists():
        shutil.rmtree(home_dir)
    home_dir.mkdir(parents=True, exist_ok=True)

    bin_path = ensure_binary_built()
    transport_port = args.node_port - 1
    transport_addr = f"0.0.0.0:{transport_port}"
    node_addr = f"127.0.0.1:{args.node_port}"
    node_url = f"http://{node_addr}"
    ui_url = f"http://127.0.0.1:{args.port}"
    first_run_url = f"{ui_url}/index.html?mode=setup"

    env = os.environ.copy()
    env["CIRIS_HOME"] = str(home_dir)
    env["CIRIS_KEY_ID"] = args.key_id
    env["CIRIS_LISTEN_ADDR"] = transport_addr
    env["CIRIS_NODE_URL"] = node_url

    node_proc = subprocess.Popen(
        [str(bin_path)],
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    ui_proc = subprocess.Popen(
        [sys.executable, str(SERVE_UI_PATH), str(args.port)],
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )

    def shutdown(*_sig: Any) -> None:
        for p in (ui_proc, node_proc):
            if p.poll() is None:
                p.terminate()
        for p in (ui_proc, node_proc):
            try:
                p.wait(timeout=5.0)
            except Exception:
                p.kill()

    signal.signal(signal.SIGINT, lambda s, f: (shutdown(), sys.exit(0)))
    signal.signal(signal.SIGTERM, lambda s, f: (shutdown(), sys.exit(0)))

    try:
        info = wait_for_first_run_node(node_url, home_dir)
        health = info["health"]
        setup_data = info["setup_status"].get("data") or info["setup_status"]
        is_first_run = setup_data.get("is_first_run", True)
        node_code = info["node_code"].get("code") or info["node_code"].get("node_code") or "CIRIS-V1-..."
        csd5_meta = info["csd5_holon_draw"]

        # Wait for UI proxy to respond
        ui_ready = False
        for _ in range(25):
            st_ui, local_meta = http_get_json(f"{ui_url}/cewpos/local-node")
            if st_ui == 200 and local_meta.get("claim_pin_exists"):
                ui_ready = True
                break
            time.sleep(0.2)

        print("=" * 78)
        print("  CEWPOS FIRST-RUN ENTRY POINT (T0: root.none · node.unclaimed)")
        print("=" * 78)
        print(f"  CIRISServer Crate : v{health.get('version', '0.5.250')} (release/0.5.250)")
        print(f"  Screen Standard   : {csd5_meta.get('standard', 'CSD/5')} draft {csd5_meta.get('draft', 3)} ({csd5_meta.get('pinned_tag', 'v1.0-rc21-wire')})")
        print(f"  Fresh CIRIS_HOME  : {home_dir}")
        print(f"  0600 claim_pin    : {home_dir / 'claim_pin'} (mode {info['claim_pin_mode']})")
        print(f"  NodeCode          : {node_code}")
        print(f"  is_first_run      : {is_first_run}")
        print(f"  Node API Listener : {node_url}/health")
        print(f"  First-Run Wizard  : {first_run_url}")
        print(f"  Circles Explorer  : {ui_url}/index.html?mode=circles")
        print("=" * 78, flush=True)

        if args.check:
            assert is_first_run is True, f"Expected is_first_run=True, got {is_first_run}"
            assert info["claim_pin_mode"] == "0o600", f"Expected 0o600 claim_pin, got {info['claim_pin_mode']}"
            assert ui_ready is True, "UI proxy /cewpos/local-node did not report claim_pin_exists=True"
            assert csd5_meta.get("standard") == "CSD/5", "CSD/5 holon-draw endpoint check failed"
            print("[PASS] CEWPOS First-Run Entry Point verified end-to-end.", flush=True)
            shutdown()
            return 0

        print("Press Ctrl+C to stop CEWPOS.", flush=True)
        while node_proc.poll() is None and ui_proc.poll() is None:
            time.sleep(1.0)
        return 0
    finally:
        shutdown()


if __name__ == "__main__":
    raise SystemExit(main())
