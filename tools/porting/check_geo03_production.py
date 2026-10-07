#!/usr/bin/env python3
"""Compare preserved GEO-03 actual initializer and stored-volume handoffs.

Reads existing artifacts only. No engine, geometry recomputation, reference
answer injection, gate mutation or report-file write occurs. Summary compares
ordinary outputs; it does not directly observe the ten initializer fields.
"""
from __future__ import annotations

import argparse
from collections import Counter
import json
from pathlib import Path
import sys

sys.dont_write_bytecode = True
from geo03_provenance import Bindings, exact, integer, path_of, read, ref, require
from check_geo03_units import CLASS_CODES, Comparison, FIELDS, bits, compare_zone_fields, from_bits, profiles, scalar_token
from geo03_production_provenance import (
    NEGATIVE, POSITIVE, PROVENANCE_SHA, compiled_projection, named, producer, recorded_run,
)

FULL_ONLY = {"compiled-geometry.json", "geometry-consumers.json", "clock-calls.json", "psychrometrics-calls.json",
    "psy02-calls.json", "geo02-geometry.json", "geo02-geometry-operands.json", "geo03-zone-volume.json"}
PHYSICAL_SCOPES = {"zone_timestep_execution", "direct_zone_purchased_air_system_call"}
OWNER_CALLER = "crates/ep_runtime/src/heat_balance/initialization.rs"
CAPACITY_CALLER = "crates/ep_runtime/src/heat_balance/air_manager.rs"


def caller(call, available, bindings, expected):
    observed = call["caller"]
    require(type(observed) is dict and set(observed) == {"file", "line", "column"}, "Malformed actual caller")
    path = observed["file"].replace("\\", "/")
    require(path == expected and path in available, "Actual caller is outside selected source owner")
    content = bindings.check(available[path]).read_text(encoding="utf-8").splitlines()
    line = integer(observed["line"], "actual caller line", 1)
    column = integer(observed["column"], "actual caller column", 1)
    require(line <= len(content) and column <= len(content[line-1]) + 1, "Actual caller location outside archived source")
    return {"file": path, "line": line, "column": column}


def owner_observations(output, zones, available, bindings, negative=False):
    artifact = read(bindings.check(ref(output / "geo03-zone-volume.json")))
    require(artifact["schema"] == "geo03-zone-volume-owner.v1"
            and artifact["capture_source"] == "actual-Rust-heat-balance-initializer-result"
            and artifact["thread_coverage"] == "collecting-thread-only"
            and artifact["observer_supplies_inputs"] is False and artifact["observer_recalculates_geometry"] is False,
            "Initializer collector provenance differs")
    rows = artifact["observations"]
    require(type(rows) is list and len(rows) > 0
            and integer(artifact["total_call_count"], "owner total", 1) == integer(artifact["retained_call_count"], "owner retained", 1) == len(rows)
            and integer(artifact["omitted_call_count"], "owner omitted") == 0,
            "Actual initializer observation prefix is incomplete")
    require(exact(artifact["unpaired_source_state"], ["implicit Space", "ErrCount5 and global counters", "warning/error IO", "scratch allocations"]),
            "Unpaired source-owner boundary differs")
    for position, row in enumerate(rows, 1):
        require(integer(row["sequence"], "owner order", 1) == position and type(row["zone_name"]) is str,
                "Actual initializer ordering/name is malformed")
        name = row["zone_name"].upper()
        require(name in zones and integer(row["zone_id"], "actual owner ZoneId") == zones[name]["id"]
                and row["phase"] == "rust_runtime_setup" and row["context"] is None,
                "Actual initializer own ID/phase differs")
        caller(row, available, bindings, OWNER_CALLER)
        if negative:
            require(row["outcome"] == "rejected" and row["properties"] is None and type(row["rejection"]) is str
                    and row["rejection"].startswith("unsupported automatic zone topology:"), "Actual topology rejection not observed")
        else:
            require(row["outcome"] == "resolved" and row["rejection"] is None and type(row["properties"]) is dict,
                    "Actual initialized volume result was not successful")
    return artifact


def compare_initializer(case, output, artifact, zones, surfaces, source, frozen_profiles, comparison):
    native = source["first_initialized"]
    native_zones, native_surfaces = named(native["zones"], 1), named(native["surfaces"], 1)
    require(set(zones) == set(native_zones), "Actual/source Zone names differ")
    first = None
    for observation in artifact["observations"]:
        label = case["id"] + "/initializer/" + str(observation["sequence"])
        value = observation["properties"]
        require(type(value["zone"]) is dict and set(value["zone"]) == set(FIELDS), "Initializer ten-field payload differs")
        compare_zone_fields(value["zone"], native_zones[observation["zone_name"].upper()], frozen_profiles, comparison, label+"/Zone")
        require(first is None or exact(first, value), "Repeated actual initializer returns changed immutable geometry")
        first = value
        require(integer(value["volume_calculation_count"], "real own volume calculations", 1) == 1,
                "Normal initializer did not execute the selected owner once")
        origin = value["p0_m"]
        require(type(origin) is list and len(origin) == 3
                and exact([scalar_token(x) for x in origin], [scalar_token(x) for x in native["native_only_state"]["p0_m"]]),
                "Actual volume summation origin differs")
        faces = value["ordered_volume_faces"]
        source_order = native["ordered_volume_base_faces"]
        require(len(source_order) == 1 and type(faces) is list and len(faces) == len(source_order[0]["faces"]) == 6,
                "Ordered normal volume face count differs")
        seen = set()
        for position, (entry, source_entry) in enumerate(zip(faces, source_order[0]["faces"]), 1):
            face = entry["face"]
            require(integer(entry["volume_face_order"], "volume face order", 1) == position and type(face["name"]) is str,
                    "Actual volume face order malformed")
            name = face["name"].upper()
            require(name not in seen and name == source_entry["name"].upper() and name in surfaces and name in native_surfaces,
                    "Actual/source canonical face summation order differs")
            seen.add(name)
            own, reference = surfaces[name], native_surfaces[name]
            require(integer(face["id"], "actual volume SurfaceId") == own["id"]
                    and integer(source_entry["own_surface_id"], "original ordered SurfaceId", 1) == reference["id"]
                    and exact(face["class"], own["class"]) and reference["class"] == CLASS_CODES[face["class"]]
                    and source_entry["class"] == reference["class"], "Volume face identity/class binding differs")
            vertices = face["world_vertices_m"]
            require(type(vertices) is list and len(vertices) == 4 and all(type(p) is list and len(p) == 3 for p in vertices)
                    and exact([[scalar_token(x) for x in p] for p in vertices], own["world_vertex_bits"])
                    and exact(own["world_vertex_bits"], reference["world_vertex_bits"]), "Actual stored summation input vertices differ")
            for field in ("area_m2", "gross_area_m2"):
                comparison.numeric(face[field], reference[field], frozen_profiles["area_m2"], label+"/"+name+"/"+field)
            av, rv = face["newell_area_vector_m2"], reference["newell_area_vector_m2"]
            require(type(av) is list and len(av) == len(rv) == 3, "Newell vector shape differs")
            for axis, (actual, expected) in enumerate(zip(av, rv)):
                comparison.numeric(actual, expected, frozen_profiles["area_m2"], label+"/"+name+"/Newell/"+str(axis))
            scalar_token(face["tilt_deg"])
        diagnostic = value["diagnostics"]
        require(integer(diagnostic["initial_unique_vertex_count"], "actual unique vertices", 1) > 0
                and diagnostic["initial_edges_not_used_twice"] == [] and diagnostic["initially_closed"] is True
                and diagnostic["edges_winding_consistent"] is True and diagnostic["topology_rejection"] is None,
                "Actual Rust automatic-volume topology required correction")
        for field in ("floor_horizontal", "roof_horizontal", "walls_vertical", "same_wall_height", "volume_differs_by_more_than_five_percent"):
            require(type(diagnostic[field]) is bool, "Actual own local predicate is not boolean")
        require(diagnostic["signed_polyhedron_volume_m3"] is not None, "Actual Rust signed-volume observation missing")
        scalar_token(diagnostic["signed_polyhedron_volume_m3"])
        # Native ordinary snapshots do not expose local CalcVolume. No inference
        # from a retained positive/entered-height selected Volume is permitted.
    final = read(output / "geometry-consumers.json")
    require(final["schema"] == "geo01-runtime-consumers.v1" and final["physics_executed"] is True
            and final["observer_supplies_inputs"] is False and final["phase"] == "runtime_final_state_geometry", "Actual final state projection differs")
    stored = named(final["zones"])
    require(set(stored) == set(zones), "Stored final zone name/ID set differs")
    volume_bits = scalar_token(first["zone"]["volume_m3"])
    for name, row in stored.items():
        require(integer(row["id"], "stored ZoneId") == zones[name]["id"]
                and bits(row["rust_consumer"]["volume_m3"]) == row["rust_consumer"]["volume_bits"] == volume_bits,
                "Initializer selected Volume was not retained in actual final zone state")
        comparison.numeric({"value": row["rust_consumer"]["volume_m3"], "value_bits": volume_bits, "value_class": "finite"},
            source["final_weather"]["zones"][0]["volume_m3"], frozen_profiles["volume_m3"], case["id"]+"/actual_final_stored_volume")
    return volume_bits


def clock_points(output, expected, scope):
    clock = read(output / "clock-calls.json")
    require(clock["schema"] == "clk01-clock-trace.v1" and clock["capture_source"] == "existing-physical-zone-loop-hook"
            and clock["complete_on_collecting_thread"] is True and clock["prepared_rows_are_executed_events"] is False
            and integer(clock["total_invocation_count"], "clock total") == integer(clock["recorded_invocation_count"], "clock retained") == expected
            and integer(clock["omitted_invocation_count"], "clock omitted") == 0 and clock["truncation_reason"] is None
            and len(clock["zone_invocations"]) == expected, "Actual zone-hook prefix incomplete")
    frames, environment = clock["prepared_hourly_frames"], clock["prepared_environment_points"]
    require(len(frames) == expected // 4 and len(environment) == (expected if scope == "B" else 0), "Actual prepared clock cardinality differs")
    for position, event in enumerate(clock["zone_invocations"]):
        hour, substep = divmod(position, 4)
        for key, expected_value in (("sequence", position+1), ("hour_index", hour), ("zone_timestep", substep+1),
                                    ("zone_steps_per_hour", 4), ("calendar_frame_index", hour)):
            require(integer(event[key], key) == expected_value, "Actual zone clock order differs")
        require(event["timestep_seconds_bits"] == bits(900.0)
                and exact(event["environment_point_index"], position if scope == "B" else None)
                and integer(frames[hour]["hourly_sample_index"], "frame index") == hour, "Actual clock duration/index differs")
    return clock


def event_context(context, clock):
    if context is None:
        return None, False
    require(type(context) is dict and set(context) == {"scope", "zone_timestep", "system_call"}, "Malformed actual execution context")
    require(context["scope"] in PHYSICAL_SCOPES | {"coupled_output_validation"}, "Unknown actual execution scope")
    zone = context["zone_timestep"]
    require(type(zone) is dict, "Missing actual zone cursor")
    sample = integer(zone["sample_index"], "actual capacity sample")
    require(sample < len(clock["zone_invocations"]), "Capacity event references missing actual zone hook")
    event = clock["zone_invocations"][sample]
    for key in ("hour_index", "zone_timestep", "zone_steps_per_hour", "timestep_seconds_bits"):
        require(exact(zone[key], event[key]), "Actual capacity event clock operands differ")
    frame = clock["prepared_hourly_frames"][event["calendar_frame_index"]]
    require(exact(zone["calendar"], {k: frame[k] for k in ("year", "month", "day_of_month", "day_of_sim", "hour_ending")}), "Actual capacity calendar differs")
    if event["environment_point_index"] is None:
        require(zone["environment"] is None, "Invented materialized environment")
    else:
        point = dict(clock["prepared_environment_points"][event["environment_point_index"]])
        point["environment_index"] = point.pop("materialized_environment_index")
        require(exact(zone["environment"], point), "Actual capacity environment operands differ")
    if context["scope"] == "direct_zone_purchased_air_system_call":
        system = context["system_call"]
        require(type(system) is dict and integer(system["schedule_sample_index"], "actual system sample") == sample
                and exact(system["timestep_seconds_bits"], zone["timestep_seconds_bits"])
                and type(system["begin_environment"]) is bool, "Actual system-call operands differ")
    else:
        require(context["system_call"] is None, "Invented system call")
    return sample, context["scope"] in PHYSICAL_SCOPES


def capacity_handoffs(case, output, expected_volume_bits, clock, available, bindings, comparison):
    trace = read(output / "psychrometrics-calls.json")
    require(trace["schema"] == "psychrometrics-calls.v2" and trace["capture_source"] == "actual-rust-pipeline-kernel-hooks"
            and trace["thread_coverage"] == "collecting-thread-only" and trace["complete_on_collecting_thread"] is True
            and integer(trace["omitted_call_count"], "kernel omitted") == 0 and trace["truncation_reason"] is None
            and integer(trace["total_call_count"], "kernel total") == integer(trace["recorded_call_count"], "kernel retained") == len(trace["ordered_ids"])
            and exact(trace["input_order"]["RustZoneAirHeatCapacity"], ["Volume_m3", "P_Pa", "T_C", "W_kg_kg"]),
            "Actual kernel trace incomplete or argument order differs")
    dictionary = trace["dictionary"]
    require(type(dictionary) is list and type(trace["ordered_ids"]) is list and len(dictionary) > 0, "Missing kernel tuple dictionary")
    selected = {}
    for index, row in enumerate(dictionary):
        if row["routine"] != "RustZoneAirHeatCapacity":
            continue
        tokens = row["input_bits"]
        require(type(tokens) is list and len(tokens) == len(row["inputs"]) == 4
                and exact([bits(x) for x in row["inputs"]], tokens)
                and bits(row["result"]) == row["result_bits"] and row["cp_cache"] is None,
                "Actual capacity IEEE argument/result encoding differs")
        require(all(from_bits(x) > 0 for x in tokens[:2]), "Actual volume/pressure arguments are not physical")
        # The caller alone is shared with report recomputation. Only the real
        # updater's explicit phase and genuine execution cursor qualify.
        if row["phase"] == "zone_air_heat_capacity_update":
            caller(row, available, bindings, CAPACITY_CALLER)
            sample, active = event_context(row["context"], clock)
            if active:
                comparison.metadata(tokens[0], expected_volume_bits, case["id"]+"/actual_physical_capacity_dictionary/"+str(index)+"/Volume_bits")
                selected[index] = sample
    counts, coverage, physical_events, first_seen = Counter(), set(), 0, set()
    previous_sample = -1
    for token in trace["ordered_ids"]:
        index = integer(token, "kernel dictionary ID")
        require(index < len(dictionary), "Kernel event references missing tuple")
        first_seen.add(index)
        counts[dictionary[index]["routine"]] += 1
        if index in selected:
            require(selected[index] >= previous_sample, "Actual physical capacity event order regressed relative to the zone clock")
            previous_sample = selected[index]
            physical_events += 1
            coverage.add(selected[index])
    require(first_seen == set(range(len(dictionary))), "Kernel dictionary has unexecuted tuples")
    require(type(trace["routine_counts"]) is dict and all(type(v) is int and v >= 0 for v in trace["routine_counts"].values())
            and exact(dict(counts), trace["routine_counts"]), "Actual routine counts differ from ordered events")
    require(physical_events > 0 and coverage == set(range(len(clock["zone_invocations"]))),
            "Actual volume capacity handoff omits physical intervals")
    return {"actual_physical_capacity_calls": physical_events, "covered_zone_intervals": len(coverage),
        "updater_phase": "zone_air_heat_capacity_update", "actual_caller": CAPACITY_CALLER,
        "all_capacity_routine_calls": counts["RustZoneAirHeatCapacity"],
        "other_capacity_calls_not_claimed_physical_updates": counts["RustZoneAirHeatCapacity"]-physical_events,
        "stored_volume_argument_bits_equal": True, "original_AirPowerCap_result_compared": False}


def invariance(case, full, summary, full_data, summary_data, comparison):
    before = sum(comparison.failures.values())
    comparison.metadata(sorted(full_data), sorted(summary_data), case+"/ordinary_summary_keys")
    for key in set(full_data)-{"artifacts", "timing", "input", "config"}:
        comparison.metadata(full_data[key], summary_data[key], case+"/ordinary_summary/"+key)
    comparison.metadata({k: v for k, v in full_data["config"].items() if k != "trace_level"},
        {k: v for k, v in summary_data["config"].items() if k != "trace_level"}, case+"/ordinary_config")
    for name in ("results/selected-outputs.csv", "results/meters.csv"):
        comparison.metadata(ref(full/name)["sha256"], ref(summary/name)["sha256"], case+"/"+name)
    for name in ("results/result-store.json", "porting_scope.json"):
        comparison.metadata(read(full/name), read(summary/name), case+"/"+name)
    require(not any((summary/name).exists() for name in FULL_ONLY), "Summary emitted Full-only observations")
    return {"ordinary_outputs_equal": sum(comparison.failures.values()) == before, "Summary_observer_files_absent": True,
        "Summary_direct_ten_field_initializer_observation": False}


def negative_observation(case, output, summary, owner, native):
    require(case["scope"] is None and case["native_outcome_policy"] == "observe-only-no-predetermined-exit",
            "Negative ordinary diagnostic relabeled CON or fatal parity")
    for name in ("geometry-consumers.json", "geo02-geometry.json", "results/selected-outputs.csv", "results/meters.csv", "results/result-store.json"):
        require(not (output/name).exists(), "Rejected topology produced final physics data")
    clock_count = None
    if (output / "clock-calls.json").exists():
        clock = read(output / "clock-calls.json")
        require(clock["schema"] == "clk01-clock-trace.v1" and integer(clock["total_invocation_count"], "negative zone hooks") == 0
                and integer(clock["recorded_invocation_count"], "negative retained hooks") == 0
                and integer(clock["omitted_invocation_count"], "negative omitted hooks") == 0 and clock["zone_invocations"] == [],
                "Rejected topology invoked physical zone loops")
        clock_count = 0
    diagnostics = read(output / "diagnostics.json")["diagnostics"]
    errors = [x for x in diagnostics if x["severity"] == "error"]
    require(len(errors) == 1 and errors[0]["blocking"] is True and errors[0]["code"] == "RuntimeConvergenceFailure"
            and errors[0]["stage"] == "runtime" and all(row["rejection"] in errors[0]["message"] for row in owner["observations"]),
            "Actual runtime diagnostic is not the observed volume topology rejection")
    return {"case_id": case["id"], "Rust_actual_exit": summary["exit_code"], "Rust_status": summary["status"],
        "Rust_runtime_summary_null": summary["rust_runtime"] is None, "actual_initializer_rejections": len(owner["observations"]),
        "actual_zone_hook_count": clock_count, "physics_data_absent": True,
        "native_actual_exit": native["energyplus_exit_code"], "native_actual_physical_callbacks": native["physical_zone_callback_count"],
        "native_final_volume_observation": native["final_weather"]["zones"][0]["volume_m3"] if native["final_weather"] is not None else None,
        "native_warning_and_fallback_retained_in_original_receipts": True,
        "native_Rust_exit_text_or_fallback_parity_claimed": False, "missing_observers_are_not_fabricated_zero_counters": True}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--matrix", required=True, help="Actual immutable seven-command GEO-03 matrix")
    args = parser.parse_args()
    bindings = Bindings()
    provenance_ref = ref(Path(__file__).with_name("geo03_provenance.py"))
    require(provenance_ref["sha256"] == PROVENANCE_SHA, "Reviewed metadata reader changed")
    bindings.check(provenance_ref)
    matrix_ref = ref(path_of(args.matrix))
    matrix = read(bindings.check(matrix_ref))
    refs, source, cases, tolerances, originals, completed, build, available, compilation, _ = producer(matrix, bindings)
    frozen_profiles = profiles(tolerances)
    frozen = {x["id"]: x for x in cases["native_cases"]}
    comparison, results, negatives, directories = Comparison(), [], [], set()
    for item in matrix["cases"] + matrix["negative_cases"]:
        case = frozen[item["case_id"]]
        negative = case["id"] in NEGATIVE
        for key in ("scope", "duration", "kind", "input", "weather", "metadata"):
            require(exact(item[key], case[key]), "Producer frozen case binding differs: " + key)
        require(item["configured_porting_scope"] == ("A" if negative else case["scope"]), "Configured scope differs")
        full, full_data, full_wrapper, _ = recorded_run(case, item["Full"], matrix, build, available, compilation, completed, bindings, directories, negative=negative)
        require(full_wrapper["trace_level"] == "full", "Full row recorded a different mode")
        _, zones, surfaces = compiled_projection(full, case, originals[case["id"]], bindings)
        observed = owner_observations(full, zones, available, bindings, negative)
        if negative:
            negatives.append(negative_observation(case, full, full_data, observed, originals[case["id"]]))
            continue
        summary, summary_data, summary_wrapper, _ = recorded_run(case, item["Summary"], matrix, build, available, compilation, completed, bindings, directories)
        require(summary_wrapper["trace_level"] == "summary", "Summary row recorded a different mode")
        volume_bits = compare_initializer(case, full, observed, zones, surfaces, originals[case["id"]], frozen_profiles, comparison)
        clock = clock_points(full, {"24H": 96, "72H": 288}[case["duration"]], case["scope"])
        handoff = capacity_handoffs(case, full, volume_bits, clock, available, bindings, comparison)
        results.append({"case_id": case["id"], "actual_initializer_result_calls": len(observed["observations"]),
            "initializer_artifact": ref(full/"geo03-zone-volume.json"), "capacity_handoff": handoff,
            "Full_Summary": invariance(case["id"], full, summary, full_data, summary_data, comparison),
            "ten_fields_observed_at_actual_initializer_return": True, "ten_fields_retained_in_Rust_physical_state_claimed": False,
            "only_stored_volume_physically_handed_off": True, "ordinary_native_local_CalcVolume_or_method_compared": False})
    bindings.unchanged()
    report = comparison.report()
    report.update(schema="geo03-production-comparison.v1", status="pass" if report["mismatch_count"] == 0 else "fail",
        tool=ref(Path(__file__)), helper_tools=[ref(Path(__file__).with_name(n)) for n in
            ("geo03_production_provenance.py", "check_geo03_units.py", "geo03_provenance.py")],
        producer_matrix=matrix_ref, contracts=refs, build=matrix["build"], implementation_commit=build["implementation_commit"],
        source_inventory_scope="archived available-source inventory; compiler file selection unclaimed",
        archived_source_vs_committed_blobs=build["archived_source_vs_committed_blobs"],
        original_helper_execution=matrix["original_helper_execution"], native_original_matrix=matrix["native_original_matrix"],
        original_first_review=matrix["original_first_review"], cases=results, negative_cases=negatives,
        successful_physical_commands=6, ordinary_topology_negative_commands=1, checked_artifact_bindings=bindings.report(),
        gates_updated=False, original_outputs_supplied_to_Rust=False,
        limitations=["Ten fields are copied from the actual Rust initializer return; only Volume is stored and handed to real capacity calculations.",
            "Native initialization/final field retention is independently observed; this is not a claim of ten retained Rust fields across timesteps.",
            "Fresh physical scope is A24/A72/BBoth24 only, not B72/annual/per-limit renewal.",
            "Summary ordinary outputs prove observer independence, not direct ten-field Summary observation.",
            "Source-only Space/global ErrCount5/warning IO/scratch/fallback lifetimes remain unpaired.",
            "No original local CalcVolume/method is inferred from a selected entered/height-priority Volume.",
            "No AirPowerCap/rhoCp/multiplier/systemdt, ZON02/SYS or full physics equivalence is claimed.",
            "Only actual updater phase, selected caller and real zone/system context qualify capacity physical-update coverage."])
    print(json.dumps(report, indent=2, allow_nan=False))
    return 0 if report["status"] == "pass" else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (ValueError, KeyError, OSError, TypeError) as error:
        print(json.dumps({"schema": "geo03-production-check-error.v1", "status": "fail", "error": str(error), "gates_updated": False}, indent=2))
        raise SystemExit(1)
