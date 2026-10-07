#!/usr/bin/env python3
"""Exact CLK-01 unit/projection comparison; never injects answers or closes gates.

Original-only mutable-state diagnostics are reported separately. Prepared
calendar rows do not constitute physical invocation or building/HVAC proof.
"""
from __future__ import annotations

import argparse
from collections import Counter
import json
from pathlib import Path
import sys

sys.dont_write_bytecode = True
import clk01_reference as reference

ROOT = reference.ROOT
ELIGIBLE = set(reference.PAIRED_HELPERS)
DAY_FIELDS = ["year", "month", "day_of_month", "gregorian_day_of_year", "weather_day_of_year",
              "schedule_day_of_year", "gregorian_day_of_week", "day_of_week", "day_type",
              "gregorian_year_is_leap_year", "weather_effective_year_is_leap_year", "leap_year_add"]


def read(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def same(left: object, right: object) -> bool:
    # bool==int must not conceal a typed contract mismatch.
    if type(left) is not type(right):
        return False
    if isinstance(left, dict):
        return left.keys() == right.keys() and all(same(left[key], right[key]) for key in left)
    if isinstance(left, list):
        return len(left) == len(right) and all(same(a, b) for a, b in zip(left, right))
    return left == right


class Checks:
    def __init__(self) -> None:
        self.count = 0
        self.mismatches: list[dict] = []

    def equal(self, label: str, actual: object, original: object) -> None:
        self.count += 1
        if not same(actual, original):
            self.mismatches.append({"field": label, "actual": actual, "original": original})

    def require(self, label: str, condition: bool) -> None:
        self.equal(label, bool(condition), True)


def helpers(request: dict, original: dict, rust: dict, checks: Checks) -> dict:
    checks.equal("helper request schema", request.get("schema"), "clk01-helper-tuples.v1")
    for name, output in [("original", original), ("Rust", rust)]:
        checks.equal(name + " helper schema", output.get("schema"), "clk01-helper-results.v1")
        checks.equal(name + " helper count", len(output["calls"]), len(request["calls"]))
    checks.require("original helper reference kind", original.get("reference_kind") in
                   ["pinned-original-pure-body-fallback", "linked-original-source-with-genuine-state"])
    checks.equal("frozen helper request", request, reference.helper_request())
    paired, unpaired, diagnostics, unavailable = (Counter() for _ in range(4))
    statistics = {}
    seen = set()
    for position, (input_row, cpp, candidate) in enumerate(zip(request["calls"], original["calls"], rust["calls"])):
        label = f"helper[{position}]"
        index = input_row["call_index"]
        checks.require(label + ".unique_index", index not in seen)
        seen.add(index)
        for key in ["call_index", "function", "inputs", "context"]:
            checks.equal(label + ".CPP_identity." + key, cpp.get(key), input_row[key])
            checks.equal(label + ".Rust_identity." + key, candidate.get(key), input_row[key])
        function = input_row["function"]
        if function in ELIGIBLE:
            checks.equal(label + ".CPP_supported", cpp.get("supported"), True)
            checks.equal(label + ".Rust_supported", candidate.get("supported"), True)
            checks.equal(label + ".resolved_inputs", candidate.get("resolved_inputs"), cpp.get("resolved_inputs"))
            checks.equal(label + ".value", candidate.get("value"), cpp.get("value"))
            paired[function] += 1
            stats = statistics.setdefault(function, {"paired_count": 0, "typed_value_mismatch_count": 0,
                                                       "numeric_count": 0, "max_absolute_error": 0, "sum_squared_error": 0})
            stats["paired_count"] += 1
            a, b = candidate.get("value"), cpp.get("value")
            if not same(a, b):
                stats["typed_value_mismatch_count"] += 1
            if type(a) is type(b) and type(a) in [bool, int]:
                error = abs(int(a) - int(b))
                stats["numeric_count"] += 1
                stats["max_absolute_error"] = max(stats["max_absolute_error"], error)
                stats["sum_squared_error"] += error * error
        else:
            unpaired[function] += 1
            checks.equal(label + ".Rust_unpaired", candidate.get("supported"), False)
            checks.require(label + ".no_invented_Rust_value_or_state",
                           all(key not in candidate for key in ["value", "state_before", "state_after"]))
            if cpp.get("supported") is True:
                diagnostics[function] += 1
                if function == "calculateDayOfWeek":
                    checks.equal(label + ".source_before", cpp.get("state_before"),
                                 {"day_of_week": input_row["inputs"]["day_of_week_before"]})
                    checks.equal(label + ".source_return_matches_write", cpp.get("state_after"),
                                 {"day_of_week": cpp.get("value")})
                if function == "SetupWeekDaysByMonth":
                    inputs = input_row["inputs"]
                    before = {"end_day_of_month": inputs["end_day_of_month"], "leap_year_add": inputs["leap_year_add"],
                              "week_days": inputs["week_days_before"]}
                    checks.equal(label + ".source_before", cpp.get("state_before"), before)
                    for key in ["end_day_of_month", "leap_year_add"]:
                        checks.equal(label + ".source_read_preserved." + key, cpp.get("state_after", {}).get(key), inputs[key])
                    checks.equal(label + ".source_result_matches_write", cpp.get("state_after", {}).get("week_days"), cpp.get("value"))
                    checks.equal(label + ".source_array_length", len(cpp.get("value", [])), 12)
            else:
                unavailable[function] += 1
                checks.equal(label + ".CPP_unavailable", cpp.get("supported"), False)
                checks.require(label + ".CPP_no_fabricated_value", "value" not in cpp)
    checks.equal("eligible helper count", sum(paired.values()), 1912)
    for stats in statistics.values():
        stats["rmse"] = (stats["sum_squared_error"] / stats["numeric_count"]) ** 0.5 if stats["numeric_count"] else None
    return {"total_calls": len(request["calls"]), "paired_calls": sum(paired.values()), "paired_routine_counts": dict(paired),
            "unpaired_routine_counts": dict(unpaired), "original_source_only_diagnostics": dict(diagnostics),
            "original_stateful_diagnostics_unavailable": dict(unavailable), "unpaired_counted_as_paired_pass": False,
            "numeric_comparison": "exact typed bool/integer equality", "routine_statistics": statistics}


def calendars(request: dict, original: dict, rust: dict, checks: Checks) -> dict:
    checks.equal("calendar request schema", request.get("schema"), "clk01-calendar-cases.v1")
    for name, output in [("original", original), ("Rust", rust)]:
        checks.equal(name + " calendar schema", output.get("schema"), "clk01-calendar-results.v1")
        checks.equal(name + " calendar count", len(output["cases"]), len(request["cases"]))
    checks.equal("original calendar kind", original.get("reference_kind"), "linked-original-inputprocessor-and-genuine-state")
    checks.equal("frozen calendar request", request, {"schema": "clk01-calendar-cases.v1", "cases": reference.calendar_cases()})
    checks.equal("calendar case count", len(request["cases"]), 15)
    annual_months: dict[int, int] = {}
    for candidate, item in zip(rust["cases"], request["cases"]):
        if item["duration"] == "ANNUAL":
            rows = candidate.get("month_start_frames", [])
            mapping = {row["month"]: row["day_of_week"] for row in rows}
            checks.equal(item["case_id"] + ".annual_month_count", len(rows), 12)
            checks.equal(item["case_id"] + ".annual_month_set", sorted(mapping), list(range(1, 13)))
            if not annual_months:
                annual_months = mapping
            else:
                checks.equal(item["case_id"] + ".annual_month_correspondence", mapping, annual_months)
    checks.equal("annual monthly coverage", len(annual_months), 12)
    summaries, day_total = [], 0
    for item, cpp, candidate in zip(request["cases"], original["cases"], rust["cases"]):
        cid = item["case_id"]
        before_mismatch = len(checks.mismatches)
        checks.equal(cid + ".CPP_case_id", cpp.get("case_id"), cid)
        checks.equal(cid + ".Rust_case_id", candidate.get("case_id"), cid)
        checks.equal(cid + ".Rust_status", candidate.get("status"), "resolved")
        checks.equal(cid + ".preparation_only", candidate.get("preparation_only"), True)
        checks.equal(cid + ".physics_executed", candidate.get("physics_executed"), False)
        checks.equal(cid + ".original_input_identity", cpp.get("source_metadata"), item["source_metadata"])
        resolved, actual = cpp["resolved_calendar"], candidate["resolved_calendar"]
        resolved_fields = ["start_year", "start_month", "start_day_of_month", "start_year_is_leap_year",
                           "end_year", "end_month", "end_day_of_month", "end_year_is_leap_year", "total_days"]
        checks.equal(cid + ".resolved_fields", sorted(actual), sorted(resolved_fields))
        for key in resolved_fields:
            checks.equal(cid + ".resolved." + key, actual.get(key), resolved.get(key))
        checks.equal(cid + ".name", candidate.get("run_period_name", "").upper(), resolved.get("run_period_name", "").upper())
        env, weather = cpp["environment"], candidate["weather_calendar"]
        checks.equal(cid + ".weather_policy", weather.get("weather_file_allows_leap_years"), item["weather_calendar"]["leap_year_observed"])
        checks.equal(cid + ".weather_days", weather.get("total_days"), env["total_days"])
        for key in ["start_year_is_weather_effective_leap_year", "end_year_is_weather_effective_leap_year"]:
            checks.equal(cid + ".weather." + key, weather.get(key), env.get("weather_effective_year_is_leap_year"))
        source_days, actual_days = cpp["days"], candidate["daily_frames"]
        checks.equal(cid + ".daily_count", len(actual_days), len(source_days))
        checks.equal(cid + ".original_days_vs_environment", len(source_days), env["total_days"])
        checks.equal(cid + ".hourly_count", candidate.get("hourly_sample_count"), len(source_days) * 24)
        zone_count = len(source_days) * 24 * item["zone_timesteps_per_hour"]
        checks.equal(cid + ".zone_count", candidate.get("zone_timestep_sample_count"), zone_count)
        checks.equal(cid + ".prepared_zone_rows", len(candidate.get("prepared_zone_points", [])), zone_count)
        for position, (source, row) in enumerate(zip(source_days, actual_days)):
            for field in DAY_FIELDS:
                checks.equal(f"{cid}.day[{position}].{field}", row.get(field), source.get(field))
            checks.equal(f"{cid}.day[{position}].label", row.get("day_type_label"), source.get("day_type_name"))
            checks.equal(f"{cid}.day[{position}].day_of_sim", row.get("day_of_sim"), position + 1)
            checks.equal(f"{cid}.day[{position}].hourly_index", row.get("hourly_sample_index"), position * 24)
            checks.equal(f"{cid}.day[{position}].no_DST_after_CON", row.get("dst"), False)
            checks.equal(f"{cid}.day[{position}].no_special_after_CON", row.get("special_day_type"), None)
        monthly = resolved["monthly_start_weekdays"]
        checks.equal(cid + ".monthly_length", len(monthly), 12)
        checks.equal(cid + ".GetRP_array_vs_consumed_annual_projection", monthly, [annual_months.get(month) for month in range(1, 13)])
        checks.equal(cid + ".SetupEnvironment_monthly_copy", env.get("monthly_start_weekdays"), monthly)
        source_months = [row for row in source_days if row["day_of_month"] == 1]
        actual_months = candidate.get("month_start_frames", [])
        checks.equal(cid + ".consumed_month_start_count", len(actual_months), len(source_months))
        for position, (source, row) in enumerate(zip(source_months, actual_months)):
            for field in DAY_FIELDS:
                checks.equal(f"{cid}.month[{position}].{field}", row.get(field), source.get(field))
            checks.equal(f"{cid}.month[{position}].source_array", row.get("day_of_week"), monthly[row["month"] - 1])
        metadata = read(ROOT / item["source_metadata"]["metadata"]["path"])
        designs = metadata["object_counts"].get("sizingperiod:designday", 0)
        checks.equal(cid + ".source_environment_mapping", env.get("selected_source_environment_ordinal"), designs + 1)
        checks.equal(cid + ".parser_design_count", cpp["state_before_setup_environment"].get("total_design_day_definitions"), designs)
        checks.equal(cid + ".GetRP_written_weekday", cpp["state_after_get_run_period"].get("day_of_week"), resolved.get("start_day_of_week"))
        checks.equal(cid + ".source_start_weekday", resolved.get("start_day_of_week"), item["run_period"]["start_weekday"])
        checks.equal(cid + ".source_kind", env.get("kind_of_sim"), 3)
        checks.equal(cid + ".source_actual_weather", env.get("actual_weather"), False)
        summaries.append({"case_id": cid, "paired_daily_rows": len(source_days), "prepared_zone_rows": zone_count,
                          "monthly_entries_compared": 12, "mismatch_count": len(checks.mismatches) - before_mismatch,
                          "physics_executed": False})
        day_total += len(source_days)
    return {"case_count": len(request["cases"]), "paired_daily_rows": day_total, "cases": summaries,
            "annual_month_start_count": len(annual_months), "monthly_arrays_paired_to_consumed_projection": True,
            "direct_SetupWeekDaysByMonth_state_cloned_in_Rust": False,
            "zone_clock_bits": "Recorded; correspondence to native API is a separate production check",
            "production_or_physics_pass_claimed": False}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ["cpp-helpers", "rust-helpers", "cpp-calendar", "rust-calendar", "cpp-build-receipt", "rust-build-receipt"]:
        parser.add_argument("--" + name, type=Path)
    parser.add_argument("--helper-request", type=Path, default=reference.DEFAULT_RUNTIME / "helper-tuples.json")
    parser.add_argument("--calendar-request", type=Path, default=reference.DEFAULT_RUNTIME / "calendar-cases.json")
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args()
    if bool(args.cpp_helpers) != bool(args.rust_helpers) or bool(args.cpp_calendar) != bool(args.rust_calendar):
        parser.error("Supply both original and Rust output for each selected comparison")
    if not args.cpp_helpers and not args.cpp_calendar:
        parser.error("Select helper and/or calendar outputs")
    report_path = args.output_dir.resolve() / "unit-comparison.json"
    if report_path.exists():
        raise ValueError("Preserve previous results; choose a fresh output directory")
    reference.verify_pins()
    contracts = reference.frozen_contracts(True)
    tolerance = read(reference.CONTRACTS / "CLK-01-tolerances.json")
    if not tolerance.get("frozen_before_comparison") or any(row["atol"] != 0 or row["rtol"] != 0 for row in tolerance["rows"]):
        raise ValueError("CLK-01 requires the frozen exact tolerances")
    checks, results, paths = Checks(), {}, []
    if args.cpp_helpers:
        results["helpers"] = helpers(read(args.helper_request), read(args.cpp_helpers), read(args.rust_helpers), checks)
        paths += [args.helper_request, args.cpp_helpers, args.rust_helpers]
    if args.cpp_calendar:
        results["calendar"] = calendars(read(args.calendar_request), read(args.cpp_calendar), read(args.rust_calendar), checks)
        paths += [args.calendar_request, args.cpp_calendar, args.rust_calendar]
    receipts = [reference.ref(path) for path in [args.cpp_build_receipt, args.rust_build_receipt] if path]
    report = {"schema": "clk01-unit-comparison.v1", "status": "pass" if not checks.mismatches else "fail",
              "comparison_count": checks.count, "mismatch_count": len(checks.mismatches), "mismatches": checks.mismatches,
              "contracts": contracts, "checker": reference.ref(Path(__file__)),
              "inputs_and_outputs": [reference.ref(path) for path in paths], "build_receipts": receipts, "results": results,
              "limits": ["Eligible helpers and preparation projections only; unpaired diagnostics are not Rust PASS",
                         "Production invocation/native clock/ESO phases use the separate production proof",
                         "Review execution/source/build provenance before closure"],
              "gates_updated": False}
    report_path.parent.mkdir(parents=True, exist_ok=True)
    reference.write(report_path, report)
    print(json.dumps({"status": report["status"], "comparison_count": checks.count, "mismatch_count": len(checks.mismatches),
                      "report": reference.ref(report_path)}))
    if checks.mismatches:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
