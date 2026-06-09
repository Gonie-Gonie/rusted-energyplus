from __future__ import annotations

import argparse
import json
import math
from dataclasses import dataclass
from pathlib import Path
from typing import Any


ORACLE_VERSION = "26.1.0"
CASE_ID = "official_1zone_uncontrolled_dynamic_diagnostic_001"
EXPECTED_SERIES_COUNT = 99
ZONE_SURFACE_CONVECTION_VARIABLE = "Zone Air Heat Balance Surface Convection Rate"
SURFACE_INSIDE_CONVECTION_VARIABLE = "Surface Inside Face Convection Heat Gain Rate"
SURFACE_INSIDE_CONDUCTION_VARIABLE = "Surface Inside Face Conduction Heat Transfer Rate"
SURFACE_OUTSIDE_CONDUCTION_VARIABLE = "Surface Outside Face Conduction Heat Transfer Rate"
SURFACE_HEAT_STORAGE_VARIABLE = "Surface Heat Storage Rate"
SURFACE_INSIDE_TEMPERATURE_VARIABLE = "Surface Inside Face Temperature"
SURFACE_OUTSIDE_TEMPERATURE_VARIABLE = "Surface Outside Face Temperature"
SURFACE_DRIVER_LIMIT = 3
FLOOR_CTF_DRIVER_KEY = "ZN001:FLR001"


@dataclass(frozen=True)
class ProbeLane:
    lane: str
    summary_path: Path


@dataclass(frozen=True)
class FocusMetric:
    key: str
    variable: str


LANES = (
    ProbeLane(
        lane="default",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-warmup-min20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-warmup-min20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-surface-iter3",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-surface-iter3"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-surface-first",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-surface-first"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-iter3",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-surface-iter3"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-iter3",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-surface-iter3"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-doe2-iter3",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-doe2-surface-iter3"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-iter3",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-surface-iter3"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-iter3",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-surface-iter3"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-iter5",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-surface-iter5"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-iter8",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-surface-iter8"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-iter8",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-surface-iter8"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-weather-storage-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-weather-air-storage-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-prevmat-surfconv-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-prevmat-surfconv-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-frozen-outside-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-frozen-outside-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-commit-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-commit-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-warmup-min20-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-warmup-min20-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-live-refair-warmup-min20-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-live-refair-warmup-min20-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-live-hconv-warmup-min20-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-live-hconv-warmup-min20-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-surf-refair-report-warmup-min20-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-surf-refair-report-warmup-min20-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-final-hconv-report-warmup-min20-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-final-hconv-report-warmup-min20-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-inside-ctf-report-warmup-min20-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-inside-ctf-report-warmup-min20-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-adhist-report-warmup-min20-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-adhist-report-warmup-min20-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-zone-surf-report-warmup-min20-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-warmup-min20-surface-iter20-zone-surf-report"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-adhist-commit-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-adhist-commit-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-current-adhist-iter20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-current-adhist-surface-iter20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-epseed-iter5",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-epseed-surface-iter5"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-iter5",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-surface-iter5"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-interior-longwave-iter5",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-lw-surface-iter5"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-iter5",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-lw-surface-iter5"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-scriptf-interior-longwave-iter5",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-scriptf-lw-surface-iter5"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-interior-longwave-iter5",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-lw-surface-iter5"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-coupled-previous-boundary-iter3",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-coupled-previous-boundary-surface-iter3"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf-analytical-surface-first-iter3",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf-analytical-surface-first-surface-iter3"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="analytical",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-analytical"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="analytical-surface-first",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-analytical-surface-first"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="third-order",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-third-order"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="warmup-min20",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-warmup-min20"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
)

FOCUS_METRICS = (
    FocusMetric("ZONE ONE", "Zone Mean Air Temperature"),
    FocusMetric("ZONE ONE", "Zone Air Heat Balance Internal Convective Heat Gain Rate"),
    FocusMetric("ZONE ONE", "Zone Air Heat Balance Surface Convection Rate"),
    FocusMetric("ZONE ONE", "Zone Air Heat Balance Air Energy Storage Rate"),
    FocusMetric("ZN001:FLR001", "Surface Inside Face Temperature"),
    FocusMetric("ZN001:FLR001", "Surface Outside Face Temperature"),
    FocusMetric("ZN001:FLR001", "Surface Inside Face Conduction Heat Transfer Rate"),
    FocusMetric("ZN001:FLR001", "Surface Outside Face Conduction Heat Transfer Rate"),
    FocusMetric("ZN001:FLR001", "Surface Heat Storage Rate"),
    FocusMetric("ZN001:ROOF001", "Surface Inside Face Temperature"),
    FocusMetric("ZN001:ROOF001", "Surface Outside Face Temperature"),
    FocusMetric("ZONE ONE", "Zone Opaque Surface Inside Faces Conduction Rate"),
    FocusMetric("ZONE ONE", "Zone Opaque Surface Outside Faces Conduction Rate"),
    FocusMetric("ZN001:ROOF001", "Surface Outside Face Incident Solar Radiation Rate per Area"),
    FocusMetric("ZN001:ROOF001", "Surface Outside Face Incident Beam Solar Radiation Rate per Area"),
    FocusMetric("ZN001:ROOF001", "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area"),
    FocusMetric("ZN001:ROOF001", "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area"),
    FocusMetric("ZN001:ROOF001", "Surface Outside Face Convection Heat Gain Rate"),
    FocusMetric("ZN001:ROOF001", "Surface Outside Face Convection Heat Transfer Coefficient"),
    FocusMetric("ZN001:ROOF001", "Surface Outside Face Net Thermal Radiation Heat Gain Rate"),
    FocusMetric("ZN001:ROOF001", "Surface Outside Face Solar Radiation Heat Gain Rate"),
)

REFERENCE_LANES = {
    "all-ctf": "default",
    "all-ctf-warmup-min20": "all-ctf",
    "all-ctf-surface-iter3": "all-ctf",
    "all-ctf-analytical-surface-first": "all-ctf",
    "all-ctf-analytical-coupled": "all-ctf-analytical-surface-first",
    "all-ctf-analytical-coupled-iter3": "all-ctf-analytical-coupled",
    "all-ctf-analytical-coupled-previous-inside-iter3": "all-ctf-analytical-coupled-iter3",
    "all-ctf-analytical-coupled-previous-inside-doe2-iter3": "all-ctf-analytical-coupled-previous-inside-iter3",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-iter3": "all-ctf-analytical-coupled-previous-inside-iter3",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-iter3": "all-ctf-analytical-coupled-previous-inside-quick-outside-iter3",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-iter5": "all-ctf-analytical-coupled-previous-inside-quick-outside-iter3",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-iter8": "all-ctf-analytical-coupled-previous-inside-quick-outside-iter5",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-iter8": "all-ctf-analytical-coupled-previous-inside-quick-outside-iter8",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-iter20": "all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-iter8",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-iter20": "all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-iter20",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-iter20": "all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-iter20",
    "all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-iter20": "all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-iter20",
    "all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-iter20": "all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-iter20",
    "all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-weather-storage-iter20": "all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-prevmat-surfconv-iter20": "all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-weather-storage-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-iter20": "all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-weather-storage-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-frozen-outside-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-commit-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-warmup-min20-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-live-refair-warmup-min20-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-warmup-min20-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-live-hconv-warmup-min20-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-warmup-min20-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-surf-refair-report-warmup-min20-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-warmup-min20-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-final-hconv-report-warmup-min20-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-warmup-min20-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-inside-ctf-report-warmup-min20-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-warmup-min20-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-adhist-report-warmup-min20-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-warmup-min20-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-zone-surf-report-warmup-min20-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-warmup-min20-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-adhist-commit-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-iter20",
    "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-current-adhist-iter20": "all-ctf-third-order-frozen-hconv-weather-storage-balance-surfconv-iter20",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-epseed-iter5": "all-ctf-analytical-coupled-previous-inside-quick-outside-iter5",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-iter5": "all-ctf-analytical-coupled-previous-inside-quick-outside-iter5",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-interior-longwave-iter5": "all-ctf-analytical-coupled-previous-inside-quick-outside-iter5",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-iter5": "all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-iter5",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-scriptf-interior-longwave-iter5": "all-ctf-analytical-coupled-previous-inside-quick-outside-interior-longwave-iter5",
    "all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-interior-longwave-iter5": "all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-iter5",
    "all-ctf-analytical-coupled-previous-boundary-iter3": "all-ctf-analytical-coupled-previous-inside-iter3",
    "all-ctf-analytical-surface-first-iter3": "all-ctf-analytical-surface-first",
    "analytical": "default",
    "analytical-surface-first": "default",
    "third-order": "default",
    "warmup-min20": "default",
}

# The rollup is an interpretation aid, not a tolerance gate. Treat tiny probe
# movements as unchanged so warmup/no-op probes do not look more decisive than
# they are.
MOVEMENT_EPSILON = 1.0


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        payload = json.load(handle)
    if not isinstance(payload, dict):
        raise ValueError(f"Expected object JSON at {path}")
    return payload


def series_output_label(series: dict[str, Any]) -> str:
    output = series.get("output", {})
    if not isinstance(output, dict):
        return "none"
    return f"{output.get('key', 'none')} / {output.get('variable', 'none')}"


def find_series(summary: dict[str, Any], metric: FocusMetric) -> dict[str, Any] | None:
    for series in summary.get("series", []):
        if not isinstance(series, dict):
            continue
        output = series.get("output", {})
        if not isinstance(output, dict):
            continue
        if output.get("key") == metric.key and output.get("variable") == metric.variable:
            return series
    return None


def focus_metric_rows(summary: dict[str, Any]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for metric in FOCUS_METRICS:
        series = find_series(summary, metric)
        if series is None:
            rows.append(
                {
                    "key": metric.key,
                    "variable": metric.variable,
                    "label": f"{metric.key} / {metric.variable}",
                    "status": "missing",
                    "samples": None,
                    "rmse_delta_c": None,
                    "max_abs_delta_c": None,
                    "mean_abs_delta_c": None,
                }
            )
            continue
        rows.append(
            {
                "key": metric.key,
                "variable": metric.variable,
                "label": series_output_label(series),
                "status": series.get("status"),
                "samples": series.get("samples"),
                "rmse_delta_c": series.get("rmse_delta_c"),
                "max_abs_delta_c": series.get("max_abs_delta_c"),
                "mean_abs_delta_c": series.get("mean_abs_delta_c"),
            }
        )
    return rows


def surface_conduction_metric_rows(
    summary: dict[str, Any],
    variable: str,
) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for series in summary.get("series", []):
        if not isinstance(series, dict):
            continue
        output = series.get("output", {})
        if not isinstance(output, dict):
            continue
        if output.get("variable") != variable:
            continue
        rows.append(
            {
                "key": output.get("key", "none"),
                "variable": output.get("variable", variable),
                "label": series_output_label(series),
                "status": series.get("status"),
                "samples": series.get("samples"),
                "rmse_delta_c": series.get("rmse_delta_c"),
                "max_abs_delta_c": series.get("max_abs_delta_c"),
                "mean_abs_delta_c": series.get("mean_abs_delta_c"),
            }
        )

    rows.sort(
        key=lambda row: numeric(row.get("rmse_delta_c")) or -1.0,
        reverse=True,
    )
    return rows


def surface_balance_driver_rows(summary: dict[str, Any]) -> list[dict[str, Any]]:
    by_surface: dict[str, dict[str, dict[str, Any]]] = {}
    for variable in (
        SURFACE_HEAT_STORAGE_VARIABLE,
        SURFACE_INSIDE_CONDUCTION_VARIABLE,
        SURFACE_OUTSIDE_CONDUCTION_VARIABLE,
        SURFACE_INSIDE_TEMPERATURE_VARIABLE,
        SURFACE_OUTSIDE_TEMPERATURE_VARIABLE,
    ):
        for metric in surface_conduction_metric_rows(summary, variable):
            key = str(metric.get("key", "none"))
            by_surface.setdefault(key, {})[variable] = metric

    rows: list[dict[str, Any]] = []
    for surface_key, metrics in by_surface.items():
        storage_metric = metrics.get(SURFACE_HEAT_STORAGE_VARIABLE)
        inside_metric = metrics.get(SURFACE_INSIDE_CONDUCTION_VARIABLE)
        outside_metric = metrics.get(SURFACE_OUTSIDE_CONDUCTION_VARIABLE)
        inside_temperature_metric = metrics.get(SURFACE_INSIDE_TEMPERATURE_VARIABLE)
        outside_temperature_metric = metrics.get(SURFACE_OUTSIDE_TEMPERATURE_VARIABLE)
        leg_rmses = {
            "storage": numeric(storage_metric.get("rmse_delta_c"))
            if isinstance(storage_metric, dict)
            else None,
            "inside": numeric(inside_metric.get("rmse_delta_c"))
            if isinstance(inside_metric, dict)
            else None,
            "outside": numeric(outside_metric.get("rmse_delta_c"))
            if isinstance(outside_metric, dict)
            else None,
            "inside_temperature": numeric(inside_temperature_metric.get("rmse_delta_c"))
            if isinstance(inside_temperature_metric, dict)
            else None,
            "outside_temperature": numeric(outside_temperature_metric.get("rmse_delta_c"))
            if isinstance(outside_temperature_metric, dict)
            else None,
        }
        available_rmses = {
            name: rmse
            for name, rmse in leg_rmses.items()
            if rmse is not None and not name.endswith("_temperature")
        }
        dominant_leg = (
            max(available_rmses.items(), key=lambda item: item[1])[0]
            if available_rmses
            else "none"
        )
        rows.append(
            {
                "key": surface_key,
                "label": surface_key,
                "status": (
                    storage_metric or inside_metric or outside_metric or {}
                ).get("status"),
                "dominant_leg": dominant_leg,
                "storage_rmse_delta_c": leg_rmses["storage"],
                "inside_rmse_delta_c": leg_rmses["inside"],
                "outside_rmse_delta_c": leg_rmses["outside"],
                "inside_temperature_rmse_delta_c": leg_rmses["inside_temperature"],
                "outside_temperature_rmse_delta_c": leg_rmses["outside_temperature"],
                "storage_rmse_per_inside_temperature_rmse": ratio(
                    leg_rmses["storage"], leg_rmses["inside_temperature"]
                ),
                "storage_rmse_per_outside_temperature_rmse": ratio(
                    leg_rmses["storage"], leg_rmses["outside_temperature"]
                ),
                "inside_conduction_rmse_per_inside_temperature_rmse": ratio(
                    leg_rmses["inside"], leg_rmses["inside_temperature"]
                ),
                "outside_conduction_rmse_per_outside_temperature_rmse": ratio(
                    leg_rmses["outside"], leg_rmses["outside_temperature"]
                ),
                "storage_max_abs_delta_c": storage_metric.get("max_abs_delta_c")
                if isinstance(storage_metric, dict)
                else None,
                "inside_max_abs_delta_c": inside_metric.get("max_abs_delta_c")
                if isinstance(inside_metric, dict)
                else None,
                "outside_max_abs_delta_c": outside_metric.get("max_abs_delta_c")
                if isinstance(outside_metric, dict)
                else None,
            }
        )

    rows.sort(
        key=lambda row: (
            numeric(row.get("storage_rmse_delta_c"))
            or numeric(row.get("inside_rmse_delta_c"))
            or numeric(row.get("outside_rmse_delta_c"))
            or -1.0
        ),
        reverse=True,
    )
    return rows


def nested_numeric(payload: dict[str, Any], *keys: str) -> float | None:
    value: Any = payload
    for key in keys:
        if not isinstance(value, dict):
            return None
        value = value.get(key)
    return numeric(value)


def signed_delta(payload: dict[str, Any], rust_key: str, oracle_key: str) -> float | None:
    rust = numeric(payload.get(rust_key))
    oracle = numeric(payload.get(oracle_key))
    if rust is None or oracle is None:
        return None
    return rust - oracle


def sum_optional(left: float | None, right: float | None) -> float | None:
    if left is None or right is None:
        return None
    return left + right


def sum_present(values: list[float | None]) -> float | None:
    present = [value for value in values if value is not None]
    if not present:
        return None
    return sum(present)


def first_matching_key(rows: Any, key: str) -> dict[str, Any] | None:
    return next(iter(matching_key_rows(rows, key)), None)


def matching_key_rows(rows: Any, key: str) -> list[dict[str, Any]]:
    if not isinstance(rows, list):
        return []
    return [
        row
        for row in rows
        if isinstance(row, dict) and row.get("key") == key
    ]


def floor_ctf_history_driver_row(summary: dict[str, Any]) -> dict[str, Any] | None:
    first_sample = first_matching_key(
        summary.get("ctf_history_first_sample_deltas"),
        FLOOR_CTF_DRIVER_KEY,
    )
    series = first_matching_key(
        summary.get("ctf_history_series_deltas"),
        FLOOR_CTF_DRIVER_KEY,
    )
    if first_sample is None and series is None:
        return None

    first_sample = first_sample or {}
    series = series or {}
    inside_current_signed = signed_delta(
        first_sample,
        "rust_inside_current_term_w",
        "oracle_inside_current_term_w",
    )
    inside_history_signed = signed_delta(
        first_sample,
        "rust_inside_history_term_w",
        "oracle_inside_history_term_w",
    )
    outside_current_signed = signed_delta(
        first_sample,
        "rust_outside_current_term_w",
        "oracle_outside_current_term_w",
    )
    outside_history_signed = signed_delta(
        first_sample,
        "rust_outside_history_term_w",
        "oracle_outside_history_term_w",
    )
    return {
        "key": FLOOR_CTF_DRIVER_KEY,
        "construction_name": first_sample.get("construction_name")
        or series.get("construction_name"),
        "area_m2": first_sample.get("area_m2") or series.get("area_m2"),
        "inside_face_temperature_delta_c": first_sample.get(
            "inside_face_temperature_delta_c"
        ),
        "outside_face_temperature_delta_c": first_sample.get(
            "outside_face_temperature_delta_c"
        ),
        "inside_current_signed_delta_w": inside_current_signed,
        "inside_history_signed_delta_w": inside_history_signed,
        "inside_cancellation_residual_w": sum_optional(
            inside_current_signed,
            inside_history_signed,
        ),
        "outside_current_signed_delta_w": outside_current_signed,
        "outside_history_signed_delta_w": outside_history_signed,
        "outside_cancellation_residual_w": sum_optional(
            outside_current_signed,
            outside_history_signed,
        ),
        "inside_current_abs_delta_w": first_sample.get("inside_current_delta_w"),
        "inside_history_abs_delta_w": first_sample.get("inside_history_delta_w"),
        "outside_current_abs_delta_w": first_sample.get("outside_current_delta_w"),
        "outside_history_abs_delta_w": first_sample.get("outside_history_delta_w"),
        "inside_current_rmse_w": nested_numeric(
            series,
            "inside_current_delta",
            "rmse_delta_c",
        ),
        "inside_history_rmse_w": nested_numeric(
            series,
            "inside_history_delta",
            "rmse_delta_c",
        ),
        "outside_current_rmse_w": nested_numeric(
            series,
            "outside_current_delta",
            "rmse_delta_c",
        ),
        "outside_history_rmse_w": nested_numeric(
            series,
            "outside_history_delta",
            "rmse_delta_c",
        ),
    }


def slot_inside_history_temperature_equivalent_delta_c(
    slot: dict[str, Any] | None,
    history_delta_w: float | None,
) -> float | None:
    if slot is None or history_delta_w is None:
        return None
    area = numeric(slot.get("area_m2"))
    coefficient = numeric(slot.get("inside_history_coefficient_w_per_m2_k"))
    if area is None or coefficient is None or area == 0.0 or coefficient == 0.0:
        return None
    return -history_delta_w / (area * coefficient)


def floor_ctf_max_sample_driver_row(summary: dict[str, Any]) -> dict[str, Any] | None:
    inside_solve = first_matching_key(
        summary.get("inside_solve_max_sample_deltas"),
        FLOOR_CTF_DRIVER_KEY,
    )
    slots = sorted(
        matching_key_rows(
            summary.get("ctf_history_max_sample_slots"),
            FLOOR_CTF_DRIVER_KEY,
        ),
        key=lambda row: numeric(row.get("slot_index")) or 0.0,
    )
    if inside_solve is None and not slots:
        return None

    inside_solve = inside_solve or {}
    slot1 = slots[0] if len(slots) >= 1 else None
    slot2 = slots[1] if len(slots) >= 2 else None
    implied_numerator_delta_w = numeric(
        inside_solve.get("implied_solve_numerator_delta_w")
    )
    reference_air_source_delta_w = numeric(
        inside_solve.get("reference_air_source_delta_w")
    )
    outside_temperature_source_delta_w = numeric(
        inside_solve.get("outside_temperature_source_delta_w")
    )
    history_delta_w = numeric(inside_solve.get("inside_history_delta_w"))
    inside_net_longwave_delta_w = numeric(
        inside_solve.get("inside_net_longwave_delta_w")
    )
    tracked_source_delta_w = numeric(
        inside_solve.get("tracked_solve_source_delta_w")
    ) or sum_present(
        [
            reference_air_source_delta_w,
            outside_temperature_source_delta_w,
            history_delta_w,
            inside_net_longwave_delta_w,
        ]
    )
    untracked_source_delta_w = numeric(
        inside_solve.get("solve_source_residual_delta_w")
    )
    if untracked_source_delta_w is None:
        untracked_source_delta_w = (
            implied_numerator_delta_w - tracked_source_delta_w
            if implied_numerator_delta_w is not None
            and tracked_source_delta_w is not None
            else None
        )
    tracked_source_coverage_ratio = numeric(
        inside_solve.get("tracked_solve_source_coverage_ratio")
    ) or ratio(
        tracked_source_delta_w,
        implied_numerator_delta_w,
    )

    return {
        "key": FLOOR_CTF_DRIVER_KEY,
        "construction_name": inside_solve.get("construction_name")
        or (slot1 or {}).get("construction_name"),
        "sample_index": inside_solve.get("sample_index")
        or (slot1 or {}).get("sample_index"),
        "area_m2": inside_solve.get("area_m2") or (slot1 or {}).get("area_m2"),
        "slot_count": len(slots),
        "inside_face_temperature_delta_c": inside_solve.get(
            "inside_face_temperature_delta_c"
        ),
        "inferred_reference_air_temperature_delta_c": inside_solve.get(
            "inferred_reference_air_temperature_delta_c"
        ),
        "implied_solve_numerator_delta_w": inside_solve.get(
            "implied_solve_numerator_delta_w"
        ),
        "reference_air_source_delta_w": inside_solve.get(
            "reference_air_source_delta_w"
        ),
        "outside_temperature_source_delta_w": inside_solve.get(
            "outside_temperature_source_delta_w"
        ),
        "inside_history_delta_w": inside_solve.get("inside_history_delta_w"),
        "rust_inside_history_temperature_term_w": inside_solve.get(
            "rust_inside_history_temperature_term_w"
        ),
        "rust_inside_history_flux_term_w": inside_solve.get(
            "rust_inside_history_flux_term_w"
        ),
        "inside_net_longwave_delta_w": inside_solve.get(
            "inside_net_longwave_delta_w"
        ),
        "tracked_source_delta_w": tracked_source_delta_w,
        "untracked_source_delta_w": untracked_source_delta_w,
        "tracked_source_coverage_ratio": tracked_source_coverage_ratio,
        "reference_air_source_share": numeric(
            inside_solve.get("reference_air_source_share")
        ) or ratio(
            reference_air_source_delta_w,
            implied_numerator_delta_w,
        ),
        "outside_temperature_source_share": numeric(
            inside_solve.get("outside_temperature_source_share")
        ) or ratio(
            outside_temperature_source_delta_w,
            implied_numerator_delta_w,
        ),
        "inside_history_source_share": numeric(
            inside_solve.get("inside_history_source_share")
        ) or ratio(
            history_delta_w,
            implied_numerator_delta_w,
        ),
        "inside_net_longwave_source_share": numeric(
            inside_solve.get("inside_net_longwave_source_share")
        ) or ratio(
            inside_net_longwave_delta_w,
            implied_numerator_delta_w,
        ),
        "untracked_source_share": numeric(
            inside_solve.get("solve_source_residual_share")
        ) or ratio(
            untracked_source_delta_w,
            implied_numerator_delta_w,
        ),
        "slot1_inside_total_term_w": (slot1 or {}).get("inside_total_term_w"),
        "slot1_inside_temperature_term_w": (slot1 or {}).get(
            "inside_temperature_term_w"
        ),
        "slot1_inside_flux_term_w": (slot1 or {}).get("inside_flux_term_w"),
        "slot1_inside_temperature_history_c": (slot1 or {}).get(
            "inside_temperature_history_c"
        ),
        "slot1_outside_temperature_history_c": (slot1 or {}).get(
            "outside_temperature_history_c"
        ),
        "slot1_inside_history_temperature_equivalent_delta_c": (
            slot_inside_history_temperature_equivalent_delta_c(
                slot1,
                history_delta_w,
            )
        ),
        "slot2_inside_total_term_w": (slot2 or {}).get("inside_total_term_w"),
        "slot2_inside_temperature_term_w": (slot2 or {}).get(
            "inside_temperature_term_w"
        ),
        "slot2_inside_flux_term_w": (slot2 or {}).get("inside_flux_term_w"),
        "slot2_inside_temperature_history_c": (slot2 or {}).get(
            "inside_temperature_history_c"
        ),
        "slot2_outside_temperature_history_c": (slot2 or {}).get(
            "outside_temperature_history_c"
        ),
        "slot2_inside_history_temperature_equivalent_delta_c": (
            slot_inside_history_temperature_equivalent_delta_c(
                slot2,
                history_delta_w,
            )
        ),
    }


def metric_identity(metric: dict[str, Any]) -> tuple[str, str]:
    return (str(metric.get("key", "")), str(metric.get("variable", "")))


def numeric(value: Any) -> float | None:
    if isinstance(value, (int, float)):
        return float(value)
    return None


def ratio(numerator: float | None, denominator: float | None) -> float | None:
    if numerator is None or denominator is None or denominator == 0.0:
        return None
    return numerator / denominator


def sample_rows(series: dict[str, Any]) -> list[dict[str, Any]]:
    rows = series.get("sample_rows")
    if not isinstance(rows, list):
        return []
    return [row for row in rows if isinstance(row, dict)]


def sample_numeric(row: dict[str, Any], field: str) -> float | None:
    return numeric(row.get(field))


def residual_stats(values: list[float]) -> dict[str, float | None]:
    if not values:
        return {
            "max_abs_w": None,
            "rmse_w": None,
            "mean_abs_w": None,
        }
    abs_values = [abs(value) for value in values]
    return {
        "max_abs_w": max(abs_values),
        "rmse_w": math.sqrt(sum(value * value for value in values) / len(values)),
        "mean_abs_w": sum(abs_values) / len(abs_values),
    }


def zone_surface_convection_closure_row(summary: dict[str, Any]) -> dict[str, Any]:
    zone_series = find_series(
        summary,
        FocusMetric("ZONE ONE", ZONE_SURFACE_CONVECTION_VARIABLE),
    )
    surface_series = []
    for series in summary.get("series", []):
        if not isinstance(series, dict):
            continue
        output = series.get("output", {})
        if not isinstance(output, dict):
            continue
        if output.get("variable") == SURFACE_INSIDE_CONVECTION_VARIABLE:
            surface_series.append(series)
    surface_series.sort(key=series_output_label)

    if zone_series is None:
        return {
            "status": "missing-zone-row",
            "surface_count": len(surface_series),
            "samples": None,
            "sign_convention": "zone_plus_surface_report_heat_gain",
        }
    if not surface_series:
        return {
            "status": "missing-surface-rows",
            "surface_count": 0,
            "samples": None,
            "sign_convention": "zone_plus_surface_report_heat_gain",
        }

    zone_samples = sample_rows(zone_series)
    surface_samples = [sample_rows(series) for series in surface_series]
    if not zone_samples or any(not rows for rows in surface_samples):
        return {
            "status": "missing-samples",
            "surface_count": len(surface_series),
            "samples": 0,
            "sign_convention": "zone_plus_surface_report_heat_gain",
        }

    sample_window = min(len(zone_samples), *(len(rows) for rows in surface_samples))
    oracle_residuals: list[float] = []
    rust_residuals: list[float] = []
    delta_residuals: list[float] = []

    for sample_index in range(sample_window):
        zone_oracle = sample_numeric(zone_samples[sample_index], "oracle_c")
        zone_rust = sample_numeric(zone_samples[sample_index], "rust_c")
        if zone_oracle is None or zone_rust is None:
            continue

        oracle_surface_sum = 0.0
        rust_surface_sum = 0.0
        sample_ok = True
        for rows in surface_samples:
            surface_oracle = sample_numeric(rows[sample_index], "oracle_c")
            surface_rust = sample_numeric(rows[sample_index], "rust_c")
            if surface_oracle is None or surface_rust is None:
                sample_ok = False
                break
            oracle_surface_sum += surface_oracle
            rust_surface_sum += surface_rust
        if not sample_ok:
            continue

        oracle_residual = zone_oracle + oracle_surface_sum
        rust_residual = zone_rust + rust_surface_sum
        oracle_residuals.append(oracle_residual)
        rust_residuals.append(rust_residual)
        delta_residuals.append(rust_residual - oracle_residual)

    oracle_stats = residual_stats(oracle_residuals)
    rust_stats = residual_stats(rust_residuals)
    delta_stats = residual_stats(delta_residuals)
    status = "computed" if oracle_residuals else "missing-values"
    if oracle_residuals and len(oracle_residuals) != sample_window:
        status = "computed-with-skipped-samples"

    return {
        "status": status,
        "zone_key": "ZONE ONE",
        "zone_variable": ZONE_SURFACE_CONVECTION_VARIABLE,
        "surface_variable": SURFACE_INSIDE_CONVECTION_VARIABLE,
        "surface_count": len(surface_series),
        "samples": len(oracle_residuals),
        "sample_window": sample_window,
        "sign_convention": "zone_plus_surface_report_heat_gain",
        "oracle_closure_max_abs_w": oracle_stats["max_abs_w"],
        "oracle_closure_rmse_w": oracle_stats["rmse_w"],
        "oracle_closure_mean_abs_w": oracle_stats["mean_abs_w"],
        "rust_closure_max_abs_w": rust_stats["max_abs_w"],
        "rust_closure_rmse_w": rust_stats["rmse_w"],
        "rust_closure_mean_abs_w": rust_stats["mean_abs_w"],
        "closure_delta_max_abs_w": delta_stats["max_abs_w"],
        "closure_delta_rmse_w": delta_stats["rmse_w"],
        "closure_delta_mean_abs_w": delta_stats["mean_abs_w"],
    }


def annotate_default_focus_deltas(lanes: list[dict[str, Any]]) -> None:
    default_lane = next((lane for lane in lanes if lane.get("lane") == "default"), None)
    if default_lane is None:
        return

    baselines: dict[tuple[str, str], float] = {}
    for metric in default_lane.get("focus_metrics", []):
        rmse = numeric(metric.get("rmse_delta_c"))
        if rmse is not None:
            baselines[metric_identity(metric)] = rmse

    for lane in lanes:
        for metric in lane.get("focus_metrics", []):
            rmse = numeric(metric.get("rmse_delta_c"))
            baseline = baselines.get(metric_identity(metric))
            if lane.get("lane") == "default" or rmse is None or baseline is None:
                metric["rmse_vs_default"] = None
                metric["rmse_ratio_vs_default"] = None
                continue
            metric["rmse_vs_default"] = rmse - baseline
            metric["rmse_ratio_vs_default"] = rmse / baseline if baseline != 0 else None


def annotate_default_surface_conduction_deltas(
    lanes: list[dict[str, Any]],
    metric_field: str,
) -> None:
    default_lane = next((lane for lane in lanes if lane.get("lane") == "default"), None)
    if default_lane is None:
        return

    baselines: dict[tuple[str, str], float] = {}
    for metric in default_lane.get(metric_field, []):
        rmse = numeric(metric.get("rmse_delta_c"))
        if rmse is not None:
            baselines[metric_identity(metric)] = rmse

    for lane in lanes:
        for metric in lane.get(metric_field, []):
            rmse = numeric(metric.get("rmse_delta_c"))
            baseline = baselines.get(metric_identity(metric))
            if lane.get("lane") == "default" or rmse is None or baseline is None:
                metric["rmse_vs_default"] = None
                metric["rmse_ratio_vs_default"] = None
                continue
            metric["rmse_vs_default"] = rmse - baseline
            metric["rmse_ratio_vs_default"] = rmse / baseline if baseline != 0 else None


def annotate_default_surface_balance_deltas(lanes: list[dict[str, Any]]) -> None:
    default_lane = next((lane for lane in lanes if lane.get("lane") == "default"), None)
    if default_lane is None:
        return

    baseline_rows = {
        str(metric.get("key")): metric
        for metric in default_lane.get("surface_balance_drivers", [])
        if isinstance(metric, dict)
    }
    for lane in lanes:
        for metric in lane.get("surface_balance_drivers", []):
            if not isinstance(metric, dict):
                continue
            baseline = baseline_rows.get(str(metric.get("key")))
            for field in ("storage", "inside", "outside"):
                rmse = numeric(metric.get(f"{field}_rmse_delta_c"))
                baseline_rmse = (
                    numeric(baseline.get(f"{field}_rmse_delta_c"))
                    if isinstance(baseline, dict)
                    else None
                )
                if lane.get("lane") == "default" or rmse is None or baseline_rmse is None:
                    metric[f"{field}_rmse_vs_default"] = None
                    metric[f"{field}_rmse_ratio_vs_default"] = None
                    continue
                metric[f"{field}_rmse_vs_default"] = rmse - baseline_rmse
                metric[f"{field}_rmse_ratio_vs_default"] = (
                    rmse / baseline_rmse if baseline_rmse != 0 else None
                )


def empty_focus_movement_rollup(reference_lane: str | None) -> dict[str, Any]:
    return {
        "reference_lane": reference_lane,
        "improved_focus_count": 0,
        "worsened_focus_count": 0,
        "unchanged_focus_count": 0,
        "missing_focus_count": 0,
        "largest_rmse_improvement": None,
        "largest_rmse_regression": None,
    }


def annotate_reference_focus_movements(lanes: list[dict[str, Any]]) -> None:
    lane_by_name = {str(lane.get("lane")): lane for lane in lanes}
    metrics_by_lane: dict[str, dict[tuple[str, str], dict[str, Any]]] = {}
    for lane in lanes:
        lane_name = str(lane.get("lane"))
        metrics_by_lane[lane_name] = {
            metric_identity(metric): metric
            for metric in lane.get("focus_metrics", [])
            if isinstance(metric, dict)
        }

    for lane in lanes:
        lane_name = str(lane.get("lane"))
        reference_lane = REFERENCE_LANES.get(lane_name)
        lane["reference_lane"] = reference_lane
        lane["focus_movement_vs_reference"] = []
        lane["focus_movement_rollup"] = empty_focus_movement_rollup(reference_lane)

        if reference_lane is None or reference_lane not in lane_by_name:
            continue

        reference_metrics = metrics_by_lane.get(reference_lane, {})
        movements: list[dict[str, Any]] = []
        rollup = empty_focus_movement_rollup(reference_lane)
        largest_improvement: dict[str, Any] | None = None
        largest_regression: dict[str, Any] | None = None

        for metric in lane.get("focus_metrics", []):
            if not isinstance(metric, dict):
                continue
            reference_metric = reference_metrics.get(metric_identity(metric))
            rmse = numeric(metric.get("rmse_delta_c"))
            reference_rmse = (
                numeric(reference_metric.get("rmse_delta_c"))
                if isinstance(reference_metric, dict)
                else None
            )
            movement = {
                "key": metric.get("key"),
                "variable": metric.get("variable"),
                "label": metric.get("label"),
                "rmse_delta_c": rmse,
                "reference_rmse_delta_c": reference_rmse,
                "rmse_vs_reference": None,
                "rmse_ratio_vs_reference": None,
                "direction": "missing",
            }
            if rmse is None or reference_rmse is None:
                rollup["missing_focus_count"] += 1
                movements.append(movement)
                continue

            delta = rmse - reference_rmse
            movement["rmse_vs_reference"] = delta
            movement["rmse_ratio_vs_reference"] = (
                rmse / reference_rmse if reference_rmse != 0 else None
            )
            if delta < -MOVEMENT_EPSILON:
                movement["direction"] = "improved"
                rollup["improved_focus_count"] += 1
                if (
                    largest_improvement is None
                    or delta < numeric(largest_improvement.get("rmse_vs_reference"))
                ):
                    largest_improvement = movement
            elif delta > MOVEMENT_EPSILON:
                movement["direction"] = "worsened"
                rollup["worsened_focus_count"] += 1
                if (
                    largest_regression is None
                    or delta > numeric(largest_regression.get("rmse_vs_reference"))
                ):
                    largest_regression = movement
            else:
                movement["direction"] = "unchanged"
                rollup["unchanged_focus_count"] += 1
            movements.append(movement)

        rollup["largest_rmse_improvement"] = largest_improvement
        rollup["largest_rmse_regression"] = largest_regression
        lane["focus_movement_vs_reference"] = movements
        lane["focus_movement_rollup"] = rollup


def best_focus_metric_rows(lanes: list[dict[str, Any]]) -> list[dict[str, Any]]:
    default_metrics = {
        metric_identity(metric): metric
        for lane in lanes
        if lane.get("lane") == "default"
        for metric in lane.get("focus_metrics", [])
        if isinstance(metric, dict)
    }
    rows = []
    for metric in FOCUS_METRICS:
        identity = (metric.key, metric.variable)
        candidates = []
        for lane in lanes:
            for focus_metric in lane.get("focus_metrics", []):
                if not isinstance(focus_metric, dict):
                    continue
                if metric_identity(focus_metric) != identity:
                    continue
                rmse = numeric(focus_metric.get("rmse_delta_c"))
                if rmse is not None:
                    candidates.append((rmse, lane, focus_metric))
        if not candidates:
            rows.append(
                {
                    "key": metric.key,
                    "variable": metric.variable,
                    "label": f"{metric.key} / {metric.variable}",
                    "best_lane": None,
                    "best_rmse_delta_c": None,
                    "default_rmse_delta_c": None,
                    "rmse_vs_default": None,
                }
            )
            continue

        best_rmse, best_lane, best_metric = min(candidates, key=lambda item: item[0])
        default_rmse = numeric(default_metrics.get(identity, {}).get("rmse_delta_c"))
        rows.append(
            {
                "key": metric.key,
                "variable": metric.variable,
                "label": best_metric.get("label", f"{metric.key} / {metric.variable}"),
                "best_lane": best_lane.get("lane"),
                "best_rmse_delta_c": best_rmse,
                "default_rmse_delta_c": default_rmse,
                "rmse_vs_default": (
                    best_rmse - default_rmse if default_rmse is not None else None
                ),
            }
        )
    return rows


def best_surface_conduction_metric_rows(
    lanes: list[dict[str, Any]],
    metric_field: str,
) -> list[dict[str, Any]]:
    default_metrics = {
        metric_identity(metric): metric
        for lane in lanes
        if lane.get("lane") == "default"
        for metric in lane.get(metric_field, [])
        if isinstance(metric, dict)
    }
    identities = sorted(
        {
            metric_identity(metric)
            for lane in lanes
            for metric in lane.get(metric_field, [])
            if isinstance(metric, dict)
        }
    )

    rows = []
    for identity in identities:
        candidates = []
        for lane in lanes:
            for metric in lane.get(metric_field, []):
                if not isinstance(metric, dict):
                    continue
                if metric_identity(metric) != identity:
                    continue
                rmse = numeric(metric.get("rmse_delta_c"))
                if rmse is not None:
                    candidates.append((rmse, lane, metric))
        if not candidates:
            continue

        best_rmse, best_lane, best_metric = min(candidates, key=lambda item: item[0])
        default_rmse = numeric(default_metrics.get(identity, {}).get("rmse_delta_c"))
        rows.append(
            {
                "key": best_metric.get("key", identity[0]),
                "variable": best_metric.get("variable", identity[1]),
                "label": best_metric.get("label", f"{identity[0]} / {identity[1]}"),
                "best_lane": best_lane.get("lane"),
                "best_rmse_delta_c": best_rmse,
                "default_rmse_delta_c": default_rmse,
                "rmse_vs_default": (
                    best_rmse - default_rmse if default_rmse is not None else None
                ),
            }
        )
    return rows


def lane_row(repo_root: Path, lane: ProbeLane) -> dict[str, Any] | None:
    summary_path = repo_root / lane.summary_path
    if not summary_path.exists():
        return None
    summary = load_json(summary_path)
    bottlenecks = summary.get("bottlenecks", [])
    top = bottlenecks[0] if bottlenecks else {}
    output = top.get("output", {}) if isinstance(top, dict) else {}
    return {
        "lane": lane.lane,
        "path": str(lane.summary_path).replace("\\", "/"),
        "status": summary.get("status"),
        "artifact_status": artifact_status(summary.get("series_count")),
        "zone_air_algorithm": summary.get("zone_air_algorithm", "unknown"),
        "ctf_seed_policy": summary.get("ctf_seed", {}).get("policy", "unknown"),
        "ctf_initial_history_policy": summary.get(
            "ctf_initial_history_policy", "boundary-u-value"
        ),
        "zone_conduction_report_source": summary.get(
            "zone_conduction_report_source", "zone-state"
        ),
        "surface_iteration_count": summary.get("surface_iteration_count", 1),
        "samples": summary.get("samples"),
        "series_count": summary.get("series_count"),
        "top_key": output.get("key", "none"),
        "top_variable": output.get("variable", "none"),
        "top_rmse_delta_c": top.get("rmse_delta_c"),
        "top_max_abs_delta_c": top.get("max_abs_delta_c"),
        "max_abs_delta_c": summary.get("max_abs_delta_c"),
        "rmse_delta_c": summary.get("rmse_delta_c"),
        "focus_metrics": focus_metric_rows(summary),
        "surface_conduction_metrics": surface_conduction_metric_rows(
            summary,
            SURFACE_INSIDE_CONDUCTION_VARIABLE,
        ),
        "surface_outside_conduction_metrics": surface_conduction_metric_rows(
            summary,
            SURFACE_OUTSIDE_CONDUCTION_VARIABLE,
        ),
        "surface_heat_storage_metrics": surface_conduction_metric_rows(
            summary,
            SURFACE_HEAT_STORAGE_VARIABLE,
        ),
        "surface_balance_drivers": surface_balance_driver_rows(summary),
        "zone_surface_convection_closure": zone_surface_convection_closure_row(
            summary
        ),
        "floor_ctf_history_driver": floor_ctf_history_driver_row(summary),
        "floor_ctf_max_sample_driver": floor_ctf_max_sample_driver_row(summary),
    }


def build_summary(repo_root: Path) -> dict[str, Any]:
    lanes = [row for lane in LANES if (row := lane_row(repo_root, lane)) is not None]
    annotate_default_focus_deltas(lanes)
    annotate_default_surface_conduction_deltas(lanes, "surface_conduction_metrics")
    annotate_default_surface_conduction_deltas(lanes, "surface_outside_conduction_metrics")
    annotate_default_surface_conduction_deltas(lanes, "surface_heat_storage_metrics")
    annotate_default_surface_balance_deltas(lanes)
    annotate_reference_focus_movements(lanes)
    return {
        "schema": "rusted-energyplus.dynamic-heat-balance-probe-summary.v17",
        "oracle_version": ORACLE_VERSION,
        "case_id": CASE_ID,
        "expected_series_count": EXPECTED_SERIES_COUNT,
        "lane_count": len(lanes),
        "lanes": lanes,
        "best_focus_metrics": best_focus_metric_rows(lanes),
        "best_surface_conduction_metrics": best_surface_conduction_metric_rows(
            lanes,
            "surface_conduction_metrics",
        ),
        "best_surface_outside_conduction_metrics": best_surface_conduction_metric_rows(
            lanes,
            "surface_outside_conduction_metrics",
        ),
        "best_surface_heat_storage_metrics": best_surface_conduction_metric_rows(
            lanes,
            "surface_heat_storage_metrics",
        ),
    }


def artifact_status(series_count: Any) -> str:
    if not isinstance(series_count, int):
        return "missing-series-count"
    if series_count != EXPECTED_SERIES_COUNT:
        return f"stale-series-count-{series_count}"
    return "current"


def fmt_number(value: Any) -> str:
    if isinstance(value, (int, float)):
        return f"{value:.6f}"
    return "none"


def fmt_signed_number(value: Any) -> str:
    if isinstance(value, (int, float)):
        return f"{value:+.6f}"
    return "none"


def fmt_movement(metric: Any) -> str:
    if not isinstance(metric, dict):
        return "none"
    return "{label} ({delta})".format(
        label=metric.get("label", "none"),
        delta=fmt_signed_number(metric.get("rmse_vs_reference")),
    )


def render_markdown(summary: dict[str, Any]) -> str:
    lines = [
        "# Official Dynamic Heat-Balance Probe Summary",
        "",
        f"case_id: {summary['case_id']}",
        f"oracle_version: {summary['oracle_version']}",
        f"expected_series_count: {summary['expected_series_count']}",
        "",
        "| lane | algorithm | CTF seed | CTF init | zone cond src | surface passes | series | artifact | top output | top RMSE | top max abs | status |",
        "|---|---|---|---|---|---:|---:|---|---|---:|---:|---|",
    ]
    for lane in summary["lanes"]:
        top_output = f"{lane['top_key']} / {lane['top_variable']}"
        lines.append(
            "| {lane} | {algorithm} | {ctf} | {ctf_init} | {zone_cond_src} | {surface_passes} | {series_count} | {artifact_status} | {top} | {rmse} | {max_abs} | {status} |".format(
                lane=lane["lane"],
                algorithm=lane["zone_air_algorithm"],
                ctf=lane["ctf_seed_policy"],
                ctf_init=lane["ctf_initial_history_policy"],
                zone_cond_src=lane["zone_conduction_report_source"],
                surface_passes=lane["surface_iteration_count"],
                series_count=lane.get("series_count") or "none",
                artifact_status=lane.get("artifact_status", "unknown"),
                top=top_output,
                rmse=fmt_number(lane["top_rmse_delta_c"]),
                max_abs=fmt_number(lane["top_max_abs_delta_c"]),
                status=lane["status"],
            )
        )
    lines.extend(
        [
            "",
            "## Probe Interpretation",
            "",
            "RMSE movement is measured against each probe lane's nearest reference lane.",
            "",
            "| lane | reference | improved | worsened | unchanged | largest improvement | largest regression |",
            "|---|---|---:|---:|---:|---|---|",
        ]
    )
    for lane in summary["lanes"]:
        rollup = lane.get("focus_movement_rollup", {})
        if not isinstance(rollup, dict):
            rollup = {}
        lines.append(
            "| {lane} | {reference} | {improved} | {worsened} | {unchanged} | {largest_improvement} | {largest_regression} |".format(
                lane=lane["lane"],
                reference=rollup.get("reference_lane") or "none",
                improved=rollup.get("improved_focus_count", 0),
                worsened=rollup.get("worsened_focus_count", 0),
                unchanged=rollup.get("unchanged_focus_count", 0),
                largest_improvement=fmt_movement(
                    rollup.get("largest_rmse_improvement")
                ),
                largest_regression=fmt_movement(
                    rollup.get("largest_rmse_regression")
                ),
            )
        )
    lines.extend(
        [
            "",
            "## Best Focus Metrics",
            "",
            "| output | best lane | best RMSE | RMSE vs default | default RMSE |",
            "|---|---|---:|---:|---:|",
        ]
    )
    for metric in summary.get("best_focus_metrics", []):
        if not isinstance(metric, dict):
            continue
        lines.append(
            "| {output} | {lane} | {best_rmse} | {vs_default} | {default_rmse} |".format(
                output=metric.get("label", "none"),
                lane=metric.get("best_lane") or "none",
                best_rmse=fmt_number(metric.get("best_rmse_delta_c")),
                vs_default=fmt_signed_number(metric.get("rmse_vs_default")),
                default_rmse=fmt_number(metric.get("default_rmse_delta_c")),
            )
        )
    lines.extend(
        [
            "",
            "## Best Surface Conduction Metrics",
            "",
            "| surface output | best lane | best RMSE | RMSE vs default | default RMSE |",
            "|---|---|---:|---:|---:|",
        ]
    )
    for metric in summary.get("best_surface_conduction_metrics", []):
        if not isinstance(metric, dict):
            continue
        lines.append(
            "| {output} | {lane} | {best_rmse} | {vs_default} | {default_rmse} |".format(
                output=metric.get("label", "none"),
                lane=metric.get("best_lane") or "none",
                best_rmse=fmt_number(metric.get("best_rmse_delta_c")),
                vs_default=fmt_signed_number(metric.get("rmse_vs_default")),
                default_rmse=fmt_number(metric.get("default_rmse_delta_c")),
            )
        )
    lines.extend(
        [
            "",
            "## Best Surface Outside Conduction Metrics",
            "",
            "| surface output | best lane | best RMSE | RMSE vs default | default RMSE |",
            "|---|---|---:|---:|---:|",
        ]
    )
    for metric in summary.get("best_surface_outside_conduction_metrics", []):
        if not isinstance(metric, dict):
            continue
        lines.append(
            "| {output} | {lane} | {best_rmse} | {vs_default} | {default_rmse} |".format(
                output=metric.get("label", "none"),
                lane=metric.get("best_lane") or "none",
                best_rmse=fmt_number(metric.get("best_rmse_delta_c")),
                vs_default=fmt_signed_number(metric.get("rmse_vs_default")),
                default_rmse=fmt_number(metric.get("default_rmse_delta_c")),
            )
        )
    lines.extend(
        [
            "",
            "## Best Surface Heat Storage Metrics",
            "",
            "| surface output | best lane | best RMSE | RMSE vs default | default RMSE |",
            "|---|---|---:|---:|---:|",
        ]
    )
    for metric in summary.get("best_surface_heat_storage_metrics", []):
        if not isinstance(metric, dict):
            continue
        lines.append(
            "| {output} | {lane} | {best_rmse} | {vs_default} | {default_rmse} |".format(
                output=metric.get("label", "none"),
                lane=metric.get("best_lane") or "none",
                best_rmse=fmt_number(metric.get("best_rmse_delta_c")),
                vs_default=fmt_signed_number(metric.get("rmse_vs_default")),
                default_rmse=fmt_number(metric.get("default_rmse_delta_c")),
            )
        )
    lines.extend(
        [
            "",
            "## Surface Conduction Balance Drivers",
            "",
            "Top per-surface storage rows paired with their inside/outside-face conduction and temperature RMSE. The amplification columns divide W-rate RMSE by face-temperature RMSE, which highlights CTF cases where a small temperature miss is being multiplied into a large storage/conduction delta.",
            "",
            "| lane | rank | surface | storage RMSE | storage vs default | inside RMSE | outside RMSE | inside temp RMSE | outside temp RMSE | storage/inside temp | storage/outside temp | dominant leg | status |",
            "|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---|---|",
        ]
    )
    for lane in summary["lanes"]:
        for rank, metric in enumerate(
            lane.get("surface_balance_drivers", [])[:SURFACE_DRIVER_LIMIT],
            start=1,
        ):
            lines.append(
                "| {lane} | {rank} | {surface} | {storage_rmse} | {storage_vs_default} | {inside_rmse} | {outside_rmse} | {inside_temp_rmse} | {outside_temp_rmse} | {storage_per_inside_temp} | {storage_per_outside_temp} | {dominant_leg} | {status} |".format(
                    lane=lane["lane"],
                    rank=rank,
                    surface=metric["label"],
                    storage_rmse=fmt_number(metric.get("storage_rmse_delta_c")),
                    storage_vs_default=fmt_signed_number(
                        metric.get("storage_rmse_vs_default")
                    ),
                    inside_rmse=fmt_number(metric.get("inside_rmse_delta_c")),
                    outside_rmse=fmt_number(metric.get("outside_rmse_delta_c")),
                    inside_temp_rmse=fmt_number(
                        metric.get("inside_temperature_rmse_delta_c")
                    ),
                    outside_temp_rmse=fmt_number(
                        metric.get("outside_temperature_rmse_delta_c")
                    ),
                    storage_per_inside_temp=fmt_number(
                        metric.get("storage_rmse_per_inside_temperature_rmse")
                    ),
                    storage_per_outside_temp=fmt_number(
                        metric.get("storage_rmse_per_outside_temperature_rmse")
                    ),
                    dominant_leg=metric.get("dominant_leg", "none"),
                    status=metric.get("status", "none"),
                )
            )
    lines.extend(
        [
            "",
            "## Zone Surface Convection Report Closure",
            "",
            "Compares `Zone Air Heat Balance Surface Convection Rate` with the signed sum of `Surface Inside Face Convection Heat Gain Rate` rows using `zone + surface_sum`. Nonzero oracle closure means EnergyPlus zone AirRpt uses a different report timing/source than the individual surface report rows.",
            "",
            "| lane | surfaces | samples | oracle RMSE | Rust RMSE | delta RMSE | oracle max | Rust max | delta max | oracle mean | Rust mean | delta mean | status |",
            "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|",
        ]
    )
    for lane in summary["lanes"]:
        closure = lane.get("zone_surface_convection_closure")
        if not isinstance(closure, dict):
            continue
        lines.append(
            "| {lane} | {surfaces} | {samples} | {oracle_rmse} | {rust_rmse} | {delta_rmse} | {oracle_max} | {rust_max} | {delta_max} | {oracle_mean} | {rust_mean} | {delta_mean} | {status} |".format(
                lane=lane["lane"],
                surfaces=closure.get("surface_count", "none"),
                samples=closure.get("samples", "none"),
                oracle_rmse=fmt_number(closure.get("oracle_closure_rmse_w")),
                rust_rmse=fmt_number(closure.get("rust_closure_rmse_w")),
                delta_rmse=fmt_number(closure.get("closure_delta_rmse_w")),
                oracle_max=fmt_number(closure.get("oracle_closure_max_abs_w")),
                rust_max=fmt_number(closure.get("rust_closure_max_abs_w")),
                delta_max=fmt_number(closure.get("closure_delta_max_abs_w")),
                oracle_mean=fmt_number(closure.get("oracle_closure_mean_abs_w")),
                rust_mean=fmt_number(closure.get("rust_closure_mean_abs_w")),
                delta_mean=fmt_number(closure.get("closure_delta_mean_abs_w")),
                status=closure.get("status", "none"),
            )
        )
    lines.extend(
        [
            "",
            "## Floor CTF History Cancellation Drivers",
            "",
            "Signed first-sample current/history deltas show whether large CTF current and history misses cancel into the reported conduction residual. Annual RMSE columns keep the latent current/history mismatch visible even when the reported storage row partially cancels.",
            "",
            "| lane | in temp abs C | out temp abs C | in current dW | in history dW | in residual W | out current dW | out history dW | out residual W | in current RMSE | in history RMSE | out current RMSE | out history RMSE |",
            "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for lane in summary["lanes"]:
        driver = lane.get("floor_ctf_history_driver")
        if not isinstance(driver, dict):
            continue
        lines.append(
            "| {lane} | {in_temp} | {out_temp} | {in_current} | {in_history} | {in_residual} | {out_current} | {out_history} | {out_residual} | {in_current_rmse} | {in_history_rmse} | {out_current_rmse} | {out_history_rmse} |".format(
                lane=lane["lane"],
                in_temp=fmt_number(driver.get("inside_face_temperature_delta_c")),
                out_temp=fmt_number(driver.get("outside_face_temperature_delta_c")),
                in_current=fmt_signed_number(
                    driver.get("inside_current_signed_delta_w")
                ),
                in_history=fmt_signed_number(
                    driver.get("inside_history_signed_delta_w")
                ),
                in_residual=fmt_signed_number(
                    driver.get("inside_cancellation_residual_w")
                ),
                out_current=fmt_signed_number(
                    driver.get("outside_current_signed_delta_w")
                ),
                out_history=fmt_signed_number(
                    driver.get("outside_history_signed_delta_w")
                ),
                out_residual=fmt_signed_number(
                    driver.get("outside_cancellation_residual_w")
                ),
                in_current_rmse=fmt_number(driver.get("inside_current_rmse_w")),
                in_history_rmse=fmt_number(driver.get("inside_history_rmse_w")),
                out_current_rmse=fmt_number(driver.get("outside_current_rmse_w")),
                out_history_rmse=fmt_number(driver.get("outside_history_rmse_w")),
            )
        )
    lines.extend(
        [
            "",
            "## Floor CTF Max-Sample Drivers",
            "",
            "Max-sample solve/source rows keep the active floor storage bottleneck visible after the first-sample history cancellation has settled. Tracked source coverage sums the currently decomposed numerator deltas: reference air, outside-temperature source, CTF history, and inside longwave. The untracked residual mostly represents still-unsplit damping/source-order effects and is the next place to add source probes when coverage is low.",
            "",
            "| lane | sample | Tin dC | ref air dC | numerator dW | tracked dW | coverage | untracked dW | history share | ref-air share | LW share | outside share | history dW | ref air dW | LW dW | history temp W | history flux W | slots | slot1 total W | slot1 Tin C | slot1 equiv dC | slot2 total W | slot2 Tin C | slot2 equiv dC |",
            "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for lane in summary["lanes"]:
        driver = lane.get("floor_ctf_max_sample_driver")
        if not isinstance(driver, dict):
            continue
        lines.append(
            "| {lane} | {sample} | {in_temp} | {ref_air_temp} | {numerator} | {tracked} | {coverage} | {untracked} | {history_share} | {ref_air_share} | {lw_share} | {outside_share} | {history} | {ref_air} | {lw} | {history_temp} | {history_flux} | {slots} | {slot1_total} | {slot1_tin} | {slot1_equiv} | {slot2_total} | {slot2_tin} | {slot2_equiv} |".format(
                lane=lane["lane"],
                sample=driver.get("sample_index") or "none",
                in_temp=fmt_number(driver.get("inside_face_temperature_delta_c")),
                ref_air_temp=fmt_number(
                    driver.get("inferred_reference_air_temperature_delta_c")
                ),
                numerator=fmt_number(driver.get("implied_solve_numerator_delta_w")),
                tracked=fmt_number(driver.get("tracked_source_delta_w")),
                coverage=fmt_number(driver.get("tracked_source_coverage_ratio")),
                untracked=fmt_signed_number(driver.get("untracked_source_delta_w")),
                history_share=fmt_number(driver.get("inside_history_source_share")),
                ref_air_share=fmt_number(driver.get("reference_air_source_share")),
                lw_share=fmt_number(driver.get("inside_net_longwave_source_share")),
                outside_share=fmt_number(
                    driver.get("outside_temperature_source_share")
                ),
                ref_air=fmt_number(driver.get("reference_air_source_delta_w")),
                history=fmt_number(driver.get("inside_history_delta_w")),
                history_temp=fmt_number(
                    driver.get("rust_inside_history_temperature_term_w")
                ),
                history_flux=fmt_number(
                    driver.get("rust_inside_history_flux_term_w")
                ),
                lw=fmt_number(driver.get("inside_net_longwave_delta_w")),
                slots=driver.get("slot_count") or "none",
                slot1_total=fmt_number(driver.get("slot1_inside_total_term_w")),
                slot1_tin=fmt_number(
                    driver.get("slot1_inside_temperature_history_c")
                ),
                slot1_equiv=fmt_signed_number(
                    driver.get("slot1_inside_history_temperature_equivalent_delta_c")
                ),
                slot2_total=fmt_number(driver.get("slot2_inside_total_term_w")),
                slot2_tin=fmt_number(
                    driver.get("slot2_inside_temperature_history_c")
                ),
                slot2_equiv=fmt_signed_number(
                    driver.get("slot2_inside_history_temperature_equivalent_delta_c")
                ),
            )
        )
    lines.extend(
        [
            "",
            "## Surface Inside Conduction Drivers",
            "",
            "Top inside-face conduction RMSE rows per lane. These rows decompose the zone aggregate conduction signal so aggregate cancellation or regression can be traced back to surfaces.",
            "",
            "| lane | rank | surface output | RMSE | RMSE vs default | ratio | max abs | mean abs | status |",
            "|---|---:|---|---:|---:|---:|---:|---:|---|",
        ]
    )
    for lane in summary["lanes"]:
        for rank, metric in enumerate(
            lane.get("surface_conduction_metrics", [])[:SURFACE_DRIVER_LIMIT],
            start=1,
        ):
            lines.append(
                "| {lane} | {rank} | {output} | {rmse} | {vs_default} | {ratio} | {max_abs} | {mean_abs} | {status} |".format(
                    lane=lane["lane"],
                    rank=rank,
                    output=metric["label"],
                    rmse=fmt_number(metric["rmse_delta_c"]),
                    vs_default=fmt_signed_number(metric.get("rmse_vs_default")),
                    ratio=fmt_number(metric.get("rmse_ratio_vs_default")),
                    max_abs=fmt_number(metric["max_abs_delta_c"]),
                    mean_abs=fmt_number(metric["mean_abs_delta_c"]),
                    status=metric["status"],
                )
            )
    lines.extend(
        [
            "",
            "## Surface Outside Conduction Drivers",
            "",
            "Top outside-face conduction RMSE rows per lane. These rows decompose the outside aggregate conduction signal so exterior boundary/source/history regressions can be traced back to surfaces.",
            "",
            "| lane | rank | surface output | RMSE | RMSE vs default | ratio | max abs | mean abs | status |",
            "|---|---:|---|---:|---:|---:|---:|---:|---|",
        ]
    )
    for lane in summary["lanes"]:
        for rank, metric in enumerate(
            lane.get("surface_outside_conduction_metrics", [])[:SURFACE_DRIVER_LIMIT],
            start=1,
        ):
            lines.append(
                "| {lane} | {rank} | {output} | {rmse} | {vs_default} | {ratio} | {max_abs} | {mean_abs} | {status} |".format(
                    lane=lane["lane"],
                    rank=rank,
                    output=metric["label"],
                    rmse=fmt_number(metric["rmse_delta_c"]),
                    vs_default=fmt_signed_number(metric.get("rmse_vs_default")),
                    ratio=fmt_number(metric.get("rmse_ratio_vs_default")),
                    max_abs=fmt_number(metric["max_abs_delta_c"]),
                    mean_abs=fmt_number(metric["mean_abs_delta_c"]),
                    status=metric["status"],
                )
            )
    lines.extend(
        [
            "",
            "## Focus Metrics",
            "",
            "| lane | output | RMSE | RMSE vs default | ratio | max abs | mean abs | status |",
            "|---|---|---:|---:|---:|---:|---:|---|",
        ]
    )
    for lane in summary["lanes"]:
        for metric in lane.get("focus_metrics", []):
            lines.append(
                "| {lane} | {output} | {rmse} | {vs_default} | {ratio} | {max_abs} | {mean_abs} | {status} |".format(
                    lane=lane["lane"],
                    output=metric["label"],
                    rmse=fmt_number(metric["rmse_delta_c"]),
                    vs_default=fmt_number(metric.get("rmse_vs_default")),
                    ratio=fmt_number(metric.get("rmse_ratio_vs_default")),
                    max_abs=fmt_number(metric["max_abs_delta_c"]),
                    mean_abs=fmt_number(metric["mean_abs_delta_c"]),
                    status=metric["status"],
                )
            )
    lines.append("")
    return "\n".join(lines)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Summarize official dynamic heat-balance diagnostic probe lanes."
    )
    parser.add_argument(
        "--repo-root",
        type=Path,
        default=Path.cwd(),
        help="Repository root containing .runtime artifacts.",
    )
    parser.add_argument("--json-output", type=Path, default=None)
    parser.add_argument("--markdown-output", type=Path, default=None)
    return parser.parse_args()


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    summary = build_summary(repo_root)
    if not summary["lanes"]:
        print("No official dynamic heat-balance probe summaries found.")
        return 1

    markdown = render_markdown(summary)
    if args.json_output:
        write_text(args.json_output, json.dumps(summary, indent=2) + "\n")
    if args.markdown_output:
        write_text(args.markdown_output, markdown)
    print(markdown)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
