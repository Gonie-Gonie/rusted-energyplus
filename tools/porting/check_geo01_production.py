#!/usr/bin/env python3
"""Read retained GEO-01 production outputs; emit a comparison on stdout only.

No executable, Git, reference, geometry equation, output writer, or card updater
is invoked. Compile projections remain preparation evidence; only successful
ordinary physical runs supply the stored consumer fields.
"""
from __future__ import annotations

import argparse
from datetime import datetime
import hashlib
import json
import math
from pathlib import Path
import re
import sys

sys.dont_write_bytecode = True
import check_geo01_units as units

ROOT, CONTRACTS = units.ROOT, units.CONTRACTS
CASES = ["A-24H", "A-72H", "B-NOLIMIT-24H", "B-FLOW-24H",
         "B-CAPACITY-24H", "B-BOTH-24H", "B-BOTH-72H"]
FROZEN = {
    "source": "1c07297412d2408cf50905609fc6157a4dfb3488d833c9dacb15fa31e2275739",
    "cases": "b6c8189ffc86e973986610a5a48fd8edb72f9f3fb557a3f1e7ab432a3a5a0ff0",
    "tolerances": "87baa3978c3a26dd0d05b28850265cee96a3eb73dbf5aa82b3abfe8dec116bf2",
}
ORIGINAL_MATRIX_SHA = "f7e460dabd9eb9b2e3a5f8dfc15989d7fb8c8e9408f3c47bf400308c225764ab"
ORIGINAL_REVIEW_SHA = "968814926a26a7a60ab30972b119f784f4c9e1053e7cb221073fbb7de9bf3466"
require, read, bits, integer, same_binding = (
    units.require, units.read, units.bits, units.integer, units.same_binding)


class Bindings:
    """Hash each immutable file once, then check it again before returning."""

    def __init__(self):
        self.files = {}

    def verify(self, value):
        require(type(value) is dict and type(value.get("path")) is str
                and type(value.get("sha256")) is str
                and re.fullmatch(r"[0-9a-f]{64}", value["sha256"]), "Malformed SHA256 reference")
        lexical = Path(value["path"])
        require(not lexical.is_absolute() and not lexical.drive and ".." not in lexical.parts,
                "Reference must be repository relative: " + value["path"])
        path = (ROOT / lexical).resolve()
        require(path.is_relative_to(ROOT) and path.is_file(), "Missing or escaped file reference")
        if path not in self.files:
            self.files[path] = units.sha(path)
        require(self.files[path] == value["sha256"], "Changed reference: " + value["path"])
        return path

    def unchanged(self):
        for path, digest in self.files.items():
            require(units.sha(path) == digest, "File changed during read-only review: " + str(path))


def exact(left, right):
    """Exact JSON types and f64 bits, including signed zero, within one engine."""
    if type(left) is not type(right):
        return False
    if type(left) is float:
        return bits(left) == bits(right)
    if type(left) is dict:
        return left.keys() == right.keys() and all(exact(left[key], right[key]) for key in left)
    if type(left) is list:
        return len(left) == len(right) and all(exact(a, b) for a, b in zip(left, right))
    return left == right


class Comparison(units.Comparison):
    def __init__(self):
        super().__init__()
        self.connection_metrics = {}
        self.connections = []

    def invariant(self, left, right, location):
        self.checks += 1
        if not exact(left, right):
            self.fail("Full_Summary_invariance", location, left, right)

    def consumer(self, actual, original, value_key, bit_key, profile, profile_key, location):
        a, o = actual[value_key], original[value_key]
        ab, ob = actual[bit_key], original[bit_key]
        require(bits(a) == ab and bits(o) == ob, "Consumer payload/bits disagree: " + location)
        self.checks += 1
        require(math.isfinite(a) and math.isfinite(o), "Nonfinite stored consumer: " + location)
        delta = abs(a - o)
        metric = self.connection_metrics.setdefault(profile_key, {
            "count": 0, "maximum_absolute_error": 0.0,
            "maximum_relative_error_nonzero_reference": 0.0, "sum_squared_error": 0.0,
            "bit_equal_count": 0})
        metric["count"] += 1
        metric["bit_equal_count"] += int(ab == ob)
        metric["maximum_absolute_error"] = max(metric["maximum_absolute_error"], delta)
        metric["sum_squared_error"] += delta * delta
        if o != 0:
            metric["maximum_relative_error_nonzero_reference"] = max(
                metric["maximum_relative_error_nonzero_reference"], delta / abs(o))
        if delta > profile["absolute_tolerance"] + profile["relative_tolerance"] * abs(o):
            self.fail("stored_consumer_tolerance", location, a, o)
        self.connections.append({"location": location, "profile": profile_key,
                                 "rust": a, "original": o, "rust_bits": ab,
                                 "original_bits": ob, "absolute_error": delta})


def verify_build(matrix, bindings):
    build = read(bindings.verify(matrix["build"]))
    require(build["schema"] == "geo01-Rust-build.v1" and build["source_worktree_clean"] is True,
            "Production requires an archived, clean committed Rust build")
    commit = build["implementation_commit"]
    require(type(commit) is str and re.fullmatch(r"[0-9a-f]{40}", commit)
            and commit == build["repository_head"] == matrix.get("implementation_commit", commit),
            "Committed build identity differs")
    require(type(build["crates_tree"]) is str and re.fullmatch(r"[0-9a-f]{40}", build["crates_tree"]),
            "Missing crates tree identity")
    require(same_binding(matrix["binary"], build["binary"]), "Producer/build binary differs")
    bindings.verify(build["binary"])
    for key in ["cargo_lock", "toolchain", "executed_launcher"]:
        bindings.verify(build[key])
    require(type(build["rustc"]) is str and build["rustc"]
            and type(build["cargo"]) is str and build["cargo"], "Missing recorded toolchain versions")
    execution = read(bindings.verify(build["build_execution"]))
    require(execution["command"] == ["cargo", "build", "-p", "ep_cli", "--bin", "eplus-rs", "-j", "2"]
            and integer(execution["exit_code"], "build exit") == 0
            and units.path_of(execution["cwd"]) == ROOT
            and execution["repository_head"] == commit, "Actual build command/commit differs")
    for key in ["stdout", "stderr", "executed_launcher"]:
        bindings.verify(execution[key])
    require(exact(build["crates_sources"], execution["sources"]), "Archived build source list differs")
    owners = set()
    for source in build["crates_sources"]:
        bindings.verify(source)
        historical = source["historical_path"]
        require(type(historical) is str and historical.startswith("crates/")
                and historical not in owners and ".." not in Path(historical).parts,
                "Duplicate or invalid source archive owner")
        owners.add(historical)
    require({"crates/ep_compiler/src/compiler.rs", "crates/ep_run/src/geometry_trace.rs",
             "crates/ep_run/src/pipeline.rs", "crates/ep_runtime/src/heat_balance/initialization.rs"}
            <= owners, "Production/compiler observer source archive missing")
    precision = build["compiled_source_vs_committed_blobs"]
    count = integer(precision["source_count"], "compiled source count")
    exact_count = integer(precision["exact_byte_matches"], "exact committed byte matches")
    differences = precision["line_ending_only_differences"]
    require(count == len(owners) and 0 <= exact_count <= count and type(differences) is list
            and exact_count + len(differences) == count and precision["all_other_content_exact"] is True,
            "Missing/inconsistent compiled-versus-committed byte precision")
    source_by_owner = {source["historical_path"]: source for source in build["crates_sources"]}
    changed_owners = set()
    for difference in differences:
        owner = difference["historical_path"]
        require(owner in source_by_owner and owner not in changed_owners
                and same_binding(difference["compiled_archive"], source_by_owner[owner]),
                "Line-ending difference not bound to actual compiled source")
        changed_owners.add(owner)
        actual = bindings.verify(difference["compiled_archive"]).read_bytes()
        normalized = actual.replace(b"\r\n", b"\n")
        require(actual != normalized and type(difference["git_blob_sha256"]) is str
                and hashlib.sha256(normalized).hexdigest() == difference["git_blob_sha256"],
                "Declared committed-body SHA256 differs from actual LF-normalized compiled bytes")
        require(type(difference["git_blob"]) is str and re.fullmatch(r"[0-9a-f]{40}", difference["git_blob"])
                and hashlib.sha1(b"blob " + str(len(normalized)).encode("ascii") + b"\0" + normalized).hexdigest()
                    == difference["git_blob"], "Declared Git blob identity differs from normalized source body")
    return build


def frozen_original(matrix, bindings):
    contracts = {}
    for key, digest in FROZEN.items():
        value = units.ref(CONTRACTS / f"GEO-01-{key}.json")
        require(value["sha256"] == digest and same_binding(matrix["contracts"][key], value),
                "Frozen contract/producer binding changed: " + key)
        contracts[key] = read(bindings.verify(value))
    require(contracts["source"]["energyplus_commit"] == units.PIN
            and contracts["cases"]["case_count"] == 37
            and contracts["cases"]["production_case_count"] == 15
            and contracts["tolerances"]["frozen_before_comparison"] is True, "Wrong frozen domain")
    require(matrix["original_first_matrix"]["sha256"] == ORIGINAL_MATRIX_SHA
            and matrix["original_first_review"]["sha256"] == ORIGINAL_REVIEW_SHA,
            "Accepted original-first proof changed")
    original = read(bindings.verify(matrix["original_first_matrix"]))
    review = read(bindings.verify(matrix["original_first_review"]))
    require(original["schema"] == "geo01-original-matrix.v1"
            and original["case_count"] == 37 and original["rust_compared"] is False,
            "Incomplete original-first matrix")
    require(review["schema"] == "geo01-original-preparation-review.v1"
            and review["status"] == "pass" and review["energyplus_commit"] == units.PIN
            and same_binding(review["original_matrix"], matrix["original_first_matrix"]),
            "Original-first review crossbinding failed")
    bindings.verify(review["executed_checker"])
    require(matrix["original_first_completed_before_Rust"] is True
            and type(matrix["original_first_order_basis"]) is str
            and matrix["original_first_order_basis"].startswith("Root accepted the completed native matrix"),
            "Missing explicit original-first producer order assertion")
    originals = {row["case_id"]: row for row in original["cases"]}
    require(len(originals) == len(original["cases"]) == 37, "Duplicate/incomplete original cases")
    return contracts, originals


def verify_run(case, row, matrix, bindings, used_directories):
    level = row["trace_level"]
    relative = Path(row["output_directory"])
    require(not relative.is_absolute() and not relative.drive and ".." not in relative.parts,
            "Output directory must be repository relative")
    output = (ROOT / relative).resolve()
    require(output.is_relative_to(ROOT) and output.is_dir() and output not in used_directories,
            "Missing, reused or escaped output directory")
    used_directories.add(output)
    execution = read(bindings.verify(row["execution"]))
    require(integer(execution["exit_code"], "CLI exit") == 0 and execution["dry_run"] is False
            and execution["trace_level"] == level and execution["original_outputs_supplied_to_Rust"] is False,
            "Prepared/failed/answer-fed command cannot prove production")
    require(units.path_of(execution["cwd"]) == ROOT, "CLI working directory differs")
    require(type(execution["repository_head"]) is str
            and re.fullmatch(r"[0-9a-f]{40}", execution["repository_head"]), "Missing execution revision")
    require(type(execution["elapsed_seconds"]) in (int, float)
            and math.isfinite(execution["elapsed_seconds"]) and execution["elapsed_seconds"] >= 0,
            "Invalid elapsed time")
    start, end = (datetime.fromisoformat(execution[key]) for key in ["started_utc", "finished_utc"])
    require(start.utcoffset() is not None and end.utcoffset() is not None
            and start.utcoffset().total_seconds() == end.utcoffset().total_seconds() == 0 and end >= start,
            "Missing/invalid recorded UTC execution interval")
    for key, expected in [("binary", matrix["binary"]), ("build", matrix["build"]),
                          ("executed_launcher", matrix["executed_launcher"]),
                          ("input", case["input"]), ("weather", case["weather"])]:
        require(same_binding(execution[key], expected), "Execution crossbinding differs: " + key)
        bindings.verify(execution[key])
    for key in ["stdout", "stderr"]:
        bindings.verify(execution[key])
    expected_command = [str(units.path_of(matrix["binary"]["path"])), "run", str(units.path_of(case["input"]["path"])),
                        "--weather", str(units.path_of(case["weather"]["path"])), "--output-dir", str(output),
                        "--mode", "compatibility", "--partial", "deny", "--trace-level", level,
                        "--porting-scope", case["scope"]]
    command = execution["command"]
    require(type(command) is list and len(command) == len(expected_command)
            and all(type(value) is str for value in command), "Malformed actual CLI argv")
    # The normal producer accepts a relative output root; only path operands
    # resolve against the already verified repository working directory.
    path_indices = {0, 2, 4, 6}
    require(all(units.path_of(command[index]) == units.path_of(value) if index in path_indices
                else command[index] == value for index, value in enumerate(expected_command)),
            "Command differs from ordinary bounded production invocation")
    artifacts = {}
    for value in row["artifacts"]:
        path = bindings.verify(value)
        require(path.is_relative_to(output) and path not in artifacts, "Duplicate/foreign output artifact")
        artifacts[path] = value
    require(set(artifacts) == {path.resolve() for path in output.rglob("*") if path.is_file()},
            "Producer artifact hashes omit/add output files")
    summary_path = bindings.verify(row["run_summary"])
    require(summary_path == output / "run-summary.json" and same_binding(row["run_summary"], artifacts[summary_path]),
            "Producer summary is not its actual output artifact")
    summary = read(summary_path)
    require(summary["status"] == "success" and integer(summary["exit_code"], "summary exit") == 0
            and summary["message"] == "arbitrary run completed", "No successful ordinary physical run")
    config = summary["config"]
    require(config["mode"] == "compatibility" and config["partial_policy"] == "deny"
            and config["trace_level"] == level and config["dry_run"] is False
            and config["oracle_baseline"] is False and config["compare_oracle"] is False
            and config["hours"] is None and config["output_format"] == "rust-native",
            "Unexpected runtime/oracle/partial/duration configuration")
    require(summary["oracle"] is None and summary["comparison"] is None
            and summary["oracle_status"] == summary["compare_status"] == "not-requested", "Original answers supplied in run")
    require(summary["support"]["status"] == "supported-compatibility"
            and summary["support"]["conformance_claim"] is False
            and summary["support"]["run_result_state"] == "supported_compatibility_run",
            "Unsupported or promoted runtime")
    runtime = summary["rust_runtime"]
    metadata = read(bindings.verify(case["metadata"]))
    steps = integer(metadata["expected_zone_steps_excluding_warmup"], "frozen physical steps")
    require(steps in [96, 288] and metadata["id"] == case["id"] and metadata["scope"] == case["scope"]
            and same_binding(metadata["input"], case["input"]) and same_binding(metadata["weather"], case["weather"]),
            "Wrong original CON duration/input metadata")
    require(type(runtime) is dict and integer(runtime["samples"], "physical samples") == steps // 4,
            "Physical runtime absent or duration differs")
    runtime_class = "one-zone-heat-balance-compatibility" if case["scope"] == "A" else "ideal-loads-direct-zone-coupled-compatibility"
    require(runtime["runtime_class"] == summary["support"]["runtime_class"] == runtime_class,
            "Different production consumer route")
    if case["scope"] == "B":
        require(runtime["fixture_demand_injection_used"] is False
                and integer(runtime["purchased_air_coupling_call_count"], "actual B coupling calls") == steps,
                "Fixture demand or missing actual coupling")
    require(summary["source_order_gate"]["matches"] is True
            and summary["selected_algorithm_lane"]["id"] == "compatibility-source-order"
            and summary["selected_algorithm_lane"]["diagnostic_probe_used"] is False,
            "Different actual runtime lane")
    scope = read(output / "porting_scope.json")
    require(scope["schema"] == "porting-scope.v1" and scope["scope"] == case["scope"]
            and scope["admissible"] is True and scope["violations"] == []
            and all(scope["production"][key] is False for key in ["fixture_inputs_used", "oracle_inputs_used", "conformance_claim"]),
            "Actual bounded admission differs")
    for key, expected in [("original", output / "input/original.idf"), ("converted_epjson", output / "input/converted.epJSON"),
                          ("weather", units.path_of(case["weather"]["path"]))]:
        require(units.path_of(summary["input"][key]) == expected, "Summary input path crossbinding differs")
    require(summary["input"]["kind"] == "idf", "Different ordinary input route")
    input_receipt = read(output / "input/input-hashes.json")
    require(input_receipt["algorithm"] == "fnv-1a-64", "Wrong actual input hash algorithm")
    for key, expected in [("source", units.path_of(case["input"]["path"])),
                          ("staged_original", output / "input/original.idf"),
                          ("converted_epjson", output / "input/converted.epJSON")]:
        value = input_receipt[key]
        require(units.path_of(value["path"]) == expected, "Input receipt file path differs")
        content = expected.read_bytes()
        require(integer(value["bytes"], "input byte count") == len(content)
                and value["hash"] == units.fnv1a(content), "Actual staged/input receipt hash differs")
    require(units.sha(output / "input/original.idf") == case["input"]["sha256"],
            "Actual physical staged IDF differs from original frozen input")
    if level == "full":
        require(all((output / filename) in artifacts for filename in ["compiled-geometry.json", "geometry-consumers.json", "clock-calls.json"]),
                "Full physical geometry/clock artifacts absent")
        clock = read(output / "clock-calls.json")
        require(clock["schema"] == "clk01-clock-trace.v1" and clock["complete_on_collecting_thread"] is True
                and clock["capture_source"] == "existing-physical-zone-loop-hook"
                and clock["thread_coverage"] == "collecting-thread-only"
                and integer(clock["omitted_invocation_count"], "omitted clock events") == 0
                and clock["truncation_reason"] is None and clock["prepared_rows_are_executed_events"] is False
                and integer(clock["total_invocation_count"], "actual clock events") == steps
                and integer(clock["recorded_invocation_count"], "retained clock events") == steps
                and len(clock["zone_invocations"]) == steps, "Missing/incomplete actual physical interval observations")
        is_b = case["scope"] == "B"
        require(len(clock["prepared_hourly_frames"]) == steps // 4
                and len(clock["prepared_environment_points"]) == (steps if is_b else 0)
                and clock["source_environment_number"] is None
                and exact(clock["materialized_environment_index"], 1 if is_b else None),
                "Prepared axes differ from actual bounded Rust physical route")
        for index, event in enumerate(clock["zone_invocations"]):
            expected = {"sequence": index + 1, "hour_index": index // 4,
                        "calendar_frame_index": index // 4, "zone_timestep": index % 4 + 1,
                        "zone_steps_per_hour": 4, "timestep_seconds_bits": bits(900.0),
                        "environment_point_index": index if is_b else None}
            require(all(exact(event[key], value) for key, value in expected.items()),
                    "Actual retained physical hook order/operands differ")
    else:
        require(not any((output / filename).exists() for filename in ["compiled-geometry.json", "geometry-consumers.json",
                    "clock-calls.json", "psy02-calls.json", "psychrometrics-calls.json"]), "Summary unexpectedly enabled Full observers")
    return output, summary, steps


def compare_consumers(case, original_row, output, cmp, profiles):
    native = read(units.binding(original_row["results"]))
    source = native["final_weather"]
    compiled = read(output / "compiled-geometry.json")
    observed = read(output / "geometry-consumers.json")
    require(observed["schema"] == "geo01-runtime-consumers.v1" and observed["phase"] == "runtime_final_state_geometry"
            and observed["physics_executed"] is True and observed["observer_supplies_inputs"] is False
            and observed["compiled_geometry_artifact"] == "compiled-geometry.json"
            and integer(observed["compiled_surface_count"], "compiled surface count") == 6
            and integer(observed["compiled_zone_count"], "compiled zone count") == 1
            and integer(observed["observed_timestep_index"], "final stored timestep index") > 0,
            "Copied final physical state boundary/count differs")
    ns, cs, rs = [units.named_rows(rows) for rows in [source["surfaces"], compiled["surfaces"], observed["surfaces"]]]
    nz, cz, rz = [units.named_rows(rows) for rows in [source["zones"], compiled["zones"], observed["zones"]]]
    require(set(ns) == set(cs) == set(rs) and len(rs) == 6 and set(nz) == set(cz) == set(rz) and len(rz) == 1,
            "Final-state named topology differs from actual compile/native storage")
    zone_names = {row["id"]: key for key, row in cz.items()}
    for field in ["surfaces", "zones"]:
        for order, row in enumerate(observed[field], 1):
            require(integer(row["runtime_iteration_order"], field + " storage order") == order,
                    "Runtime storage order metadata differs")
    for key, row in rs.items():
        require(integer(row["id"], "runtime surface ID") == cs[key]["id"]
                and integer(row["zone_id"], "runtime surface zone ID") == cs[key]["zone_id"]
                and zone_names[row["zone_id"]] == units.name(ns[key]["zone_name"]),
                "Runtime/compiler own-ID surface/zone binding differs")
        nc = ns[key]["native_consumer"]
        require(nc["vertices_processed"] is True and nc["is_degenerate"] is False and nc["heat_transfer_surface"] is True,
                "Different original consumer surface branch")
        for value_key, bit_key, profile_key in [("area_m2", "area_bits", "connection_area_m2"),
                ("azimuth_deg", "azimuth_bits", "connection_azimuth_deg"), ("tilt_deg", "tilt_bits", "connection_tilt_deg")]:
            cmp.consumer(row["rust_consumer"], nc, value_key, bit_key, profiles[profile_key], profile_key,
                         case["id"] + "." + key + "." + value_key)
    volume_policy = []
    parsed_zones = {units.name(key): value for key, value in compiled["parsed_input"]["objects"]["Zone"].items()}
    for key, row in rz.items():
        require(integer(row["id"], "runtime zone ID") == cz[key]["id"], "Runtime/compiler own-ID zone binding differs")
        declaration = parsed_zones[key]["volume"]
        if case["scope"] == "A":
            require(type(declaration) is str and declaration.casefold() == "autocalculate", "A volume input is not Autocalculate")
            policy = "existing runtime Autocalculate; actual stored volume paired only"
        else:
            require(type(declaration) in (int, float) and math.isfinite(declaration) and declaration > 0,
                    "B volume input is not a positive declared number")
            cmp.exact(row["rust_consumer"]["volume_bits"], bits(declaration), case["id"] + ".declared_volume_consumed")
            policy = "declared numeric input; actual stored volume, not geometry-derived volume"
        cmp.consumer(row["rust_consumer"], nz[key]["native_consumer"], "volume_m3", "volume_bits",
                     profiles["connection_zone_volume_m3"], "connection_zone_volume_m3", case["id"] + "." + key + ".volume_m3")
        volume_policy.append({"zone_name": key, "input_declaration": declaration, "policy": policy})
    return {"artifact": units.ref(output / "geometry-consumers.json"), "native_snapshot": "final_weather",
            "stored_consumer_scalar_count": 19, "projection_count": 1,
            "projection_interpretation": "one copied final-state geometry projection; scalar count is not kernel invocation count",
            "own_engine_ids_bound_by_name": True,
            "observed_timestep_index": observed["observed_timestep_index"], "volume_policy": volume_policy}


def invariance(case_id, full, summary, full_s, summary_s, cmp):
    cmp.invariant(sorted(full_s), sorted(summary_s), case_id + ".RunSummary.keys")
    excluded = {"artifacts", "timing", "input", "config"}
    for key in (full_s.keys() & summary_s.keys()) - excluded:
        cmp.invariant(full_s[key], summary_s[key], case_id + ".RunSummary." + key)
    cmp.invariant({key: value for key, value in full_s["config"].items() if key != "trace_level"},
                  {key: value for key, value in summary_s["config"].items() if key != "trace_level"}, case_id + ".RunSummary.config")
    for filename in ["results/selected-outputs.csv", "results/meters.csv"]:
        cmp.invariant(units.sha(full / filename), units.sha(summary / filename), case_id + "." + filename)
    cmp.invariant(read(full / "results/result-store.json"), read(summary / "results/result-store.json"), case_id + ".result-store")
    cmp.invariant(read(full / "porting_scope.json"), read(summary / "porting_scope.json"), case_id + ".bounded_admission")
    full_receipt, summary_receipt = (read(path / "input/input-hashes.json") for path in [full, summary])
    cmp.invariant(full_receipt["source"], summary_receipt["source"], case_id + ".same_input_source")
    for key in ["staged_original", "converted_epjson"]:
        cmp.invariant({field: full_receipt[key][field] for field in ["bytes", "hash"]},
                      {field: summary_receipt[key][field] for field in ["bytes", "hash"]}, case_id + ".same_staged_input." + key)
        cmp.invariant(units.sha(units.path_of(full_receipt[key]["path"])), units.sha(units.path_of(summary_receipt[key]["path"])),
                      case_id + ".staged_bytes." + key)
    return {"summary_observer_artifacts_absent": True, "comparison": "exact JSON types/f64 bits and byte hashes",
            "RunSummary_compared": "all fields except artifacts/timing/input paths; config excludes intentional trace_level",
            "input_paths_checked_separately": True, "selected_outputs_csv_meters_csv_result_store_compared": True}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--matrix", type=Path, required=True, help="Preserved geo01-rust-cli-matrix.v1 producer JSON")
    parser.add_argument("--case", action="append", choices=CASES, default=[], help="Review a subset; coverage is explicitly reported")
    args = parser.parse_args()
    matrix_path = args.matrix.resolve()
    require(matrix_path.is_relative_to(ROOT), "Producer matrix outside repository")
    bindings = Bindings()
    matrix_ref = units.ref(matrix_path)
    matrix = read(bindings.verify(matrix_ref))
    require(matrix["schema"] == "geo01-rust-cli-matrix.v1" and matrix["kind"] == "production"
            and matrix["original_outputs_supplied_to_Rust"] is False, "Wrong producer kind/input boundary")
    bindings.verify(matrix["executed_launcher"])
    contracts, originals = frozen_original(matrix, bindings)
    build = verify_build(matrix, bindings)
    rows = {row["case_id"]: row for row in matrix["cases"]}
    require(len(rows) == len(matrix["cases"]) and set(rows) <= set(CASES), "Duplicate/out-of-domain production cases")
    selected = args.case or CASES
    require(len(selected) == len(set(selected)) and set(selected) <= set(rows), "Missing/duplicate requested production cases")
    if not args.case:
        require(set(rows) == set(CASES), "Final production review requires all seven fixed cases")
    frozen = {case["id"]: case for case in contracts["cases"]["cases"]}
    cmp, summaries, directories = Comparison(), [], set()
    for case_id in selected:
        before = sum(cmp.mismatches.values())
        case, row = frozen[case_id], rows[case_id]
        require(case["kind"] == "con-production-input" and row["scope"] == case["scope"], "Expanded/changed production scope")
        for key in ["input", "weather", "metadata"]:
            require(same_binding(row[key], case[key]), "Producer/frozen case binding differs: " + key)
            bindings.verify(row[key])
        runs = {run["trace_level"]: run for run in row["runs"]}
        require(set(runs) == {"full", "summary"} and len(row["runs"]) == 2, "Need exactly one actual Full/Summary pair")
        full, full_s, steps = verify_run(case, runs["full"], matrix, bindings, directories)
        summary, summary_s, summary_steps = verify_run(case, runs["summary"], matrix, bindings, directories)
        require(steps == summary_steps, "Full/Summary physical duration differs")
        projection = units.compare_case(case, originals[case_id], full / "compiled-geometry.json", cmp,
                                        contracts["tolerances"]["profiles"]["world_vertices_m"])
        native = read(units.binding(originals[case_id]["results"]))
        require(integer(native["physical_zone_callback_count"], "native physical interval count") == steps, "Native CON duration differs")
        consumers = compare_consumers(case, originals[case_id], full, cmp, contracts["tolerances"]["profiles"])
        equality = invariance(case_id, full, summary, full_s, summary_s, cmp)
        summaries.append({"case_id": case_id, "scope": case["scope"], "full_execution": runs["full"]["execution"],
                          "summary_execution": runs["summary"]["execution"], "physical_zone_steps": steps,
                          "compile_preparation": projection, "physical_connection": consumers,
                          "Full_Summary": equality, "mismatch_count": sum(cmp.mismatches.values()) - before})
    bindings.unchanged()
    metrics = {}
    for key, metric in cmp.connection_metrics.items():
        metrics[key] = {field: value for field, value in metric.items() if field != "sum_squared_error"}
        metrics[key]["rmse"] = math.sqrt(metric["sum_squared_error"] / metric["count"])
    total = sum(cmp.mismatches.values())
    report = {"schema": "geo01-production-comparison.v1", "card": "GEO-01", "status": "pass" if total == 0 else "fail",
              "command": [sys.executable, *sys.argv], "checker": units.ref(Path(__file__)),
              "unit_helper": units.ref(Path(units.__file__)), "producer_matrix": matrix_ref,
              "build": matrix["build"], "binary": matrix["binary"], "implementation_commit": build["implementation_commit"],
              "crates_tree": build["crates_tree"], "contracts": matrix["contracts"],
              "compiled_source_vs_committed_blobs": build["compiled_source_vs_committed_blobs"],
              "original_first_matrix": matrix["original_first_matrix"], "original_first_review": matrix["original_first_review"],
              "original_first_order_basis": matrix["original_first_order_basis"],
              "original_first_order_interpretation": "producer/Root order assertion bound to accepted immutable native proofs; native receipts have no invented start timestamps",
              "case_count": len(summaries), "complete_seven_case_matrix": set(selected) == set(CASES), "cases": summaries,
              "checks": cmp.checks, "coordinate_count": cmp.coordinate_count, "stored_consumer_scalar_count": len(cmp.connections),
              "mismatch_count": total, "mismatch_kinds": dict(cmp.mismatches), "examples": cmp.examples,
              "connection_metrics": metrics, "stored_consumer_observations": cmp.connections,
              "original_outputs_supplied_to_Rust": False, "physics_executed": True,
              "compiled_projection_is_physics_evidence": False, "GEO02_GEO03_algorithms_certified": False,
              "whole_physics_certified": False, "gates_updated": False, "files_written": False,
              "limitations": ["selected CON stored-field connection only", "diagnostic units not production coverage",
                              "native-only geometry scratch counter/trig state has no Rust peer",
                              "no coordinate/area/azimuth/tilt/volume equation is recomputed by this checker",
                              "Full/Summary equality excludes intentional trace level, paths and elapsed timing",
                              "final internal timestep index is copied, not equated to clock counts; Rust warmup closure unclaimed"]}
    print(json.dumps(report, ensure_ascii=False, indent=2, allow_nan=False))
    return 0 if total == 0 else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (ValueError, OSError, KeyError, TypeError) as error:
        print(str(error), file=sys.stderr)
        raise SystemExit(2)
