#!/usr/bin/env python3
"""Validate the exact EnergyPlus 26.1 psychrometric routine inventory."""

from __future__ import annotations

import argparse
import copy
import re
import sys
import tomllib
from pathlib import Path
from typing import Any


INVENTORY_ID = "psychrometric_routine_inventory"
SOURCE_MAP_PATH = "docs/src/porting-map/psychrometrics-source-map.md"
SOURCE_CC = "src/EnergyPlus/Psychrometrics.cc"
SOURCE_HH = "src/EnergyPlus/Psychrometrics.hh"

# EnergyPlus 26.1 Psychrometrics.hh interface order. Conditional cached and
# no-cache implementations are one logical routine; the default build owner is
# recorded for cached wrappers.
EXPECTED_ROUTINES: tuple[tuple[str, str], ...] = (
    (SOURCE_CC, "InitializePsychRoutines"),
    (SOURCE_CC, "ShowPsychrometricSummary"),
    (SOURCE_CC, "PsyRhoAirFnPbTdbW_error"),
    (SOURCE_HH, "PsyRhoAirFnPbTdbW"),
    (SOURCE_HH, "PsyRhoAirFnPbTdbW_fast"),
    (SOURCE_HH, "PsyHfgAirFnWTdb"),
    (SOURCE_HH, "PsyHgAirFnWTdb"),
    (SOURCE_HH, "PsyHFnTdbW"),
    (SOURCE_HH, "PsyHFnTdbW_fast"),
    (SOURCE_HH, "PsyCpAirFnW"),
    (SOURCE_HH, "PsyCpAirFnW_fast"),
    (SOURCE_HH, "PsyTdbFnHW"),
    (SOURCE_HH, "PsyRhovFnTdbRhLBnd0C"),
    (SOURCE_HH, "PsyRhovFnTdbWPb"),
    (SOURCE_HH, "PsyRhovFnTdbWPb_fast"),
    (SOURCE_CC, "PsyRhFnTdbRhovLBnd0C_error"),
    (SOURCE_HH, "PsyRhFnTdbRhovLBnd0C"),
    (SOURCE_CC, "PsyTwbFnTdbWPb"),
    (SOURCE_CC, "PsyTwbFnTdbWPb_raw"),
    (SOURCE_CC, "PsyVFnTdbWPb_error"),
    (SOURCE_HH, "PsyVFnTdbWPb"),
    (SOURCE_CC, "PsyWFnTdbH_error"),
    (SOURCE_HH, "PsyWFnTdbH"),
    (SOURCE_CC, "PsyPsatFnTemp_raw"),
    (SOURCE_HH, "PsyPsatFnTemp"),
    (SOURCE_CC, "PsyTsatFnHPb_raw"),
    (SOURCE_HH, "PsyTsatFnHPb"),
    (SOURCE_HH, "PsyRhovFnTdbRh"),
    (SOURCE_CC, "PsyRhFnTdbRhov_error"),
    (SOURCE_HH, "PsyRhFnTdbRhov"),
    (SOURCE_CC, "PsyRhFnTdbWPb_error"),
    (SOURCE_HH, "PsyRhFnTdbWPb"),
    (SOURCE_CC, "PsyWFnTdpPb_error"),
    (SOURCE_HH, "PsyWFnTdpPb"),
    (SOURCE_CC, "PsyWFnTdbRhPb_error"),
    (SOURCE_HH, "PsyWFnTdbRhPb"),
    (SOURCE_CC, "PsyWFnTdbTwbPb_temperature_error"),
    (SOURCE_CC, "PsyWFnTdbTwbPb_humidity_error"),
    (SOURCE_HH, "PsyWFnTdbTwbPb"),
    (SOURCE_HH, "PsyHFnTdbRhPb"),
    (SOURCE_CC, "PsyTsatFnPb_raw"),
    (SOURCE_HH, "PsyTsatFnPb"),
    (SOURCE_HH, "PsyTdpFnWPb"),
    (SOURCE_CC, "PsyTdpFnTdbTwbPb_error"),
    (SOURCE_HH, "PsyTdpFnTdbTwbPb"),
    (SOURCE_HH, "F6"),
    (SOURCE_HH, "F7"),
    (SOURCE_HH, "CPCW"),
    (SOURCE_HH, "CPHW"),
    (SOURCE_HH, "RhoH2O"),
    (SOURCE_HH, "PsyDeltaHSenFnTdb2Tdb1W"),
    (SOURCE_HH, "PsyDeltaHSenFnTdb2W2Tdb1W1"),
    (SOURCE_CC, "CSplineint"),
)

STATE_MAPPED_ROUTINES = frozenset(
    {
        "PsyRhoAirFnPbTdbW",
        "PsyRhoAirFnPbTdbW_fast",
        "PsyHfgAirFnWTdb",
        "PsyHgAirFnWTdb",
        "PsyHFnTdbW",
        "PsyHFnTdbW_fast",
        "PsyCpAirFnW",
        "PsyCpAirFnW_fast",
        "PsyTdbFnHW",
        "PsyRhovFnTdbRhLBnd0C",
        "PsyRhovFnTdbWPb",
        "PsyRhovFnTdbWPb_fast",
        "PsyRhFnTdbRhovLBnd0C",
        "PsyVFnTdbWPb",
        "PsyWFnTdbH",
        "PsyPsatFnTemp_raw",
        "PsyRhovFnTdbRh",
        "PsyRhFnTdbRhov",
        "PsyRhFnTdbWPb",
    }
)
EXPECTED_STATUS_COUNTS = {"source_mapped": 34, "state_mapped": 19}


def expected_completion_status(source_routine: str) -> str:
    return (
        "state_mapped"
        if source_routine in STATE_MAPPED_ROUTINES
        else "source_mapped"
    )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate the exact EnergyPlus 26.1 psychrometric routine inventory."
    )
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--self-test", action="store_true")
    return parser.parse_args()


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def inventory_record(spec: dict[str, Any], errors: list[str]) -> dict[str, Any] | None:
    algorithms = spec.get("algorithm", [])
    if not isinstance(algorithms, list):
        errors.append("algorithm ledger 'algorithm' value must be a list")
        return None

    matches = [
        algorithm
        for algorithm in algorithms
        if isinstance(algorithm, dict) and algorithm.get("id") == INVENTORY_ID
    ]
    if len(matches) != 1:
        errors.append(f"expected exactly one {INVENTORY_ID!r} algorithm record; found {len(matches)}")
        return None
    return matches[0]


def source_map_inventory(source_map_text: str) -> list[tuple[int, str]]:
    """Extract numbered logical-identifier rows from the checked-in source map."""

    row_pattern = re.compile(
        r"^\|\s*(\d+)\s*\|\s*`([A-Za-z_][A-Za-z0-9_]*)`\s*\|",
        re.MULTILINE,
    )
    return [
        (int(match.group(1)), match.group(2))
        for match in row_pattern.finditer(source_map_text)
    ]


def validate_psychrometric_inventory(
    spec: dict[str, Any],
    source_map_text: str,
) -> list[str]:
    """Return exact-inventory contract violations in deterministic order."""

    errors: list[str] = []
    algorithm = inventory_record(spec, errors)
    if algorithm is None:
        return errors

    if algorithm.get("domain") != "psychrometrics":
        errors.append(f"{INVENTORY_ID}: domain must be exactly 'psychrometrics'")
    if algorithm.get("source_map") != SOURCE_MAP_PATH:
        errors.append(
            f"{INVENTORY_ID}: source_map must be exactly {SOURCE_MAP_PATH!r}"
        )

    expected_map = [
        (index, source_routine)
        for index, (_, source_routine) in enumerate(EXPECTED_ROUTINES, start=1)
    ]
    actual_map = source_map_inventory(source_map_text)
    if len(actual_map) != len(expected_map):
        errors.append(
            f"{INVENTORY_ID}: source map must contain exactly "
            f"{len(expected_map)} numbered routine rows; found {len(actual_map)}"
        )
    maximum_map_rows = max(len(actual_map), len(expected_map))
    for index in range(maximum_map_rows):
        position = index + 1
        if index >= len(expected_map):
            errors.append(
                f"{INVENTORY_ID}: unexpected source-map row at position "
                f"{position}: {actual_map[index]!r}"
            )
            continue
        expected_row = expected_map[index]
        if index >= len(actual_map):
            errors.append(
                f"{INVENTORY_ID}: missing source-map row at position "
                f"{position}: {expected_row[1]!r}"
            )
            continue
        if actual_map[index] != expected_row:
            errors.append(
                f"{INVENTORY_ID}: source-map position {position} must be "
                f"{expected_row!r}; found {actual_map[index]!r}"
            )

    routines = algorithm.get("routine")
    if not isinstance(routines, dict):
        errors.append(f"{INVENTORY_ID}: routine must be a TOML table")
        return errors

    actual = list(routines.items())
    if len(actual) != len(EXPECTED_ROUTINES):
        errors.append(
            f"{INVENTORY_ID}: expected exactly {len(EXPECTED_ROUTINES)} routines; found {len(actual)}"
        )

    maximum = max(len(actual), len(EXPECTED_ROUTINES))
    for index in range(maximum):
        position = index + 1
        if index >= len(EXPECTED_ROUTINES):
            routine_id, row = actual[index]
            source_routine = row.get("source_routine") if isinstance(row, dict) else None
            errors.append(
                f"{INVENTORY_ID}: unexpected routine at position {position}: "
                f"{routine_id!r} ({source_routine!r})"
            )
            continue
        expected_file, expected_routine = EXPECTED_ROUTINES[index]
        if index >= len(actual):
            errors.append(
                f"{INVENTORY_ID}: missing routine at position {position}: {expected_routine!r}"
            )
            continue

        routine_id, row = actual[index]
        if not isinstance(row, dict):
            errors.append(
                f"{INVENTORY_ID}: routine {routine_id!r} at position {position} must be a TOML table"
            )
            continue

        if row.get("source_routine") != expected_routine:
            errors.append(
                f"{INVENTORY_ID}: position {position} source_routine must be "
                f"{expected_routine!r}; found {row.get('source_routine')!r}"
            )
        if row.get("source_file") != expected_file:
            errors.append(
                f"{INVENTORY_ID}: {expected_routine} source_file must be "
                f"{expected_file!r}; found {row.get('source_file')!r}"
            )
        expected_status = expected_completion_status(expected_routine)
        if row.get("completion_status") != expected_status:
            errors.append(
                f"{INVENTORY_ID}: {expected_routine} completion_status must be exactly "
                f"{expected_status!r}; found {row.get('completion_status')!r}"
            )
        if row.get("required_for_full_domain") is not False:
            errors.append(
                f"{INVENTORY_ID}: {expected_routine} required_for_full_domain must be false"
            )

    actual_status_counts = {
        status: sum(
            1
            for _, row in actual
            if isinstance(row, dict) and row.get("completion_status") == status
        )
        for status in EXPECTED_STATUS_COUNTS
    }
    if actual_status_counts != EXPECTED_STATUS_COUNTS:
        errors.append(
            f"{INVENTORY_ID}: completion_status counts must be exactly "
            f"{EXPECTED_STATUS_COUNTS!r}; found {actual_status_counts!r}"
        )

    return errors


def happy_path_spec() -> dict[str, Any]:
    routines: dict[str, dict[str, Any]] = {}
    for index, (source_file, source_routine) in enumerate(EXPECTED_ROUTINES, start=1):
        routines[f"routine_{index:02d}"] = {
            "source_file": source_file,
            "source_routine": source_routine,
            "completion_status": expected_completion_status(source_routine),
            "required_for_full_domain": False,
        }
    return {
        "algorithm": [
            {
                "id": INVENTORY_ID,
                "domain": "psychrometrics",
                "source_map": SOURCE_MAP_PATH,
                "routine": routines,
            }
        ]
    }


def happy_path_source_map() -> str:
    return "\n".join(
        f"| {index} | `{source_routine}` | test obligation |"
        for index, (_, source_routine) in enumerate(EXPECTED_ROUTINES, start=1)
    )


def self_test_inventory() -> int:
    baseline = happy_path_spec()
    baseline_source_map = happy_path_source_map()
    passed: list[str] = []

    def routines(candidate: dict[str, Any]) -> dict[str, dict[str, Any]]:
        return candidate["algorithm"][0]["routine"]

    def find_key(candidate: dict[str, Any], source_routine: str) -> str:
        for key, row in routines(candidate).items():
            if row.get("source_routine") == source_routine:
                return key
        raise KeyError(source_routine)

    def expect_invalid(
        name: str,
        candidate: dict[str, Any],
        token: str,
        source_map_text: str = baseline_source_map,
    ) -> None:
        errors = validate_psychrometric_inventory(candidate, source_map_text)
        if not any(token in error for error in errors):
            raise AssertionError(f"{name}: expected error containing {token!r}; got {errors}")
        passed.append(name)

    baseline_errors = validate_psychrometric_inventory(baseline, baseline_source_map)
    if baseline_errors:
        raise AssertionError(f"happy_path: expected no errors; got {baseline_errors}")
    passed.append("happy_path")

    candidate = copy.deepcopy(baseline)
    del routines(candidate)[find_key(candidate, "F7")]
    expect_invalid("missing_f7", candidate, "expected exactly 53 routines")

    candidate = copy.deepcopy(baseline)
    ordered = list(routines(candidate).items())
    ordered[45], ordered[46] = ordered[46], ordered[45]
    candidate["algorithm"][0]["routine"] = dict(ordered)
    expect_invalid("order_swap", candidate, "position 46")

    candidate = copy.deepcopy(baseline)
    routines(candidate)[find_key(candidate, "F7")]["source_file"] = SOURCE_CC
    expect_invalid("wrong_source_file", candidate, "F7 source_file")

    candidate = copy.deepcopy(baseline)
    routines(candidate)[find_key(candidate, "F7")]["source_routine"] = "F8"
    expect_invalid("wrong_source_routine", candidate, "source_routine must be 'F7'")

    candidate = copy.deepcopy(baseline)
    routines(candidate)[find_key(candidate, "F7")]["completion_status"] = "implemented"
    expect_invalid("wrong_completion_status", candidate, "completion_status must be exactly")

    candidate = copy.deepcopy(baseline)
    routines(candidate)[find_key(candidate, "PsyRhoAirFnPbTdbW")][
        "completion_status"
    ] = "source_mapped"
    expect_invalid(
        "state_mapped_routine_downgrade",
        candidate,
        "PsyRhoAirFnPbTdbW completion_status must be exactly 'state_mapped'",
    )

    for name, source_routine in (
        ("rho_fast_state_mapped_downgrade", "PsyRhoAirFnPbTdbW_fast"),
        ("hfg_state_mapped_downgrade", "PsyHfgAirFnWTdb"),
        ("hg_state_mapped_downgrade", "PsyHgAirFnWTdb"),
        ("h_state_mapped_downgrade", "PsyHFnTdbW"),
        ("h_fast_state_mapped_downgrade", "PsyHFnTdbW_fast"),
        ("cp_fast_state_mapped_downgrade", "PsyCpAirFnW_fast"),
        ("tdb_state_mapped_downgrade", "PsyTdbFnHW"),
        ("rhov_rh_lbnd0c_state_mapped_downgrade", "PsyRhovFnTdbRhLBnd0C"),
        ("rhov_w_pb_state_mapped_downgrade", "PsyRhovFnTdbWPb"),
        ("rhov_w_pb_fast_state_mapped_downgrade", "PsyRhovFnTdbWPb_fast"),
        ("rh_lbnd0c_state_mapped_downgrade", "PsyRhFnTdbRhovLBnd0C"),
        ("v_state_mapped_downgrade", "PsyVFnTdbWPb"),
        ("w_tdb_h_state_mapped_downgrade", "PsyWFnTdbH"),
        ("psat_raw_state_mapped_downgrade", "PsyPsatFnTemp_raw"),
        ("rhov_rh_state_mapped_downgrade", "PsyRhovFnTdbRh"),
        ("rh_rhov_state_mapped_downgrade", "PsyRhFnTdbRhov"),
        ("rh_w_pb_state_mapped_downgrade", "PsyRhFnTdbWPb"),
    ):
        candidate = copy.deepcopy(baseline)
        routines(candidate)[find_key(candidate, source_routine)][
            "completion_status"
        ] = "source_mapped"
        expect_invalid(
            name,
            candidate,
            f"{source_routine} completion_status must be exactly 'state_mapped'",
        )

    candidate = copy.deepcopy(baseline)
    routines(candidate)[find_key(candidate, "F7")]["completion_status"] = "state_mapped"
    expect_invalid(
        "source_mapped_routine_promotion",
        candidate,
        "F7 completion_status must be exactly 'source_mapped'",
    )

    candidate = copy.deepcopy(baseline)
    routines(candidate)[find_key(candidate, "PsyCpAirFnW")][
        "completion_status"
    ] = "source_mapped"
    routines(candidate)[find_key(candidate, "F7")]["completion_status"] = "state_mapped"
    expect_invalid(
        "status_swap_preserves_counts",
        candidate,
        "PsyCpAirFnW completion_status must be exactly 'state_mapped'",
    )

    candidate = copy.deepcopy(baseline)
    routines(candidate)[find_key(candidate, "F7")]["required_for_full_domain"] = True
    expect_invalid("required_for_full_domain_true", candidate, "required_for_full_domain must be false")

    source_map_lines = baseline_source_map.splitlines()
    missing_f7_source_map = "\n".join(
        line for line in source_map_lines if "`F7`" not in line
    )
    expect_invalid(
        "source_map_missing_f7",
        baseline,
        "source map must contain exactly 53 numbered routine rows",
        missing_f7_source_map,
    )

    swapped_source_map_lines = list(source_map_lines)
    swapped_source_map_lines[45], swapped_source_map_lines[46] = (
        swapped_source_map_lines[46],
        swapped_source_map_lines[45],
    )
    expect_invalid(
        "source_map_order_swap",
        baseline,
        "source-map position 46",
        "\n".join(swapped_source_map_lines),
    )

    candidate = copy.deepcopy(baseline)
    candidate["algorithm"][0]["source_map"] = "docs/src/porting-map/wrong.md"
    expect_invalid("wrong_source_map_path", candidate, "source_map must be exactly")

    print("Psychrometric routine inventory validator self-tests")
    print(f"  passed: {len(passed)}")
    for name in passed:
        print(f"  - {name}")
    return 0


def main() -> int:
    args = parse_args()
    if args.self_test:
        return self_test_inventory()

    repo_root = args.repo_root.resolve()
    ledger_path = repo_root / "specs" / "algorithm_ledger.toml"
    source_map_path = repo_root / SOURCE_MAP_PATH
    try:
        spec = load_toml(ledger_path)
        source_map_text = source_map_path.read_text(encoding="utf-8")
    except (OSError, tomllib.TOMLDecodeError) as error:
        print(f"Psychrometric routine inventory validation failed: {error}", file=sys.stderr)
        return 1

    errors = validate_psychrometric_inventory(spec, source_map_text)
    if errors:
        print("Psychrometric routine inventory validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print("Psychrometric routine inventory check")
    print(f"  routines: {len(EXPECTED_ROUTINES)}")
    print("  source_order: exact EnergyPlus 26.1 interface order")
    print("  completion_status: source_mapped=34, state_mapped=19")
    print("  required_for_full_domain: false")
    print("  status: valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
