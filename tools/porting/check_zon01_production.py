#!/usr/bin/env python3
"""Validate actual ZON-01 initializer return, storage and solver-entry handoff.

Reads preserved artifacts only. Original ordinary callbacks remain source-stage
witnesses; dynamic Rust weather and whole warmup alignment are unpaired. No
initializer formula is rebuilt and no reference result is supplied to Rust.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import json
import math
from pathlib import Path
import struct
import sys

sys.dont_write_bytecode = True
from geo03_provenance import ROOT, Bindings, exact, integer, path_of, read, ref, require, same_binding, timestamp, verify_rust_build
from zon01_production_provenance import producer, recorded_run

LEGACY = {
    "MAT": "mean_air_temperature_c", "ZTAV": "zone_timestep_average_air_temperature_c",
    "airHumRat": "air_humidity_ratio", "airHumRatAvg": "zone_timestep_average_air_humidity_ratio",
    "XMAT": "previous_mean_air_temperatures_c", "DSXMAT": "previous_system_mean_air_temperatures_c",
    "WPrevZoneTS": "previous_air_humidity_ratios", "DSWPrevZoneTS": "previous_system_air_humidity_ratios",
}
DIAGNOSTICS = {
    "tempIndLoad": "third_order_temp_independent_load_w",
    "tempDepLoad": "third_order_temp_dependent_load_w_per_k", "AirPowerCap": "air_power_cap_w_per_k",
}
W_COPIES = {"WPrevZoneTS", "DSWPrevZoneTS", "WTimeMinusP", "W1", "WMX", "WM2"}
FULL_ONLY = {"zon01-initialization.json", "compiled-geometry.json", "geometry-consumers.json", "clock-calls.json",
    "psychrometrics-calls.json", "psy02-calls.json", "geo02-geometry.json", "geo02-geometry-operands.json", "geo03-zone-volume.json"}


def scalar(value):
    require(type(value) is dict and set(value) == {"value", "value_bits", "value_class"}, "Typed observed scalar required")
    token = value["value_bits"]
    require(type(token) is str and len(token) == 16 and token == token.lower(), "Binary64 token required")
    bits = bytes.fromhex(token)
    require(len(bits) == 8 and type(value["value"]) is float and math.isfinite(value["value"])
            and struct.pack(">d", value["value"]) == bits, "Observed scalar value/bit identity differs")
    number = struct.unpack(">d", bits)[0]
    kind = "negative_zero" if number == 0.0 and token.startswith("8") else "positive_zero" if number == 0.0 else "finite"
    require(value["value_class"] == kind, "Observed scalar class differs")
    return token, kind


class Comparison:
    def __init__(self):
        self.count = 0
        self.failures = []

    def compare(self, actual, expected, location):
        self.count += 1
        if not exact(actual, expected):
            self.failures.append({"location": location, "actual": actual, "expected": expected})

    def copied(self, actual, expected, location):
        self.compare(scalar(actual), scalar(expected), location)

    def field(self, actual, expected, location):
        if type(actual) is list:
            require(type(expected) is list and len(actual) == len(expected), "Observed field array lengths differ")
            for index, (a, b) in enumerate(zip(actual, expected)):
                self.copied(a, b, location + "/" + str(index))
        else:
            self.copied(actual, expected, location)


def selected_fields(row, request):
    policy = request["state_policy"]
    fields = set(policy["selected_written_fields"] + policy["selected_retained_fields"])
    require(type(row) is dict and set(row) == fields, "Exactly24 actual initializer fields required")
    for key, value in row.items():
        if key in policy["four_slot_arrays"]:
            require(type(value) is list and len(value) == 4, "All six actual arrays require four slots")
            for item in value:
                scalar(item)
        else:
            scalar(value)


def actual_caller(row, available, bindings, expected):
    caller = row["caller"]
    require(type(caller) is dict and set(caller) == {"file", "line", "column"}, "Actual observer caller required")
    path = caller["file"].replace("\\", "/")
    require(path == expected and path in available, "Actual observer caller outside expected owner")
    lines = bindings.check(available[path]).read_text(encoding="utf8").splitlines()
    line, column = integer(caller["line"], "caller line", 1), integer(caller["column"], "caller column", 1)
    require(line <= len(lines) and column <= len(lines[line - 1]) + 1, "Caller outside exact archived source")
    require(type(row["phase"]) is str and bool(row["phase"]), "Actual own execution phase required")


def handoff(check, actual, returned, location):
    projection_shape(actual)
    for source, target in LEGACY.items():
        expected = returned[source][:3] if type(returned[source]) is list else returned[source]
        check.field(actual["legacy_fields"][target], expected, location + "/" + target)
    for source, target in DIAGNOSTICS.items():
        check.copied(actual["legacy_diagnostic_snapshots"][target], returned[source], location + "/" + target)


def projection_shape(actual):
    require(type(actual) is dict and set(actual) == {"legacy_fields", "legacy_diagnostic_snapshots"}, "Actual stored projection required")
    require(set(actual["legacy_fields"]) == set(LEGACY.values())
            and set(actual["legacy_diagnostic_snapshots"]) == set(DIAGNOSTICS.values()), "Actual stored projection keys differ")
    for source, target in LEGACY.items():
        observed = actual["legacy_fields"][target]
        if source in {"XMAT", "DSXMAT", "WPrevZoneTS", "DSWPrevZoneTS"}:
            require(type(observed) is list and len(observed) == 3, "Actual stored history requires three slots")
            for item in observed:
                scalar(item)
        else:
            scalar(observed)
    for item in actual["legacy_diagnostic_snapshots"].values():
        scalar(item)


def initializer_rows(check, artifact, original, request, available, bindings):
    defaults = original["sequences"][0]["allocated_zone_constructor"]["zones"][0]["fields"]
    zero_return = next(row for row in original["sequences"][0]["operations"] if row["operation_id"] == "bare_zero")["after"]["zones"][0]["fields"]
    rows = artifact["initializer_observations"]
    require(type(rows) is list and bool(rows), "Actual initializer observations required")
    require(len({row["zone_id"] for row in rows}) == len(rows), "Bounded one-environment own zone identities repeated")
    for position, row in enumerate(rows, 1):
        prefix = "initializer/" + str(position)
        require(integer(row["sequence"], "initializer sequence", 1) == position
                and integer(row["zone_id"], "own ZoneId") >= 0 and type(row["zone_name"]) is str,
                "Actual initializer order/identity differs")
        actual_caller(row, available, bindings, "crates/ep_runtime/src/heat_balance/air_manager.rs")
        out = row["out_hum_rat"]
        scalar(out)
        for phase in ("constructor", "after_bulk", "before_begin", "after_begin"):
            selected_fields(row[phase], request)
        for key in defaults:
            check.field(row["constructor"][key], defaults[key], prefix + "/constructor/" + key)
            check.field(row["after_bulk"][key], out if key in {"airHumRat", "airHumRatAvg"} else defaults[key], prefix + "/bulk/" + key)
        caller_input = row["caller_temperature_inputs"]
        require(set(caller_input) == {"MAT", "ZTAV", "XMAT_first_three", "DSXMAT_first_three"}, "Explicit own temperature preparation inputs required")
        for key in row["after_bulk"]:
            actual = row["before_begin"][key]
            if key in {"MAT", "ZTAV"}:
                check.copied(actual, caller_input[key], prefix + "/prepared/" + key)
            elif key in {"XMAT", "DSXMAT"}:
                prepared = caller_input[key + "_first_three"]
                require(type(prepared) is list and len(prepared) == 3, "Caller temperature history has three slots")
                check.field(actual[:3], prepared, prefix + "/prepared/" + key)
                check.copied(actual[3], row["after_bulk"][key][3], prefix + "/retained-fourth/" + key)
            else:
                check.field(actual, row["after_bulk"][key], prefix + "/prepared-retention/" + key)
        for key in request["state_policy"]["selected_retained_fields"]:
            check.field(row["after_begin"][key], row["before_begin"][key], prefix + "/member-retention/" + key)
        for key in request["state_policy"]["selected_written_fields"]:
            expected = [out] * 4 if key in {"WPrevZoneTS", "DSWPrevZoneTS"} else out if key in W_COPIES else zero_return[key]
            check.field(row["after_begin"][key], expected, prefix + "/member-copy/" + key)
        # The unit proof pairs all guard states. Here observe the actual one-time production invocation.
        guard = row["guard"]
        require(guard["initializer_invocations_are_Rust_only"] is True, "Actual Rust invocation count must not claim native observations")
        require(all(type(guard[key]) is bool for key in ("begin_environment", "my_environment_before", "eligible_before", "my_environment_after")),
                "Actual typed global guard required")
        for key in ("begin_environment", "my_environment_before", "eligible_before"):
            check.compare(guard[key], True, prefix + "/guard/" + key)
        check.compare(guard["my_environment_after"], False, prefix + "/guard/after")
        check.compare(integer(guard["initializer_invocations"], "actual Rust-only calls"), len(rows), prefix + "/Rust-owner-loop-count")
        handoff(check, row["handoff"], row["after_begin"], prefix + "/actual-store")
    return rows


def entry_rows(check, artifact, initializers, available, bindings):
    rows = artifact["timestep_entry_observations"]
    require(type(rows) is list and bool(rows), "Actual solver entry observation required")
    owners = {row["zone_id"]: row for row in initializers}
    for position, row in enumerate(rows, 1):
        require(integer(row["sequence"], "entry sequence", 1) == position
                and integer(row["timestep_index"], "entry timestep") == 0,
                "Actual retained zero-index entry differs")
        actual_caller(row, available, bindings, "crates/ep_runtime/src/heat_balance/timestep.rs")
        require(type(row["MyEnvrnFlag"]) is bool and row["MyEnvrnFlag"] is False,
                "Stored persistent guard not received at solver entry")
        require(type(row["zones"]) is list and len(row["zones"]) == len(owners), "Actual solver-entry owner count differs")
        own_ids = [integer(zone["zone_id"], "entry own zone ID") for zone in row["zones"]]
        require(len(set(own_ids)) == len(own_ids) and set(own_ids) == set(owners), "Solver-entry owner identity set is incomplete or repeated")
        for zone in row["zones"]:
            owner = owners[zone["zone_id"]]
            require(zone["zone_name"] == owner["zone_name"], "Actual entry zone identity differs")
            projection_shape(zone["handoff"])
            # Only the first entry is an initializer handoff; later state evolution belongs to SYS/history cards.
            if position == 1:
                handoff(check, zone["handoff"], owner["after_begin"], "first-solver-entry")
    return len(rows)


def verified_unit(path, command_path, review_path, matrix, bindings):
    unit_ref, command_ref, review_ref = ref(path), ref(command_path), ref(review_path)
    unit, command, review = [read(bindings.check(item)) for item in (unit_ref, command_ref, review_ref)]
    require(unit["schema"] == "zon01-unit-comparison.v1" and unit["status"] == "pass-exact-selected-state"
            and unit["baseline_diagnostic"] is False and unit["scientific_certification_passed"] is True
            and unit["comparison"]["mismatch_count"] == 0 and unit["comparison"]["comparison_count"] > 0
            and unit["gates_updated"] is False, "Actual canonical unit comparison must pass first")
    require(all(same_binding(unit["contracts"][key], binding) for key, binding in matrix["contracts"].items())
            and same_binding(unit["original_helper"], matrix["original_helper_execution"])
            and same_binding(unit["native_matrix"], matrix["native_original_matrix"])
            and same_binding(unit["independent_original_review"], matrix["original_first_review"]),
            "Canonical unit/original contract crossbinding differs")
    require(unit["Rust_execution"]["implementation_commit"] == matrix["implementation_commit"]
            and unit["Rust_execution"]["crates_tree"] == matrix["crates_tree"]
            and unit["Rust_execution"]["committed_source_certification"] is True,
            "Actual production/unit source revision differs")
    require(command["schema"] == "recorded-porting-command.v1" and command["launch_error"] is None
            and type(command["exit_code"]) is int and command["exit_code"] == 0
            and command["source_bytes_match_before_and_after"] is True
            and exact(read(bindings.check(command["stdout"])), unit), "Unit report differs from successful actual reader stdout")
    bindings.check(command["stderr"])
    require(path_of(command["cwd"]) == ROOT and unit["engines_executed_by_reader"] is False
            and unit["Cargo_executed_by_reader"] is False and unit["Git_executed_by_reader"] is False,
            "Actual unit comparer scope differs")
    require(exact(command["command"], unit["actual_reader_command"]), "Actual unit comparer argv/report identity differs")
    require(review["schema"] == "zon01-independent-unit-data-review.v1"
            and review["status"] == "pass-committed-selected-unit-state"
            and same_binding(review["reviewed_unit_comparison_report"], unit_ref)
            and same_binding(review["reviewed_unit_comparison_command"], command_ref)
            and review["implementation_commit"] == matrix["implementation_commit"]
            and review["crates_tree"] == matrix["crates_tree"]
            and review["original_RHS_reconstructed"] is False
            and review["scientific_comparer_imported_or_executed"] is False
            and review["engines_Cargo_Git_executed"] is False and review["gates_updated"] is False,
            "Accepted independent canonical unit data review required")
    executed_readers = {}
    for row in unit["executed_reader_sources"]:
        historical = row["historical_executed_source"]
        require(historical["historical_path"] not in executed_readers, "Duplicate actual executed unit reader source")
        require(historical["sha256"] == row["matching_archive"]["sha256"], "Actual unit reader archive/source identity differs")
        bindings.check(row["matching_archive"])
        executed_readers[historical["historical_path"]] = historical["sha256"]
    static = read(bindings.check(review["reviewed_reader_static_review"]))
    require(static["schema"] == "zon01-independent-canonical-unit-reader-static-review.v1"
            and static["status"] == "pass-static-canonical-DTO-and-exact-copy-reader"
            and static["gates_updated"] is False, "Independent static reader review missing")
    reviewed_readers = {}
    for row in static["reviewed_reader_sources"]:
        require(row["historical_path"] not in reviewed_readers and row["sha256"] == row["archive"]["sha256"],
                "Duplicate/mismatched independent reader source archive")
        bindings.check(row["archive"])
        reviewed_readers[row["historical_path"]] = row["sha256"]
    require(len(executed_readers) == 7 and exact(executed_readers, reviewed_readers), "Actual executed unit reader set differs from accepted static sources")
    wrapper_ref = unit["Rust_execution"]["execution"]
    wrapper = read(bindings.check(wrapper_ref))
    require(same_binding(review["reviewed_Rust_unit_execution"], wrapper_ref)
            and same_binding(review["reviewed_Rust_unit_build"], wrapper["build"])
            and same_binding(review["reviewed_Rust_unit_binary"], wrapper["binary"]), "Independent unit engine identity differs")
    unit_build, _, _ = verify_rust_build(wrapper["build"], bindings, kind="example", example="zon01_tuples", committed=True,
        required_sources={"crates/ep_runtime/examples/zon01_tuples.rs", "crates/ep_runtime/src/heat_balance/zone_air_initialization.rs"})
    require(same_binding(unit_build["binary"], wrapper["binary"]), "Actually executed unit binary/build differs")
    actual = read(bindings.check(wrapper["command_receipt"]))
    require(actual["exit_code"] == 0 and actual["source_bytes_match_before_and_after"] is True
            and same_binding(actual["source_snapshot"], unit_build["source_snapshot"])
            and exact(actual["command"], [str(bindings.check(wrapper["binary"])), str(bindings.check(wrapper["request"]))]),
            "Actual input-only unit command/source differs")
    for key in ("stdout", "stderr"):
        bindings.check(actual[key])
    require(same_binding(actual["stdout"], wrapper["results"]), "Actual unit numerical result differs from engine stdout")
    finished = timestamp(command["finished_utc"])
    reviewed = timestamp(review["review_completed_utc"])
    require(timestamp(actual["finished_utc"]) <= timestamp(command["started_utc"]) <= finished <= reviewed,
            "Unit engine/comparison/independent review chronology differs")
    return unit_ref, command_ref, review_ref, reviewed


def physical_clock(artifact, expected, scope):
    require(artifact["schema"] == "clk01-clock-trace.v1"
            and artifact["capture_source"] == "existing-physical-zone-loop-hook"
            and artifact["complete_on_collecting_thread"] is True
            and artifact["prepared_rows_are_executed_events"] is False
            and integer(artifact["total_invocation_count"], "clock total") == integer(artifact["recorded_invocation_count"], "clock retained") == expected
            and integer(artifact["omitted_invocation_count"], "clock omitted") == 0 and artifact["truncation_reason"] is None
            and len(artifact["zone_invocations"]) == expected, "Actual physical clock prefix incomplete")
    frames = artifact["prepared_hourly_frames"]
    require(len(frames) == expected // 4 and len(artifact["prepared_environment_points"]) == (expected if scope == "B" else 0),
            "Actual clock preparation cardinality differs")
    for position, event in enumerate(artifact["zone_invocations"]):
        hour, step = divmod(position, 4)
        for key, expected_value in (("sequence", position + 1), ("hour_index", hour), ("zone_timestep", step + 1),
                                    ("zone_steps_per_hour", 4), ("calendar_frame_index", hour)):
            require(integer(event[key], key) == expected_value, "Actual physical clock invocation order differs")
        require(event["timestep_seconds_bits"] == struct.pack(">d", 900.0).hex()
                and exact(event["environment_point_index"], position if scope == "B" else None)
                and integer(frames[hour]["hourly_sample_index"], "frame hourly index") == hour, "Actual physical clock interval/index differs")
    return len(artifact["zone_invocations"])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rust", type=Path, required=True)
    parser.add_argument("--unit-report", type=Path, required=True)
    parser.add_argument("--unit-command", type=Path, required=True)
    parser.add_argument("--unit-review", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    require(Path.cwd().resolve() == ROOT, "Read from repository cwd")
    output = args.output.resolve()
    require(output.is_relative_to(ROOT / ".runtime/porting/ZON-01") and not output.exists(), "Fresh contained comparison report required")
    started = datetime.now(timezone.utc).isoformat()
    bindings, check = Bindings(), Comparison()
    matrix_ref = ref(args.rust)
    matrix = read(bindings.check(matrix_ref))
    refs, cases, request, completed, build, available, compilation = producer(matrix, bindings)
    unit_ref, unit_command_ref, unit_review_ref, unit_reviewed = verified_unit(args.unit_report, args.unit_command, args.unit_review, matrix, bindings)
    helper_receipt = read(bindings.check(matrix["original_helper_execution"]))
    original = read(bindings.check(helper_receipt["results"]))
    require(original["schema"] == "zon01-helper-results.v1" and len(original["sequences"]) == 8,
            "Actual original initializer anchors required")
    directories, summaries = set(), []
    for case in matrix["cases"]:
        observed, summary_data = {}, {}
        for level in ("Full", "Summary"):
            directory, summary, _, artifacts = recorded_run(case, case[level], matrix, build, available,
                compilation, completed, bindings, directories)
            observed[level] = directory
            summary_data[level] = summary
            actual_command = read(bindings.check(case[level]["command_receipt"]))
            require(unit_reviewed <= timestamp(actual_command["started_utc"]), "Production command preceded accepted unit comparison/review")
            if level == "Summary":
                require(not any((directory / name).exists() for name in FULL_ONLY), "Summary unexpectedly activated Full observers")
                continue
            artifact = read(bindings.check(artifacts[directory / "zon01-initialization.json"]))
            require(artifact["schema"] == "zon01-initialization-trace.v1"
                    and artifact["capture_source"] == "actual-Rust-zone-air-initialization-and-shared-solver-entry"
                    and artifact["thread_coverage"] == "collecting-thread-only"
                    and artifact["observer_supplies_inputs"] is False
                    and artifact["observer_recalculates_initialization"] is False,
                    "Full is not an actual passive initializer/entry observation")
            for total, retained, omitted, series in (
                ("total_initializer_count", "retained_initializer_count", "omitted_initializer_count", "initializer_observations"),
                ("zero_index_timestep_entry_count", "retained_zero_index_timestep_entry_count", "omitted_zero_index_timestep_entry_count", "timestep_entry_observations"),
            ):
                require(integer(artifact[total], total, 1) == integer(artifact[retained], retained, 1) == len(artifact[series])
                        and integer(artifact[omitted], omitted) == 0, "Retained observation prefix incomplete")
            require(integer(artifact["total_timestep_entry_call_count"], "all actual entry calls", 1)
                    >= artifact["zero_index_timestep_entry_count"], "Actual entry/candidate counts differ")
            initializers = initializer_rows(check, artifact, original, request, available, bindings)
            entry_count = entry_rows(check, artifact, initializers, available, bindings)
            clock_ref = artifacts[directory / "clock-calls.json"]
            interval_count = physical_clock(read(bindings.check(clock_ref)), 288 if case["duration"] == "72H" else 96, case["scope"])
            summaries.append({"case_id": case["case_id"], "Full_trace": artifacts[directory / "zon01-initialization.json"],
                "actual_initializer_count": len(initializers), "actual_zero_index_entry_count": entry_count,
                "actual_all_entry_hook_count": artifact["total_timestep_entry_call_count"],
                "actual_initializer_phases": [row["phase"] for row in initializers],
                "actual_entry_phases": [row["phase"] for row in artifact["timestep_entry_observations"]],
                "first_solver_entry_projection_paired": True, "later_history_or_native_warmup_paired": False,
                "actual_physical_clock": clock_ref, "actual_physical_zone_intervals": interval_count})
        full_data, summary_data_row = summary_data["Full"], summary_data["Summary"]
        check.compare(sorted(full_data), sorted(summary_data_row), case["case_id"] + "/ordinary-summary-keys")
        for key in set(full_data) - {"artifacts", "timing", "input", "config"}:
            check.compare(full_data[key], summary_data_row[key], case["case_id"] + "/ordinary-summary/" + key)
        check.compare({key: value for key, value in full_data["config"].items() if key != "trace_level"},
                      {key: value for key, value in summary_data_row["config"].items() if key != "trace_level"},
                      case["case_id"] + "/ordinary-config")
        for name in ("results/selected-outputs.csv", "results/meters.csv"):
            full_ref, summary_ref = [ref(observed[level] / name) for level in ("Full", "Summary")]
            bindings.check(full_ref)
            bindings.check(summary_ref)
            check.compare(full_ref["sha256"], summary_ref["sha256"], case["case_id"] + "/" + name)
        for name in ("results/result-store.json", "porting_scope.json"):
            check.compare(read(bindings.check(ref(observed["Full"] / name))), read(bindings.check(ref(observed["Summary"] / name))),
                          case["case_id"] + "/" + name)
    bindings.unchanged()
    source_dir = output.parent / (output.stem + ".reader-source")
    require(not source_dir.exists(), "Fresh executed-reader archives required")
    source_dir.mkdir(parents=True)
    reader_sources = []
    for current in (Path(__file__).resolve(), Path(__file__).with_name("zon01_production_provenance.py"),
                    Path(__file__).with_name("zon01_launch_provenance.py"), Path(__file__).with_name("geo03_provenance.py")):
        archive = source_dir / current.name
        with archive.open("xb") as stream:
            stream.write(current.read_bytes())
        reader_sources.append({"historical_path": current.relative_to(ROOT).as_posix(), "sha256": ref(current)["sha256"],
                               "matching_archive": ref(archive)})
    report = {
        "schema": "zon01-production-comparison.v1", "status": "pass" if not check.failures else "fail",
        "started_utc": started, "finished_utc": datetime.now(timezone.utc).isoformat(),
        "actual_reader_command": sys.orig_argv, "contracts": refs, "Rust_matrix": matrix_ref,
        "unit_comparison": unit_ref, "actual_unit_comparison_command": unit_command_ref,
        "independent_unit_data_review": unit_review_ref,
        "implementation_commit": build["implementation_commit"], "crates_tree": build["crates_tree"],
        "actual_command_count": matrix["actual_command_count"], "comparison_count": check.count,
        "mismatch_count": len(check.failures), "representative_mismatches": check.failures[:30],
        "observations": summaries, "executed_reader_sources": reader_sources,
        "original_helper_execution": matrix["original_helper_execution"], "native_original_matrix": matrix["native_original_matrix"],
        "independent_original_review": matrix["original_first_review"],
        "assignment_tolerance_used": False, "initializer_RHS_reconstructed": False,
        "engines_executed_by_reader": False, "Cargo_executed_by_reader": False, "Git_executed_by_reader": False,
        "original_outputs_supplied_to_Rust": False, "gates_updated": False,
        "limits": [
            "Exact selected unit source anchors support constructor/reset and actual dynamic OutHumRat copy/retention checks; no new formula is rebuilt.",
            "Only actual initializer projection and the first stored solver entry are paired. Later history/correction/retry/Space/sibling/CTF/SYS remain unpaired.",
            "Native ordinary callbacks are independently reviewed source-stage witnesses, not pristine member returns or cross-engine Rust weather/calendar/warmup samples.",
            "Rust own IDs/phases/caller/entry counts are observed. No native numeric-ID identity or member-call counts are invented.",
            "Actual enclosing context is copied by the reviewed observer and reported without cross-engine context/phase alignment; solver entry may be warmup or context-null.",
            "Summary checks observer absence and ordinary output equality only; it does not directly observe initialization fields.",
            "AirPowerCap W/K reset ownership is observed, not J/K capacity parity, later load-output parity or annual/B72 physics.",
        ],
    }
    with output.open("x", encoding="utf8", newline="\n") as stream:
        stream.write(json.dumps(report, indent=2, allow_nan=False) + "\n")
    print(json.dumps(report, allow_nan=False))
    return 0 if not check.failures else 1


if __name__ == "__main__":
    raise SystemExit(main())
