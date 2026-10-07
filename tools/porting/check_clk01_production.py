"""Compare prepared and actually consumed Rust calendars with original C++.

Prepared annual rows remain preparation evidence. Actual runs compare every real
zone invocation with pre-report native callbacks and retain separate raw system
clock/year-end fields. This tool never supplies C++ results to Rust or closes gates.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import shutil
import struct
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / "energyplus_porting_plan"
FIELDS = ["year", "month", "day_of_month", "gregorian_day_of_year", "weather_day_of_year",
          "schedule_day_of_year", "gregorian_day_of_week", "day_of_week", "day_type",
          "gregorian_year_is_leap_year", "weather_effective_year_is_leap_year", "leap_year_add"]


def read(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def write(path: Path, value) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, allow_nan=False) + "\n", encoding="utf-8", newline="\n")


def sha(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def ref(path: Path) -> dict:
    return {"path": path.resolve().relative_to(ROOT).as_posix(), "sha256": sha(path)}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def bits(value: float) -> str:
    return struct.pack(">d", value).hex()


def same_day(actual: dict, original: dict, label: str) -> None:
    require(all(type(actual.get(key)) is type(original[key]) and actual[key] == original[key] for key in FIELDS),
            f"Calendar value/type mismatch: {label}")
    require(isinstance(actual["day_type_label"], str)
            and actual["day_type_label"].casefold() == original["day_type_name"].casefold(), f"Day-type label mismatch: {label}")
    require(actual["dst"] is False and actual["special_day_type"] is None,
            f"Unexpected active DST/special day: {label}")


def native_review(case: dict, original: dict, native_root: Path) -> dict:
    directory = native_root / case["id"]
    receipt = read(directory / "receipt.json")
    require(receipt["observation_checks_passed"] and receipt["errors"] == [], "Native observation failed")
    require(receipt["input"] == case["input"] and receipt["weather"] == case["weather"], "Native inputs differ")
    require(ref(ROOT / receipt["observer_source"]["path"]) == receipt["observer_source"], "Executed native observer changed")
    for key in ["native_library", "native_error_file", "api_messages"]:
        require(receipt[key] is not None and ref(ROOT / receipt[key]["path"]) == receipt[key],
                f"Native {key} changed/missing")
    require(receipt["native_library"]["sha256"] == "bbe6f66240d108df9f51a03b9beb968808a2173234253574d27a31605aec2aa2",
            "Native library no longer matches the source-pinned original DLL")
    for binding in receipt["python_api_sources"]:
        require(ref(ROOT / binding["path"]) == binding, "Native Python API source changed")
    for binding in receipt["native_outputs"]:
        require(ref(ROOT / binding["path"]) == binding, "Native output changed after execution")
    for binding in receipt["frozen_contracts"]:
        current = ROOT / binding["path"]
        preserved = native_root / "source/frozen-contracts" / current.name
        require((current.is_file() and sha(current) == binding["sha256"])
                or (preserved.is_file() and sha(preserved) == binding["sha256"]),
                "Initial native contract bytes unavailable")
    raw = ROOT / receipt["raw_callbacks"]["path"]
    require(ref(raw) == receipt["raw_callbacks"], "Native raw callbacks changed")
    rows = [json.loads(line) for line in raw.read_text(encoding="utf-8").splitlines()]
    before = [row for row in rows if row["phase"] == "before_init_heat_balance"]
    after = [row for row in rows if row["phase"] == "after_zone_reporting"]
    count = case["expected_zone_steps_excluding_warmup"]
    require(len(before) == len(after) == count == len(original["days"]) * 96, "Native/unit duration mismatch")
    require([row["phase"] for row in rows if row["phase"] != "warmup_complete"] ==
            [phase for _ in range(count) for phase in ["before_init_heat_balance", "after_zone_reporting"]],
            "Preserved native callback order differs")
    for index, row in enumerate(before):
        day = original["days"][index // 96]
        projection = {"calendar_year": day["year"], "month": day["month"], "day_of_month": day["day_of_month"],
                      "day_of_week": day["day_of_week"], "weather_day_of_year": day["weather_day_of_year"]}
        require(all(row[key] == value for key, value in projection.items()), f"Native calendar mismatch at {index}")
        require(row["zone_timestep"] == index % 4 + 1 and row["hour_zero_based"] == index % 96 // 4
                and row["timesteps_per_hour"] == 4 and row["zone_timestep_hours"] == 0.25,
                f"Native zone interval mismatch at {index}")
        require(row["raw_c_api_dst_indicator"] == row["raw_c_api_holiday_index"] == 0,
                f"Unexpected original active date override at {index}")
        require(row["native_environment_number"] == original["environment"]["selected_source_environment_ordinal"],
                "Parsed original environment ordinal differs from live native state")
    eso_path = directory / "eplusout.eso"
    stamps = []
    with eso_path.open(encoding="utf-8", errors="strict") as stream:
        for line in stream:
            values = line.strip().split(",")
            if len(values) == 9 and values[0] == "2" and values[2].strip().isdigit():
                stamps.append(values)
    require(len(stamps) == count, "Original ESO zone timestamp count differs")
    for index, stamp in enumerate(stamps):
        row, day = before[index], original["days"][index // 96]
        require([int(stamp[q]) for q in [1, 2, 3, 4, 5]] ==
                [index // 96 + 1, row["month"], row["day_of_month"], row["raw_c_api_dst_indicator"], row["hour_zero_based"] + 1],
                f"Native ESO current date/hour mismatch at {index}")
        require(float(stamp[6]) == (row["zone_timestep"] - 1) * 15.0
                and float(stamp[7]) == row["zone_timestep"] * 15.0
                and stamp[8].strip().casefold() == day["day_type_name"].casefold(),
                f"Native ESO interval/day-type mismatch at {index}")
    return {"receipt": ref(directory / "receipt.json"), "raw_callbacks": ref(raw), "eso": ref(eso_path),
            "rows": before, "eso_rows": stamps, "verified_zone_count": count,
            "raw_system_clock_is_canonical_zone_clock": False,
            "first_weather_record_year": before[0]["weather_record_year"],
            "last_weather_record_year": before[-1]["weather_record_year"],
            "final_before_report_calendar_year": before[-1]["calendar_year"],
            "final_after_report_calendar_year": after[-1]["calendar_year"]}


def execute(command: list[str], directory: Path, label: str, source: Path, review: bool) -> dict:
    path = directory / (label + "-execution.json")
    if review:
        result = read(path)
        require(result["command"] == command and result["exit_code"] == 0, "Preserved CLI command failed/differs")
        for key in ["stdout", "stderr"]:
            require(ref(ROOT / result[key]["path"]) == result[key], "Preserved CLI log changed")
        require(ref(ROOT / result["executed_checker"]["path"]) == result["executed_checker"], "Originally executed checker changed")
        return result
    started = time.perf_counter()
    stdout, stderr = directory / (label + "-stdout.log"), directory / (label + "-stderr.log")
    with stdout.open("wb") as out, stderr.open("wb") as err:
        completed = subprocess.run(command, cwd=ROOT, stdout=out, stderr=err, check=False)
    result = {"command": command, "exit_code": completed.returncode,
              "elapsed_seconds": round(time.perf_counter() - started, 3),
              "stdout": ref(stdout), "stderr": ref(stderr), "executed_checker": ref(source),
              "repository_head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()}
    write(path, result)
    require(completed.returncode == 0, f"CLI failed; see {stderr}")
    return result


def run_case(case: dict, original: dict, native_root: Path, binary: Path, output_root: Path,
             actual: bool, source: Path, review: bool, provenance: dict) -> dict:
    mode = "actual" if actual else "prepared"
    directory = output_root / case["id"] / mode
    require(directory.exists() if review else not directory.exists(), "Fresh CLI output root required")
    directory.mkdir(parents=True, exist_ok=review)
    for binding in [case["input"], case["weather"]]:
        require(sha(ROOT / binding["path"]) == binding["sha256"], "Fixed input/weather pin changed")
    require(original["source_metadata"]["input"] == case["input"]
            and original["source_metadata"]["weather"] == case["weather"], "Original C++ calendar used different inputs")
    native = native_review(case, original, native_root)
    commands = []
    for level in (["full", "summary"] if actual else ["summary"]):
        command = [str(binary), "run", str(ROOT / case["input"]["path"]), "--weather", str(ROOT / case["weather"]["path"]),
                   "--output-dir", str(directory / level), "--porting-scope", case["scope"],
                   "--mode", "compatibility", "--partial", "deny", "--trace-level", level]
        if not actual:
            command.append("--dry-run")
        print(f"{'REVIEW' if review else 'RUN'} {case['id']} {mode} {level}", flush=True)
        commands.append(execute(command, directory, "cli-" + level, source, review))
        run = read(directory / level / "run-summary.json")
        require(run["status"] == "success" and run["exit_code"] == 0 and run["oracle"] is None
                and run["config"]["compare_oracle"] is False and run["config"]["oracle_baseline"] is False
                and run["config"]["dry_run"] is (not actual), "CLI execution/oracle/dry configuration differs")
        if actual:
            require(run["oracle_status"] == "not-requested" and run["rust_runtime"]["samples"] == len(original["days"]) * 24,
                    "Actual Full/Summary hourly duration or oracle status differs")
            if case["scope"] == "B":
                require(run["rust_runtime"]["purchased_air_coupling_call_count"] == case["expected_zone_steps_excluding_warmup"],
                        "Actual Full/Summary B coupling duration differs")
        else:
            require(run["rust_runtime"] is None and run["oracle_status"] == "skipped-dry-run", "Dry preparation executed physics/oracle")
        scope = read(directory / level / "porting_scope.json")
        require(scope["admissible"] and scope["violations"] == [], "Production admission failed")
        require(all(scope["production"][key] is False for key in
                    ["oracle_inputs_used", "fixture_inputs_used", "conformance_claim"]), "Unexpected injection/conformance claim")
        prepared = scope["environment"]["prepared_calendar"]
        require(prepared["preparation_only"] and prepared["physics_executed"] is False, "Prepared rows were mislabeled")
        daily = prepared["daily_frames"]
        require(prepared["run_period_name"] == original["resolved_calendar"]["run_period_name"]
                and prepared["hourly_sample_count"] == len(original["days"]) * 24, "Prepared period identity/duration differs")
        require(len(daily) == len(original["days"]), "Prepared daily row count differs")
        for index, (row, day) in enumerate(zip(daily, original["days"])):
            same_day(row, day, f"{case['id']} prepared day {index}")
            require(row["hourly_sample_index"] == index * 24 and row["day_of_sim"] == index + 1
                    and row["hour_ending"] == 1, "Prepared daily index differs")
        month_days = [(index, day) for index, day in enumerate(original["days"]) if day["day_of_month"] == 1]
        require(len(prepared["month_start_frames"]) == len(month_days), "Prepared month-start row count differs")
        for row, (index, day) in zip(prepared["month_start_frames"], month_days):
            same_day(row, day, f"{case['id']} month start")
            require(row == daily[index] and row["day_of_week"] == original["resolved_calendar"]["monthly_start_weekdays"][row["month"] - 1],
                    "Prepared monthly projection differs from original GetRunPeriodData array")
    require(not (directory / "summary/clock-calls.json").exists(), "Summary captured clock events")
    require(not (directory / "summary/psychrometrics-calls.json").exists(), "Summary unexpectedly captured kernels")
    result = {"schema": "clk01-production-case.v1", "case_id": case["id"], "mode": mode,
              "binary": ref(binary), "checker": ref(source), "commands": commands,
              "input": case["input"], "weather": case["weather"],
              "prepared_daily_count": len(original["days"]), "prepared_calendar_mismatches": 0,
              "prepared_calendar": ref(directory / "summary/porting_scope.json"),
              "native_reference": {key: value for key, value in native.items() if key not in ["rows", "eso_rows"]},
              "summary_clock_artifact_absent": True, "physics_executed": actual,
              "automatic_gate_updates": False}
    if actual:
        full = directory / "full"
        run = read(full / "run-summary.json")
        require(run["status"] == "success" and run["exit_code"] == 0 and run["oracle"] is None
                and run["oracle_status"] == "not-requested", "Actual ordinary CLI did not complete without oracle")
        trace = read(full / "clock-calls.json")
        count = case["expected_zone_steps_excluding_warmup"]
        events, frames = trace["zone_invocations"], trace["prepared_hourly_frames"]
        require(trace["schema"] == "clk01-clock-trace.v1" and trace["complete_on_collecting_thread"]
                and trace["omitted_invocation_count"] == 0 and trace["truncation_reason"] is None
                and len(events) == trace["total_invocation_count"] == trace["recorded_invocation_count"] == count,
                "Actual clock observation is incomplete")
        require(len(frames) == count // 4 and run["rust_runtime"]["samples"] == count // 4,
                "Actual runtime did not consume the declared hourly axis")
        require(trace["source_environment_number"] is None, "Rust invented a native source environment ordinal")
        is_b = case["scope"] == "B"
        require(trace["materialized_environment_index"] == (1 if is_b else None)
                and trace["run_period_name"] == original["resolved_calendar"]["run_period_name"], "Actual copied axis identity differs")
        environment = trace["prepared_environment_points"]
        require(len(environment) == (count if is_b else 0), "Materialized Rust environment axes differ")
        require(trace["parsed_input_design_day_declaration_count"] == original["state_before_setup_environment"]["total_design_day_definitions"],
                "Parsed design-day provenance differs from original parser")
        for index, event in enumerate(events):
            require(event["sequence"] == index + 1 and event["hour_index"] == index // 4
                    and event["calendar_frame_index"] == index // 4 and event["zone_timestep"] == index % 4 + 1
                    and event["zone_steps_per_hour"] == 4 and event["timestep_seconds_bits"] == bits(900.0)
                    and event["environment_point_index"] == (index if is_b else None),
                    f"Actual physical hook order/operands differ at {index}")
            frame = frames[event["calendar_frame_index"]]
            same_day(frame, original["days"][index // 96], f"actual step {index}")
            require(frame["hour_ending"] == native["rows"][index]["hour_zero_based"] + 1
                    and frame["day_of_sim"] == index // 96 + 1, "Actual referenced hour/day differs")
            if is_b:
                point, stamp = environment[index], native["eso_rows"][index]
                require(point["sample_index"] == index and point["simulation_timestep"] == index + 1
                        and point["zone_timestep"] == index % 4 + 1 and point["materialized_environment_index"] == 1,
                        "Actual B copied point identity differs")
                require(point["start_minute_bits"] == bits(float(stamp[6]))
                        and point["end_minute_bits"] == bits(float(stamp[7])), "Actual canonical interval differs from native ESO")
                require(point["current_time_hours_bits"] == bits(int(stamp[5]) - 1 + float(stamp[7]) / 60.0),
                        "Actual copied canonical zone clock differs from original ESO projection")
        if is_b:
            require(run["rust_runtime"]["purchased_air_coupling_call_count"] == count, "Actual B coupling duration differs")
        equality = {}
        for name in ["selected-outputs.csv", "meters.csv", "result-store.json"]:
            left, right = full / "results" / name, directory / "summary/results" / name
            equal = left.read_bytes() == right.read_bytes() if name.endswith("csv") else read(left)["series"] == read(right)["series"]
            require(equal, f"Clock observation affected numerical outputs: {name}")
            equality[name] = {"full": ref(left), "summary": ref(right), "equal": True}
        result.update({"clock_trace": ref(full / "clock-calls.json"), "actual_zone_invocations": count,
                       "actual_clock_mismatches": 0, "full_summary_numerical_equality": equality,
                       "actual_runtime_summary": ref(full / "run-summary.json")})
    result["checks_passed"] = True
    result["boundary"] = "Current consumed calendar and canonical zone interval only; raw native system clock, reporting mutation, Tomorrow/cursor, SYS/warmup and whole physics parity remain separate."
    result.update(provenance)
    write(directory / "comparison.json", result)
    print(f"PASS {case['id']} {mode}: days={len(original['days'])}, actual_steps={case['expected_zone_steps_excluding_warmup'] if actual else 0}", flush=True)
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--case", action="append", required=True)
    parser.add_argument("--actual", action="store_true")
    parser.add_argument("--review-existing", action="store_true")
    parser.add_argument("--cpp-calendar", type=Path, required=True)
    parser.add_argument("--native-root", type=Path, required=True)
    parser.add_argument("--output-root", type=Path, required=True)
    args = parser.parse_args()
    build_path = PLAN / "evidence/CLK-01/build-receipt.json"
    build = read(build_path)
    binary_binding = next(item for item in build["artifacts"] if Path(item["path"]).name == "eplus-rs.exe")
    binary = ROOT / binary_binding["path"]
    require(ref(binary) == binary_binding, "Archived committed CLI changed")
    require(subprocess.check_output(["git", "rev-parse", "HEAD:crates"], cwd=ROOT, text=True).strip() == build["crates_tree"]
            and subprocess.run(["git", "diff", "--quiet", "HEAD", "--", "crates"], cwd=ROOT).returncode == 0,
            "Committed production crates tree differs")
    cpp_reference_path = args.cpp_calendar.resolve().with_name("calendar-reference.json")
    cpp_reference = read(cpp_reference_path)
    require(cpp_reference["exit_code"] == 0 and cpp_reference["results"] == ref(args.cpp_calendar)
            and cpp_reference["reference_kind"] == "linked-original-inputprocessor-and-genuine-state",
            "Original genuine-state C++ calendar reference unavailable/changed")
    for key in ["binary", "request", "source_contract", "native_core_build", "native_driver_build"]:
        require(ref(ROOT / cpp_reference[key]["path"]) == cpp_reference[key], f"Original CPP {key} provenance changed")
    for binding in cpp_reference["artifacts"]:
        require(ref(ROOT / binding["path"]) == binding, "Original CPP command/stdout/stderr changed")
    require(cpp_reference["source_contract"] == ref(PLAN / "contracts/CLK-01-source.json"), "Original CPP source contract differs")
    request = read(ROOT / cpp_reference["request"]["path"])
    require(request == read(PLAN / "contracts/CLK-01-cases.json")["calendar_request"], "Original CPP semantic request differs from frozen cases")
    stdout_binding = next(binding for binding in cpp_reference["artifacts"] if binding["path"].endswith("calendar-stdout.log"))
    require(read(ROOT / stdout_binding["path"]) == read(args.cpp_calendar), "Original CPP results differ from actual stdout")
    core_build = read(ROOT / cpp_reference["native_core_build"]["path"])
    require(core_build["checks_passed"] is True and core_build["energyplus_commit"] == "6f2e40d10250a105b49966baa24d843711e61048",
            "Original complete native core build is unverified")
    originals = {case["case_id"]: case for case in read(args.cpp_calendar)["cases"]}
    tolerances = read(PLAN / "contracts/CLK-01-tolerances.json")
    require(tolerances["frozen_before_comparison"] and all(row["atol"] == row["rtol"] == 0 for row in tolerances["rows"]),
            "Frozen exact clock precision policy changed")
    cases = {}
    for binding in read(PLAN / "contracts/scope.json")["cases"]:
        path = ROOT / binding["metadata"]["path"]
        require(ref(path) == binding["metadata"], "Frozen case metadata changed")
        case = read(path)
        require(all(case[key] == binding[key] for key in ["id", "scope", "duration", "limit_variant", "input", "weather"]),
                "Case metadata identity differs")
        cases[case["id"]] = case
    require(len(args.case) == len(set(args.case)) and all(case in cases and case in originals for case in args.case), "Unknown/duplicate fixed case")
    output_root = args.output_root.resolve()
    require(output_root.is_relative_to((ROOT / ".runtime/porting/CLK-01").resolve()), "Output outside CLK raw proof root")
    source = output_root / "source" / ("check_clk01_production." + sha(Path(__file__))[:12] + ".py")
    if not source.exists():
        source.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(Path(__file__), source)
    require(sha(source) == sha(Path(__file__)), "Archived executed checker differs")
    results = []
    provenance = {"implementation_commit": build["implementation_commit"], "crates_tree": build["crates_tree"],
                  "build_receipt": ref(build_path), "original_cpp_calendar": ref(args.cpp_calendar),
                  "original_cpp_reference": ref(cpp_reference_path),
                  "contracts": [ref(PLAN / "contracts" / name) for name in
                                ["CLK-01-source.json", "CLK-01-cases.json", "CLK-01-tolerances.json"]]}
    for name in args.case:
        result = run_case(cases[name], originals[name], args.native_root.resolve(), binary, output_root,
                          args.actual, source, args.review_existing, provenance)
        write(PLAN / "evidence/CLK-01/production" / (name + ("-actual.json" if args.actual else "-prepared.json")), result)
        results.append({"case_id": name, "checks_passed": True,
                        "comparison": ref(output_root / name / ("actual" if args.actual else "prepared") / "comparison.json")})
    case_group = hashlib.sha256("\n".join(args.case).encode("utf-8")).hexdigest()[:12]
    write(output_root / (("actual-matrix." + case_group + ".json") if args.actual else "prepared-matrix.json"),
          {"schema": "clk01-production-matrix.v1", "cases": results, "gates_updated": False})
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
