#!/usr/bin/env python3
"""Compare preserved genuine GEO-02 helper results with input-only Rust results.

Only recorded artifacts are read. No engine, Git command, geometry formula,
angle wrapping, tolerance adjustment, artifact write or gate update is performed.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path
import re
import sys

sys.dont_write_bytecode = True
from geo02_unit_provenance import (
    ROOT, PIN, Bindings, Comparison, bits, exact, from_bits, path_of, read, ref,
    require, same_binding, scalar, sha, timestamp, value_class,
)

CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
SOURCE = ROOT / ".reference/energyplus-src/26.1.0"
FIELDS = {
    "area_m2": ("area_m2", 1), "gross_area_m2": ("area_m2", 1),
    "net_area_shadow_m2": ("area_m2", 1), "azimuth_deg": ("angles_deg", 1),
    "tilt_deg": ("angles_deg", 1), "newell_area_vector_m2": ("newell_area_vector_m2", 3),
    "newell_normal": ("normals_and_lcs", 3), "out_norm": ("normals_and_lcs", 3),
    "centroid_m": ("centroid_m", 3), "lcsx": ("normals_and_lcs", 3),
    "lcsy": ("normals_and_lcs", 3), "lcsz": ("normals_and_lcs", 3),
    "sin_azimuth": ("trig", 1), "cos_azimuth": ("trig", 1),
    "sin_tilt": ("trig", 1), "cos_tilt": ("trig", 1),
}
BASELINE_FIELDS = {"area_m2", "azimuth_deg", "tilt_deg"}
PROFILE_RULES = {
    "area_m2": ("m2", 1e-10), "newell_area_vector_m2": ("m2", 1e-10),
    "centroid_m": ("m", 1e-10), "normals_and_lcs": ("1", 1e-12),
    "trig": ("1", 1e-12), "angles_deg": ("deg", 1e-10),
}


def profiles(tolerances):
    require(tolerances["schema"] == "geo02-tolerances.v1"
            and tolerances["frozen_before_numerical_comparison"] is True,
            "Unfrozen or wrong tolerance contract")
    require(set(tolerances["profiles"]) == set(PROFILE_RULES), "Tolerance profile set differs")
    for name, (unit, atol) in PROFILE_RULES.items():
        row = tolerances["profiles"][name]
        require(row == {"unit": unit, "absolute_tolerance": atol, "relative_tolerance": 1e-13,
                        "rule": "abs(actual-reference)<=atol+rtol*abs(reference)",
                        "finite_class": "exact", "both_zero_sign": "exact", "angle_wrapping": "forbidden"},
                "Frozen profile changed: " + name)
    require(tolerances["cen_precision"] == {
        "output_bits": "exact unchanged-header cen3 observation", "input_bits": "exact",
        "class_and_zero_sign": "exact", "numeric_tolerance_used": False,
        "floating_environment": "read actual GNU x64 x87 control word/PC/RC and MXCSR rounding before/after calls and initialization; no control writes; select source-compatible product precision from actual PC observation rather than LDBL_MANT_DIG assumption"},
        "Source cen precision policy differs")
    return tolerances["profiles"]


def scalar_token(value):
    require(type(value) is dict, "Expected a scalar observation object")
    number = scalar(value["value"], value["value_bits"])
    require(value["value_class"] == value_class(number), "Scalar class and observed bits differ")
    return value["value_bits"]


def field_tokens(value, count):
    if count == 1:
        return [scalar_token(value)]
    require(type(value) is list and len(value) == count, "Wrong observed vector cardinality")
    return [scalar_token(component) for component in value]


def vertex_bits(vertices):
    require(type(vertices) is list, "Expected input vertex array")
    output = []
    for vertex in vertices:
        require(type(vertex) is list and len(vertex) == 3, "Expected three-component vertex")
        tokens = [bits(value) for value in vertex]
        require(all(math.isfinite(from_bits(token)) for token in tokens),
                "Helper input contains a nonfinite coordinate")
        output.append(tokens)
    return output


def verify_request(request, cases_contract):
    require(request["schema"] == "geo02-helper-cases.v1", "Wrong helper request schema")
    cases = request["cases"]
    require(type(cases) is list and len(cases) == cases_contract["helper_quad_count"] == 17,
            "Frozen helper quad count differs")
    require(len({row["case_id"] for row in cases}) == len(cases), "Duplicate helper case ID")
    kinds = [row["kind"] for row in cases]
    require(kinds.count("valid_quad") == cases_contract["valid_helper_quads"] == 15
            and kinds.count("source_only_degenerate") == cases_contract["source_only_degenerate_quads"] == 2,
            "Helper admitted/source-only case partition differs")
    for case in cases:
        require(set(case) <= {"case_id", "kind", "surface_class", "vertices_m",
                             "input_vertex_bits", "initial_centroid_m", "branch_intent",
                             "required_kernel_input_identity"},
                "Unexpected helper input field; reference geometry is forbidden")
        require(case["surface_class"] in {"Wall", "Roof", "Floor", "Ceiling"}, "Unknown surface class")
        require(len(case["vertices_m"]) == 4, "Frozen helper input is not a quad")
        require(vertex_bits(case["vertices_m"]) == case["input_vertex_bits"],
                "Request input numbers and emitted bits differ")
        require(len(case["initial_centroid_m"]) == 3, "Initial centroid cardinality differs")
        vertex_bits([case["initial_centroid_m"]])
    cen = request["cen_calls"]
    require(type(cen) is list and len(cen) == cases_contract["cen_precision_input_count"] == 526,
            "Frozen actual cen operand probe count differs")
    require(len({row["case_id"] for row in cen}) == len(cen), "Duplicate cen case ID")
    for call in cen:
        require(set(call) <= {"case_id", "x_operands", "x_operand_bits", "kind"},
                "Unexpected cen input field; reference values are forbidden")
        require(len(call["x_operands"]) == len(call["x_operand_bits"]) == 3,
                "Actual cen requires all three declared input operands")
        require([bits(value) for value in call["x_operands"]] == call["x_operand_bits"],
                "Cen input bits differ from declared values")
        vertex_bits([call["x_operands"]])


def verify_sources(source_contract):
    require(source_contract["schema"] == "geo02-source-contract.v1"
            and source_contract["energyplus_commit"] == PIN, "Wrong original source pin")
    verified = []
    for row in source_contract["source_files"]:
        path = (SOURCE / row["path"]).resolve()
        require(path.is_relative_to(SOURCE) and sha(path) == row["sha256"],
                "Changed locked source file: " + row["path"])
        verified.append(ref(path))
    for row in source_contract["selected_ranges"]:
        path = (SOURCE / row["path"]).resolve()
        require(path.is_relative_to(SOURCE), "Source range path escapes locked source")
        lines = path.read_text(encoding="utf-8").splitlines(keepends=True)
        require(type(row["start"]) is int and type(row["end"]) is int
                and 1 <= row["start"] <= row["end"] <= len(lines), "Invalid source range")
        text = "".join(lines[row["start"] - 1:row["end"]])
        require(hashlib.sha256(text.encode()).hexdigest() == row["range_sha256"],
                "Changed locked source range")
        require(row["symbol"] in text, "Source symbol absent from selected actual range")
    return verified


def contains_binding(value, target):
    if type(value) is dict:
        if "path" in value and "sha256" in value and same_binding(value, target):
            return True
        return any(contains_binding(child, target) for child in value.values())
    if type(value) is list:
        return any(contains_binding(child, target) for child in value)
    return False


def floating_environment(value):
    for key in ("sizeof_float", "sizeof_double", "sizeof_long_double", "FLT_MANT_DIG",
                "DBL_MANT_DIG", "LDBL_MANT_DIG", "fe_round", "x87_control_word",
                "x87_precision_control_bits", "x87_precision_bits", "x87_rounding_control_bits",
                "mxcsr", "mxcsr_rounding_control_bits"):
        require(type(value[key]) is int, "Floating control field is not an integer: " + key)
    require(0 <= value["x87_control_word"] <= 0xffff and 0 <= value["mxcsr"] <= 0xffffffff,
            "Raw floating control register range differs")
    pc = (value["x87_control_word"] >> 8) & 3
    require(pc in {0, 2, 3} and pc == value["x87_precision_control_bits"]
            and value["x87_precision_bits"] == {0: 24, 2: 53, 3: 64}[pc],
            "Observed x87 precision disagrees with actual control word")
    require(value["x87_rounding_control_bits"] == ((value["x87_control_word"] >> 10) & 3)
            and value["mxcsr_rounding_control_bits"] == ((value["mxcsr"] >> 13) & 3),
            "Observed rounding disagrees with actual control registers")
    require(value["sizeof_double"] == 8 and value["DBL_MANT_DIG"] == 53
            and value["LDBL_MANT_DIG"] == 64 and value["x87_precision_bits"] in (24, 53, 64),
            "Original cen floating precision observation is malformed")
    require(value["fe_round"] == 0 and value["x87_rounding_control_bits"] == 0
            and value["mxcsr_rounding_control_bits"] == 0 and value["control_writes_added"] is False,
            "Original floating control/rounding differs")


def verify_original_stages(row, supplied):
    states = row["source_state"]
    expected_stages = ["after_get_vertices", "after_centroid", "after_process", "after_second_process"]
    require(row["second_process_owned_state_unchanged"] is True, "Original second Process changed owned state")
    require(exact(states["after_process"], states["after_second_process"]),
            "Original repeated Process snapshots differ")
    for stage in expected_stages:
        state = states[stage]
        globals_ = state["globals"]
        require(state["sides"] == 4 and state["vertex_bits"] == supplied["input_vertex_bits"]
                and state["vertex_values_observed"] is True, "Original retained stage vertices differ")
        require(globals_["counterclockwise"] is True and globals_["world_coordinate_system"] is True
                and globals_["aspect_transform"] is False and globals_["no_transform"] is True
                and globals_["total_coincident_vertices"] == 0 and globals_["total_degenerate_surfaces"] == 0,
                "Original helper activated excluded corrections")
        floating_environment(state["floating_environment"])
    require(states["after_centroid"]["vertices_processed"] is False
            and states["after_process"]["vertices_processed"] is True
            and states["after_centroid"]["globals"]["process_one_time_flag"] is True
            and states["after_process"]["globals"]["process_one_time_flag"] is False,
            "Actual original Process lifecycle differs")
    for array in ("xpsv", "ypsv", "zpsv"):
        require(states["after_process"]["globals"][array] == {"allocated": True, "size": 4},
                "Actual original Process scratch shape differs")
    for stage in ("after_centroid", "after_process", "after_second_process"):
        require(exact(states[stage]["geometry"], row["geometry"]),
                "Original geometry fields changed after the selected producer")


def compare_results(request, native, rust, frozen_profiles, baseline):
    require(native["schema"] == rust["schema"] == "geo02-helper-results.v1", "Wrong result schema")
    require(native["valid_kernel_input_identity_passed"] is True
            and native["expected_answers_supplied"] is False
            and native["original_parser_admission_claimed"] is False
            and native["physics_executed"] is False and native["gates_updated"] is False,
            "Original helper's actual preparation/proof boundary differs")
    require(rust["physics_executed"] is False and rust["reference_outputs_supplied_to_Rust"] is False,
            "Helper projection must not claim physics or consume original answers")
    require(len(native["cases"]) == len(rust["cases"]) == len(request["cases"]),
            "Actual helper result cardinality differs")
    if not baseline:
        profile = rust["cen_precision_profile"]
        observed_modes = {row["floating_environment_before"]["x87_precision_bits"] for row in native["cen_calls"]}
        observed_modes |= {row["floating_environment_after"]["x87_precision_bits"] for row in native["cen_calls"]}
        for row in native["cases"]:
            if row["kind"] == "valid_quad":
                for stage in ("after_get_vertices", "after_centroid", "after_process", "after_second_process"):
                    observed_modes.add(row["source_state"][stage]["floating_environment"]["x87_precision_bits"])
        require(len(observed_modes) == 1 and type(profile["product_precision_bits"]) is int
                and profile["product_precision_bits"] == next(iter(observed_modes)),
                "Rust product precision is not bound to actual original cen/centroid mode")
        require(type(profile["sum_precision_bits"]) is int and profile["sum_precision_bits"] == 53
                and profile["third_bits"] == "3fd5555555555555"
                and profile["rounding"] == "nearest_ties_even"
                and profile["control_writes_added"] is False
                and profile["selection_basis"] == "actual_original_x87_precision_observation",
                "Canonical source-compatible cen precision metadata differs")
    comparison = Comparison()
    unpaired = []
    for supplied, original, candidate in zip(request["cases"], native["cases"], rust["cases"]):
        label = supplied["case_id"]
        for row in (original, candidate):
            comparison.metadata(row["case_id"], label, label + "/case_id")
            comparison.metadata(row["kind"], supplied["kind"], label + "/kind")
            comparison.metadata(row["input"], supplied, label + "/own_input")
        require(original["input_vertex_bits"] == supplied["input_vertex_bits"],
                "Original parsed helper input differs from frozen request")
        require(candidate["observed_input_vertex_bits"] == supplied["input_vertex_bits"],
                "Actual Rust typed storage differs from frozen helper input")
        if supplied["kind"] == "source_only_degenerate":
            require(original["status"] == candidate["status"] == "unsupported_source_only",
                    "Unsafe helper may not claim paired admission or completed geometry")
            unpaired.append({"case_id": label, "original_source_state": original["source_state"],
                             "source_only_diagnostics": original["source_only_diagnostics"],
                             "Rust_status": candidate["status"]})
            continue
        require(original["kernel_input_identity_checked"] is True
                and original["retained_vertex_bits"] == supplied["input_vertex_bits"],
                "GetVertices changed helper-facing input; pairing must stop before arithmetic")
        require(original["status"] == "source_complete", "Original valid helper did not complete")
        verify_original_stages(original, supplied)
        require(candidate["status"] == ("baseline_partial" if baseline else "source_complete"),
                "Rust result stage differs from requested comparison boundary")
        for field, (profile, count) in FIELDS.items():
            original_tokens = field_tokens(original["geometry"][field], count)
            if field not in candidate["geometry"]:
                comparison.missing(label + "/" + field)
                continue
            candidate_tokens = field_tokens(candidate["geometry"][field], count)
            for index, (actual, expected) in enumerate(zip(candidate_tokens, original_tokens)):
                comparison.numeric(actual, expected, frozen_profiles[profile], f"{label}/{field}/{index}")
        if baseline:
            require(set(candidate["geometry"]) == BASELINE_FIELDS,
                    "Baseline may expose only actually existing public helper outputs")
    require(len(native["cen_calls"]) == len(rust["cen_calls"]) == len(request["cen_calls"]),
            "Actual cen result cardinality differs")
    for supplied, original, candidate in zip(request["cen_calls"], native["cen_calls"], rust["cen_calls"]):
        label = supplied["case_id"]
        comparison.metadata(original["case_id"], label, label + "/case_id")
        comparison.metadata(original["input"], supplied, label + "/original_own_cen_input")
        comparison.metadata(candidate["input"], supplied, label + "/Rust_own_cen_input")
        require(original["route"] == "unchanged-ObjexxFCL-cen3", "Wrong original cen producer")
        floating_environment(original["floating_environment_before"])
        floating_environment(original["floating_environment_after"])
        before, after = original["floating_environment_before"], original["floating_environment_after"]
        # Sticky exception flags are genuine numerical effects, not control
        # writes. Compare control modes without pretending those flags are inert.
        for key in ("sizeof_double", "DBL_MANT_DIG", "LDBL_MANT_DIG", "fe_round",
                    "x87_control_word", "x87_precision_control_bits", "x87_precision_bits",
                    "x87_rounding_control_bits", "mxcsr_rounding_control_bits", "control_writes_added"):
            require(exact(before[key], after[key]), "Original cen changed floating control: " + key)
        require((before["mxcsr"] & ~0x3f) == (after["mxcsr"] & ~0x3f),
                "Original cen changed MXCSR control bits")
        original_token = scalar_token(original["value"])
        if baseline:
            require(candidate["status"] == "unsupported_baseline" and "value" not in candidate,
                    "Baseline may not invent a cen implementation")
            comparison.missing(label + "/cen3")
        else:
            comparison.metadata(candidate["case_id"], label, label + "/Rust_case_id")
            comparison.precision(scalar_token(candidate["value"]), original_token, label + "/cen3")
    return comparison, unpaired


def verify_contract_bindings(actual, expected, bindings):
    require(set(actual) == set(expected), "Execution contract set differs")
    for key in expected:
        require(same_binding(actual[key], expected[key]), "Execution contract freeze differs: " + key)
        bindings.check(actual[key])


def command_paths(actual, expected):
    require(type(actual) is list and len(actual) == len(expected), "Helper command cardinality differs")
    require(all(path_of(a) == path_of(b) for a, b in zip(actual, expected)),
            "Helper command is not the archived binary plus frozen input-only request")


def verify_native(receipt, contract_refs, request_ref, bindings):
    require(receipt["schema"] == "geo02-original-helper-execution.v1"
            and receipt["kernel_input_identity_passed"] is True
            and receipt["original_parser_admission_claimed"] is False
            and receipt["physics_executed"] is False and receipt["Rust_compared"] is False
            and receipt["gates_updated"] is False, "Original helper provenance boundary differs")
    verify_contract_bindings(receipt["contracts"], contract_refs, bindings)
    require(same_binding(receipt["request"], request_ref), "Original request differs from frozen input")
    bindings.check(receipt["request"])
    result_path = bindings.check(receipt["results"])
    execution = read(bindings.check(receipt["execution"]))
    require(execution["exit_code"] == 0 and path_of(execution["cwd"]) == ROOT, "Original helper failed/wrong cwd")
    command_paths(execution["command"], [receipt["binary"]["path"], request_ref["path"]])
    bindings.check(execution["stdout"])
    bindings.check(execution["stderr"])
    require(exact(read(bindings.check(execution["stdout"])), read(result_path)),
            "Original retained results differ from actual executable stdout")
    require(timestamp(execution["started_utc"]) <= timestamp(execution["finished_utc"]),
            "Invalid original execution chronology")
    bindings.check(receipt["binary"])
    core = read(bindings.check(receipt["native_core_build"]))
    driver = read(bindings.check(receipt["native_driver_build"]))
    require(core["checks_passed"] is True and core["energyplus_commit"] == PIN
            and core["scientific_source_patches"] is False, "Unverified genuine original core")
    for key in ("core_library", "api_library"):
        bindings.check(core["artifacts"][key])
    require(driver["schema"] == "geo02-native-driver-build.v1"
            and driver["checks_passed"] is True and driver["energyplus_commit"] == PIN,
            "Unverified genuine original driver")
    require(same_binding(driver["core_build"], receipt["native_core_build"])
            and same_binding(driver["helper_binary"], receipt["binary"]),
            "Helper driver/core/binary bindings differ")
    verify_contract_bindings(driver["contracts"], contract_refs, bindings)
    for key in ("frozen_original_build_commands_unchanged", "existing_target_compile_commands_unchanged",
                "container_library_unchanged", "original_core_and_API_bytes_unchanged",
                "same_compiler_owned_state", "original_directory_and_target_definitions_inherited",
                "assertions_enabled"):
        require(driver[key] is True, "Original build invariant did not pass: " + key)
    require(driver["scientific_source_patches"] is False and driver["fp_contract"] == "off",
            "Original reference numerical configuration differs")
    for key in ("configure", "compile_link"):
        command = read(bindings.check(driver[key]))
        require(command["exit_code"] == 0, "Original driver build command failed")
        bindings.check(command["stdout"])
        bindings.check(command["stderr"])
    compile_rows = read(bindings.check(driver["actual_compile_commands"]))
    require(len(compile_rows) == 2, "Missing actual original driver compile rows")
    helper_rows = [row for row in compile_rows if Path(row["file"]).name == "geo02_reference_helper.cpp"]
    require(len(helper_rows) == 1, "Missing original helper translation unit")
    for row in compile_rows:
        for flag in ("-DEP_psych_errors", "-UNDEBUG", "-Werror", "-O0", "-ffp-contract=off"):
            require(flag in row["command"], "Original compile flag missing: " + flag)
        for flag in ("-D_GLIBCXX_DEBUG", "-ffast-math", "-DEP_psych_stats"):
            require(flag not in row["command"], "Original compile configuration differs: " + flag)
    source_archives = {}
    for row in driver["executed_source_archives"]:
        require(row["historical_path"] not in source_archives, "Duplicate original driver source")
        bindings.check(row["archive"])
        source_archives[row["historical_path"]] = row["archive"]
    for row in receipt["executed_source_archives"]:
        bindings.check(row["archive"])
        require(row["historical_path"] in source_archives
                and row["archive"]["sha256"] == source_archives[row["historical_path"]]["sha256"],
                "Original executed wrapper source differs from built bytes")
    for name in ("geo02_reference_helper.cpp", "geo02_reference_fields.hh", "geo02_reference.cmake"):
        require("tools/porting/" + name in source_archives, "Missing built helper/field/CMake source archive")
    fragment = driver["setup_fragment"]
    bindings.check(fragment["artifact"])
    source = read(bindings.check(contract_refs["source"]))
    selected = [row for row in source["selected_ranges"]
                if row["path"] == "src/EnergyPlus/SurfaceGeometry.cc" and row["start"] == 273 and row["end"] == 289]
    require(len(selected) == 1 and exact(fragment["source_range"], selected[0]),
            "Driver setup fragment is not the frozen exact source range")
    define = re.search(r"(?:^|\s)-DGEO02_SETUP_FRAGMENT=(\S+)", helper_rows[0]["command"])
    require(define is not None, "Helper compile row omits actual source fragment define")
    define_path = define.group(1).replace(r'\"', '"').strip('"')
    require(path_of(define_path) == bindings.check(fragment["artifact"]),
            "Helper compiled a different initialization fragment")
    require(fragment["artifact"]["sha256"] == selected[0]["range_sha256"]
            and fragment["whole_SetupZoneGeometry_claimed"] is False,
            "Source initialization fragment was rewritten or overclaimed")
    return read(result_path), execution


def verify_rust(execution, request_ref, original_first_ref, original_execution, baseline, bindings):
    require(execution["exit_code"] == 0 and path_of(execution["cwd"]) == ROOT
            and execution["reference_outputs_supplied"] is False, "Rust input-only execution failed/boundary differs")
    require(same_binding(execution["input_request"], request_ref)
            and same_binding(execution["original_first_receipt_ref"], original_first_ref),
            "Rust execution is not bound to same frozen input and original-first proof")
    command_paths(execution["command"], [execution["binary"]["path"], request_ref["path"]])
    bindings.check(execution["input_request"])
    bindings.check(execution["binary"])
    bindings.check(execution["executed_launcher"])
    bindings.check(execution["stdout"])
    bindings.check(execution["stderr"])
    require(timestamp(original_execution["finished_utc"]) <= timestamp(execution["started_utc"]),
            "Rust numerical execution began before genuine original execution completed")
    end = execution.get("ended_utc", execution.get("finished_utc"))
    require(timestamp(execution["started_utc"]) <= timestamp(end), "Invalid Rust execution chronology")
    build = read(bindings.check(execution["build"]))
    require(build["schema"] == "geo02-Rust-build.v1" and build["kind"] == "example"
            and same_binding(build["binary"], execution["binary"]), "Wrong compiled Rust helper/binary")
    for key in ("binary", "build_execution", "source_snapshot", "cargo_lock", "toolchain", "executed_launcher"):
        bindings.check(build[key])
    compilation = read(bindings.check(build["build_execution"]))
    require(compilation["exit_code"] == 0
            and compilation["command"] == ["cargo", "build", "-p", "ep_runtime", "--example", "geo02_tuples", "-j", "2"]
            and compilation["source_bytes_match_before_and_after"] is True,
            "Rust build failed or executed different/mutating inputs")
    for key in ("stdout", "stderr"):
        bindings.check(compilation[key])
    require(same_binding(compilation["source_snapshot"], build["source_snapshot"])
            and compilation["repository_before"]["head"] == compilation["repository_after"]["head"] == build["repository_head"],
            "Rust build snapshot/revision changed during execution")
    source_snapshot = read(bindings.check(build["source_snapshot"]))
    files = source_snapshot["files"]
    for key, historical_path in (("cargo_lock", "Cargo.lock"), ("toolchain", "rust-toolchain.toml")):
        rows = [row for row in files if row["historical_path"] == historical_path]
        require(len(rows) == 1, "Missing actual build-input snapshot: " + historical_path)
        bindings.check(rows[0]["archive"])
        require(build[key]["sha256"] == rows[0]["archive"]["sha256"],
                "Archived build input differs from actual compiled snapshot: " + historical_path)
    rust_files = {row["historical_path"]: row["archive"] for row in files
                  if row["historical_path"].endswith(".rs")}
    require(len(rust_files) == sum(row["historical_path"].endswith(".rs") for row in files),
            "Duplicate Rust source snapshot identity")
    actual_sources = {}
    for row in build["crates_sources"]:
        require(row["historical_path"] not in actual_sources, "Duplicate compiled Rust source")
        bindings.check(row)
        actual_sources[row["historical_path"]] = row
    require(set(actual_sources) == set(rust_files), "Compiled Rust source cardinality differs")
    for historical_path, archived in rust_files.items():
        require(same_binding(archived, actual_sources[historical_path]), "Compiled Rust source archive differs")
        bindings.check(archived)
    for required in ("crates/ep_runtime/examples/geo02_tuples.rs", "crates/ep_runtime/src/geometry.rs"):
        require(required in actual_sources, "Required actually executed Rust source missing")
    require(type(build["source_worktree_clean"]) is bool, "Missing precise source cleanliness field")
    if not baseline:
        require(build["source_worktree_clean"] is True
                and build["implementation_commit"] == build["repository_head"],
                "Final canonical unit proof requires committed compiled source")
        precision = build["compiled_source_vs_committed_blobs"]
        differences = precision["line_ending_only_differences"]
        require(precision["all_other_content_exact"] is True
                and precision["source_count"] == len(actual_sources)
                and type(precision["exact_byte_matches"]) is int
                and precision["exact_byte_matches"] + len(differences) == len(actual_sources),
                "Committed/compiled source precision counts differ")
        seen = set()
        for row in differences:
            historical_path = row["historical_path"]
            require(historical_path in actual_sources and historical_path not in seen
                    and same_binding(row["compiled_archive"], actual_sources[historical_path]),
                    "Committed source difference archive binding differs")
            seen.add(historical_path)
            content = bindings.check(row["compiled_archive"]).read_bytes().replace(b"\r\n", b"\n")
            require(hashlib.sha256(content).hexdigest() == row["git_blob_sha256"]
                    and hashlib.sha1(b"blob " + str(len(content)).encode() + b"\0" + content).hexdigest() == row["git_blob"],
                    "Claimed committed LF-normalized source bytes differ")
    return read(bindings.check(execution["stdout"])), build


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--original-first", required=True, help="Preserved original-first execution receipt")
    parser.add_argument("--native-output", required=True, help="Original helper JSON bound by that receipt")
    parser.add_argument("--rust-execution", required=True, help="Preserved input-only Rust execution receipt")
    parser.add_argument("--baseline", action="store_true", help="Compare existing outputs; missing targets explicitly fail")
    args = parser.parse_args()
    bindings = Bindings()
    original_first_ref = ref(path_of(args.original_first))
    first = read(bindings.check(original_first_ref))
    native_ref = ref(path_of(args.native_output))
    require(contains_binding(first, native_ref), "Original-first receipt does not bind selected native output")
    # --original-first is the actual geo02-original-helper-execution receipt.
    # A broader Root matrix may bind it independently without changing raw proof.
    contract_refs = {name: ref(CONTRACTS / f"GEO-02-{name}.json")
                     for name in ("source", "cases", "tolerances")}
    source, cases, tolerances = [read(bindings.check(contract_refs[name]))
                                 for name in ("source", "cases", "tolerances")]
    source_files = verify_sources(source)
    request_ref = cases["helper_request"]
    request = read(bindings.check(request_ref))
    verify_request(request, cases)
    frozen_profiles = profiles(tolerances)
    native, original_execution = verify_native(first, contract_refs, request_ref, bindings)
    require(same_binding(first["results"], native_ref), "Selected native output differs from actual helper execution")
    rust_execution_ref = ref(path_of(args.rust_execution))
    rust_execution = read(bindings.check(rust_execution_ref))
    rust, build = verify_rust(rust_execution, request_ref, original_first_ref, original_execution,
                              args.baseline, bindings)
    comparison, unpaired = compare_results(request, native, rust, frozen_profiles, args.baseline)
    result = comparison.report()
    status = "fail-incomplete" if args.baseline else ("pass" if result["mismatch_count"] == 0 else "fail")
    controls = sorted({row["floating_environment_before"]["x87_precision_bits"] for row in native["cen_calls"]})
    report = {
        "schema": "geo02-unit-comparison.v1", "status": status,
        "comparison_boundary": "existing_helpers_baseline" if args.baseline else "canonical_geometry_owner",
        "tool": ref(Path(__file__)), "provenance_tool": ref(Path(__file__).with_name("geo02_unit_provenance.py")),
        "contracts": contract_refs, "input_request": request_ref,
        "original_first": original_first_ref, "native_output": native_ref,
        "Rust_execution": rust_execution_ref, "Rust_binary": rust_execution["binary"],
        "Rust_build": rust_execution["build"], "source_worktree_clean": build["source_worktree_clean"],
        "implementation_commit": build.get("implementation_commit"),
        "paired_valid_quad_count": 15, "source_only_unsafe_quad_count": 2, "cen_input_count": 526,
        "actual_original_cen_x87_precision_bits": controls, "original_source_files_verified": source_files,
        "Rust_cen_precision_profile": rust.get("cen_precision_profile"),
        "source_only_unpaired": unpaired, **result, "checked_artifact_bindings": bindings.report(),
        "claim_limits": ["Helper units do not execute Rust physics or establish full parser admission.",
                         "Original counters, shape/scratch lifecycle and unsafe warning retention are source-only.",
                         "Cen exact bits follow unchanged original header and actual observed floating control.",
                         "No SRC04/SRC05, GEO03 or whole-engine numerical parity is claimed."],
        "reference_outputs_supplied_to_Rust": False, "gates_updated": False,
    }
    print(json.dumps(report, indent=2, allow_nan=False))
    return 0 if status == "pass" else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (ValueError, KeyError, OSError, TypeError) as error:
        print(json.dumps({"schema": "geo02-unit-check-error.v1", "status": "fail",
                          "error": str(error), "gates_updated": False}, indent=2))
        raise SystemExit(1)
