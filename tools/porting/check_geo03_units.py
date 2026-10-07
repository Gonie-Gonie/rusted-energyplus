#!/usr/bin/env python3
"""Compare preserved GEO-03 helper artifacts without launching any tools.

The baseline compares only the current public summary's volume/floor outputs.
Missing mutable Zone fields and owner phases explicitly fail completeness. No
geometry formula, expected-answer injection, tolerance change or gate write is
performed. Reports go to stdout; original-only diagnostics stay unpaired.
"""
from __future__ import annotations

import argparse
from collections import Counter
import json
import math
from pathlib import Path
import re
import struct
import sys

sys.dont_write_bytecode = True
from geo03_provenance import (
    ROOT, Bindings, archive_map, command_paths, exact, execution_interval,
    frozen_contracts, identical_content, integer, path_of, read, ref, require,
    same_binding, timestamp, verify_after_original, verify_contract_bindings,
    verify_original_helper, verify_rust_build, verify_sources,
)

PROVENANCE_SHA = "5ca27645245ee9e7206a31c8906904394a5216625723cbfad12a5d5f49da9bf5"
FIELDS = {
    "volume_m3": "volume_m3", "ceiling_height_m": "height_m",
    "ceiling_height_entered": None, "floor_area_m2": "area_m2",
    "user_entered_floor_area_m2": "area_m2", "geometric_floor_area_m2": "area_m2",
    "ceiling_area_m2": "area_m2", "geometric_ceiling_area_m2": "area_m2",
    "has_floor": None, "has_roof": None,
}
BASELINE_FIELDS = {"volume_m3", "floor_area_m2"}
PHASES = ["geometry_prepared", "before_volume", "after_volume", "after_second_volume"]
BASELINE_UNIMPLEMENTED = [
    "ceiling_height_m", "ceiling_height_entered", "user_entered_floor_area_m2",
    "geometric_floor_area_m2", "ceiling_area_m2", "geometric_ceiling_area_m2",
    "has_floor", "has_roof", "closed_topology_admission",
    "mutable_before_after_second_volume_state",
]
SOURCE_ONLY_UNPAIRED = ["implicit Space", "global counters", "warning/error IO", "source scratch allocations"]
HEX64 = re.compile(r"[0-9a-f]{16}\Z")


def bits(value):
    require(type(value) in (int, float), "Expected a numerical scalar, not a boolean")
    return struct.pack(">d", float(value)).hex()


def from_bits(token):
    require(type(token) is str and HEX64.fullmatch(token), "Expected lowercase binary64 bits")
    return struct.unpack(">d", bytes.fromhex(token))[0]


def value_class(value):
    if math.isnan(value):
        return "nan"
    if math.isinf(value):
        return "positive_infinity" if value > 0 else "negative_infinity"
    if value == 0:
        return "negative_zero" if math.copysign(1, value) < 0 else "positive_zero"
    return "finite"


def scalar_token(row):
    require(type(row) is dict and row.keys() == {"value", "value_bits", "value_class"}, "Malformed scalar observation")
    value = from_bits(row["value_bits"])
    if math.isfinite(value):
        require(type(row["value"]) in (int, float) and bits(row["value"]) == row["value_bits"], "Scalar value and bits differ")
    else:
        spelling = "NaN" if math.isnan(value) else ("+Infinity" if value > 0 else "-Infinity")
        require(row["value"] is None or row["value"] == spelling, "Nonfinite scalar encoding differs")
    require(row["value_class"] == value_class(value), "Scalar class and bits differ")
    return row["value_bits"]


def profiles(tolerances):
    require(tolerances["schema"] == "geo03-tolerances.v1" and tolerances["frozen_before_numerical_comparison"] is True,
            "Unfrozen GEO-03 profiles")
    require(set(tolerances["profiles"]) == {"volume_m3", "area_m2", "height_m"}, "Frozen profile set differs")
    for name, unit, atol in (("volume_m3", "m3", 1e-9), ("area_m2", "m2", 1e-10), ("height_m", "m", 1e-10)):
        require(exact(tolerances["profiles"][name], {
            "unit": unit, "absolute_tolerance": atol, "relative_tolerance": 1e-13,
            "rule": "abs(actual-reference)<=atol+rtol*abs(reference)",
            "finite_class": "exact", "both_zero_sign": "exact"}), "Frozen profile differs: " + name)
    return tolerances["profiles"]


class Comparison:
    """Tolerance arithmetic on actual observations; no volume implementation."""
    def __init__(self):
        self.checks = 0
        self.failures = Counter()
        self.examples = []
        self.metrics = {}

    def failed(self, kind, label, actual=None, original=None):
        self.failures[kind] += 1
        if len(self.examples) < 60 and sum(x["category"] == kind for x in self.examples) < 15:
            self.examples.append({"category": kind, "field": label, "actual": actual, "reference": original})

    def metadata(self, actual, original, label):
        self.checks += 1
        if not exact(actual, original):
            self.failed("metadata", label, actual, original)

    def missing(self, label):
        self.checks += 1
        self.failed("missing_required_owner_observation", label)

    def numeric(self, actual_row, original_row, profile, label):
        self.checks += 1
        a_token, r_token = scalar_token(actual_row), scalar_token(original_row)
        a, r = from_bits(a_token), from_bits(r_token)
        if not math.isfinite(a) or not math.isfinite(r):
            if value_class(a) != value_class(r):
                self.failed("numerical_class", label, a_token, r_token)
            return
        if a == 0 and r == 0 and a_token != r_token:
            self.failed("zero_sign", label, a_token, r_token)
            return
        error = abs(a-r)
        limit = profile["absolute_tolerance"] + profile["relative_tolerance"] * abs(r)
        data = self.metrics.setdefault(profile["unit"], {"count": 0, "maximum_absolute_error": 0.0, "sum_squared_error": 0.0})
        data["count"] += 1
        data["maximum_absolute_error"] = max(data["maximum_absolute_error"], error)
        data["sum_squared_error"] += error * error
        if error > limit:
            self.failed("numerical_tolerance", label, a_token, r_token)

    def report(self):
        return {"checks": self.checks, "mismatch_count": sum(self.failures.values()),
                "mismatch_categories": dict(self.failures), "mismatch_examples": self.examples,
                "scalar_error_metrics": {unit: {"count": row["count"], "maximum_absolute_error": row["maximum_absolute_error"],
                    "rmse": math.sqrt(row["sum_squared_error"]/row["count"])} for unit, row in self.metrics.items()}}


def verify_request(request, cases):
    require(request["schema"] == "geo03-helper-cases.v1" and request["expected_values_supplied"] is False
            and len(request["cases"]) == cases["helper_case_count"] == 19, "Frozen input-only helper request differs")
    require(len({row["case_id"] for row in request["cases"]}) == 19, "Duplicate request identity")
    for row in request["cases"]:
        require(row["kind"] in {"closed_box", "source_only_nonclosed", "source_only_reversed_winding"}, "Unknown helper case kind")
        paired = row["kind"] == "closed_box"
        require(len(row["surfaces"]) == 6 if paired else len(row["surfaces"]) in {5, 6, 7}, "Frozen surface cardinality differs")
        names = [face["name"].upper() for face in row["surfaces"]]
        require(len(set(names)) == len(names), "Duplicate helper surface name")
        for face in row["surfaces"]:
            require(face["class"] in {"Wall", "Floor", "Roof"} and len(face["vertices_m"]) == len(face["input_vertex_bits"]) == 4,
                    "Frozen face class/cardinality differs")
            for point, tokens in zip(face["vertices_m"], face["input_vertex_bits"]):
                require(len(point) == len(tokens) == 3 and exact([bits(x) for x in point], tokens)
                        and all(math.isfinite(from_bits(x)) for x in tokens), "Declared coordinate bits differ")
                require(not paired or all(x != "8000000000000000" for x in tokens), "Paired raw geometry includes a forbidden negative zero")
        for group in ("zone_input", "prepared_zone", "prepared_implicit_space"):
            numerical = {key: bits(value) for key, value in row[group].items() if type(value) in (int, float)}
            require(exact(numerical, row["input_scalar_bits"][group]), "Prepared scalar input bits differ")
            require(all(type(x) in (int, float, bool) or x == "AutoCalculate" for x in row[group].values()), "Unsupported prepared scalar input type")
    require(sum(x["kind"] == "closed_box" for x in request["cases"]) == 16, "Paired helper partition differs")


def verify_original_review(review_ref, helper_ref, refs, helper_finished, bindings):
    review = read(bindings.check(review_ref))
    require(review["schema"] == "geo03-independent-original-data-review.v1"
            and review["status"] == "pass-source-provenance-before-Rust-numerical-execution"
            and same_binding(review["reviewed_helper_reference"], helper_ref)
            and review["Rust_compared"] is False and review["scientific_reference_math_recomputed"] is False
            and review["engines_executed_by_review"] is False and review["gates_updated"] is False,
            "Original-first independent review scope differs")
    verify_contract_bindings(review["reviewed_contracts"], refs, bindings)
    helper = read(bindings.check(helper_ref))
    require(same_binding(review["helper_results"], helper["results"]), "Independent reviewed helper output differs")
    bindings.check(review["reviewed_native_matrix"])
    bindings.check(review["reviewed_build_identity"])
    require(same_binding(review["reviewed_build_identity"], helper["independent_native_build_review"]),
            "Independent reviewed native build differs from the executed helper")
    completed = timestamp(review["review_completed_utc"])
    require(helper_finished <= timestamp(review["original_completed_utc"]) <= completed, "Review preceded actual original completion")
    for key, count in (("helper_case_count", 19), ("paired_closed_helpers", 16), ("source_only_helpers", 3), ("native_case_count", 12)):
        require(integer(review[key], key) == count, "Independent original count differs")
    return completed


def verify_rust(execution_ref, request_ref, helper_ref, review_ref, original_completed, baseline, bindings):
    wrapper = read(bindings.check(execution_ref))
    require(wrapper["schema"] == "geo03-Rust-helper-execution.v1"
            and wrapper["reference_outputs_supplied"] is False and wrapper["gates_updated"] is False,
            "Rust execution wrapper boundary differs")
    for key, expected in (("input_request", request_ref), ("original_first_receipt", helper_ref), ("original_first_review", review_ref)):
        require(same_binding(wrapper[key], expected), "Rust proof/input binding differs: " + key)
        bindings.check(wrapper[key])
    bindings.check(wrapper["executed_launcher"])
    build, available, compilation = verify_rust_build(wrapper["build"], bindings, kind="example", example="geo03_tuples",
        committed=not baseline, required_sources={"crates/ep_runtime/examples/geo03_tuples.rs", "crates/ep_runtime/src/geometry.rs"})
    require(same_binding(build["binary"], wrapper["binary"]), "Rust invoked binary differs from archived build")
    execution = read(bindings.check(wrapper["command_receipt"]))
    require(execution["schema"] == "recorded-porting-command.v1" and execution["launch_error"] is None
            and execution["source_bytes_match_before_and_after"] is True and execution["recorder_updates_gates"] is False
            and execution["recorder_supplies_reference_answers"] is False, "Actual Rust command/snapshot boundary differs")
    start, _ = execution_interval(execution, bindings)
    require(timestamp(compilation["finished_utc"]) <= start, "Rust ran before successful Cargo command")
    verify_after_original(execution, original_completed)
    command_paths(execution["command"], [wrapper["binary"]["path"], request_ref["path"]])
    require(execution["repository_before"]["head"] == execution["repository_after"]["head"], "Repository revision changed during Rust execution")
    launcher = execution["executed_launcher"]
    require(launcher["historical_path"] == "tools/porting/record_command.py", "Actual execution recorder differs")
    bindings.check(launcher["archive"])
    snapshot = read(bindings.check(execution["source_snapshot"]))
    require(snapshot["schema"] == "rust-command-source-snapshot.v1" and snapshot["bytes_normalized"] is False,
            "Rust command source snapshot differs")
    executed = archive_map(snapshot["files"], bindings)
    require(set(executed) == set(available)
            and all(identical_content(executed[k], available[k], bindings) for k in available),
            "Rust execution available-source bytes differ from actual build snapshot")
    result = read(bindings.check(execution["stdout"]))
    return result, build, execution, wrapper


def verify_baseline_row(row, supplied, original, comparison):
    label = supplied["case_id"]
    require(row["status"] == ("baseline_partial" if supplied["kind"] == "closed_box" else "unsupported_source_only")
            and row["route"] == "ep_runtime::geometry::zone_geometry_summaries", "Baseline uses a different route/status")
    for key in ("canonical_coordinate_conversion_claimed", "prepared_read_fields_consumed", "original_parser_admission_claimed",
                "admission_checked", "source_state_observed", "physics_executed"):
        require(row[key] is False, "Baseline manufactured unavailable coverage: " + key)
    require(row["numeric_input_identity_checked"] is True and row["source_state"] is None
            and exact(row["unavailable_source_state_phases"], PHASES)
            and exact(row["unimplemented_fields"], BASELINE_UNIMPLEMENTED)
            and exact(row["unconsumed_prepared_input_groups"], ["prepared_zone", "prepared_implicit_space"])
            and exact(row["source_only_state_unpaired"], SOURCE_ONLY_UNPAIRED), "Baseline missing-state disclosure differs")
    owned = row["typed_surface_order"]
    require(type(owned) is list and len(owned) == len(supplied["surfaces"]), "Actual typed surface count differs")
    ids = set()
    for position, (surface, face) in enumerate(zip(owned, supplied["surfaces"]), 1):
        own_id = integer(surface["id"], "Rust own SurfaceId")
        require(own_id not in ids and integer(surface["request_ordinal"], "request ordinal", 1) == position,
                "Actual surface order/own identity differs")
        ids.add(own_id)
        require(type(surface["name"]) is str and surface["name"].upper() == face["name"].upper()
                and exact(surface["class"], face["class"])
                and exact(surface["input_vertex_bits"], face["input_vertex_bits"]),
                "Actual kernel input identity failed before numerical comparison")
        comparison.metadata(surface["name"].upper(), face["name"].upper(), label+"/surface_name/"+str(position))
        comparison.metadata(surface["class"], face["class"], label+"/surface_class/"+str(position))
        comparison.metadata(surface["input_vertex_bits"], face["input_vertex_bits"], label+"/actual_stored_vertex_bits/"+str(position))
        integer(surface["zone_id"], "own owning ZoneId")
    if supplied["kind"] != "closed_box":
        require(row["zone"] is None, "Source-only baseline diagnostic calculated a paired Zone")
        return None
    zone = row["zone"]
    require(type(zone) is dict and zone.keys() == {"id", "name", "surface_count", "volume_m3", "floor_area_m2"},
            "Baseline summary claims unavailable owner fields")
    own_zone = integer(zone["id"], "Rust own ZoneId")
    require(all(x["zone_id"] == own_zone for x in owned) and integer(zone["surface_count"], "Rust zone surface count") == 6,
            "Summary is not bound to actual surface owners")
    source_zones = original["source_state"]["after_volume"]["zones"]
    require(len(source_zones) == 1, "Original paired helper owns another zone count")
    require(type(zone["name"]) is str and zone["name"].upper() == source_zones[0]["name"].upper(),
            "Zone identity failed before numerical comparison")
    comparison.metadata(zone["name"].upper(), source_zones[0]["name"].upper(), label+"/zone_name")
    for phase in PHASES:
        source_rows = original["source_state"][phase]["surfaces"]
        require([s["name"].upper() for s in source_rows] == [s["name"].upper() for s in supplied["surfaces"]]
                and all(exact(s["world_vertex_bits"], f["input_vertex_bits"]) for s, f in zip(source_rows, supplied["surfaces"])),
                "Original kernel input order/retained bits differ")
    return source_zones[0]


def compare_results(request, native, rust, frozen_profiles, baseline):
    require(baseline, "Canonical owner DTO is not enabled before its reviewed implementation; use explicit --baseline")
    require(rust["schema"] == "geo03-helper-results.v1" and rust["implementation_stage"] == "baseline_public_summary"
            and rust["reference_outputs_supplied_to_Rust"] is False and rust["expected_answers_supplied"] is False
            and rust["original_parser_admission_claimed"] is False and rust["physics_executed"] is False
            and rust["gates_updated"] is False, "Baseline result input/proof boundary differs")
    require(type(rust["cases"]) is list and len(rust["cases"]) == len(native["cases"]) == len(request["cases"]) == 19,
            "Helper result cardinality differs")
    comparison, unpaired, paired = Comparison(), [], 0
    for supplied, source, actual in zip(request["cases"], native["cases"], rust["cases"]):
        label = supplied["case_id"]
        require(actual["case_id"] == source["case_id"] == label and actual["kind"] == source["kind"] == supplied["kind"]
                and exact(actual["input"], supplied), "Rust request echo/case order differs")
        observed = verify_baseline_row(actual, supplied, source, comparison)
        if supplied["kind"] != "closed_box":
            unpaired.append({"case_id": label, "original_status": source["status"], "Rust_status": actual["status"],
                "numerical_comparison_performed": False, "original_diagnostics_retained_in": "original helper result/source_state",
                "Rust_normal_parser_admission_or_rejection_claimed": False})
            continue
        paired += 1
        for field in FIELDS:
            if field not in BASELINE_FIELDS or actual["zone"][field] is None:
                comparison.missing(label+"/Zone/"+field)
            else:
                comparison.numeric(actual["zone"][field], observed[field], frozen_profiles[FIELDS[field]], label+"/available_summary/"+field)
        for phase in PHASES:
            comparison.missing(label+"/actual_mutable_owner_state/"+phase)
    require(paired == 16 and len(unpaired) == 3, "Actual pairing partition differs")
    return comparison, unpaired


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--original-first", required=True, help="Actual original helper execution receipt")
    parser.add_argument("--native-output", required=True, help="Original helper JSON bound by that receipt")
    parser.add_argument("--original-review", required=True, help="Actual raw independent original-data review bound by the Rust wrapper")
    parser.add_argument("--rust-execution", required=True, help="Root wrapper binding actual recorded command/build/request")
    parser.add_argument("--baseline", action="store_true", help="Only current summary fields; incomplete owner coverage explicitly fails")
    args = parser.parse_args()
    bindings = Bindings()
    provenance_ref = ref(Path(__file__).with_name("geo03_provenance.py"))
    require(provenance_ref["sha256"] == PROVENANCE_SHA, "Reviewed metadata reader changed")
    bindings.check(provenance_ref)
    refs, source, cases, tolerances = frozen_contracts(None, bindings)
    verify_sources(source, bindings)
    request_ref = cases["helper_request"]
    request = read(bindings.check(request_ref))
    verify_request(request, cases)
    frozen_profiles = profiles(tolerances)
    original_ref, native_ref = ref(path_of(args.original_first)), ref(path_of(args.native_output))
    original_receipt = read(bindings.check(original_ref))
    require(same_binding(original_receipt["results"], native_ref), "Selected original output differs from actual execution")
    native, _, finished = verify_original_helper(original_ref, refs, source, cases, bindings)
    review_ref = ref(path_of(args.original_review))
    completed = verify_original_review(review_ref, original_ref, refs, finished, bindings)
    rust_ref = ref(path_of(args.rust_execution))
    rust, build, execution, wrapper = verify_rust(rust_ref, request_ref, original_ref, review_ref, completed, args.baseline, bindings)
    comparison, unpaired = compare_results(request, native, rust, frozen_profiles, args.baseline)
    bindings.unchanged()
    result = comparison.report()
    status = "fail-incomplete" if args.baseline else ("pass" if result["mismatch_count"] == 0 else "fail")
    report = {"schema": "geo03-unit-comparison.v1", "status": status,
        "comparison_boundary": "current_public_zone_summary_baseline",
        "tool": ref(Path(__file__)), "provenance_tool": provenance_ref, "contracts": refs,
        "input_request": request_ref, "original_first": original_ref, "native_output": native_ref,
        "original_first_review": review_ref, "Rust_execution": rust_ref,
        "Rust_command_receipt": wrapper["command_receipt"], "Rust_binary": wrapper["binary"], "Rust_build": wrapper["build"],
        "source_worktree_clean": build["source_worktree_clean"], "implementation_commit": build["implementation_commit"],
        "available_Rust_source_count": len(build["available_Rust_sources"]),
        "source_inventory_scope": "Archived available-source inventory; compiler file selection unclaimed",
        "actual_available_summary_fields": sorted(BASELINE_FIELDS), "comparison_source_phase": "after_volume",
        "missing_Zone_owner_fields": sorted(set(FIELDS)-BASELINE_FIELDS), "missing_mutable_owner_phases": PHASES,
        "paired_closed_helper_count": 16, "source_only_helper_count": 3, "source_only_unpaired": unpaired,
        "source_only_state_unpaired": SOURCE_ONLY_UNPAIRED, **result, "checked_artifact_bindings": bindings.report(),
        "reference_outputs_supplied_to_Rust": False, "physics_executed": False, "gates_updated": False,
        "claim_limits": ["Prepared helper area fields are declared input state, not normal parser producer evidence.",
            "The current public summary does not consume prepared area/Space groups or own mutable volume state.",
            "Baseline comparison cannot pass completeness even if all available numerical outputs agree.",
            "Original warnings/counters/implicit Space/scratch and unsupported-route fallback are unpaired.",
            "No normal parser admission, AirPowerCap, ZON02/SYS or full physical equivalence is claimed."]}
    print(json.dumps(report, indent=2, allow_nan=False))
    return 0 if status == "pass" else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (ValueError, KeyError, OSError, TypeError) as error:
        print(json.dumps({"schema": "geo03-unit-check-error.v1", "status": "fail", "error": str(error), "gates_updated": False}, indent=2))
        raise SystemExit(1)
