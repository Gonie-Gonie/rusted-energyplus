"""Execute and verify committed ordinary CLI PSY-01 production observations.

This check compares actual Full/Summary numerical artifacts and replays every
recorded kernel invocation through the pinned original C++ implementation. It
does not supply numerical operands/results to Rust or change card gates.
"""

from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import struct
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
EVIDENCE = ROOT / "energyplus_porting_plan/evidence/PSY-01"
CASES = ["A-24H", "B-NOLIMIT-24H", "B-FLOW-24H", "B-CAPACITY-24H", "B-BOTH-24H"]
EXECUTED_SOURCE = None


def read(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def write(path: Path, value: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, allow_nan=False) + "\n", encoding="utf-8", newline="\n")


def sha(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def ref(path: Path) -> dict:
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": sha(path)}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def execute(command: list[str], directory: Path, name: str) -> dict:
    stdout, stderr = directory / (name + "-stdout.log"), directory / (name + "-stderr.log")
    started = time.perf_counter()
    with stdout.open("wb") as out, stderr.open("wb") as err:
        completed = subprocess.run(command, cwd=ROOT, stdout=out, stderr=err, check=False)
    receipt = {"command": command, "cwd": ROOT.as_posix(), "exit_code": completed.returncode,
               "elapsed_seconds": round(time.perf_counter() - started, 3),
               "stdout": ref(stdout), "stderr": ref(stderr), "execution_repository_head": git("rev-parse", "HEAD")}
    receipt["orchestrator_source"] = ref(EXECUTED_SOURCE)
    write(directory / (name + "-execution.json"), receipt)
    require(completed.returncode == 0, f"{name} failed ({completed.returncode}); see {stderr}")
    return receipt


def trace_review(path: Path, is_b: bool) -> dict:
    trace = read(path)
    require(trace["schema"] == "psychrometrics-calls.v2", "Expected compact actual-production trace")
    rows, ordered = trace["dictionary"], trace["ordered_ids"]
    require(trace["omitted_call_count"] == 0 and trace["truncation_reason"] is None,
            "Truncated observations cannot prove complete-run helper replay")
    require(trace["complete_on_collecting_thread"], "Collecting-thread capture is incomplete")
    require(len(ordered) == trace["recorded_call_count"] == trace["total_call_count"], "Event counts differ")
    weights = Counter(ordered)
    require(all(isinstance(index, int) and 0 <= index < len(rows) for index in weights), "Invalid dictionary ID")
    observed = Counter()
    caller_counts, scope_counts = Counter(), Counter()
    physical_steps, validation_steps, system_steps = set(), set(), set()
    physical_tdb, other_tdb, returned = 0, 0, []
    first_cp = None
    humidity_ranges = {}
    for index, weight in weights.items():
        row = rows[index]
        routine = row["routine"]
        observed[routine] += weight
        caller = row["caller"]
        file = caller["file"].replace("\\", "/")
        context = row["context"]
        scope = context["scope"] if context else "unbound"
        caller_counts[f"{routine}|{file}:{caller['line']}|{row['phase']}|{scope}"] += weight
        scope_counts[scope] += weight
        if routine in ["PsyCpAirFnW", "PsyCpAirFnW_fast"]:
            require(isinstance(row["cp_cache"], dict), "Actual Cp state is missing")
            if routine == "PsyCpAirFnW" and first_cp is None:
                first_cp = row["cp_cache"]
        # These are actual kernel arguments, including any upstream wrapper floor.
        if routine != "PsyWFnTdbH":
            humidity = struct.unpack(">d", bytes.fromhex(row["input_bits"][-1]))[0]
            bounds = humidity_ranges.setdefault(routine, [humidity, humidity])
            bounds[0], bounds[1] = min(bounds[0], humidity), max(bounds[1], humidity)
        if context and context["zone_timestep"]:
            zone = context["zone_timestep"]
            sample = zone["sample_index"]
            require(zone["zone_steps_per_hour"] == 4 and sample == zone["hour_index"] * 4 + zone["zone_timestep"] - 1,
                    "Actual loop operands are not the 24-hour four-substep axis")
            require(0 <= sample < 96 and zone["timestep_seconds_bits"] == struct.pack(">d", 900.0).hex(),
                    "Unexpected observed zone interval")
            calendar = zone["calendar"]
            require(calendar == {"year": 2013, "month": 1, "day_of_month": 1, "day_of_sim": 1,
                                 "hour_ending": zone["hour_index"] + 1}, "Actual runtime calendar is inconsistent")
            if scope == "coupled_output_validation":
                validation_steps.add(sample)
            else:
                physical_steps.add(sample)
            if is_b:
                env = zone["environment"]
                require(env and env["environment_index"] == 1 and env["sample_index"] == sample
                        and env["simulation_timestep"] == sample + 1 and env["zone_timestep"] == zone["zone_timestep"],
                        "Existing B environment point is inconsistent")
                require(env["begin_environment"] == (sample == 0) and env["end_environment"] == (sample == 95),
                        "Existing B environment boundary flags differ")
            else:
                require(zone["environment"] is None, "A did not materialize a B environment axis")
            if context["system_call"]:
                system = context["system_call"]
                require(scope == "direct_zone_purchased_air_system_call" and system["schedule_sample_index"] == sample
                        and system["begin_environment"] == (sample == 0)
                        and system["timestep_seconds_bits"] == zone["timestep_seconds_bits"], "Actual system operands differ")
                system_steps.add(sample)
        if routine == "PsyTdbFnHW":
            if file.endswith("/ideal_loads/calc/no_oa.rs") and scope == "direct_zone_purchased_air_system_call":
                physical_tdb += weight
                returned.append({"sample_index": context["zone_timestep"]["sample_index"],
                                 "input_bits": row["input_bits"], "returned_temperature_bits": row["result_bits"],
                                 "returned_temperature_C": row["result"], "caller": caller, "event_count": weight})
            else:
                other_tdb += weight
    require(dict(observed) == trace["routine_counts"] and sum(observed.values()) == len(ordered), "Weighted kernel totals differ")
    require(physical_steps == set(range(96)), "Physical invocation contexts do not cover all 96 steps")
    if is_b:
        require(bool(system_steps) and system_steps <= set(range(96)) and validation_steps <= set(range(96)),
                "Observed B kernel scopes must bind valid real intervals")
    else:
        require(not system_steps and not validation_steps, "Unexpected B scopes in A")
    require(first_cp and first_cp["before"]["dw_save_bits"] == struct.pack(">d", -100.0).hex()
            and first_cp["before"]["cpa_save_bits"] == struct.pack(">d", -100.0).hex(), "First actual Cp call was not cold")
    last_physical, last_validation, seen_validation = -1, -1, False
    for index in ordered:
        context = rows[index]["context"]
        if not context or not context["zone_timestep"]:
            continue
        sample = context["zone_timestep"]["sample_index"]
        if context["scope"] == "coupled_output_validation":
            seen_validation = True
            require(sample >= last_validation, "Output validation cursor reversed")
            last_validation = sample
        else:
            require(not seen_validation and sample >= last_physical, "Physical context order reversed/interleaved with post-run validation")
            last_physical = sample
    returned.sort(key=lambda row: row["sample_index"])
    return {"trace": ref(path), "total_call_count": trace["total_call_count"], "dictionary_count": len(rows),
            "omitted_call_count": 0, "event_limit": trace["event_limit"], "unique_tuple_limit": trace["unique_tuple_limit"],
            "routine_counts": dict(observed), "scope_counts": dict(scope_counts), "caller_counts": dict(caller_counts),
            "kernel_argument_humidity_ranges": humidity_ranges, "physical_zone_steps": len(physical_steps),
            "actual_system_call_steps": len(system_steps), "referenced_output_validation_steps": len(validation_steps),
            "kernel_bearing_system_samples": sorted(system_steps), "kernel_bearing_validation_samples": sorted(validation_steps),
            "scope_coverage_boundary": "Only scopes containing actual psychrometric calls are observable here; zero-kernel invocations create no synthetic trace events. Actual coupling invocation count comes independently from the runtime summary.",
            "context_order_verified": True, "first_actual_normal_cp": first_cp,
            "physical_no_oa_tdb_call_count": physical_tdb, "other_transition_validation_tdb_call_count": other_tdb,
            "physical_return_samples": returned[:2] + returned[-2:] if len(returned) > 4 else returned,
            "return_consumption": "no_oa.rs assigns the actual kernel return to supply_temperature_c before the mixed-air min; dispatch produces supply_node_update/report and coupling derives actual system feedback",
            "return_consumer_source": [ref(path.parents[2] / "source/ideal_loads" / name)
                                       for name in ["calc/no_oa.rs", "dispatch.rs", "coupling.rs"]]}


def run_case(case: str, binary: Path, build: dict, output_root: Path, review_existing: bool) -> dict:
    directory = output_root / case
    require(directory.exists() if review_existing else not directory.exists(),
            f"Expected {'existing' if review_existing else 'fresh'} evidence directory: {directory}")
    directory.mkdir(parents=True, exist_ok=review_existing)
    metadata_path = ROOT / f"energyplus_porting_plan/cases/{case}/metadata.json"
    metadata = read(metadata_path)
    input_path, weather_path = ROOT / metadata["input"]["path"], ROOT / metadata["weather"]["path"]
    require(sha(input_path) == metadata["input"]["sha256"] and sha(weather_path) == metadata["weather"]["sha256"], "Case input/weather pin differs")
    require(metadata["expected_zone_steps_excluding_warmup"] == 96, "Case is not the 96-step contract")
    commands = []
    for level in ["full", "summary"]:
        print(f"{'REVIEW' if review_existing else 'RUN'} {case} {level}", flush=True)
        command = [str(binary), "run", str(input_path), "--weather", str(weather_path), "--output-dir", str(directory / level),
                   "--porting-scope", metadata["scope"], "--mode", "compatibility", "--partial", "deny", "--trace-level", level]
        if review_existing:
            receipt = read(directory / ("cli-" + level + "-execution.json"))
            require(receipt["command"] == command and receipt["exit_code"] == 0, "Existing ordinary CLI command differs")
            receipt.setdefault("orchestrator_source", ref(output_root / "source/check_psy01_production.before-scope-review.py"))
            require(ref(ROOT / receipt["stdout"]["path"]) == receipt["stdout"]
                    and ref(ROOT / receipt["stderr"]["path"]) == receipt["stderr"], "Original CLI raw logs changed")
            commands.append(receipt)
        else:
            commands.append(execute(command, directory, "cli-" + level))
        summary = read(directory / level / "run-summary.json")
        require(summary["status"] == "success" and summary["exit_code"] == 0 and summary["rust_runtime"]["samples"] == 24,
                "Normal CLI did not complete its declared 24-hour runtime")
        if metadata["scope"] == "B":
            require(summary["rust_runtime"]["purchased_air_coupling_call_count"] == 96, "B did not execute 96 actual coupling calls")
    full, summary = directory / "full", directory / "summary"
    require(not (summary / "psychrometrics-calls.json").exists(), "Default Summary unexpectedly captured kernels")
    equality = {}
    for name in ["selected-outputs.csv", "meters.csv", "result-store.json"]:
        left, right = full / "results" / name, summary / "results" / name
        equal = left.read_bytes() == right.read_bytes() if name.endswith("csv") else read(left)["series"] == read(right)["series"]
        require(equal, f"Full/Summary numerical difference in {case}/{name}")
        equality[name] = {"equal": True, "full": ref(left), "summary": ref(right),
                          "comparison": "byte-exact" if name.endswith("csv") else "complete result series JSON equality"}
    observation = trace_review(full / "psychrometrics-calls.json", metadata["scope"] == "B")
    print(f"TRACE {case}: events={observation['total_call_count']} unique={observation['dictionary_count']} physical_Tdb={observation['physical_no_oa_tdb_call_count']} omission=0 steps=96", flush=True)
    reference_path = directory / "replay/source-reference.json"
    if review_existing and reference_path.exists():
        receipt = read(directory / "original-replay-execution.json")
        require(receipt["exit_code"] == 0, "Existing original replay failed")
        receipt.setdefault("orchestrator_source", ref(output_root / "source/check_psy01_production.before-scope-review.py"))
        commands.append(receipt)
    else:
        commands.append(execute([sys.executable, str(ROOT / "tools/porting/psy01_reference.py"), "--production-trace",
                                 str(full / "psychrometrics-calls.json"), "--output-dir", str(directory / "replay")], directory, "original-replay"))
    reference = read(reference_path)
    comparison = reference["results"][0]["comparison"]
    require(reference["checks_passed"] and reference["comparisons_executed"] and comparison["checks_passed"]
            and comparison["mismatch_count"] == 0 and comparison["event_count"] == observation["total_call_count"]
            and comparison["ordered_input_identity_checked_every_event"], "Complete original C++ comparison failed")
    print(f"PASS {case}: all actual ordered calls compared; Full/Summary equal", flush=True)
    return {"schema": "psy01-production-case.v1", "case_id": case, "scope": metadata["scope"],
            "implementation_commit": build["implementation_commit"], "crates_tree": build["crates_tree"],
            "binary": ref(binary), "case_metadata": ref(metadata_path), "input": ref(input_path), "weather": ref(weather_path),
            "checker": ref(EXECUTED_SOURCE),
            "actual_coupling_invocations": read(full / "run-summary.json")["rust_runtime"]["purchased_air_coupling_call_count"] if metadata["scope"] == "B" else None,
            "commands": commands, "full_summary_numerical_equality": equality, "summary_kernel_artifact_absent": True,
            "observation": observation, "original_cpp_replay": {"receipt": ref(reference_path),
                "event_count": comparison["event_count"], "unique_pair_count": comparison["unique_pair_count"],
                "mismatch_count": comparison["mismatch_count"], "per_function": comparison["per_function"],
                "cache_context_order_checked": True}, "checks_passed": True,
            "claim": "conditional kernel replay on actual Rust arguments; physical pipeline and upstream EP caller parity remain unverified"}


def main() -> int:
    global EXECUTED_SOURCE
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--case", choices=CASES, help="Execute one fresh case; omit for all five serially")
    parser.add_argument("--review-existing", action="store_true", help="Review preserved ordinary CLI executions without rerunning Rust; replay C++ only if absent")
    args = parser.parse_args()
    build_path = EVIDENCE / "build-receipt.json"
    build = read(build_path)
    binary_row = next(row for row in build["artifacts"] if Path(row["path"]).name == "eplus-rs.exe")
    binary = ROOT / binary_row["path"]
    require(sha(binary) == binary_row["sha256"], "Archived committed CLI hash differs")
    require(git("rev-parse", "HEAD:crates") == build["crates_tree"], "Committed production crates tree differs")
    require(subprocess.run(["git", "diff", "--quiet", "HEAD", "--", "crates"], cwd=ROOT).returncode == 0, "Uncommitted production crates edits")
    output_root = ROOT / ".runtime/porting/PSY-01/production" / build["implementation_commit"][:12]
    EXECUTED_SOURCE = output_root / "source" / ("check_psy01_production." + sha(Path(__file__))[:12] + ".py")
    if not EXECUTED_SOURCE.exists():
        EXECUTED_SOURCE.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(Path(__file__), EXECUTED_SOURCE)
    require(sha(EXECUTED_SOURCE) == sha(Path(__file__)), "Executed checker archive differs")
    for name in ["calc/no_oa.rs", "dispatch.rs", "coupling.rs"]:
        source = ROOT / "crates/ep_runtime/src/ideal_loads" / name
        archive = output_root / "source/ideal_loads" / name
        if not archive.exists():
            archive.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, archive)
        require(sha(archive) == sha(source), "Archived committed consumer differs")
    for case in [args.case] if args.case else CASES:
        receipt = run_case(case, binary, build, output_root, args.review_existing)
        require(sha(binary) == binary_row["sha256"], "Archived CLI changed during execution")
        write(EVIDENCE / "production" / (case + ".json"), receipt)
    rows = []
    for case in CASES:
        path = EVIDENCE / "production" / (case + ".json")
        if path.exists():
            value = read(path)
            require(value["implementation_commit"] == build["implementation_commit"], "Stale production receipt")
            rows.append({"case_id": case, "receipt": ref(path), "actual_calls": value["observation"]["total_call_count"],
                         "actual_zone_steps": value["observation"]["physical_zone_steps"], "omitted_calls": value["observation"]["omitted_call_count"],
                         "physical_no_oa_tdb_calls": value["observation"]["physical_no_oa_tdb_call_count"],
                         "cpp_mismatches": value["original_cpp_replay"]["mismatch_count"], "checks_passed": value["checks_passed"]})
    write(EVIDENCE / "production-matrix.json", {"schema": "psy01-production-matrix.v1", "implementation_commit": build["implementation_commit"],
          "crates_tree": build["crates_tree"], "build_receipt": ref(build_path), "binary": ref(binary), "checker": ref(EXECUTED_SOURCE),
          "cases": rows, "complete_five_case_matrix": len(rows) == 5, "all_recorded_events_original_cpp_replayed": len(rows) == 5,
          "gates_updated": False, "limitations": ["actual Rust kernel argument replay only; upstream EP caller inputs/order unverified",
          "guarded density/Cp wrappers may normalize humidity before the recorded canonical kernel arguments",
          "copied Rust calendar/environment axes do not prove original EP system/source ordering, warmup or adaptive steps",
          "one collecting execution thread; original unsafe static races excluded", "physical heat-balance/HVAC/SYS gates remain separate",
          "fast variants can have zero genuine production calls; separate unit coverage does not manufacture production callers"]})
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
