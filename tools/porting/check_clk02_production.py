#!/usr/bin/env python3
"""Read preserved CLK-02 CLI traces and verify literal raw-owner handoffs.

No engine, Cargo or Git is run. Native values are comparison operands only;
physical conversion, selection, interpolation and warmup science stay unpaired.
"""
from __future__ import annotations

import argparse
from collections import Counter
from datetime import datetime, timezone
import json
import math
from pathlib import Path
import shutil
import struct
import sys

sys.dont_write_bytecode = True
from geo03_provenance import (
    ROOT, Bindings, command_paths, exact, execution_interval, integer, path_of,
    read, ref, require, same_binding, timestamp, verify_contract_bindings,
    verify_rust_build,
)

FROZEN = {
    "source": "4fc97c6a0a775f956c79855795f5a23c6e190ba2ad25dc2cba4b04477a9cf1d0",
    "cases": "4752d6704b6ad8f6e4f1ea4907ce4d082bec041d0e2f22e28d11bd1c04ab062c",
    "tolerances": "3fb137a3096a331ea1ab3f751d2667530d9a6d8ca40d785bdd5c07bfd304350e",
}
PLAN_SHA = "c47ced3332232d106555e27b5b4b27a0379b31c22af6b7f7ca2733f9a6212585"
PROVENANCE_SHA = "5ca27645245ee9e7206a31c8906904394a5216625723cbfad12a5d5f49da9bf5"
UNIT_PROJECTION_SHA = "c074e2ee4d969294762ed6888b0ae9f22ed3956c05d2a472644b63501bb49681"
ORIGINAL_SHA = "afa446961b4df0651c916a84a24a816814da8becb48b952e511049e7cc4bc3d0"
ORIGINAL_REVIEW_SHA = "a1acff78956de2f20ac101caef5e90e7a4970b2d4f62ec925fef158d39492b28"
LAUNCHER_SHA = "f388d57e472577304327ce6ab0b922ac82dbc1bc4225e3c3656716927d58bd50"
LAUNCHER_REVIEW = {
    "path": ".runtime/porting/CLK-02/independent-production-launcher-static-review-02/review.json",
    "sha256": "eb1c8a50a0e96c1b8d04e047cf047f4b69e0bb7f75f58074513ac9a6ddfce4d6",
}
REQUIRED = {
    "crates/ep_run/src/pipeline.rs", "crates/ep_run/src/clk02_trace.rs",
    "crates/ep_runtime/src/weather.rs", "crates/ep_runtime/src/weather_raw/loader.rs",
    "crates/ep_runtime/src/weather_raw/mod.rs", "crates/ep_runtime/src/weather_raw/production_trace.rs",
}
COPIED = {
    "DryBulb": "dry_bulb_c", "DewPoint": "dew_point_c", "RelHum": "relative_humidity_percent",
    "AtmPress": "atmospheric_pressure_pa", "IRHoriz": "horizontal_infrared_radiation_wh_per_m2",
    "GLBHoriz": "global_horizontal_radiation_wh_per_m2", "DirectRad": "direct_normal_radiation_wh_per_m2",
    "DiffuseRad": "diffuse_horizontal_radiation_wh_per_m2", "WindDir": "wind_direction_deg", "WindSpeed": "wind_speed_m_per_s",
}
DATES = {"WYear": "year", "WMonth": "month", "WDay": "day", "WHour": "hour", "WMinute": "minute"}
HEADER_CONTEXT = ("EndDayOfMonth", "LeapYearAdd", "wvarsMissedCounts.WeathCodes", "SpecialDays_allocated", "DataPeriods_allocated")
STREAM_KEYS = ("is_open", "good", "eof", "fail", "bad", "position_byte", "position_available")
SAMPLE_FIELDS = {"dry_bulb_c", "wet_bulb_c", "relative_humidity_percent", "outdoor_humidity_ratio",
    "atmospheric_pressure_pa", "horizontal_infrared_radiation_w_per_m2", "global_horizontal_radiation_w_per_m2",
    "direct_normal_radiation_w_per_m2", "diffuse_horizontal_radiation_w_per_m2", "wind_speed_m_per_s",
    "wind_direction_deg", "liquid_precipitation_depth_mm"}
TRACE_NAME = "clk02-weather-production-trace.json"
FULL_ONLY = {TRACE_NAME, "clock-calls.json", "psy02-calls.json", "psychrometrics-calls.json",
    "zon01-initialization.json", "compiled-geometry.json", "geometry-consumers.json",
    "geo02-geometry.json", "geo02-geometry-operands.json", "geo03-zone-volume.json"}


def scalar(value: object, label: str, *, finite: bool = True) -> tuple[str, str]:
    require(type(value) is dict and set(value) == {"value", "value_bits", "value_class"}, "Typed scalar required: " + label)
    bits = value["value_bits"]
    require(type(bits) is str and len(bits) == 16 and bits == bits.lower(), "Binary64 encoding required: " + label)
    try:
        decoded = struct.unpack(">d", bytes.fromhex(bits))[0]
    except (ValueError, struct.error) as error:
        raise ValueError("Invalid binary64: " + label) from error
    if math.isfinite(decoded):
        require(type(value["value"]) is float and struct.pack(">d", value["value"]).hex() == bits,
                "Scalar bit/value identity differs: " + label)
        kind = "negative_zero" if bits == "8000000000000000" else "positive_zero" if bits == "0000000000000000" else "finite"
    else:
        require(not finite and value["value"] is None, "Nonfinite scalar outside paired scope: " + label)
        kind = "nan" if math.isnan(decoded) else "positive_infinity" if decoded > 0 else "negative_infinity"
    require(value["value_class"] == kind, "Scalar class differs: " + label)
    return bits, kind


class Comparison:
    def __init__(self) -> None:
        self.count = 0
        self.mismatch_count = 0
        self.failures: list[dict] = []
        self.categories: Counter[str] = Counter()

    def equal(self, actual: object, expected: object, location: str, category: str = "typed_value") -> None:
        self.count += 1
        self.categories[category] += 1
        if not exact(actual, expected):
            self.mismatch_count += 1
            if len(self.failures) < 1000:
                self.failures.append({"location": location, "actual": actual, "expected": expected})

    def value(self, actual: object, expected: object, location: str, category: str) -> None:
        if type(expected) is dict and set(expected) == {"value", "value_bits", "value_class"}:
            self.equal(scalar(actual, location), scalar(expected, location), location, category)
        elif type(expected) is dict:
            require(type(actual) is dict and set(actual) == set(expected), "Selected nested field set differs: " + location)
            for key in expected:
                self.value(actual[key], expected[key], location + "/" + key, category)
        elif type(expected) is list:
            require(type(actual) is list and len(actual) == len(expected), "Array cardinality differs: " + location)
            self.equal(len(actual), len(expected), location + "/length", category)
            for index, (a, e) in enumerate(zip(actual, expected, strict=True)):
                self.value(a, e, location + "/" + str(index), category)
        else:
            self.equal(actual, expected, location, category)


def selected(check: Comparison, actual: dict, expected: dict, keys: list | tuple, location: str, category: str) -> None:
    require(type(actual) is dict and type(expected) is dict and set(keys) <= set(actual) and set(keys) <= set(expected),
            "Every selected field is required: " + location)
    for key in keys:
        check.value(actual[key], expected[key], location + "/" + key, category)


def caller(row: dict, owner: str, available: dict, bindings: Bindings) -> None:
    value = row["caller"]
    require(type(value) is dict and set(value) == {"file", "line", "column"} and value["file"].replace("\\", "/") == owner,
            "Actual passive caller outside its owner")
    lines = bindings.check(available[owner]).read_text(encoding="utf-8").splitlines()
    line, column = integer(value["line"], "caller line", 1), integer(value["column"], "caller column", 1)
    require(line <= len(lines) and column <= len(lines[line - 1]) + 1, "Actual caller outside archived source")
    require(("record_prepared(" if owner.endswith("pipeline.rs") else "record_consumer(") in lines[line - 1],
            "Actual caller does not point at the archived observation hook")
    require(type(row["phase"]) is str and bool(row["phase"]), "Actual Rust phase label required")


def projected_shape(value: dict, location: str) -> None:
    require(type(value) is dict and set(value) == {"source_date_fields", "legacy_fields"}, "Actual projected record shape differs: " + location)
    require(set(value["source_date_fields"]) == set(DATES.values()) and all(type(x) is int for x in value["source_date_fields"].values()),
            "All five actual projected dates required: " + location)
    require(set(value["legacy_fields"]) == set(COPIED.values()) | {"liquid_precipitation_depth_mm"}, "All eleven actual projected fields required: " + location)
    for field, number in value["legacy_fields"].items():
        scalar(number, location + "/" + field)


def raw_shape(value: dict, source: dict, location: str) -> None:
    policy = source["paired_raw_outputs"]
    require(type(value) is dict and set(value) == {"ErrorFound", "WObs", "date_fields", "mandatory_reals", "optional_reals", "weather_codes", "private_RField21_observed"},
            "All defined public raw storage required: " + location)
    require(type(value["ErrorFound"]) is bool and type(value["WObs"]) is int and value["private_RField21_observed"] is False,
            "Raw flags/observation/private storage limit differs: " + location)
    require(set(value["date_fields"]) == set(policy["date_integer_fields"]) and all(type(x) is int for x in value["date_fields"].values()),
            "All raw source dates required: " + location)
    require(type(value["weather_codes"]) is list and len(value["weather_codes"]) == 9 and all(type(x) is int for x in value["weather_codes"]),
            "Nine actual integer codes required: " + location)
    for group, keys in (("mandatory_reals", policy["mandatory_real_fields"]), ("optional_reals", policy["optional_real_fields"])):
        require(set(value[group]) == set(keys), "All raw scalar fields required: " + location)
        for key in keys:
            scalar(value[group][key], location + "/" + key)


def observe(check: Comparison, artifact: dict, original: dict, source: dict, case: dict, available: dict, bindings: Bindings) -> dict:
    require(artifact["schema"] == "clk02-weather-production-trace.v1"
            and artifact["capture_source"] == "actual-Rust-runtime-raw-weather-preparation-and-existing-consumers"
            and artifact["thread_coverage"] == "collecting-thread-only"
            and artifact["observer_supplies_inputs"] is False and artifact["observer_recomputes_weather"] is False
            and artifact["prepared_rows_are_executed_zone_events"] is False
            and artifact["consumer_repeated_identities_deduplicated"] is False
            and artifact["computed_precomputed_sample_values_paired_to_native"] is False
            and artifact["raw_source_year_equals_civil_calendar_year_claimed"] is False
            and artifact["native_weather_callback_phase_claimed"] is False,
            "Actual passive raw/consumer trace required")
    require(artifact["complete_on_collecting_thread"] is True and artifact["truncation_reason"] is None
            and artifact["observation_limit_per_series"] == artifact["selected_record_pool_limit"] == 10_000,
            "Complete bounded capture required")
    for total, retained, omitted, series in (("total_prepared_count", "retained_prepared_count", "omitted_prepared_count", "prepared"),
            ("total_consumer_count", "retained_consumer_count", "omitted_consumer_count", "consumers")):
        require(integer(artifact[total], total, 1) == integer(artifact[retained], retained, 1) == len(artifact[series])
                and integer(artifact[omitted], omitted) == 0, "Actual observation prefix incomplete")
    require(len(artifact["prepared"]) == 1, "One actual runtime preparation required; admission preparation is distinct")
    prepared = artifact["prepared"][0]
    caller(prepared, "crates/ep_run/src/pipeline.rs", available, bindings)
    require(prepared["sequence"] == 1 and path_of(prepared["weather_path"]) == bindings.check(case["weather"]), "Actual prepared path/order differs")
    fixed = original["fixed_epw"]
    hours = 72 if case["duration"] == "72H" else 24
    require(prepared["raw_record_count"] == fixed["record_count"] == 8760
            and prepared["weather_byte_length"] == bindings.check(case["weather"]).stat().st_size,
            "Actual raw loaded count/byte length differs")
    require(prepared["selected_record_count"] == prepared["retained_selected_record_count"] == len(prepared["selected_records"]) == hours
            and prepared["omitted_selected_record_count"] == 0
            and artifact["total_selected_record_count"] == artifact["retained_selected_record_count"] == hours
            and artifact["omitted_selected_record_count"] == 0, "Actual selected source prefix incomplete")
    selected(check, prepared["header_after_load"], fixed["final_state"], source["paired_header_state"] + list(HEADER_CONTEXT),
             case["case_id"] + "/header_after_load", "native_header_state")
    for actual_key, original_key in (("stream_after_header", "stream_after_header"), ("final_stream", "final_stream")):
        selected(check, prepared[actual_key], fixed[original_key], STREAM_KEYS, case["case_id"] + "/" + actual_key, "native_semantic_stream")
        require(prepared[actual_key]["native_rdstate_bits_claimed"] is False, "Native iostate bits cannot be inferred")
    for index, row in enumerate(prepared["selected_records"]):
        location = case["case_id"] + "/selected/" + str(index)
        require(integer(row["selected_record_index"], "selected index") == index, "Actual selected ordinal differs")
        source_index = integer(row["source_record_index"], "actual source index")
        require(source_index < len(fixed["records"]), "Actual selected source index outside preserved source records")
        raw_shape(row["raw"], source, location)
        check.value(row["raw"], fixed["records"][source_index]["after"], location + "/actual_raw_owner", "native_raw_storage")
        projected_shape(row["projected"], location)
        for source_key, target in DATES.items():
            check.equal(row["projected"]["source_date_fields"][target], row["raw"]["date_fields"][source_key], location + "/copied_date/" + target, "literal_date_copy")
        for source_key, target in COPIED.items():
            check.value(row["projected"]["legacy_fields"][target], row["raw"]["mandatory_reals"][source_key], location + "/copied_scalar/" + target, "literal_scalar_copy")
    phases, context_scopes, substeps, sample_count = Counter(), Counter(), set(), 0
    for position, row in enumerate(artifact["consumers"], 1):
        location = case["case_id"] + "/consumer/" + str(position)
        require(integer(row["sequence"], "consumer sequence", 1) == position, "Actual consumer call order differs")
        caller(row, "crates/ep_runtime/src/weather.rs", available, bindings)
        record_index = integer(row["record_index"], "actual selected-series index")
        substep = integer(row["zone_timestep"], "actual zone timestep", 1)
        require(record_index < len(prepared["selected_records"]), "Actual consumer index outside prepared selected series")
        projected_shape(row["received_hourly_record"], location)
        check.value(row["received_hourly_record"], prepared["selected_records"][record_index]["projected"], location + "/received_hourly_record", "actual_consumer_record_handoff")
        sample = row["received_precomputed_sample"]
        if sample is not None:
            sample_count += 1
            require(type(sample) is dict and set(sample) == SAMPLE_FIELDS | {"record_index", "zone_timestep"}
                    and integer(sample["record_index"], "sample record index") == record_index
                    and integer(sample["zone_timestep"], "sample zone timestep", 1) == substep,
                    "Actual received sample index/substep differs")
            for key in SAMPLE_FIELDS:
                scalar(sample[key], location + "/sample/" + key, finite=False)
        phases[row["phase"]] += 1
        context = row["context"]
        if context is not None:
            require(type(context) is dict and set(context) == {"scope", "zone_timestep", "system_call"}
                    and type(context["scope"]) is str, "Actual execution context required")
            context_scopes[context["scope"]] += 1
            zone = context["zone_timestep"]
            if zone is not None:
                require(integer(zone["hour_index"], "context hour index") == record_index
                        and integer(zone["zone_timestep"], "context zone timestep", 1) == substep
                        and substep <= integer(zone["zone_steps_per_hour"], "context zone steps", 1),
                        "Actual consumer context operands differ")
        substeps.add(substep)
    require({1, 4} <= substeps, "Bounded physical route lacks actual subdivided consumer operands")
    return {"case_id": case["case_id"], "actual_prepared_count": 1, "actual_loaded_raw_record_count": prepared["raw_record_count"],
        "actual_selected_record_count": hours, "actual_consumer_count": len(artifact["consumers"]),
        "actual_precomputed_sample_available_count": sample_count,
        "actual_consumer_phases": dict(phases), "actual_context_scopes": dict(context_scopes),
        "actual_consumer_substeps": sorted(substeps), "liquid_policy_paired_to_native": False,
        "sample_physics_paired_to_native": False, "native_warmup_phase_inferred": False}


def recorded_run(case: dict, item: dict, level: str, build: dict, completed: object, planned: object, bindings: Bindings, directories: set) -> tuple:
    output = path_of(item["output_directory"])
    require(output.is_dir() and output not in directories, "Missing/reused actual CLI directory")
    directories.add(output)
    execution = read(bindings.check(item["command_receipt"]))
    require(execution["schema"] == "recorded-porting-command.v1" and execution["launch_error"] is None
            and execution["source_bytes_match_before_and_after"] is True and execution["recorder_updates_gates"] is False
            and execution["recorder_supplies_reference_answers"] is False, "Actual read-only-recorder boundary differs")
    started, _ = execution_interval(execution, bindings)
    require(completed <= started and planned <= started, "Actual CLI preceded completed unit proof or declared plan")
    require(execution["repository_before"]["head"] == execution["repository_after"]["head"] == build["implementation_commit"]
            and same_binding(execution["source_snapshot"], build["source_snapshot"]), "Actual CLI source/head differs from build")
    expected = [build["binary"]["path"], "run", case["input"]["path"], "--weather", case["weather"]["path"],
        "--output-dir", str(output), "--mode", "compatibility", "--partial", "deny", "--trace-level", level.lower(), "--porting-scope"]
    require(len(execution["command"]) == len(expected) + 1 and execution["command"][-1].strip().upper() == case["scope"], "Actual scope flag differs")
    command_paths(execution["command"][:-1], expected, {0, 2, 4, 6})
    require(execution["executed_launcher"]["historical_path"] == "tools/porting/record_command.py", "Unexpected actual recorder owner")
    require(execution["executed_launcher"]["archive"]["sha256"] == build["actual_executed_recorder"]["archive"]["sha256"],
            "Actual CLI recorder differs from validated build recorder")
    bindings.check(execution["executed_launcher"]["archive"])
    artifacts = {}
    for value in item["artifacts"]:
        path = bindings.check(value)
        require(path.is_relative_to(output) and path not in artifacts, "Foreign/duplicate actual CLI artifact")
        artifacts[path] = value
    require(set(artifacts) == {path.resolve() for path in output.rglob("*") if path.is_file()}, "Actual retained artifact inventory differs")
    require(output / "run-summary.json" in artifacts and output / "porting_scope.json" in artifacts, "Actual summary and requested scope missing")
    summary = read(bindings.check(artifacts[output / "run-summary.json"]))
    config = summary["config"]
    require(summary["exit_code"] == execution["exit_code"] == 0 and type(summary["exit_code"]) is int
            and summary["status"] == "success" and summary["message"] == "arbitrary run completed"
            and summary["source_order_gate"]["matches"] is True and summary["selected_algorithm_lane"]["diagnostic_probe_used"] is False,
            "Actual successful source-order runtime required")
    require(config["mode"] == "compatibility" and config["partial_policy"] == "deny" and config["trace_level"] == level.lower()
            and config["dry_run"] is False and config["hours"] is None and config["output_format"] == "rust-native"
            and config["oracle_baseline"] is False and config["compare_oracle"] is False
            and summary["oracle"] is None and summary["comparison"] is None
            and summary["oracle_status"] == summary["compare_status"] == "not-requested", "Unsupported/answer-fed CLI configuration")
    require(summary["rust_runtime"]["samples"] == (72 if case["duration"] == "72H" else 24)
            and summary["support"]["conformance_claim"] is False, "Actual bounded duration/scope boundary differs")
    if case["scope"] == "B":
        require(summary["rust_runtime"]["fixture_demand_injection_used"] is False
                and integer(summary["rust_runtime"]["purchased_air_coupling_call_count"], "actual B coupling", 1) > 0,
                "Actual B physical demand/coupling missing")
    else:
        require(case["scope"] == "A" and summary["rust_runtime"]["runtime_class"] == "one-zone-heat-balance-compatibility"
                and summary["rust_runtime"]["fixture_demand_injection_used"] is None
                and summary["rust_runtime"]["purchased_air_coupling_call_count"] is None,
                "A unavailable PurchasedAir registrations must remain explicit")
    scope = read(bindings.check(artifacts[output / "porting_scope.json"]))
    require(scope["schema"] == "porting-scope.v1" and scope["scope"] == case["scope"] and scope["admissible"] is True and scope["violations"] == [],
            "Actual requested input scope was not admitted")
    return output, summary, artifacts


def unit_execution(matrix: dict, unit: dict, cases: dict, refs: dict, bindings: Bindings) -> tuple:
    """Bind the already executed unit proof without repeating its comparison."""
    require(unit["declared_pre_execution_DTO_projection"]["sha256"] == UNIT_PROJECTION_SHA,
            "Accepted pre-execution unit projection differs")
    projection = read(bindings.check(unit["declared_pre_execution_DTO_projection"]))
    verify_contract_bindings(projection["contracts"], refs, bindings)
    require(unit["original_helper"]["sha256"] == ORIGINAL_SHA
            and unit["independent_original_review"]["sha256"] == ORIGINAL_REVIEW_SHA,
            "Preserved original and independent data review identities differ")
    original = read(bindings.check(unit["original_helper"]))
    peer = read(bindings.check(unit["independent_original_review"]))
    require(original["schema"] == "clk02-original-helper-execution.v1" and original["actual_helper_exit_code"] == 0
            and original["preservation_checks_passed"] is True and original["source_expected_answers_supplied"] is False
            and original["gates_updated"] is False and same_binding(original["results"], unit["original_results"])
            and peer["status"] == "pass-preserved-original-data-before-Rust-baseline"
            and same_binding(peer["reviewed_reference"], unit["original_helper"])
            and same_binding(peer["reviewed_results"], unit["original_results"]), "Native actual result/peer chain differs")
    verify_contract_bindings(original["contracts"], refs, bindings)
    require(same_binding(unit["request"], cases["helper_request"])
            and same_binding(original["request"], cases["helper_request"]), "Unit/original input-only request differs")
    build, _, compilation = verify_rust_build(unit["Rust_build"], bindings, kind="example", example="clk02_probe",
        committed=True, required_sources=("crates/ep_runtime/examples/clk02_probe.rs",))
    probe = read(bindings.check(unit["Rust_execution"]))
    probe_started, probe_finished = execution_interval(probe, bindings)
    require(probe["schema"] == "recorded-porting-command.v1" and probe["launch_error"] is None
            and probe["source_bytes_match_before_and_after"] is True and probe["recorder_updates_gates"] is False
            and probe["recorder_supplies_reference_answers"] is False
            and same_binding(probe["source_snapshot"], build["source_snapshot"])
            and probe["repository_before"]["head"] == probe["repository_after"]["head"] == build["implementation_commit"]
            and same_binding(probe["stdout"], unit["Rust_results"]), "Actual successful canonical probe identity differs")
    require(probe["executed_launcher"]["historical_path"] == "tools/porting/record_command.py"
            and probe["executed_launcher"]["archive"]["sha256"] == build["actual_executed_recorder"]["archive"]["sha256"],
            "Actual canonical probe recorder differs from validated build recorder")
    bindings.check(probe["executed_launcher"]["archive"])
    command_paths(probe["command"], [build["binary"]["path"], cases["helper_request"]["path"]])
    require(timestamp(compilation["finished_utc"]) <= probe_started
            and timestamp(projection["created_utc"]) <= probe_started
            and timestamp(peer["review_completed_utc"]) <= probe_started, "Canonical probe chronology differs")
    execution = read(bindings.check(matrix["unit_comparison_execution"]))
    started, finished = execution_interval(execution, bindings)
    require(execution["schema"] == "recorded-porting-command.v1" and execution["launch_error"] is None
            and execution["source_bytes_match_before_and_after"] is True
            and execution["recorder_updates_gates"] is False and execution["recorder_supplies_reference_answers"] is False
            and same_binding(execution["source_snapshot"], build["source_snapshot"])
            and execution["repository_before"]["head"] == execution["repository_after"]["head"] == build["implementation_commit"],
            "Actual successful unit comparer receipt/source differs")
    require(execution["executed_launcher"]["historical_path"] == "tools/porting/record_command.py"
            and execution["executed_launcher"]["archive"]["sha256"] == build["actual_executed_recorder"]["archive"]["sha256"],
            "Actual unit comparer recorder differs from validated build recorder")
    require(exact(execution["command"], unit["actual_reader_command"])
            and path_of(unit["actual_reader_cwd"]) == ROOT
            and exact(read(bindings.check(execution["stdout"])), unit), "Unit report differs from actual successful comparer stdout")
    command = execution["command"]
    require(len(command) >= 6 and command[1:4] == ["-X", "utf8", "-B"]
            and path_of(command[4]) == path_of(projection["comparer"]["path"]), "Actual Python unit comparer owner differs")
    flags = command[5:]
    require(len(flags) % 2 == 0 and len(set(flags[::2])) == len(flags[::2]), "Actual unit comparer flags malformed")
    flag_values = dict(zip(flags[::2], flags[1::2], strict=True))
    for flag, binding in (("--original-reference", unit["original_helper"]), ("--original-data-review", unit["independent_original_review"]),
            ("--baseline-review", unit["independent_legacy_gap_review"]), ("--rust-build", unit["Rust_build"]),
            ("--rust-execution", unit["Rust_execution"]), ("--dto-projection", unit["declared_pre_execution_DTO_projection"])):
        require(flag in flag_values and path_of(flag_values[flag]) == bindings.check(binding), "Unit comparer declared argument differs: " + flag)
    require("--output-dir" in flag_values and path_of(flag_values["--output-dir"]) == bindings.check(matrix["unit_comparison"]).parent,
            "Actual unit report directory differs")
    archives = {row["historical_path"]: row["archive"] for row in unit["executed_reader_source_archives"]}
    require(len(archives) == len(unit["executed_reader_source_archives"]) == 2
            and archives[projection["comparer"]["path"]]["sha256"] == projection["comparer"]["sha256"]
            and archives["tools/porting/geo03_provenance.py"]["sha256"] == PROVENANCE_SHA,
            "Actual unit executed source archives differ from pre-execution freeze")
    for archive in archives.values():
        bindings.check(archive)
    require(probe_finished <= started <= timestamp(unit["started_utc"]) <= timestamp(unit["completed_utc"]) <= finished,
            "Actual unit comparer chronology differs")
    bindings.check(execution["executed_launcher"]["archive"])
    return original, build, finished


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--matrix", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args()
    require(Path.cwd().resolve() == ROOT, "Read from repository cwd")
    output = args.output_dir.resolve()
    require(output.is_relative_to(ROOT / ".runtime/porting/CLK-02") and not output.exists(), "Fresh contained output required")
    started = datetime.now(timezone.utc).isoformat()
    bindings, check = Bindings(), Comparison()
    bindings.check({"path": "tools/porting/geo03_provenance.py", "sha256": PROVENANCE_SHA})
    matrix_ref = ref(args.matrix)
    matrix = read(bindings.check(matrix_ref))
    require(matrix["schema"] == "clk02-production-matrix.v1" and matrix["complete"] is True and matrix["actual_command_count"] == 6
            and matrix["original_outputs_supplied_to_Rust"] is False and matrix["gates_updated"] is False, "Complete actual six-command matrix required")
    refs = {key: {"path": f"energyplus_porting_plan/contracts/CLK-02-{key}.json", "sha256": digest} for key, digest in FROZEN.items()}
    verify_contract_bindings(matrix["contracts"], refs, bindings)
    source, cases = [read(bindings.check(refs[key])) for key in ("source", "cases")]
    require(matrix["declared_plan"]["sha256"] == PLAN_SHA, "Frozen before-execution physical plan differs")
    plan = read(bindings.check(matrix["declared_plan"]))
    require(plan["schema"] == "clk02-production-plan.v1" and plan["actual_command_count"] == 6 and plan["expected_outputs_supplied"] is False
            and plan["physical_execution_performed"] is False and plan["gates_updated"] is False, "Input-only declared six-command plan required")
    verify_contract_bindings(plan["contracts"], refs, bindings)
    launcher_review = read(bindings.check(LAUNCHER_REVIEW))
    require(matrix["executed_launcher"]["sha256"] == LAUNCHER_SHA
            and launcher_review["schema"] == "clk02-independent-production-launcher-static-review.v1"
            and launcher_review["status"] == "pass-source-and-plan-before-production-launch"
            and launcher_review["reviewed_launcher"]["sha256"] == LAUNCHER_SHA
            and same_binding(launcher_review["reviewed_plan"], matrix["declared_plan"])
            and launcher_review["production_launcher_executed_by_review"] is False
            and launcher_review["scientific_comparison_executed"] is False
            and launcher_review["gates_updated"] is False, "Actual launcher differs from accepted independent source/plan review")
    bindings.check(matrix["executed_launcher"])
    expected_ids = [value["case_id"] for value in cases["Rust_physical_cases"]]
    require([value["case_id"] for value in matrix["cases"]] == [value["case_id"] for value in plan["cases"]] == expected_ids
            and len(expected_ids) == 3, "All frozen physical case identities/order required")
    for actual, declared in zip(matrix["cases"], plan["cases"], strict=True):
        require(all(exact(actual[key], declared[key]) for key in ("case_id", "scope", "duration", "input", "weather", "metadata"))
                and declared["trace_levels"] == ["full", "summary"], "Actual physical case/inputs differ from declared plan")
        for key in ("input", "weather", "metadata"):
            bindings.check(actual[key])
    unit = read(bindings.check(matrix["unit_comparison"]))
    require(unit["schema"] == "clk02-unit-comparison.v1" and unit["status"] == "pass-exact-selected-raw-and-header-state"
            and unit["scientific_certification_passed"] is True and unit["baseline_diagnostic"] is False
            and unit["comparison"]["mismatch_count"] == 0 and unit["comparison"]["comparison_count"] > 0 and unit["gates_updated"] is False,
            "Actual canonical unit comparison must pass first")
    verify_contract_bindings(unit["contracts"], refs, bindings)
    _, unit_build, unit_finished = unit_execution(matrix, unit, cases, refs, bindings)
    original = read(bindings.check(unit["original_results"]))
    require(original["schema"] == "clk02-helper-results.v1" and original["complete"] is True and original["expected_answers_supplied"] is False,
            "Actual independently reviewed native source DTO required")
    build, available, compilation = verify_rust_build(matrix["build"], bindings, kind="cli", committed=True, required_sources=REQUIRED)
    require(unit["crates_tree"] == unit_build["crates_tree"] == build["crates_tree"] and unit["committed_source_certification"] is True
            and unit["implementation_commit"] == unit_build["implementation_commit"] == build["implementation_commit"]
            and same_binding(unit_build["source_snapshot"], build["source_snapshot"]), "Unit and CLI Rust source tree differs")
    completed = max(unit_finished, timestamp(compilation["finished_utc"]))
    planned = max(timestamp(plan["created_utc"]), timestamp(launcher_review["review_completed_utc"]))
    directories, observations, raw_commands = set(), [], 0
    for case in matrix["cases"]:
        ordinary, summaries, traces = {}, {}, {}
        for level in ("Full", "Summary"):
            directory, summary, artifacts = recorded_run(case, case[level], level, build, completed, planned, bindings, directories)
            ordinary[level], summaries[level] = directory, summary
            raw_commands += 1
            if level == "Summary":
                require(not any((directory / name).exists() for name in FULL_ONLY), "Summary activated a Full-only observer")
            else:
                require(directory / TRACE_NAME in artifacts, "Full missing actual raw/consumer observer artifact")
                trace_ref = artifacts[directory / TRACE_NAME]
                observed = observe(check, read(bindings.check(trace_ref)), original, source, case, available, bindings)
                observed["actual_fixture_demand_injection_registration"] = {
                    "available": summary["rust_runtime"]["fixture_demand_injection_used"] is not None,
                    "value": summary["rust_runtime"]["fixture_demand_injection_used"],
                    "unavailable_counted_as_false": False,
                }
                observed["actual_Full_trace"] = trace_ref
                observations.append(observed)
                traces[level] = trace_ref
        full, summary = summaries["Full"], summaries["Summary"]
        check.equal(sorted(full), sorted(summary), case["case_id"] + "/ordinary-summary-keys", "Full_Summary_ordinary_equality")
        for key in set(full) - {"artifacts", "timing", "input", "config"}:
            check.equal(full[key], summary[key], case["case_id"] + "/ordinary-summary/" + key, "Full_Summary_ordinary_equality")
        check.equal({key: value for key, value in full["config"].items() if key != "trace_level"},
            {key: value for key, value in summary["config"].items() if key != "trace_level"}, case["case_id"] + "/ordinary-config", "Full_Summary_ordinary_equality")
        for name in ("results/selected-outputs.csv", "results/meters.csv"):
            a, b = [bindings.check(ref(ordinary[level] / name)) for level in ("Full", "Summary")]
            check.equal(a.read_bytes() == b.read_bytes(), True, case["case_id"] + "/" + name, "Full_Summary_export_bytes")
        for name in ("results/result-store.json", "porting_scope.json"):
            check.equal(read(bindings.check(ref(ordinary["Full"] / name))), read(bindings.check(ref(ordinary["Summary"] / name))),
                case["case_id"] + "/" + name, "Full_Summary_export_values")
    require(raw_commands == matrix["actual_command_count"] == 6, "Actual six distinct CLI receipts required")
    bindings.unchanged()
    output.mkdir()
    archives = []
    for path in (Path(__file__).resolve(), Path(__file__).with_name("geo03_provenance.py")):
        archive = output / path.name
        shutil.copyfile(path, archive)
        require(path.read_bytes() == archive.read_bytes(), "Executed reader archive differs")
        archives.append({"historical_path": path.relative_to(ROOT).as_posix(), "archive": ref(archive)})
    report = {
        "schema": "clk02-production-comparison.v1", "status": "pass-literal-raw-owner-and-consumer-handoff" if check.mismatch_count == 0 else "fail-production-handoff",
        "started_utc": started, "completed_utc": datetime.now(timezone.utc).isoformat(), "actual_reader_command": list(sys.orig_argv),
        "contracts": refs, "Rust_matrix": matrix_ref, "declared_plan": matrix["declared_plan"], "unit_comparison": matrix["unit_comparison"],
        "unit_comparison_execution": matrix["unit_comparison_execution"],
        "executed_production_launcher": matrix["executed_launcher"], "independent_production_launcher_review": LAUNCHER_REVIEW,
        "original_results": unit["original_results"], "Rust_build": matrix["build"], "implementation_commit": build["implementation_commit"],
        "crates_tree": build["crates_tree"], "actual_command_count": raw_commands,
        "comparison_count": check.count, "mismatch_count": check.mismatch_count, "comparison_categories": dict(check.categories),
        "representative_mismatches": check.failures, "mismatch_payload_limit": 1000,
        "mismatch_payloads_truncated": check.mismatch_count > len(check.failures), "observations": observations,
        "executed_reader_source_archives": archives, "available_source_inventory_count": len(available),
        "source_inventory_scope": "Available immutable Rust source bytes; not a compiler file-selection inventory",
        "engines_executed_by_reader": False, "Cargo_executed_by_reader": False, "Git_executed_by_reader": False,
        "original_outputs_supplied_to_Rust": False, "original_RHS_reconstructed": False, "assignment_tolerance_used": False, "gates_updated": False,
        "limits": ["Only actual selected raw outputs/header/stream observations, literal copied record fields and actual consumer record receipt are paired.",
            "The liquid compatibility field is checked from actual projected record to actual consumer; source normalization/replacement policy remains unpaired.",
            "Computed sample values are preserved and their own typed IEEE storage validated, without native interpolation or weather-physics equality claims.",
            "Actual Rust caller/context/phase/order is observed; native callback, civil calendar and warmup correspondence is not inferred.",
            "Summary establishes observer absence and unchanged ordinary results; it supplies no raw/consumer snapshots.",
            "Card closure, later weather selection/lifecycle, annual performance and downstream thermal/HVAC science need separate evidence."],
    }
    with (output / "production-comparison.json").open("x", encoding="utf-8", newline="\n") as stream:
        stream.write(json.dumps(report, indent=2, allow_nan=False) + "\n")
    print(json.dumps(report, allow_nan=False))
    if check.mismatch_count:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
