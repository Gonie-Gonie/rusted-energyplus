#!/usr/bin/env python3
"""Reference-only protocol check; never substitutes for Rust production proof."""
import os
from pathlib import Path
import sys

sys.dont_write_bytecode = True
import psy01_reference as reference


def main() -> None:
    reference.verify_pins()
    reference.frozen_contracts(check=True)
    root = reference.ROOT
    runtime = reference.DEFAULT_RUNTIME
    receipt = reference.read_json(runtime / "source-reference.json")
    exe = root / receipt["reference_build"]["binary"]["path"]
    for record in [receipt["reference_build"]["binary"], receipt["reference_build"]["source"]]:
        if reference.sha(root / record["path"]) != record["sha256"]:
            raise ValueError("Reference build/source changed; rerun psy01_reference.py first")
    compiler = root / receipt["reference_build"]["compiler"]["path"]
    env = os.environ.copy()
    env["PATH"] = str(compiler.parent) + os.pathsep + env.get("PATH", "")
    directory = runtime / "compression-check"
    directory.mkdir(parents=True, exist_ok=True)
    checks = []
    for process in receipt["results"]:
        name = process["process_id"]
        original_path = root / process["original_results"]["path"]
        if reference.sha(original_path) != process["original_results"]["sha256"]:
            raise ValueError("Original unit reference outputs changed")
        original = reference.read_json(original_path)
        dictionary, expected, ordered, intern = [], [], [], {}
        for row in original["calls"]:
            point = {"function": row["function"], "inputs": row["resolved_inputs"], "context": row["context"]}
            observed = {key: value for key, value in row.items() if key not in ["call_index", "sequence_id", "inputs"]}
            observed["inputs"] = row["resolved_inputs"]
            key = reference.data_bytes(observed)
            if key not in intern:
                index = len(dictionary)
                intern[key] = index
                dictionary.append(point)
                expected.append(dict(observed, input_id=index))
            ordered.append(intern[key])
        request, output = directory / f"{name}-input.json", directory / f"{name}-reference.json"
        reference.write_json(request, {"schema": "psy01-tuples.v2", "dictionary": dictionary, "ordered_ids": ordered})
        execution = reference.execute([str(exe), str(request), str(output), str(reference.DLL)], directory, name, env)
        comparison = reference.compare_compressed(reference.read_json(output), {"schema": "psy01-results.v2", "dictionary": expected, "ordered_ids": ordered})
        if not comparison["checks_passed"]:
            raise ValueError(comparison)
        checks.append({"process_id": name, "events": len(ordered), "comparison": comparison, "execution": execution,
                       "input": reference.ref(request), "output": reference.ref(output)})
    # A repeated input ID must retain multiple actual original cache states.
    request, output = directory / "variant-input.json", directory / "variant-reference.json"
    points = [{"function": "PsyCpAirFnW", "inputs": {"w_kg_per_kg": w}, "context": {"phase": "compression-only"}} for w in [0.008, 0.02]]
    order = [0, 0, 1, 0, 0, 1, 1]
    reference.write_json(request, {"schema": "psy01-tuples.v2", "dictionary": points, "ordered_ids": order})
    execution = reference.execute([str(exe), str(request), str(output), str(reference.DLL)], directory, "variants", env)
    result = reference.read_json(output)
    if len(result["dictionary"]) != 5 or result["executed_event_count"] != len(order):
        raise ValueError("Original cache-state variants were incorrectly skipped")
    if any(result["dictionary"][ref_id]["input_id"] != input_id for input_id, ref_id in zip(order, result["ordered_ids"])):
        raise ValueError("Ordered input identity changed")
    summary = {"schema": "psy01-source-compression-check.v1", "source": reference.ref(Path(__file__)),
               "reference_python_tool": reference.ref(Path(reference.__file__)),
               "reference_build": receipt["reference_build"]["binary"], "checks": checks,
               "unit_replayed_events": sum(check["events"] for check in checks),
               "state_variant_check": {"events": 7, "input_dictionary_count": 2, "original_output_variants": 5,
                                       "execution": execution, "input": reference.ref(request), "output": reference.ref(output)},
               "checks_passed": True, "rust_compared": False, "gates_updated": False,
               "meaning": "Reference-only ordered/cache-state protocol check; no Rust numerical or production gate claim"}
    reference.write_json(reference.EVIDENCE / "source-compression-check.json", summary)
    print({"unit_replay_events": summary["unit_replayed_events"], "zero_mismatches": True,
           "variant_events": 7, "original_cache_variants": 5, "rust_compared": False})


if __name__ == "__main__":
    main()
