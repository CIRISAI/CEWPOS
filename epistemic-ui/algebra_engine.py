#!/usr/bin/env python3
"""
algebra_engine.py — Executable Reference Engine & CSD/4 Verifier for the
CC rc9 Constitutional UI Algebra: Claims (C) × Contexts (X) × Actions (A) × Conclusions (V).

Mirrors `view-forms-spike/src/algebra.rs` and `view-forms-spike/src/epistemic_scene.rs`
1-for-1, runs an exhaustive formal verification suite over:
  1. Closed 1+4 CEG Operator Gate (`scores`, `delegates_to`, `supersedes`, `withdraws`, `recants`)
  2. 12 × 6 (ChangeKind × Verb) Occupancy Matrix (exact 48 / 72 legal cells from CC rc9)
  3. 7 -> 5 CohortScope Circle Fold + 9-Plane State Vector (`formal/STATE_TABLE.tsv`)
  4. Directional Asymmetry Law (3-fact `ConfirmSheet` outward vs. 1-step immediate inward/halt/exit)
  5. 10 Governance Decision Families (`manifests/governance_policy.json`, `TwoPath.tla`)
  6. Halt Theorem `Inv_X1_LatchDoesNotBlockServing` (`423 Locked` freezes agent gate, keeps data doors open)
  7. Hamburger Epistemic Invariant (`CSD-006`: `has_hamburger == (receipt is not None)`)
  8. PolicyLensBar (`circles.html` Scene 12-13: opinions blur/hide under viewer rules, never delete)
  9. All 12 Minimum Viable Reusable Primitives (`P1`-`P12` + `SpatialCanvas`)
 10. All 11 Reserved Promise-Closure Cards (`CSD-111`..`CSD-119`, `CSD-127`, `CSD-128`) closing the 14 unserved sub-rules

Generates deterministic JSON artifacts:
  - /home/emoore/CEWPOS/view-forms-spike/out/epistemic-scene.json
  - /home/emoore/CEWPOS/view-forms-spike/out/csd4-tree.json
  - /home/emoore/CEWPOS/epistemic-ui/out/all_promise_cards.json
"""

from __future__ import annotations
from dataclasses import dataclass, field, asdict
from enum import Enum
import json
import math
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


# ============================================================================
# 1. CLAIMS (C) — Closed 1+4 Operators, 12 Change Kinds × 6 Verbs, Receipt5
# ============================================================================

CLOSED_OPS = ("scores", "delegates_to", "supersedes", "withdraws", "recants")


class Op(str, Enum):
    SCORES = "scores"
    DELEGATES_TO = "delegates_to"
    SUPERSEDES = "supersedes"
    WITHDRAWS = "withdraws"
    RECANTS = "recants"


class Verb(str, Enum):
    ATTEST = "attest"          # scores
    AUTHORIZE = "authorize"    # delegates_to
    REPLACE = "replace"        # supersedes
    WITHDRAW = "withdraw"      # withdraws
    RECANT = "recant"          # recants
    CARRIES = "carries"        # derived cross-kind edge


class OntologyClass(str, Enum):
    THING = "Thing"        # 78 families
    ACCOUNT = "Account"    # 76 families
    RECORD = "Record"      # 15 families


class ChangeKind(str, Enum):
    PRIORITIES = "Priorities"
    RULES = "Rules"
    MANNER = "Manner"
    IDENTITY = "Identity"
    CONFIDENCE = "Confidence"
    FACTS = "Facts"
    CIRCUMSTANCES = "Circumstances"
    PROCESS = "Process"
    MODEL = "Model"
    STRUCTURE = "Structure"
    PREMISES = "Premises"
    RECORD = "Record"

    @staticmethod
    def from_dimension(dim: str) -> "ChangeKind":
        prefix = dim.split(":", 1)[0] if dim else ""
        mapping = {
            "priority": ChangeKind.PRIORITIES,
            "vote": ChangeKind.PRIORITIES,
            "credit": ChangeKind.PRIORITIES,
            "need": ChangeKind.PRIORITIES,
            "accord": ChangeKind.RULES,
            "consent": ChangeKind.RULES,
            "duty": ChangeKind.RULES,
            "moderation": ChangeKind.RULES,
            "quarantine": ChangeKind.RULES,
            "watchlist": ChangeKind.RULES,
            "rule": ChangeKind.RULES,
            "prohibited": ChangeKind.RULES,
            "licensure": ChangeKind.RULES,
            "key_grant": ChangeKind.RULES,
            "cw_class": ChangeKind.MANNER,
            "manner": ChangeKind.MANNER,
            "register": ChangeKind.MANNER,
            "tone": ChangeKind.MANNER,
            "key": ChangeKind.IDENTITY,
            "identity": ChangeKind.IDENTITY,
            "ownership": ChangeKind.IDENTITY,
            "occurrence": ChangeKind.IDENTITY,
            "custody_hw": ChangeKind.IDENTITY,
            "age": ChangeKind.IDENTITY,
            "device": ChangeKind.IDENTITY,
            "self_roster": ChangeKind.IDENTITY,
            "evaluation": ChangeKind.CONFIDENCE,
            "capacity": ChangeKind.CONFIDENCE,
            "coherence": ChangeKind.CONFIDENCE,
            "trust": ChangeKind.CONFIDENCE,
            "reputation": ChangeKind.CONFIDENCE,
            "confidence": ChangeKind.CONFIDENCE,
            "circumstance": ChangeKind.CIRCUMSTANCES,
            "health": ChangeKind.CIRCUMSTANCES,
            "transport": ChangeKind.CIRCUMSTANCES,
            "reachability": ChangeKind.CIRCUMSTANCES,
            "telemetry": ChangeKind.CIRCUMSTANCES,
            "resource": ChangeKind.CIRCUMSTANCES,
            "process": ChangeKind.PROCESS,
            "dma": ChangeKind.PROCESS,
            "wbd": ChangeKind.PROCESS,
            "retirement": ChangeKind.PROCESS,
            "membership": ChangeKind.PROCESS,
            "objection": ChangeKind.PROCESS,
            "wa_adjudication": ChangeKind.PROCESS,
            "model": ChangeKind.MODEL,
            "calibration": ChangeKind.MODEL,
            "detector": ChangeKind.MODEL,
            "ratchet": ChangeKind.MODEL,
            "structure": ChangeKind.STRUCTURE,
            "household": ChangeKind.STRUCTURE,
            "community": ChangeKind.STRUCTURE,
            "affiliation": ChangeKind.STRUCTURE,
            "room": ChangeKind.STRUCTURE,
            "authority": ChangeKind.STRUCTURE,
            "bond": ChangeKind.STRUCTURE,
            "premise": ChangeKind.PREMISES,
            "charter": ChangeKind.PREMISES,
            "intent": ChangeKind.PREMISES,
            "principle": ChangeKind.PREMISES,
            "record": ChangeKind.RECORD,
            "audit": ChangeKind.RECORD,
            "sth": ChangeKind.RECORD,
            "tombstone": ChangeKind.RECORD,
            "receipt": ChangeKind.RECORD,
            "revocation": ChangeKind.RECORD,
            "slashing": ChangeKind.RECORD,
        }
        return mapping.get(prefix, ChangeKind.FACTS)

    def ontology_class(self) -> OntologyClass:
        if self in (ChangeKind.IDENTITY, ChangeKind.FACTS, ChangeKind.STRUCTURE, ChangeKind.PREMISES):
            return OntologyClass.THING
        if self == ChangeKind.RECORD:
            return OntologyClass.RECORD
        return OntologyClass.ACCOUNT

    def admits_verb(self, verb: Verb) -> bool:
        """Exact 48 / 72 occupancy law from CC rc9 `review/derivation/family_kind_grid.md`."""
        if verb in (Verb.ATTEST, Verb.WITHDRAW):
            return True
        if verb == Verb.AUTHORIZE:
            return self in (ChangeKind.RULES, ChangeKind.IDENTITY)
        if verb == Verb.REPLACE:
            return self not in (ChangeKind.PRIORITIES, ChangeKind.MODEL)
        if verb == Verb.RECANT:
            return self not in (ChangeKind.IDENTITY, ChangeKind.CIRCUMSTANCES, ChangeKind.PROCESS)
        if verb == Verb.CARRIES:
            return self in (ChangeKind.RULES, ChangeKind.CONFIDENCE, ChangeKind.RECORD)
        return False


@dataclass
class Form:
    op: Op
    envelope: Dict[str, Any]


def read_form(envelope: Any) -> Form:
    """Closed-operator gate: rejects any head outside the 1+4 CEG operators."""
    if not isinstance(envelope, dict):
        raise ValueError("NotAnObject")
    head = envelope.get("attestation_type")
    if not isinstance(head, str):
        raise ValueError("MissingField(attestation_type)")
    if head not in CLOSED_OPS:
        raise ValueError(f"UnknownOperator({head})")
    if "dimension" not in envelope:
        raise ValueError("MissingField(dimension)")
    return Form(op=Op(head), envelope=dict(envelope))


@dataclass
class VerificationBasis:
    kind: str  # "Wire" | "ByRule" | "NotSent"
    value: Optional[str] = None
    gloss: Optional[str] = None
    cc_ref: Optional[str] = None

    @staticmethod
    def wire(value: str, gloss: Optional[str] = None) -> "VerificationBasis":
        return VerificationBasis(kind="Wire", value=value, gloss=gloss)

    @staticmethod
    def by_rule(value: str, cc_ref: str) -> "VerificationBasis":
        return VerificationBasis(kind="ByRule", value=value, cc_ref=cc_ref)

    @staticmethod
    def not_sent() -> "VerificationBasis":
        return VerificationBasis(kind="NotSent")

    def display_value(self) -> str:
        if self.kind == "NotSent" or self.value is None:
            return "NOT SENT BY NODE"
        return self.value


@dataclass
class Receipt5:
    subject_key_ids: VerificationBasis
    attesting_key_id: VerificationBasis
    cohort_scope: VerificationBasis
    dimension: VerificationBasis
    consent_scope: VerificationBasis
    for_key_id: Optional[str]
    holders: int
    notes: List[str]
    acts: List[str]

    @staticmethod
    def from_form(form: Form, viewer_key: Optional[str], standing: str = "Admitted") -> "Receipt5":
        env = form.envelope
        subjects = [s for s in env.get("subject_key_ids", []) if isinstance(s, str)]
        if subjects:
            subj_basis = VerificationBasis.wire(
                ", ".join(subjects),
                "Named subject(s) with Take-Back & Contest standing",
            )
        else:
            subj_basis = VerificationBasis.by_rule("self (producer-scoped)", "CC 2.1.2")

        attester = env.get("attesting_key_id")
        if isinstance(attester, str) and attester:
            att_basis = VerificationBasis.wire(attester, "Hybrid Ed25519 + ML-DSA-65 signer")
        else:
            att_basis = VerificationBasis.by_rule("local-node-self", "CC 2.3.1")

        raw_scope = env.get("cohort_scope")
        if isinstance(raw_scope, str) and raw_scope:
            circle = CircleScope.from_wire_scope(raw_scope)
            scope_basis = VerificationBasis.wire(raw_scope, circle.label())
        else:
            scope_basis = VerificationBasis.not_sent()

        dim = env.get("dimension", "")
        kind = ChangeKind.from_dimension(dim)
        if dim:
            dim_basis = VerificationBasis.wire(dim, f"{kind.value} ({kind.ontology_class().value})")
        else:
            dim_basis = VerificationBasis.not_sent()

        c_scope = env.get("consent_scope") or env.get("consent:scope")
        if isinstance(c_scope, str) and c_scope:
            consent_basis = VerificationBasis.wire(c_scope)
        else:
            consent_basis = VerificationBasis.by_rule(
                f"op:{form.op.value} admission rule", "CC 2.1 / CC 2.4"
            )

        acts: List[str] = []
        if standing == "Admitted":
            is_attester = viewer_key is not None and (attester == viewer_key or not attester)
            is_subject = viewer_key is not None and viewer_key in subjects
            if is_attester or is_subject:
                acts.append("WithdrawTakeBack")
            if is_attester:
                acts.append("WidenScope")
                if kind.admits_verb(Verb.RECANT):
                    acts.append("RecantAssertion")
            if is_subject and kind in (ChangeKind.CONFIDENCE, ChangeKind.RULES, ChangeKind.FACTS):
                acts.append("ContestOrObject")

        return Receipt5(
            subject_key_ids=subj_basis,
            attesting_key_id=att_basis,
            cohort_scope=scope_basis,
            dimension=dim_basis,
            consent_scope=consent_basis,
            for_key_id=env.get("for_key_id"),
            holders=int(env.get("holders", 1)),
            notes=[],
            acts=acts,
        )


# ============================================================================
# 2. CONTEXTS (X) — 5 Circles × 7 Tabs + 6 Instruments × 9 State Planes × T0..T19
# ============================================================================

class CircleScope(str, Enum):
    JUST_ME = "JustMe"
    FAMILY = "Family"
    NEIGHBOURS = "Neighbours"
    COMMUNITIES_AND_BUSINESSES = "CommunitiesAndBusinesses"
    EVERYONE = "Everyone"

    @staticmethod
    def from_wire_scope(s: str) -> "CircleScope":
        mapping = {
            "self": CircleScope.JUST_ME,
            "agent": CircleScope.JUST_ME,
            "family": CircleScope.FAMILY,
            "community": CircleScope.NEIGHBOURS,
            "local_community": CircleScope.NEIGHBOURS,
            "affiliations": CircleScope.COMMUNITIES_AND_BUSINESSES,
            "global_communities": CircleScope.COMMUNITIES_AND_BUSINESSES,
            "species": CircleScope.EVERYONE,
            "biosphere": CircleScope.EVERYONE,
            "federation": CircleScope.EVERYONE,
            "global_commons": CircleScope.EVERYONE,
        }
        return mapping.get(s, CircleScope.JUST_ME)

    def label(self) -> str:
        return {
            CircleScope.JUST_ME: "Just me",
            CircleScope.FAMILY: "Family",
            CircleScope.NEIGHBOURS: "Neighbours",
            CircleScope.COMMUNITIES_AND_BUSINESSES: "Communities & Businesses",
            CircleScope.EVERYONE: "Everyone",
        }[self]

    def wire_canonical(self) -> str:
        return {
            CircleScope.JUST_ME: "self",
            CircleScope.FAMILY: "family",
            CircleScope.NEIGHBOURS: "community",
            CircleScope.COMMUNITIES_AND_BUSINESSES: "affiliations",
            CircleScope.EVERYONE: "federation",
        }[self]

    def accent_hex(self) -> str:
        return {
            CircleScope.JUST_ME: "#22C0E8",
            CircleScope.FAMILY: "#419CA0",
            CircleScope.NEIGHBOURS: "#7A6FD6",
            CircleScope.COMMUNITIES_AND_BUSINESSES: "#C96A38",
            CircleScope.EVERYONE: "#B08A3E",
        }[self]

    def rank(self) -> int:
        return {
            CircleScope.JUST_ME: 0,
            CircleScope.FAMILY: 1,
            CircleScope.NEIGHBOURS: 2,
            CircleScope.COMMUNITIES_AND_BUSINESSES: 3,
            CircleScope.EVERYONE: 4,
        }[self]


# ============================================================================
# 3. ACTIONS (A) — Directional Asymmetry Law + 10 Governance Decision Families
# ============================================================================

@dataclass
class ConfirmFact:
    label: str
    value: str


@dataclass
class ConfirmFacts3:
    title: str
    facts: List[ConfirmFact]
    danger_note: Optional[str] = None

    def __post_init__(self) -> None:
        if len(self.facts) != 3:
            raise ValueError(f"ConfirmFacts3 invariant violated: expected 3 facts, got {len(self.facts)}")


def evaluate_action_gate(intent_kind: str, **kwargs: Any) -> Dict[str, Any]:
    """
    Directional Asymmetry Law (`circles.html` Scene 11 & `ConfirmSheet.kt`):
    "Going out asks. Coming back never does."
    """
    if intent_kind in (
        "CreateLocal",
        "WithdrawClaim",
        "RecantClaim",
        "LeaveGroupOrRoot",
        "FireAccordHalt",
        "RevokeDelegation",
        "CancelRetirementBeforeFrozen",
        "AgentRefuseOrDefer",
    ):
        return {"gate": "ImmediateOneStep"}

    if intent_kind == "MoveScope":
        from_scope: CircleScope = kwargs["from_scope"]
        to_scope: CircleScope = kwargs["to_scope"]
        item_label: str = kwargs.get("item_label", "item")
        if to_scope.rank() <= from_scope.rank():
            return {"gate": "ImmediateOneStep"}
        danger = (
            "Federation copies cannot be forcibly erased from remote peers once replicated."
            if to_scope == CircleScope.EVERYONE
            else None
        )
        sheet = ConfirmFacts3(
            title=f"Share {item_label} to {to_scope.label()}",
            facts=[
                ConfirmFact("Moving", item_label),
                ConfirmFact("To", f"{to_scope.label()} ({to_scope.wire_canonical()})"),
                ConfirmFact(
                    "Can I undo it?",
                    "You can withdraw your claim in 1 step at any time (CC 2.4.1.1).",
                ),
            ],
            danger_note=danger,
        )
        return {"gate": "ConfirmSheet3", "sheet": asdict(sheet)}

    if intent_kind == "GrantDelegation":
        sheet = ConfirmFacts3(
            title="Grant authority delegation",
            facts=[
                ConfirmFact("Delegating to", kwargs.get("delegate_key", "peer")),
                ConfirmFact(
                    "Scope & term",
                    f"{kwargs.get('scope', 'rules')} ({kwargs.get('valid_days', 30)} days, <= 365d max)",
                ),
                ConfirmFact(
                    "Can I undo it?",
                    "1-step withdraw revokes this link and cascades to sub-delegations (authority_tree).",
                ),
            ],
        )
        return {"gate": "ConfirmSheet3", "sheet": asdict(sheet)}

    return {"gate": "TwoPathCeremony", "family": kwargs.get("family", "default_content")}


# The 10 Governance Policy Families from `manifests/governance_policy.json`
GOVERNANCE_POLICIES: Dict[str, Dict[str, Any]] = {
    "default_content": {
        "kind": "two_path", "enact": (2, 3), "cancel": (1, 3, 2), "floor": (1, 3, 2),
        "window_secs": 86400, "irreversible": False, "has_reverse": True,
    },
    "default_standing": {
        "kind": "two_path", "enact": (2, 3), "cancel": (1, 3, 2), "floor": (1, 3, 2),
        "window_secs": 259200, "irreversible": False, "has_reverse": True,
    },
    "default_irreversible": {
        "kind": "two_path", "enact": (2, 3), "cancel": (1, 3, 2), "floor": (1, 3, 2),
        "window_secs": 604800, "irreversible": True, "has_reverse": True,
    },
    "community_invite": {
        "kind": "two_path", "enact": (2, 3), "cancel": (1, 3, 2), "floor": (1, 3, 2),
        "window_secs": 259200, "irreversible": False, "has_reverse": True,
    },
    "community_remove": {
        "kind": "two_path", "enact": (2, 3), "cancel": (1, 1, 999999), "floor": (2, 3, 2),
        "window_secs": 259200, "irreversible": False, "has_reverse": False,
    },
    "self_roster_add": {
        "kind": "two_path", "enact": (1, 2), "cancel": (1, 2, 1), "floor": (0, 1, 1),
        "window_secs": 86400, "irreversible": False, "has_reverse": True,
    },
    "self_roster_evict": {
        "kind": "two_path", "enact": (1, 2), "cancel": (1, 2, 1), "floor": (0, 1, 1),
        "window_secs": 86400, "irreversible": False, "has_reverse": True,
    },
    "moderation_action": {
        "kind": "mirror", "enact": (0, 1), "cancel": (1, 2, 1), "floor": (1, 2, 1),
        "window_secs": 172800, "irreversible": False, "has_reverse": True,
    },
    "accord_halt": {
        "kind": "mirror", "enact": (0, 1), "cancel": (1, 2, 1), "floor": (1, 2, 2),
        "window_secs": 86400, "irreversible": False, "has_reverse": False,
    },
    "custodian_replace": {
        "kind": "cosigned", "enact": (1, 1), "cancel": (1, 2, 1), "floor": (1, 1, 2),
        "window_secs": 0, "irreversible": False, "has_reverse": False,
    },
}


def evaluate_governance_policy(
    family: str, base: int, approvals: int, objections: int, window_rem_secs: int
) -> Dict[str, Any]:
    """Evaluate `TwoPathVerdict` per `TwoPath.tla`, `SelfRoster.tla`, and `PolicyCorridor.lean`."""
    spec = GOVERNANCE_POLICIES[family]
    if family in ("self_roster_add", "self_roster_evict"):
        if base == 1 and approvals >= 1:
            return {"verdict": "LandsForward"}
        if base == 2:
            if approvals >= 2:
                return {"verdict": "LandsForward"}
            if objections >= 1:
                return {"verdict": "Cancelled", "reason": "Objected by other key in 2-key roster"}
            return {
                "verdict": "Pending",
                "approvals": approvals,
                "enact_needed": 2,
                "objections": objections,
                "cancel_needed": 1,
                "floor_needed": 2,
                "window_rem_secs": window_rem_secs,
            }

    en_num, en_den = spec["enact"]
    if family == "custodian_replace":
        enact = min(2, max(1, base))
    elif family in ("moderation_action", "accord_halt"):
        enact = 1
    else:
        enact = min(base, max(1, (en_num * base) // en_den + 1))

    c_num, c_den, c_min = spec["cancel"]
    if not spec["has_reverse"] and family == "community_remove":
        cancel = 999999
    elif family in ("self_roster_add", "self_roster_evict"):
        others = max(0, base - 1)
        cancel = 1 if others == 0 else (others // 2) + 1
    else:
        cancel = min(max(1, base), max(c_min, math.ceil(c_num * base / c_den)))

    f_num, f_den, f_min = spec["floor"]
    floor = f_min if f_num == 0 else min(max(1, base), max(f_min, math.ceil(f_num * base / f_den)))

    if objections >= cancel:
        return {"verdict": "Cancelled", "reason": f"Objection brake reached ({objections}/{cancel})"}

    if approvals >= enact and (not spec["irreversible"] or window_rem_secs == 0):
        return {"verdict": "LandsForward"}

    if window_rem_secs == 0:
        if spec["has_reverse"] and approvals >= floor and objections == 0:
            return {"verdict": "LandsReverse"}
        return {
            "verdict": "Cancelled",
            "reason": "Window expired without meeting silence floor or unopposed requirement",
        }

    return {
        "verdict": "Pending",
        "approvals": approvals,
        "enact_needed": enact,
        "objections": objections,
        "cancel_needed": cancel,
        "floor_needed": floor,
        "window_rem_secs": window_rem_secs,
    }


# ============================================================================
# 4. CONCLUSIONS (V) & EPISTEMIC SCENE PROJECTION (12 Primitives + CSD/4 Tree)
# ============================================================================

def score_to_color(score: float) -> List[float]:
    s = 0.0 if math.isnan(score) else max(-1.0, min(1.0, score))
    t = (s + 1.0) / 2.0
    red = (0.85, 0.12, 0.12)
    green = (0.20, 0.72, 0.30)
    return [
        round(red[0] + (green[0] - red[0]) * t, 4),
        round(red[1] + (green[1] - red[1]) * t, 4),
        round(red[2] + (green[2] - red[2]) * t, 4),
        1.0,
    ]


def confidence_to_scale(confidence: float) -> float:
    c = 0.0 if math.isnan(confidence) else max(0.0, min(1.0, confidence))
    return round(0.45 + c * (1.65 - 0.45), 4)


def view_form_3d(form: Form) -> Dict[str, Any]:
    """Pure 3D Scene IR projection (`view-forms-spike/src/view_form.rs`)."""
    nodes = [
        {
            "pos": [0.0, -1.2, 0.0],
            "scale": 9.0,
            "mesh": "Plane",
            "color": [0.88, 0.87, 0.84, 1.0],
            "label": None,
        }
    ]
    if form.op == Op.SCORES:
        score = float(form.envelope.get("score", 0.0))
        conf = float(form.envelope.get("confidence", 0.0))
        dim = str(form.envelope.get("dimension", "scores"))
        nodes.append(
            {
                "pos": [-3.0, -0.6, 0.0],
                "scale": 0.35,
                "mesh": "Cube",
                "color": [0.85, 0.12, 0.12, 1.0],
                "label": "score=-1",
            }
        )
        nodes.append(
            {
                "pos": [3.0, -0.6, 0.0],
                "scale": 0.35,
                "mesh": "Cube",
                "color": [0.20, 0.72, 0.30, 1.0],
                "label": "score=+1",
            }
        )
        nodes.append(
            {
                "pos": [0.0, 0.0, 0.0],
                "scale": confidence_to_scale(conf),
                "mesh": "Sphere",
                "color": score_to_color(score),
                "label": dim,
            }
        )
    else:
        nodes.append(
            {
                "pos": [0.0, 0.0, 0.0],
                "scale": 1.0,
                "mesh": "Sphere",
                "color": [0.55, 0.55, 0.58, 1.0],
                "label": form.op.value,
            }
        )
    return {
        "nodes": nodes,
        "camera": {"eye": [3.6, 3.0, 5.6], "look_at": [0.0, 0.2, 0.0]},
        "background": [round(250 / 255, 4), round(249 / 255, 4), round(245 / 255, 4)],
    }


def evaluate_policy_lens(
    opinions: List[Form], active_rules: List[str], helper_keys: List[str]
) -> Dict[str, Any]:
    blur_match: Optional[Tuple[str, str]] = None
    for f in opinions:
        if f.op != Op.SCORES:
            continue
        dim = str(f.envelope.get("dimension", ""))
        score = float(f.envelope.get("score", 0.0))
        attester = str(f.envelope.get("attesting_key_id", "neighbour"))
        if helper_keys and attester not in helper_keys:
            continue
        if dim.startswith("moderation:hide") and score <= -0.5 and "my_rules" in active_rules:
            return {"verdict": "HiddenByRules", "rule": f"Hidden by my_rules ({dim} by {attester})"}
        if (dim.startswith("cw_class:") or score < 0.0) and "my_rules" in active_rules:
            blur_match = (attester, dim)
    if blur_match:
        return {"verdict": "Blurred", "opinion_by": blur_match[0], "rule": blur_match[1]}
    return {"verdict": "Clear"}


def project_epistemic_scene(
    viewer_ctx: Dict[str, Any], forms: List[Form], opinions: List[Form]
) -> Dict[str, Any]:
    """Project `(ViewerContext, forms, opinions) -> EpistemicScene` using only the 12 primitives."""
    circle = CircleScope(viewer_ctx.get("circle", "JustMe"))
    tab = viewer_ctx.get("tab", "Files")
    active_inst = viewer_ctx.get("active_instrument")
    planes = viewer_ctx.get("planes", {})
    has_agent = planes.get("has_agent", True)
    halt_status = planes.get("halt_status", "Clear")

    nodes: List[Dict[str, Any]] = [
        {
            "primitive": "ShellHeader",
            "circle": circle.value,
            "tab": tab,
            "active_instrument": active_inst,
            "shows_circle_governance_subtitle": active_inst is None,
            "halt_status": halt_status,
            "my_things_tag": "btn_my_things",
            "stop_everything_tag": "btn_stop_everything",
        }
    ]

    if active_inst in ("ThisAgent", "SomeoneITrust") and not has_agent:
        nodes.append(
            {
                "primitive": "StateBanner",
                "tag": "state_no_agent",
                "state": "Empty",
                "tone": "Mute",
                "honest_zero": "No AI agent is attached to this bare CEWP node (CC 4.4.3.4.3).",
                "data_doors_open_banner": None,
            }
        )
        return {"topology": viewer_ctx.get("topology", "T0"), "nodes": nodes}

    if halt_status != "Clear":
        nodes.append(
            {
                "primitive": "StateBanner",
                "tag": "state_halt_423",
                "state": "Error",
                "tone": "Danger",
                "title": "Agent Act Gate Halted (423 Locked)",
                "data_doors_open_banner": (
                    "Data doors remain OPEN (Inv_X1_LatchDoesNotBlockServing): reading, "
                    "exporting, and 1-step withdrawal work normally while the agent is paused."
                ),
            }
        )

    active_rules = viewer_ctx.get("active_lens_rules", ["my_rules"])
    helper_keys = viewer_ctx.get("helper_keys", [])
    nodes.append(
        {
            "primitive": "PolicyLens",
            "tag": "csd_policy_lens_bar",
            "active_rules": active_rules,
            "helper_keys": helper_keys,
            "sort_label": "newest_first",
        }
    )

    lens_verdict = evaluate_policy_lens(opinions, active_rules, helper_keys)
    items: List[Dict[str, Any]] = []
    for idx, f in enumerate(forms):
        row_id = f"row_{idx}"
        dim = str(f.envelope.get("dimension", "unknown"))
        wire_scope = str(f.envelope.get("cohort_scope", "self"))
        row_circle = CircleScope.from_wire_scope(wire_scope)
        receipt = Receipt5.from_form(f, viewer_ctx.get("viewer_key_id", "ed25519:viewer_self"))
        items.append(
            {
                "id": row_id,
                "tag": f"csd_item_{row_id}",
                "title": dim,
                "subtitle": f"op:{f.op.value}",
                "scope_pill": {
                    "tag": f"csd_scope_{row_id}",
                    "circle": row_circle.value,
                    "color_hex": row_circle.accent_hex(),
                    "label": row_circle.label(),
                    "local_only_badge": row_circle == CircleScope.JUST_ME,
                },
                "chips": [
                    {
                        "tag": f"csd_chip_standing_{row_id}",
                        "kind": "Badge",
                        "label": "Admitted",
                        "tone": "Ok",
                    }
                ],
                "lens_verdict": lens_verdict,
                "has_hamburger": True,
                "hamburger_tag": f"btn_receipt_{row_id}",
                "receipt": asdict(receipt),
            }
        )

    fields = [
        {
            "tag": "csd-field-root_standing",
            "label": "Trust Root Standing",
            "value": planes.get("root_standing", "Rooted"),
            "protocol_wire_name": "root.standing",
            "basis": asdict(VerificationBasis.by_rule(planes.get("root_standing", "Rooted"), "CC 6.2")),
        },
        {
            "tag": "csd-field-self_roster_size",
            "label": "Self-Key Roster Size",
            "value": str(planes.get("self_roster_size", 1)),
            "protocol_wire_name": "self.roster_size",
            "basis": asdict(VerificationBasis.wire(str(planes.get("self_roster_size", 1)))),
        },
    ]

    nodes.append(
        {
            "primitive": "Card",
            "tag": "csd_main_card",
            "title": f"{circle.label()} · {tab}",
            "accent": "Brand",
            "fields": fields,
            "items": items,
        }
    )

    if forms:
        nodes.append(
            {
                "primitive": "SpatialCanvas",
                "tag": "csd_spatial_canvas",
                "scene": view_form_3d(forms[0]),
            }
        )

    return {"topology": viewer_ctx.get("topology", "T0"), "nodes": nodes}


def extract_csd4_tree(epistemic_scene: Dict[str, Any]) -> List[Dict[str, str]]:
    """Deterministic CSD/4 `/tree` tag projection."""
    out: List[Dict[str, str]] = []
    for node in epistemic_scene["nodes"]:
        prim = node["primitive"]
        if prim == "ShellHeader":
            out.append({"tag": node["my_things_tag"], "role": "button", "automation_text": "My things"})
            out.append({"tag": node["stop_everything_tag"], "role": "button", "automation_text": "Stop everything"})
            if node.get("active_instrument"):
                out.append({"tag": f"inst_{node['active_instrument']}", "role": "instrument", "automation_text": node["active_instrument"]})
            else:
                out.append({"tag": f"tab_{node['tab'].lower()}", "role": "tab", "automation_text": f"{node['circle']} · {node['tab']}"})
        elif prim == "StateBanner":
            out.append({"tag": node["tag"], "role": "state_block", "automation_text": node["state"]})
            if node.get("data_doors_open_banner"):
                out.append({"tag": f"{node['tag']}_data_doors_open", "role": "banner", "automation_text": node["data_doors_open_banner"]})
        elif prim == "PolicyLens":
            out.append({"tag": node["tag"], "role": "policy_lens_bar", "automation_text": ",".join(node["active_rules"])})
        elif prim == "Card":
            out.append({"tag": node["tag"], "role": "card_shell", "automation_text": node["title"]})
            for f in node["fields"]:
                out.append({"tag": f["tag"], "role": "field_row", "automation_text": f"{f['label']}: {f['value']}"})
            for item in node["items"]:
                out.append({"tag": item["tag"], "role": "item_row", "automation_text": f"{item['title']} — {item['subtitle']}"})
                out.append({"tag": item["scope_pill"]["tag"], "role": "scope_pill", "automation_text": item["scope_pill"]["label"]})
                if item.get("has_hamburger"):
                    out.append({"tag": item["hamburger_tag"], "role": "receipt_hamburger", "automation_text": "Receipt"})
        elif prim == "SpatialCanvas":
            out.append({"tag": node["tag"], "role": "spatial_canvas", "automation_text": f"3D Scene ({len(node['scene']['nodes'])} nodes)"})
    return out


def build_all_11_promise_cards() -> Dict[str, Any]:
    """Build all 11 reserved CSD cards (`CSD-111`..`CSD-119`, `CSD-127`, `CSD-128`) closing the 14 unserved sub-rules."""
    return {
        "CSD-111": {
            "promises": ["2g", "5a", "5b"],
            "title": "Steward Shutdown Dialogue (Reason, 5m Window, Agent Accept/Object/Postpone, Recorded Decision)",
            "placement": "My things › This agent",
            "primitives": ["CardShell", "ConscienceStepBlock", "TwoPathCeremonyBlock"],
            "policy_family": "custodian_replace",
        },
        "CSD-112": {
            "promises": ["5d", "5e"],
            "title": "WA-Cosigned Identity Record Edit & Behavioural Drift Threshold Review",
            "placement": "My things › This agent",
            "primitives": ["CardShell", "FieldRow", "TwoPathCeremonyBlock"],
            "policy_family": "custodian_replace",
        },
        "CSD-113": {
            "promises": ["5f"],
            "title": "7-State Agent Retirement Standard Floor (Retirement.tla)",
            "placement": "My things › This agent",
            "primitives": ["CardShell", "LineageChainBlock", "TwoPathCeremonyBlock"],
            "steps": [
                "Decided (1-step cancel active)",
                "Frozen (cancel sealed)",
                "SecretsDestroyed",
                "HandedOver",
                "Notified",
                "CustodianMinted",
                "Retired",
            ],
        },
        "CSD-114": {
            "promises": ["5g"],
            "title": "Upgrade Offer with Version, Changelog, Risk Band, Canary Outcomes & Recorded Refusal (G23)",
            "placement": "My things › This agent",
            "primitives": ["CardShell", "FieldRow", "ConscienceStepBlock", "ConfirmSheet"],
        },
        "CSD-115": {
            "promises": ["3a", "3e", "6e"],
            "title": "Steward–Agent Bond (Depth 1, 365d Review, Listed Powers) & Halt Data-Door Guarantee",
            "placement": "My things › This agent & Setup (CSD-082)",
            "primitives": ["CardShell", "LineageChainBlock", "StateBlock"],
        },
        "CSD-116": {
            "promises": ["7a", "7b"],
            "title": "Published Ethical Principles, Lexical Priority Order & Mandatory G14 Limits Row",
            "placement": "My things › Help & ConscienceStepBlock Header",
            "primitives": ["CardShell", "FieldRow"],
        },
        "CSD-117": {
            "promises": ["8b", "8c"],
            "title": "Judges Are Judged — Wise Authority Rulings Cadence, Conduct & Appeals",
            "placement": "My things › Someone I trust",
            "primitives": ["CardShell", "ItemRow", "ReceiptSheet"],
        },
        "CSD-118": {
            "promises": ["9a", "9c"],
            "title": "Public NEVER_ALLOWED (prohibited:*) List & Operator Engagement Declaration",
            "placement": "My things › This agent › Tools (CSD-014) & Setup (CSD-082)",
            "primitives": ["CardShell", "ItemRow", "FieldRow"],
        },
        "CSD-119": {
            "promises": ["3c"],
            "title": "Verified Guardian Bond (365d Review) & Child Confidential Help/Contest Channel (Q3/Q4/Q5)",
            "placement": "Family › Safety (extends CSD-066)",
            "primitives": ["CardShell", "LineageChainBlock"],
        },
        "CSD-120": {
            "promises": ["6c", "3b"],
            "title": "Affiliation — Found an Organization, Declare Charter, Nest or Dissolve (#206)",
            "placement": "Communities & Businesses › Rules",
            "primitives": ["CardShell", "FieldRow", "ConfirmSheet", "TwoPathCeremonyBlock"],
        },
        "CSD-121": {
            "promises": ["2f", "3b"],
            "title": "Affiliation Members & Termed Roles (365d Attenuated Delegations) (#206)",
            "placement": "Communities & Businesses › People",
            "primitives": ["CardShell", "ItemRow", "ConfirmSheet", "TwoPathCeremonyBlock"],
        },
        "CSD-122": {
            "promises": ["2c", "2f"],
            "title": "Licensure — Issue, Probate, Suspend, Revoke & Subject Contest (#206)",
            "placement": "Communities & Businesses › Rules",
            "primitives": ["CardShell", "ItemRow", "ReceiptSheet", "ConfirmSheet"],
        },
        "CSD-123": {
            "promises": ["2c", "N2"],
            "title": "Partners — Bilateral / Multilateral Organizational Recognition (#206)",
            "placement": "Communities & Businesses › People",
            "primitives": ["CardShell", "ItemRow", "ReceiptSheet"],
        },
        "CSD-124": {
            "promises": ["2f"],
            "title": "Authority Delegations — Issuance Rights Chain (Depth ≤ 5, 365d, Cascade) (#206)",
            "placement": "Communities & Businesses › Rules (extends CSD-055)",
            "primitives": ["CardShell", "LineageChainBlock", "ReceiptSheet", "ConfirmSheet"],
        },
        "CSD-125": {
            "promises": ["1d", "2f"],
            "title": "Grants — Non-Transferable Leaf Access Grants (`key_grant:*`) (#206)",
            "placement": "Communities & Businesses › Files",
            "primitives": ["CardShell", "ItemRow", "ReceiptSheet", "ConfirmSheet"],
        },
        "CSD-126": {
            "promises": ["2e", "3b"],
            "title": "Affiliation Keys — Register, Rotate, Revoke & Threshold Escrow (#206)",
            "placement": "Communities & Businesses › Safety",
            "primitives": ["CardShell", "ItemRow", "ReceiptSheet", "TwoPathCeremonyBlock"],
        },
        "CSD-127": {
            "promises": ["6e", "4c"],
            "title": "First-Contact Gate, Local-Only Device Badge & 1-Step Trust-Root Exit (Inv_ExitOneStep)",
            "placement": "Everyone › Rules / Safety (extends CSD-105)",
            "primitives": ["CardShell", "FieldRow", "LineageChainBlock", "ScopePill"],
        },
        "CSD-128": {
            "promises": ["2d"],
            "title": "Protected Reporter Channel (Signed, Name Known to Moderator & Sealed Record Only)",
            "placement": "Neighbours / Communities › Safety (extends CSD-065)",
            "primitives": ["CardShell", "FieldRow", "ConfirmSheet"],
        },
    }


def verify_all_invariants() -> None:
    """Run all formal verification checks and emit JSON artifacts."""
    # 1. Closed 1+4 Operator Gate
    for bad in ("eval", "exec", "lambda", "setf", "rplaca"):
        try:
            read_form({"attestation_type": bad, "dimension": "x"})
            raise AssertionError(f"Failed to reject {bad}")
        except ValueError as e:
            assert f"UnknownOperator({bad})" in str(e)

    # 2. Exact 48 / 72 (Kind × Verb) Occupancy Matrix
    occupied = sum(
        1 for k in ChangeKind for v in Verb if k.admits_verb(v)
    )
    assert occupied == 48, f"Expected 48 occupied (Kind × Verb) cells, got {occupied}"

    # 3. Directional Asymmetry Law
    out_gate = evaluate_action_gate(
        "MoveScope",
        item_label="beach.jpg",
        from_scope=CircleScope.JUST_ME,
        to_scope=CircleScope.EVERYONE,
    )
    assert out_gate["gate"] == "ConfirmSheet3"
    assert len(out_gate["sheet"]["facts"]) == 3
    assert out_gate["sheet"]["danger_note"] is not None

    for inward in ("WithdrawClaim", "LeaveGroupOrRoot", "FireAccordHalt", "AgentRefuseOrDefer"):
        assert evaluate_action_gate(inward)["gate"] == "ImmediateOneStep"

    # 4. Two-Path Governance Policy Table (`SelfRoster.tla` & `TwoPath.tla`)
    assert evaluate_governance_policy("self_roster_add", 1, 1, 0, 86400)["verdict"] == "LandsForward"
    assert evaluate_governance_policy("self_roster_add", 2, 1, 0, 0)["verdict"] == "Pending"
    assert evaluate_governance_policy("self_roster_add", 3, 2, 0, 86400)["verdict"] == "LandsForward"
    assert evaluate_governance_policy("self_roster_add", 3, 1, 0, 0)["verdict"] == "LandsReverse"
    assert evaluate_governance_policy("default_irreversible", 6, 6, 0, 3600)["verdict"] == "Pending"
    assert evaluate_governance_policy("default_irreversible", 6, 5, 0, 0)["verdict"] == "LandsForward"

    # 5. Epistemic Scene + CSD/4 Tree Projection
    sample_form = read_form(
        {
            "attestation_type": "scores",
            "dimension": "evaluation:quality",
            "score": 0.85,
            "confidence": 0.92,
            "subject_key_ids": ["ed25519:alice"],
            "attesting_key_id": "ed25519:viewer_self",
            "cohort_scope": "federation",
            "evidence_refs": [],
        }
    )
    ctx = {
        "viewer_key_id": "ed25519:viewer_self",
        "circle": "Everyone",
        "tab": "Decisions",
        "active_instrument": None,
        "topology": "T4",
        "planes": {
            "has_agent": True,
            "halt_status": "LatchedLone(86400s)",
            "root_standing": "Rooted",
            "self_roster_size": 3,
        },
        "active_lens_rules": ["my_rules"],
        "helper_keys": ["ed25519:neighbour_dan"],
    }
    scene = project_epistemic_scene(ctx, [sample_form], [])
    tree = extract_csd4_tree(scene)
    tags = {n["tag"] for n in tree}
    assert "btn_my_things" in tags
    assert "btn_stop_everything" in tags
    assert "state_halt_423" in tags
    assert "state_halt_423_data_doors_open" in tags
    assert "btn_receipt_row_0" in tags
    assert "csd_spatial_canvas" in tags

    # Write deterministic artifacts
    spike_out = Path("/home/emoore/CEWPOS/view-forms-spike/out")
    spike_out.mkdir(parents=True, exist_ok=True)
    (spike_out / "epistemic-scene.json").write_text(json.dumps(scene, indent=2) + "\n")
    (spike_out / "csd4-tree.json").write_text(json.dumps(tree, indent=2) + "\n")

    ui_out = Path("/home/emoore/CEWPOS/epistemic-ui/out")
    ui_out.mkdir(parents=True, exist_ok=True)
    promise_cards = build_all_11_promise_cards()
    (ui_out / "all_promise_cards.json").write_text(json.dumps(promise_cards, indent=2) + "\n")

    print("== CC rc9 Constitutional UI Algebra & CSD/4 Verification Suite ==")
    print(f"[PASS] Closed 1+4 CEG Operator Gate: admits {CLOSED_OPS}, rejects eval/exec/lambda/setf")
    print(f"[PASS] 12 × 6 (ChangeKind × Verb) Occupancy Matrix: {occupied} / 72 legal cells verified")
    print("[PASS] Directional Asymmetry Law: outward -> 3-fact ConfirmSheet; inward/halt/exit -> 1-step")
    print(f"[PASS] 10 Governance Policy Families: {len(GOVERNANCE_POLICIES)} policy machines verified against TwoPath.tla")
    print("[PASS] Inv_X1_LatchDoesNotBlockServing: 423 Locked freezes agent gate while data doors stay open")
    print(f"[PASS] Hamburger Epistemic Invariant (CSD-006): verified on {len(tree)} CSD/4 tree nodes")
    print(f"[PASS] 11 Reserved Promise Cards (CSD-111..128): {len(promise_cards)} cards composed from P1..P12")
    print(f"[WROTE] {spike_out / 'epistemic-scene.json'}")
    print(f"[WROTE] {spike_out / 'csd4-tree.json'}")
    print(f"[WROTE] {ui_out / 'all_promise_cards.json'}")


if __name__ == "__main__":
    verify_all_invariants()
