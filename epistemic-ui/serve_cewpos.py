#!/usr/bin/env python3
"""CEWPOS Epistemic UI HTTP Server & Live CIRISServer v0.5.241 Reverse Proxy.

Serves `/home/emoore/CEWPOS/epistemic-ui` on `http://127.0.0.1:8787` and forwards
`/health` and `/v1/*` requests to the local `CIRISServer` read API (`http://127.0.0.1:4243`)
so the browser UI can interact with a live local node without cross-origin refusals.
Also exposes `GET /cewpos/local-node` to surface local first-run metadata (such as
whether `/tmp/ciris_qa_fresh/claim_pin` exists and its mode) for local QA.
"""
from __future__ import annotations

import http.server
import json
import os
import stat
import sys
import urllib.error
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
NODE_BASE = os.environ.get("CIRIS_NODE_URL", "http://127.0.0.1:4243")
QA_HOME = Path(os.environ.get("CIRIS_HOME", "/tmp/ciris_qa_fresh"))


class CewposRequestHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(HERE), **kwargs)

    def log_message(self, format: str, *args) -> None:
        sys.stderr.write(f"[CEWPOS :8787] {format % args}\n")

    def _send_json(self, status: int, payload: object) -> None:
        raw = json.dumps(payload).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(raw)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(raw)

    def _proxy_to_node(self, method: str) -> None:
        target_url = f"{NODE_BASE}{self.path}"
        content_len = int(self.headers.get("Content-Length", "0") or "0")
        body = self.rfile.read(content_len) if content_len > 0 else None

        headers: dict[str, str] = {}
        for k in ("Content-Type", "Accept", "Authorization"):
            val = self.headers.get(k)
            if val:
                headers[k] = val

        req = urllib.request.Request(target_url, data=body, headers=headers, method=method)
        try:
            with urllib.request.urlopen(req, timeout=10.0) as resp:
                resp_body = resp.read()
                self.send_response(resp.status)
                self.send_header("Content-Type", resp.headers.get("Content-Type", "application/json"))
                self.send_header("Content-Length", str(len(resp_body)))
                self.send_header("Cache-Control", "no-store")
                self.end_headers()
                self.wfile.write(resp_body)
        except urllib.error.HTTPError as exc:
            err_body = exc.read()
            self.send_response(exc.code)
            self.send_header("Content-Type", exc.headers.get("Content-Type", "application/json"))
            self.send_header("Content-Length", str(len(err_body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(err_body)
        except Exception as exc:
            self._send_json(502, {"error": f"Node unreachable at {target_url}: {exc}"})

    def do_GET(self) -> None:
        if self.path == "/cewpos/local-node":
            pin_path = QA_HOME / "claim_pin"
            pin_exists = pin_path.exists()
            pin_val = None
            pin_mode = None
            if pin_exists:
                try:
                    st = pin_path.stat()
                    pin_mode = oct(stat.S_IMODE(st.st_mode))
                    pin_val = pin_path.read_text(encoding="utf-8").strip()
                except Exception:
                    pass
            self._send_json(
                200,
                {
                    "node_base": NODE_BASE,
                    "ciris_home": str(QA_HOME),
                    "claim_pin_exists": pin_exists,
                    "claim_pin_mode": pin_mode,
                    "claim_pin": pin_val,
                },
            )
            return
        if self.path == "/health" or self.path.startswith("/v1/"):
            self._proxy_to_node("GET")
            return
        super().do_GET()

    def do_POST(self) -> None:
        if self.path.startswith("/v1/"):
            self._proxy_to_node("POST")
            return
        self._send_json(404, {"error": "Not found"})


def main() -> int:
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8787
    server = http.server.ThreadingHTTPServer(("127.0.0.1", port), CewposRequestHandler)
    print(f"CEWPOS Epistemic UI serving {HERE} at http://127.0.0.1:{port} (proxying /v1/* -> {NODE_BASE})", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        return 0
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
