"""Run read-only EnergyPlus observers for pinned CON-01 cases.

Raw callback/output/EIO artifacts stay in .runtime. A compact, hashed reference
summary and zone-step trace are saved as evidence. This does not change gates,
compare Rust physics, or claim an unexecuted duration passed. --rust-cli also
invokes the real bounded dry-run production entrypoint and compares settings.
"""
from __future__ import annotations

import argparse
import collections
import csv
import datetime as dt
import json
import math
import os
import re
import subprocess
import sys
import traceback
from pathlib import Path

sys.dont_write_bytecode = True
from prepare_con01 import AVAILABILITY_NAME, EP_COMMIT, EP_ROOT, PLAN, ROOT, json_bytes, sha

PHASES = {
    "begin_environment": "callback_begin_new_environment",
    "warmup_complete": "callback_after_new_environment_warmup_complete",
    "zone_before_init": "callback_begin_zone_timestep_before_init_heat_balance",
    "zone_after_init": "callback_begin_zone_timestep_after_init_heat_balance",
    "before_predictor": "callback_begin_system_timestep_before_predictor",
    "after_predictor_before_hvac": "callback_after_predictor_before_hvac_managers",
    "after_predictor_after_hvac": "callback_after_predictor_after_hvac_managers",
    "system_report": "callback_end_system_timestep_after_hvac_reporting",
    "zone_report": "callback_end_zone_timestep_after_zone_reporting",
}


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def load_case_metadata(binding: dict) -> dict:
    metadata = binding["metadata"]
    path = ROOT / metadata["path"]
    if sha(path.read_bytes()) != metadata["sha256"]:
        raise ValueError(f"case metadata hash mismatch: {metadata['path']}")
    case = read_json(path)
    for key in ("id", "scope", "duration", "limit_variant", "input", "weather"):
        if case.get(key) != binding[key]:
            raise ValueError(f"case metadata identity mismatch: {metadata['path']}.{key}")
    return case


def relative(path: Path) -> str:
    return path.resolve().relative_to(ROOT).as_posix()


def hashed(path: Path) -> dict:
    return {"path": relative(path), "sha256": sha(path.read_bytes())}


def eio_settings(path: Path, expected: dict) -> tuple[dict, list[str], dict]:
    rows = list(csv.reader(path.read_text(encoding="utf-8", errors="replace").splitlines()))
    by_name: dict[str, list[list[str]]] = collections.defaultdict(list)
    for row in rows:
        if row and not row[0].lstrip().startswith("!"):
            by_name[row[0].strip()].append([value.strip() for value in row[1:]])
    errors = []
    observed = {}
    building = by_name.get("Building Information", [[]])[0]
    if len(building) >= 8:
        observed["building"] = {"name": building[0], "effective_north_axis_deg": float(building[1]), "terrain": building[2],
                                "loads_convergence_tolerance": float(building[3]), "temperature_convergence_tolerance": float(building[4]),
                                "solar_distribution": building[5], "maximum_warmup_days": int(building[6]), "minimum_warmup_days": int(building[7])}
        for key, value in observed["building"].items():
            if value != expected["building"][key]:
                errors.append(f"EIO building.{key}: {value!r} != {expected['building'][key]!r}")
    else:
        errors.append("missing EIO Building Information")
    for label, key in [("Inside Convection Algorithm", "inside_convection"), ("Outside Convection Algorithm", "outside_convection"), ("Zone Air Solution Algorithm", "zone_air_heat_balance_algorithm")]:
        values = by_name.get(label, [[]])[0]
        observed[key] = values[0] if values else None
        if observed[key] != expected[key]:
            errors.append(f"EIO {key}: {observed[key]!r} != {expected[key]!r}")
    if by_name.get("Zone Air Solution Algorithm", [[]])[0][1:] != ["No", "No"]:
        errors.append("EIO space heat balance flags are not No/No")
    version = by_name.get("Program Version", [[]])[0]
    observed["program_version"] = version
    if not any("26.1.0-" + EP_COMMIT[:10] in value for value in version):
        errors.append("EnergyPlus EIO version does not match pinned 26.1.0 source commit")
    heat = by_name.get("Surface Heat Transfer Algorithm", [[]])[0]
    if heat:
        observed["heat_balance_algorithm"] = heat[0]
        observed["heat_balance_defaults"] = {"maximum_surface_temperature_C": float(heat[1]), "minimum_convection_coefficient_W_m2_K": float(heat[2]), "maximum_convection_coefficient_W_m2_K": float(heat[3])}
        if heat[0] != "CTF - ConductionTransferFunction" or observed["heat_balance_defaults"] != expected["heat_balance_defaults"]:
            errors.append("EIO CTF/default numeric controls mismatch")
    else:
        errors.append("missing EIO Surface Heat Transfer Algorithm")
    period = expected["run_periods"][0]
    environment = by_name.get("Environment", [])
    expected_dates = [dt.date.fromisoformat(period[key]).strftime("%m/%d/%Y") for key in ("begin_date", "end_date")]
    if len(environment) != 1 or environment[0][1:5] != ["WeatherFileRunPeriod", *expected_dates, period["start_weekday"]]:
        errors.append("EIO does not contain exactly the requested weather-only civil environment")
    if by_name.get("Environment:Daylight Saving", [[]])[0][0:1] != ["No"]:
        errors.append("EIO effective daylight saving is not inactive")
    if by_name.get("Environment:Special Days"):
        errors.append("unexpected effective weather holidays/special days")
    for label in ("Zone Air Carbon Dioxide Balance Simulation", "Zone Air Generic Contaminant Balance Simulation", "Zone Air Mass Flow Balance Simulation"):
        if by_name.get(label, [[]])[0][0:1] != ["No"]:
            errors.append(f"EIO unsupported active default: {label}")
    if by_name.get("HVACSystemRootFindingAlgorithm", [[]])[0] != ["RegulaFalsi"]:
        errors.append("EIO root-finding default mismatch")
    site = by_name.get("Environment:Site Atmospheric Variation", [[]])[0]
    site_keys = ("wind_speed_profile_exponent", "wind_speed_profile_boundary_layer_thickness_m", "air_temperature_gradient_k_per_m")
    if len(site) == 3:
        observed["site_atmosphere"] = dict(zip(site_keys, map(float, site)))
        if observed["site_atmosphere"] != expected["site_atmosphere"]:
            errors.append("EIO effective site wind profile/temperature-gradient defaults mismatch")
    else:
        errors.append("missing EIO Environment:Site Atmospheric Variation")
    # EIO is not an introspection API for every control field. Unreported fields
    # are checked against pinned IDF and bounded Rust trace, never fabricated.
    observed["unreported_controls"] = ["simulation_control (weather-only verified by actual environments)", "room_air_model_type (default; no room-air declarations)"]
    selected = {k: v for k, v in by_name.items() if k in ("Program Version", "Building Information", "Inside Convection Algorithm", "Outside Convection Algorithm", "Zone Air Solution Algorithm", "Surface Heat Transfer Algorithm", "Environment", "Environment:Site Atmospheric Variation", "Environment:Daylight Saving", "Environment:WarmupDays", "Warmup Convergence Information", "Zone Air Carbon Dioxide Balance Simulation", "Zone Air Generic Contaminant Balance Simulation", "Zone Air Mass Flow Balance Simulation", "HVACSystemRootFindingAlgorithm", "Site:Location", "Zone Information", "Construction CTF", "Component Sizing Information")}
    return observed, errors, selected


def rust_settings(case: dict, output: Path, cli: Path, zone_rows: list[dict]) -> dict:
    output.mkdir(parents=True, exist_ok=True)
    command = [str(cli), "run", str(ROOT / case["input"]["path"]), "--weather", str(ROOT / case["weather"]["path"]),
               "--output-dir", str(output), "--porting-scope", case["scope"], "--dry-run", "--keep-intermediate", "--overwrite"]
    run = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, encoding="utf-8", errors="replace", check=False)
    (output / "command.json").write_bytes(json_bytes(command))
    (output / "stdout.log").write_text(run.stdout, encoding="utf-8")
    (output / "stderr.log").write_text(run.stderr, encoding="utf-8")
    path = output / "porting_scope.json"
    errors = []
    trace = read_json(path) if path.is_file() else None
    if run.returncode:
        errors.append(f"Rust bounded dry run exited {run.returncode}")
    if trace is None:
        errors.append("Rust did not emit porting_scope.json")
    else:
        settings = trace.get("settings", trace.get("project_settings", trace))
        expected = case["expected_settings"]
        def compare(label, actual, value):
            if actual != value:
                errors.append(f"Rust {label}: {actual!r} != {value!r}")
        compare("trace.schema", trace.get("schema"), "porting-scope.v1")
        compare("trace.scope", trace.get("scope"), case["scope"])
        compare("trace.admissible", trace.get("admissible"), True)
        compare("trace.violations", trace.get("violations"), [])
        for key in ("oracle_inputs_used", "fixture_inputs_used", "conformance_claim"):
            compare("trace.production." + key, trace.get("production", {}).get(key), False)
        for key in ("building", "heat_balance_algorithm", "zone_air_heat_balance_algorithm", "inside_convection", "outside_convection", "timesteps_per_hour", "room_air_model_type", "simulation_control", "site_atmosphere"):
            actual = settings.get(key)
            if isinstance(expected[key], dict):
                for field, value in expected[key].items():
                    if key == "building" and field == "name":
                        continue  # identity is pinned in IDF and checked against EIO
                    if not isinstance(actual, dict) or actual.get(field) != value:
                        errors.append(f"Rust {key}.{field}: {actual.get(field) if isinstance(actual,dict) else actual!r} != {value!r}")
            elif actual != expected[key]:
                errors.append(f"Rust {key}: {actual!r} != {expected[key]!r}")
        controls = settings.get("heat_balance_controls", {})
        for actual_key, expected_key in [("surface_temperature_upper_limit", "maximum_surface_temperature_C"),
                                         ("minimum_surface_convection_heat_transfer_coefficient_value", "minimum_convection_coefficient_W_m2_K"),
                                         ("maximum_surface_convection_heat_transfer_coefficient_value", "maximum_convection_coefficient_W_m2_K")]:
            compare("heat_balance_controls." + actual_key, controls.get(actual_key), expected["heat_balance_defaults"][expected_key])
        for key in ("do_space_heat_balance_for_sizing", "do_space_heat_balance_for_simulation"):
            compare("zone_air_controls." + key, settings.get("zone_air_controls", {}).get(key), False)
        periods = settings.get("run_periods", [])
        compare("run_period_count", len(periods), 1)
        if len(periods) == 1:
            period = expected["run_periods"][0]
            begin, end = (dt.date.fromisoformat(period[key]) for key in ("begin_date", "end_date"))
            period_fields = {"begin_year": 2013, "end_year": 2013, "begin_month": begin.month, "begin_day_of_month": begin.day,
                             "end_month": end.month, "end_day_of_month": end.day, "day_of_week_for_start_day": period["start_weekday"],
                             "use_weather_file_holidays_and_special_days": True, "use_weather_file_daylight_saving_period": True,
                             "apply_weekend_holiday_rule": False, "use_weather_file_rain_indicators": True, "use_weather_file_snow_indicators": True,
                             "treat_weather_as_actual": False}
            for key, value in period_fields.items():
                compare("run_periods." + key, periods[0].get(key), value)
        systems = settings.get("ideal_loads", [])
        compare("ideal_loads_count", len(systems), 0 if case["scope"] == "A" else 1)
        if case["scope"] == "B" and len(systems) == 1:
            ideal = expected["ideal_loads"][0]
            ideal_fields = {"name": ideal["name"], "heating_limit": ideal["heating_limit"], "cooling_limit": ideal["cooling_limit"],
                            "maximum_heating_air_flow_rate_m3_per_s": ideal["maximum_heating_volume_flow_m3_s"],
                            "maximum_cooling_air_flow_rate_m3_per_s": ideal["maximum_cooling_volume_flow_m3_s"],
                            "maximum_sensible_heating_capacity_w": ideal["maximum_sensible_heating_capacity_W"],
                            "maximum_total_cooling_capacity_w": ideal["maximum_total_cooling_capacity_W"],
                            "maximum_heating_supply_air_temperature_c": 50.0, "minimum_cooling_supply_air_temperature_c": 13.0,
                            "heating_availability_schedule_id": None, "cooling_availability_schedule_id": None,
                            "supply_air_node": "ZONE ONE INLETS", "system_inlet_air_node": None,
                            "dehumidification_control_type": "None", "humidification_control_type": "None", "outdoor_air_specification": None}
            for key, value in ideal_fields.items():
                compare("ideal_loads." + key, systems[0].get(key), value)
            available_id = systems[0].get("availability_schedule_id")
            available_names = [schedule["name"] for schedule in trace.get("environment", {}).get("schedule_values", []) if schedule.get("id") == available_id]
            compare("ideal_loads.availability_schedule_resolved_name", [name.upper() for name in available_names], [ideal["availability_schedule_name"].upper()])
        environment = trace.get("environment", {})
        compare("environment.calendar_year", environment.get("calendar_year"), 2013)
        compare("environment.zone_timestep_samples", environment.get("zone_timestep_samples"), case["expected_zone_steps_excluding_warmup"])
        compare("environment.hourly_samples", environment.get("hourly_samples"), case["expected_zone_steps_excluding_warmup"] // 4)
        compare("environment.daylight_saving.active", environment.get("daylight_saving", {}).get("active"), False)
        if zone_rows:
            for edge, row in (("first", zone_rows[0]), ("last", zone_rows[-1])):
                values = {"year": row["civil_year"], "month": row["month"], "day_of_month": row["day"],
                          "hour": row["hour"] + 1, "zone_timestep": row["zone_step_number"], "end_minute": row["zone_step_number"] * 15}
                for key, value in values.items():
                    compare(f"environment.{edge}.{key}", environment.get(edge, {}).get(key), value)
            compare("environment.weather_record_year", environment.get("weather_record_year"), zone_rows[0]["weather_year"])
        schedules = environment.get("schedule_values", [])
        observed_names = {key.split("|")[0] for key in zone_rows[0].get("values", {}) if key.endswith("|Schedule Value")} if zone_rows else set()
        compare("environment.schedule_count", len(schedules), len(observed_names))
        for schedule in schedules:
            name = schedule["name"]
            compare("environment.schedule_name_exists." + name, name.upper() in {n.upper() for n in observed_names}, True)
            reference = [next((v for key,v in row["values"].items() if key.upper() == (name + "|Schedule Value").upper()), None) for row in zone_rows]
            actual = schedule.get("values", [])
            compare("environment.schedule_length." + name, len(actual), len(reference))
            if len(actual) == len(reference):
                for index, (a,b) in enumerate(zip(actual, reference)):
                    if b is None or not math.isclose(a, b, rel_tol=0, abs_tol=1e-9):
                        errors.append(f"Rust prepared schedule {name} differs from EP at zone step {index + 1}: {a!r} != {b!r}")
                        break
    return {"command": command, "exit_code": run.returncode, "errors": errors, "trace": trace, "binary_sha256": sha(cli.read_bytes()),
            "trace_artifact": hashed(path) if path.is_file() else None,
            "physics_executed": False, "scope_guard_called_via_production_entrypoint": trace is not None and run.returncode == 0}


def run_case(case: dict, outputs: list[dict], ep_root: Path, raw_root: Path, evidence_root: Path, rust_cli: Path | None) -> dict:
    for binding in (case["input"], case["weather"]):
        if sha((ROOT / binding["path"]).read_bytes()) != binding["sha256"]:
            raise ValueError(f"hash mismatch: {binding['path']}")
    raw = raw_root / case["id"]
    raw.mkdir(parents=True, exist_ok=True)
    evidence = evidence_root / case["id"]
    evidence.mkdir(parents=True, exist_ok=True)
    sys.path.insert(0, str(ep_root))
    dll = os.add_dll_directory(str(ep_root)) if os.name == "nt" else None
    from pyenergyplus.api import EnergyPlusAPI
    api = EnergyPlusAPI()
    state = api.state_manager.new_state()
    api.verify_api_version_match(state)
    exchange = api.exchange
    counts = {"warmup": collections.Counter(), "reporting": collections.Counter()}
    environment_records, callback_errors, handle_errors = [], [], []
    handles: dict[str, int] = {}
    contract_by_id = {f"{row['key']}|{row['variable']}": row for row in outputs}
    zone_rows, system_rows = [], []
    sequence: list[str] = []
    ordered_zone_sequences = []
    log = (raw / "callbacks.jsonl").open("w", encoding="utf-8", newline="\n")
    for row in outputs:
        exchange.request_variable(state, row["variable"], row["key"])

    def context(s) -> dict:
        return {"weather_year": exchange.year(s), "civil_year": exchange.calendar_year(s), "month": exchange.month(s), "day": exchange.day_of_month(s),
                "day_of_week": exchange.day_of_week(s), "day_of_year": exchange.day_of_year(s),
                "hour": exchange.hour(s), "minutes": exchange.minutes(s), "current_time_hours": exchange.current_time(s),
                "zone_step_number": exchange.zone_time_step_number(s), "zone_dt_hours": exchange.zone_time_step(s),
                "canonical_zone_end_hours": exchange.hour(s) + exchange.zone_time_step_number(s) * exchange.zone_time_step(s),
                "system_dt_hours": exchange.system_time_step(s), "environment": exchange.current_environment_num(s),
                "kind_of_sim": exchange.kind_of_sim(s), "warmup": exchange.warmup_flag(s)}

    def values(s) -> dict:
        if not exchange.api_data_fully_ready(s):
            return {}
        if not handles:
            for key, row in contract_by_id.items():
                handle = exchange.get_variable_handle(s, row["variable"], row["key"])
                handles[key] = handle
                if handle < 0:
                    handle_errors.append(key)
            (raw / "api_exchange_points.csv").write_bytes(exchange.list_available_api_data_csv(s))
        return {key: exchange.get_variable_value(s, handle) for key, handle in handles.items() if handle >= 0}

    def make_callback(phase: str):
        def callback(s):
            try:
                frame = context(s)
                record = {"phase": phase, **frame}
                counts["warmup" if frame["warmup"] else "reporting"][phase] += 1
                if phase in ("begin_environment", "warmup_complete"):
                    environment_records.append(record.copy())
                if phase in ("zone_report", "system_report"):
                    record["values"] = values(s)
                log.write(json.dumps(record, separators=(",", ":"), allow_nan=False) + "\n")
                if not frame["warmup"] and phase not in ("begin_environment", "warmup_complete"):
                    if phase == "zone_before_init":
                        sequence.clear()
                    sequence.append(phase)
                    if phase == "zone_report":
                        zone_rows.append(record)
                        ordered_zone_sequences.append(list(sequence))
                    elif phase == "system_report":
                        system_rows.append(record)
            except Exception:
                callback_errors.append(traceback.format_exc())
        return callback

    callbacks = []  # retain Python callback owners until the API state is deleted
    for phase, name in PHASES.items():
        callback = make_callback(phase)
        callbacks.append(callback)
        getattr(api.runtime, name)(state, callback)
    messages = (raw / "api_messages.log").open("w", encoding="utf-8", newline="\n")
    def message_callback(message):
        messages.write(message.decode("utf-8", errors="replace") + "\n")
    api.runtime.callback_message(state, message_callback)
    api.runtime.set_console_output_status(state, False)
    command = ["-w", str(ROOT / case["weather"]["path"]), "-d", str(raw), str(ROOT / case["input"]["path"])]
    try:
        exit_code = api.runtime.run_energyplus(state, command)
    finally:
        log.close()
        messages.close()
        api.state_manager.delete_state(state)
        if dll is not None:
            dll.close()
    errors = []
    if exit_code:
        errors.append(f"EnergyPlus exited {exit_code}")
    errors.extend(f"unavailable output: {key}" for key in handle_errors)
    errors.extend(callback_errors)
    if len(handles) != len(contract_by_id):
        errors.append(f"resolved {len(handles)} output handles for {len(contract_by_id)} required output requests")
    count = case["expected_zone_steps_excluding_warmup"]
    if len(zone_rows) != count:
        errors.append(f"expected {count} non-warmup zone steps, observed {len(zone_rows)}")
    for phase in ("zone_before_init", "zone_after_init", "zone_report"):
        if counts["reporting"][phase] != count:
            errors.append(f"{phase} count {counts['reporting'][phase]} != {count}")
    environments = {row["environment"] for row in zone_rows}
    if len(environments) != 1 or any(row["kind_of_sim"] != 3 for row in zone_rows):
        errors.append("reporting did not remain in exactly one RunPeriodWeather environment (enum 3)")
    if sum(r["phase"] == "warmup_complete" for r in environment_records) != 1:
        errors.append("expected one completed warmup environment")
    expected_period = case["expected_settings"]["run_periods"][0]
    begin = dt.date.fromisoformat(expected_period["begin_date"])
    for index, row in enumerate(zone_rows):
        if set(row.get("values", {})) != set(contract_by_id):
            errors.append(f"incomplete required output snapshot at zone step {index + 1}")
            break
        date = begin + dt.timedelta(days=index // 96)
        if (row["civil_year"], row["month"], row["day"]) != (date.year, date.month, date.day) or row["day_of_week"] != (date.weekday() + 1) % 7 + 1:
            errors.append(f"civil calendar mismatch at zone step {index + 1}")
            break
        if row["zone_dt_hours"] != 0.25 or row["zone_step_number"] != index % 4 + 1:
            errors.append(f"zone time-step mismatch at step {index + 1}")
            break
        if row["canonical_zone_end_hours"] != (index % 96 + 1) / 4 or row["hour"] != index % 96 // 4:
            errors.append(f"accepted zone timestamp mismatch at step {index + 1}")
            break
        expected_api_clock = row["canonical_zone_end_hours"] + (row["system_dt_hours"] if row["system_dt_hours"] < row["zone_dt_hours"] else 0)
        if not math.isclose(row["current_time_hours"], expected_api_clock, rel_tol=0, abs_tol=1e-12):
            errors.append(f"EnergyPlus API currentTime formula mismatch at step {index + 1}")
            break
        if not all(math.isfinite(v) for v in row.get("values", {}).values()):
            errors.append(f"non-finite output at zone step {index + 1}")
    required_order = ["zone_before_init", "zone_after_init", "before_predictor", "after_predictor_before_hvac", "after_predictor_after_hvac", "system_report", "zone_report"]
    for index, phases in enumerate(ordered_zone_sequences):
        positions = [phases.index(phase) if phase in phases else -1 for phase in required_order]
        if -1 in positions or positions != sorted(positions):
            errors.append(f"callback stage ordering mismatch at zone step {index + 1}: {phases}")
            break
    eio_path = raw / "eplusout.eio"
    observed_eio, eio_errors, selected_eio = eio_settings(eio_path, case["expected_settings"]) if eio_path.is_file() else ({}, ["missing EIO"], {})
    errors.extend(eio_errors)
    eso_path = raw / "eplusout.eso"
    eso_stamps = []
    if eso_path.is_file():
        for line in eso_path.read_text(encoding="utf-8", errors="replace").splitlines():
            row = line.split(",")
            if row[0] == "2" and len(row) == 9 and row[2].strip().isdigit():
                eso_stamps.append(row)
    if len(eso_stamps) != count:
        errors.append(f"ESO zone-report timestamp count {len(eso_stamps)} != {count}")
    else:
        for index, row in enumerate(eso_stamps):
            date = begin + dt.timedelta(days=index // 96)
            if (int(row[2]), int(row[3]), int(row[4]), int(row[5]), float(row[6]), float(row[7]), row[8].strip()) != (date.month, date.day, 0, index % 96 // 4 + 1, (index % 4) * 15.0, (index % 4 + 1) * 15.0, date.strftime("%A")):
                errors.append(f"ESO accepted zone-report timestamp mismatch at step {index + 1}")
                break
    err_path = raw / "eplusout.err"
    err_text = err_path.read_text(encoding="utf-8", errors="replace") if err_path.is_file() else ""
    severe_lines = [line for line in err_text.splitlines() if "** Severe" in line or "**  Fatal" in line]
    errors.extend(severe_lines)
    branch = {"limit_variant": case["limit_variant"], "scope": case["scope"], "observed": {}}
    if case["scope"] == "B":
        def value(record, name):
            return record["values"].get("ZONE ONE IDEAL LOADS|" + name, 0.0)
        heating = [value(r, "Zone Ideal Loads Supply Air Sensible Heating Rate") for r in system_rows]
        cooling = [value(r, "Zone Ideal Loads Supply Air Total Cooling Rate") for r in system_rows]
        flow = [value(r, "Zone Ideal Loads Supply Air Mass Flow Rate") for r in system_rows]
        volume = [value(r, "Zone Ideal Loads Supply Air Standard Density Volume Flow Rate") for r in system_rows]
        outdoor = [value(r, "Zone Ideal Loads Outdoor Air Mass Flow Rate") for r in system_rows]
        availability_key = AVAILABILITY_NAME + "|Schedule Value"
        availability = [row["values"].get(availability_key) for row in system_rows]
        zone_availability = [row["values"].get(availability_key) for row in zone_rows]
        branch["observed"] = {"heating_system_steps": sum(v > 1e-6 for v in heating), "cooling_system_steps": sum(v > 1e-6 for v in cooling),
                              "zero_delivery_system_steps": sum(h <= 1e-6 and c <= 1e-6 for h,c in zip(heating,cooling)),
                              "max_supply_sensible_heating_W": max(heating, default=0), "max_supply_total_cooling_W": max(cooling, default=0),
                              "max_supply_mass_flow_kg_s": max(flow, default=0), "max_supply_standard_volume_flow_m3_s": max(volume, default=0),
                              "max_outdoor_air_mass_flow_kg_s": max(outdoor, default=0), "availability_off_zone_steps": sum(v == 0 for v in zone_availability),
                              "availability_on_zone_steps": sum(v == 1 for v in zone_availability), "availability_off_system_steps": sum(v == 0 for v in availability)}
        expected_availability = [0.0 if index % 96 < 12 else 1.0 for index in range(len(zone_rows))]
        if zone_availability != expected_availability:
            errors.append("B system availability EP schedule values differ from declared daily 3h Off/21h On profile")
        if any(v not in (0.0, 1.0) for v in availability):
            errors.append("B system availability was not resolved to the declared 0/1 profile at system-report callbacks")
        if any(a == 0 and (abs(h) > 1e-3 or abs(c) > 1e-3 or abs(m) > 1e-9) for a,h,c,m in zip(availability, heating,cooling,flow)):
            errors.append("B availability-Off interval delivered heat/cooling/supply mass flow")
        if not any(v > 1e-6 for v in heating) or not any(v > 1e-6 for v in cooling):
            errors.append("B 24h reference did not exercise both heating and cooling")
        if case["limit_variant"] in ("CAPACITY", "BOTH") and (max(heating, default=0) > 120.001 or max(cooling, default=0) > 120.001):
            errors.append("hard capacity limit exceeded")
        if case["limit_variant"] in ("FLOW", "BOTH") and (max(volume, default=0) > 0.020000001 or not any(abs(v - 0.02) <= 1e-9 for v in volume)):
            errors.append("hard flow limit exceeded or never reached")
        if any(abs(v) > 1e-9 for v in outdoor):
            errors.append("no-OA branch produced outdoor air mass flow")
        branch["interpretation"] = "availability0 plus zero delivery observes declared Off intervals; other zero-delivery samples cannot identify Off vs deadband. Internal HVAC-03/SYS-06 phase gates remain unverified; this is not a C++ branch-state wrapper"
    selected_variables = [key for key, row in contract_by_id.items() if row["variable"] in (
        "Zone Mean Air Temperature", "Zone Air Humidity Ratio", "Schedule Value", "Surface Inside Face Temperature", "Surface Outside Face Temperature",
        "System Node Temperature", "System Node Mass Flow Rate", "System Node Humidity Ratio", "Zone Thermostat Heating Setpoint Temperature",
        "Zone Thermostat Cooling Setpoint Temperature", "Zone System Predicted Sensible Load to Setpoint Heat Transfer Rate",
        "Zone Ideal Loads Supply Air Total Heating Rate", "Zone Ideal Loads Supply Air Total Cooling Rate", "Zone Ideal Loads Supply Air Mass Flow Rate",
        "Zone Ideal Loads Supply Air Standard Density Volume Flow Rate", "Zone Ideal Loads Outdoor Air Mass Flow Rate")]
    compact_trace = {"schema_version": "con01-zone-trace.v1", "case_id": case["id"], "callback": "end_zone_timestep_after_zone_reporting", "warmup_excluded": True,
                     "columns": selected_variables, "rows": [{"frame": {k:v for k,v in r.items() if k not in ("values", "phase")}, "values": [r["values"].get(k) for k in selected_variables]} for r in zone_rows]}
    trace_path = evidence / "zone-trace.json"
    trace_path.write_bytes(json_bytes(compact_trace))
    eio_selected_path = evidence / "eio-settings.json"
    eio_selected_path.write_bytes(json_bytes(selected_eio))
    rust = rust_settings(case, raw_root / "rust" / case["id"], rust_cli, zone_rows) if rust_cli else {"status": "not_run", "physics_executed": False}
    errors.extend(rust.get("errors", []))
    revision = subprocess.run(["git", "rev-parse", "HEAD"], cwd=ROOT, capture_output=True, text=True, check=True).stdout.strip()
    summary = {"schema_version": "con01-reference-summary.v1", "case_id": case["id"], "reference_kind": "read-only EnergyPlus API observer trace",
               "execution_timestamp_utc": dt.datetime.now(dt.timezone.utc).isoformat(), "repository_revision": revision,
               "tested_artifact_hashes": [hashed(Path(__file__)), hashed(ROOT / "tools/porting/prepare_con01.py"), hashed(ROOT / PLAN / "contracts/scope.json"), hashed(ROOT / PLAN / "contracts/source-boundaries.json"), hashed(ROOT / PLAN / "contracts/state-contracts.json"), hashed(ROOT / PLAN / "contracts/output-tolerances.json"), hashed(ep_root / "energyplusapi.dll")],
               "energyplus_commit": EP_COMMIT, "energyplus_exit_code": exit_code, "run_command": ["EnergyPlusAPI.runtime.run_energyplus", *command],
               "observer_policy": "variable requests/getters and callbacks; no actuators, HVAC override or physical-state setters",
               "input": case["input"], "weather": case["weather"], "zone_steps_excluding_warmup": len(zone_rows), "system_steps_excluding_warmup": len(system_rows),
               "callback_counts": {k: dict(v) for k,v in counts.items()}, "environments": environment_records,
               "callback_order": {"required_order": required_order, "distinct_zone_sequences": [list(seq) for seq in sorted(set(tuple(s) for s in ordered_zone_sequences))]},
               "output_handle_count": len(handles), "unavailable_output_handles": handle_errors, "eio_settings": observed_eio, "branches": branch,
               "raw_artifacts": [hashed(path) for path in (raw / "callbacks.jsonl", raw / "api_messages.log", raw / "api_exchange_points.csv", eio_path, err_path, eso_path, raw / "eplusout.mtr") if path.is_file()],
               "evidence_artifacts": [hashed(trace_path), hashed(eio_selected_path)], "rust_bounded_dry_run": rust,
               "errors": errors, "reference_checks_passed": not errors, "gate_status": "unreviewed",
               "timestamp_policy": {"canonical_zone_end": "API hour + zone_step_number * zone_dt, checked exactly against 96 ESO accepted Timestep timestamps",
                                    "raw_api_current_time": "preserved separately; at end-zone reporting with shortened system dt it equals canonical zone end + system dt (api/datatransfer.cc909-919)",
                                    "weather_year": "EPW record year via API year(); civil_year via API calendar_year()"},
               "limitations": ["CON-01 settings/environment/order evidence only; downstream numerical cards are unverified", "no Rust numerical comparison performed", "only this case duration was executed", "API values at zone-report callback are instantaneous report fields; system raw samples retain adaptive dt and must be aggregated for system-variable Timestep comparison"]}
    (evidence / "summary.json").write_bytes(json_bytes(summary))
    return summary


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--case", action="append", help="repeat to select exact case IDs; default all five 24H cases")
    parser.add_argument("--unit-settings", action="store_true", help="run the two separate north/warmup settings probes; never extend the 15 production cases")
    parser.add_argument("--ep-root", type=Path, default=ROOT / EP_ROOT)
    parser.add_argument("--raw-root", type=Path, default=ROOT / ".runtime/porting/CON-01/reference")
    parser.add_argument("--evidence-root", type=Path, default=ROOT / PLAN / "evidence/CON-01/reference")
    parser.add_argument("--rust-cli", type=Path)
    args = parser.parse_args()
    scope = read_json(ROOT / PLAN / "contracts/scope.json")
    tolerance = read_json(ROOT / PLAN / "contracts/output-tolerances.json")
    if scope.get("schema_version") != "con01-scope.v2" or tolerance.get("schema_version") != "con01-output-tolerances.v2":
        raise ValueError("expected normalized CON-01 v2 contracts")
    chosen = args.case or [case["id"] for case in scope["cases"] if case["duration"] == "24H"]
    matrix_cases = [load_case_metadata(binding) for binding in scope["cases"]]
    cases = {case["id"]: case for case in matrix_cases}
    if len(cases) != 15 or len(matrix_cases) != 15:
        raise ValueError("expected exactly 15 unique production cases")
    case_profiles = tolerance["case_profiles"]
    if args.unit_settings:
        unit_root = ROOT / PLAN / "evidence/CON-01/unit-settings"
        matrix_cases = [load_case_metadata(binding) for binding in scope["unit_settings_cases"]]
        if len(matrix_cases) != 2:
            raise ValueError("expected exactly two prepared unit settings probes")
        cases = {case["id"]: case for case in matrix_cases}
        chosen = args.case or list(cases)
        case_profiles = tolerance["unit_settings_profiles"]
        if args.evidence_root == ROOT / PLAN / "evidence/CON-01/reference":
            args.evidence_root = unit_root
        if args.raw_root == ROOT / ".runtime/porting/CON-01/reference":
            args.raw_root = ROOT / ".runtime/porting/CON-01/unit-settings"
    if len(chosen) != len(set(chosen)) or any(case not in cases for case in chosen):
        parser.error("case IDs must be known and unique")
    failed = False
    run_summaries = []
    for name in chosen:
        summary = run_case(cases[name], tolerance["profiles"][case_profiles[name]], args.ep_root.resolve(), args.raw_root.resolve(), args.evidence_root.resolve(), args.rust_cli.resolve() if args.rust_cli else None)
        print(json.dumps({"case": name, "zone_steps": summary["zone_steps_excluding_warmup"], "reference_checks_passed": summary["reference_checks_passed"], "errors": summary["errors"]}, ensure_ascii=False), flush=True)
        failed |= not summary["reference_checks_passed"]
        run_summaries.append(summary)
    matrix = []
    for case in matrix_cases:
        summary_path = args.evidence_root.resolve() / case["id"] / "summary.json"
        old = read_json(summary_path) if summary_path.is_file() else None
        matrix.append({"case_id": case["id"], "scope": case["scope"], "duration": case["duration"], "limit_variant": case["limit_variant"],
                       "reference_status": "checks_passed" if old and old["reference_checks_passed"] else "failed" if old else "not_run",
                       "rust_guard_status": "checks_passed" if old and old["rust_bounded_dry_run"].get("scope_guard_called_via_production_entrypoint") and not old["rust_bounded_dry_run"].get("errors") else "not_run",
                       "physics_comparison_status": "not_run", "gate_status": "unreviewed", "summary": hashed(summary_path) if old else None})
    matrix_root = args.evidence_root.resolve() if args.unit_settings else args.evidence_root.resolve().parent
    matrix_root.joinpath("execution-matrix.json").write_bytes(json_bytes({"schema_version": "con01-execution-matrix.v1", "production_cases": not args.unit_settings, "case_count": len(matrix_cases), "cases": matrix, "automatic_gate_updates": False}))
    return int(failed)


if __name__ == "__main__":
    raise SystemExit(main())
