#!/usr/bin/env python3
"""Compare actual CLK-02 selected raw/header DTOs with frozen exact policies.

This reader never runs either engine, reconstructs parser results, injects
reference answers, changes tolerance profiles or closes card gates.
"""
from __future__ import annotations

import argparse
from collections import Counter
from datetime import datetime, timezone
import json
import math
from pathlib import Path
import shutil
import struct
import sys

sys.dont_write_bytecode = True
from geo03_provenance import (
    ROOT, Bindings, exact, execution_interval, path_of, read, ref, require,
    same_binding, timestamp, verify_contract_bindings, verify_rust_build,
)

FROZEN = {
    "source": "4fc97c6a0a775f956c79855795f5a23c6e190ba2ad25dc2cba4b04477a9cf1d0",
    "cases": "4752d6704b6ad8f6e4f1ea4907ce4d082bec041d0e2f22e28d11bd1c04ab062c",
    "tolerances": "3fb137a3096a331ea1ab3f751d2667530d9a6d8ca40d785bdd5c07bfd304350e",
}
REQUEST_SHA = "06059e9be2b8e2a20dc9a5d603d061dc427bb3dfa8ba83ec2b9fca75c27a709a"
PROVENANCE_SHA = "5ca27645245ee9e7206a31c8906904394a5216625723cbfad12a5d5f49da9bf5"
RAW_CONTEXT = ("EndDayOfMonth", "wvarsMissedCounts.WeathCodes")
HEADER_CONTEXT = ("EndDayOfMonth", "LeapYearAdd", "wvarsMissedCounts.WeathCodes")
HEADER_STRUCTURAL = ("SpecialDays_allocated", "DataPeriods_allocated")
STREAM_FIELDS = ("is_open", "good", "eof", "fail", "bad", "position_available", "position_byte")
UNPAIRED = (
    "source_only_context", "source_only_diagnostics", "native rdstate_bits and error_state_text",
    "native exception/error/warning text and global error IO state",
    "private RField21 and original internal dispatcher/read call counts",
    "whole-open private Line and unobserved source-local write events",
    "Typical/Extreme and Ground branch state unused by fixed CON consumers",
    "sentinel replacement, physical normalization, rain/snow, sky, record selection, interpolation, civil calendar and warmup",
)


def write(path: Path, value: object) -> None:
    with path.open("xb") as stream:
        stream.write((json.dumps(value, indent=2, allow_nan=False) + "\n").encode("utf-8"))


def scalar(value: object, label: str) -> tuple[str, str]:
    require(type(value) is dict and set(value) == {"value", "value_bits", "value_class"}, "Typed scalar required: " + label)
    bits = value["value_bits"]
    require(type(bits) is str and len(bits) == 16 and bits == bits.lower(), "Binary64 token required: " + label)
    try:
        decoded = struct.unpack(">d", bytes.fromhex(bits))[0]
    except (ValueError, struct.error) as error:
        raise ValueError("Invalid binary64 token: " + label) from error
    require(type(value["value"]) is float and math.isfinite(decoded)
            and math.isfinite(value["value"]) and struct.pack(">d", value["value"]).hex() == bits,
            "Finite scalar value/bit identity differs: " + label)
    kind = "negative_zero" if bits == "8000000000000000" else "positive_zero" if bits == "0000000000000000" else "finite"
    require(value["value_class"] == kind, "Scalar classification differs: " + label)
    return bits, kind


class Comparison:
    def __init__(self) -> None:
        self.count = 0
        self.mismatch_count = 0
        self.mismatches: list[dict] = []
        self.categories: Counter[str] = Counter()

    def compare(self, actual: object, original: object, location: str, category: str = "typed_state") -> None:
        self.count += 1
        self.categories[category] += 1
        if not exact(actual, original):
            self.mismatch_count += 1
            if len(self.mismatches) < 1000:
                self.mismatches.append({"location": location, "actual": actual, "original": original})

    def value(self, actual: object, original: object, location: str) -> None:
        if type(original) is dict and set(original) == {"value", "value_bits", "value_class"}:
            self.compare(scalar(actual, location), scalar(original, location), location, "finite_binary64_slots")
        elif type(original) is dict:
            require(type(actual) is dict and set(actual) == set(original), "Selected nested field set differs: " + location)
            for key in original:
                self.value(actual[key], original[key], location + "/" + key)
        elif type(original) is list:
            require(type(actual) is list and len(actual) == len(original), "Selected array cardinality differs: " + location)
            self.compare(len(actual), len(original), location + "/length", "array_cardinality")
            for index, (a, o) in enumerate(zip(actual, original, strict=True)):
                self.value(a, o, location + "/" + str(index))
        else:
            self.compare(actual, original, location)


def selected(check: Comparison, actual: dict, original: dict, keys: tuple | list, location: str) -> None:
    require(type(actual) is dict and type(original) is dict and set(keys) <= set(actual) and set(keys) <= set(original),
            "All selected fields required: " + location)
    for key in keys:
        check.value(actual[key], original[key], location + "/" + key)


def stream(check: Comparison, actual: dict, original: dict, location: str) -> None:
    for row in (actual, original):
        require(type(row) is dict and set(STREAM_FIELDS) | {"file_path"} <= set(row), "Semantic stream fields required: " + location)
        require(type(row["file_path"]) is str, "Typed stream path required: " + location)
        require(all(type(row[key]) is bool for key in STREAM_FIELDS if key != "position_byte"), "Typed stream flags required: " + location)
        require((type(row["position_byte"]) is int and row["position_byte"] >= 0) if row["position_available"] else row["position_byte"] is None,
                "Stream position availability differs: " + location)
    selected(check, actual, original, STREAM_FIELDS, location)
    if not actual["file_path"] or not original["file_path"]:
        # The original default closed InputFile has no assigned path yet.
        check.compare(actual["file_path"], original["file_path"], location + "/unassigned_file_path", "input_identity")
    else:
        check.compare(path_of(actual["file_path"]).relative_to(ROOT).as_posix(),
                      path_of(original["file_path"]).relative_to(ROOT).as_posix(), location + "/actual_file_path", "input_identity")


def outcome(check: Comparison, actual: dict, original: dict, location: str) -> None:
    for row in (actual, original):
        require(type(row) is dict and {"status", "source_fatal", "exception_message"} <= set(row), "Outcome availability required: " + location)
        require(type(row["source_fatal"]) is bool and row["status"] in ("source_returned", "source_fatal"), "Outcome type differs: " + location)
        require(row["source_fatal"] == (row["status"] == "source_fatal"), "Outcome status/flag inconsistent: " + location)
        require(type(row["exception_message"]) is str if row["source_fatal"] else row["exception_message"] is None,
                "Exception availability differs: " + location)
    selected(check, actual, original, ("status", "source_fatal"), location)


def raw_snapshot(check: Comparison, actual: dict, original: dict, policy: dict, location: str) -> None:
    keys = {"date_fields", "mandatory_reals", "optional_reals", "WObs", "weather_codes", "ErrorFound", "private_RField21_observed"}
    for row in (actual, original):
        require(type(row) is dict and set(row) == keys, "Exactly defined raw storage required: " + location)
        require(type(row["ErrorFound"]) is bool and row["private_RField21_observed"] is False, "Raw flags/private output limit differs: " + location)
        require(type(row["WObs"]) is int and len(row["weather_codes"]) == policy["weather_codes"] == 9
                and all(type(value) is int for value in row["weather_codes"]), "Raw WObs/nine code storage differs: " + location)
        require(set(row["date_fields"]) == set(policy["date_integer_fields"])
                and all(type(value) is int for value in row["date_fields"].values()), "All five typed dates required: " + location)
        require(set(row["mandatory_reals"]) == set(policy["mandatory_real_fields"])
                and set(row["optional_reals"]) == set(policy["optional_real_fields"]), "All 20+6 raw scalar outputs required: " + location)
    check.value(actual, original, location)


def raw_call(check: Comparison, actual: dict, original: dict, policy: dict, location: str, fixed: bool) -> None:
    for row in (actual, original):
        require(row["public_storage_defined"] is True and row["individual_source_write_events_observed"] is False,
                "Only defined public caller storage may be paired: " + location)
    identity = ("record_index", "read_ordinal") if fixed else ("record_id", "kind")
    selected(check, actual, original, identity + ("input_line_utf8", "input_line_sha256"), location)
    for phase in ("before", "after"):
        raw_snapshot(check, actual[phase], original[phase], policy, location + "/" + phase)
    for row in (actual, original):
        require(type(row["weather_code_missed_count_before"]) is int and type(row["weather_code_missed_count_after"]) is int,
                "Typed original-owned missed counter required: " + location)
    selected(check, actual, original, ("weather_code_missed_count_before", "weather_code_missed_count_after"), location)
    outcome(check, actual["call_outcome"], original["call_outcome"], location + "/call_outcome")
    if fixed:
        selected(check, actual, original, ("read_good", "read_eof"), location)
        for phase in ("stream_before_read", "stream_after_read"):
            stream(check, actual[phase], original[phase], location + "/" + phase)


def header_state(check: Comparison, actual: dict, original: dict, source: dict, location: str) -> None:
    selected(check, actual, original, source["paired_header_state"] + list(HEADER_CONTEXT), location)
    for row in (actual, original):
        require(all(type(row[key]) is bool for key in HEADER_STRUCTURAL), "Actual container availability required: " + location)
    selected(check, actual, original, HEADER_STRUCTURAL, location + "/container_availability")


def header_call(check: Comparison, actual: dict, original: dict, source: dict, location: str) -> None:
    selected(check, actual, original, ("case_id", "ErrorsFound_before", "ErrorsFound_after", "Line_observed"), location)
    for row in (actual, original):
        require(type(row["ErrorsFound_before"]) is bool and type(row["ErrorsFound_after"]) is bool
                and type(row["Line_observed"]) is bool, "Typed header context required: " + location)
        require(row["internal_dispatch_or_read_call_counts_observed"] is False, "Original private call counts cannot be claimed: " + location)
    if original["Line_observed"]:
        require(actual["Line_observed"] is True, "Direct header Line availability differs: " + location)
        selected(check, actual, original, ("Line_before", "Line_after", "Line_before_sha256", "Line_after_sha256"), location)
    else:
        for row in (actual, original):
            require(row["Line_observed"] is False and row["Line_before"] is None and row["Line_after"] is None,
                    "Whole-open local Line must remain unavailable: " + location)
    for phase in ("constructor", "prepared", "before", "after"):
        header_state(check, actual[phase], original[phase], source, location + "/" + phase)
    for phase in ("stream_before", "stream_after"):
        stream(check, actual[phase], original[phase], location + "/" + phase)
    outcome(check, actual["call_outcome"], original["call_outcome"], location + "/call_outcome")


def packet(bindings: Bindings) -> tuple:
    refs = {key: {"path": f"energyplus_porting_plan/contracts/CLK-02-{key}.json", "sha256": sha} for key, sha in FROZEN.items()}
    source, cases, tolerances = [read(bindings.check(refs[key])) for key in FROZEN]
    require(all(value["frozen_before_numerical_execution"] is True and value["card"] == "CLK-02" for value in (source, cases, tolerances)),
            "Frozen CLK-02 packet required")
    require(cases["helper_request"]["sha256"] == REQUEST_SHA and source["paired_raw_outputs"]["private_RField21_observed"] is False,
            "Frozen request/private output boundary differs")
    request = read(bindings.check(cases["helper_request"]))
    require(request["expected_values_supplied"] is False and request["expected_exits_supplied"] is False, "Input-only request required")
    return refs, source, cases, request


def metadata(args: argparse.Namespace, refs: dict, cases: dict, bindings: Bindings) -> tuple:
    original_ref, peer_ref, baseline_peer_ref = ref(args.original_reference), ref(args.original_data_review), ref(args.baseline_review)
    original, peer, baseline_peer = [read(bindings.check(value)) for value in (original_ref, peer_ref, baseline_peer_ref)]
    require(original["schema"] == "clk02-original-helper-execution.v1" and original["actual_helper_exit_code"] == 0
            and original["preservation_checks_passed"] is True and original["Rust_compared"] is False
            and original["gates_updated"] is False and original["source_expected_answers_supplied"] is False,
            "Preserved actual original execution required")
    verify_contract_bindings(original["contracts"], refs, bindings)
    require(same_binding(original["request"], cases["helper_request"]), "Original request differs")
    require(peer["schema"] == "clk02-independent-original-data-review.v1"
            and peer["status"] == "pass-preserved-original-data-before-Rust-baseline"
            and same_binding(peer["reviewed_reference"], original_ref)
            and same_binding(peer["reviewed_results"], original["results"])
            and peer["gates_updated"] is False and peer["engines_executed_by_review"] is False,
            "Accepted original data peer required")
    require(baseline_peer["schema"] == "clk02-independent-legacy-gap-review.v1"
            and baseline_peer["status"] == "pass-preserved-baseline-and-real-gap"
            and same_binding(baseline_peer["reviewed_original_reference"], original_ref)
            and same_binding(baseline_peer["reviewed_original_data_review"], peer_ref)
            and baseline_peer["gates_updated"] is False, "Preserved honest legacy baseline review required")
    original_command = read(bindings.check(original["execution"]))
    _, original_completed = execution_interval(original_command, bindings)
    require(len(original_command["command"]) == 4 and path_of(original_command["command"][0]) == bindings.check(original["binary"])
            and path_of(original_command["command"][1]) == ROOT
            and path_of(original_command["command"][2]) == bindings.check(original["request"]), "Actual original argv differs")
    require(same_binding(original_command["binary"], original["binary"]) and same_binding(original_command["request"], original["request"]),
            "Original executed input/binary identity differs")
    for key in ("native_driver_build", "independent_native_build_review", "independent_contract_review", "validated_derivative"):
        bindings.check(original[key])
    completed = max(timestamp(peer["review_completed_utc"]), timestamp(baseline_peer["review_completed_utc"]))
    require(original_completed <= timestamp(peer["review_completed_utc"]) <= completed, "Original-first data review chronology differs")
    build_ref = ref(args.rust_build)
    build, available, build_command = verify_rust_build(build_ref, bindings, kind="example", example=args.rust_example,
        committed=True, required_sources=(f"crates/ep_runtime/examples/{args.rust_example}.rs",))
    execution_ref = ref(args.rust_execution)
    execution = read(bindings.check(execution_ref))
    require(execution["schema"] == "recorded-porting-command.v1" and execution["launch_error"] is None
            and execution["source_bytes_match_before_and_after"] is True and execution["recorder_updates_gates"] is False
            and execution["recorder_supplies_reference_answers"] is False, "Recorded canonical Rust execution required")
    started, finished = execution_interval(execution, bindings)
    require(completed <= started and timestamp(build_command["finished_utc"]) <= started, "Canonical Rust ran before complete original/baseline evidence")
    require(len(execution["command"]) == 2 and path_of(execution["command"][0]) == bindings.check(build["binary"])
            and path_of(execution["command"][1]) == bindings.check(cases["helper_request"]), "Rust receives exactly input-only request")
    require(same_binding(execution["source_snapshot"], build["source_snapshot"])
            and execution["repository_before"]["head"] == execution["repository_after"]["head"] == build["implementation_commit"],
            "Built and executed canonical source identity differs")
    results_ref = ref(args.rust_results) if args.rust_results else execution["stdout"]
    require(same_binding(results_ref, execution["stdout"]), "Rust results must be actual successful stdout")
    return original, read(bindings.check(original["results"])), read(bindings.check(results_ref)), {
        "original_helper": original_ref, "independent_original_review": peer_ref, "independent_legacy_gap_review": baseline_peer_ref,
        "Rust_build": build_ref, "Rust_execution": execution_ref, "Rust_results": results_ref,
        "implementation_commit": build["implementation_commit"], "crates_tree": build["crates_tree"],
        "committed_source_certification": build["committed_source_certification"],
        "available_source_inventory_count": len(available), "actual_Rust_finished_utc": finished.isoformat(),
    }


def projection(args: argparse.Namespace, refs: dict, source: dict, cases: dict, identities: dict, bindings: Bindings) -> dict:
    projection_ref = ref(args.dto_projection)
    declared = read(bindings.check(projection_ref))
    require(declared["schema"] == "clk02-unit-dto-projection.v1"
            and declared["status"] == "declared-before-canonical-probe-execution"
            and declared["scientific_comparer_executed"] is False
            and declared["prospective_numerical_PASS_claimed"] is False and declared["gates_updated"] is False,
            "Declared pre-execution DTO policy required")
    verify_contract_bindings(declared["contracts"], refs, bindings)
    require(same_binding(declared["input_request"], cases["helper_request"])
            and same_binding(declared["comparer"], ref(Path(__file__)))
            and declared["comparer_archive"]["sha256"] == declared["comparer"]["sha256"]
            and declared["metadata_dependency_archive"]["sha256"] == PROVENANCE_SHA,
            "Frozen comparer/dependency/input identity differs")
    for key in ("comparer_archive", "metadata_dependency_archive", "source_container_supplement", "independent_source_container_review"):
        bindings.check(declared[key])
    require(exact(declared["raw_sequences"]["owned_phase_fields"], list(RAW_CONTEXT))
            and exact(declared["raw_sequences"]["raw_output_fields"], source["paired_raw_outputs"])
            and exact(declared["header_roots"]["selected_value_fields"], source["paired_header_state"])
            and exact(declared["header_roots"]["observed_read_and_counter_context"], list(HEADER_CONTEXT))
            and exact(declared["header_roots"]["container_availability_fields"], list(HEADER_STRUCTURAL))
            and declared["header_roots"]["container_flags_derived_from_lengths"] is False
            and exact(declared["all_stream_selected_fields"], list(STREAM_FIELDS) + ["file_path"]),
            "Pre-execution selected DTO paths differ")
    execution = read(bindings.check(identities["Rust_execution"]))
    require(timestamp(declared["created_utc"]) <= timestamp(execution["started_utc"]), "DTO mapping was declared after Rust numerical execution")
    return projection_ref


def compare(check: Comparison, original: dict, rust: dict, source: dict, cases: dict, request: dict) -> dict:
    require(original["schema"] == "clk02-helper-results.v1" and rust["schema"] == "clk02-rust-probe-results.v1", "Actual DTO schemas differ")
    for dto in (original, rust):
        require(dto["complete"] is True and dto["expected_answers_supplied"] is False and dto["gates_updated"] is False
                and dto["physics_executed"] is False, "Probe orchestration/scope boundary differs")
    policy = source["paired_raw_outputs"]
    sequences, candidate_sequences = original["record_sequences"], rust["record_sequences"]
    headers, candidate_headers = original["header_cases"], rust["header_cases"]
    require(type(sequences) is list and type(candidate_sequences) is list and len(sequences) == len(candidate_sequences) == 40,
            "All 40 raw sequence owners required")
    require([row["sequence_id"] for row in sequences] == [row["sequence_id"] for row in candidate_sequences] == cases["record_sequence_ids"],
            "Raw sequence identities/order differ")
    require(type(headers) is list and type(candidate_headers) is list and len(headers) == len(candidate_headers) == 22,
            "All 22 diagnostic header roots required")
    require([row["case_id"] for row in headers] == [row["case_id"] for row in candidate_headers] == cases["header_case_ids"],
            "Header identities/order differ")
    outcomes: Counter[str] = Counter()
    raw_count = 0
    for actual, expected, inputs in zip(candidate_sequences, sequences, request["record_sequences"], strict=True):
        location = "raw/" + expected["sequence_id"]
        for phase in ("constructor", "prepared", "final_state"):
            selected(check, actual[phase], expected[phase], RAW_CONTEXT, location + "/" + phase)
        require(len(actual["operations"]) == len(expected["operations"]) == len(inputs["operations"]), "Raw operation cardinality differs: " + location)
        for a, o, supplied in zip(actual["operations"], expected["operations"], inputs["operations"], strict=True):
            require(a["record_id"] == o["record_id"] == supplied["record_id"], "Raw record identity differs: " + location)
            require(a["input_line_utf8"] == o["input_line_utf8"] == supplied["line_utf8"]
                    and a["input_line_sha256"] == o["input_line_sha256"] == supplied["line_sha256"], "Actual raw diagnostic input differs: " + location)
            raw_call(check, a, o, policy, location + "/" + o["record_id"], fixed=False)
            outcomes[o["call_outcome"]["status"]] += 1
            raw_count += 1
    require(raw_count == cases["counts"]["diagnostic_raw_calls"] == 46, "All diagnostic raw calls required")
    for actual, expected in zip(candidate_headers, headers, strict=True):
        header_call(check, actual, expected, source, "header/" + expected["case_id"])
    fixed, candidate_fixed = original["fixed_epw"], rust["fixed_epw"]
    selected(check, candidate_fixed, fixed, ("case_id", "record_count", "ErrorsFound_before", "ErrorsFound_after"), "fixed_epw")
    require(candidate_fixed["record_count"] == fixed["record_count"] == 8760, "Fixed weather raw record count differs")
    for phase in ("constructor", "prepared", "header_after", "final_state"):
        header_state(check, candidate_fixed[phase], fixed[phase], source, "fixed_epw/" + phase)
    outcome(check, candidate_fixed["open_call_outcome"], fixed["open_call_outcome"], "fixed_epw/open_call_outcome")
    for phase in ("stream_before_open", "stream_after_header", "final_stream"):
        stream(check, candidate_fixed[phase], fixed[phase], "fixed_epw/" + phase)
    require(len(candidate_fixed["records"]) == len(fixed["records"]) == 8760, "All fixed raw actual records required")
    for index, (actual, expected) in enumerate(zip(candidate_fixed["records"], fixed["records"], strict=True)):
        require(actual["record_index"] == expected["record_index"] == index, "Fixed raw ordinal differs")
        raw_call(check, actual, expected, policy, "fixed_epw/records/" + str(index), fixed=True)
    terminal, candidate_terminal = fixed["terminal_read"], candidate_fixed["terminal_read"]
    selected(check, candidate_terminal, terminal, ("data", "eof", "good", "read_ordinal"), "fixed_epw/terminal_read")
    for phase in ("stream_before", "stream_after"):
        stream(check, candidate_terminal[phase], terminal[phase], "fixed_epw/terminal_read/" + phase)
    return {"fresh_raw_owners": len(sequences), "diagnostic_raw_calls": raw_count,
        "fixed_epw_raw_calls": len(fixed["records"]), "all_raw_calls": raw_count + len(fixed["records"]),
        "diagnostic_header_roots": len(headers), "all_header_roots": len(headers) + 1,
        "diagnostic_original_actual_outcome_counts": dict(outcomes),
        "selected_header_state_keys": source["paired_header_state"],
        "raw_owner_context_keys": list(RAW_CONTEXT), "header_read_context_keys": list(HEADER_CONTEXT),
        "header_container_availability_keys": list(HEADER_STRUCTURAL),
        "semantic_stream_keys": list(STREAM_FIELDS) + ["file_path"],
        "source_error_text_paired": False, "unpaired_fields_counted_as_PASS": False}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("original-reference", "original-data-review", "baseline-review", "rust-build", "rust-execution", "dto-projection", "output-dir"):
        parser.add_argument("--" + name, type=Path, required=True)
    parser.add_argument("--rust-results", type=Path)
    parser.add_argument("--rust-example", default="clk02_probe")
    args = parser.parse_args()
    output = args.output_dir.resolve()
    require(output.is_relative_to(ROOT / ".runtime") and not output.exists(), "Fresh ignored output directory required")
    started = datetime.now(timezone.utc).isoformat()
    bindings = Bindings()
    dependency = {"path": "tools/porting/geo03_provenance.py", "sha256": PROVENANCE_SHA}
    bindings.check(dependency)
    refs, source, cases, request = packet(bindings)
    original_receipt, original, rust, identities = metadata(args, refs, cases, bindings)
    projection_ref = projection(args, refs, source, cases, identities, bindings)
    verify_contract_bindings(original["contracts"], refs, bindings)
    verify_contract_bindings(rust["contracts"], refs, bindings)
    require(same_binding(rust["actual_request"], cases["helper_request"]), "Rust observed request identity differs")
    require(same_binding(original["actual_request"], cases["helper_request"]), "Original observed request identity differs")
    check = Comparison()
    coverage = compare(check, original, rust, source, cases, request)
    bindings.unchanged()
    output.mkdir()
    source_dir = output / "source"
    source_dir.mkdir()
    archives = []
    for path in (Path(__file__).resolve(), bindings.check(dependency)):
        archived = source_dir / path.name
        shutil.copyfile(path, archived)
        require(path.read_bytes() == archived.read_bytes(), "Comparer archive identity differs")
        archives.append({"historical_path": path.relative_to(ROOT).as_posix(), "archive": ref(archived)})
    report = {
        "schema": "clk02-unit-comparison.v1", "status": "pass-exact-selected-raw-and-header-state" if check.mismatch_count == 0 else "fail-selected-raw-or-header-state",
        "scientific_certification_passed": check.mismatch_count == 0, "baseline_diagnostic": False,
        "started_utc": started, "completed_utc": datetime.now(timezone.utc).isoformat(),
        "actual_reader_command": list(sys.orig_argv), "actual_reader_cwd": str(Path.cwd()),
        "contracts": refs, "request": cases["helper_request"], **identities,
        "declared_pre_execution_DTO_projection": projection_ref,
        "original_results": original_receipt["results"],
        "comparison": {"comparison_count": check.count, "mismatch_count": check.mismatch_count,
            "categories": dict(check.categories), "mismatches": check.mismatches,
            "mismatch_payload_limit": 1000, "mismatch_payloads_truncated": check.mismatch_count > len(check.mismatches)},
        "coverage": coverage, "executed_reader_source_archives": archives,
        "source_inventory_scope": "Available archived source bytes; not a compiler file-selection inventory",
        "engines_executed_by_reader": False, "Cargo_executed_by_reader": False, "Git_executed_by_reader": False,
        "reference_outputs_supplied_to_Rust": False, "original_RHS_recomputed": False, "gates_updated": False,
        "unpaired_scope": list(UNPAIRED),
        "limits": ["This certifies only frozen raw parser/header fields and their observed contexts.",
            "Finite scalars, signed zero, typed integers/text/booleans, array cardinality/order and actual source-fatal availability use exact comparisons.",
            "Original public caller storage remains defined after source-fatal outcomes; no private source-local writes or global error-text parity is inferred.",
            "Raw constructor phases pair only the actual raw owner context, rather than claiming Rust owns all original EnergyPlus state.",
            "Production adapter handoff, physical invocation and all excluded downstream science require separate evidence."],
    }
    write(output / "unit-comparison.json", report)
    print(json.dumps(report, allow_nan=False))
    if check.mismatch_count:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
