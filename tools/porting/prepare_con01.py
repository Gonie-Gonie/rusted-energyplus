"""Deterministically prepare the bounded CON-01 inputs/contracts; never pass gates.

The two input templates and weather file are pinned by their original byte hashes.
Only listed object-field patches and comment/whitespace serialization are allowed.
Use --check to compare every generated artifact without changing the filesystem.
"""
from __future__ import annotations

import argparse
import copy
import datetime as dt
import hashlib
import json
import re
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PLAN = Path("energyplus_porting_plan")
EP_ROOT = Path(".runtime/energyplus/26.1.0")
EP_SOURCE = Path(".reference/energyplus-src/26.1.0/src/EnergyPlus")
EP_COMMIT = "6f2e40d10250a105b49966baa24d843711e61048"
RUST_BASELINE = "d7b516627f421259012f3e61bc28cca831452468"
SOURCES = {
    "A": (EP_ROOT / "ExampleFiles/1ZoneUncontrolled.idf", "77d099ff08791ab40ddc7c7bd7894bee3d03a994deafb4fd887c853d356e0600"),
    "B": (Path("data/conformance_cases/ideal_loads_flow_capacity_limit_diagnostic_001/ideal_loads_flow_capacity_limit_diagnostic.idf"), "46dd8b61825aeaefcd688cbdc608a58d10430dfe16ad6da5cde2b994c8ed851a"),
}
WEATHER = EP_ROOT / "WeatherData/USA_CO_Golden-NREL.724666_TMY3.epw"
WEATHER_HASH = "c184b947cd34d41c6d6474d63d66dbb82bc0e6cae4c888edcf837348282bce7f"
PERIODS = {
    "24H": (dt.date(2013, 1, 1), dt.date(2013, 1, 1)),
    "72H": (dt.date(2013, 6, 30), dt.date(2013, 7, 2)),
    "ANNUAL": (dt.date(2013, 1, 1), dt.date(2013, 12, 31)),
}
LIMITS = {"NOLIMIT": "NoLimit", "FLOW": "LimitFlowRate", "CAPACITY": "LimitCapacity", "BOTH": "LimitFlowRateAndCapacity"}
AVAILABILITY_NAME = "CON-01 System Availability"


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def json_bytes(value: object) -> bytes:
    return (json.dumps(value, ensure_ascii=False, indent=2) + "\n").encode("utf-8")


def parse_idf(data: bytes) -> list[list[str]]:
    text = data.decode("utf-8-sig")
    text = re.sub(r"!.*", "", text)
    return [[field.strip() for field in item.split(",")] for item in text.split(";") if item.strip()]


def serialize(objects: list[list[str]], case_id: str) -> bytes:
    header = f"! CON-01 pinned case {case_id}; patches and original hashes are in metadata.json.\n"
    return (header + "\n\n".join(obj[0] + ",\n  " + ",\n  ".join(obj[1:]) + ";" for obj in objects) + "\n").encode("utf-8")


def only(objects: list[list[str]], kind: str) -> list[str]:
    matches = [obj for obj in objects if obj[0].lower() == kind.lower()]
    if len(matches) != 1:
        raise ValueError(f"expected one {kind}, got {len(matches)}")
    return matches[0]


def patch_field(obj: list[str], index: int, after: str, patches: list[dict], field: str) -> None:
    before = obj[index] if index < len(obj) else None
    while len(obj) <= index:
        obj.append("")
    obj[index] = after
    if before != after:
        patches.append({"object_type": obj[0], "object_name": obj[1] if len(obj) > 1 else None,
                        "field_index": index, "field": field, "before": before, "after": after})


def explicit_object(objects: list[list[str]], kind: str, fields: list[str], patches: list[dict]) -> None:
    matches = [obj for obj in objects if obj[0].lower() == kind.lower()]
    if matches:
        for index, value in enumerate(fields, 1):
            patch_field(matches[0], index, value, patches, f"field_{index}")
    else:
        obj = [kind, *fields]
        objects.append(obj)
        patches.append({"object_type": kind, "operation": "add", "before": None, "after_fields": fields,
                        "reason": "make the selected EnergyPlus default explicit"})


def output_rows(objects: list[list[str]], scope: str) -> list[dict]:
    zone = only(objects, "Zone")[1]
    rows: list[dict] = []
    def add(key: str, name: str, units: str, tolerance: float, stage: str = "zone") -> None:
        rows.append({"key": key, "variable": name, "units": units, "frequency": "Timestep",
                     "sampling_stage": stage, "atol": tolerance, "rtol": 0.0, "rmse_max": tolerance})
    add(zone, "Zone Mean Air Temperature", "C", 1e-6)
    add(zone, "Zone Air Humidity Ratio", "kgWater/kgDryAir", 1e-9)
    for obj in objects:
        if obj[0].lower() in ("schedule:constant", "schedule:compact"):
            add(obj[1], "Schedule Value", "scalar (declared schedule type retained)", 1e-9)
    for obj in objects:
        if obj[0].lower() != "buildingsurface:detailed":
            continue
        for name in ("Surface Inside Face Temperature", "Surface Outside Face Temperature"):
            add(obj[1], name, "C", 1e-6)
        for name in ("Surface Inside Face Conduction Heat Transfer Rate", "Surface Outside Face Conduction Heat Transfer Rate", "Surface Heat Storage Rate"):
            add(obj[1], name, "W", 1e-3)
            add(obj[1], name + " per Area", "W/m2", 1e-5)
    for name in ("Site Outdoor Air Drybulb Temperature", "Site Outdoor Air Wetbulb Temperature", "Site Sky Temperature"):
        add("Environment", name, "C", 1e-5)
    add("Environment", "Site Horizontal Infrared Radiation Rate per Area", "W/m2", 1e-5)
    if scope == "B":
        for name in ("Zone Thermostat Heating Setpoint Temperature", "Zone Thermostat Cooling Setpoint Temperature"):
            add(zone, name, "C", 1e-9, "system")
        for name in ("Zone System Predicted Sensible Load to Setpoint Heat Transfer Rate", "Zone System Predicted Sensible Load to Heating Setpoint Heat Transfer Rate", "Zone System Predicted Sensible Load to Cooling Setpoint Heat Transfer Rate"):
            add(zone, name, "W", 1e-3, "system")
        for node in ("ZONE ONE INLET", "ZONE ONE AIR NODE", "ZONE ONE RETURN"):
            add(node, "System Node Temperature", "C", 1e-6, "system")
            add(node, "System Node Humidity Ratio", "kgWater/kgDryAir", 1e-9, "system")
            add(node, "System Node Mass Flow Rate", "kg/s", 1e-9, "system")
        for location in ("Zone", "Supply Air"):
            for mode in ("Heating", "Cooling"):
                for component in ("Total", "Sensible", "Latent"):
                    add("ZONE ONE IDEAL LOADS", f"Zone Ideal Loads {location} {component} {mode} Rate", "W", 1e-3, "system")
                    add("ZONE ONE IDEAL LOADS", f"Zone Ideal Loads {location} {component} {mode} Energy", "J", 1.0, "system")
        for mode in ("Heating", "Cooling"):
            add("ZONE ONE IDEAL LOADS", f"Zone Ideal Loads Supply Air Total {mode} Fuel Energy", "J", 1.0, "system")
        add("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Supply Air Mass Flow Rate", "kg/s", 1e-9, "system")
        add("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Supply Air Standard Density Volume Flow Rate", "m3/s", 1e-9, "system")
        add("ZONE ONE IDEAL LOADS", "Zone Ideal Loads Outdoor Air Mass Flow Rate", "kg/s", 1e-9, "system")
    return rows


def make_case(scope: str, period: str, variant: str | None) -> tuple[dict, bytes, list[dict]]:
    path, digest = SOURCES[scope]
    raw = (ROOT / path).read_bytes()
    if sha(raw) != digest:
        raise ValueError(f"source hash changed: {path}")
    objects = parse_idf(raw)
    patches: list[dict] = []
    case_id = f"{scope}-" + (f"{variant}-" if variant else "") + period
    begin, end = PERIODS[period]
    run = only(objects, "RunPeriod")
    run_fields = ["Run Period 1", str(begin.month), str(begin.day), "2013", str(end.month), str(end.day), "2013", begin.strftime("%A"), "Yes", "Yes", "No", "Yes", "Yes"]
    for index, value in enumerate(run_fields, 1):
        patch_field(run, index, value, patches, ["name", "begin_month", "begin_day", "begin_year", "end_month", "end_day", "end_year", "start_weekday", "weather_holidays", "weather_dst", "weekend_holiday", "weather_rain", "weather_snow"][index - 1])
    sim = only(objects, "SimulationControl")
    for index, value in enumerate(["No", "No", "No", "No", "Yes", "No", "1"], 1):
        patch_field(sim, index, value, patches, f"simulation_control_{index}")
    explicit_object(objects, "HeatBalanceAlgorithm", ["ConductionTransferFunction", "200", "0.1", "1000"], patches)
    explicit_object(objects, "SurfaceConvectionAlgorithm:Inside", ["TARP"], patches)
    explicit_object(objects, "SurfaceConvectionAlgorithm:Outside", ["DOE-2"], patches)
    explicit_object(objects, "ZoneAirHeatBalanceAlgorithm", ["ThirdOrderBackwardDifference", "No", "No"], patches)
    if scope == "B":
        ideal = only(objects, "ZoneHVAC:IdealLoadsAirSystem")
        availability_fields = [AVAILABILITY_NAME, "Fraction", "Through: 12/31", "For: AllDays", "Until: 03:00", "0", "Until: 24:00", "1"]
        objects.append(["Schedule:Compact", *availability_fields])
        patches.append({"object_type": "Schedule:Compact", "object_name": AVAILABILITY_NAME, "singleton": False, "operation": "add", "before": None, "after_fields": availability_fields,
                        "reason": "declare the existing HVAC-03 availability route: daily 00:00-03:00 Off, 03:00-24:00 On; downstream phase gate remains unverified"})
        patch_field(ideal, 2, AVAILABILITY_NAME, patches, "system_availability_schedule_name")
        for index in (10, 13):
            patch_field(ideal, index, LIMITS[variant], patches, "heating_limit" if index == 10 else "cooling_limit")
        for index in (11, 14):
            patch_field(ideal, index, "0.02" if variant in ("FLOW", "BOTH") else "", patches, "maximum_volume_flow_m3_s")
        for index in (12, 15):
            patch_field(ideal, index, "120" if variant in ("CAPACITY", "BOTH") else "", patches, "maximum_capacity_W")
    rows = output_rows(objects, scope)
    old_outputs = [obj[1:] for obj in objects if obj[0].lower() == "output:variable"]
    objects = [obj for obj in objects if obj[0].lower() != "output:variable"]
    objects.extend(["Output:Variable", row["key"], row["variable"], "Timestep"] for row in rows)
    patches.append({"object_type": "Output:Variable", "operation": "replace_collection", "before_fields": old_outputs,
                    "after_fields": [[r["key"], r["variable"], "Timestep"] for r in rows], "reason": "freeze the output comparison contract at zone-step reporting frequency"})
    data = serialize(objects, case_id)
    building = only(objects, "Building")
    expected = {
        "building": {"name": building[1], "north_axis_deg": float(building[2]), "effective_north_axis_deg": 0.0,
                     "terrain": building[3], "loads_convergence_tolerance": float(building[4]),
                     "temperature_convergence_tolerance": float(building[5]), "solar_distribution": building[6],
                     "maximum_warmup_days": int(building[7]), "minimum_warmup_days": int(building[8])},
        "heat_balance_algorithm": "ConductionTransferFunction", "zone_air_heat_balance_algorithm": "ThirdOrderBackwardDifference",
        "inside_convection": "TARP", "outside_convection": "DOE-2", "timesteps_per_hour": 4,
        "room_air_model_type": "Mixing", "room_air_model_provenance": "EnergyPlus default; RoomAirModelType absent", "air_temperature_coupling": "Direct",
        "heat_balance_defaults": {"maximum_surface_temperature_C": 200, "minimum_convection_coefficient_W_m2_K": 0.1, "maximum_convection_coefficient_W_m2_K": 1000},
        "simulation_control": {"do_zone_sizing_calculation": False, "do_system_sizing_calculation": False, "do_plant_sizing_calculation": False, "run_simulation_for_sizing_periods": False, "run_simulation_for_weather_file_run_periods": True, "do_hvac_sizing_simulation_for_sizing_periods": False, "maximum_number_of_hvac_sizing_simulation_passes": 1},
        "run_periods": [{"name": "Run Period 1", "begin_date": str(begin), "end_date": str(end), "start_weekday": begin.strftime("%A"),
                         "use_weather_holidays": True, "use_weather_dst": True, "apply_weekend_holiday_rule": False,
                         "use_weather_rain": True, "use_weather_snow": True}],
        "space_heat_balance": {"sizing": False, "simulation": False},
        "contaminants": False, "zone_air_mass_flow_conservation": False,
        "site_atmosphere": {"wind_speed_profile_exponent": 0.22, "wind_speed_profile_boundary_layer_thickness_m": 370.0, "air_temperature_gradient_k_per_m": 0.0065},
        "loads_convergence_tolerance_units": "dimensionless normalized load difference (source IDF comment incorrectly labels W)",
        "ideal_loads": [] if scope == "A" else [{"name": "ZONE ONE IDEAL LOADS", "heating_limit": LIMITS[variant], "cooling_limit": LIMITS[variant],
                        "maximum_heating_volume_flow_m3_s": 0.02 if variant in ("FLOW", "BOTH") else None,
                        "maximum_cooling_volume_flow_m3_s": 0.02 if variant in ("FLOW", "BOTH") else None,
                        "maximum_sensible_heating_capacity_W": 120.0 if variant in ("CAPACITY", "BOTH") else None,
                        "maximum_total_cooling_capacity_W": 120.0 if variant in ("CAPACITY", "BOTH") else None,
                        "dehumidification_control": "None", "humidification_control": "None", "outdoor_air": False,
                        "economizer": "NoEconomizer", "heat_recovery": "None", "availability_schedule_name": AVAILABILITY_NAME,
                        "maximum_heating_supply_temperature_C": 50.0, "minimum_cooling_supply_temperature_C": 13.0}],
    }
    days = (end - begin).days + 1
    metadata = {"schema_version": "con01-case.v1", "id": case_id, "scope": scope, "duration": period, "limit_variant": variant,
                "source": {"path": path.as_posix(), "sha256": digest}, "weather": {"path": WEATHER.as_posix(), "sha256": WEATHER_HASH},
                "input": {"path": (PLAN / "cases" / case_id / "input.idf").as_posix(), "sha256": sha(data)},
                "serialization": "UTF-8/LF; comments removed; object/field order retained except documented added defaults and replaced Output:Variable collection",
                "patches": patches, "object_counts": dict(sorted(Counter(o[0].lower() for o in objects).items())),
                "expected_zone_steps_excluding_warmup": days * 96, "expected_weather_environments": 1,
                "expected_settings": expected,
                "active_branch_declarations": [] if scope == "A" else [{"branch": "AvailabilityActive", "active": True, "schedule_name": AVAILABILITY_NAME,
                    "off_interval": "daily 00:00-03:00 (first 12 zone-step report endpoints)", "on_interval": "daily 03:00-24:00 (remaining 84 endpoints)",
                    "producer_card": "HVAC-03", "internal_phase_gate": "SYS-06", "internal_branch_verified_by_preparation": False}],
                "execution_status": "recorded separately in evidence; preparation does not imply execution", "gate_status": "unreviewed"}
    return metadata, data, rows


def boundaries() -> dict:
    # These source ranges fix the CON input/settings skeleton, not numerical helper certification.
    ranges = [
        ("HeatBalanceManager.cc", "GetHeatBalanceInput", 243, 327, "input-order skeleton", "active"),
        ("HeatBalanceManager.cc", "GetProjectControlData", 494, 1250, "selected project controls/defaults", "active"),
        ("HeatBalanceManager.cc", "GetSiteAtmosphereData", 1252, 1317, "absent atmosphere object default branch", "default"),
        ("HeatBalanceManager.cc", "GetConstructData", 1319, 1798, "three A/one B opaque constructions; advanced branches absent", "active/delegated CTF"),
        ("HeatBalanceManager.cc", "GetBuildingData", 1800, 1820, "one-zone geometry dispatch", "active/delegated GEO"),
        ("HeatBalanceManager.cc", "GetFrameAndDividerData", 3439, 3598, "no frame/divider objects", "inactive"),
        ("HeatBalanceManager.cc", "GetIncidentSolarMultiplier", 2084, 2177, "no multiplier objects", "inactive"),
        ("HeatBalanceManager.cc", "GetScheduledSurfaceGains", 4984, 5183, "no scheduled surface-gain objects", "inactive"),
        ("HeatBalanceManager.cc", "CreateTCConstructions", 5256, 5329, "no thermochromic material", "inactive"),
        ("HeatBalanceManager.cc", "CheckUsedConstructions", 329, 403, "used construction membership", "active"),
        ("Material.cc", "GetWindowGlassSpectralData", 2946, 3101, "no spectral glass", "inactive"),
        ("Material.cc", "GetMaterialData", 101, 2836, "Material/Material:NoMass branches only", "active/delegated CTF"),
        ("PhaseChangeModeling/HysteresisModel.cc", "GetHysteresisData", 298, 401, "no phase-change hysteresis", "inactive"),
        ("DataSurfaces.cc", "GetVariableAbsorptanceSurfaceList", 749, 798, "no variable-absorptance objects", "inactive"),
        ("HeatBalanceIntRadExchange.cc", "InitSolarViewFactors", 811, 1103, "opaque radiative setup", "active/delegated RAD"),
        ("InternalHeatGains.cc", "ManageInternalHeatGains", 195, 229, "InitOnly=true input/setup then return; gains delegated", "active/delegated SRC"),
        ("WindowManager.cc", "initWindowModel", 8512, 8518, "unconditional factory initialization; zero-window numerical model inactive", "default"),
        ("WindowModel.cc", "CWindowModel constructor/WindowModelFactory", 69, 100, "absent WindowsCalculationEngine => BuiltIn", "default"),
        ("WindowModel.cc", "CWindowOpticalModel constructor/WindowOpticalModelFactory", 121, 137, "absent complex fenestration => Simplified", "default"),
        ("DataRoomAirModel.hh", "AirModel/TempCoupleScheme defaults", 170, 176, "Mixing/Direct defaults with no RoomAirModelType object", "default"),
        ("DataHeatBalance.hh", "project-control mutable fields/defaults", 1790, 1838, "low/high convection limits, algorithm flags, warmup and zone-air controls", "default"),
        ("ConvectionConstants.hh", "HcInt enum/name mapping", 62, 190, "selected TARP enum/name mapping", "declaration"),
        ("ConvectionConstants.hh", "HcExt enum/name mapping", 235, 349, "selected DOE-2 enum/name mapping", "declaration"),
        ("InputProcessing/InputProcessor.cc", "getNumObjectsFound", 505, 541, "object-count boundary", "active"),
        ("InputProcessing/InputProcessor.cc", "getObjectItem", 932, 1120, "typed input retrieval boundary", "active"),
        ("UtilityRoutines.hh", "getEnumValue/getYesNoValue", 665, 681, "selected algorithm/NoYes conversion", "active"),
    ]
    result = []
    for file, symbol, start, end, purpose, active in ranges:
        path = EP_SOURCE / file
        data = (ROOT / path).read_bytes()
        lines = data.splitlines(keepends=True)
        result.append({"path": path.as_posix(), "upstream_path": "src/EnergyPlus/" + file, "symbol": symbol,
                       "start_line": start, "end_line": end, "file_sha256": sha(data),
                       "range_sha256": sha(b"".join(lines[start-1:end])), "purpose": purpose, "branch": active,
                       "numerical_body_certified": False})
    return {"schema_version": "con01-source-boundaries.v1", "energyplus_commit": EP_COMMIT,
            "scope": "selected input branches and project defaults; downstream geometry/CTF/radiation/gains numerics retain their card gates",
            "get_heat_balance_input_order": ["GetProjectControlData", "GetSiteAtmosphereData", "GetWindowGlassSpectralData", "GetMaterialData", "GetHysteresisData", "GetFrameAndDividerData", "GetConstructData", "GetBuildingData", "GetVariableAbsorptanceSurfaceList", "GetIncidentSolarMultiplier", "GetScheduledSurfaceGains", "CreateTCConstructions", "CheckUsedConstructions", "InitSolarViewFactors", "ManageInternalHeatGains(InitOnly=true)"],
            "project_control_selected_lines": {"Building": [530, 722], "inside_convection": [724, 762], "outside_convection": [764, 798], "CTF": [800, 893], "zone_air": [921, 985], "contaminants": [987, 1100], "mass_flow_conservation": [1102, 1202], "default_HVAC_root_finding": [1208, 1250]},
            "get_building_data_direct_calls": ["SolarShading::GetShadowingInput (default/no shading geometry; SRC cards)", "GetZoneData (one zone; GEO cards)", "SurfaceGeometry::SetupZoneGeometry (six opaque surfaces; GEO cards)"],
            "inactive_calls": ["representative surface printing", "invalid zero-zone surface check", "Kiva setup"], "ranges": result}


def state_contracts() -> dict:
    specs = [
        ("Geometry", "GEO-01/GEO-02/GEO-03", ["CTF", "SRC", "RAD", "ZON"], "immutable after geometry initialization", ["vertices[m]", "area[m2]", "normal[1]", "azimuth/tilt[deg]", "centroid[m]", "volume[m3]"], "TypedModel geometry (existing owner; field mapping refined by GEO cards)"),
        ("CalendarFrame", "CLK-01/CLK-03", ["SCH", "weather", "report"], "environment/date transition then each zone step", ["civil_date[date]", "weather_ordinal[int]", "schedule_ordinal[int]", "day_type[enum]", "hour[int]", "timestep[int]", "environment[int]"], "calendar/weather runtime"),
        ("WeatherStep", "CLK-02/CLK-03/CLK-04/CLK-05/CLK-06/PSY", ["SRC", "ZON"], "begin accepted zone timestep, after weather update", ["dry_bulb[C]", "wet_bulb[C]", "humidity_ratio[kg/kg]", "pressure[Pa]", "wind[m/s]", "rain[bool]", "DNI/DHI/IR[W/m2]"], "weather runtime step"),
        ("ScheduleValue", "SCH-03", ["gains", "setpoints", "availability", "efficiency"], "original lookup/update stage; retain queried civil timestamp", ["schedule_id[id]", "value[scalar]", "lookup_time[date/time]", "update_stage[enum]"], "schedule evaluator"),
        ("CtfCoefficients", "CTF-01..CTF-10", ["SUR"], "immutable after construction initialization", ["X/Y/Z/flux_coefficients[array]", "NumCTFTerms[int]", "CTFTimeStep[h]", "NumHistories[int]"], "ConstructionThermalDataCache"),
        ("CtfHistory", "SUR-01/SUR-06", ["SUR-02", "SUR-03", "SUR-04"], "frozen during retries/iterations; shift only at accepted zone-step commit", ["inside/outside_temperature_current/lag/master[C]", "inside/outside_flux_current/lag/master[W/m2]"], "SurfaceCtfState"),
        ("SurfaceIterationInput", "SRC/RAD/SUR", ["SUR-04", "SUR-05"], "per inner heat-balance iteration", ["exterior_solve[C]", "frozen_history_constants[W/m2]", "previous_iteration_inside_temperature[C]", "reference_air_temperature[C]", "h[W/m2/K]", "radiant_source[W/m2]"], "SurfaceHeatBalanceState iteration snapshot"),
        ("ZoneAirState", "ZON-01/ZON-04/ZON-05/ZON-06", ["ZON-02", "ZON-03", "HVAC"], "separate previous-zone/previous-system/current-iteration/accepted/zone-average meanings", ["MAT[C]", "ZT[C]", "ZTAV[C]", "W[kg/kg]", "zone_history[array]", "system_history[array]", "dt[s]", "retry_flags[bool]"], "ZoneHeatBalanceState"),
        ("ZoneCoefficients", "ZON-02/HVAC-08", ["predictor", "corrector"], "snapshot at each predictor/corrector stage", ["SumHA[W/K]", "SumHATsurf/ref[W]", "SumIntGain[W]", "SumMCp/SumSysMCp[W/K]", "SumMCpT/SumSysMCpT[W]", "AirPowerCap[W/K]"], "zone predictor/corrector coefficient snapshot"),
        ("Demand", "ZON-03", ["HVAC-03", "HVAC-04", "HVAC-05"], "before equipment dispatch; preserve zone/group multiplier application", ["heating_threshold[W]", "cooling_threshold[W]", "active_demand[W]", "deadband[bool]", "multipliers[1]"], "zone demand snapshot"),
        ("SupplyState", "HVAC-04/HVAC-05/HVAC-06", ["HVAC-08", "report"], "commit at CalcPurchasedAir tail; node update afterwards", ["temperature[C]", "humidity_ratio[kg/kg]", "enthalpy[J/kg]", "mass_flow[kg/s]", "delivered_sensible[W]", "delivered_moisture[kg/s]"], "PurchasedAirRuntimeState"),
        ("StepReport", "SUR-07/HVAC-07", ["SYS-04"], "iteration values overwritten; sum accepted system/zone step exactly once", ["rate[W]", "energy[J]", "store_type[Average/Sum]", "accepted_dt[s]", "warmup/report_flags[bool]"], "ResultStore/report accumulators"),
        ("WarmupConvergence", "SYS-02", ["SYS-03"], "day end; all four temperature/heating/cooling convergence conditions", ["Tmax/Tmin[C]", "heating/cooling_extrema[W]", "previous_day_values[matching units]", "absolute_temperature_difference[C]", "normalized_load_difference[1]"], "warmup convergence state"),
    ]
    declarations = []
    for file, start, end, fields in [
        ("Construction.hh", 159, 186, ["CTFCross", "CTFFlux", "CTFInside", "CTFOutside", "CTFTimeStep", "NumHistories", "NumCTFTerms"]),
        ("ZoneTempPredictorCorrector.hh", 107, 121, ["MAT", "ZT", "ZTAV", "XMPT", "XMAT", "DSXMAT"]),
        ("ZoneTempPredictorCorrector.hh", 128, 137, ["airHumRat", "airHumRatAvg", "airHumRatTemp", "WPrevZoneTS", "DSWPrevZoneTS"]),
        ("ZoneTempPredictorCorrector.hh", 147, 154, ["SumIntGain", "SumHA", "SumHATsurf", "SumHATref", "SumMCp", "SumMCpT", "SumSysMCp", "SumSysMCpT"]),
        ("ZoneTempPredictorCorrector.hh", 208, 208, ["AirPowerCap"]),
        ("DataZoneEnergyDemands.hh", 66, 86, ["RemainingOutputRequired", "UnadjRemainingOutputRequired", "TotalOutputRequired", "OutputRequiredToHeatingSP", "OutputRequiredToCoolingSP"]),
        ("PurchasedAirManager.hh", 143, 146, ["MaxHeatSuppAirTemp", "MinCoolSuppAirTemp", "MaxHeatSuppAirHumRat", "MinCoolSuppAirHumRat"]),
        ("PurchasedAirManager.hh", 173, 186, ["MaxHeatMassFlowRate", "MaxCoolMassFlowRate", "SupplyAirMassFlowRate"]),
        ("PurchasedAirManager.hh", 259, 259, ["SupplyTemp"]),
        ("HeatBalanceManager.hh", 68, 86, ["PassFlag", "TestMaxTempValue", "TestMinTempValue", "TestMaxHeatLoadValue", "TestMaxCoolLoadValue"]),
        ("WeatherManager.cc", 2076, 2076, ["DayOfYear_Schedule = General::OrdinalDay(month,day,1)"]),
        ("ScheduleManager.cc", 2516, 2529, ["schedule lookup uses always-leap 366-day ordinal, independent of civil/weather ordinal"]),
    ]:
        path = EP_SOURCE / file
        raw = (ROOT / path).read_bytes()
        declarations.append({"path": path.as_posix(), "start_line": start, "end_line": end, "fields": fields,
                             "file_sha256": sha(raw), "range_sha256": sha(b"".join(raw.splitlines(keepends=True)[start-1:end]))})
    return {"schema_version": "con01-state-contracts.v1", "normative_origin": {"path": (PLAN / "STATE_CONTRACTS.md").as_posix(), "sha256": sha((ROOT / PLAN / "STATE_CONTRACTS.md").read_bytes())},
            "status": "design contract; ownership mappings do not certify numerical implementation",
            "mutation_rule": "each card may write only declared output/mutable fields; compare changed caches, flags and arrays as well as return values",
            "temperature_semantics": {"MAT": "end-of-zone-step mean air temperature", "ZT": "air temperature averaged over the system time step", "ZTAV": "air temperature averaged over the zone time step"},
            "contracts": [{"name": n, "producer_cards": p, "consumers": c, "timing": t, "fields_with_units": f, "rust_owner_or_planned_boundary": r} for n,p,c,t,f,r in specs],
            "source_declarations": declarations,
            "comparison_policy": {"ids_enums_array_lengths_timestamps": "exact", "cache_input_version_invalidation_and_calculation_order": "preserve", "MAT_ZT_ZTAV_may_be_merged": False}}


def audit_patch_record(metadata: dict, data: bytes) -> None:
    """Replay the declared patch record independently to detect silent changes."""
    objects = parse_idf((ROOT / metadata["source"]["path"]).read_bytes())
    for patch in metadata["patches"]:
        kind = patch["object_type"].lower()
        operation = patch.get("operation")
        if operation == "add":
            if any(obj[0].lower() == kind and (patch.get("singleton", True) or obj[1].lower() == patch["object_name"].lower()) for obj in objects):
                raise ValueError(f"cannot replay duplicate added object: {kind}")
            objects.append([patch["object_type"], *patch["after_fields"]])
        elif operation == "replace_collection":
            before = [obj[1:] for obj in objects if obj[0].lower() == kind]
            if before != patch["before_fields"]:
                raise ValueError(f"patch collection precondition failed: {kind}")
            objects = [obj for obj in objects if obj[0].lower() != kind]
            objects.extend([patch["object_type"], *fields] for fields in patch["after_fields"])
        else:
            matches = [obj for obj in objects if obj[0].lower() == kind]
            if len(matches) != 1:
                raise ValueError(f"patch object selection ambiguous: {kind}")
            obj, index = matches[0], patch["field_index"]
            before = obj[index] if index < len(obj) else None
            if before != patch["before"]:
                raise ValueError(f"patch field precondition failed: {kind}.{index}")
            while len(obj) <= index:
                obj.append("")
            obj[index] = patch["after"]
    if serialize(objects, metadata["id"]) != data:
        raise ValueError(f"undocumented input change in {metadata['id']}")


def unit_settings_cases() -> list[tuple[dict, bytes]]:
    result = []
    base, base_data, _ = make_case("A", "24H", None)
    for case_id, north, maximum, minimum, effective_north, effective_max, effective_min in [
        ("UNIT-NORTH-NEGATIVE-WARMUP", "-450", "25", "30", -90.0, 30, 30),
        ("UNIT-NORTH-POSITIVE-DEFAULT-WARMUP", "765", "", "", 45.0, 25, 1),
    ]:
        metadata = copy.deepcopy(base)
        metadata["id"] = case_id
        metadata["unit_settings_probe"] = True
        metadata["production_case"] = False
        metadata["derived_from_case"] = {"id": base["id"], "sha256": sha(base_data)}
        objects = parse_idf(base_data)
        building = only(objects, "Building")
        patch_field(building, 2, north, metadata["patches"], "north_axis_deg")
        patch_field(building, 7, maximum, metadata["patches"], "maximum_warmup_days")
        patch_field(building, 8, minimum, metadata["patches"], "minimum_warmup_days")
        metadata["expected_settings"]["building"].update({"north_axis_deg": float(north), "effective_north_axis_deg": effective_north,
                                                            "maximum_warmup_days": effective_max, "minimum_warmup_days": effective_min})
        metadata["source_setting_branches"] = ["Building north axis uses signed C++ mod(value,360)",
                                               "maximum warmup days raised to minimum" if minimum == "30" else "blank warmup limits use defaults max25/min1"]
        data = serialize(objects, case_id)
        metadata["input"] = {"path": (PLAN / "evidence/CON-01/unit-settings" / case_id / "input.idf").as_posix(), "sha256": sha(data)}
        audit_patch_record(metadata, data)
        result.append((metadata, data))
    return result


def artifacts() -> dict[Path, bytes]:
    if sha((ROOT / WEATHER).read_bytes()) != WEATHER_HASH:
        raise ValueError("weather hash changed")
    files: dict[Path, bytes] = {}
    cases, profiles, meter_profiles, case_profiles = [], {}, {}, {}
    for scope in ("A", "B"):
        for variant in ([None] if scope == "A" else LIMITS):
            for period in PERIODS:
                meta, data, outputs = make_case(scope, period, variant)
                audit_patch_record(meta, data)
                files[PLAN / "cases" / meta["id"] / "input.idf"] = data
                metadata_path = PLAN / "cases" / meta["id"] / "metadata.json"
                metadata_data = json_bytes(meta)
                files[metadata_path] = metadata_data
                cases.append({key: meta[key] for key in ("id", "scope", "duration", "limit_variant", "input", "weather")} |
                             {"metadata": {"path": metadata_path.as_posix(), "sha256": sha(metadata_data)}})
                if scope in profiles and outputs != profiles[scope]:
                    raise ValueError(f"case {meta['id']} no longer shares the declared {scope} output profile")
                profiles[scope] = outputs
                case_profiles[meta["id"]] = scope
                meters = [{"meter": obj[1], "units": "J", "frequency": obj[2].title(), "atol": 1.0, "rtol": 0.0, "rmse_max": 1.0,
                                             "numeric_comparison_card": "SYS-04", "reference_artifact": "eplusout.mtr (ancillary reporting; no CON-01 numerical comparison)"}
                                            for obj in parse_idf(data) if obj[0].lower() == "output:meter:meterfileonly"]
                if scope in meter_profiles and meters != meter_profiles[scope]:
                    raise ValueError(f"case {meta['id']} no longer shares the declared {scope} meter profile")
                meter_profiles[scope] = meters
    units, unit_settings_profiles = [], {}
    for metadata, data in unit_settings_cases():
        input_path = Path(metadata["input"]["path"])
        metadata_path = input_path.with_name("metadata.json")
        metadata_data = json_bytes(metadata)
        files[input_path] = data
        files[metadata_path] = metadata_data
        units.append({key: metadata[key] for key in ("id", "scope", "duration", "limit_variant", "input", "weather")} |
                     {"metadata": {"path": metadata_path.as_posix(), "sha256": sha(metadata_data)}})
        unit_settings_profiles[metadata["id"]] = "A"
    scope = {"schema_version": "con01-scope.v2", "energyplus": {"version": "26.1.0", "commit": EP_COMMIT}, "original_rust_baseline": RUST_BASELINE,
             "calendar": {"civil_year": 2013, "time_steps_per_hour": 4, "nominal_zone_step_seconds": 900, "time_zone_hours": -7, "leap_year": False,
                          "weather_only": True, "design_day_declarations_inactive": True, "effective_dst": False, "effective_weather_holidays": 0,
                          "weather_data_period_weekday": "Sunday", "weekday_policy": "RunPeriod explicit 2013 civil weekday overrides EPW header weekday",
                          "ordinal_policy": {"civil_weather_day_of_year": "2013 non-leap 365-day calendar", "schedule_day_of_year": "always-leap 366-day index; Jun30=182 while civil/weather=181 (WeatherManager.cc2076, ScheduleManager.cc2527)"},
                          "api_year_semantics": {"calendar_year": "civil simulation year (2013)", "year": "weather record year from EPW, not civil year"}},
             "scopes": {"A": {"zone_count": 1, "opaque_surfaces": 6, "hvac_count": 0, "allowed_thermal_features": ["Material", "Material:NoMass", "opaque CTF", "OtherEquipment", "TARP", "DOE-2", "MinimalShadowing", "Zone ceiling-height/volume Autocalculate from opaque surfaces (delegated GEO)"],
                               "nonthermal_features_retained": ["Exterior:Lights (exterior meter only)", "inactive SizingPeriod:DesignDay declarations", "output/report requests"]},
                        "B": {"zone_count": 1, "opaque_surfaces": 6, "ideal_loads_count": 1, "room_air": "Mixing", "connection": "direct zone equipment", "outdoor_air": False,
                              "dehumidification": "None", "humidification": "None", "limits": LIMITS, "hard_sizes": {"flow_m3_s": 0.02, "capacity_W": 120},
                              "active_branch_declarations": {"AvailabilityActive": {"schedule_name": AVAILABILITY_NAME, "daily_off_hours": 3, "daily_on_hours": 21, "downstream_cards": ["HVAC-03", "SYS-06"], "internal_phase_gate_passed": False}}}},
             "excluded_active_features": ["windows/fenestration", "shading devices", "infiltration/ventilation/AFN", "outdoor-air paths", "non-Mixing room air", "air loops", "plant", "EMS", "equipment/HVAC autosizing/autocalculate", "multiple zones/equipment", "Kiva", "phase-change/hysteresis", "spectral/thermochromic glass", "space heat-balance sizing/simulation", "contaminants", "zone mass-flow conservation"],
             "execution_policy": {"reference": "read-only EnergyPlus API observers; no actuators or state setters", "raw_results_root": ".runtime/porting/CON-01", "automatic_gate_updates": False,
                                  "prepared_is_executed": False, "downstream_numerics_certified_by_CON01": False}, "cases": cases, "unit_settings_cases": units}
    files[PLAN / "contracts/scope.json"] = json_bytes(scope)
    files[PLAN / "contracts/source-boundaries.json"] = json_bytes(boundaries())
    files[PLAN / "contracts/state-contracts.json"] = json_bytes(state_contracts())
    files[PLAN / "contracts/output-tolerances.json"] = json_bytes({"schema_version": "con01-output-tolerances.v2", "status": "fixed before comparison; no completed numerical gates implied", "units_policy": "per variable/key/frequency, never one mixed-unit tolerance", "exact": ["branch IDs", "enum values", "timestamps", "environment IDs", "array lengths", "integer counts", "NumCTFTerms", "NumHistories", "rain flags"], "profiles": profiles, "case_profiles": case_profiles, "unit_settings_profiles": unit_settings_profiles, "meter_profiles": meter_profiles})
    return files


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    files = artifacts()
    mismatches = []
    for path, data in files.items():
        target = ROOT / path
        if args.check:
            if not target.is_file() or target.read_bytes() != data:
                mismatches.append(path.as_posix())
        else:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
    print(json.dumps({"mode": "check" if args.check else "prepare", "production_case_count": 15, "unit_settings_case_count": 2, "artifact_count": len(files), "mismatches": mismatches}))
    return 1 if mismatches else 0


if __name__ == "__main__":
    raise SystemExit(main())
