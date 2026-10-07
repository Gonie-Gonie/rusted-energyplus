#!/usr/bin/env python3
"""Compare preserved GEO-02 physical outputs, without running any engine.

Full stores the actual final geometry and ordered consumer arguments. Summary
proves output independence; it does not directly observe sixteen geometry fields.
No geometry, solar, weather-height, calendar or convection formula is evaluated.
"""
from __future__ import annotations

import argparse
from collections import Counter, defaultdict
from datetime import datetime, timezone
import json
import math
from pathlib import Path
import sys

sys.dont_write_bytecode = True
import geo02_production_provenance as provenance
from geo02_production_provenance import (
    Bindings, NEGATIVE_CASES, PHYSICAL_CASES, ROOT, compiled_projection, exact,
    from_bits, frozen_contracts, integer, named, path_of, read, recorded_run, ref,
    require, scalar, sha, value_class, verify_build, verify_families, verify_original,
)

FIELD_PROFILES = {
    "area_m2": "area_m2", "gross_area_m2": "area_m2", "net_area_shadow_m2": "area_m2",
    "azimuth_deg": "angles_deg", "tilt_deg": "angles_deg",
    "newell_area_vector_m2": "newell_area_vector_m2", "centroid_m": "centroid_m",
    "newell_normal": "normals_and_lcs", "out_norm": "normals_and_lcs",
    "lcsx": "normals_and_lcs", "lcsy": "normals_and_lcs", "lcsz": "normals_and_lcs",
    "sin_azimuth": "trig", "cos_azimuth": "trig", "sin_tilt": "trig", "cos_tilt": "trig",
}
VECTOR_FIELDS = {"newell_area_vector_m2", "centroid_m", "newell_normal", "out_norm", "lcsx", "lcsy", "lcsz"}
CONSUMERS = {
    "outdoor_air_temperature_height": ("centroid_height", "convection/geometry_operands.rs"),
    "outside_wind_speed_height": ("centroid_height", "convection/geometry_operands.rs"),
    "outside_convection_orientation": ("convection_orientation", "convection.rs"),
    "solar_incidence_orientation": ("orientation", "solar.rs"),
    "surface_heat_transfer_area": ("area", "inside_convection.rs"),
}
PHYSICAL_SCOPES = {"zone_timestep_execution", "direct_zone_purchased_air_system_call"}
FULL_ONLY = {"compiled-geometry.json", "geometry-consumers.json", "geo02-geometry.json",
             "geo02-geometry-operands.json", "clock-calls.json", "psychrometrics-calls.json", "psy02-calls.json"}


def scalar_token(value):
    require(type(value) is dict and set(value) == {"value", "value_bits", "value_class"},
            "Malformed actual scalar observation")
    observed = scalar(value["value"], value["value_bits"])
    require(value["value_class"] == value_class(observed), "Scalar class differs from emitted IEEE bits")
    return value["value_bits"]


def geometry_tokens(value):
    require(type(value) is dict and set(value) == set(FIELD_PROFILES), "Stored geometry field set differs")
    result = {}
    for field in FIELD_PROFILES:
        item = value[field]
        if field in VECTOR_FIELDS:
            require(type(item) is list and len(item) == 3, "Geometry vector cardinality differs")
            result[field] = [scalar_token(component) for component in item]
        else:
            result[field] = scalar_token(item)
    return result


class Comparison:
    def __init__(self):
        self.checks = 0
        self.failures = Counter()
        self.examples = []
        self.metrics = {}

    def mismatch(self, category, location, actual, original, weight=1):
        self.failures[category] += weight
        if len(self.examples) < 60:
            self.examples.append({"category": category, "location": location, "actual": actual, "reference": original,"observed_repeat_count":weight})

    def metadata(self, actual, original, location, weight=1):
        self.checks += weight
        if not exact(actual, original):
            self.mismatch("exact_metadata", location, actual, original,weight)

    def numeric(self, actual, original, profile, field, location, weight=1):
        require(type(weight) is int and weight > 0, "Invalid observed repeat weight")
        self.checks += weight
        a, o = from_bits(actual), from_bits(original)
        require(math.isfinite(a) and math.isfinite(o), "Nonfinite physical geometry operand")
        delta = abs(a-o)
        row = self.metrics.setdefault(field, {"unit": profile["unit"], "count": 0,
            "maximum_absolute_error": 0.0, "sum_squared_error": 0.0, "bit_equal_count": 0})
        row["count"] += weight
        row["bit_equal_count"] += weight if actual == original else 0
        row["maximum_absolute_error"] = max(row["maximum_absolute_error"], delta)
        row["sum_squared_error"] += delta*delta*weight
        if a == 0 and o == 0 and actual != original:
            self.mismatch("zero_sign", location, actual, original,weight)
        elif delta > profile["absolute_tolerance"] + profile["relative_tolerance"]*abs(o):
            self.mismatch("frozen_tolerance", location, actual, original,weight)

    def report(self):
        metrics = {field:{key:value for key,value in row.items() if key != "sum_squared_error"}
                   | {"rmse":math.sqrt(row["sum_squared_error"]/row["count"])} for field,row in self.metrics.items()}
        return {"checks":self.checks, "mismatch_count":sum(self.failures.values()),
                "mismatch_categories":dict(self.failures), "mismatch_examples":self.examples,
                "per_field_scalar_metrics":metrics}


def compare_geometry(case, output, compiled, original, profiles, cmp):
    path = output/"geo02-geometry.json"
    observed = read(path)
    require(observed["schema"] == "geo02-runtime-geometry.v1" and observed["phase"] == "runtime_final_state_geometry"
            and observed["physics_executed"] is True and observed["observer_supplies_inputs"] is False
            and integer(observed["observed_timestep_index"], "final actual timestep", 1) > 0,
            "No copied final physical geometry state")
    profile = observed["cen_precision_profile"]
    require(integer(profile["sum_precision_bits"], "cen sum precision") == 53
            and integer(profile["product_precision_bits"], "cen product precision") == 64
            and profile["third_bits"] == "3fd5555555555555"
            and profile["rounding"] == "nearest_ties_even" and profile["control_writes_added"] is False
            and profile["selection_basis"] == "actual_original_x87_precision_observation", "Cen actual observed precision differs")
    typed = named(compiled["surfaces"])
    source = named(original["final_weather"]["surfaces"])
    rust = named(observed["surfaces"])
    require(set(rust) == set(typed) == set(source) and len(rust) == 6, "Physical named geometry topology differs")
    rust_by_id = {}
    for order,row in enumerate(observed["surfaces"],1):
        key = row["name"].upper()
        require(integer(row["runtime_iteration_order"], "runtime storage order", 1) == order
                and integer(row["id"], "runtime surface ID", 1) == typed[key]["id"]
                and integer(row["zone_id"], "runtime owning zone ID", 1) == typed[key]["zone_id"],
                "Actual runtime/compiler ID ownership differs")
        values = geometry_tokens(row["geometry"])
        original_values = geometry_tokens(source[key]["geometry"])
        for field,kind in FIELD_PROFILES.items():
            a,o = values[field],original_values[field]
            for index,(aa,oo) in enumerate(zip(a,o,strict=True) if field in VECTOR_FIELDS else [(a,o)]):
                cmp.numeric(aa,oo,profiles[kind],field,f"{case['id']}/{key}/{field}/{index}")
        require(set(row["legacy_geometry"]) == {"area_m2","azimuth_deg","tilt_deg"}, "Legacy stored geometry fields differ")
        for field, value in row["legacy_geometry"].items():
            cmp.metadata(scalar_token(value), values[field], f"{case['id']}/{key}/legacy/{field}")
        rust_by_id[row["id"]] = row
    # The old consumer projection is newly produced by this same ordinary run.
    # It is not a reused GEO-01 physical execution.
    old = read(output/"geometry-consumers.json")
    require(old["physics_executed"] is True and old["observer_supplies_inputs"] is False, "Legacy physical projection missing")
    legacy = named(old["surfaces"])
    require(set(legacy) == set(rust), "Actual legacy/new stored named fields differ")
    for key,row in legacy.items():
        require(row["id"] == rust[key]["id"], "Actual legacy/new own IDs differ")
        for field,bit_field in (("area_m2","area_bits"),("azimuth_deg","azimuth_bits"),("tilt_deg","tilt_bits")):
            value = row["rust_consumer"]
            scalar(value[field],value[bit_field])
            cmp.metadata(value[bit_field],scalar_token(rust[key]["geometry"][field]),f"{case['id']}/{key}/legacy_projection/{field}")
    return observed,rust_by_id


def clock_points(output, expected, scope):
    clock = read(output/"clock-calls.json")
    require(clock["schema"] == "clk01-clock-trace.v1" and clock["capture_source"] == "existing-physical-zone-loop-hook"
            and clock["complete_on_collecting_thread"] is True and clock["prepared_rows_are_executed_events"] is False
            and integer(clock["total_invocation_count"], "physical clock total") == expected
            and integer(clock["recorded_invocation_count"], "physical clock recorded") == expected
            and integer(clock["omitted_invocation_count"], "physical clock omitted") == 0
            and clock["truncation_reason"] is None and len(clock["zone_invocations"]) == expected,
            "Actual physical clock capture incomplete")
    hours = expected // 4
    frames, environment = clock["prepared_hourly_frames"], clock["prepared_environment_points"]
    require(type(frames) is list and len(frames) == hours and type(environment) is list
            and len(environment) == (expected if scope == "B" else 0),
            "Actual prepared clock cardinalities differ from the frozen duration")
    for index,event in enumerate(clock["zone_invocations"]):
        hour, substep = divmod(index, 4)
        require(integer(event["sequence"], "clock event sequence", 1) == index + 1
                and integer(event["hour_index"], "actual clock hour") == hour
                and integer(event["zone_timestep"], "actual clock substep", 1) == substep + 1
                and integer(event["zone_steps_per_hour"], "actual clock steps", 1) == 4
                and integer(event["calendar_frame_index"], "actual calendar index") == hour
                and event["timestep_seconds_bits"] == "408c200000000000"
                and exact(event["environment_point_index"], index if scope == "B" else None),
                "Actual four-per-hour physical clock order or interval differs")
        require(integer(frames[hour]["hourly_sample_index"], "prepared clock hour") == hour,
                "Actual clock frame index differs")
    return clock


def event_context(context, clock):
    if context is None:
        return None,False
    require(type(context) is dict and set(context) == {"scope","zone_timestep","system_call"}, "Event context missing/unknown")
    require(context["scope"] in PHYSICAL_SCOPES | {"coupled_output_validation"}, "Unrecognized actual invocation scope")
    zone = context["zone_timestep"]
    require(type(zone) is dict, "Actual scope has no zone or validation cursor")
    sample = integer(zone["sample_index"], "actual zone sample")
    require(sample < len(clock["zone_invocations"]), "Actual event references unavailable zone invocation")
    observed = clock["zone_invocations"][sample]
    for key in ("hour_index","zone_timestep","zone_steps_per_hour","timestep_seconds_bits"):
        require(exact(zone[key],observed[key]),"Event zone operands differ from actual loop: "+key)
    frame = clock["prepared_hourly_frames"][integer(observed["calendar_frame_index"],"hourly frame index")]
    require(exact(zone["calendar"],{key:frame[key] for key in ["year","month","day_of_month","day_of_sim","hour_ending"]}),
            "Actual event calendar differs from existing clock axis")
    env_index = observed["environment_point_index"]
    if env_index is None:
        require(zone["environment"] is None, "Fabricated environment point")
    else:
        point = dict(clock["prepared_environment_points"][integer(env_index,"environment point index")])
        point["environment_index"] = point.pop("materialized_environment_index")
        require(exact(zone["environment"],point),"Actual event environment differs from materialized point")
    if context["scope"] == "direct_zone_purchased_air_system_call":
        system = context["system_call"]
        require(type(system) is dict and integer(system["schedule_sample_index"],"actual system sample") == sample
                and exact(system["timestep_seconds_bits"],zone["timestep_seconds_bits"])
                and type(system["begin_environment"]) is bool, "Actual system-call operands differ")
    else:
        require(context["system_call"] is None,"Fabricated system invocation")
    return sample,context["scope"] in PHYSICAL_SCOPES


def caller_source(call, build, bindings):
    source = "crates/ep_runtime/src/heat_balance/" + CONSUMERS[call["consumer"]][1]
    caller = call["caller"]
    require(type(caller) is dict and caller["file"].replace('\\','/') == source
            and integer(caller["column"],"actual caller column",1) > 0, "Consumer actual source caller differs")
    line = integer(caller["line"],"actual caller line",1)
    matches = [row for row in build["crates_sources"] if row["historical_path"] == source]
    require(len(matches) == 1,"Consumer compiled caller source absent")
    path = bindings.check(matches[0])
    if path not in bindings.source_lines:
        bindings.source_lines[path] = path.read_text(encoding='utf-8').splitlines()
    lines = bindings.source_lines[path]
    require(line <= len(lines) and 'record' in lines[line-1], "Caller does not identify a built observation call")


def operand_tokens(call):
    operand = call["operand"]
    kind = operand["kind"]
    require(kind == CONSUMERS[call["consumer"]][0],"Consumer operand kind differs")
    if kind == "centroid_height":
        require(set(operand) == {"kind","centroid_m","height_m"}
                and type(operand["centroid_m"]) is list and len(operand["centroid_m"]) == 3,"Height operand shape differs")
        values = operand["centroid_m"] + [operand["height_m"]]
    else:
        keys = {"orientation":["azimuth_deg","tilt_deg","azimuth_rad","tilt_rad"],
                "convection_orientation":["azimuth_deg","tilt_deg","cos_tilt"],"area":["area_m2"]}[kind]
        require(set(operand) == {"kind",*keys},"Consumer operand field set differs")
        values = [operand[key] for key in keys]
    tokens = call["operand_bits"]
    require(type(tokens) is list and len(tokens) == len(values),"Actual operand bit cardinality differs")
    for value,token in zip(values,tokens,strict=True):
        require(math.isfinite(scalar(value,token)),"Nonfinite actual physical operand")
    return tokens


def compare_operands(case, output, rust_by_id, original, profiles, build, bindings, cmp):
    trace = read(output/"geo02-geometry-operands.json")
    require(trace["schema"] == "geo02-geometry-operands.v1"
            and trace["capture_source"] == "actual-rust-pipeline-geometry-consumer-hooks"
            and trace["thread_coverage"] == "collecting-thread-only"
            and trace["observer_supplies_inputs"] is False and trace["complete_on_collecting_thread"] is True
            and integer(trace["omitted_call_count"],"omitted geometry events") == 0 and trace["truncation_reason"] is None,
            "Geometry event capture incomplete or answer-fed")
    dictionary,order = trace["dictionary"],trace["ordered_ids"]
    require(type(dictionary) is list and type(order) is list
            and integer(trace["total_call_count"],"geometry calls") == integer(trace["retained_call_count"],"retained calls") == len(order)
            and len(order) <= integer(trace["event_limit"],"event limit")
            and len(dictionary) <= integer(trace["unique_tuple_limit"],"unique limit"),"Geometry event totals differ")
    clock = clock_points(output,{"24H":96,"72H":288}[case["duration"]],case["scope"])
    first,weights,actual_counts = {},Counter(),Counter()
    physical_counts,validation_counts,unscoped_counts,occupied = Counter(),Counter(),Counter(),defaultdict(set)
    physical_surfaces = defaultdict(set)
    last_physical = -1
    for sequence,index in enumerate(order,1):
        require(type(index) is int and 0 <= index < len(dictionary),"Ordered geometry ID out of range")
        call = dictionary[index]
        require(call["consumer"] in CONSUMERS,"Unexpected geometry consumer")
        if index not in first:
            require(index == len(first) and integer(call["first_sequence"],"first actual sequence",1) == sequence,
                    "Dictionary does not follow actual first appearance")
            first[index] = sequence
        weights[index] += 1
        actual_counts[call["consumer"]] += 1
        sample,physical = event_context(call["context"],clock)
        if physical:
            require(sample >= last_physical,"Actual physical event order regressed")
            last_physical = sample
            physical_counts[call["consumer"]] += 1
            occupied[call["consumer"]].add(sample)
            physical_surfaces[call["consumer"]].add(integer(call["surface_id"],"actual surface ID",1))
        elif call["context"] is None:
            unscoped_counts[call["consumer"]] += 1
        else:
            validation_counts[call["consumer"]] += 1
    require(len(first) == len(dictionary) and exact(dict(actual_counts),trace["consumer_counts"]),"Unused dictionary rows or consumer totals differ")
    require(set(physical_counts) == set(CONSUMERS), "Selected consumers lack genuine physical coverage")
    expected_samples = set(range(len(clock["zone_invocations"])))
    for consumer in CONSUMERS:
        if consumer == "solar_incidence_orientation":
            require(bool(occupied[consumer]) and occupied[consumer] <= expected_samples,
                    "Solar actual Some-position subset is empty or outside physical steps")
        else:
            require(occupied[consumer] == expected_samples,
                    "Non-solar consumer lacks actual coverage of every physical zone interval: " + consumer)
    source = named(original["final_weather"]["surfaces"])
    seen_keys = set()
    for index,call in enumerate(dictionary):
        caller_source(call,build,bindings)
        require(call["phase"] in {"rust_runtime","rust_runtime_setup"},"Unexpected geometry observation phase")
        surface_id = integer(call["surface_id"],"geometry surface ID",1)
        require(surface_id in rust_by_id,"Unknown actual geometry surface")
        row = rust_by_id[surface_id]
        require(integer(call["zone_id"],"geometry owning zone ID",1) == row["zone_id"],"Actual owning zone differs")
        tokens = operand_tokens(call)
        key = json.dumps({k:v for k,v in call.items() if k not in {"first_sequence","operand"}},sort_keys=True,separators=(',',':'))
        require(key not in seen_keys,"Duplicate exact dictionary observation")
        seen_keys.add(key)
        stored = geometry_tokens(row["geometry"])
        native = geometry_tokens(source[row["name"].upper()]["geometry"])
        sample,physical = event_context(call["context"],clock)
        prefix = f"{case['id']}/dict{index}/{call['consumer']}/sample{sample}"
        if call["operand"]["kind"] == "centroid_height":
            cmp.metadata(tokens[:3],stored["centroid_m"],prefix+"/stored_centroid",weights[index])
            cmp.metadata(tokens[3],stored["centroid_m"][2],prefix+"/actual_height_is_centroid_z",weights[index])
            if physical:
                cmp.numeric(tokens[3],native["centroid_m"][2],profiles["centroid_m"],"consumed_height_m",prefix,weights[index])
        elif call["operand"]["kind"] in {"orientation","convection_orientation"}:
            cmp.metadata(tokens[0],stored["azimuth_deg"],prefix+"/actual_stored_azimuth",weights[index])
            cmp.metadata(tokens[1],stored["tilt_deg"],prefix+"/actual_stored_tilt",weights[index])
            if physical:
                cmp.numeric(tokens[0],native["azimuth_deg"],profiles["angles_deg"],"consumed_azimuth_deg",prefix,weights[index])
                cmp.numeric(tokens[1],native["tilt_deg"],profiles["angles_deg"],"consumed_tilt_deg",prefix,weights[index])
                if call["operand"]["kind"] == "convection_orientation":
                    cmp.numeric(tokens[2],native["cos_tilt"],profiles["trig"],"consumed_convection_cos_tilt",prefix,weights[index])
        else:
            cmp.metadata(tokens[0],stored["area_m2"],prefix+"/actual_stored_area",weights[index])
            if physical:
                cmp.numeric(tokens[0],native["area_m2"],profiles["area_m2"],"consumed_area_m2",prefix,weights[index])
    return {"actual_ordered_event_count":len(order),"dictionary_count":len(dictionary),"omitted_event_count":0,
            "physical_consumer_counts":dict(physical_counts),"postrun_validation_consumer_counts":dict(validation_counts),
            "unscoped_setup_or_diagnostic_consumer_counts":dict(unscoped_counts),
            "physical_occupied_samples":{key:sorted(value) for key,value in occupied.items()},
            "physical_surface_ids":{key:sorted(value) for key,value in physical_surfaces.items()},
            "actual_zone_invocations":len(clock["zone_invocations"]),
            "solar_radians":"actual consumed operands recorded; no solar conversion/incidence-output oracle",
            "outward_normal_dot_consumer_claimed":False}


def invariance(case, full, summary, full_data, summary_data, cmp):
    before = sum(cmp.failures.values())
    cmp.metadata(sorted(full_data),sorted(summary_data),case+"/summary_keys")
    for key in set(full_data)-{"artifacts","timing","input","config"}:
        cmp.metadata(full_data[key],summary_data[key],case+"/summary/"+key)
    cmp.metadata({k:v for k,v in full_data["config"].items() if k != "trace_level"},
                 {k:v for k,v in summary_data["config"].items() if k != "trace_level"},case+"/config")
    for filename in ("results/selected-outputs.csv","results/meters.csv"):
        cmp.metadata(sha(full/filename),sha(summary/filename),case+"/"+filename)
    cmp.metadata(read(full/"results/result-store.json"),read(summary/"results/result-store.json"),case+"/result_store")
    cmp.metadata(read(full/"porting_scope.json"),read(summary/"porting_scope.json"),case+"/bounded_admission")
    require(not any((summary/name).exists() for name in FULL_ONLY),"Summary emitted Full-only observer artifacts")
    return {"ordinary_reports_and_results_equal":sum(cmp.failures.values())==before,"Summary_observer_files_absent":True,
            "Summary_direct_sixteen_field_geometry_observation":False}


def negative_observation(case, output, summary, original):
    require(integer(original["energyplus_exit_code"],"original negative exit",1) > 0
            and integer(original["physical_zone_callback_count"],"original negative physical") == 0,
            "Original same ordinary input was not blocked")
    require(not (output/"geo02-geometry.json").exists() and not (output/"geometry-consumers.json").exists(),
            "Negative input produced final runtime geometry")
    for filename in ("selected-outputs.csv","meters.csv","result-store.json"):
        require(not (output/"results"/filename).exists(),"Negative input produced physics data")
    observations = {}
    path = output/"geo02-geometry-operands.json"
    if path.exists():
        trace = read(path)
        require(trace["schema"] == "geo02-geometry-operands.v1"
                and integer(trace["total_call_count"],"negative calls") == 0
                and integer(trace["retained_call_count"],"negative retained") == 0
                and integer(trace["omitted_call_count"],"negative omitted") == 0
                and trace["dictionary"] == [] and trace["ordered_ids"] == [] and trace["consumer_counts"] == {}
                and trace["complete_on_collecting_thread"] is True,"Negative geometry trace has real consumer events")
        observations["geometry_operand_events"] = 0
    else:
        observations["geometry_operand_events"] = None
    path = output/"clock-calls.json"
    if path.exists():
        clock = read(path)
        require(clock["schema"] == "clk01-clock-trace.v1"
                and integer(clock["total_invocation_count"],"negative clock total") == 0
                and integer(clock["recorded_invocation_count"],"negative clock retained") == 0
                and integer(clock["omitted_invocation_count"],"negative clock omitted") == 0
                and clock["zone_invocations"] == [],"Negative input invoked physical zone steps")
        observations["actual_zone_invocations"] = 0
    else:
        observations["actual_zone_invocations"] = None
    return {"case_id":case["id"],"frozen_scope":case["scope"],"configured_porting_scope":"A",
            "original_exit":original["energyplus_exit_code"],"Rust_exit":summary["exit_code"],"Rust_status":summary["status"],
            "runtime_summary_null":summary["rust_runtime"] is None,"final_runtime_geometry_absent":True,
            "physics_data_absent":True,"actual_observations":observations,
            "absence_is_not_fabricated_zero_counter":True,"error_message_parity_claimed":False}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--matrix",required=True,help="Preserved eight-command production matrix")
    parser.add_argument("--output-dir",required=True,help="Fresh ignored directory for checker bytes and report")
    args = parser.parse_args()
    directory = path_of(args.output_dir)
    require(directory.is_relative_to(ROOT/".runtime/porting/GEO-02") and not directory.exists(),"Output must be a fresh GEO-02 raw directory")
    directory.mkdir(parents=True)
    archived=[]
    for name in ("check_geo02_production.py","geo02_production_provenance.py","check_geo02_units.py","geo02_unit_provenance.py"):
        source=Path(__file__).parent/name
        target=directory/"source"/name
        target.parent.mkdir(exist_ok=True)
        target.write_bytes(source.read_bytes())
        archived.append({"historical_path":"tools/porting/"+name,"archive":ref(target)})
    bindings=Bindings()
    matrix_ref=ref(path_of(args.matrix))
    matrix=read(bindings.check(matrix_ref))
    require(matrix["schema"] == "geo02-production-matrix.v1" and matrix["complete"] is True
            and matrix["original_outputs_supplied_to_Rust"] is False and matrix["gates_updated"] is False,
            "Incomplete or answer-fed production matrix")
    refs,source,cases,tolerances=frozen_contracts(matrix,bindings)
    provenance.units.verify_sources(source)
    family,family_raw=verify_families(matrix["input_families"],cases,bindings)
    originals,finished=verify_original(matrix,refs,cases,bindings)
    build=verify_build(matrix,bindings)
    bindings.check(matrix["executed_launcher"])
    frozen={row["id"]:row for row in cases["native_cases"]+cases["native_invalid_cases"]}
    require(len(matrix["cases"])==3 and {row["case_id"] for row in matrix["cases"]}==PHYSICAL_CASES
            and len(matrix["negative_cases"])==2 and {row["case_id"] for row in matrix["negative_cases"]}==NEGATIVE_CASES,
            "Actual producer case set differs from frozen representative/negative scope")
    cmp=Comparison()
    results=[]
    negatives=[]
    directories=set()
    for item in matrix["cases"]:
        case=frozen[item["case_id"]]
        for key in ("scope","duration","kind","input","weather","metadata"):
            require(exact(item[key],case[key]),"Actual case binding differs: "+key)
        runs={}
        for field in ("Full","Summary"):
            row=dict(item[field],trace_level=field.lower(),configured_porting_scope=item["configured_porting_scope"])
            runs[field]=recorded_run(case,row,matrix,build,finished,bindings,directories)
        full,full_summary,_,_=runs["Full"]
        summary,summary_summary,_,_=runs["Summary"]
        compiled=compiled_projection(full,case,originals[case["id"]],family_raw,bindings)
        observed,stored=compare_geometry(case,full,compiled,originals[case["id"]],tolerances["profiles"],cmp)
        operands=compare_operands(case,full,stored,originals[case["id"]],tolerances["profiles"],build,bindings,cmp)
        results.append({"case_id":case["id"],"Full_geometry":ref(full/"geo02-geometry.json"),
            "actual_operands":ref(full/"geo02-geometry-operands.json"),"stored_surface_count":len(stored),
            "stored_fields_per_surface":len(FIELD_PROFILES),"operand_proof":operands,
            "Full_Summary":invariance(case["id"],full,summary,full_summary,summary_summary,cmp)})
    for item in matrix["negative_cases"]:
        case=frozen[item["case_id"]]
        for key in ("scope","duration","kind","input","weather","metadata"):
            require(exact(item[key],case[key]),"Actual negative binding differs: "+key)
        require(case["scope"] is None and item["configured_porting_scope"]=="A","Invalid input was relabeled admitted CON")
        row=dict(item["Full"],trace_level="full",configured_porting_scope="A")
        output,summary,_,_=recorded_run(case,row,matrix,build,finished,bindings,directories,negative=True)
        if (output/"compiled-geometry.json").exists():
            compiled_projection(output,case,None,None,bindings)
        negatives.append(negative_observation(case,output,summary,originals[case["id"]]))
    report=cmp.report()
    bindings.unchanged()
    report.update(schema="geo02-production-comparison.v1",status="pass" if report["mismatch_count"]==0 else "fail",
        completed_utc=datetime.now(timezone.utc).isoformat(),producer_matrix=matrix_ref,
        executed_checker=archived[0]["archive"],executed_reader_sources=archived,
        actual_command=[sys.executable,*sys.argv],implementation_commit=build["implementation_commit"],
        contracts=refs,build=matrix["build"],native_original_matrix=matrix["native_original_matrix"],
        original_helper_execution=matrix["original_helper_execution"],original_first_reviews=matrix["original_first_reviews"],
        input_family_proof=matrix["input_families"],physical_case_count=3,actual_successful_physical_commands=6,
        ordinary_negative_commands=2,cases=results,negative_cases=negatives,
        verified_artifacts=bindings.report(),gates_updated=False,reference_outputs_supplied_to_Rust=False,
        limitations=["Full directly observes sixteen final geometry fields; Summary proves ordinary-output independence only.",
            "Fresh physical consumers are A24/A72/BBoth24; no new per-limit, B72 or annual Rust consumption claim.",
            "All seven unchanged CON geometry inputs share independently reviewed A/B families; missing outputs are not inferred from GEO01.",
            "Actual Rust geometry arguments are compared with corresponding original stored geometry; the original solar incidence consumer route/output remains unpaired.",
            "SRC04 solar-clock/incidence output and SRC05 atmospheric equations are unclaimed.",
            "Outward normal is stored; no original outward-normal dot-product consumer parity is claimed.",
            "Source-only globals/scratch/lifecycle/error messages are not fabricated Rust peers.",
            "Collector completeness applies to the collecting thread."])
    target=directory/"comparison-report.json"
    target.write_text(json.dumps(report,indent=2,allow_nan=False)+'\n',encoding='utf-8',newline='\n')
    print(json.dumps({"report":ref(target),"status":report["status"],"mismatch_count":report["mismatch_count"]}))
    return 0 if report["status"]=="pass" else 1


if __name__ == "__main__":
    raise SystemExit(main())
