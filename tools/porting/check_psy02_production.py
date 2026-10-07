"""Run immutable normal CLI cases and compare selected PSY-02 kernel roots.

Original results are a test reference only. This launcher never passes their
values to the Rust executable, mutates scientific inputs, or updates card gates.
"""

from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import struct
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
EVIDENCE = ROOT / "energyplus_porting_plan/evidence/PSY-02"
CASES = ["A-24H", "A-72H", "B-NOLIMIT-24H", "B-FLOW-24H",
         "B-CAPACITY-24H", "B-BOTH-24H", "B-BOTH-72H"]


def read(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def write(path: Path, value) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, allow_nan=False) + "\n",
                    encoding="utf-8", newline="\n")


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def same_metadata(left, right) -> bool:
    if type(left) is not type(right):
        return False
    if isinstance(left, dict):
        return left.keys() == right.keys() and all(same_metadata(value, right[key])
                                                   for key, value in left.items())
    if isinstance(left, list):
        return len(left) == len(right) and all(same_metadata(a, b) for a, b in zip(left, right))
    return left == right


def sha(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def ref(path: Path) -> dict:
    return {"path": path.resolve().relative_to(ROOT).as_posix(), "sha256": sha(path)}


def verify(binding: dict) -> Path:
    path = (ROOT / binding["path"]).resolve()
    require(path.is_relative_to(ROOT), "Artifact escapes repository")
    require(ref(path) == binding, f"Artifact changed: {binding['path']}")
    return path


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def execute(command: list[str], directory: Path, name: str, source: Path,
            review: bool) -> dict:
    receipt_path = directory / (name + "-execution.json")
    if review:
        receipt = read(receipt_path)
        require(receipt["command"] == command and receipt["exit_code"] == 0,
                "Preserved execution command or exit differs")
        for key in ["stdout", "stderr", "executed_checker"]:
            verify(receipt[key])
        return receipt
    require(not receipt_path.exists(), "Execution receipt already exists")
    stdout, stderr = directory / (name + "-stdout.log"), directory / (name + "-stderr.log")
    require(not stdout.exists() and not stderr.exists(), "Raw execution logs already exist")
    start = time.perf_counter()
    with stdout.open("wb") as out, stderr.open("wb") as err:
        result = subprocess.run(command, cwd=ROOT, stdout=out, stderr=err, check=False)
    receipt = {"command": command, "cwd": ROOT.as_posix(), "exit_code": result.returncode,
               "elapsed_seconds": round(time.perf_counter() - start, 3),
               "repository_head": git("rev-parse", "HEAD"), "stdout": ref(stdout),
               "stderr": ref(stderr), "executed_checker": ref(source)}
    write(receipt_path, receipt)
    require(result.returncode == 0, f"{name} failed; see preserved {stderr}")
    return receipt


def normal_result(directory: Path, metadata: dict) -> dict:
    summary = read(directory / "run-summary.json")
    require(summary["status"] == "success" and summary["exit_code"] == 0,
            "Normal CLI did not succeed")
    require(summary["rust_runtime"] is not None, "No physical Rust runtime executed")
    hours = metadata["expected_zone_steps_excluding_warmup"] // 4
    require(summary["rust_runtime"]["samples"] == hours, "Physical duration differs")
    require(summary["config"]["oracle_baseline"] is False
            and summary["config"]["compare_oracle"] is False
            and summary["oracle"] is None, "Unexpected oracle calculation")
    require(summary["config"]["dry_run"] is False, "Prepared rows are not physical runs")
    scope = read(directory / "porting_scope.json")
    require(scope["scope"] == metadata["scope"] and scope["admissible"] is True,
            "Admitted scope differs")
    require(all(scope["production"][key] is False for key in
                ["fixture_inputs_used", "oracle_inputs_used", "conformance_claim"]),
            "Unexpected production input source or conformance promotion")
    if metadata["scope"] == "B":
        require(summary["rust_runtime"]["fixture_demand_injection_used"] is False,
                "Fixture demand supplied to production")
        require(summary["rust_runtime"]["purchased_air_coupling_call_count"] == hours * 4,
                "Actual B coupling invocation count differs")
    return summary


def actual_intervals(directory: Path, metadata: dict, trace: dict) -> dict:
    """Bind copied kernel contexts to real Rust zone hooks, not native EP order."""
    clock_path = directory / "clock-calls.json"
    clock = read(clock_path)
    count = metadata["expected_zone_steps_excluding_warmup"]
    require(clock["schema"] == "clk01-clock-trace.v1", "Wrong actual clock schema")
    require(clock["complete_on_collecting_thread"] is True
            and clock["omitted_invocation_count"] == 0
            and clock["truncation_reason"] is None
            and clock["prepared_rows_are_executed_events"] is False,
            "Prepared or truncated rows cannot prove actual execution")
    events = clock["zone_invocations"]
    require(same_metadata(clock["total_invocation_count"], count)
            and same_metadata(clock["recorded_invocation_count"], count)
            and len(events) == count, "Actual zone invocation count differs")
    frames, points = clock["prepared_hourly_frames"], clock["prepared_environment_points"]
    scope_b = metadata["scope"] == "B"
    require(len(frames) == count // 4 and len(points) == (count if scope_b else 0),
            "Prepared axis lengths differ from actual intervals")
    require(clock["source_environment_number"] is None
            and same_metadata(clock["materialized_environment_index"], 1 if scope_b else None),
            "Materialized Rust index promoted to native environment ordinal")
    duration = struct.pack(">d", 900.0).hex()
    for index, event in enumerate(events):
        expected = {"sequence": index + 1, "hour_index": index // 4,
                    "calendar_frame_index": index // 4, "zone_timestep": index % 4 + 1,
                    "zone_steps_per_hour": 4, "timestep_seconds_bits": duration,
                    "environment_point_index": index if scope_b else None}
        require(all(type(event[key]) is type(value) and event[key] == value
                    for key, value in expected.items()), "Actual interval identity differs")
    require(trace["schema"] == "psy02-calls.v1"
            and trace["complete_on_collecting_thread"] is True
            and trace["omitted_root_count"] == 0 and trace["truncation_reason"] is None,
            "Incomplete selected-kernel root observations")
    ids, rows = trace["ordered_ids"], trace["dictionary"]
    require(len(ids) > 0, "Empty roots cannot establish selected-kernel production coverage")
    require(type(trace["total_root_count"]) is int
            and type(trace["recorded_root_count"]) is int
            and trace["total_root_count"] == trace["recorded_root_count"] == len(ids),
            "Actual kernel root count differs")
    require(all(type(i) is int and 0 <= i < len(rows) for i in ids), "Invalid typed root ID")
    weights = Counter(ids)
    require(set(weights) == set(range(len(rows))), "Unreferenced kernel dictionary row")
    first = {}
    for index, identifier in enumerate(ids):
        first.setdefault(identifier, index + 1)
    routines, context_scopes, phases, callers = Counter(), Counter(), Counter(), Counter()
    for identifier, row in enumerate(rows):
        require(type(row["sequence"]) is int and row["sequence"] == first[identifier],
                "Kernel dictionary first-occurrence sequence differs")
        routines[row["routine"]] += weights[identifier]
        context = row["context"]
        scope = context["scope"] if context else "unbound"
        context_scopes[scope] += weights[identifier]
        phases[row["phase"]] += weights[identifier]
        caller = row["caller"]
        require(type(row["routine"]) is str and type(row["phase"]) is str
                and type(scope) is str and type(caller["file"]) is str
                and type(caller["line"]) is int and type(caller["column"]) is int,
                "Kernel routine/phase/caller metadata has wrong types")
        callers[(row["routine"], row["phase"], scope, caller["file"], caller["line"], caller["column"])] += weights[identifier]
        if context is not None:
            require((context["system_call"] is not None)
                    == (context["scope"] == "direct_zone_purchased_air_system_call"),
                    "Kernel system-call presence differs from actual context scope")
        if context is None or context["zone_timestep"] is None:
            continue
        zone = context["zone_timestep"]
        index = zone["sample_index"]
        require(type(index) is int and 0 <= index < count, "Kernel context outside physical axis")
        event, frame = events[index], frames[index // 4]
        for key in ["hour_index", "zone_timestep", "zone_steps_per_hour", "timestep_seconds_bits"]:
            require(type(zone[key]) is type(event[key]) and zone[key] == event[key],
                    "Kernel zone operand differs from actual physical event")
        require(same_metadata(zone["calendar"], {key: frame[key] for key in
                ["year", "month", "day_of_month", "day_of_sim", "hour_ending"]}),
                "Kernel calendar differs from actual referenced frame")
        environment = None
        if scope_b:
            environment = dict(points[index])
            environment["environment_index"] = environment.pop("materialized_environment_index")
        require(same_metadata(zone["environment"], environment), "Kernel environment differs from actual axis")
        if context["system_call"] is not None:
            system = context["system_call"]
            require(type(system["schedule_sample_index"]) is int
                    and system["schedule_sample_index"] == index
                    and system["timestep_seconds_bits"] == duration
                    and system["begin_environment"] is (index == 0),
                    "Kernel system-call operands differ from actual interval")
    require(dict(routines) == trace["routine_counts"]
            and all(type(value) is int for value in trace["routine_counts"].values()),
            "Weighted actual root routine counts differ")
    return {"clock": ref(clock_path), "actual_zone_invocations": count,
            "kernel_contexts_bound_to_actual_intervals": True,
            "routine_counts": dict(routines), "context_scope_counts": dict(context_scopes),
            "phase_counts": dict(phases), "caller_counts": [
                {"routine": key[0], "phase": key[1], "scope": key[2],
                 "file": key[3], "line": key[4], "column": key[5], "root_count": value}
                for key, value in sorted(callers.items())],
            "scope_count_interpretation": "Actual Rust scopes include validation/recomputation; these are not native producer call counts.",
            "native_clock_or_manager_order_parity_claimed": False}


def run_case(case: str, build: dict, binary: Path, driver_build: Path, output_root: Path,
             source: Path, review: bool) -> dict:
    metadata_path = ROOT / f"energyplus_porting_plan/cases/{case}/metadata.json"
    metadata = read(metadata_path)
    input_path, weather = verify(metadata["input"]), verify(metadata["weather"])
    directory = output_root / case
    require(directory.is_dir() if review else not directory.exists(), "Expected existing/fresh case directory")
    directory.mkdir(parents=True, exist_ok=review)
    commands = []
    for level in ["full", "summary"]:
        print(f"{'REVIEW' if review else 'RUN'} {case} {level}", flush=True)
        command = [str(binary), "run", str(input_path), "--weather", str(weather),
                   "--output-dir", str(directory / level), "--porting-scope", metadata["scope"],
                   "--mode", "compatibility", "--partial", "deny", "--trace-level", level]
        commands.append(execute(command, directory, "cli-" + level, source, review))
        normal_result(directory / level, metadata)
    full, summary = directory / "full", directory / "summary"
    require(all(not (summary / name).exists() for name in
                ["psychrometrics-calls.json", "psy02-calls.json", "clock-calls.json"]),
            "Summary unexpectedly emitted an optional observer artifact")
    equality = {}
    for name in ["selected-outputs.csv", "meters.csv", "result-store.json"]:
        left, right = full / "results" / name, summary / "results" / name
        equal = left.read_bytes() == right.read_bytes() if name.endswith("csv") else same_metadata(read(left)["series"], read(right)["series"])
        require(equal, f"Full/Summary scientific output differs: {case}/{name}")
        equality[name] = {"equal": True, "full": ref(left), "summary": ref(right),
                          "comparison": "byte-exact" if name.endswith("csv") else "complete-series-JSON"}
    # The reference tool owns frozen-contract validation and ordered stateful
    # replay. It is called only after normal Rust execution has finished.
    trace = full / "psy02-calls.json"
    require(trace.is_file(), "PSY-02 root observation missing")
    trace_before = ref(trace)
    trace_data = read(trace)
    intervals = actual_intervals(full, metadata, trace_data)
    original_build = read(driver_build)
    original_binary = verify(original_build["binary"])
    verify(original_build["core_build"])
    command = [sys.executable, str(ROOT / "tools/porting/psy02_reference.py"),
               "--production-trace", str(trace), "--driver-build", str(driver_build),
               "--output-dir", str(directory / "replay")]
    commands.append(execute(command, directory, "original-replay", source, review))
    replay_path = directory / "replay/comparison-report.json"
    replay = read(replay_path)
    require(replay["status"] == "pass" and replay["mismatch_count"] == 0,
            "Ordered original kernel root replay failed")
    require(replay["ordered_root_inputs_checked"] and replay["final_cache_tables_checked"],
            "Reference replay omitted root order or final cache state")
    require(replay["omitted_root_count"] == 0, "Selected-root observations truncated")
    require(ref(trace) == trace_before == replay["trace"], "Replayed trace differs from actual Full trace")
    require(replay["native_driver_build"] == ref(driver_build)
            and replay["native_core_build"] == original_build["core_build"]
            and replay["original_binary"] == ref(original_binary), "Original peer provenance differs")
    require(replay["total_root_count"] == trace_data["total_root_count"]
            and replay["every_original_ordered_root_evaluated"] is True
            and replay["command_exit_code"] == 0, "Original replay did not execute every actual root")
    for key in ["original_stdout", "original_stderr", "executed_checker"]:
        verify(replay[key])
    require(replay["contracts"] == [ref(ROOT / "energyplus_porting_plan/contracts" / name)
            for name in ["PSY-02-source.json", "PSY-02-cases.json", "PSY-02-tolerances.json"]],
            "Replayed frozen contracts differ")
    return {"schema": "psy02-production-case.v1", "case_id": case, "scope": metadata["scope"],
            "implementation_commit": build["implementation_commit"], "crates_tree": build["crates_tree"],
            "binary": ref(binary), "metadata": ref(metadata_path), "input": ref(input_path),
            "weather": ref(weather), "checker": ref(source), "commands": commands,
            "original_driver_build": ref(driver_build),
            "expected_zone_invocations": metadata["expected_zone_steps_excluding_warmup"],
            "actual_interval_observations": intervals,
            "full_summary_equality": equality, "summary_observer_artifacts_absent": True,
            "trace": ref(trace), "original_replay": ref(replay_path), "status": "pass",
            "claim": "Selected actual Rust kernel root replay. Upstream weather/HVAC/SYS and full physics remain pending their cards."}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--case", choices=CASES, required=True)
    parser.add_argument("--original-driver-build", type=Path, required=True)
    parser.add_argument("--review-existing", action="store_true")
    args = parser.parse_args()
    build_path = EVIDENCE / "build-receipt.json"
    build = read(build_path)
    driver_build = args.original_driver_build.resolve()
    require(driver_build.is_relative_to(ROOT), "Original build receipt escapes repository")
    require(read(driver_build)["checks_passed"] is True, "Original driver build unverified")
    require(git("rev-parse", "HEAD:crates") == build["crates_tree"], "Committed crates differ from producer")
    require(subprocess.run(["git", "diff", "--quiet", "HEAD", "--", "crates"], cwd=ROOT).returncode == 0,
            "Uncommitted production source")
    binary_binding = next(row for row in build["artifacts"] if Path(row["path"]).name == "eplus-rs.exe")
    binary = verify(binary_binding)
    output_root = ROOT / ".runtime/porting/PSY-02/production" / build["implementation_commit"][:12]
    source = output_root / "source" / ("check_psy02_production." + sha(Path(__file__))[:12] + ".py")
    if not source.exists():
        source.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(Path(__file__), source)
    require(sha(source) == sha(Path(__file__)), "Executed checker archive differs")
    receipt = run_case(args.case, build, binary, driver_build, output_root, source, args.review_existing)
    verify(binary_binding)
    write(EVIDENCE / "production" / (args.case + ".json"), receipt)
    print(f"PASS {args.case}: normal CLI, Full/Summary and original selected-root replay", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
