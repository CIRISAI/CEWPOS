"""Minimal `check_csd_v3` block regex provider for `check_csd4_upstream.py`."""
import re

BLOCK = re.compile(r"^```yaml csd:([a-z0-9_-]+)\s*\n(.*?)^```", re.M | re.S)
