#!/usr/bin/env python3
"""Prepare frozen CLK-01 contracts and original-source reference requests.

No gate updates or production result injection. A native executable must use the
same GNU original core and flag targets as its genuine EnergyPlusData. The pure
fallback compiles unchanged source bodies and explicitly cannot execute stateful
helpers or GetRunPeriodData. Raw reference output remains under .runtime.
"""
from __future__ import annotations

import argparse
from collections import Counter
from datetime import date, timedelta
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / ".reference/energyplus-src/26.1.0"
CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
DEFAULT_RUNTIME = ROOT / ".runtime/porting/CLK-01/preparation"
PIN = "6f2e40d10250a105b49966baa24d843711e61048"
SOURCE_PINS = {
    "src/EnergyPlus/WeatherManager.cc": "ced9d22e42c3319d64c505b97f27cdd36c618c0bc3648905c8636de7f8498386",
    "src/EnergyPlus/WeatherManager.hh": "448687bb00511cb327d4d6a4f3b6d1873e5f2a3082d4975598179b9dac82cbcc",
    "src/EnergyPlus/General.cc": "7ffbd1fe67b1d9806c169c6cd40e08a039a1c8692a93622137f07200cea02658",
    "src/EnergyPlus/General.hh": "d7dcfac7390ef66dfee198739e1886110c494dd54ef953d204bad78d710eb021",
    "src/EnergyPlus/DataGlobals.hh": "2a8e72eb70d669df625fb7933037e6f129986217129f6ef1dc8ef1014f225c25",
    "src/EnergyPlus/DataEnvironment.hh": "7a4eecf4ababb041db5b2ee8241ea82a839a05e9ab5a7c195cc6a810cd2f08b4",
    "src/EnergyPlus/Data/EnergyPlusData.cc": "6416c03de88a2b913d33393e82141b4183b9e446cd93fb7e5c7c9fde22d15f9f",
    "src/EnergyPlus/Data/EnergyPlusData.hh": "2bdbb583c802ba2b8141b3f79f3e98d8ade68774bf2fe6c571ead5462323bf21",
    "src/EnergyPlus/InputProcessing/InputProcessor.cc": "2feb4e57345a747d5467c6de0bd89b6d5078ca6e5279525b984228b0ebc91fcb",
    "src/EnergyPlus/InputProcessing/InputProcessor.hh": "e99fa1600d88921975c45fcbca9c9c95985ea04d58eb02b0bdab049e2757463d",
    "src/EnergyPlus/SimulationManager.cc": "bff6379ea2b54f91cc344b617f9b713c7ed44d9f2e0fed263ce6d30ca3cff685",
    "src/EnergyPlus/HeatBalanceManager.cc": "d810cb331f9c0dfe13d8371252ea2fe863a8043480f1f72384d824428c98d2b1",
    "src/EnergyPlus/api/datatransfer.cc": "17ba348cbbc67afbe878d982bcc9e5e04e55abeccef9113a9f3c644ea4555e1b",
    "src/EnergyPlus/api/runtime.cc": "74e45dca0d71e9cd1ac37f4cdc85da6fe7780d78a5ec6cf3d304da4e770b9ea2",
    "tst/EnergyPlus/unit/Fixtures/EnergyPlusFixture.cc": "cfb2a7173b12bfe6ac291bebaf6100a6351b0d05e4f81411a8c5a5c28b5baec9",
    "tst/EnergyPlus/unit/RunPeriod.unit.cc": "36f50ce5385ab62cbb21fd28bb571b6dded3c79ddbae1abc36756da6a553a148",
    "third_party/ObjexxFCL/src/ObjexxFCL/Fmath.hh": "36756879c587a6248ab8ccfd3ebf6d9ad34572f66ce9c874fc4973b3c070ef5c",
}
PURE_RANGES = [
    ("src/EnergyPlus/WeatherManager.cc", "isLeapYear", 8521, 8531),
    ("src/EnergyPlus/WeatherManager.cc", "computeJulianDate", 8533, 8558),
    ("src/EnergyPlus/WeatherManager.cc", "computeJulianDate GregorianDate overload", 8560, 8563),
    ("src/EnergyPlus/WeatherManager.cc", "computeGregorianDate", 8565, 8579),
    ("src/EnergyPlus/WeatherManager.cc", "calculateDayOfYear", 8616, 8634),
    ("src/EnergyPlus/WeatherManager.cc", "validMonthDay", 8636, 8675),
    ("src/EnergyPlus/General.cc", "OrdinalDay", 705, 737),
]
PAIRED_HELPERS = ["isLeapYear", "calculateDayOfYear", "General::OrdinalDay"]
WEEKDAYS = {"Sunday": 1, "Monday": 2, "Tuesday": 3, "Wednesday": 4, "Thursday": 5, "Friday": 6, "Saturday": 7}


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def encoded(value: object) -> bytes:
    return (json.dumps(value, ensure_ascii=False, indent=2, allow_nan=False) + "\n").encode("utf-8")


def read(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def write(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(encoded(value))


def ref(path: Path) -> dict:
    try:
        name = path.resolve().relative_to(ROOT).as_posix()
    except ValueError:
        name = str(path.resolve())
    return {"path": name, "sha256": sha(path)}


def verify_ref(item: dict) -> None:
    if sha(ROOT / item["path"]) != item["sha256"]:
        raise ValueError(f"Input/provenance hash mismatch: {item['path']}")


def verify_pins() -> None:
    for name, expected in SOURCE_PINS.items():
        if sha(SOURCE / name) != expected:
            raise ValueError(f"Original source pin mismatch: {name}")
    scope = read(CONTRACTS / "scope.json")
    if len(scope["cases"]) != 15:
        raise ValueError("CLK-01 must retain exactly the frozen 15 CON cases")
    for item in scope["cases"]:
        for key in ["input", "weather", "metadata"]:
            verify_ref(item[key])


def source_bytes(name: str, start: int, end: int) -> bytes:
    return b"".join((SOURCE / name).read_bytes().splitlines(keepends=True)[start - 1:end])


def selected(name: str, symbol: str, start: int, end: int, role: str = "selected") -> dict:
    return {"file": name, "symbol": symbol, "start_line": start, "end_line": end,
            "range_sha256": hashlib.sha256(source_bytes(name, start, end)).hexdigest(), "role": role}


def pure_copy() -> bytes:
    # Only namespace envelopes and the original mod using-declaration are added.
    # Every selected function body is copied byte-for-byte and restored/checkable.
    out = b"// Exact pinned original pure bodies; generated by clk01_reference.py.\n"
    for namespace, file in [("Weather", "src/EnergyPlus/WeatherManager.cc"), ("General", "src/EnergyPlus/General.cc")]:
        out += f"namespace EnergyPlus::{namespace} {{\nusing ObjexxFCL::mod;\n".encode()
        for name, _, start, end in PURE_RANGES:
            if name == file:
                out += source_bytes(name, start, end) + b"\n"
        out += b"}\n"
    return out


def source_contract() -> dict:
    wm = "src/EnergyPlus/WeatherManager.cc"
    ranges = [selected(*row, "original-pure-body") for row in PURE_RANGES]
    ranges += [selected(wm, "SetupWeekDaysByMonth", 1246, 1327),
               selected(wm, "GetRunPeriodData input allocation and fields", 5012, 5083),
               selected(wm, "GetRunPeriodData year/date/weekday selection", 5092, 5299),
               selected(wm, "GetRunPeriodData policy flags and month weekday array", 5301, 5381),
               selected(wm, "SetupEnvironmentTypes selected regular RunPeriod", 8438, 8519),
               selected(wm, "calculateDayOfWeek actual DayOfWeek write", 8581, 8614),
               selected(wm, "SetDayOfWeekInitialValues", 3228, 3250),
               selected(wm, "GetNextEnvironment calendar and environment selection", 775, 884, "consumed-calendar-context"),
               selected(wm, "GetNextEnvironment weekday/DST/reset policy", 1083, 1128, "consumed-calendar-context"),
               selected(wm, "SetCurrentWeather schedule ordinal/current zone time", 2069, 2092, "actual-consumer-context"),
               selected(wm, "UpdateWeatherData daily fields", 2009, 2034, "CLK-03-producer-context-only"),
               selected(wm, "InitializeWeather warmup/last-day/prefetch/reset", 1788, 1930, "CLK-03-context-only"),
               selected(wm, "DatesShouldBeReset timing", 1954, 1958, "CLK-03-context-only"),
               selected("src/EnergyPlus/WeatherManager.hh", "EnvironmentData", 176, 223),
               selected("src/EnergyPlus/WeatherManager.hh", "RunPeriodData and DayWeatherVariables", 275, 319),
               selected("src/EnergyPlus/WeatherManager.hh", "weekday/DST arrays and flags", 818, 836),
               selected("src/EnergyPlus/WeatherManager.hh", "month/leap/reset state", 861, 868),
               selected("src/EnergyPlus/DataGlobals.hh", "calendar/day/hour/zone flags", 75, 114),
               selected("src/EnergyPlus/DataEnvironment.hh", "weather/schedule ordinal and weekday fields", 103, 121),
               selected("src/EnergyPlus/SimulationManager.cc", "environment reset", 413, 419, "caller-context"),
               selected("src/EnergyPlus/SimulationManager.cc", "day counter advance", 444, 459, "caller-context"),
               selected("src/EnergyPlus/SimulationManager.cc", "flags and Weather before HeatBalance", 515, 546, "callback-phase-proof"),
               selected("src/EnergyPlus/HeatBalanceManager.cc", "before/after InitHeatBalance callback", 189, 200, "callback-phase-proof"),
               selected("src/EnergyPlus/api/runtime.cc", "before-init callback registration", 187, 208, "opaque-public-api"),
               selected("src/EnergyPlus/api/datatransfer.cc", "year/calendar/time/state public projections", 860, 998, "opaque-public-api"),
               selected("src/EnergyPlus/InputProcessing/InputProcessor.cc", "original processInput parser/schema/maps/buffers", 259, 322),
               selected("tst/EnergyPlus/unit/Fixtures/EnergyPlusFixture.cc", "original process_idf pattern", 360, 416, "setup-provenance-only"),
               selected("tst/EnergyPlus/unit/RunPeriod.unit.cc", "original GetRunPeriodData unit call", 197, 201, "setup-provenance-only"),
               selected("third_party/ObjexxFCL/src/ObjexxFCL/Fmath.hh", "integer mod", 692, 715)]
    return {"schema": "clk01-source-contract.v1", "card": "CLK-01", "energyplus_commit": PIN,
            "named_card_functions": ["GetRunPeriodData", "SetupWeekDaysByMonth", "calculateDayOfYear", "isLeapYear"],
            "source_files": [{"path": name, "sha256": value} for name, value in SOURCE_PINS.items()],
            "selected_ranges": ranges,
            "scope": {"calendar_year": 2013, "weather_run_periods": 1, "actual_weather": False,
                      "sizing_environment_execution": False, "actual_dst_steps": 0, "actual_special_days": 0,
                      "dst_holiday_input_policy": "Yes is retained; no active overrides after pinned EPW resolution",
                      "unit_gregorian_leap_inputs_do_not_admit_production_leap_years": True},
            "reference": {"native": "Unchanged original linked bodies with compiler-owned genuine EnergyPlusData",
                          "input": "Original InputProcessor::processInput on pinned IDFs, then GetRunPeriodData",
                          "environment_setup": "Parser-derived definition counts and declared EPW leap policy are explicit before-state inputs to original SetupEnvironmentTypes; no design/sizing payloads are fabricated",
                          "flags": "Same GNU compiler and energypluslib/project_options/project_fp_options/project_warnings targets; Debug ABI flags must match",
                          "pure_fallback": "Only listed exact pure bodies, original WeatherManager.hh GregorianDate/type declarations and original ObjexxFCL mod",
                          "pure_copy_sha256": hashlib.sha256(pure_copy()).hexdigest(),
                          "private_state_from_installed_msvc_dll": False,
                          "pure_fallback_stateful_calls": "explicit unsupported; never a stateful-reference PASS"},
            "state_contract": {"GetRunPeriodData": "Read original input buffers; write RunPeriodInput, dataEnvrn.DayOfWeek, month weekday array; record genuine before/after state",
                               "SetupWeekDaysByMonth": "Read EndDayOfMonth/LeapYearAdd and 12-element WeekDays initial content; write original array in source order, including retained entries",
                               "calculateDayOfWeek": "Writes dataEnvrn.DayOfWeek, returns matching Sched::DayType",
                               "CalendarFrame": "Current consumed date/weekday, Gregorian/weather/schedule ordinal, day type, hour/zone interval and report-column date projection; not future weather buffer or post-report year mutation"},
            "native_phase": {"selected": "callback_begin_zone_timestep_before_init_heat_balance",
                             "source_order": "SimulationManager Weather::ManageWeather -> ManageHeatBalance -> HeatBalanceManager before-init callback -> InitHeatBalance -> reporting later",
                             "raw_api_clock_at_both_phases": "Retain public API year/calendar/current_time/minutes separately; even the pre-report phase can carry shortened SYS-adjusted currentTime, so neither raw phase clock is a canonical zone end"},
            "production_proof": {"prepared_calendars": "All 15 fixed cases including all 365 annual days; preparation is not consumed invocation proof",
                                 "actual_zone_consumers_required": "A/B24H and72H; A annual when feasible; B annual physics deferred to SYS obligations",
                                 "native_reference": "Separate installed pinned original DLL public exports only; root-owned all15 phase trace harness",
                                 "daily_projection": "Original native GetRunPeriodData/SetupEnvironmentTypes plus original date/weekday/ordinal helpers; not CLK-03 producer execution"},
            "paired_direct_helpers": PAIRED_HELPERS,
            "exclusions": ["CLK-02 full EPW parsing/missing data", "CLK-03 record selection, Today/Tomorrow, prefetch, DatesShouldBeReset and repeat/warmup producer",
                           "active DST/holiday/IDF overrides rejected by CON", "OutputProcessor CalendarYear mutation and SYS04 reporting sequence",
                           "design/sizing/multiyear/actual-weather production branches", "schedule scalar values and SCH-03 lookup timing", "whole building/HVAC numerical equivalence"],
            "comparison_status": "not_run", "gates_updated": False}


def calendar_cases() -> list[dict]:
    out = []
    for item in read(CONTRACTS / "scope.json")["cases"]:
        metadata = read(ROOT / item["metadata"]["path"])
        values = metadata["expected_settings"]["run_periods"]
        if len(values) != 1:
            raise ValueError("Frozen case must have one RunPeriod")
        rp = values[0]
        begin = date.fromisoformat(rp["begin_date"])
        end = date.fromisoformat(rp["end_date"])
        if begin.year != 2013 or end.year != 2013:
            raise ValueError("Frozen production case year must remain 2013")
        out.append({"case_id": item["id"], "scope": item["scope"], "duration": item["duration"],
                    "run_period": {"name": rp["name"], "begin_month": begin.month, "begin_day_of_month": begin.day,
                                   "begin_year": begin.year, "end_month": end.month, "end_day_of_month": end.day,
                                   "end_year": end.year, "start_weekday": WEEKDAYS[rp["start_weekday"]],
                                   "first_hour_interpolation_starting_values": "Hour24", "use_weather_holidays": rp["use_weather_holidays"],
                                   "use_weather_dst": rp["use_weather_dst"], "apply_weekend_holiday_rule": rp["apply_weekend_holiday_rule"],
                                   "treat_weather_as_actual": False},
                    "weather_calendar": {"leap_year_observed": False}, "zone_timesteps_per_hour": 4,
                    "source_metadata": {key: item[key] for key in ["input", "weather", "metadata"]}})
    return out


def helper_dates() -> list[list[int]]:
    # date arithmetic creates input tuples only; no Python-derived output is used.
    first = date(2013, 1, 1)
    inputs = [[d.year, d.month, d.day] for d in (first + timedelta(days=n) for n in range(365))]
    inputs += [[1583, 1, 1], [1600, 2, 28], [1600, 2, 29], [1600, 3, 1],
               [1900, 2, 28], [1900, 3, 1], [2000, 2, 28], [2000, 2, 29], [2000, 3, 1],
               [2012, 2, 28], [2012, 2, 29], [2012, 3, 1], [2100, 2, 28], [2100, 3, 1],
               [2400, 2, 28], [2400, 2, 29], [2400, 3, 1], [2012, 12, 31], [2014, 1, 1]]
    return inputs


def setup_cases() -> list[dict]:
    end_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    return [{"start_month": month, "start_day": day, "start_weekday": weekday,
             "end_day_of_month": end_days, "leap_year_add": leap, "week_days_before": initial}
            for month, day, weekday, leap, initial in [(1, 1, 3, 0, [0] * 12), (6, 30, 1, 0, [0] * 12),
                                                     (12, 31, 3, 0, [0] * 12), (3, 1, 6, 0, [0] * 12),
                                                     (3, 1, 5, 1, [0] * 12), (6, 30, 7, 1, [0] * 12),
                                                     (6, 30, 1, 0, [7] * 12), (6, 30, 1, 0, [7, 0] + [7] * 10)]]


def helper_request() -> dict:
    calls = []
    def add(function: str, inputs: dict, group: str) -> int:
        index = len(calls)
        calls.append({"call_index": index, "function": function, "inputs": inputs,
                      "context": {"phase": "unit", "group": group}})
        return index
    for year, month, day in helper_dates():
        group = f"date-{year:04d}-{month:02d}-{day:02d}"
        add("isLeapYear", {"year": year}, group)
        # Both declared leap-shape inputs are valid arithmetic-helper domains;
        # no Python leap answer is supplied to the original reference.
        for leap in [False, True]:
            if month == 2 and day == 29 and not leap:
                continue
            add("calculateDayOfYear", {"month": month, "day": day, "leap_year": leap}, group)
            add("General::OrdinalDay", {"month": month, "day": day, "leap_year_add": int(leap)}, group)
        jd = add("computeJulianDate", {"year": year, "month": month, "day": day}, group)
        add("computeGregorianDate", {"julian_date": {"from_call": jd}}, group)
        add("calculateDayOfWeek", {"year": year, "month": month, "day": day, "day_of_week_before": 0}, group)
    for inputs in setup_cases():
        add("SetupWeekDaysByMonth", inputs, "monthly-state")
    for month, day, leap in [(1, 1, 0), (2, 28, 0), (2, 29, 0), (2, 29, 1), (4, 31, 0), (13, 1, 0), (1, 0, 0), (1, -1, 0)]:
        add("validMonthDay", {"month": month, "day": day, "leap_year_add": leap}, "source-only-validation-boundary")
    return {"schema": "clk01-helper-tuples.v1", "calls": calls}


def cases_contract() -> dict:
    helpers = helper_request()
    counts = Counter(call["function"] for call in helpers["calls"])
    return {"schema": "clk01-cases.v1", "card": "CLK-01", "scope": ["A", "B"],
            "calendar_request": {"schema": "clk01-calendar-cases.v1", "cases": calendar_cases()},
            "helper_recipe": {"schema": "clk01-helper-tuples.v1", "dates": helper_dates(),
                              "date_routine_order": ["isLeapYear", "calculateDayOfYear(false/true)", "General::OrdinalDay(0/1)",
                                                     "computeJulianDate", "computeGregorianDate(from own Julian result)", "calculateDayOfWeek"],
                              "february_29_nonleap_shape_calls": "not generated; paired arithmetic inputs remain valid",
                              "setup_weekday_inputs": setup_cases(), "call_count": len(helpers["calls"]),
                              "routine_counts": dict(counts), "expanded_request_sha256": hashlib.sha256(encoded(helpers)).hexdigest()},
            "pairing": {"direct_helpers": PAIRED_HELPERS, "unsupported_rust_helpers": "Source-only diagnostics, never counted as paired PASS",
                        "calendar": "Genuine original parsed-state and consumed daily/month-start projections; actual Rust public calendar/axis APIs",
                        "clock": "Separate representative real production invocations, native pre-report phase; prepared annual rows do not imply annual physics execution"},
            "inputs_only": True, "expected_numerical_outputs": False, "production_input_matrix_expanded": False,
            "comparison_status": "not_run", "gates_updated": False}


def tolerance_contract() -> dict:
    return {"schema": "clk01-tolerances.v1", "frozen_before_comparison": True,
            "rows": [{"quantity": name, "unit": unit, "atol": 0, "rtol": 0, "comparison": "exact"}
                     for name, unit in [("civil_year/month/day", "calendar integer"), ("Gregorian/weather/schedule ordinal", "day index"),
                                        ("weekday/day_type/DST/holiday", "enum or integer"), ("Julian date", "integer day"),
                                        ("leap flags", "boolean"), ("monthly weekday array", "12 integer elements"),
                                        ("selected source environment ordinal", "integer identity"), ("hour/zone step/interval endpoints", "hour/minute/second"),
                                        ("canonical zone interval duration", "binary64 second"), ("source state before/after", "typed exact fields")]],
            "policy": {"identity_inputs_order_counts_context": "exact", "undefined_source_input_domains": "excluded",
                       "raw_api_clock_pre_and_post_report": "recorded separately at both phases; not compared as a canonical zone timestamp",
                       "post_report_calendar_year_mutation": "outside CLK-01 gate",
                       "environment_id": "Source full ordinal and Rust materialized ordinal have different provenance; require explicit mapping based on raw definitions, never arbitrary numeric equality",
                       "physical_output_tolerance": "No building/HVAC physical outputs compared by CLK-01",
                       "unpaired_diagnostics": "Exact original result recorded; no paired PASS"}}


def frozen_contracts(check: bool) -> list[dict]:
    values = {"CLK-01-source.json": source_contract(), "CLK-01-cases.json": cases_contract(), "CLK-01-tolerances.json": tolerance_contract()}
    for name, value in values.items():
        path = CONTRACTS / name
        content = encoded(value)
        if check:
            if not path.exists() or path.read_bytes() != content:
                raise ValueError(f"Frozen contract mismatch: {name}; review --prepare before comparison")
        else:
            path.write_bytes(content)
    return [ref(CONTRACTS / name) for name in values]


def execute(command: list[str], directory: Path, stem: str, env: dict | None = None) -> dict:
    if any((directory / f"{stem}-{suffix}").exists() for suffix in ["command.json", "stdout.log", "stderr.log", "execution.json"]):
        raise ValueError("Preserve previous execution logs; choose a fresh output directory")
    done = subprocess.run(command, cwd=ROOT, env=env, capture_output=True, check=False)
    write(directory / f"{stem}-command.json", command)
    (directory / f"{stem}-stdout.log").write_bytes(done.stdout)
    (directory / f"{stem}-stderr.log").write_bytes(done.stderr)
    receipt = {"command": command, "exit_code": done.returncode,
               "artifacts": [ref(directory / f"{stem}-{suffix}") for suffix in ["command.json", "stdout.log", "stderr.log"]]}
    write(directory / f"{stem}-execution.json", receipt)
    if done.returncode != 0:
        raise RuntimeError(f"Reference command exit {done.returncode}; see {directory / (stem + '-stderr.log')}")
    return receipt


def prepare(directory: Path, check: bool) -> dict:
    verify_pins()
    contracts = frozen_contracts(check)
    if not check:
        directory.mkdir(parents=True, exist_ok=True)
        write(directory / "calendar-cases.json", {"schema": "clk01-calendar-cases.v1", "cases": calendar_cases()})
        write(directory / "helper-tuples.json", helper_request())
        (directory / "clk01_original_pure.inc").write_bytes(pure_copy())
        write(directory / "preparation.json", {"schema": "clk01-preparation.v1", "energyplus_commit": PIN, "contracts": contracts,
                                               "tools": [ref(Path(__file__)), ref(ROOT / "tools/porting/clk01_reference.cpp"),
                                                         ref(ROOT / "tools/porting/clk01_reference.cmake")],
                                               "requests": [ref(directory / name) for name in ["calendar-cases.json", "helper-tuples.json", "clk01_original_pure.inc"]],
                                               "reference_execution": "not_run", "rust_comparison": "not_run", "gates_updated": False})
    return {"contracts": contracts, "calendar_case_count": 15, "helper_call_count": len(helper_request()["calls"]),
            "paired_direct_helper_call_count": sum(c["function"] in PAIRED_HELPERS for c in helper_request()["calls"]), "check": check}


def build_pure(directory: Path) -> Path:
    manifest = read(ROOT / "tools/porting/reference_tools.json")
    llvm = next(tool for tool in manifest["tools"] if tool["name"] == "llvm-mingw")
    compiler = ROOT / ".runtime/reference-tools" / f"llvm-mingw-{llvm['version']}" / llvm["executable"]
    binary = directory / "clk01_reference_pure.exe"
    command = [str(compiler), "-std=c++20", "-O0", "-ffp-contract=off", "-UNDEBUG", "-DCLK01_PURE_ONLY", "-I" + str(directory)]
    for include in ["src", "third_party", "third_party/ObjexxFCL/src", "third_party/fmt-8.0.1/include", "third_party/valijson/include"]:
        command.append("-I" + str(SOURCE / include))
    command += [str(ROOT / "tools/porting/clk01_reference.cpp"), "-o", str(binary)]
    env = os.environ.copy()
    env["PATH"] = str(compiler.parent) + os.pathsep + env.get("PATH", "")
    receipt = execute(command, directory, "build-pure", env)
    receipt.update({"reference_kind": "pure-body-fallback-only", "compiler": ref(compiler), "binary": ref(binary),
                    "source": ref(ROOT / "tools/porting/clk01_reference.cpp"), "original_copy": ref(directory / "clk01_original_pure.inc"),
                    "stateful_functions_available": False, "gates_updated": False})
    write(directory / "build-pure.json", receipt)
    return binary


def native_bindings(core_path: Path | None, driver_path: Path | None, binary: Path) -> dict:
    if core_path is None or driver_path is None:
        raise ValueError("Genuine-state references require --native-core-build and --native-driver-build receipts")
    core, driver = read(core_path), read(driver_path)
    for label, receipt in [("core", core), ("driver", driver)]:
        if receipt.get("checks_passed") is not True or receipt.get("energyplus_commit") != PIN:
            raise ValueError(f"Native {label} build must have verified completion and the exact source commit")
    if driver.get("core_build") != ref(core_path):
        raise ValueError("Native driver receipt must bind the exact verified core receipt")
    if driver.get("binary") != ref(binary):
        raise ValueError("Native driver receipt binary does not match the executed reference")
    required = ["energypluslib", "project_options", "project_fp_options", "project_warnings"]
    if sorted(driver.get("flag_targets", [])) != sorted(required):
        raise ValueError("Native driver must inherit all original core ABI/math/warning targets")
    for field, name in [("driver_source", "clk01_reference.cpp"), ("cmake_source", "clk01_reference.cmake")]:
        if driver.get(field) != ref(ROOT / "tools/porting" / name):
            raise ValueError(f"Native driver {field} pin mismatch")
    return {"native_core_build": ref(core_path), "native_driver_build": ref(driver_path)}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--prepare", action="store_true")
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--build-pure", action="store_true")
    mode.add_argument("--run-helpers", action="store_true")
    mode.add_argument("--run-calendar", action="store_true")
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_RUNTIME)
    parser.add_argument("--reference-exe", type=Path)
    parser.add_argument("--native-core-build", type=Path)
    parser.add_argument("--native-driver-build", type=Path)
    parser.add_argument("--runtime-bin", type=Path, action="append", default=[],
                        help="Compiler/runtime DLL directory for a native original-core executable; repeated bindings are recorded")
    args = parser.parse_args()
    directory = args.output_dir.resolve()
    if args.prepare or args.check:
        result = prepare(directory, args.check)
    else:
        prepare(directory, True)
        directory.mkdir(parents=True, exist_ok=True)
        if args.build_pure:
            # Only preparation creates the exact extraction/request artifacts.
            if not (directory / "clk01_original_pure.inc").exists():
                (directory / "clk01_original_pure.inc").write_bytes(pure_copy())
            result = {"binary": ref(build_pure(directory)), "stateful_reference": "not_available", "gates_updated": False}
        else:
            if not args.reference_exe:
                raise ValueError("Supply --reference-exe built against the verified matching original source core (or explicit pure fallback for helpers)")
            bindings = native_bindings(args.native_core_build, args.native_driver_build, args.reference_exe) if args.run_calendar else {}
            request_name = "helper-tuples.json" if args.run_helpers else "calendar-cases.json"
            request = helper_request() if args.run_helpers else {"schema": "clk01-calendar-cases.v1", "cases": calendar_cases()}
            write(directory / request_name, request)
            stem = "helpers" if args.run_helpers else "calendar"
            runtime_bins = list(args.runtime_bin)
            pure_receipt = directory / "build-pure.json"
            if pure_receipt.exists():
                build = read(pure_receipt)
                if sha(args.reference_exe) == build["binary"]["sha256"]:
                    runtime_bins.append((ROOT / build["compiler"]["path"]).parent)
            env = os.environ.copy()
            if runtime_bins:
                env["PATH"] = os.pathsep.join(str(p.resolve()) for p in runtime_bins) + os.pathsep + env.get("PATH", "")
            receipt = execute([str(args.reference_exe.resolve()), "--" + stem, str(directory / request_name)], directory, stem, env)
            output = json.loads((directory / f"{stem}-stdout.log").read_text(encoding="utf-8"))
            if args.run_helpers and output["reference_kind"] == "linked-original-source-with-genuine-state":
                bindings = native_bindings(args.native_core_build, args.native_driver_build, args.reference_exe)
            write(directory / f"{stem}-results.json", output)
            receipt.update({"binary": ref(args.reference_exe), "request": ref(directory / request_name),
                            "results": ref(directory / f"{stem}-results.json"), "reference_kind": output["reference_kind"],
                            "source_contract": ref(CONTRACTS / "CLK-01-source.json"),
                            "runtime_path_bindings": [str(p.resolve()) for p in runtime_bins],
                            "rust_comparison": "not_run", "gates_updated": False, **bindings})
            write(directory / f"{stem}-reference.json", receipt)
            result = receipt
    print(json.dumps(result, ensure_ascii=False))


if __name__ == "__main__":
    main()
