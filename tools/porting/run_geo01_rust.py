#!/usr/bin/env python3
"""Execute frozen GEO inputs through an archived ordinary Rust CLI.

Only IDF and weather inputs are supplied. Original output is never read here.
Both scientific comparison and card gate changes are separate operations.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
PRODUCTION = ["A-24H", "A-72H", "B-NOLIMIT-24H", "B-FLOW-24H",
              "B-CAPACITY-24H", "B-BOTH-24H", "B-BOTH-72H"]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read(path):
    return Path(path).read_text(encoding="utf-8")


def ref(path):
    path = Path(path).resolve()
    require(path.is_relative_to(ROOT), "File outside repository")
    with path.open("rb") as stream:
        digest = hashlib.file_digest(stream, "sha256").hexdigest()
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": digest}


def verify(binding):
    require(type(binding) is dict and type(binding.get("path")) is str, "Invalid reference")
    path = ROOT / binding["path"]
    require(ref(path) == binding, "Changed reference: " + binding["path"])
    return path


def write(path, payload):
    Path(path).write_text(json.dumps(payload, indent=2, allow_nan=False) + "\n",
                          encoding="utf-8", newline="\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--kind", choices=["unit", "production"], required=True)
    parser.add_argument("--case", action="append", default=[])
    args = parser.parse_args()
    require(not args.output_dir.exists(), "Fresh output directory required")
    build_ref = ref(args.build)
    build = json.loads(read(args.build))
    require(build["schema"] == "geo01-Rust-build.v1", "Wrong build schema")
    binary = verify(build["binary"])
    build_execution = json.loads(read(verify(build["build_execution"])))
    require(type(build_execution["exit_code"]) is int and build_execution["exit_code"] == 0,
            "Rust CLI build failed")
    for binding in build["crates_sources"]:
        verify({k: binding[k] for k in ["path", "sha256"]})
    verify(build["cargo_lock"])
    verify(build["toolchain"])
    if args.kind == "production":
        require(build["source_worktree_clean"] is True and type(build.get("implementation_commit")) is str,
                "Production requires a committed source build")
    frozen = json.loads(read(CONTRACTS / "GEO-01-cases.json"))
    selected_ids = args.case or (PRODUCTION if args.kind == "production" else [c["id"] for c in frozen["cases"]])
    require(len(set(selected_ids)) == len(selected_ids), "Duplicate requested case")
    selected = [c for c in frozen["cases"] if c["id"] in selected_ids]
    require(len(selected) == len(selected_ids), "Unknown requested case")
    if args.kind == "production":
        require(all(c["kind"] == "con-production-input" for c in selected), "Diagnostic input is not a production case")
    args.output_dir.mkdir(parents=True)
    script = args.output_dir / "run_geo01_rust.executed.py"
    shutil.copyfile(Path(__file__), script)
    head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    matrix = {"schema": "geo01-rust-cli-matrix.v1", "kind": args.kind,
              "binary": build["binary"], "build": build_ref, "executed_launcher": ref(script),
              "contracts": {label: ref(CONTRACTS / f"GEO-01-{label}.json") for label in ["source", "cases", "tolerances"]},
              "original_outputs_supplied_to_Rust": False,
              "original_first_matrix": ref(ROOT / ".runtime/porting/GEO-01/original-first/matrix.json"),
              "original_first_review": ref(ROOT / ".runtime/porting/GEO-01/original-first-review/preparation-review.json"),
              "original_first_completed_before_Rust": True,
              "original_first_order_basis": "Root accepted the completed native matrix and its preparation review before invoking this Rust launcher; these bindings hash proof files without reading their numerical values",
              "cases": []}
    if "implementation_commit" in build:
        matrix["implementation_commit"] = build["implementation_commit"]
    for case in selected:
        input_path, weather = verify(case["input"]), verify(case["weather"])
        verify(case["metadata"])
        row = {key: case[key] for key in ["scope", "input", "weather", "metadata"]}
        row.update(case_id=case["id"], runs=[])
        for level in (["full"] if args.kind == "unit" else ["full", "summary"]):
            case_dir = args.output_dir / case["id"]
            case_dir.mkdir(exist_ok=True)
            output = case_dir / ("dry-run" if args.kind == "unit" else level)
            command = [str(binary), "run", str(input_path), "--weather", str(weather),
                       "--output-dir", str(output), "--mode", "compatibility", "--partial", "deny", "--trace-level", level]
            if args.kind == "unit":
                command.append("--dry-run")
            else:
                command += ["--porting-scope", case["scope"]]
            stdout = case_dir / f"{level}-stdout.log"
            stderr = case_dir / f"{level}-stderr.log"
            started = time.perf_counter()
            started_utc = datetime.now(timezone.utc).isoformat()
            with stdout.open("wb") as out, stderr.open("wb") as err:
                result = subprocess.run(command, cwd=ROOT, stdout=out, stderr=err)
            require(ref(binary) == build["binary"], "Rust executable changed during execution")
            execution = {"command": command, "cwd": ROOT.as_posix(), "repository_head": head,
                         "exit_code": result.returncode, "elapsed_seconds": time.perf_counter() - started,
                         "started_utc": started_utc, "finished_utc": datetime.now(timezone.utc).isoformat(),
                         "stdout": ref(stdout), "stderr": ref(stderr), "binary": build["binary"], "build": build_ref,
                         "input": case["input"], "weather": case["weather"], "executed_launcher": ref(script),
                         "original_outputs_supplied_to_Rust": False, "dry_run": args.kind == "unit", "trace_level": level}
            execution_path = case_dir / f"{level}-execution.json"
            write(execution_path, execution)
            allowed_exits = (0, 4) if args.kind == "unit" else (0,)
            require(result.returncode in allowed_exits, f"{case['id']} {level} failed; preserved execution receipt")
            summary_path = output / "run-summary.json"
            summary = json.loads(read(summary_path))
            require(type(summary["exit_code"]) is int and summary["exit_code"] == result.returncode
                    and summary["status"] == {0: "success", 4: "unsupported"}[result.returncode],
                    "Ordinary CLI status/exit disagrees with actual command")
            require(summary["config"]["dry_run"] is (args.kind == "unit") and summary["config"]["oracle_baseline"] is False
                    and summary["config"]["compare_oracle"] is False and summary["oracle"] is None,
                    "Unexpected dry-run/oracle production boundary")
            if args.kind == "unit":
                require(summary["rust_runtime"] is None, "Unit compile-only command ran physics")
                require(summary["message"] == "dry run completed", "Unit command did not complete ordinary preparation")
                projection = json.loads(read(output / "compiled-geometry.json"))
                require(projection["physics_executed"] is False and projection["preparation_only"] is True,
                        "Compile projection boundary differs")
            row["runs"].append({"trace_level": level, "output_directory": output.resolve().relative_to(ROOT).as_posix(),
                                "execution": ref(execution_path), "run_summary": ref(summary_path),
                                "artifacts": [ref(p) for p in sorted(output.rglob("*")) if p.is_file()]})
            matrix["cases"] = [*matrix["cases"], row] if level == "full" else matrix["cases"]
            write(args.output_dir / "matrix.json", matrix)
            print(f"{case['id']} {level} exit={result.returncode}", flush=True)
    require(ref(args.build) == build_ref, "Build receipt changed during execution")
    print(json.dumps({"matrix": ref(args.output_dir / "matrix.json"), "case_count": len(matrix["cases"]), "kind": args.kind}), flush=True)


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, KeyError) as error:
        print(str(error), file=sys.stderr)
        raise SystemExit(2)
