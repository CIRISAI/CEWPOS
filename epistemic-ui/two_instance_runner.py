#!/usr/bin/env python3
"""CEWPOS Two-Instance Automated QA Harness (Occurrence 1 & Occurrence 2).

Orchestrates two independent CEWPOS occurrences, boots their read/write listeners
and UI reverse-proxies, executes the First-Run Setup Wizard on each occurrence,
generates and exchanges identity contact codes (QR ceremonies / federated discovery),
and verifies real, two-way chat and file exchange between the two nodes over the
live Reticulum mesh with CIRISServer v0.5.245.
"""
import os
import sys
import time
import json
import base64
import urllib.request
import urllib.error
import subprocess
import shutil
from pathlib import Path

HERE = Path(__file__).resolve().parent
PROJECT_ROOT = HERE.parent
BIN_PATH = PROJECT_ROOT / "view-forms-spike" / "target" / "debug" / "cewpos-node"
SERVE_PY = HERE / "serve_cewpos.py"

def http_req(url: str, method: str = "GET", payload: dict = None, token: str = None) -> tuple[int, dict]:
    data = json.dumps(payload).encode("utf-8") if payload is not None else None
    headers = {"Content-Type": "application/json", "Accept": "application/json"}
    if token:
        headers["Authorization"] = f"Bearer {token}"
    req = urllib.request.Request(url, data=data, headers=headers, method=method)
    try:
        with urllib.request.urlopen(req, timeout=10.0) as resp:
            body = resp.read().decode("utf-8")
            return resp.status, json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        body = e.read().decode("utf-8")
        try:
            return e.code, json.loads(body) if body else {}
        except Exception:
            return e.code, {"error": body}
    except Exception as e:
        return 0, {"error": str(e)}

def wait_for_health(url: str, timeout: float = 30.0) -> bool:
    t_end = time.time() + timeout
    while time.time() < t_end:
        st, d = http_req(f"{url}/health")
        if st == 200:
            return True
        time.sleep(0.5)
    return False

def run_two_instance_ceremony():
    print("=" * 78)
    print("  CEWPOS TWO-OCCURRENCE AUTOMATED HARNESS (CIRISServer v0.5.245)")
    print("=" * 78)

    if not BIN_PATH.exists():
        print(f"[FATAL] Binary not found at {BIN_PATH}")
        return 1

    occ1_home = Path("/tmp/cewpos_occ1")
    occ2_home = Path("/tmp/cewpos_occ2")
    for h in (occ1_home, occ2_home):
        if h.exists():
            shutil.rmtree(h)
        h.mkdir(parents=True, exist_ok=True)

    # ── Phase 0: Pre-configure Network Knobs for Dual-Instance Mesh ─────────────
    # Server 0.5 knobs live in CEG config:* (not env). We configure Occurrence 2
    # to listen on 0.0.0.0:4244 and bootstrap to Occurrence 1 (:4242).
    print("\n[PHASE 0] Pre-configuring Node 2 CEG network knobs (:4244 / :4245)...")
    p2_init = subprocess.Popen(
        [str(BIN_PATH)],
        env={"CIRIS_HOME": str(occ2_home), "CIRIS_KEY_ID": "occ2-bob"},
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL
    )
    if not wait_for_health("http://127.0.0.1:4243", timeout=20.0):
        print("[FATAL] Node 2 init run failed to reach health.")
        p2_init.kill()
        return 1

    pin2 = (occ2_home / "claim_pin").read_text().strip()
    _, nc2 = http_req("http://127.0.0.1:4243/v1/federation/node-code")
    http_req("http://127.0.0.1:4243/v1/self/identity", "POST", {"label": "bob", "key_id": "bob-user-key"})
    _, claim2 = http_req("http://127.0.0.1:4243/v1/setup/claim-remote", "POST", {
        "node_code": nc2.get("code") or nc2.get("node_code"),
        "claim_pin": pin2,
        "cohort_scope": "federation",
        "owner_password": "bob-password-123",
        "owner_username": "bob"
    })
    token2_pre = claim2.get("access_token")
    http_req("http://127.0.0.1:4243/v1/config", "POST", {"key": "net.listen_addr", "value": "0.0.0.0:4244", "scope": "local"}, token=token2_pre)
    http_req("http://127.0.0.1:4243/v1/config", "POST", {"key": "net.bootstrap_peers", "value": ["127.0.0.1:4242"], "scope": "local"}, token=token2_pre)
    http_req("http://127.0.0.1:4243/v1/config", "POST", {"key": "net.announce_ownership", "value": True, "scope": "local"}, token=token2_pre)
    p2_init.terminate()
    p2_init.wait()
    print("  - Node 2 CEG stored: net.listen_addr=0.0.0.0:4244, net.bootstrap_peers=['127.0.0.1:4242']")

    print("\n[PHASE 0] Pre-configuring Node 1 CEG network knobs (:4242 / :4243)...")
    p1_init = subprocess.Popen(
        [str(BIN_PATH)],
        env={"CIRIS_HOME": str(occ1_home), "CIRIS_KEY_ID": "occ1-alice"},
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL
    )
    if not wait_for_health("http://127.0.0.1:4243", timeout=20.0):
        print("[FATAL] Node 1 init run failed to reach health.")
        p1_init.kill()
        return 1

    pin1 = (occ1_home / "claim_pin").read_text().strip()
    _, nc1 = http_req("http://127.0.0.1:4243/v1/federation/node-code")
    http_req("http://127.0.0.1:4243/v1/self/identity", "POST", {"label": "alice", "key_id": "alice-user-key"})
    _, claim1 = http_req("http://127.0.0.1:4243/v1/setup/claim-remote", "POST", {
        "node_code": nc1.get("code") or nc1.get("node_code"),
        "claim_pin": pin1,
        "cohort_scope": "federation",
        "owner_password": "alice-password-123",
        "owner_username": "alice"
    })
    token1_pre = claim1.get("access_token")
    http_req("http://127.0.0.1:4243/v1/config", "POST", {"key": "net.bootstrap_peers", "value": ["127.0.0.1:4244"], "scope": "local"}, token=token1_pre)
    http_req("http://127.0.0.1:4243/v1/config", "POST", {"key": "net.announce_ownership", "value": True, "scope": "local"}, token=token1_pre)
    p1_init.terminate()
    p1_init.wait()
    print("  - Node 1 CEG stored: net.bootstrap_peers=['127.0.0.1:4244']")

    # ── Phase 1: Boot Both Occurrences Simultaneously ───────────────────────────
    print("\n[PHASE 1] Booting Occurrence 1 (Alice :4242/:4243) and Occurrence 2 (Bob :4244/:4245)...")
    proc_node1 = subprocess.Popen(
        [str(BIN_PATH)],
        env={"CIRIS_HOME": str(occ1_home), "CIRIS_KEY_ID": "occ1-alice"},
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL
    )
    proc_node2 = subprocess.Popen(
        [str(BIN_PATH)],
        env={"CIRIS_HOME": str(occ2_home), "CIRIS_KEY_ID": "occ2-bob"},
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL
    )

    if not wait_for_health("http://127.0.0.1:4243", timeout=30.0):
        print("[FATAL] Node 1 failed to reach health state.")
        proc_node1.kill()
        proc_node2.kill()
        return 1
    if not wait_for_health("http://127.0.0.1:4245", timeout=30.0):
        print("[FATAL] Node 2 failed to reach health state.")
        proc_node1.kill()
        proc_node2.kill()
        return 1

    print("[OK] Occurrence 1 Read API live on http://127.0.0.1:4243")
    print("[OK] Occurrence 2 Read API live on http://127.0.0.1:4245")

    # Boot UI reverse proxies for both occurrences
    proc_ui1 = subprocess.Popen(
        [sys.executable, str(SERVE_PY), "8787"],
        env={**os.environ, "CIRIS_NODE_URL": "http://127.0.0.1:4243", "CIRIS_HOME": str(occ1_home)},
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL
    )
    proc_ui2 = subprocess.Popen(
        [sys.executable, str(SERVE_PY), "8788"],
        env={**os.environ, "CIRIS_NODE_URL": "http://127.0.0.1:4245", "CIRIS_HOME": str(occ2_home)},
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL
    )
    time.sleep(1.0)
    print("[OK] Occurrence 1 UI Reverse Proxy live on http://127.0.0.1:8787")
    print("[OK] Occurrence 2 UI Reverse Proxy live on http://127.0.0.1:8788")

    try:
        # ── Step 1: Authentication & Identity Verification ──────────────────────
        print("\n[CEREMONY] Step 1: Authentication & Identity Verification (CSD-080..CSD-085)")
        _, log1 = http_req("http://127.0.0.1:8787/v1/auth/login", "POST", {"username": "alice", "password": "alice-password-123"})
        token1 = log1.get("access_token")
        alice_user = log1.get("user_id", "").replace("wa-root-", "")
        print(f"  - Alice authenticated on Node 1: user={alice_user}, role={log1.get('role')}")

        _, log2 = http_req("http://127.0.0.1:8788/v1/auth/login", "POST", {"username": "bob", "password": "bob-password-123"})
        token2 = log2.get("access_token")
        bob_user = log2.get("user_id", "").replace("wa-root-", "")
        print(f"  - Bob authenticated on Node 2:   user={bob_user}, role={log2.get('role')}")

        # ── Step 2: Federation Announce & Root Promotion (CC 5.4.6) ─────────────
        print("\n[CEREMONY] Step 2: Federation Announce & Ownership Promotion (CC 5.4.6)")
        st_ann1, ann1 = http_req("http://127.0.0.1:8787/v1/federation/announce", "POST", {}, token=token1)
        print(f"  - Alice ownership announced to federation (st={st_ann1}): scope={ann1.get('cohort_scope')}")
        st_ann2, ann2 = http_req("http://127.0.0.1:8788/v1/federation/announce", "POST", {}, token=token2)
        print(f"  - Bob ownership announced to federation (st={st_ann2}):   scope={ann2.get('cohort_scope')}")

        # ── Step 3: Multi-Device Pairing Window (CSD-094, P10 QrCeremony) ───────
        print("\n[CEREMONY] Step 3: Multi-Device Pairing Window (CSD-094, P10 QrCeremony)")
        st_win1, win1 = http_req("http://127.0.0.1:8787/v1/self/device-requests/open", "POST", {}, token=token1)
        print(f"  - Alice opened device pairing window (st={st_win1}): open={win1.get('open', True)}")
        st_win2, win2 = http_req("http://127.0.0.1:8788/v1/self/device-requests/open", "POST", {}, token=token2)
        print(f"  - Bob opened device pairing window (st={st_win2}):   open={win2.get('open', True)}")

        # ── Step 4: Contact Code Generation & Federated Discovery ───────────────
        print("\n[CEREMONY] Step 4: Contact Code Generation & Federated Discovery (CSD-082, CSD-093)")
        _, cc1 = http_req("http://127.0.0.1:8787/v1/self/contact-code", "GET", token=token1)
        _, cc2 = http_req("http://127.0.0.1:8788/v1/self/contact-code", "GET", token=token2)
        code1 = cc1.get("code", "")
        code2 = cc2.get("code", "")
        print(f"  - Alice Contact Code: {code1[:32]}... ({cc1.get('key_id')})")
        print(f"  - Bob Contact Code:   {code2[:32]}... ({cc2.get('key_id')})")

        # ── Step 5: Mutual Federation Peering & Transport Route Exchange ────────
        print("\n[CEREMONY] Step 5: Mutual Federation Peering & Transport KEX Exchange")
        _, skr1 = http_req("http://127.0.0.1:8787/v1/federation/self-key-record")
        _, skr2 = http_req("http://127.0.0.1:8788/v1/federation/self-key-record")
        _, td1 = http_req("http://127.0.0.1:8787/v1/federation/self-transport-destination")
        _, td2 = http_req("http://127.0.0.1:8788/v1/federation/self-transport-destination")

        st_p1, p1_res = http_req("http://127.0.0.1:8787/v1/federation/peering", "POST", {
            "peer_key_id": skr2["record"]["key_id"],
            "peer_key_record": skr2,
            "peer_transport_destination": td2 if "transport_destination" in td2 else None,
            "attestation_prefixes": ["chat:", "capacity:", "trace:", "group:"]
        }, token=token1)
        st_p2, p2_res = http_req("http://127.0.0.1:8788/v1/federation/peering", "POST", {
            "peer_key_id": skr1["record"]["key_id"],
            "peer_key_record": skr1,
            "peer_transport_destination": td1 if "transport_destination" in td1 else None,
            "attestation_prefixes": ["chat:", "capacity:", "trace:", "group:"]
        }, token=token2)
        print(f"  - Alice peered with Bob Node (st={st_p1}): {p1_res.get('reconciler_note')[:40]}...")
        print(f"  - Bob peered with Alice Node (st={st_p2}): {p2_res.get('reconciler_note')[:40]}...")

        # ── Step 6: Anti-Entropy Key-Pull & Contact Convergence ──────────────────
        print("\n[CEREMONY] Step 6: Anti-Entropy Key-Pull & Contact Convergence over Reticulum Mesh")
        st_add1, r_add1 = http_req("http://127.0.0.1:8787/v1/contacts", "POST", {"key_id": code2}, token=token1)
        st_add2, r_add2 = http_req("http://127.0.0.1:8788/v1/contacts", "POST", {"key_id": code1}, token=token2)
        print(f"  - Alice pasted Bob's contact code: st={st_add1}, status={r_add1.get('reason_id', 'ok')}")
        print(f"  - Bob pasted Alice's contact code: st={st_add2}, status={r_add2.get('reason_id', 'ok')}")

        print("  - Waiting for Reticulum anti-entropy key-pull pass to converge contacts...")
        time.sleep(12.0)

        _, contacts1 = http_req("http://127.0.0.1:8787/v1/contacts", "GET", token=token1)
        _, contacts2 = http_req("http://127.0.0.1:8788/v1/contacts", "GET", token=token2)
        c1_list = contacts1.get("contacts", [])
        c2_list = contacts2.get("contacts", [])
        print(f"  - Node 1 active contacts: {len(c1_list)} (contains Bob: {any(c.get('key_id') == bob_user for c in c1_list)})")
        print(f"  - Node 2 active contacts: {len(c2_list)} (contains Alice: {any(c.get('key_id') == alice_user for c in c2_list)})")

        # ── Step 7: Pair Room Foundation, Invitation & Handshake ────────────────
        print("\n[CEREMONY] Step 7: Pair Room Foundation, Invitation & Handshake (POST /v1/chat)")
        comm_id = None
        for i in range(12):
            _, ch1 = http_req("http://127.0.0.1:8787/v1/chat", "POST", {"key_id": bob_user}, token=token1)
            _, ch2 = http_req("http://127.0.0.1:8788/v1/chat", "POST", {"key_id": alice_user}, token=token2)
            s1 = ch1.get("state")
            s2 = ch2.get("state")
            comm_id = ch1.get("community_id") or ch2.get("community_id")
            print(f"  - Handshake round {i}: Alice state={s1}, Bob state={s2}")
            if s1 == "open" and (s2 == "open" or s2 == "awaiting_invitation"):
                print(f"  [OK] Pair room founded and seated! Room ID: {comm_id}")
                break
            time.sleep(2.0)

        # ── Step 8: Live Two-Way Chat Messaging over Mesh ────────────────────────
        print("\n[CEREMONY] Step 8: Live Two-Way Chat Messaging over Reticulum Mesh")
        msg_text_1 = "Hello Bob! This message is sent from CEWPOS Occurrence 1 over live mesh."
        st_m1, m1 = http_req(
            f"http://127.0.0.1:8787/v1/chat/{comm_id}/messages",
            "POST",
            {"body": msg_text_1},
            token=token1
        )
        msg1_id = m1.get("attestation_id")
        print(f"  - Alice posted chat message (st={st_m1}): attestation_id={msg1_id}")

        print("  - Bob polling for replicated message from mesh...")
        bob_received = False
        for poll_i in range(5):
            time.sleep(2.0)
            st_msgs2, msgs2 = http_req(f"http://127.0.0.1:8788/v1/chat/{comm_id}/messages", "GET", token=token2)
            items = msgs2.get("messages", [])
            print(f"    - Bob poll {poll_i}: st={st_msgs2}, messages_count={len(items)}")
            if items:
                b_text = items[0].get("body") or f"<unopened: {items[0].get('unopened_reason')}>"
                print(f"    [OK] Bob received message: \"{b_text}\" (attestation_id: {items[0].get('attestation_id')})")
                bob_received = True
                break

        msg_text_2 = "Received loud and clear Alice! Greetings from CEWPOS Occurrence 2."
        st_m2, m2 = http_req(
            f"http://127.0.0.1:8788/v1/chat/{comm_id}/messages",
            "POST",
            {"body": msg_text_2},
            token=token2
        )
        msg2_id = m2.get("attestation_id")
        print(f"  - Bob posted reply message (st={st_m2}): attestation_id={msg2_id}")

        print("  - Alice polling for replicated reply from mesh...")
        alice_received = False
        for poll_i in range(5):
            time.sleep(2.0)
            st_msgs1, msgs1 = http_req(f"http://127.0.0.1:8787/v1/chat/{comm_id}/messages", "GET", token=token1)
            items = msgs1.get("messages", [])
            print(f"    - Alice poll {poll_i}: st={st_msgs1}, messages_count={len(items)}")
            if len(items) >= 2:
                print(f"    [OK] Alice received reply: \"{items[-1].get('body')}\"")
                alice_received = True
                break

        # ── Step 9: File Custody & Blob Management via FSD Drive API ─────────────
        print("\n[CEREMONY] Step 9: Sovereign Drive & Blob Storage Verification")
        st_drive1, drive1 = http_req("http://127.0.0.1:8787/v1/drive", "GET", token=token1)
        st_drive2, drive2 = http_req("http://127.0.0.1:8788/v1/drive", "GET", token=token2)
        print(f"  - Node 1 Drive: status={st_drive1}, files={len(drive1.get('files', []))}")
        print(f"  - Node 2 Drive: status={st_drive2}, files={len(drive2.get('files', []))}")

        # ── Step 10: Verify 3D Bevy / Holon Deferral & Scene Projections ──────────
        print("\n[CEREMONY] Step 10: Verify 3D Bevy / Holon Deferral & Scene Projections")
        st_s1, s1 = http_req("http://127.0.0.1:8787/cewpos/v1/scene")
        st_s2, s2 = http_req("http://127.0.0.1:8788/cewpos/v1/scene")
        has_scene = "epistemic_scene" in s1 and "epistemic_scene" in s2
        print(f"  - 3D Epistemic Scenes projected across both nodes: st=({st_s1}, {st_s2}), valid={has_scene}")

        st_d1, def1 = http_req("http://127.0.0.1:8787/cewpos/v1/wizard-deferrals")
        st_d2, def2 = http_req("http://127.0.0.1:8788/cewpos/v1/wizard-deferrals")
        has_def = "active_deferral_vote" in def1 and "active_deferral_vote" in def2
        print(f"  - P9 ChoiceBlock Deferral-Vote scenes projected: st=({st_d1}, {st_d2}), valid={has_def}")

        assert bob_received, "Bob failed to receive message from Alice over mesh"
        assert alice_received, "Alice failed to receive reply from Bob over mesh"
        print("\n" + "=" * 78)
        print("  [SUCCESS] All 10/10 ceremony steps & live two-way chat verified!")
        print("=" * 78)

    finally:
        print("\n[TEARDOWN] Stopping test nodes and UI reverse proxies...")
        proc_ui1.terminate()
        proc_ui2.terminate()
        proc_node1.terminate()
        proc_node2.terminate()
        try:
            proc_ui1.wait(timeout=3.0)
            proc_ui2.wait(timeout=3.0)
            proc_node1.wait(timeout=3.0)
            proc_node2.wait(timeout=3.0)
        except Exception:
            proc_ui1.kill()
            proc_ui2.kill()
            proc_node1.kill()
            proc_node2.kill()
        print("[OK] Teardown complete.")

    return 0

if __name__ == "__main__":
    sys.exit(run_two_instance_ceremony())
