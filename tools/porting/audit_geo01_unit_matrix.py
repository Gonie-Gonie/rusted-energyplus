#!/usr/bin/env python3
"""Read-only provenance audit of GEO-01 ordinary-CLI dry-run collections.

This does not run Rust, C++, Cargo, Git or any scientific comparer. Coordinate
values are never compared or used as answers. Mathematical comparison status
remains pending regardless of this provenance result.
"""
from __future__ import annotations
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import shutil
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
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
    stat = p.stat()
    return stat.st_size, stat.st_mtime_ns


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
    require(type(v) is dict and type(v.get("path")) is str and type(v.get("sha256")) is str, "Malformed artifact binding")
    p = path(v["path"])
    require(sha(p) == v["sha256"], "Artifact hash differs: " + v["path"])
    return p


def exact(a, b):
    if type(a) is not type(b):
        return False
    if isinstance(a, dict):
        return a.keys() == b.keys() and all(exact(a[k], b[k]) for k in a)
    if isinstance(a, list):
        return len(a) == len(b) and all(exact(x, y) for x, y in zip(a, b))
    return a == b


def integer(v, label):
    require(type(v) is int, "Expected integer " + label)
    return v


def bind_all(v):
    if isinstance(v, dict):
        if "path" in v and "sha256" in v:
            binding(v)
        for child in v.values():
            bind_all(child)
    elif isinstance(v, list):
        for child in v:
            bind_all(child)


def audit(matrix_path, out):
    require(not out.exists(), "Use fresh audit output directory")
    matrix = read(matrix_path)
    require(matrix["schema"] == "geo01-rust-cli-matrix.v1" and matrix["kind"] == "unit", "Wrong unit matrix kind")
    require(matrix["original_outputs_supplied_to_Rust"] is False and matrix["original_first_completed_before_Rust"] is True, "Original-first/no-answer protocol differs")
    contracts = {k: ref(CONTRACTS / f"GEO-01-{k}.json") for k in ["source", "cases", "tolerances"]}
    require(exact(matrix["contracts"], contracts), "Unit collection did not bind current frozen contracts")
    original_matrix = read(binding(matrix["original_first_matrix"]))
    original_review = read(binding(matrix["original_first_review"]))
    require(original_review["status"] == "pass" and original_review["rust_compared"] is False
            and exact(original_review["original_matrix"], matrix["original_first_matrix"]), "Original-first preparation binding differs")
    frozen = read(CONTRACTS / "GEO-01-cases.json")
    require(integer(frozen["case_count"], "frozen count") == 37 and len(original_matrix["cases"]) == 37 and len(matrix["cases"]) == 37, "Complete 37 input rows required")
    frozen_by_id = {c["id"]: c for c in frozen["cases"]}
    require(len({c["case_id"] for c in matrix["cases"]}) == 37 and {c["case_id"] for c in matrix["cases"]} == set(frozen_by_id), "Unit case set differs")
    build = read(binding(matrix["build"]))
    binding(matrix["binary"])
    binding(matrix["executed_launcher"])
    require(exact(build["binary"], matrix["binary"]), "Archived build/binary differs")
    bind_all(build)
    built = read(binding(build["build_execution"]))
    require(integer(built["exit_code"], "Rust build exit") == 0, "Actual Rust build failed")
    require(exact(built["sources"], build["crates_sources"]), "Archived build sources differ from actual build execution")
    require(built["repository_head"] == build["repository_head"] and type(build["source_worktree_clean"]) is bool, "Build revision/state declaration differs")
    bind_all(built)
    require(exact(built["command"], ["cargo", "build", "-p", "ep_cli", "--bin", "eplus-rs", "-j", "2"])
            and path(built["cwd"]) == ROOT, "Actual build command/cwd differs from the frozen ordinary CLI recipe")
    exits = Counter()
    rows = []
    for item in matrix["cases"]:
        case = frozen_by_id[item["case_id"]]
        require(exact(item["input"], case["input"]) and exact(item["weather"], case["weather"])
                and exact(item["metadata"], case["metadata"]), "Frozen unit input/metadata differs")
        for key in ["input", "weather", "metadata"]:
            binding(item[key])
        require(type(item["runs"]) is list and len(item["runs"]) == 1, "Unit requires exactly one actual Full dry-run")
        run = item["runs"][0]
        require(run["trace_level"] == "full", "Unit trace level differs")
        execution = read(binding(run["execution"]))
        output = path(run["output_directory"])
        require(exact(execution["binary"], matrix["binary"]) and exact(execution["build"], matrix["build"])
                and exact(execution["executed_launcher"], matrix["executed_launcher"]), "Unit execution build/launcher differs")
        require(exact(execution["input"], case["input"]) and exact(execution["weather"], case["weather"]), "Unit execution input differs")
        require(execution["original_outputs_supplied_to_Rust"] is False and execution["dry_run"] is True and execution["trace_level"] == "full", "Unit execution protocol differs")
        require(path(execution["cwd"]) == ROOT and execution["repository_head"] == build["repository_head"], "Actual unit cwd/revision differs")
        command = execution["command"]
        expected = [str(path(matrix["binary"]["path"])), "run", str(path(case["input"]["path"])), "--weather",
                    str(path(case["weather"]["path"])), "--output-dir", str(output), "--mode", "compatibility",
                    "--partial", "deny", "--trace-level", "full", "--dry-run"]
        require(type(command) is list and len(command) == len(expected), "Unit CLI has unexpected extra flags/answer input")
        for i, (a, b) in enumerate(zip(command, expected)):
            require(path(a) == path(b) if i in (0, 2, 4, 6) else a == b, "Unit argv differs at index " + str(i))
        for key in ["stdout", "stderr"]:
            binding(execution[key])
        summary_path = binding(run["run_summary"])
        require(summary_path == output / "run-summary.json", "Run-summary output path differs")
        summary = read(summary_path)
        code = integer(execution["exit_code"], "unit exit")
        require(integer(summary["exit_code"], "summary exit") == code and code in (0, 4), "Unit actual exit is neither compiled-success nor declared unsupported")
        require(summary["message"] == "dry run completed" and summary["rust_runtime"] is None
                and summary["oracle"] is None and summary["comparison"] is None
                and summary["oracle_status"] == summary["compare_status"] == "skipped-dry-run", "Dry-run executed runtime/oracle/comparison")
        require(summary["config"]["dry_run"] is True and summary["config"]["trace_level"] == "full"
                and summary["config"]["oracle_baseline"] is False and summary["config"]["compare_oracle"] is False, "Dry-run/oracle configuration differs")
        require(summary["status"] == {0: "success", 4: "unsupported"}[code]
                and summary["input"]["kind"] == "idf", "Unit support/input status inconsistent")
        if summary["selected_algorithm_lane"] is not None:
            require(summary["selected_algorithm_lane"]["diagnostic_probe_used"] is False, "Diagnostic numerical probe used")
        artifact_paths = set()
        for artifact in run["artifacts"]:
            artifact_paths.add(binding(artifact))
        require(len(artifact_paths) == len(run["artifacts"]), "Duplicate collected artifact")
        require(set(p.resolve() for p in output.rglob("*") if p.is_file()) == artifact_paths, "Preserved output inventory differs from execution matrix")
        compiled_path = output / "compiled-geometry.json"
        require(compiled_path in artifact_paths, "Successful typed compile projection missing")
        compiled = read(compiled_path)
        require(compiled["schema"] == "compiled-geometry.v1" and compiled["physics_executed"] is False
                and compiled["preparation_only"] is True and compiled["observer_supplies_inputs"] is False, "Compiled projection boundary differs")
        require(compiled["inputs"]["source_assisted_lexical_conversion"] is True and compiled["inputs"]["conversion_is_physical_observation"] is False, "Input conversion boundary differs")
        require(not (output / "geometry-consumers.json").exists()
                and not any(p.is_file() for p in (output / "oracle").rglob("*"))
                and not any(p.is_file() for p in (output / "results").rglob("*")), "Dry-run contains physical/oracle result artifacts")
        for key in ["original", "converted_epjson"]:
            require(path(summary["input"][key]).is_file(), "Actual lexical input artifact missing")
        require(sha(path(summary["input"]["original"])) == case["input"]["sha256"], "Staged original IDF differs")
        require(path(summary["input"]["weather"]) == path(case["weather"]["path"]), "Summary weather differs")
        exits[code] += 1
        rows.append({"case_id": case["id"], "execution": run["execution"], "run_summary": run["run_summary"],
                     "compiled_geometry": ref(compiled_path), "artifact_count": len(artifact_paths), "actual_exit_code": code,
                     "physics_executed": False, "runtime_admission_claimed": False})
    out.mkdir(parents=True)
    archived = out / Path(__file__).name
    shutil.copyfile(Path(__file__), archived)
    report = {"schema": "geo01-unit-provenance-audit.v1", "status": "pass", "matrix": ref(matrix_path),
              "executed_auditor": ref(archived), "contracts": contracts, "binary": matrix["binary"], "build": matrix["build"],
              "case_count": 37, "cases": rows, "actual_exit_codes": dict(exits), "archived_Rust_source_count": len(build["crates_sources"]),
              "unique_hashed_artifacts": len(CACHE), "original_first_and_no_answer_protocol_checked": True,
              "actual_full_dry_run_commands_and_summaries_checked": True, "source_worktree_clean": build["source_worktree_clean"],
              "repository_head": build["repository_head"], "unit_mathematical_comparison": "pending-separate-report",
              "physics_executed": False, "runtime_admission_claimed": False, "gates_updated": False}
    for p, (state, _) in CACHE.items():
        require(fingerprint(p) == state, "Artifact changed before audit completed: " + str(p))
    target = out / "provenance-audit.json"
    target.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"status": "pass", "case_count": 37, "actual_exit_codes": dict(exits), "unique_hashed_artifacts": len(CACHE), "report": ref(target)}))


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--matrix", type=Path, required=True)
    p.add_argument("--output-dir", type=Path, required=True)
    args = p.parse_args()
    audit(args.matrix.resolve(), args.output_dir.resolve())


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, OSError) as error:
        print(str(error), file=sys.stderr)
        raise SystemExit(2)
