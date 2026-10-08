#!/usr/bin/env python3
"""Input-only ZON-01 launch provenance, derived from preserved baseline metadata.

The reused metadata validators read receipts/reviews and hash original output
bytes. They never deserialize original scientific results or supply them to Rust.
Actual execution and numerical comparison are separate.
"""
from __future__ import annotations

import json
from pathlib import Path
import shutil
import sys

sys.dont_write_bytecode = True
from geo03_provenance import (
    ROOT, Bindings, exact, path_of, read, ref, require,
    same_binding, timestamp, verify_contract_bindings, verify_rust_build,
)

RAW = ROOT / ".runtime/porting/ZON-01"
PROVENANCE = ROOT / "tools/porting/geo03_provenance.py"
PROVENANCE_SHA = "5ca27645245ee9e7206a31c8906904394a5216625723cbfad12a5d5f49da9bf5"


FROZEN = {
    "source": "7e1e5582e9ec5348cd62f00c8e8c99cbfd6d03bea2ddb91e992fa10bb2900fee",
    "cases": "f9c00245eedaaaa24600b6a15fa9864fbaa89255fe8a935221e8b1ab93bfd7b2",
    "tolerances": "56479ebb3b821f1c90176d0792141c57252196ac6f9706f1a1313ff514c0b0f4",
}


REQUEST_SHA = "a3a69aadef063f5710d3a7af739d306d6486e68d62d376f3a3c009f737fa2972"


def write(path, value):
    with Path(path).open("xb") as stream:
        stream.write((json.dumps(value, indent=2, allow_nan=False) + "\n").encode("utf-8"))


def packet(bindings):
    refs = {key: {"path": f"energyplus_porting_plan/contracts/ZON-01-{key}.json", "sha256": digest}
            for key, digest in FROZEN.items()}
    for binding in refs.values():
        bindings.check(binding)
    cases = read(bindings.check(refs["cases"]))
    request_ref = cases["helper_request"]
    require(request_ref["sha256"] == REQUEST_SHA, "Reviewed input request changed")
    request = read(bindings.check(request_ref))
    require(request["schema"] == "zon01-helper-cases.v1" and request["expected_values_supplied"] is False
            and type(request["sequences"]) is list and len(request["sequences"]) == 8
            and [row["sequence_id"] for row in request["sequences"]] == cases["helper_sequence_ids"]
            and sum(len(row["operations"]) for row in request["sequences"]) == 34,
            "Input-only sequence identity/count differs")
    return refs, cases, request_ref


def original_metadata(helper_ref, matrix_ref, review_ref, refs, cases, bindings):
    """No original output JSON is deserialized by this launcher."""
    helper = read(bindings.check(helper_ref))
    review = read(bindings.check(review_ref))
    matrix = read(bindings.check(matrix_ref))
    require(helper["schema"] == "zon01-original-helper-execution.v1"
            and helper["Rust_compared"] is False and helper["gates_updated"] is False
            and helper["physics_executed"] is False and helper["original_parser_admission_claimed"] is False,
            "Original helper boundary differs")
    verify_contract_bindings(helper["contracts"], refs, bindings)
    require(same_binding(helper["request"], cases["helper_request"]), "Original request differs")
    bindings.check(helper["results"])
    actual = read(bindings.check(helper["execution"]))
    require(type(actual["exit_code"]) is int and actual["exit_code"] == 0
            and path_of(actual["cwd"]) == ROOT
            and len(actual["command"]) == 2
            and path_of(actual["command"][0]) == bindings.check(helper["binary"])
            and path_of(actual["command"][1]) == bindings.check(helper["request"]),
            "Original helper actual argv/outcome differs")
    bindings.check(actual["stdout"])
    bindings.check(actual["stderr"])
    helper_finished = timestamp(actual["finished_utc"])
    require(timestamp(actual["started_utc"]) <= helper_finished, "Original helper chronology differs")
    require(review["schema"] == "zon01-independent-original-data-review.v1"
            and review["status"] == "pass-source-provenance-before-Rust-numerical-execution"
            and same_binding(review["reviewed_helper_reference"], helper_ref)
            and same_binding(review["reviewed_native_matrix"], matrix_ref)
            and review["Rust_compared"] is False and review["gates_updated"] is False
            and review["scientific_reference_math_recomputed"] is False
            and review["engines_executed_by_review"] is False,
            "Accepted independent original-first review required")
    require(matrix["schema"] == "zon01-original-matrix.v1" and matrix["complete"] is True
            and matrix["requested_subset_complete"] is True and type(matrix["case_count"]) is int
            and matrix["case_count"] == len(matrix["cases"]) == 7
            and {row["case_id"] for row in matrix["cases"]} == {row["id"] for row in cases["native_cases"]}
            and matrix["Rust_compared"] is False and matrix["gates_updated"] is False,
            "Original ordinary-stage witnesses incomplete")
    finished = [helper_finished]
    for row in matrix["cases"]:
        receipt = read(bindings.check(row["receipt"]))
        verify_contract_bindings(receipt["contracts"], refs, bindings)
        require(receipt["schema"] == "zon01-original-execution.v1"
                and receipt["case_id"] == row["case_id"] and receipt["Rust_compared"] is False
                and receipt["gates_updated"] is False and receipt["pristine_member_output_pairing_claimed"] is False
                and same_binding(receipt["results"], row["results"]), "Ordinary receipt boundary differs")
        bindings.check(row["results"])
        command = read(bindings.check(receipt["execution"]))
        require(type(command["exit_code"]) is int and command["exit_code"] == 0
                and timestamp(command["started_utc"]) <= timestamp(command["finished_utc"]),
                "Actual original ordinary wrapper failed")
        bindings.check(command["stdout"])
        bindings.check(command["stderr"])
        finished.append(timestamp(command["finished_utc"]))
    completed = timestamp(review["review_completed_utc"])
    require(max(finished) <= completed, "Independent review predates complete original-first evidence")
    return completed


def archive_sources(directory, paths):
    source = directory / "source"
    source.mkdir()
    rows = []
    for current in paths:
        current = Path(current).resolve()
        target = source / current.name
        require(not target.exists(), "Duplicate archive basename")
        shutil.copyfile(current, target)
        require(target.read_bytes() == current.read_bytes(), "Source archive byte identity differs")
        rows.append({"historical_path": current.relative_to(ROOT).as_posix(), "archive": ref(target)})
    return rows
