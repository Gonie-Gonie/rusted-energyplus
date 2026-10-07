#!/usr/bin/env python3
"""Read-only GEO-03 ordinary CLI, source inventory and input bindings."""
from __future__ import annotations

from pathlib import Path
import sys

sys.dont_write_bytecode = True
from geo03_provenance import (
    ROOT, archive_map, command_paths, exact, execution_interval, frozen_contracts,
    identical_content, integer, path_of, read, ref, require, same_binding, timestamp,
    verify_contract_bindings, verify_original_helper, verify_original_matrix,
    verify_rust_build, verify_sources,
)
from check_geo03_units import bits, verify_original_review

POSITIVE = {"A-24H", "A-72H", "B-BOTH-24H"}
NEGATIVE = {"GEO03-A-TOPOLOGY-DUPLICATE-WALL"}
PROVENANCE_SHA = "5ca27645245ee9e7206a31c8906904394a5216625723cbfad12a5d5f49da9bf5"
REQUIRED_SOURCES = {
    "crates/ep_cli/src/main.rs", "crates/ep_runtime/src/geometry.rs",
    "crates/ep_runtime/src/geometry/zone_volume.rs", "crates/ep_runtime/src/geometry/zone_volume/topology.rs",
    "crates/ep_runtime/src/geometry/zone_volume_trace.rs", "crates/ep_runtime/src/heat_balance/initialization.rs",
    "crates/ep_runtime/src/heat_balance/air_manager.rs", "crates/ep_runtime/src/psychrometrics.rs",
    "crates/ep_runtime/src/psychrometrics/production_trace.rs", "crates/ep_raw_model/src/lib.rs",
    "crates/ep_raw_model/src/idf_order.rs", "crates/ep_compiler/src/compiler.rs",
    "crates/ep_run/src/geo03_trace.rs", "crates/ep_run/src/pipeline.rs",
    "crates/ep_run/src/geometry_trace.rs", "crates/ep_run/src/psychrometrics_trace.rs",
}


def named(rows, minimum=0):
    require(type(rows) is list, "Expected owned row array")
    result, ids = {}, set()
    for row in rows:
        own_id = integer(row["id"], "own ID", minimum)
        require(type(row["name"]) is str and row["name"], "Missing canonical name")
        key = row["name"].upper()
        require(key not in result and own_id not in ids, "Duplicate owned name/ID")
        result[key] = row
        ids.add(own_id)
    return result


def producer(matrix, bindings):
    require(matrix["schema"] == "geo03-production-matrix.v1" and matrix["complete"] is True
            and matrix["requested_subset_complete"] is True and matrix["original_outputs_supplied_to_Rust"] is False
            and matrix["gates_updated"] is False and integer(matrix["actual_command_count"], "CLI commands") == 7,
            "Incomplete or answer-fed production matrix")
    require(type(matrix["requested_case_ids"]) is list and len(matrix["requested_case_ids"]) == 4
            and set(matrix["requested_case_ids"]) == POSITIVE | NEGATIVE,
            "Requested production case set differs")
    refs, source, cases, tolerances = frozen_contracts(matrix["contracts"], bindings)
    verify_sources(source, bindings)
    require(set(cases["Rust_physical_case_ids"]) == POSITIVE and set(cases["Rust_physical_negative_case_ids"]) == NEGATIVE
            and exact(cases["Rust_physical_trace_levels"], ["full", "summary"])
            and exact(cases["Rust_physical_negative_trace_levels"], ["full"]), "Frozen physical scope differs")
    require(type(matrix["cases"]) is list and len(matrix["cases"]) == 3
            and {x["case_id"] for x in matrix["cases"]} == POSITIVE
            and type(matrix["negative_cases"]) is list and len(matrix["negative_cases"]) == 1
            and {x["case_id"] for x in matrix["negative_cases"]} == NEGATIVE,
            "Actual positive/negative case partition differs")
    _, _, helper_finished = verify_original_helper(matrix["original_helper_execution"], refs, source, cases, bindings)
    originals, native_finished = verify_original_matrix(matrix["native_original_matrix"], refs, source, cases, bindings)
    reviewed = verify_original_review(matrix["original_first_review"], matrix["original_helper_execution"], refs, helper_finished, bindings)
    review = read(bindings.check(matrix["original_first_review"]))
    require(same_binding(review["reviewed_native_matrix"], matrix["native_original_matrix"])
            and native_finished <= timestamp(review["original_completed_utc"]) <= reviewed,
            "Reviewed native original chain/chronology differs")
    build, available, compilation = verify_rust_build(matrix["build"], bindings, kind="cli", committed=True,
        required_sources=REQUIRED_SOURCES)
    require(same_binding(build["binary"], matrix["binary"])
            and build["implementation_commit"] == matrix["implementation_commit"]
            and build["crates_tree"] == matrix["crates_tree"], "Shared committed CLI build differs")
    bindings.check(matrix["executed_launcher"])
    archives = matrix["executed_source_archives"]
    require(type(archives) is list and len(archives) == 3, "Executed production launcher archive set differs")
    mapping = {x["historical_path"]: x["archive"] for x in archives}
    require(set(mapping) == {"tools/porting/run_geo03_rust.py", "tools/porting/geo03_provenance.py", "tools/porting/record_command.py"}
            and same_binding(mapping["tools/porting/run_geo03_rust.py"], matrix["executed_launcher"])
            and mapping["tools/porting/geo03_provenance.py"]["sha256"] == PROVENANCE_SHA,
            "Executed launcher/dependency identity differs")
    for item in mapping.values():
        bindings.check(item)
    return refs, source, cases, tolerances, originals, reviewed, build, available, compilation, mapping


def recorded_run(case, item, matrix, build, available, compilation, completed, bindings, directories, negative=False):
    output = path_of(item["output_directory"])
    require(output.is_dir() and output not in directories, "Missing/reused physical run directory")
    directories.add(output)
    wrapper = read(bindings.check(item["execution"]))
    require(wrapper["schema"] == "geo03-Rust-cli-execution.v1" and wrapper["dry_run"] is False
            and wrapper["original_outputs_supplied_to_Rust"] is False and wrapper["gates_updated"] is False
            and path_of(wrapper["output_directory"]) == output, "Wrong ordinary invocation wrapper")
    level = wrapper["trace_level"]
    configured = "A" if negative else case["scope"]
    require(level in {"full", "summary"} and (not negative or level == "full")
            and wrapper["configured_porting_scope"] == configured, "Wrong physical configured scope/trace level")
    for key, expected in (("build", matrix["build"]), ("binary", matrix["binary"]), ("input", case["input"]),
                          ("weather", case["weather"]), ("metadata", case["metadata"]),
                          ("executed_launcher", matrix["executed_launcher"]), ("command_receipt", item["command_receipt"]),
                          ("run_summary", item["run_summary"])):
        require(same_binding(wrapper[key], expected), "Actual wrapper crossbinding differs: " + key)
        bindings.check(wrapper[key])
    require(wrapper["implementation_commit"] == wrapper["execution_repository_head"] == build["implementation_commit"],
            "Actual committed run revision differs")
    execution = read(bindings.check(wrapper["command_receipt"]))
    require(execution["schema"] == "recorded-porting-command.v1" and execution["launch_error"] is None
            and execution["source_bytes_match_before_and_after"] is True and execution["recorder_updates_gates"] is False
            and execution["recorder_supplies_reference_answers"] is False and path_of(execution["cwd"]) == ROOT,
            "Actual recorder boundary differs")
    start, _ = execution_interval(execution, bindings, exit_code=6 if negative else 0)
    require(completed <= start and timestamp(compilation["finished_utc"]) <= start, "Physical CLI precedes original/build review")
    require(execution["repository_before"]["head"] == execution["repository_after"]["head"] == build["implementation_commit"],
            "Repository changed or differs from the actual CLI build")
    command_paths(execution["command"], [matrix["binary"]["path"], "run", case["input"]["path"], "--weather", case["weather"]["path"],
        "--output-dir", str(output), "--mode", "compatibility", "--partial", "deny", "--trace-level", level,
        "--porting-scope", configured], {0, 2, 4, 6})
    require(exact(wrapper["executed_recorder"], execution["executed_launcher"])
            and execution["executed_launcher"]["historical_path"] == "tools/porting/record_command.py",
            "Actual executed recorder identity differs")
    bindings.check(execution["executed_launcher"]["archive"])
    expected_recorder = {x["historical_path"]: x["archive"] for x in matrix["executed_source_archives"]}["tools/porting/record_command.py"]
    require(identical_content(execution["executed_launcher"]["archive"], expected_recorder, bindings), "Executed recorder bytes differ")
    snapshot = read(bindings.check(execution["source_snapshot"]))
    require(snapshot["schema"] == "rust-command-source-snapshot.v1" and snapshot["bytes_normalized"] is False,
            "Actual source snapshot claim differs")
    executed = archive_map(snapshot["files"], bindings)
    require(set(executed) == set(available) and all(identical_content(executed[k], available[k], bindings) for k in available),
            "Physical invocation available-source inventory differs from actual build")
    require(exact(wrapper["artifacts"], item["artifacts"]), "Wrapper/output inventories differ")
    artifacts = {}
    for binding in item["artifacts"]:
        path = bindings.check(binding)
        require(path.is_relative_to(output) and path not in artifacts, "Duplicate/foreign artifact")
        artifacts[path] = binding
    require(set(artifacts) == {p.resolve() for p in output.rglob("*") if p.is_file()}, "Retained physical output inventory differs")
    summary_path = output / "run-summary.json"
    require(summary_path in artifacts and same_binding(item["run_summary"], artifacts[summary_path]), "Actual run summary absent")
    summary = read(summary_path)
    require(type(execution["exit_code"]) is int and type(summary["exit_code"]) is int
            and summary["exit_code"] == execution["exit_code"] == (6 if negative else 0), "Actual CLI outcome differs")
    config = summary["config"]
    require(config["dry_run"] is False and config["mode"] == "compatibility" and config["partial_policy"] == "deny"
            and config["trace_level"] == level and config["oracle_baseline"] is False and config["compare_oracle"] is False
            and config["hours"] is None and config["output_format"] == "rust-native"
            and summary["oracle"] is None and summary["comparison"] is None
            and summary["oracle_status"] == summary["compare_status"] == "not-requested", "Nonphysical or answer-fed configuration")
    if negative:
        require(summary["status"] == "runtime" and summary["message"] == "Rust runtime failed" and summary["rust_runtime"] is None,
                "Negative did not reach actual early runtime initialization rejection")
    else:
        require(summary["status"] == "success" and summary["message"] == "arbitrary run completed"
                and integer(summary["rust_runtime"]["samples"], "actual hourly samples") == {"24H": 24, "72H": 72}[case["duration"]]
                and summary["source_order_gate"]["matches"] is True and summary["selected_algorithm_lane"]["diagnostic_probe_used"] is False
                and summary["support"]["conformance_claim"] is False, "Successful supported normal route absent")
        if configured == "B":
            require(summary["rust_runtime"]["fixture_demand_injection_used"] is False
                    and integer(summary["rust_runtime"]["purchased_air_coupling_call_count"], "B actual system coupling") == 96,
                    "B is fixture-driven or missing real coupling")
    return output, summary, wrapper, artifacts


def fnv1a(content):
    value = 14695981039346656037
    for byte in content:
        value = ((value ^ byte) * 1099511628211) & ((1 << 64) - 1)
    return f"{value:016x}"


def raw_numbers(parsed):
    """Bits of actual parsed numerical values; parser-only IDF bookkeeping excluded."""
    actual = {}
    def visit(value, path):
        if type(value) in (int, float):
            if not any(x.startswith("idf_") for x in path.split("/")):
                key = path.upper()
                require(key not in actual, "Duplicate canonical parsed path")
                actual[key] = bits(value)
        elif type(value) is list:
            for i, child in enumerate(value):
                visit(child, path+"/"+str(i))
        elif type(value) is dict:
            for key, child in value.items():
                visit(child, path+"/"+key)
    visit(parsed["objects"], "")
    emitted = {}
    for row in parsed["numeric_bits"]:
        require(type(row["path"]) is str, "Invalid parsed numeric path")
        if any(x.startswith("idf_") for x in row["path"].split("/")):
            continue
        key = row["path"].upper()
        require(key not in emitted and type(row["bits"]) is str, "Duplicate/invalid numerical path observation")
        emitted[key] = row["bits"]
    require(exact(emitted, actual), "Actual parsed numeric observations differ from numbers")
    return emitted


def compiled_projection(output, case, original, bindings):
    projection = read(bindings.check(ref(output / "compiled-geometry.json")))
    require(projection["schema"] == "compiled-geometry.v1" and projection["phase"] == "typed_compile"
            and projection["physics_executed"] is False and projection["preparation_only"] is True
            and projection["observer_supplies_inputs"] is False and projection["surface_idf_declaration_overlay"] is True,
            "Compiled geometry is not actual bounded preparation/IDF-order observation")
    hashes = read(bindings.check(ref(output / "input/input-hashes.json")))
    require(hashes["algorithm"] == projection["inputs"]["hash_algorithm"] == "fnv-1a-64"
            and projection["inputs"]["source_assisted_lexical_conversion"] is True
            and projection["inputs"]["conversion_is_physical_observation"] is False, "Lexical conversion proof boundary differs")
    for key, name in (("source", "original_input"), ("staged_original", "staged_original_input"), ("converted_epjson", "converted_epjson")):
        item = hashes[key]
        require(exact(item, projection["inputs"][name]), "Actual input receipt differs")
        path = path_of(item["path"])
        content = path.read_bytes()
        require(integer(item["bytes"], "staged input bytes") == len(content) and item["hash"] == fnv1a(content), "Staged input hash differs")
        binding = ref(path)
        bindings.check(binding)
        if key == "source":
            require(same_binding(binding, case["input"]), "Actual staged source differs from frozen IDF")
        else:
            require(path.is_relative_to(output), "Staged input belongs to another run")
            if key == "staged_original":
                require(binding["sha256"] == case["input"]["sha256"], "Staged original IDF was modified")
    require(original["parsed_input"]["is_epjson"] is False and original["parsed_input"]["preserve_idf_order"] is True
            and exact(raw_numbers(projection["parsed_input"]), raw_numbers(original["parsed_input"])),
            "Actual original/Rust parsed numerical input bits or order flags differ")
    zones, surfaces = named(projection["zones"]), named(projection["surfaces"])
    native_zones, native_surfaces = named(original["first_initialized"]["zones"], 1), named(original["first_initialized"]["surfaces"], 1)
    require(len(zones) == 1 and len(surfaces) == 6 and set(zones) == set(native_zones) and set(surfaces) == set(native_surfaces),
            "Actual compiled/native owner names differ")
    own_zone = next(iter(zones.values()))["id"]
    parsed_zones = {name.upper(): value for name, value in original["parsed_input"]["objects"]["Zone"].items()}
    require(set(parsed_zones) == set(zones), "Actual parsed/compiled Zone identity differs")
    for name, zone in zones.items():
        for observed, field in (("declared_ceiling_height", "ceiling_height"), ("declared_volume", "volume"), ("declared_floor_area", "floor_area")):
            number = parsed_zones[name].get(field, "AutoCalculate")
            declared = zone[observed]
            if type(number) is str and number.casefold() == "autocalculate":
                require(exact(declared, {"kind": "AutoCalculate"}), "Actual typed AutoCalculate declaration differs")
            else:
                require(type(number) in (int, float) and type(declared) is dict
                        and set(declared) == {"kind", "value", "value_bits"} and declared["kind"] == "Value"
                        and bits(declared["value"]) == declared["value_bits"] == bits(number),
                        "Actual typed declared Zone operand bits differ from parsed original input")
    for name, surface in surfaces.items():
        source = native_surfaces[name]
        require(integer(surface["zone_id"], "compiled zone owner") == own_zone and surface["zone_name"].upper() in zones
                and integer(surface["sides"], "compiled sides", 1) == integer(source["sides"], "native sides", 1) == 4
                and exact(surface["world_vertex_bits"], source["world_vertex_bits"]), "Actual volume input vertex identity differs")
        require(len(surface["world_vertices_m"]) == 4
                and exact([[bits(v) for v in point] for point in surface["world_vertices_m"]], surface["world_vertex_bits"]),
                "Actual compiled vertex numbers/bits differ")
    return projection, zones, surfaces
