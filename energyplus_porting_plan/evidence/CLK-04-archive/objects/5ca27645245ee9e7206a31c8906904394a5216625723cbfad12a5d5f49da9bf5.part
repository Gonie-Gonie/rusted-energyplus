#!/usr/bin/env python3
"""Read-only GEO-03 contract, artifact and execution provenance.

This module neither launches tools nor compares scientific outputs. Available
Rust-source inventories are evidence of archived bytes, not compiler selection.
Original helper diagnostics and ordinary input admission remain distinct.
"""
from __future__ import annotations

import argparse
from datetime import datetime
import hashlib
import json
from pathlib import Path
import re
import struct
import sys
import tomllib

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / ".reference/energyplus-src/26.1.0"
PIN = "6f2e40d10250a105b49966baa24d843711e61048"
SHA256 = re.compile(r"[0-9a-f]{64}\Z")
SHA1 = re.compile(r"[0-9a-f]{40}\Z")
FROZEN = {
    "source": "a17138b811096a9295ea4e87830100fc31be446771db198cce5e43d700330bd1",
    "cases": "ba21724f2d0cda4ffe92a382178d20e3618d293bf5457c1b1561967f6f561e97",
    "tolerances": "f636d6d59bf152b3a5ae8b8cdc6dc57b89f6825a410703ab05a24e2fb2ca592b",
}
REQUEST_SHA = "8b02d3d8a15ce8d782da5a5e6bae07a57d9e94c18a7f524aa45b2a4f533561d6"
PHYSICAL_CASES = {"A-24H", "A-72H", "B-BOTH-24H"}
NEGATIVE_CASES = {"GEO03-A-TOPOLOGY-DUPLICATE-WALL"}
NATIVE_SOURCE_NAMES = {
    "geo03_reference.cpp", "geo03_reference_helper.cpp", "geo03_reference_fields.hh",
    "geo03_reference.cmake", "geo03_reference_native.py", "geo02_reference_fields.hh",
    "geo02_reference.cmake", "geo01_reference.cmake", "psy02_reference.cmake",
    "psy02_reachability.cmake", "clk01_reference.cmake",
}
HELPER_PHASES = {
    "constructor": "genuine-original-constructor",
    "allocated_owner_defaults": "allocated-original-owner-defaults-before-input-writes",
    "geometry_prepared": "after-original-geometry-prerequisites",
    "declared_pre_volume_read_fields": "declared-input-only-prepared-read-fields",
    "before_volume": "after-exact-height-fragment-before-original-volume",
    "after_volume": "after-first-original-CalculateZoneVolume",
    "after_second_volume": "after-second-original-CalculateZoneVolume-without-height-fragment-repeat",
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def integer(value, label, minimum=0):
    require(type(value) is int and value >= minimum, "Invalid integer: " + label)
    return value


def exact(actual, expected):
    """Typed recursive metadata identity, including IEEE signed-zero bits."""
    if type(actual) is not type(expected):
        return False
    if type(actual) is float:
        return struct.pack(">d", actual) == struct.pack(">d", expected)
    if type(actual) is dict:
        return actual.keys() == expected.keys() and all(exact(actual[k], expected[k]) for k in actual)
    if type(actual) is list:
        return len(actual) == len(expected) and all(exact(a, b) for a, b in zip(actual, expected))
    return actual == expected


def read(path):
    def invalid(token):
        raise ValueError("Nonstandard JSON token: " + token)
    def object_pairs(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, "Duplicate JSON key: " + key)
            result[key] = value
        return result
    return json.loads(Path(path).read_text(encoding="utf-8"), parse_constant=invalid,
                      object_pairs_hook=object_pairs)


def path_of(value):
    require(type(value) is str and bool(value), "Expected a nonempty path")
    path = Path(value)
    path = (path if path.is_absolute() else ROOT / path).resolve()
    require(path.is_relative_to(ROOT), "Artifact path escapes repository: " + value)
    return path


def sha(path):
    with Path(path).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def ref(path):
    path = path_of(str(path))
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": sha(path)}


class Bindings:
    """Check immutable file bytes once and detect changes during inspection."""
    def __init__(self):
        self.checked = {}

    @staticmethod
    def stamp(path):
        s = path.stat()
        return s.st_size, s.st_mtime_ns, s.st_ctime_ns

    def check(self, value):
        require(type(value) is dict and type(value.get("path")) is str
                and type(value.get("sha256")) is str and SHA256.fullmatch(value["sha256"]),
                "Malformed SHA256 artifact binding")
        path = path_of(value["path"])
        require(path.is_file(), "Missing artifact: " + value["path"])
        stamp = self.stamp(path)
        if path not in self.checked:
            digest = sha(path)
            require(stamp == self.stamp(path), "File changed while hashing")
            self.checked[path] = (stamp, digest)
        previous, digest = self.checked[path]
        require(previous == stamp and digest == value["sha256"], "Changed artifact: " + value["path"])
        if "bytes" in value:
            require(type(value["bytes"]) is int and value["bytes"] == stamp[0], "Artifact size differs")
        return path

    def recursive(self, value):
        if type(value) is dict:
            if "path" in value and "sha256" in value:
                self.check(value)
            for child in value.values():
                self.recursive(child)
        elif type(value) is list:
            for child in value:
                self.recursive(child)

    def unchanged(self):
        for path, (stamp, _) in self.checked.items():
            require(path.is_file() and self.stamp(path) == stamp, "Artifact changed during inspection")

    def report(self):
        return [{"path": p.relative_to(ROOT).as_posix(), "sha256": item[1]}
                for p, item in sorted(self.checked.items())]


def same_binding(actual, expected):
    return (type(actual) is dict and type(expected) is dict
            and actual.get("sha256") == expected.get("sha256")
            and path_of(actual["path"]) == path_of(expected["path"]))


def identical_content(actual, expected, bindings):
    a, b = bindings.check(actual), bindings.check(expected)
    return actual["sha256"] == expected["sha256"] and a.read_bytes() == b.read_bytes()


def timestamp(value):
    require(type(value) is str, "Expected execution UTC timestamp")
    result = datetime.fromisoformat(value.replace("Z", "+00:00"))
    require(result.tzinfo is not None and result.utcoffset().total_seconds() == 0, "Timestamp must carry UTC")
    return result


def execution_interval(execution, bindings, exit_code=0):
    require(type(execution) is dict and type(execution.get("exit_code")) is int
            and (exit_code is None or execution["exit_code"] == exit_code)
            and path_of(execution["cwd"]) == ROOT, "Execution exit/cwd differs")
    require(type(execution["command"]) is list and bool(execution["command"])
            and all(type(x) is str and x for x in execution["command"]), "Malformed executed argv")
    for key in ("stdout", "stderr"):
        bindings.check(execution[key])
    start, finish = timestamp(execution["started_utc"]), timestamp(execution["finished_utc"])
    require(start <= finish, "Execution ended before it began")
    return start, finish


def verify_after_original(execution, original_completed):
    """Bind scientific execution ordering; building before a reference is separate."""
    require(isinstance(original_completed, datetime)
            and original_completed <= timestamp(execution["started_utc"]),
            "Rust scientific execution preceded completed original evidence")


def command_paths(actual, expected, path_indexes=None):
    indexes = set(range(len(expected))) if path_indexes is None else set(path_indexes)
    require(type(actual) is list and len(actual) == len(expected), "Executed argv length differs")
    for i, (a, b) in enumerate(zip(actual, expected)):
        require(path_of(a) == path_of(b) if i in indexes else exact(a, b),
                "Executed argv differs at index " + str(i))


def verify_contract_bindings(actual, expected, bindings):
    require(type(actual) is dict and actual.keys() == expected.keys(), "Contract binding set differs")
    for key, value in expected.items():
        require(same_binding(actual[key], value), "Contract binding differs: " + key)
        bindings.check(actual[key])


def frozen_contracts(declared, bindings):
    refs = {k: ref(ROOT / f"energyplus_porting_plan/contracts/GEO-03-{k}.json") for k in FROZEN}
    require(all(refs[k]["sha256"] == v for k, v in FROZEN.items()), "Frozen GEO-03 contract changed")
    if declared is not None:
        verify_contract_bindings(declared, refs, bindings)
    source, cases, tolerances = [read(bindings.check(refs[k])) for k in FROZEN]
    require(source["energyplus_commit"] == PIN and source["card"] == cases["card"] == tolerances["card"] == "GEO-03"
            and tolerances["frozen_before_numerical_comparison"] is True, "Wrong source or tolerance boundary")
    require(cases["helper_request"]["sha256"] == REQUEST_SHA
            and cases["helper_case_count"] == 19 and cases["paired_closed_boxes"] == 16
            and cases["source_only_diagnostics"] == 3 and cases["native_case_count"] == 12
            and cases["native_positive_case_count"] == 11 and cases["native_topology_diagnostic_case_count"] == 1
            and set(cases["Rust_physical_case_ids"]) == PHYSICAL_CASES
            and set(cases["Rust_physical_negative_case_ids"]) == NEGATIVE_CASES,
            "Frozen GEO-03 case coverage differs")
    bindings.check(cases["helper_request"])
    for row in cases["native_cases"]:
        for key in ("input", "weather", "metadata"):
            bindings.check(row[key])
    return refs, source, cases, tolerances


def verify_sources(source, bindings):
    require(source["energyplus_commit"] == PIN, "Wrong original source pin")
    config = tomllib.loads((ROOT / "config/default.toml").read_text(encoding="utf-8"))
    require(config["oracle"]["source_commit"] == PIN and config["oracle"]["energyplus_version"] == "26.1.0"
            and path_of(config["oracle"]["source_dir"]) == SOURCE, "Canonical reference pin differs")
    files = {}
    require(len(source["source_files"]) == 14 and len(source["selected_ranges"]) == 68, "Source closure count differs")
    for item in source["source_files"]:
        path = (SOURCE / item["path"]).resolve()
        require(path.is_relative_to(SOURCE) and item["path"] not in files, "Original source path/identity differs")
        files[item["path"]] = bindings.check({"path": path.relative_to(ROOT).as_posix(), "sha256": item["sha256"]}).read_bytes()
    seen = set()
    for row in source["selected_ranges"]:
        key = (row["path"], integer(row["start"], "source range start", 1), integer(row["end"], "source range end", 1))
        require(key not in seen and key[0] in files and key[1] <= key[2], "Source range identity differs")
        seen.add(key)
        lines = files[key[0]].splitlines(keepends=True)
        require(key[2] <= len(lines) and hashlib.sha256(b"".join(lines[key[1]-1:key[2]])).hexdigest() == row["range_sha256"],
                "Original source range bytes differ")


def archive_map(rows, bindings):
    require(type(rows) is list, "Missing source archive array")
    result = {}
    for row in rows:
        owner = row["historical_path"]
        path_of(owner)
        require(owner not in result, "Duplicate historical source path")
        bindings.check(row["archive"])
        result[owner] = row["archive"]
    return result


def _committed_precision(precision, rust, bindings):
    require(type(precision) is dict and precision["all_other_content_exact"] is True
            and integer(precision["source_count"], "available-source count") == len(rust), "Committed available-source audit differs")
    differences = precision["line_ending_only_differences"]
    require(type(differences) is list and integer(precision["exact_byte_matches"], "exact inventory count") + len(differences) == len(rust),
            "Committed available-source cardinality differs")
    seen = set()
    for row in differences:
        owner = row["historical_path"]
        require(owner in rust and owner not in seen and same_binding(row["archive"], rust[owner]), "Line-ending archive identity differs")
        seen.add(owner)
        body = bindings.check(row["archive"]).read_bytes()
        normalized = body.replace(b"\r\n", b"\n")
        require(body != normalized and SHA1.fullmatch(row["git_blob"])
                and hashlib.sha256(normalized).hexdigest() == row["git_blob_sha256"]
                and hashlib.sha1(b"blob " + str(len(normalized)).encode("ascii") + b"\0" + normalized).hexdigest() == row["git_blob"],
                "Committed line-ending-only body proof differs")


def verify_rust_build(build_ref, bindings, *, kind="cli", example=None, committed=False, required_sources=()):
    """Verify recorded Cargo target and archives without querying Git or Cargo.

Returns (receipt, available-source mapping, actual build command). The source
inventory is never described as a list of files compiled into the executable.
"""
    build = read(bindings.check(build_ref))
    require(build["schema"] == "porting-Rust-build.v1" and build["kind"] == kind
            and exact(build["example"], example) and build["gates_updated"] is False
            and build["reference_outputs_supplied"] is False, "Rust build target/boundary differs")
    require(type(build["source_worktree_clean"]) is bool
            and exact(build["committed_source_certification"], build["source_worktree_clean"]), "Rust clean-source certification differs")
    require(SHA1.fullmatch(build["repository_head"]), "Invalid Rust build commit")
    execution = read(bindings.check(build["build_execution"]))
    require(execution["schema"] == "recorded-porting-command.v1"
            and execution["launch_error"] is None and execution["source_bytes_match_before_and_after"] is True
            and execution["recorder_updates_gates"] is False and execution["recorder_supplies_reference_answers"] is False,
            "Recorded Cargo command boundary differs")
    _, finish = execution_interval(execution, bindings)
    if kind == "cli":
        require(example is None, "CLI build cannot name an example")
        package, target, target_kind, source = "ep_cli", "eplus-rs", "bin", "crates/ep_cli/src/main.rs"
        expected = ["cargo", "build", "-p", package, "--bin", target, "-j", "2", "--message-format=json-render-diagnostics"]
    else:
        require(kind == "example" and type(example) is str and example.isascii() and example.isidentifier(), "Invalid example target")
        package, target, target_kind, source = "ep_runtime", example, "example", f"crates/ep_runtime/examples/{example}.rs"
        expected = ["cargo", "build", "-p", package, "--example", target, "-j", "2", "--message-format=json-render-diagnostics"]
    require(exact(execution["command"], expected), "Actual Cargo build arguments differ")
    require(execution["repository_before"]["head"] == execution["repository_after"]["head"] == build["repository_head"], "Build revision changed")
    require(same_binding(execution["source_snapshot"], build["source_snapshot"]), "Build source snapshot differs")
    snapshot = read(bindings.check(build["source_snapshot"]))
    require(snapshot["schema"] == "rust-command-source-snapshot.v1" and snapshot["bytes_normalized"] is False,
            "Source snapshot normalizes bytes")
    available = archive_map(snapshot["files"], bindings)
    require(integer(build["available_source_inventory_count"], "available source inventory") == len(available)
            and exact(build["source_inventory_scope"], snapshot["scope"])
            and set(required_sources) <= set(available), "Available source inventory/required owners differ")
    for row in snapshot["files"]:
        require(integer(row["size_bytes"], "source archive bytes") == bindings.check(row["archive"]).stat().st_size,
                "Source inventory size differs")
    rust = {k: v for k, v in available.items() if k.endswith(".rs")}
    rows = build["available_Rust_sources"]
    require(type(rows) is list and len(rows) == len(rust), "Available Rust-source count differs")
    seen = set()
    for row in rows:
        owner = row["historical_path"]
        require(owner in rust and owner not in seen and same_binding(row, rust[owner]), "Available Rust-source binding differs")
        seen.add(owner)
        bindings.check(row)
    for key, owner in (("cargo_lock", "Cargo.lock"), ("toolchain", "rust-toolchain.toml")):
        require(owner in available and identical_content(build[key], available[owner], bindings), "Archived Cargo/toolchain source differs")
    capture = execution["cargo_compiler_artifacts_capture"]
    require(capture["requested"] is True and capture["status"] == "pass" and capture["error"] is None
            and finish <= timestamp(capture["captured_utc"]), "Cargo executable capture differs")
    candidates = []
    for row in capture["rows"]:
        message = row["cargo_message"]
        if (message["target"]["name"] == target and exact(message["target"]["kind"], [target_kind])
                and path_of(message["manifest_path"]) == ROOT / f"crates/{package}/Cargo.toml"
                and path_of(message["target"]["src_path"]) == ROOT / source):
            candidates.append(row)
    require(len(candidates) == 1, "Missing unique actual Cargo executable identity")
    artifact = build["actual_Cargo_executable"]
    require(exact({k: v for k, v in artifact.items() if k != "matching_archive"}, candidates[0])
            and same_binding(artifact["matching_archive"], build["binary"]), "Actual Cargo artifact/build binary binding differs")
    emitted = [json.loads(line) for line in bindings.check(execution["stdout"]).read_text(encoding="utf-8").splitlines()
               if line.strip().startswith("{")]
    require(artifact["cargo_message"]["reason"] == "compiler-artifact"
            and sum(exact(row, artifact["cargo_message"]) for row in emitted) == 1,
            "Selected Cargo executable was not uniquely emitted in preserved stdout")
    require(path_of(artifact["cargo_message"]["executable"]) == path_of(artifact["executable"]["historical_path"])
            and artifact["executable"]["sha256"] == build["binary"]["sha256"], "Historical Cargo executable identity differs")
    bindings.check(build["binary"])
    require(exact(build["actual_executed_recorder"], execution["executed_launcher"]), "Actually executed recorder differs")
    recorder = execution["executed_launcher"]
    require(recorder["historical_path"] == "tools/porting/record_command.py", "Unexpected recorder owner")
    bindings.check(recorder["archive"])
    for key in ("executed_freezer", "reader_dependency"):
        bindings.check(build[key])
    historical = build["historical_reader_dependency"]
    require(historical["historical_path"] == "tools/porting/record_command.py"
            and historical["sha256"] == build["reader_dependency"]["sha256"], "Freezer reader archive differs")
    if build["source_worktree_clean"]:
        require(build["implementation_commit"] == build["repository_head"] and SHA1.fullmatch(build["crates_tree"]), "Committed Rust build identity differs")
        _committed_precision(build["archived_source_vs_committed_blobs"], rust, bindings)
    else:
        require(not committed and build["implementation_commit"] is None and build["crates_tree"] is None
                and build["archived_source_vs_committed_blobs"] is None, "Dirty build claims committed scientific evidence")
    require(not committed or build["source_worktree_clean"] is True, "Final production requires committed source")
    return build, available, execution


def _define_path(command, name):
    tokens = re.findall(r"-D" + re.escape(name) + r'=(\\"[^\"]+\\"|"[^\"]+"|[^\s]+)', command)
    require(len(tokens) == 1, "Missing/duplicate original compile definition: " + name)
    return path_of(tokens[0].replace('\\"', '').strip('"'))


def verify_native_driver(receipt, refs, source, bindings):
    driver = read(bindings.check(receipt["native_driver_build"]))
    core_ref = source["references"]["native_core_build"]
    require(same_binding(receipt["native_core_build"], core_ref) and same_binding(driver["core_build"], core_ref), "Original core identity differs")
    core = read(bindings.check(core_ref))
    require(core["checks_passed"] is True and core["energyplus_commit"] == PIN
            and core["scientific_source_patches"] is False and core["assertions_enabled"] is True
            and core["extra_warning_waivers"] is False, "Unverified unchanged genuine original core")
    for key in ("core_library", "api_library"):
        bindings.check(core["artifacts"][key])
    bindings.check(core["compiler"]["binary"])
    require(driver["schema"] == "geo03-native-driver-build.v1" and driver["checks_passed"] is True
            and driver["energyplus_commit"] == PIN and same_binding(driver["source_contract"], refs["source"]), "Original driver identity differs")
    verify_contract_bindings(driver["contracts"], refs, bindings)
    for key in ("original_core_and_API_bytes_unchanged", "existing_target_compile_commands_unchanged", "same_compiler_owned_state",
                "original_directory_and_target_definitions_inherited", "assertions_enabled"):
        require(driver[key] is True, "Original build invariant failed: " + key)
    require(driver["scientific_source_patches"] is False and driver["fp_contract"] == "off", "Original build math policy differs")
    built = archive_map(driver["executed_source_archives"], bindings)
    require(set(built) == {"tools/porting/" + name for name in NATIVE_SOURCE_NAMES}, "Native source archive inventory differs")
    executed = archive_map(receipt["executed_source_archives"], bindings)
    require(set(executed) == set(built) and all(identical_content(executed[k], built[k], bindings) for k in built), "Built/executed wrapper source differs")
    for key in ("binary", "helper_binary", "cache", "compile_commands", "linked_container_library"):
        bindings.check(driver[key])
    started, completed = [], []
    for key in ("configure", "compile_link"):
        command = read(bindings.check(driver[key]))
        start, finish = execution_interval(command, bindings)
        started.append(start)
        completed.append(finish)
    rows = read(bindings.check(driver["actual_compile_commands"]))
    require(len(rows) == 2 and {Path(r["file"]).name for r in rows} == {"geo03_reference.cpp", "geo03_reference_helper.cpp"}, "Original compile target set differs")
    for row in rows:
        command = row["command"]
        for flag in ("-DEP_psych_errors", "-UNDEBUG", "-Werror", "-O0", "-ffp-contract=off", "-Wa,-mbig-obj"):
            require(re.search(r"(?:^|\s)" + re.escape(flag) + r"(?:\s|$)", command), "Missing native compile flag: " + flag)
        require(not any(flag in command for flag in ("-D_GLIBCXX_DEBUG", "-ffast-math", "-DEP_psych_stats"))
                and command.rfind("-UNDEBUG") > command.rfind("-DNDEBUG"), "Native ABI/assertion/FP policy differs")
        owner = "tools/porting/" + Path(row["file"]).name
        require(path_of(row["file"]) == path_of(owner), "Actual native translation unit differs")
    helper = next(r for r in rows if Path(r["file"]).name == "geo03_reference_helper.cpp")
    fragment_ranges = {"height_fragment": source["references"]["height_fragment"],
                       "trig_fragment": next(r for r in source["selected_ranges"] if r["path"] == "src/EnergyPlus/SurfaceGeometry.cc" and r["start"] == 273 and r["end"] == 289)}
    for key, selected in fragment_ranges.items():
        fragment = driver[key]
        require(exact(fragment["source_range"], selected) and fragment["artifact"]["sha256"] == selected["range_sha256"], "Native fragment is not the selected original range")
        require(_define_path(helper["command"], "GEO03_HEIGHT_FRAGMENT" if key == "height_fragment" else "GEO03_TRIG_FRAGMENT")
                == bindings.check(fragment["artifact"]), "Actual compiled original fragment path differs")
    previous = read(bindings.check(driver["previous_target_compile_commands"]))
    frozen = read(bindings.check(driver["frozen_original_build_commands"]))
    all_rows = read(bindings.check(driver["compile_commands"]))
    encode = lambda r: json.dumps(r, sort_keys=True, separators=(",", ":"))
    all_keys = {encode(r) for r in all_rows}
    require(integer(driver["frozen_original_build_command_count"], "native original compile count") == len(frozen) == 643
            and integer(driver["previous_card_compile_row_count"], "prior compile count") == len(previous) == 650
            and all(encode(r) in all_keys for r in previous + frozen), "Original/prior compile rows changed")
    require(len(driver["previous_card_products"]) == 7, "Prior native product inventory differs")
    bindings.recursive(driver["previous_card_products"])
    guard = driver["original_source_guard"]
    require(guard["source_changes"] == [], "Original archive source preservation failed")
    bindings.recursive(guard)
    review = read(bindings.check(receipt["independent_native_build_review"]))
    require(review["schema"] == "geo03-independent-native-build-final-review.v1"
            and review["status"] == "pass-build-identity-before-original-numerical-execution"
            and same_binding(review["reviewed_build"], receipt["native_driver_build"]), "Independent native build review differs")
    reviewed = timestamp(review["review_completed_utc"])
    require(max(completed) <= reviewed, "Independent build review predates successful compile/link")
    contract_review = read(bindings.check(driver["independent_contract_review"]))
    require(contract_review["schema"] == "geo03-independent-contract-review.v1"
            and contract_review["status"] == "pass-before-scientific-execution"
            and contract_review["scientific_execution_performed"] is False and contract_review["gates_updated"] is False,
            "Independent contract review boundary differs")
    for key, expected in refs.items():
        archived = contract_review["reviewed_contracts"][key]
        require(path_of(archived["historical_path"]) == path_of(expected["path"])
                and identical_content(archived["archive"], expected, bindings), "Reviewed contract archive differs")
    request = contract_review["reviewed_contracts"]["helper_request"]
    require(request["archive"]["sha256"] == REQUEST_SHA, "Independent reviewed request differs")
    require(path_of(request["historical_path"]) == ROOT / "energyplus_porting_plan/cases/GEO-03/helper-request.json",
            "Independent reviewed request historical owner differs")
    bindings.check(request["archive"])
    require(timestamp(contract_review["review_completed_utc"]) <= started[0], "Native configuration preceded contract review")
    return driver, core, reviewed


def verify_original_helper(receipt_ref, refs, source, cases, bindings):
    """Validate helper receipt/schema/identities only; no geometry comparison."""
    receipt = read(bindings.check(receipt_ref))
    require(receipt["schema"] == "geo03-original-helper-execution.v1"
            and receipt["original_parser_admission_claimed"] is False and receipt["physics_executed"] is False
            and receipt["Rust_compared"] is False and receipt["gates_updated"] is False, "Original helper claim boundary differs")
    verify_contract_bindings(receipt["contracts"], refs, bindings)
    require(same_binding(receipt["request"], cases["helper_request"]), "Original helper request differs")
    request = read(bindings.check(receipt["request"]))
    driver, _, built = verify_native_driver(receipt, refs, source, bindings)
    require(same_binding(receipt["binary"], driver["helper_binary"]), "Original helper executable differs")
    execution = read(bindings.check(receipt["execution"]))
    start, finish = execution_interval(execution, bindings)
    require(built <= start, "Original helper ran before successful build")
    command_paths(execution["command"], [receipt["binary"]["path"], receipt["request"]["path"]])
    result = read(bindings.check(receipt["results"]))
    require(exact(read(bindings.check(execution["stdout"])), result), "Helper results differ from real stdout")
    require(result["schema"] == "geo03-helper-results.v1" and result["expected_answers_supplied"] is False
            and result["original_parser_admission_claimed"] is False and result["physics_executed"] is False
            and result["gates_updated"] is False, "Original helper result scope differs")
    require(len(result["cases"]) == len(request["cases"]) == 19, "Original helper count differs")
    paired = 0
    for row, supplied in zip(result["cases"], request["cases"]):
        require(row["case_id"] == supplied["case_id"] and row["kind"] == supplied["kind"]
                and row["route"] == supplied["route"] and exact(row["input"], supplied), "Helper input/identity/order differs")
        require(row["status"] == ("source_complete" if supplied["kind"] == "closed_box" else "unsupported_source_only"), "Helper scope classification differs")
        states = row["source_state"]
        require(states.keys() == HELPER_PHASES.keys() and all(states[k]["phase"] == v for k, v in HELPER_PHASES.items()), "Original helper phase set differs")
        if supplied["kind"] == "closed_box":
            paired += 1
            require(row["kernel_input_identity_checked"] is True and row["input_vertex_identity_exact"] is True,
                    "Original helper input identity failed")
            auxiliary = row["auxiliary_original_helpers"]
            require(auxiliary["initial_edges_not_used_twice"] == [] and auxiliary["enclosed"] is True
                    and auxiliary["edges_not_used_twice"] == []
                    and auxiliary["method"] == "additional-original-helper-observations-not-internal-local-trace"
                    and auxiliary["unobservable_CalculateZoneVolume_local_method_reported"] is False,
                    "Original paired helper required healing or claims unobserved locals")
        require(row["source_only_diagnostics"]["source_local_CalcVolume_and_method_observed"] is False,
                "Original unobservable local was manufactured")
    require(paired == 16, "Original paired helper set differs")
    return result, execution, finish


def retained_identity(snapshot):
    return {k: snapshot[k] for k in ("zones", "spaces", "surfaces", "ordered_volume_base_faces")} | {
        "p0_m": snapshot["native_only_state"]["p0_m"]}


def verify_original_matrix(matrix_ref, refs, source, cases, bindings):
    """Validate ordinary original execution and retention, not Rust equality."""
    matrix = read(bindings.check(matrix_ref))
    require(matrix["schema"] == "geo03-original-matrix.v1" and matrix["complete"] is True
            and matrix["requested_subset_complete"] is True and integer(matrix["case_count"], "native count") == 12
            and matrix["Rust_compared"] is False and matrix["gates_updated"] is False, "Original full matrix is incomplete")
    expected = {x["id"]: x for x in cases["native_cases"]}
    require(len(expected) == len(matrix["cases"]) == 12 and {x["case_id"] for x in matrix["cases"]} == set(expected), "Original matrix case set differs")
    outputs, completed, shared = {}, [], None
    for row in matrix["cases"]:
        case = expected[row["case_id"]]
        receipt = read(bindings.check(row["receipt"]))
        verify_contract_bindings(receipt["contracts"], refs, bindings)
        require(receipt["schema"] == "geo03-original-execution.v1" and receipt["case_id"] == case["id"]
                and receipt["kind"] == case["kind"] == row["kind"] and receipt["Rust_compared"] is False
                and receipt["gates_updated"] is False and receipt["physical_input_scope_expanded"] is False,
                "Original ordinary execution scope differs")
        driver, core, built = verify_native_driver(receipt, refs, source, bindings)
        identity = (receipt["native_core_build"], receipt["native_driver_build"], receipt["independent_native_build_review"])
        require(shared is None or exact(shared, identity), "Original matrix uses differing builds/reviews")
        shared = identity
        result = read(bindings.check(row["results"]))
        require(result["schema"] == "geo03-original-volume.v1" and result["case_id"] == case["id"]
                and same_binding(receipt["results"], row["results"])
                and same_binding(receipt["binary"], driver["binary"])
                and same_binding(result["library"], core["artifacts"]["api_library"])
                and same_binding(result["native_core_build"], receipt["native_core_build"])
                and same_binding(result["idd"], receipt["idd"]), "Original output/build bindings differ")
        for key in ("input", "weather"):
            require(same_binding(receipt[key], case[key]) and same_binding(result[key], case[key]), "Original input bytes differ")
            bindings.check(case[key])
        bindings.check(receipt["idd"])
        if receipt["source_error_log"] is not None:
            bindings.check(receipt["source_error_log"])
        execution = read(bindings.check(receipt["execution"]))
        start, finish = execution_interval(execution, bindings)
        require(built <= start, "Original ordinary input ran before successful build")
        command_paths(execution["command"], [receipt["binary"]["path"], str(ROOT), receipt["native_core_build"]["path"], case["id"],
                      case["input"]["path"], case["weather"]["path"], receipt["idd"]["path"], str(bindings.check(row["results"]).parent)], {0, 1, 2, 4, 5, 6, 7})
        command_paths(result["command"], ["energyplus", "-i", receipt["idd"]["path"], "-w", case["weather"]["path"],
                      "-d", str(bindings.check(row["results"]).parent), case["input"]["path"]], {2, 4, 6, 7})
        require(result["callback_error"] == "" and type(result["energyplus_exit_code"]) is int
                and type(result["volume_probe_calls_added"]) is int and result["volume_probe_calls_added"] == 0
                and result["state_reset_requested"] is False and result["simulation_inputs_modified"] is False
                and result["gates_updated"] is False, "Original observer changed execution")
        integer(result["physical_zone_callback_count"], "original physical callbacks")
        integer(result["warmup_zone_callback_count"], "original warmup callbacks")
        if case["id"] in NEGATIVE_CASES:
            require(case["scope"] is None and case["native_outcome_policy"] == receipt["native_outcome_policy"] == "observe-only-no-predetermined-exit",
                    "Original topology diagnostic outcome was prescribed")
            for phase in ("first_initialized", "first_physical", "final_weather", "final"):
                require(result[phase] is None or type(result[phase]) is dict, "Original diagnostic phase availability malformed")
        else:
            require(receipt["native_outcome_policy"] == "admitted-valid-success-and-retention"
                    and result["energyplus_exit_code"] == 0 and result["first_final_zone_space_surface_identity_exact"] is True
                    and result["physical_zone_callback_count"] == {"24H": 96, "72H": 288}[case["duration"]], "Original positive lifecycle incomplete")
            initial = result["first_initialized"]
            for phase in ("first_initialized", "first_physical", "final_weather", "final"):
                value = result[phase]
                require(type(value) is dict and len(value["zones"]) == len(value["spaces"]) == 1 and len(value["surfaces"]) == 6
                        and exact(retained_identity(value), retained_identity(initial)), "Original owned volume/face state not retained")
                context = value["native_only_state"]["geometry_context"]
                require(integer(context["total_coincident_vertices"], "coincident") == 0
                        and integer(context["total_degenerate_surfaces"], "degenerate") == 0
                        and context["aspect_transform"] is False and context["no_transform"] is True,
                        "Original excluded geometry correction active")
        outputs[case["id"]] = result
        completed.append(finish)
    return outputs, max(completed)


def self_test():
    """Pure in-memory negative metadata tests; no scientific values or writes."""
    probes = [(True, 1), (1, 1.0), (2**53 + 1, 2**53), (0.0, -0.0), ({"a": 1}, {"a": True}), ([1], [1.0])]
    require(all(not exact(a, b) for a, b in probes), "Typed metadata self-test failed")
    require(exact({"a": [1, False, 0.0]}, {"a": [1, False, 0.0]}), "Typed equality self-test failed")
    for value in (True, 1.0, -1):
        try:
            integer(value, "negative fixture")
        except ValueError:
            continue
        raise ValueError("Invalid integer fixture accepted")
    try:
        path_of("../outside-proof")
    except ValueError:
        pass
    else:
        raise ValueError("Escaping path fixture accepted")
    original = timestamp("2026-01-01T01:00:00+00:00")
    verify_after_original({"started_utc": "2026-01-01T01:00:01+00:00"}, original)
    try:
        verify_after_original({"started_utc": "2026-01-01T00:59:59+00:00"}, original)
    except ValueError:
        pass
    else:
        raise ValueError("Before-original execution fixture accepted")
    try:
        timestamp("2026-01-01T01:00:00")
    except ValueError:
        pass
    else:
        raise ValueError("Unzoned execution fixture accepted")
    return {"metadata_self_tests": 14, "scientific_comparison_performed": False, "files_written": False}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check-contracts", action="store_true", help="Read frozen contracts/source pins only; no engines or reports")
    mode.add_argument("--self-test", action="store_true", help="Pure typed-metadata negative fixtures; no files written")
    args = parser.parse_args()
    if args.self_test:
        result = self_test()
    else:
        bindings = Bindings()
        refs, source, cases, _ = frozen_contracts(None, bindings)
        verify_sources(source, bindings)
        bindings.unchanged()
        result = {"contracts": refs, "source_files": len(source["source_files"]), "source_ranges": len(source["selected_ranges"]),
                  "helper_cases": cases["helper_case_count"], "native_cases": cases["native_case_count"],
                  "scientific_comparison_performed": False, "files_written": False, "gates_updated": False}
    print(json.dumps(result, allow_nan=False, sort_keys=True))


if __name__ == "__main__":
    main()
