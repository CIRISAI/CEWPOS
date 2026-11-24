#!/usr/bin/env python3
"""Regional Architecture Model Pipeline (Part 2 of Unified Regional Buildings).

Maps `(region, koppen_climate, lcz_density, floors, units, w, d)` to `holon-draw`
building parameters:
  - `roof_shape` (0..6): 0=flat, 1=gabled, 2=hipped, 3=skillion, 4=mansard, 5=pyramidal, 6=half_hipped
  - `roof_tone`  (1..7): 1=RedTile, 2=BrownTile, 3=WeatheredWood, 4=GreySlate, 5=DarkShingle, 6=MetalLight, 7=ConcreteFlat
  - `wall_rgb`   ([r, g, b] 0..255)
  - `eave_ratio` (float, roof overhang relative to half-span)
  - `porch_prob` (float in 0..1)
  - `tree_biome` (0..4): 0=broadleaf, 1=conifer_mixed, 2=tropical_palm, 3=desert_scrub, 4=subtropical_lush

Modes:
  --calibrate               Offline calibration using empirical architectural priors
                            across Koppen-Geiger climate bands & WUDAPT LCZ 1..10
                            for 6 reference regions (zero external Python deps).
  --overture <parquet_glob> DuckDB SQL ingestion joining Overture Maps Buildings
                            (`roof_shape`, `roof_color`, `facade_material`, `num_floors`)
                            with Koppen-Geiger & WUDAPT LCZ grids.

Outputs (in `--out-dir`, default alongside this script):
  1. regional_priors.json   Compact quantized lookup table + decision trees
  2. regional_model.rs      Zero-dependency pure Rust module for holon-buildings / wasm32
  3. regional_model.js      Zero-dependency ES module for CEWPOS/docs/index.html & CEWP
"""

from __future__ import annotations

import argparse
import json
import math
import os
import random
import sys
from collections import Counter
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List, Optional, Sequence, Tuple

# ---------------------------------------------------------------------------
# Canonical Enums & Schemas (aligned with CEWP RoofTone & holon-draw Leaf)
# ---------------------------------------------------------------------------

ROOF_SHAPES: Tuple[str, ...] = (
    "flat",         # 0
    "gabled",       # 1
    "hipped",       # 2
    "skillion",     # 3
    "mansard",      # 4
    "pyramidal",    # 5
    "half_hipped",  # 6
)

ROOF_TONES: Dict[int, Tuple[str, Tuple[int, int, int]]] = {
    1: ("RedTile", (178, 98, 72)),
    2: ("BrownTile", (142, 122, 104)),
    3: ("WeatheredWood", (128, 112, 94)),
    4: ("GreySlate", (92, 100, 110)),
    5: ("DarkShingle", (84, 82, 80)),
    6: ("MetalLight", (168, 172, 176)),
    7: ("ConcreteFlat", (150, 150, 146)),
}

TREE_BIOMES: Tuple[str, ...] = (
    "broadleaf",        # 0
    "conifer_mixed",    # 1
    "tropical_palm",    # 2
    "desert_scrub",     # 3
    "subtropical_lush", # 4
)

KOPPEN_GROUPS: Tuple[str, ...] = (
    "tropical",       # 0: Af, Am, Aw
    "arid",           # 1: BWh, BWk, BSh, BSk
    "temperate",      # 2: Cfa, Cfb, Cfc, Cwa, Cwb, Cwc
    "mediterranean",  # 3: Csa, Csb, Csc
    "continental",    # 4: Dfa, Dfb, Dfc, Dfd, Dsa..Dwd, ET, EF
)

# CEWP builtform::CLIMATE_SYMBOLS order (1..30)
CLIMATE_SYMBOLS: Tuple[str, ...] = (
    "Af", "Am", "Aw", "BWh", "BWk", "BSh", "BSk", "Csa", "Csb", "Csc",
    "Cwa", "Cwb", "Cwc", "Cfa", "Cfb", "Cfc", "Dsa", "Dsb", "Dsc", "Dsd",
    "Dwa", "Dwb", "Dwc", "Dwd", "Dfa", "Dfb", "Dfc", "Dfd", "ET", "EF",
)

DENSITY_CLASSES: Tuple[str, ...] = (
    "village",       # 0: LCZ 9
    "suburban",      # 1: LCZ 6, 8, 10
    "terraced",      # 2: LCZ 3, 7
    "blocks",        # 3: LCZ 2, 4, 5
    "dense_fabric",  # 4: LCZ 1
)

# Exact holon-draw Leaf feature names
FEATURE_NAMES: Tuple[str, ...] = (
    "log_area",       # 0: ln(max(1.0, w * d))
    "aspect",         # 1: max(w, d) / max(0.5, min(w, d))
    "floors",         # 2: number of storeys (>= 1)
    "has_units",      # 3: 1 if units > 0 else 0
    "density_class",  # 4: 0..4 derived from WUDAPT LCZ 1..10
    "koppen_group",   # 5: 0..4
    "region_id",      # 6: 0..5
)
NOMINAL_FEATURES = {5, 6}  # koppen_group, region_id use equality splits


def koppen_to_group(symbol: str) -> int:
    s = symbol.strip()
    if s.isdigit():
        idx = int(s)
        if 1 <= idx <= len(CLIMATE_SYMBOLS):
            s = CLIMATE_SYMBOLS[idx - 1]
    sl = s.lower()
    if sl in ("tropical",) or sl.startswith(("af", "am", "aw")):
        return 0
    if sl in ("arid",) or sl.startswith(("bw", "bs")):
        return 1
    if sl in ("mediterranean",) or sl.startswith(("cs",)):
        return 3
    if sl in ("temperate",) or sl.startswith(("cf", "cw")):
        return 2
    if sl in ("continental", "boreal") or sl.startswith(("df", "ds", "dw", "et", "ef")):
        return 4
    return 2  # safe temperate default


def lcz_to_density_class(lcz: int) -> int:
    if lcz == 1:
        return 4
    if lcz in (2, 4, 5):
        return 3
    if lcz in (3, 7):
        return 2
    if lcz in (6, 8, 10):
        return 1
    if lcz == 9:
        return 0
    return max(0, min(4, lcz))


# ---------------------------------------------------------------------------
# 6 Reference Regions & Empirical Architectural Priors
# ---------------------------------------------------------------------------

@dataclass(frozen=True)
class RegionSpec:
    region_id: int
    key: str
    name: str
    label: str
    koppen_mix: Tuple[Tuple[str, float], ...]
    lcz_weights: Tuple[Tuple[int, float], ...]
    tree_biome: int
    eave_pitched: float
    eave_flat: float
    porch_base: float
    wall_palette: Tuple[Tuple[int, int, int], ...]
    ground_rgb: Tuple[int, int, int]
    tree_rgb: Tuple[int, int, int]


REGIONS: Tuple[RegionSpec, ...] = (
    RegionSpec(
        region_id=0,
        key="us_midwest",
        name="US Midwest",
        label="Ohio / Your Town",
        koppen_mix=(("Dfa", 0.65), ("Cfa", 0.35)),
        lcz_weights=((9, 0.14), (6, 0.44), (3, 0.14), (5, 0.08), (8, 0.08), (2, 0.06), (4, 0.04), (1, 0.02)),
        tree_biome=0,  # broadleaf
        eave_pitched=0.11,
        eave_flat=0.03,
        porch_base=0.72,
        wall_palette=((224, 214, 196), (166, 98, 82), (168, 180, 192), (178, 190, 172)),
        ground_rgb=(88, 132, 76),
        tree_rgb=(64, 118, 62),
    ),
    RegionSpec(
        region_id=1,
        key="eu_alpine",
        name="Central Europe / Alpine",
        label="Zurich / Central Europe",
        koppen_mix=(("Cfb", 0.62), ("Dfb", 0.28), ("Dfc", 0.10)),
        lcz_weights=((9, 0.12), (6, 0.26), (3, 0.22), (2, 0.18), (5, 0.12), (8, 0.06), (4, 0.03), (1, 0.01)),
        tree_biome=1,  # conifer_mixed
        eave_pitched=0.19,
        eave_flat=0.03,
        porch_base=0.18,
        wall_palette=((234, 224, 204), (222, 194, 146), (188, 158, 124), (210, 206, 198)),
        ground_rgb=(84, 126, 78),
        tree_rgb=(46, 96, 56),
    ),
    RegionSpec(
        region_id=2,
        key="jp_east_asia",
        name="East Asia",
        label="Tokyo / East Asia",
        koppen_mix=(("Cfa", 1.00),),
        lcz_weights=((3, 0.28), (2, 0.24), (1, 0.14), (4, 0.12), (5, 0.10), (6, 0.08), (8, 0.04)),
        tree_biome=4,  # subtropical_lush
        eave_pitched=0.13,
        eave_flat=0.02,
        porch_base=0.12,
        wall_palette=((206, 210, 214), (226, 224, 216), (138, 124, 110), (184, 188, 192)),
        ground_rgb=(92, 124, 84),
        tree_rgb=(58, 110, 68),
    ),
    RegionSpec(
        region_id=3,
        key="af_west_tropical",
        name="West Africa Tropical",
        label="Lagos / Tropical West Africa",
        koppen_mix=(("Aw", 0.72), ("Am", 0.18), ("Af", 0.10)),
        lcz_weights=((7, 0.24), (3, 0.24), (6, 0.22), (9, 0.10), (5, 0.08), (2, 0.06), (8, 0.04), (4, 0.02)),
        tree_biome=2,  # tropical_palm
        eave_pitched=0.26,
        eave_flat=0.04,
        porch_base=0.55,
        wall_palette=((220, 182, 128), (232, 218, 188), (206, 144, 108), (208, 198, 178)),
        ground_rgb=(104, 124, 72),
        tree_rgb=(54, 124, 58),
    ),
    RegionSpec(
        region_id=4,
        key="us_southwest_arid",
        name="Arid Southwest",
        label="Phoenix / Arid Southwest",
        koppen_mix=(("BWh", 0.80), ("BSh", 0.20)),
        lcz_weights=((6, 0.42), (9, 0.14), (8, 0.14), (3, 0.10), (5, 0.10), (2, 0.05), (4, 0.03), (1, 0.02)),
        tree_biome=3,  # desert_scrub
        eave_pitched=0.09,
        eave_flat=0.02,
        porch_base=0.32,
        wall_palette=((218, 188, 152), (228, 210, 178), (202, 138, 106), (234, 222, 200)),
        ground_rgb=(176, 152, 114),
        tree_rgb=(108, 122, 74),
    ),
    RegionSpec(
        region_id=5,
        key="sa_tropical",
        name="South America Subtropical/Tropical",
        label="Sao Paulo / South America",
        koppen_mix=(("Cfa", 0.65), ("Aw", 0.35)),
        lcz_weights=((2, 0.20), (3, 0.22), (4, 0.14), (1, 0.10), (5, 0.12), (6, 0.12), (7, 0.06), (8, 0.04)),
        tree_biome=4,  # subtropical_lush
        eave_pitched=0.15,
        eave_flat=0.03,
        porch_base=0.42,
        wall_palette=((232, 218, 192), (216, 162, 132), (194, 192, 188), (228, 204, 150)),
        ground_rgb=(94, 134, 74),
        tree_rgb=(50, 118, 56),
    ),
)

REGION_BY_KEY: Dict[str, RegionSpec] = {r.key: r for r in REGIONS}


def _weighted_choice(rng: random.Random, items: Sequence[Tuple[Any, float]]) -> Any:
    total = sum(w for _, w in items)
    u = rng.random() * total
    acc = 0.0
    for val, w in items:
        acc += w
        if u <= acc:
            return val
    return items[-1][0]


def _sample_footprint(rng: random.Random, region: RegionSpec, lcz: int) -> Tuple[int, int, float, float]:
    """Sample realistic (floors, units, w, d) conditioned on WUDAPT LCZ & region."""
    if lcz == 1:  # Compact high-rise
        floors = rng.randint(6, 16)
        w = rng.uniform(16.0, 32.0)
        d = rng.uniform(14.0, 24.0)
        units = max(8, int((w * d / 75.0) * floors))
    elif lcz == 2:  # Compact mid-rise
        floors = rng.randint(3, 6)
        w = rng.uniform(11.0, 22.0)
        d = rng.uniform(9.5, 16.0)
        units = max(4, int((w * d / 80.0) * floors))
    elif lcz == 3:  # Compact low-rise
        floors = 1 if rng.random() < 0.35 else 2
        if region.region_id == 2:  # Tokyo compact
            w = rng.uniform(5.5, 9.5)
            d = rng.uniform(5.0, 8.5)
        else:
            w = rng.uniform(6.0, 11.0)
            d = rng.uniform(5.5, 10.0)
        units = 1 if rng.random() < 0.72 else 2
    elif lcz == 4:  # Open high-rise
        floors = rng.randint(5, 14)
        w = rng.uniform(18.0, 34.0)
        d = rng.uniform(13.0, 22.0)
        units = max(10, int((w * d / 85.0) * floors))
    elif lcz == 5:  # Open mid-rise
        floors = rng.randint(3, 5)
        w = rng.uniform(12.0, 24.0)
        d = rng.uniform(9.0, 15.0)
        units = max(3, int((w * d / 85.0) * floors))
    elif lcz == 6:  # Open low-rise (suburban)
        floors = 1 if rng.random() < 0.56 else 2
        if region.region_id == 4:  # Phoenix ranch / courtyard
            w = rng.uniform(9.5, 16.5)
            d = rng.uniform(8.0, 13.0)
            floors = 1 if rng.random() < 0.78 else 2
        elif region.region_id == 2:  # Tokyo suburban
            w = rng.uniform(6.0, 10.5)
            d = rng.uniform(5.5, 9.0)
        else:
            w = rng.uniform(7.5, 14.0)
            d = rng.uniform(6.0, 10.5)
        units = 0 if rng.random() < 0.08 else (1 if rng.random() < 0.85 else 2)
    elif lcz == 7:  # Lightweight low-rise
        floors = 1 if rng.random() < 0.85 else 2
        w = rng.uniform(4.2, 8.5)
        d = rng.uniform(3.8, 7.2)
        units = 0 if rng.random() < 0.06 else 1
    elif lcz == 8:  # Large low-rise (commercial / warehouse)
        floors = 1 if rng.random() < 0.68 else 2
        w = rng.uniform(18.0, 38.0)
        d = rng.uniform(14.0, 28.0)
        units = 0 if rng.random() < 0.75 else 2
    elif lcz == 10:  # Heavy industry
        floors = 1 if rng.random() < 0.6 else 2
        w = rng.uniform(20.0, 42.0)
        d = rng.uniform(15.0, 30.0)
        units = 0
    else:  # LCZ 9: Sparsely built / village
        floors = 1 if rng.random() < 0.64 else 2
        w = rng.uniform(5.5, 12.5)
        d = rng.uniform(4.8, 9.5)
        units = 0 if rng.random() < 0.22 else 1

    # Randomize orientation swap so w and d vary naturally
    if rng.random() < 0.35:
        w, d = d, w
    return floors, units, round(w, 2), round(d, 2)


def _empirical_roof_dist(
    region_id: int,
    koppen_group: int,
    density_class: int,
    floors: int,
    has_units: int,
    area: float,
    aspect: float,
) -> Tuple[List[Tuple[int, float]], List[Tuple[int, float]]]:
    """Return empirical (roof_shape_weights, roof_tone_weights) grounded in climate & architecture."""
    # Large non-residential or mid/high-rise buildings strongly favor flat roofs + ConcreteFlat / MetalLight
    is_tall = floors >= 4 or (floors >= 3 and density_class >= 3)
    is_large_box = area >= 300.0 and has_units == 0

    if region_id == 0:  # us_midwest
        if is_tall:
            shapes = [(0, 0.82), (2, 0.10), (4, 0.08)]
            tones = [(7, 0.78), (5, 0.12), (4, 0.10)]
        elif is_large_box:
            shapes = [(0, 0.72), (1, 0.18), (3, 0.10)]
            tones = [(7, 0.52), (6, 0.36), (5, 0.12)]
        elif has_units == 0:
            shapes = [(1, 0.64), (3, 0.24), (2, 0.12)]
            tones = [(5, 0.42), (6, 0.34), (3, 0.24)]
        else:
            shapes = [(1, 0.58), (2, 0.32), (6, 0.06), (4, 0.04)]
            tones = [(5, 0.60), (4, 0.16), (3, 0.12), (2, 0.12)]

    elif region_id == 1:  # eu_alpine
        if is_tall:
            shapes = [(0, 0.80), (4, 0.12), (1, 0.08)]
            tones = [(7, 0.78), (4, 0.14), (6, 0.08)]
        elif is_large_box:
            shapes = [(0, 0.62), (1, 0.28), (3, 0.10)]
            tones = [(7, 0.45), (6, 0.35), (4, 0.20)]
        elif floors == 3 and density_class >= 2:
            shapes = [(1, 0.46), (4, 0.28), (0, 0.16), (2, 0.10)]
            tones = [(1, 0.48), (4, 0.34), (7, 0.18)]
        else:
            shapes = [(1, 0.56), (6, 0.20), (2, 0.14), (4, 0.10)]
            if koppen_group == 4:  # colder alpine Dfb/Dfc
                tones = [(1, 0.44), (4, 0.34), (3, 0.12), (2, 0.10)]
            else:
                tones = [(1, 0.58), (4, 0.24), (2, 0.12), (3, 0.06)]

    elif region_id == 2:  # jp_east_asia
        if is_tall:
            shapes = [(0, 0.92), (2, 0.05), (3, 0.03)]
            tones = [(7, 0.88), (6, 0.08), (4, 0.04)]
        elif is_large_box:
            shapes = [(0, 0.78), (3, 0.14), (1, 0.08)]
            tones = [(7, 0.60), (6, 0.30), (4, 0.10)]
        elif floors == 3:
            shapes = [(0, 0.58), (2, 0.24), (1, 0.18)]
            tones = [(7, 0.56), (4, 0.30), (5, 0.14)]
        else:
            if aspect < 1.22:
                shapes = [(2, 0.46), (1, 0.28), (5, 0.16), (0, 0.10)]
            else:
                shapes = [(2, 0.48), (1, 0.36), (0, 0.10), (3, 0.06)]
            tones = [(4, 0.54), (5, 0.26), (6, 0.12), (7, 0.08)]

    elif region_id == 3:  # af_west_tropical
        if is_tall:
            shapes = [(0, 0.84), (3, 0.10), (2, 0.06)]
            tones = [(7, 0.80), (6, 0.20)]
        elif is_large_box:
            shapes = [(3, 0.46), (0, 0.44), (1, 0.10)]
            tones = [(6, 0.62), (7, 0.38)]
        elif floors == 3:
            shapes = [(2, 0.46), (0, 0.38), (3, 0.16)]
            tones = [(7, 0.38), (6, 0.36), (1, 0.26)]
        else:
            if density_class == 2 and area < 55.0:  # lightweight / compact
                shapes = [(3, 0.48), (2, 0.36), (1, 0.16)]
                tones = [(6, 0.64), (2, 0.18), (1, 0.18)]
            else:
                shapes = [(2, 0.50), (3, 0.28), (1, 0.16), (5, 0.06)]
                tones = [(6, 0.48), (1, 0.30), (2, 0.14), (5, 0.08)]

    elif region_id == 4:  # us_southwest_arid
        if floors >= 3 or density_class >= 3:
            shapes = [(0, 0.90), (2, 0.10)]
            tones = [(7, 0.84), (1, 0.10), (6, 0.06)]
        elif is_large_box:
            shapes = [(0, 0.86), (3, 0.14)]
            tones = [(7, 0.76), (6, 0.24)]
        else:
            shapes = [(0, 0.48), (2, 0.42), (1, 0.06), (3, 0.04)]
            tones = [(1, 0.46), (7, 0.36), (2, 0.18)]

    else:  # sa_tropical (region_id == 5)
        if floors >= 3 or density_class >= 3:
            shapes = [(0, 0.84), (2, 0.10), (1, 0.06)]
            tones = [(7, 0.82), (1, 0.12), (6, 0.06)]
        elif is_large_box:
            shapes = [(0, 0.66), (3, 0.22), (1, 0.12)]
            tones = [(7, 0.56), (6, 0.32), (1, 0.12)]
        else:
            shapes = [(2, 0.44), (1, 0.32), (0, 0.16), (3, 0.08)]
            tones = [(1, 0.58), (2, 0.18), (7, 0.16), (6, 0.08)]

    # Mediterranean override if queried explicitly with Csa
    if koppen_group == 3 and floors <= 2 and not is_large_box:
        shapes = [(2, 0.48), (1, 0.34), (0, 0.18)]
        tones = [(1, 0.68), (2, 0.20), (7, 0.12)]

    return shapes, tones


def extract_features(
    region_id: int,
    koppen_group: int,
    density_class: int,
    floors: int,
    units: int,
    w: float,
    d: float,
) -> List[float]:
    area = max(1.0, float(w) * float(d))
    lo = max(0.5, min(float(w), float(d)))
    hi = max(float(w), float(d))
    return [
        math.log(area),
        hi / lo,
        float(max(1, floors)),
        1.0 if units > 0 else 0.0,
        float(max(0, min(4, density_class))),
        float(max(0, min(4, koppen_group))),
        float(max(0, min(5, region_id))),
    ]


def generate_calibration_dataset(n_samples: int, seed: int = 20261124) -> List[Dict[str, Any]]:
    rng = random.Random(seed)
    rows: List[Dict[str, Any]] = []
    per_region = n_samples // len(REGIONS)

    for reg in REGIONS:
        for i in range(per_region):
            # Ensure every LCZ 1..10 is represented while preserving regional weights
            if i < 400:
                lcz = (i % 10) + 1
            else:
                lcz = _weighted_choice(rng, reg.lcz_weights)

            kop_sym = _weighted_choice(rng, reg.koppen_mix)
            # Inject occasional Mediterranean Csa rows on EU/Arid boundary so Csa band is calibrated
            if reg.region_id == 1 and i % 40 == 0:
                kop_sym = "Csa"
            kop_grp = koppen_to_group(kop_sym)
            d_cls = lcz_to_density_class(lcz)
            floors, units, w, d = _sample_footprint(rng, reg, lcz)
            area = w * d
            aspect = max(w, d) / max(0.5, min(w, d))
            has_units = 1 if units > 0 else 0

            shape_w, tone_w = _empirical_roof_dist(
                reg.region_id, kop_grp, d_cls, floors, has_units, area, aspect
            )
            roof_shape = _weighted_choice(rng, shape_w)
            roof_tone = _weighted_choice(rng, tone_w)

            # Enforce physical coherence: flat roof uses flat/metal tone, pitched doesn't pick ConcreteFlat
            if roof_shape == 0 and roof_tone not in (7, 6, 1):
                roof_tone = 7
            if roof_shape != 0 and roof_tone == 7:
                roof_tone = tone_w[0][0] if tone_w[0][0] != 7 else 5

            feats = extract_features(reg.region_id, kop_grp, d_cls, floors, units, w, d)
            rows.append({
                "features": feats,
                "roof_shape": roof_shape,
                "roof_tone": roof_tone,
                "region_id": reg.region_id,
                "koppen_group": kop_grp,
                "density_class": d_cls,
                "lcz": lcz,
                "floors": floors,
                "units": units,
                "w": w,
                "d": d,
            })

    rng.shuffle(rows)
    return rows


# ---------------------------------------------------------------------------
# Pure-Python CART Decision Tree Classifier (zero external dependencies)
# ---------------------------------------------------------------------------

def _gini(counts: Dict[int, int], total: int) -> float:
    if total <= 0:
        return 0.0
    inv = 1.0 / total
    s = 0.0
    for c in counts.values():
        p = c * inv
        s += p * p
    return 1.0 - s


def _fit_cart_tree(
    samples: List[Tuple[List[float], int]],
    classes: Sequence[int],
    max_depth: int = 7,
    min_samples_leaf: int = 25,
    min_impurity_gain: float = 1e-4,
    depth: int = 0,
) -> Dict[str, Any]:
    n = len(samples)
    counts = Counter(y for _, y in samples)
    majority = max(classes, key=lambda c: (counts.get(c, 0), -c))
    parent_gini = _gini(counts, n)

    if depth >= max_depth or n < 2 * min_samples_leaf or len(counts) <= 1:
        return {
            "leaf": int(majority),
            "n": n,
            "counts": {int(k): int(v) for k, v in counts.items()},
        }

    best_gain = 0.0
    best_split: Optional[Tuple[int, str, float, List[Tuple[List[float], int]], List[Tuple[List[float], int]]]] = None
    n_feats = len(FEATURE_NAMES)

    for fi in range(n_feats):
        if fi in NOMINAL_FEATURES:
            vals = sorted({x[fi] for x, _ in samples})
            if len(vals) <= 1:
                continue
            for v in vals:
                left = [row for row in samples if row[0][fi] == v]
                right = [row for row in samples if row[0][fi] != v]
                if len(left) < min_samples_leaf or len(right) < min_samples_leaf:
                    continue
                lc = Counter(y for _, y in left)
                rc = Counter(y for _, y in right)
                w_gini = (len(left) * _gini(lc, len(left)) + len(right) * _gini(rc, len(right))) / n
                gain = parent_gini - w_gini
                if gain > best_gain:
                    best_gain = gain
                    best_split = (fi, "eq", float(v), left, right)
        else:
            # Quantile candidate thresholds (up to 24 thresholds) for fast exact-ish CART
            vals = sorted(row[0][fi] for row in samples)
            uniq = sorted(set(vals))
            if len(uniq) <= 1:
                continue
            if len(uniq) <= 24:
                thresholds = [0.5 * (uniq[i] + uniq[i + 1]) for i in range(len(uniq) - 1)]
            else:
                step = len(uniq) / 24.0
                cands = [uniq[min(len(uniq) - 1, int(k * step))] for k in range(1, 24)]
                thresholds = sorted(set(cands))
            for th in thresholds:
                left = [row for row in samples if row[0][fi] <= th]
                right = [row for row in samples if row[0][fi] > th]
                if len(left) < min_samples_leaf or len(right) < min_samples_leaf:
                    continue
                lc = Counter(y for _, y in left)
                rc = Counter(y for _, y in right)
                w_gini = (len(left) * _gini(lc, len(left)) + len(right) * _gini(rc, len(right))) / n
                gain = parent_gini - w_gini
                if gain > best_gain:
                    best_gain = gain
                    best_split = (fi, "le", float(th), left, right)

    if best_split is None or best_gain < min_impurity_gain:
        return {
            "leaf": int(majority),
            "n": n,
            "counts": {int(k): int(v) for k, v in counts.items()},
        }

    fi, op, th, left_rows, right_rows = best_split
    left_node = _fit_cart_tree(left_rows, classes, max_depth, min_samples_leaf, min_impurity_gain, depth + 1)
    right_node = _fit_cart_tree(right_rows, classes, max_depth, min_samples_leaf, min_impurity_gain, depth + 1)

    # Prune redundant branches that predict the exact same leaf class
    if "leaf" in left_node and "leaf" in right_node and left_node["leaf"] == right_node["leaf"]:
        return {
            "leaf": int(left_node["leaf"]),
            "n": n,
            "counts": {int(k): int(v) for k, v in counts.items()},
        }

    return {
        "feat": fi,
        "name": FEATURE_NAMES[fi],
        "op": op,
        "thresh": round(th, 4),
        "left": left_node,
        "right": right_node,
    }


def _predict_tree(node: Dict[str, Any], x: Sequence[float]) -> int:
    while "leaf" not in node:
        v = x[node["feat"]]
        go_left = (v == node["thresh"]) if node["op"] == "eq" else (v <= node["thresh"])
        node = node["left"] if go_left else node["right"]
    return int(node["leaf"])


def _tree_stats(node: Dict[str, Any]) -> Tuple[int, int]:
    if "leaf" in node:
        return 1, 0
    ln, ld = _tree_stats(node["left"])
    rn, rd = _tree_stats(node["right"])
    return ln + rn, 1 + max(ld, rd)


# ---------------------------------------------------------------------------
# Overture Maps Parquet Ingestion via DuckDB (--overture mode)
# ---------------------------------------------------------------------------

OVERTURE_SQL = """
-- Joins Overture Maps Buildings Parquet (`roof_shape`, `roof_color`, `facade_material`, `num_floors`)
-- with Koppen-Geiger climate band & WUDAPT LCZ 1..10 lookup tables by 0.1-degree cell or spatial join.
WITH raw_b AS (
    SELECT
        COALESCE(LOWER(roof_shape), 'unknown') AS raw_roof_shape,
        COALESCE(LOWER(roof_color), '') AS raw_roof_color,
        COALESCE(LOWER(roof_material), '') AS raw_roof_material,
        COALESCE(LOWER(facade_material), '') AS raw_facade_material,
        GREATEST(1, COALESCE(num_floors, 1)) AS floors,
        COALESCE(subtype, 'residential') AS subtype,
        (bbox.xmax - bbox.xmin) * 111320.0 * COS(RADIANS((bbox.ymin + bbox.ymax) * 0.5)) AS w_m,
        (bbox.ymax - bbox.ymin) * 111320.0 AS d_m,
        (bbox.ymin + bbox.ymax) * 0.5 AS lat,
        (bbox.xmin + bbox.xmax) * 0.5 AS lon
    FROM read_parquet(?)
    WHERE bbox.xmax > bbox.xmin AND bbox.ymax > bbox.ymin
      AND roof_shape IS NOT NULL
)
SELECT
    CASE
        WHEN raw_roof_shape IN ('flat', 'sawtooth') THEN 0
        WHEN raw_roof_shape IN ('gabled', 'saltbox') THEN 1
        WHEN raw_roof_shape IN ('hipped', 'hip_and_valley') THEN 2
        WHEN raw_roof_shape IN ('skillion', 'shed', 'lean_to') THEN 3
        WHEN raw_roof_shape IN ('mansard', 'gambrel') THEN 4
        WHEN raw_roof_shape IN ('pyramidal', 'dome', 'round', 'onion') THEN 5
        WHEN raw_roof_shape IN ('half-hipped', 'half_hipped', 'jerkinhead') THEN 6
        ELSE 1
    END AS roof_shape,
    CASE
        WHEN raw_roof_shape = 'flat' OR raw_roof_material IN ('concrete', 'tar_and_gravel') THEN 7
        WHEN raw_roof_material IN ('metal', 'zinc', 'aluminum', 'corrugated_metal') THEN 6
        WHEN raw_roof_material IN ('slate', 'stone') OR raw_roof_color IN ('grey', 'gray', 'slate', 'charcoal') THEN 4
        WHEN raw_roof_color IN ('red', 'terracotta', 'orange', 'maroon') OR raw_roof_material IN ('roof_tiles', 'clay', 'terracotta') THEN 1
        WHEN raw_roof_color IN ('brown', 'tan') THEN 2
        WHEN raw_roof_material IN ('wood', 'thatch', 'wood_shingles') THEN 3
        ELSE 5
    END AS roof_tone,
    floors,
    CASE WHEN subtype IN ('commercial', 'industrial', 'agricultural', 'outbuilding') THEN 0 ELSE 1 END AS has_units,
    GREATEST(2.0, LEAST(120.0, w_m)) AS w,
    GREATEST(2.0, LEAST(120.0, d_m)) AS d,
    lat,
    lon
FROM raw_b
WHERE w_m BETWEEN 2.0 AND 150.0 AND d_m BETWEEN 2.0 AND 150.0
LIMIT 200000;
"""


def _infer_region_and_koppen(lat: float, lon: float) -> Tuple[int, int]:
    """Coarse bounding-box fallback mapping (lat, lon) -> (region_id, koppen_group) for Overture rows."""
    if 34.0 <= lat <= 49.0 and -97.0 <= lon <= -78.0:
        return 0, 4  # us_midwest (Dfa)
    if 44.0 <= lat <= 51.0 and 4.0 <= lon <= 17.0:
        return 1, 2  # eu_alpine (Cfb)
    if 30.0 <= lat <= 41.0 and 128.0 <= lon <= 145.0:
        return 2, 2  # jp_east_asia (Cfa)
    if 3.0 <= lat <= 12.0 and -12.0 <= lon <= 12.0:
        return 3, 0  # af_west_tropical (Aw)
    if 29.0 <= lat <= 37.0 and -117.0 <= lon <= -103.0:
        return 4, 1  # us_southwest_arid (BWh)
    if -30.0 <= lat <= -5.0 and -55.0 <= lon <= -35.0:
        return 5, 0 if lat > -18.0 else 2  # sa_tropical
    # Global Koppen latitude band fallback
    alat = abs(lat)
    if alat < 15.0:
        return 3, 0
    if alat < 32.0:
        return 4, 1
    if alat < 46.0:
        return 0, 2
    return 1, 4


def load_overture_dataset(parquet_glob: str) -> List[Dict[str, Any]]:
    try:
        import duckdb  # type: ignore
    except ImportError as exc:
        raise RuntimeError(
            "duckdb Python package is required for --overture mode (install via `pip install duckdb`). "
            "Run with `--calibrate` for zero-dependency offline calibration."
        ) from exc

    con = duckdb.connect(":memory:")
    result = con.execute(OVERTURE_SQL, [parquet_glob]).fetchall()
    rows: List[Dict[str, Any]] = []
    for roof_shape, roof_tone, floors, has_units, w, d, lat, lon in result:
        region_id, kop_grp = _infer_region_and_koppen(float(lat), float(lon))
        d_cls = 4 if floors >= 7 else (3 if floors >= 3 else (2 if w * d < 75.0 else 1))
        feats = extract_features(region_id, kop_grp, d_cls, int(floors), int(has_units), float(w), float(d))
        rows.append({
            "features": feats,
            "roof_shape": int(roof_shape),
            "roof_tone": int(roof_tone),
            "region_id": region_id,
            "koppen_group": kop_grp,
            "density_class": d_cls,
            "lcz": 2 if d_cls == 3 else (1 if d_cls == 4 else (3 if d_cls == 2 else 6)),
            "floors": int(floors),
            "units": int(has_units),
            "w": float(w),
            "d": float(d),
        })
    return rows


# ---------------------------------------------------------------------------
# Quantized Lookup Table + Code Generation (JSON / Rust / JS)
# ---------------------------------------------------------------------------

def build_priors_payload(
    rows: List[Dict[str, Any]],
    shape_tree: Dict[str, Any],
    tone_tree: Dict[str, Any],
    metrics: Dict[str, Any],
    mode: str,
) -> Dict[str, Any]:
    # Quantized lookup table indexed by (region_id, density_class)
    lookup: Dict[str, Dict[str, Any]] = {}
    for reg in REGIONS:
        for d_cls in range(5):
            sub = [r for r in rows if r["region_id"] == reg.region_id and r["density_class"] == d_cls]
            if not sub:
                sub = [r for r in rows if r["region_id"] == reg.region_id]
            s_counts = Counter(r["roof_shape"] for r in sub)
            t_counts = Counter(r["roof_tone"] for r in sub)
            s_tot = max(1, sum(s_counts.values()))
            t_tot = max(1, sum(t_counts.values()))
            # Quantize probabilities to 0..255 (u8)
            shape_q = [round(255 * s_counts.get(k, 0) / s_tot) for k in range(7)]
            tone_q = [round(255 * t_counts.get(k, 0) / t_tot) for k in range(1, 8)]

            porch_scale = 1.0 if d_cls <= 1 else (0.55 if d_cls == 2 else 0.06)
            porch_prob = round(reg.porch_base * porch_scale, 3)
            eave_ratio = round(reg.eave_flat if d_cls >= 3 else reg.eave_pitched, 3)

            lookup[f"{reg.region_id}:{d_cls}"] = {
                "roof_shape_q": shape_q,
                "roof_tone_q": tone_q,
                "eave_ratio_q": round(eave_ratio * 255),
                "porch_prob_q": round(porch_prob * 255),
                "eave_ratio": eave_ratio,
                "porch_prob": porch_prob,
                "tree_biome": reg.tree_biome,
                "wall_palette": [list(c) for c in reg.wall_palette],
            }

    presets = []
    for reg in REGIONS:
        presets.append({
            "region_id": reg.region_id,
            "key": reg.key,
            "name": reg.name,
            "label": reg.label,
            "default_koppen": reg.koppen_mix[0][0],
            "koppen_group": koppen_to_group(reg.koppen_mix[0][0]),
            "tree_biome": reg.tree_biome,
            "tree_biome_name": TREE_BIOMES[reg.tree_biome],
            "eave_pitched": reg.eave_pitched,
            "eave_flat": reg.eave_flat,
            "porch_base": reg.porch_base,
            "wall_palette": [list(c) for c in reg.wall_palette],
            "ground_rgb": list(reg.ground_rgb),
            "tree_rgb": list(reg.tree_rgb),
        })

    return {
        "schema": "cewpos-regional-priors/v1",
        "mode": mode,
        "features": list(FEATURE_NAMES),
        "enums": {
            "roof_shapes": list(ROOF_SHAPES),
            "roof_tones": {str(k): {"name": v[0], "srgb": list(v[1])} for k, v in ROOF_TONES.items()},
            "tree_biomes": list(TREE_BIOMES),
            "koppen_groups": list(KOPPEN_GROUPS),
            "density_classes": list(DENSITY_CLASSES),
        },
        "metrics": metrics,
        "presets": presets,
        "lookup": lookup,
        "trees": {
            "roof_shape": shape_tree,
            "roof_tone": tone_tree,
        },
    }


def _emit_rust_tree(node: Dict[str, Any], indent: int = 4) -> List[str]:
    pad = " " * indent
    if "leaf" in node:
        return [f"{pad}{node['leaf']}"]
    fname = node["name"]
    if node["op"] == "eq":
        cond = f"f.{fname} == {int(node['thresh'])}"
    else:
        cond = f"f.{fname} <= {node['thresh']:.4f}" if fname in ("log_area", "aspect", "floors") else f"f.{fname} <= {int(node['thresh'])}"
    lines = [f"{pad}if {cond} {{"]
    lines.extend(_emit_rust_tree(node["left"], indent + 4))
    lines.append(f"{pad}}} else {{")
    lines.extend(_emit_rust_tree(node["right"], indent + 4))
    lines.append(f"{pad}}}")
    return lines


def _emit_js_tree(node: Dict[str, Any], indent: int = 2) -> List[str]:
    pad = " " * indent
    if "leaf" in node:
        return [f"{pad}return {node['leaf']};"]
    fname = node["name"]
    if node["op"] == "eq":
        cond = f"f.{fname} === {int(node['thresh'])}"
    else:
        cond = f"f.{fname} <= {node['thresh']:.4f}" if fname in ("log_area", "aspect", "floors") else f"f.{fname} <= {int(node['thresh'])}"
    lines = [f"{pad}if ({cond}) {{"]
    lines.extend(_emit_js_tree(node["left"], indent + 2))
    lines.append(f"{pad}}} else {{")
    lines.extend(_emit_js_tree(node["right"], indent + 2))
    lines.append(f"{pad}}}")
    return lines


def generate_rust_module(shape_tree: Dict[str, Any], tone_tree: Dict[str, Any]) -> str:
    shape_body = "\n".join(_emit_rust_tree(shape_tree, 4))
    tone_body = "\n".join(_emit_rust_tree(tone_tree, 4))

    wall_rows = []
    eave_p_rows = []
    eave_f_rows = []
    porch_rows = []
    biome_rows = []
    for reg in REGIONS:
        cols = ", ".join(f"[{c[0]}, {c[1]}, {c[2]}]" for c in reg.wall_palette)
        wall_rows.append(f"    [{cols}], // {reg.key}")
        eave_p_rows.append(f"{reg.eave_pitched:.3f}")
        eave_f_rows.append(f"{reg.eave_flat:.3f}")
        porch_rows.append(f"{reg.porch_base:.3f}")
        biome_rows.append(str(reg.tree_biome))

    rust_src = f'''//! Zero-dependency Regional Architecture Model for `holon-buildings` / `wasm32`.
//!
//! Auto-generated by `tools/regional-buildings/train_regional_priors.py`.
//! Maps `(region, koppen_climate, lcz_density, floors, units, w, d)` to
//! `holon-draw` building parameters (`roof_shape 0..6`, `roof_tone 1..7`,
//! `wall_rgb`, `eave_ratio`, `porch_prob`, `tree_biome`).

#![forbid(unsafe_code)]

/// Canonical `roof_shape` codes (`0..=6`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RoofShape {{
    Flat = 0,
    Gabled = 1,
    Hipped = 2,
    Skillion = 3,
    Mansard = 4,
    Pyramidal = 5,
    HalfHipped = 6,
}}

/// Canonical `roof_tone` codes (`1..=7`), matching `CEWP::builtform::RoofTone`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RoofTone {{
    RedTile = 1,
    BrownTile = 2,
    WeatheredWood = 3,
    GreySlate = 4,
    DarkShingle = 5,
    MetalLight = 6,
    ConcreteFlat = 7,
}}

/// Canonical `tree_biome` codes (`0..=4`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TreeBiome {{
    Broadleaf = 0,
    ConiferMixed = 1,
    TropicalPalm = 2,
    DesertScrub = 3,
    SubtropicalLush = 4,
}}

/// Exact `holon-draw` `Leaf` feature vector fed to the decision trees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LeafFeatures {{
    pub log_area: f32,
    pub aspect: f32,
    pub floors: f32,
    pub has_units: u8,
    pub density_class: u8,
    pub koppen_group: u8,
    pub region_id: u8,
}}

/// Predicted `holon-draw` regional building appearance & context parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BuildingPrediction {{
    pub roof_shape: u8,
    pub roof_tone: u8,
    pub roof_rgb: [u8; 3],
    pub wall_rgb: [u8; 3],
    pub eave_ratio: f32,
    pub porch_prob: f32,
    pub tree_biome: u8,
}}

const ROOF_TONE_SRGB: [[u8; 3]; 8] = [
    [120, 116, 110], // 0: Neutral fallback
    [178, 98, 72],   // 1: RedTile
    [142, 122, 104], // 2: BrownTile
    [128, 112, 94],  // 3: WeatheredWood
    [92, 100, 110],  // 4: GreySlate
    [84, 82, 80],    // 5: DarkShingle
    [168, 172, 176], // 6: MetalLight
    [150, 150, 146], // 7: ConcreteFlat
];

const WALL_PALETTES: [[[u8; 3]; 4]; 6] = [
{chr(10).join(wall_rows)}
];

const EAVE_PITCHED: [f32; 6] = [{", ".join(eave_p_rows)}];
const EAVE_FLAT: [f32; 6] = [{", ".join(eave_f_rows)}];
const PORCH_BASE: [f32; 6] = [{", ".join(porch_rows)}];
const REGION_BIOMES: [u8; 6] = [{", ".join(biome_rows)}];

/// Map a region key (`us_midwest`, `eu_alpine`, `jp_east_asia`, `af_west_tropical`,
/// `us_southwest_arid`, `sa_tropical`) to its numeric `region_id` (`0..=5`).
#[must_use]
pub fn region_from_str(region: &str) -> u8 {{
    if region.eq_ignore_ascii_case("us_midwest") || region.eq_ignore_ascii_case("ohio") {{
        0
    }} else if region.eq_ignore_ascii_case("eu_alpine") || region.eq_ignore_ascii_case("zurich") {{
        1
    }} else if region.eq_ignore_ascii_case("jp_east_asia") || region.eq_ignore_ascii_case("tokyo") {{
        2
    }} else if region.eq_ignore_ascii_case("af_west_tropical") || region.eq_ignore_ascii_case("lagos") {{
        3
    }} else if region.eq_ignore_ascii_case("us_southwest_arid") || region.eq_ignore_ascii_case("phoenix") {{
        4
    }} else if region.eq_ignore_ascii_case("sa_tropical") || region.eq_ignore_ascii_case("sao_paulo") {{
        5
    }} else {{
        0
    }}
}}

/// Map a Koppen-Geiger symbol (`Af`, `Aw`, `BWh`, `Cfa`, `Cfb`, `Csa`, `Dfa`, `Dfb`, etc.)
/// or climate group name to `koppen_group` (`0..=4`).
#[must_use]
pub fn koppen_group_from_str(koppen: &str) -> u8 {{
    let b = koppen.as_bytes();
    if b.is_empty() {{
        return 2;
    }}
    match b[0] {{
        b'A' | b'a' | b'T' | b't' => {{
            if koppen.eq_ignore_ascii_case("temperate") {{ 2 }} else {{ 0 }}
        }}
        b'B' | b'b' => 1,
        b'C' | b'c' => {{
            if (b.len() >= 2 && (b[1] == b's' || b[1] == b'S'))
                || koppen.eq_ignore_ascii_case("continental")
            {{
                if koppen.eq_ignore_ascii_case("continental") {{ 4 }} else {{ 3 }}
            }} else {{
                2
            }}
        }}
        b'D' | b'd' | b'E' | b'e' => 4,
        b'M' | b'm' => 3,
        _ => 2,
    }}
}}

/// Map WUDAPT LCZ (`1..=10`) to `holon-draw` `density_class` (`0..=4`).
#[must_use]
pub const fn lcz_to_density_class(lcz_density: u8) -> u8 {{
    match lcz_density {{
        1 => 4,
        2 | 4 | 5 => 3,
        3 | 7 => 2,
        6 | 8 | 10 => 1,
        9 => 0,
        other if other <= 4 => other,
        _ => 1,
    }}
}}

/// Return approximate sRGB `[r, g, b]` for `roof_tone` (`1..=7`).
#[must_use]
pub const fn roof_tone_srgb(tone: u8) -> [u8; 3] {{
    let idx = if (tone as usize) < ROOF_TONE_SRGB.len() {{ tone as usize }} else {{ 0 }};
    ROOF_TONE_SRGB[idx]
}}

/// Extract the exact 7 `holon-draw` `Leaf` features.
#[must_use]
pub fn extract_leaf_features(
    region_id: u8,
    koppen_group: u8,
    lcz_density: u8,
    floors: u8,
    units: u16,
    w: f32,
    d: f32,
) -> LeafFeatures {{
    let wf = if w > 0.5 {{ w }} else {{ 0.5 }};
    let df = if d > 0.5 {{ d }} else {{ 0.5 }};
    let area = if wf * df > 1.0 {{ wf * df }} else {{ 1.0 }};
    let (hi, lo) = if wf >= df {{ (wf, df) }} else {{ (df, wf) }};
    LeafFeatures {{
        log_area: area.ln(),
        aspect: hi / lo,
        floors: if floors > 0 {{ floors as f32 }} else {{ 1.0 }},
        has_units: if units > 0 {{ 1 }} else {{ 0 }},
        density_class: lcz_to_density_class(lcz_density),
        koppen_group: if koppen_group <= 4 {{ koppen_group }} else {{ 2 }},
        region_id: if region_id <= 5 {{ region_id }} else {{ 0 }},
    }}
}}

/// Predict `roof_shape` (`0..=6`) from extracted `LeafFeatures`.
#[must_use]
pub fn predict_roof_shape(f: &LeafFeatures) -> u8 {{
{shape_body}
}}

/// Predict `roof_tone` (`1..=7`) from extracted `LeafFeatures`.
#[must_use]
pub fn predict_roof_tone(f: &LeafFeatures) -> u8 {{
{tone_body}
}}

#[inline]
const fn mix_hash(region_id: u8, floors: u8, w_q: u32, d_q: u32, seed: u32) -> u32 {{
    let mut x = (region_id as u32).wrapping_mul(0x9E37_79B1)
        ^ (floors as u32).wrapping_mul(0x85EB_CA77)
        ^ w_q.wrapping_mul(0xC2B2_AE35)
        ^ d_q.wrapping_mul(0x27D4_EB2F)
        ^ seed.wrapping_mul(0x1656_67B1);
    x ^= x >> 15;
    x = x.wrapping_mul(0x2C1B_3C6D);
    x ^= x >> 12;
    x
}}

/// Predict complete `holon-draw` regional building parameters from string labels.
#[must_use]
pub fn predict_building(
    region: &str,
    koppen_climate: &str,
    lcz_density: u8,
    floors: u8,
    units: u16,
    w: f32,
    d: f32,
) -> BuildingPrediction {{
    let rid = region_from_str(region);
    let kg = koppen_group_from_str(koppen_climate);
    predict_building_from_ids(rid, kg, lcz_density, floors, units, w, d, 0)
}}

/// Predict complete `holon-draw` regional building parameters from numeric IDs + optional seed.
#[must_use]
pub fn predict_building_from_ids(
    region_id: u8,
    koppen_group: u8,
    lcz_density: u8,
    floors: u8,
    units: u16,
    w: f32,
    d: f32,
    seed: u32,
) -> BuildingPrediction {{
    let f = extract_leaf_features(region_id, koppen_group, lcz_density, floors, units, w, d);
    let rid = f.region_id as usize;
    let roof_shape = predict_roof_shape(&f);
    let mut roof_tone = predict_roof_tone(&f);
    if roof_shape == 0 && roof_tone != 6 && roof_tone != 7 {{
        roof_tone = 7;
    }}
    let roof_rgb = roof_tone_srgb(roof_tone);

    let h = mix_hash(f.region_id, floors, (w * 10.0) as u32, (d * 10.0) as u32, seed);
    let mut wall_rgb = WALL_PALETTES[rid][(h & 3) as usize];
    if f.density_class >= 3 && floors >= 4 {{
        // Urban concrete/plaster cooling for high-rise blocks
        wall_rgb = [
            ((wall_rgb[0] as u16 * 3 + 194) / 4) as u8,
            ((wall_rgb[1] as u16 * 3 + 196) / 4) as u8,
            ((wall_rgb[2] as u16 * 3 + 200) / 4) as u8,
        ];
    }}

    let eave_ratio = if roof_shape == 0 {{
        EAVE_FLAT[rid]
    }} else {{
        EAVE_PITCHED[rid]
    }};

    let porch_scale = if f.density_class <= 1 && floors <= 2 && units > 0 {{
        1.0_f32
    }} else if f.density_class == 2 && floors <= 2 && units > 0 {{
        0.55_f32
    }} else {{
        0.05_f32
    }};
    let porch_prob = PORCH_BASE[rid] * porch_scale;

    let tree_biome = match f.koppen_group {{
        0 => 2, // tropical -> tropical_palm
        1 => 3, // arid -> desert_scrub
        3 => 4, // mediterranean -> subtropical_lush
        _ => REGION_BIOMES[rid],
    }};

    BuildingPrediction {{
        roof_shape,
        roof_tone,
        roof_rgb,
        wall_rgb,
        eave_ratio,
        porch_prob,
        tree_biome,
    }}
}}
'''
    return rust_src


def generate_js_module(shape_tree: Dict[str, Any], tone_tree: Dict[str, Any]) -> str:
    shape_body = "\n".join(_emit_js_tree(shape_tree, 2))
    tone_body = "\n".join(_emit_js_tree(tone_tree, 2))

    presets_obj: Dict[str, Any] = {}
    for reg in REGIONS:
        presets_obj[reg.key] = {
            "region_id": reg.region_id,
            "key": reg.key,
            "name": reg.name,
            "label": reg.label,
            "koppen_default": reg.koppen_mix[0][0],
            "koppen_group": koppen_to_group(reg.koppen_mix[0][0]),
            "default_lcz": reg.lcz_weights[0][0],
            "eave_pitched": reg.eave_pitched,
            "eave_flat": reg.eave_flat,
            "porch_base": reg.porch_base,
            "tree_biome": reg.tree_biome,
            "tree_biome_name": TREE_BIOMES[reg.tree_biome],
            "wall_palette": [list(c) for c in reg.wall_palette],
            "ground_rgb": list(reg.ground_rgb),
            "tree_rgb": list(reg.tree_rgb),
        }

    wall_rows = []
    eave_p_rows = []
    eave_f_rows = []
    porch_rows = []
    biome_rows = []
    for reg in REGIONS:
        cols = ", ".join(f"[{c[0]}, {c[1]}, {c[2]}]" for c in reg.wall_palette)
        wall_rows.append(f"  [{cols}],")
        eave_p_rows.append(f"{reg.eave_pitched:.3f}")
        eave_f_rows.append(f"{reg.eave_flat:.3f}")
        porch_rows.append(f"{reg.porch_base:.3f}")
        biome_rows.append(str(reg.tree_biome))

    js_src = f'''// Zero-dependency ES Module for Regional Architecture Prediction (CEWPOS & CEWP).
// Auto-generated by tools/regional-buildings/train_regional_priors.py.

export const REGION_IDS = {{
  us_midwest: 0,
  eu_alpine: 1,
  jp_east_asia: 2,
  af_west_tropical: 3,
  us_southwest_arid: 4,
  sa_tropical: 5,
}};

export const ROOF_SHAPES = {json.dumps(list(ROOF_SHAPES))};

// Index 1..7 matches CEWP builtform::RoofTone and docs/tiling.js TL_ROOF_TONES
export const ROOF_TONE_SRGB = [
  [120, 116, 110], // 0: Neutral fallback
  [178, 98, 72],   // 1: RedTile
  [142, 122, 104], // 2: BrownTile
  [128, 112, 94],  // 3: WeatheredWood
  [92, 100, 110],  // 4: GreySlate
  [84, 82, 80],    // 5: DarkShingle
  [168, 172, 176], // 6: MetalLight
  [150, 150, 146], // 7: ConcreteFlat
];

export const TREE_BIOMES = {json.dumps(list(TREE_BIOMES))};

export const REGIONAL_PRESETS = {json.dumps(presets_obj, indent=2)};

const WALL_PALETTES = [
{chr(10).join(wall_rows)}
];
const EAVE_PITCHED = [{", ".join(eave_p_rows)}];
const EAVE_FLAT = [{", ".join(eave_f_rows)}];
const PORCH_BASE = [{", ".join(porch_rows)}];
const REGION_BIOMES = [{", ".join(biome_rows)}];

export function regionFromStr(region) {{
  if (typeof region === "number") return Math.max(0, Math.min(5, region | 0));
  const k = String(region || "us_midwest").trim().toLowerCase();
  if (k in REGION_IDS) return REGION_IDS[k];
  if (k === "ohio" || k === "your_town") return 0;
  if (k === "zurich" || k === "europe") return 1;
  if (k === "tokyo" || k === "east_asia") return 2;
  if (k === "lagos" || k === "west_africa") return 3;
  if (k === "phoenix" || k === "southwest") return 4;
  if (k === "sao_paulo" || k === "south_america") return 5;
  return 0;
}}

export function koppenGroupFromStr(koppen) {{
  if (typeof koppen === "number") return Math.max(0, Math.min(4, koppen | 0));
  const s = String(koppen || "Cfa").trim().toLowerCase();
  if (s === "tropical" || s.startsWith("af") || s.startsWith("am") || s.startsWith("aw")) return 0;
  if (s === "arid" || s.startsWith("bw") || s.startsWith("bs")) return 1;
  if (s === "mediterranean" || s.startsWith("cs")) return 3;
  if (s === "temperate" || s.startsWith("cf") || s.startsWith("cw")) return 2;
  if (s === "continental" || s === "boreal" || s.startsWith("df") || s.startsWith("ds") || s.startsWith("dw") || s.startsWith("et") || s.startsWith("ef")) return 4;
  return 2;
}}

export function lczToDensityClass(lczDensity) {{
  const lcz = lczDensity | 0;
  if (lcz === 1) return 4;
  if (lcz === 2 || lcz === 4 || lcz === 5) return 3;
  if (lcz === 3 || lcz === 7) return 2;
  if (lcz === 6 || lcz === 8 || lcz === 10) return 1;
  if (lcz === 9) return 0;
  return Math.max(0, Math.min(4, lcz));
}}

export function extractLeafFeatures(region, koppenClimate, lczDensity, floors, units, w, d) {{
  const wf = Math.max(0.5, Number(w) || 6.0);
  const df = Math.max(0.5, Number(d) || 5.0);
  const area = Math.max(1.0, wf * df);
  const hi = Math.max(wf, df);
  const lo = Math.min(wf, df);
  return {{
    log_area: Math.log(area),
    aspect: hi / lo,
    floors: Math.max(1, Number(floors) || 1),
    has_units: (Number(units) || 0) > 0 ? 1 : 0,
    density_class: lczToDensityClass(lczDensity),
    koppen_group: koppenGroupFromStr(koppenClimate),
    region_id: regionFromStr(region),
  }};
}}

export function predictRoofShape(f) {{
{shape_body}
}}

export function predictRoofTone(f) {{
{tone_body}
}}

function mixHash(regionId, floors, wQ, dQ, seed) {{
  let x = (Math.imul(regionId | 0, 0x9e3779b1) ^
           Math.imul(floors | 0, 0x85ebca77) ^
           Math.imul(wQ | 0, 0xc2b2ae35) ^
           Math.imul(dQ | 0, 0x27d4eb2f) ^
           Math.imul(seed | 0, 0x165667b1)) | 0;
  x ^= x >>> 15;
  x = Math.imul(x, 0x2c1b3c6d);
  x ^= x >>> 12;
  return x >>> 0;
}}

/**
 * Predict regional `holon-draw` building parameters from either positional arguments
 * `(region, koppenClimate, lczDensity, floors, units, w, d, seed = 0)` or a single spec object.
 */
export function predictBuilding(region, koppenClimate, lczDensity, floors, units, w, d, seed = 0) {{
  if (region && typeof region === "object") {{
    const spec = region;
    return predictBuilding(
      spec.region ?? spec.region_id ?? "us_midwest",
      spec.koppen_climate ?? spec.koppen ?? "Cfa",
      spec.lcz_density ?? spec.lcz ?? 6,
      spec.floors ?? spec.storeys ?? 1,
      spec.units ?? 1,
      spec.w ?? 8.0,
      spec.d ?? 6.0,
      spec.seed ?? 0
    );
  }}
  const f = extractLeafFeatures(region, koppenClimate, lczDensity, floors, units, w, d);
  const rid = f.region_id;
  const roof_shape = predictRoofShape(f);
  let roof_tone = predictRoofTone(f);
  if (roof_shape === 0 && roof_tone !== 6 && roof_tone !== 7) {{
    roof_tone = 7;
  }}
  const roof_rgb = ROOF_TONE_SRGB[roof_tone] || ROOF_TONE_SRGB[0];

  const h = mixHash(rid, floors | 0, ((Number(w) || 0) * 10) | 0, ((Number(d) || 0) * 10) | 0, seed | 0);
  let wall_rgb = WALL_PALETTES[rid][h & 3].slice();
  if (f.density_class >= 3 && f.floors >= 4) {{
    wall_rgb = [
      ((wall_rgb[0] * 3 + 194) / 4) | 0,
      ((wall_rgb[1] * 3 + 196) / 4) | 0,
      ((wall_rgb[2] * 3 + 200) / 4) | 0,
    ];
  }}

  const eave_ratio = roof_shape === 0 ? EAVE_FLAT[rid] : EAVE_PITCHED[rid];
  const porchScale = (f.density_class <= 1 && f.floors <= 2 && f.has_units > 0)
    ? 1.0
    : (f.density_class === 2 && f.floors <= 2 && f.has_units > 0 ? 0.55 : 0.05);
  const porch_prob = Number((PORCH_BASE[rid] * porchScale).toFixed(4));
  const tree_biome = f.koppen_group === 0 ? 2 : (f.koppen_group === 1 ? 3 : (f.koppen_group === 3 ? 4 : REGION_BIOMES[rid]));

  return {{
    roof_shape,
    roof_shape_name: ROOF_SHAPES[roof_shape],
    roof_tone,
    roof_rgb,
    wall_rgb,
    eave_ratio,
    porch_prob,
    tree_biome,
    tree_biome_name: TREE_BIOMES[tree_biome],
  }};
}}
'''
    return js_src


# ---------------------------------------------------------------------------
# Main Pipeline
# ---------------------------------------------------------------------------

def train_and_export(rows: List[Dict[str, Any]], out_dir: Path, mode: str) -> None:
    split = int(0.8 * len(rows))
    train_rows = rows[:split]
    test_rows = rows[split:]

    shape_train = [(r["features"], r["roof_shape"]) for r in train_rows]
    tone_train = [(r["features"], r["roof_tone"]) for r in train_rows]

    shape_tree = _fit_cart_tree(shape_train, list(range(7)), max_depth=7, min_samples_leaf=24)
    tone_tree = _fit_cart_tree(tone_train, list(range(1, 8)), max_depth=7, min_samples_leaf=24)

    shape_ok = sum(1 for r in test_rows if _predict_tree(shape_tree, r["features"]) == r["roof_shape"])
    tone_ok = sum(1 for r in test_rows if _predict_tree(tone_tree, r["features"]) == r["roof_tone"])
    shape_acc = shape_ok / max(1, len(test_rows))
    tone_acc = tone_ok / max(1, len(test_rows))

    s_leaves, s_depth = _tree_stats(shape_tree)
    t_leaves, t_depth = _tree_stats(tone_tree)

    per_region_summary: Dict[str, Any] = {}
    for reg in REGIONS:
        r_test = [r for r in test_rows if r["region_id"] == reg.region_id]
        r_all = [r for r in rows if r["region_id"] == reg.region_id]
        rs_acc = sum(1 for r in r_test if _predict_tree(shape_tree, r["features"]) == r["roof_shape"]) / max(1, len(r_test))
        rt_acc = sum(1 for r in r_test if _predict_tree(tone_tree, r["features"]) == r["roof_tone"]) / max(1, len(r_test))
        sc = Counter(_predict_tree(shape_tree, r["features"]) for r in r_all)
        tc = Counter(_predict_tree(tone_tree, r["features"]) for r in r_all)
        per_region_summary[reg.key] = {
            "samples": len(r_all),
            "test_shape_acc": round(rs_acc, 4),
            "test_tone_acc": round(rt_acc, 4),
            "pred_roof_shapes": {ROOF_SHAPES[k]: v for k, v in sorted(sc.items())},
            "pred_roof_tones": {ROOF_TONES[k][0]: v for k, v in sorted(tc.items())},
        }

    metrics = {
        "total_samples": len(rows),
        "train_samples": len(train_rows),
        "test_samples": len(test_rows),
        "roof_shape_test_acc": round(shape_acc, 4),
        "roof_tone_test_acc": round(tone_acc, 4),
        "roof_shape_tree": {"leaves": s_leaves, "depth": s_depth},
        "roof_tone_tree": {"leaves": t_leaves, "depth": t_depth},
        "per_region": per_region_summary,
    }

    out_dir.mkdir(parents=True, exist_ok=True)
    priors_path = out_dir / "regional_priors.json"
    rust_path = out_dir / "regional_model.rs"
    js_path = out_dir / "regional_model.js"

    payload = build_priors_payload(rows, shape_tree, tone_tree, metrics, mode)
    priors_path.write_text(json.dumps(payload, separators=(",", ":"), sort_keys=False) + "\n", encoding="utf-8")
    rust_path.write_text(generate_rust_module(shape_tree, tone_tree), encoding="utf-8")
    js_path.write_text(generate_js_module(shape_tree, tone_tree), encoding="utf-8")

    print(f"[regional-buildings] mode={mode} samples={len(rows)} (train={len(train_rows)}, test={len(test_rows)})")
    print(f"  roof_shape CART: depth={s_depth}, leaves={s_leaves}, test_acc={shape_acc:.4f}")
    print(f"  roof_tone  CART: depth={t_depth}, leaves={t_leaves}, test_acc={tone_acc:.4f}")
    for key, info in per_region_summary.items():
        print(
            f"  - {key:18s} | n={info['samples']:4d} | shape_acc={info['test_shape_acc']:.3f} | "
            f"tone_acc={info['test_tone_acc']:.3f} | shapes={info['pred_roof_shapes']}"
        )
    print("Artifacts:")
    for p in (priors_path, rust_path, js_path):
        print(f"  {p} ({p.stat().st_size} bytes)")


def main(argv: Optional[Sequence[str]] = None) -> int:
    parser = argparse.ArgumentParser(description="Train regional architectural priors & export Rust/JS/JSON models.")
    mode_grp = parser.add_mutually_exclusive_group()
    mode_grp.add_argument("--calibrate", action="store_true", help="Offline empirical calibration (default)")
    mode_grp.add_argument("--overture", metavar="PARQUET_GLOB", help="DuckDB Parquet glob for Overture Maps Buildings")
    parser.add_argument("--samples", type=int, default=12000, help="Calibration sample count (default: 12000)")
    parser.add_argument("--out-dir", type=Path, default=Path(__file__).resolve().parent, help="Output directory")
    args = parser.parse_args(argv)

    if args.overture:
        rows = load_overture_dataset(args.overture)
        mode = f"overture:{args.overture}"
    else:
        rows = generate_calibration_dataset(args.samples)
        mode = "calibrate"

    train_and_export(rows, args.out_dir, mode)
    return 0


if __name__ == "__main__":
    sys.exit(main())
