"""Replay 15 bounded Rust dry-run admissions and three negative input probes.

Checks original input contracts, prepared calendars/schedules, and rejection
traces. No EnergyPlus/Rust physics execution or automatic card/gate updates.
"""
from __future__ import annotations
import datetime as dt
import json
import subprocess
import sys
from pathlib import Path
sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/porting"))
from prepare_con01 import AVAILABILITY_NAME, PLAN, audit_patch_record, json_bytes, only, parse_idf, patch_field, serialize, sha
from trace_con01 import load_case_metadata, read_json

CLI = ROOT / "target/debug/eplus-rs.exe"
EVIDENCE = ROOT / PLAN / "evidence/CON-01"
RAW = ROOT / ".runtime/porting/CON-01/admission"
REVISION = subprocess.run(["git", "rev-parse", "HEAD"], cwd=ROOT, capture_output=True, text=True, check=True).stdout.strip()


def artifact(path):
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": sha(path.read_bytes())}


def command_run(case, output):
    output.mkdir(parents=True, exist_ok=True)
    command = [str(CLI), "run", str(ROOT / case["input"]["path"]), "--weather", str(ROOT / case["weather"]["path"]),
               "--output-dir", str(output), "--porting-scope", case["scope"], "--dry-run", "--keep-intermediate", "--overwrite"]
    run = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, encoding="utf-8", errors="replace", check=False)
    (output / "command.json").write_bytes(json_bytes(command))
    (output / "stdout.log").write_text(run.stdout, encoding="utf-8")
    (output / "stderr.log").write_text(run.stderr, encoding="utf-8")
    trace_path = output / "porting_scope.json"
    return {"command": command, "exit_code": run.returncode,
            "artifacts": [artifact(p) for p in (output / "command.json", output / "stdout.log", output / "stderr.log", trace_path) if p.is_file()]}, read_json(trace_path) if trace_path.is_file() else {}


def compare_admission(case, trace, exit_code):
    errors = []
    count = 0
    def check(label, actual, expected):
        nonlocal count
        count += 1
        if actual != expected:
            errors.append(f"{label}: {actual!r} != {expected!r}")
    check("exit_code", exit_code, 0)
    check("schema", trace.get("schema"), "porting-scope.v1")
    check("scope", trace.get("scope"), case["scope"])
    check("admissible", trace.get("admissible"), True)
    check("violations", trace.get("violations"), [])
    for key in ("oracle_inputs_used", "fixture_inputs_used", "conformance_claim"):
        check("production." + key, trace.get("production", {}).get(key), False)
    expected, settings = case["expected_settings"], trace.get("settings", {})
    for key in ("heat_balance_algorithm", "zone_air_heat_balance_algorithm", "room_air_model_type", "inside_convection", "outside_convection", "timesteps_per_hour"):
        check("settings." + key, settings.get(key), expected[key])
    for key, value in expected["building"].items():
        if key != "name":
            check("building." + key, settings.get("building", {}).get(key), value)
    for key, value in expected["simulation_control"].items():
        check("simulation_control." + key, settings.get("simulation_control", {}).get(key), value)
    for key, value in expected["site_atmosphere"].items():
        check("site_atmosphere." + key, settings.get("site_atmosphere", {}).get(key), value)
    for key, value in {"surface_temperature_upper_limit": 200, "minimum_surface_convection_heat_transfer_coefficient_value": 0.1,
                       "maximum_surface_convection_heat_transfer_coefficient_value": 1000}.items():
        check("heat_balance_controls." + key, settings.get("heat_balance_controls", {}).get(key), value)
    for key in ("do_space_heat_balance_for_sizing", "do_space_heat_balance_for_simulation"):
        check("zone_air_controls." + key, settings.get("zone_air_controls", {}).get(key), False)
    periods = settings.get("run_periods", [])
    check("run_period_count", len(periods), 1)
    period = expected["run_periods"][0]
    begin, end = (dt.date.fromisoformat(period[k]) for k in ("begin_date", "end_date"))
    if len(periods) == 1:
        for key, value in {"begin_year": 2013, "end_year": 2013, "begin_month": begin.month, "begin_day_of_month": begin.day,
                           "end_month": end.month, "end_day_of_month": end.day, "day_of_week_for_start_day": begin.strftime("%A"),
                           "use_weather_file_holidays_and_special_days": True, "use_weather_file_daylight_saving_period": True,
                           "apply_weekend_holiday_rule": False, "use_weather_file_rain_indicators": True, "use_weather_file_snow_indicators": True,
                           "treat_weather_as_actual": False}.items():
            check("run_period." + key, periods[0].get(key), value)
    environment = trace.get("environment", {})
    check("calendar_year", environment.get("calendar_year"), 2013)
    check("zone_timestep_samples", environment.get("zone_timestep_samples"), case["expected_zone_steps_excluding_warmup"])
    check("hourly_samples", environment.get("hourly_samples"), case["expected_zone_steps_excluding_warmup"] // 4)
    check("resolved_start_day_of_week", environment.get("resolved_start_day_of_week"), begin.strftime("%A"))
    check("daylight_saving.active", environment.get("daylight_saving", {}).get("active"), False)
    check("active_dst_step_count", environment.get("active_dst_step_count"), 0)
    check("active_special_day_count", environment.get("active_special_day_count"), 0)
    for label, date, hour, step, minute in (("first", begin, 1, 1, 15), ("last", end, 24, 4, 60)):
        frame = environment.get(label, {})
        for key, value in {"year": 2013, "month": date.month, "day_of_month": date.day, "hour": hour, "zone_timestep": step, "end_minute": minute}.items():
            check("calendar." + label + "." + key, frame.get(key), value)
    first = environment.get("first", {})
    for key, value in {"day_of_week": begin.strftime("%A"), "day_type": begin.strftime("%A"), "day_of_year": begin.timetuple().tm_yday,
                       "schedule_day_of_year": dt.date(2012,begin.month,begin.day).timetuple().tm_yday, "dst": False}.items():
        check("calendar.first." + key, first.get(key), value)
    weather_lines = (ROOT / case["weather"]["path"]).read_text().splitlines()[8:]
    first_weather_index = (begin.timetuple().tm_yday - 1) * 24
    check("source_start_record_index", environment.get("source_start_record_index"), first_weather_index)
    check("weather_record_year", environment.get("weather_record_year"), int(weather_lines[first_weather_index].split(",")[0]))
    indices = environment.get("selected_source_record_indices", [])
    check("selected_source_record_indices", indices, list(range(first_weather_index, first_weather_index + case["expected_zone_steps_excluding_warmup"] // 4)))
    objects = parse_idf((ROOT / case["input"]["path"]).read_bytes())
    declared = {o[1].upper(): o for o in objects if o[0].lower() in ("schedule:constant", "schedule:compact")}
    schedules = environment.get("schedule_values", [])
    check("schedule_count", len(schedules), len(declared))
    schedule_checks = []
    for schedule in schedules:
        obj = declared.get(schedule["name"].upper())
        check("schedule_name." + schedule["name"], obj is not None, True)
        if obj is None:
            continue
        values = schedule.get("values", [])
        check("schedule_length." + schedule["name"], len(values), case["expected_zone_steps_excluding_warmup"])
        intervals = []
        if obj[0].lower() == "schedule:compact":
            for index, field in enumerate(obj):
                if field.lower().startswith("until:"):
                    hh, mm = map(int, field.split(":", 1)[1].strip().split(":"))
                    intervals.append((hh + mm / 60, float(obj[index + 1])))
        mismatches, max_error = 0, 0.0
        for index, actual in enumerate(values):
            time = (index % 96 + 1) / 4
            wanted = float(obj[3]) if obj[0].lower() == "schedule:constant" else next(value for stop,value in intervals if time <= stop)
            error = abs(actual - wanted)
            max_error = max(max_error, error)
            mismatches += error > 1e-9
        check("declared_schedule_mismatch_count." + schedule["name"], mismatches, 0)
        schedule_checks.append({"name": schedule["name"], "id": schedule["id"], "samples": len(values), "max_abs_error": max_error,
                                "contract_atol": 1e-9, "rtol": 0, "basis": "pinned input schedule; 24H also compared independently with actual EP API"})
    systems = settings.get("ideal_loads", [])
    check("ideal_loads_count", len(systems), 0 if case["scope"] == "A" else 1)
    if case["scope"] == "B" and len(systems) == 1:
        ideal = expected["ideal_loads"][0]
        for key, value in {"name": ideal["name"], "heating_limit": ideal["heating_limit"], "cooling_limit": ideal["cooling_limit"],
                           "maximum_heating_air_flow_rate_m3_per_s": ideal["maximum_heating_volume_flow_m3_s"],
                           "maximum_cooling_air_flow_rate_m3_per_s": ideal["maximum_cooling_volume_flow_m3_s"],
                           "maximum_sensible_heating_capacity_w": ideal["maximum_sensible_heating_capacity_W"],
                           "maximum_total_cooling_capacity_w": ideal["maximum_total_cooling_capacity_W"],
                           "maximum_heating_supply_air_temperature_c": 50.0, "minimum_cooling_supply_air_temperature_c": 13.0,
                           "heating_availability_schedule_id": None, "cooling_availability_schedule_id": None,
                           "supply_air_node": "ZONE ONE INLETS", "system_inlet_air_node": None,
                           "dehumidification_control_type": "None", "humidification_control_type": "None", "outdoor_air_specification": None}.items():
            check("ideal_loads." + key, systems[0].get(key), value)
        availability_names = [s["name"].upper() for s in schedules if s["id"] == systems[0].get("availability_schedule_id")]
        check("ideal_loads.availability_name", availability_names, [AVAILABILITY_NAME.upper()])
    return {"checks": count, "errors": errors, "schedule_checks": schedule_checks, "calendar_frames": {"first": first, "last": environment.get("last")},
            "settings_compared": list(settings), "checked_source_record_count": len(indices), "physics_executed": False}


def main():
    scope = read_json(ROOT / PLAN / "contracts/scope.json")
    cases = [load_case_metadata(b) for b in scope["cases"]]
    if scope.get("schema_version") != "con01-scope.v2" or len(cases) != 15 or len({c["id"] for c in cases}) != 15:
        raise ValueError("expected the 15 unique frozen CON-01 v2 production cases")
    admissions = []
    for case in cases:
        for binding in (case["input"],case["weather"]):
            if sha((ROOT / binding["path"]).read_bytes()) != binding["sha256"]:
                raise ValueError(f"input/weather hash mismatch: {binding['path']}")
        command, trace = command_run(case, RAW / case["id"])
        comparison = compare_admission(case, trace, command["exit_code"])
        row = {"case_id": case["id"], "scope": case["scope"], "duration": case["duration"], "limit_variant": case["limit_variant"],
               "metadata": artifact((ROOT / case["input"]["path"]).with_name("metadata.json")),
               "input": case["input"], "weather": case["weather"], "command_result": command, "comparison": comparison,
               "admission_checks_passed": not comparison["errors"], "EP_physics_executed_in_this_admission_run": False,
               "Rust_physics_executed": False, "gate_status": "unreviewed"}
        admissions.append(row)
        print(json.dumps({"case": case["id"], "admission_passed": row["admission_checks_passed"], "errors": comparison["errors"]}),flush=True)
    base = next(c for c in cases if c["id"] == "B-BOTH-24H")
    negatives = []
    for name, feature in (("NEG-LATENT", "latent"), ("NEG-OUTDOOR-AIR", "outdoor_air"), ("NEG-AUTOSIZE", "autosize")):
        objects = parse_idf((ROOT / base["input"]["path"]).read_bytes())
        patches = []
        if feature == "latent":
            patch_field(only(objects,"OtherEquipment"),9,"0.1",patches,"fraction_latent")
        elif feature == "outdoor_air":
            patch_field(only(objects,"ZoneHVAC:IdealLoadsAirSystem"),21,"CON-01 Negative OA",patches,"design_specification_outdoor_air_object_name")
            fields = ["CON-01 Negative OA","Flow/Zone","0","0","0.01","0"]
            objects.append(["DesignSpecification:OutdoorAir",*fields])
            patches.append({"object_type":"DesignSpecification:OutdoorAir","operation":"add","after_fields":fields})
        else:
            patch_field(only(objects,"ZoneHVAC:IdealLoadsAirSystem"),11,"Autosize",patches,"maximum_heating_air_flow_rate")
        data = serialize(objects,name)
        folder = EVIDENCE / "negative-admission" / name
        folder.mkdir(parents=True,exist_ok=True)
        input_path = folder / "input.idf"
        input_path.write_bytes(data)
        metadata = {"schema_version":"con01-negative-case.v1","id":name,"scope":"B","feature":feature,"source":base["input"],
                    "input":artifact(input_path),"weather":base["weather"],"patches":patches,"expected_admissible":False,"expected_exit_code":4,
                    "serialization":"UTF-8/LF; source objects retained in order, listed patches only, probe ID comment header"}
        audit_patch_record(metadata,data)
        (folder / "metadata.json").write_bytes(json_bytes(metadata))
        command,trace = command_run(metadata,RAW / name)
        errors = []
        if command["exit_code"] != 4: errors.append(f"negative input exit {command['exit_code']} != RunBlocked/unsupported exit 4")
        if trace.get("admissible") is not False: errors.append("negative input was not explicitly rejected")
        if trace.get("schema") != "porting-scope.v1" or trace.get("scope") != "B": errors.append("negative scope/schema trace missing")
        violations = trace.get("violations",[])
        needle = {"latent":"latent", "outdoor_air":"outdoor", "autosize":"autosize"}[feature]
        if not any(needle in value.lower() for value in violations): errors.append(f"missing specific {feature} violation")
        for key in ("oracle_inputs_used","fixture_inputs_used","conformance_claim"):
            if trace.get("production",{}).get(key) is not False: errors.append(f"source flag {key} was not false")
        result = {"case_id":name,"metadata":artifact(folder / "metadata.json"),"input":metadata["input"],"command_result":command,
                  "violations":violations,"errors":errors,"rejection_checks_passed":not errors,"physics_executed":False,"gate_status":"unreviewed"}
        (folder / "summary.json").write_bytes(json_bytes(result))
        negatives.append(result)
        print(json.dumps({"case":name,"rejection_passed":not errors,"exit_code":command["exit_code"],"violations":violations,"errors":errors}),flush=True)
    proof = {"schema_version":"con01-rust-admission-matrix.v1","implementation_commit":REVISION,"binary":artifact(CLI),
             "execution_harness":artifact(Path(__file__)), "execution_timestamp_utc":dt.datetime.now(dt.timezone.utc).isoformat(),
             "contracts":[artifact(ROOT / PLAN / "contracts" / name) for name in ("scope.json","source-boundaries.json","state-contracts.json","output-tolerances.json")],
             "energyplus_pin":scope["energyplus"],"production_case_count":15,"cases":admissions,"negative_cases":negatives,
             "passed":all(c["admission_checks_passed"] for c in admissions) and all(c["rejection_checks_passed"] for c in negatives),
             "limitations":["Rust dry-run admission/settings/calendar/schedule preparation only; no numerical runtime execution", "EP72H/annual physics not executed", "physics gates remain unreviewed"]}
    (EVIDENCE / "rust-admission-matrix.json").write_bytes(json_bytes(proof))
    return 0 if proof["passed"] else 1


if __name__ == "__main__": raise SystemExit(main())
