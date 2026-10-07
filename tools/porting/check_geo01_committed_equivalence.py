#!/usr/bin/env python3
"""Bind existing GEO-01 unit results to a byte-identical committed build.

This reads preserved artifacts only. It does not invoke Git, Cargo, either
engine, or a scientific comparer, and does not turn old runs into new runs.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import shutil
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
CACHE = {}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def path(value):
    p = Path(value)
    return p.resolve() if p.is_absolute() else (ROOT / p).resolve()


def read(p):
    return json.loads(Path(p).read_text(encoding="utf-8"))


def fingerprint(p):
    s = p.stat()
    return s.st_size, s.st_mtime_ns


def sha(p):
    p = Path(p).resolve()
    state = fingerprint(p)
    if p in CACHE:
        require(CACHE[p][0] == state, "Artifact changed during audit: " + str(p))
        return CACHE[p][1]
    h = hashlib.sha256()
    with p.open("rb") as f:
        for block in iter(lambda: f.read(1048576), b""):
            h.update(block)
    require(fingerprint(p) == state, "Artifact changed while hashing: " + str(p))
    CACHE[p] = state, h.hexdigest()
    return h.hexdigest()


def ref(p):
    p = Path(p).resolve()
    return {"path": p.relative_to(ROOT).as_posix(), "sha256": sha(p)}


def binding(v):
    require(type(v) is dict and type(v.get("path")) is str and type(v.get("sha256")) is str, "Malformed binding")
    p = path(v["path"])
    require(sha(p) == v["sha256"], "Artifact hash differs: " + v["path"])
    return p


def bind_all(v):
    if type(v) is dict:
        if "path" in v and "sha256" in v:
            binding(v)
        for child in v.values():
            bind_all(child)
    elif type(v) is list:
        for child in v:
            bind_all(child)


def exact(a, b):
    if type(a) is not type(b):
        return False
    if type(a) is dict:
        return a.keys() == b.keys() and all(exact(a[k], b[k]) for k in a)
    if type(a) is list:
        return len(a) == len(b) and all(exact(x, y) for x, y in zip(a, b))
    return a == b


def equal_bytes(a, b, label):
    require(a["sha256"] == b["sha256"] and binding(a).read_bytes() == binding(b).read_bytes(), label + " bytes differ")


def source_map(build):
    rows = build["crates_sources"]
    require(type(rows) is list and len(rows) == 3637, "Expected all 3637 source archives")
    require(all(type(r.get("historical_path")) is str for r in rows), "Source identity missing")
    result = {r["historical_path"]: r for r in rows}
    require(len(result) == len(rows), "Duplicate historical source path")
    return result


def check_build(build):
    require(build["schema"] == "geo01-Rust-build.v1" and type(build["source_worktree_clean"]) is bool, "Wrong build receipt")
    bind_all(build)
    execution = read(binding(build["build_execution"]))
    bind_all(execution)
    require(type(execution["exit_code"]) is int and execution["exit_code"] == 0, "Actual build failed")
    require(exact(execution["command"], ["cargo", "build", "-p", "ep_cli", "--bin", "eplus-rs", "-j", "2"])
            and path(execution["cwd"]) == ROOT, "Actual build recipe/cwd differs")
    require(execution["repository_head"] == build["repository_head"]
            and exact(execution["sources"], build["crates_sources"]), "Actual build source/revision differs")


def audit(args):
    out = args.output_dir.resolve()
    require(not out.exists() and not args.evidence.exists(), "Use fresh raw/evidence destinations")
    fixed = read(args.fixed_build)
    committed = read(args.committed_build)
    matrix = read(args.matrix)
    comparison = read(args.comparison)
    provenance = read(args.provenance)
    for document in [matrix, comparison, provenance]:
        bind_all(document)
    check_build(fixed)
    check_build(committed)
    require(committed["source_worktree_clean"] is True and fixed["source_worktree_clean"] is False,
            "Preserve committed-clean versus historical modified build state")
    require(committed["implementation_commit"] == committed["repository_head"], "Committed build revision differs")
    contracts = {k: ref(ROOT / "energyplus_porting_plan/contracts" / f"GEO-01-{k}.json")
                 for k in ["source", "cases", "tolerances"]}
    for document in [matrix, comparison, provenance]:
        require(exact(document["contracts"], contracts), "Frozen contract binding differs")
    require(exact(matrix["build"], ref(args.fixed_build)) and exact(matrix["binary"], fixed["binary"])
            and exact(provenance["build"], matrix["build"]) and exact(provenance["binary"], matrix["binary"])
            and exact(provenance["matrix"], ref(args.matrix)), "Historical build/proof binding differs")
    require(matrix["schema"] == "geo01-rust-cli-matrix.v1" and matrix["kind"] == "unit"
            and matrix["original_outputs_supplied_to_Rust"] is False
            and matrix["original_first_completed_before_Rust"] is True, "Historical unit boundary differs")
    require(provenance["schema"] == "geo01-unit-provenance-audit.v1" and provenance["status"] == "pass"
            and exact(provenance["case_count"], 37) and exact(provenance["archived_Rust_source_count"], 3637)
            and provenance["original_first_and_no_answer_protocol_checked"] is True
            and provenance["actual_full_dry_run_commands_and_summaries_checked"] is True
            and provenance["source_worktree_clean"] is False
            and provenance["repository_head"] == fixed["repository_head"], "Historical provenance differs")
    require(exact(provenance["actual_exit_codes"], {"4": 37})
            and provenance["physics_executed"] is False and provenance["runtime_admission_claimed"] is False,
            "Do not relabel historical exit 4 dry-runs as successful runtime admission")
    require(comparison["schema"] == "geo01-unit-comparison.v1" and comparison["status"] == "pass"
            and exact(comparison["case_count"], 37) and exact(comparison["coordinate_count"], 2664)
            and exact(comparison["mismatch_count"], 0) and comparison["full_37_case_matrix"] is True
            and comparison["physics_executed"] is False and comparison["production_connection_compared"] is False,
            "Historical mathematical proof boundary differs")
    require(exact(comparison["original_matrix"], matrix["original_first_matrix"]), "Original reference differs")
    frozen_cases = read(ROOT / "energyplus_porting_plan/contracts/GEO-01-cases.json")["cases"]
    expected = {c["id"]: c for c in frozen_cases}
    require(len(matrix["cases"]) == len(comparison["cases"]) == len(provenance["cases"]) == 37,
            "Exactly 37 historical result rows required")
    rows = {c["case_id"]: c for c in matrix["cases"]}
    compared = {c["case_id"]: c for c in comparison["cases"]}
    audited = {c["case_id"]: c for c in provenance["cases"]}
    require(len(expected) == len(rows) == len(compared) == len(audited) == 37
            and set(expected) == set(rows) == set(compared) == set(audited), "Complete frozen case sets differ")
    for case_id, case in expected.items():
        row, peer, checked = rows[case_id], compared[case_id], audited[case_id]
        for key in ["input", "weather"]:
            require(exact(row[key], case[key]) and exact(peer[key], case[key]), "Historical frozen input differs")
        require(exact(row["metadata"], case["metadata"]), "Historical metadata differs")
        require(len(row["runs"]) == 1 and row["runs"][0]["trace_level"] == "full", "Historical run count differs")
        run = row["runs"][0]
        compiled = ref(path(run["output_directory"]) / "compiled-geometry.json")
        require(exact(peer["rust_compiled_geometry"], compiled) and exact(checked["compiled_geometry"], compiled)
                and exact(checked["execution"], run["execution"]) and exact(checked["run_summary"], run["run_summary"]),
                "Mathematical/provenance proof does not refer to the same actual compile artifact")
        require(exact(peer["mismatch_count"], 0) and exact(checked["actual_exit_code"], 4), "Historical case status differs")
    equal_bytes(fixed["binary"], committed["binary"], "Ordinary CLI")
    for key in ["cargo_lock", "toolchain"]:
        equal_bytes(fixed[key], committed[key], key)
    require(fixed["rustc"] == committed["rustc"] and fixed["cargo"] == committed["cargo"], "Compiler identity differs")
    old_sources, new_sources = source_map(fixed), source_map(committed)
    require(old_sources.keys() == new_sources.keys(), "Historical source inventory differs")
    inventory = []
    for historical in sorted(old_sources):
        equal_bytes(old_sources[historical], new_sources[historical], historical)
        inventory.append([historical, new_sources[historical]["sha256"]])
    blob_audit = committed["compiled_source_vs_committed_blobs"]
    require(exact(blob_audit["source_count"], 3637) and exact(blob_audit["exact_byte_matches"], 3636)
            and blob_audit["all_other_content_exact"] is True
            and len(blob_audit["line_ending_only_differences"]) == 1, "Root-recorded committed blob audit differs")
    difference = blob_audit["line_ending_only_differences"][0]
    require(difference["historical_path"] == "crates/ep_runtime/src/ideal_loads/binding/scheduled_output.rs", "Unexpected line-ending difference")
    source = new_sources[difference["historical_path"]]
    require(exact(difference["compiled_archive"], {k: source[k] for k in ["path", "sha256"]}), "Line-ending archive binding differs")
    b = binding(source).read_bytes()
    require(b"\r\n" in b and hashlib.sha256(b.replace(b"\r\n", b"\n")).hexdigest() == difference["git_blob_sha256"]
            and source["sha256"] != difference["git_blob_sha256"], "Recorded CRLF-only normalization differs")
    out.mkdir(parents=True)
    archived = out / Path(__file__).name
    shutil.copyfile(Path(__file__), archived)
    report = {
        "schema": "geo01-committed-unit-equivalence.v1", "status": "pass", "card": "GEO-01",
        "method": "reuse of preserved unit results through exact executable and source byte identity",
        "command": [sys.executable, *sys.orig_argv[1:]], "executed_checker": ref(archived), "actual_exit_code": 0,
        "contracts": contracts,
        "fixed_proofs": {"matrix": ref(args.matrix), "mathematical_comparison": ref(args.comparison), "producer_provenance": ref(args.provenance)},
        "builds": {"fixed_modified_source": ref(args.fixed_build), "committed_clean_source": ref(args.committed_build)},
        "binary": committed["binary"], "binary_exact_bytes_equal": True,
        "implementation_commit": committed["implementation_commit"], "crates_tree": committed["crates_tree"],
        "source_archive_count": 3637, "source_archive_byte_matches": 3637,
        "source_inventory_path_sha256": hashlib.sha256(json.dumps(inventory, separators=(",", ":")).encode()).hexdigest(),
        "cargo_lock_and_toolchain_bytes_equal": True, "compiler_identities_equal": True,
        "compiled_source_vs_committed_blobs": blob_audit,
        "committed_blob_audit_basis": "Root-recorded build receipt; no Git invocation by this checker; CRLF archive normalization independently rehashed",
        "historical_repository_head": fixed["repository_head"], "historical_source_worktree_clean": False,
        "committed_source_worktree_clean": True, "reused_unit_case_count": 37, "reused_coordinate_count": 2664,
        "reused_unit_metrics": comparison["metrics"], "reused_unit_mismatch_count": comparison["mismatch_count"],
        "historical_unit_exit_codes": provenance["actual_exit_codes"],
        "historical_input_and_artifact_identity_checked": True, "original_outputs_supplied_to_Rust": False,
        "fresh_committed_unit_execution_claimed": False, "new_unit_cli_runs": 0, "new_numerical_comparisons": 0,
        "physics_executed": False, "runtime_admission_claimed": False, "gates_updated": False,
        "limitations": ["historical Full dry-run outputs retain precommit HEAD and exit 4",
                        "this is executable/source equivalence, not a fresh committed 37-case execution",
                        "physical connection proofs use separate committed CLI runs",
                        "native-only state and GEO02/GEO03 algorithm closure remain outside this unit proof"],
    }
    for p, (state, _) in CACHE.items():
        require(fingerprint(p) == state, "Artifact changed before audit completed: " + str(p))
    target = out / "equivalence-report.json"
    target.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8", newline="\n")
    evidence = dict(report)
    evidence["raw_equivalence_report"] = ref(target)
    args.evidence.parent.mkdir(parents=True, exist_ok=True)
    args.evidence.write_text(json.dumps(evidence, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"status": "pass", "source_archive_byte_matches": 3637, "reused_unit_cases": 37,
                      "new_unit_cli_runs": 0, "report": ref(target), "evidence": ref(args.evidence)}))


def main():
    p = argparse.ArgumentParser(description=__doc__)
    for key in ["fixed-build", "committed-build", "matrix", "comparison", "provenance", "output-dir", "evidence"]:
        p.add_argument("--" + key, type=Path, required=True)
    audit(p.parse_args())


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, OSError) as error:
        print(str(error), file=sys.stderr)
        raise SystemExit(2)
