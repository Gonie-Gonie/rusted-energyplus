#!/usr/bin/env python3
"""Pinned original-header PSY-01 reference, tuple replay and comparison.

Test-only: never changes Rust calculations or card gates. Raw builds and traces
remain in .runtime; contracts and source receipts are reviewable repository files.
"""
from __future__ import annotations

import argparse
from array import array
from collections import Counter
import hashlib
import json
import math
import os
from pathlib import Path
import struct
import subprocess
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / ".reference/energyplus-src/26.1.0"
CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
EVIDENCE = ROOT / "energyplus_porting_plan/evidence/PSY-01"
DEFAULT_RUNTIME = ROOT / ".runtime/porting/PSY-01"
PIN = "6f2e40d10250a105b49966baa24d843711e61048"
DLL = ROOT / ".runtime/energyplus/26.1.0/energyplusapi.dll"
DLL_HASH = "bbe6f66240d108df9f51a03b9beb968808a2173234253574d27a31605aec2aa2"
SOURCE_PINS = {
    "src/EnergyPlus/Psychrometrics.hh": "30c9575bc5a8e73d33d111e0d54a4da8916af4534175e9b95071aca2513aef45",
    "src/EnergyPlus/DataGlobalConstants.hh": "611963e974f18a83f81401aaeef2ff5393948d04d33421b1036fbcde35356cb3",
    "src/EnergyPlus/api/TypeDefs.h": "8fff8cf997e09d396357b0b63762ec7f532490b7ae0d1a3d0395359e627e5a32",
    "src/EnergyPlus/api/func.cc": "78cff481145ac70564101b102748d87da9bd1b964783d27891df162e3cd1e0c9",
    "src/EnergyPlus/api/state.cc": "ee6d3ef12048ea82bd1cb942f2bdb7755fe4a39cdcb91272a37aee742ad3b639",
    "src/EnergyPlus/EnergyPlus.hh": "e205dc8fe4cd11f23c0cb38757aaea2522c047e22d4416097a79c0d8c6d1f494",
    "src/EnergyPlus/PurchasedAirManager.cc": "54d960bcbfdf4f424a84ba73bf62040677424ad93e2f9362584898b0b146c005",
    "third_party/ObjexxFCL/src/ObjexxFCL/Fmath.hh": "36756879c587a6248ab8ccfd3ebf6d9ad34572f66ce9c874fc4973b3c070ef5c",
}
# Each entry is an insertion before this 1-based original source line.
OBSERVERS = [
    (701, "        ::psy01::observe_before(false, dwSave, cpaSave);\n"),
    (704, "            ::psy01::observe_after(false, dwSave, cpaSave, true);\n"),
    (715, "        ::psy01::observe_after(false, dwSave, cpaSave, false);\n"),
    (727, "        ::psy01::observe_before(true, dwSave, cpaSave);\n"),
    (730, "            ::psy01::observe_after(true, dwSave, cpaSave, true);\n"),
    (740, "        ::psy01::observe_after(true, dwSave, cpaSave, false);\n"),
]
FUNCTIONS = {
    "PsyCpAirFnW": ({"w_kg_per_kg"}, "J/(kg*K)", 1e-9),
    "PsyCpAirFnW_fast": ({"w_kg_per_kg"}, "J/(kg*K)", 1e-9),
    "PsyRhoAirFnPbTdbW": ({"p_pa", "t_db_c", "w_kg_per_kg"}, "kg/m3", 1e-12),
    "PsyRhoAirFnPbTdbW_fast": ({"p_pa", "t_db_c", "w_kg_per_kg"}, "kg/m3", 1e-12),
    "PsyHFnTdbW": ({"t_db_c", "w_kg_per_kg"}, "J/kg", 1e-7),
    "PsyHFnTdbW_fast": ({"t_db_c", "w_kg_per_kg"}, "J/kg", 1e-7),
    "PsyTdbFnHW": ({"h_j_per_kg", "w_kg_per_kg"}, "degC", 1e-10),
    "PsyWFnTdbH": ({"t_db_c", "h_j_per_kg"}, "kg/kg", 1e-12),
}


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def data_bytes(value: object) -> bytes:
    return (json.dumps(value, ensure_ascii=False, indent=2, allow_nan=False) + "\n").encode("utf-8")


def write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if isinstance(value, dict) and "ordered_ids" in value:
        # Stream the u32 sequence without building millions of JSON fragments.
        with path.open("w", encoding="utf-8", newline="\n") as output:
            output.write("{")
            for position, (key, item) in enumerate(value.items()):
                if position:
                    output.write(",")
                output.write(json.dumps(key) + ":")
                if key == "ordered_ids":
                    output.write("[")
                    for start in range(0, len(item), 8192):
                        if start:
                            output.write(",")
                        output.write(",".join(map(str, item[start:start + 8192])))
                    output.write("]")
                else:
                    json.dump(item, output, ensure_ascii=False, allow_nan=False, separators=(",", ":"))
            output.write("}\n")
        return
    path.write_bytes(data_bytes(value))


def ref(path: Path) -> dict:
    try:
        name = path.resolve().relative_to(ROOT).as_posix()
    except ValueError:
        name = str(path.resolve())
    return {"path": name, "sha256": sha(path)}


def read_json(path: Path) -> dict:
    with path.open(encoding="utf-8") as source:
        result = json.load(source)
    if isinstance(result, dict) and "ordered_ids" in result:
        if any(not isinstance(value, int) or isinstance(value, bool) or not 0 <= value <= 0xFFFFFFFF for value in result["ordered_ids"]):
            raise ValueError("ordered_ids must be exact u32 indices")
        result["ordered_ids"] = array("I", result["ordered_ids"])
    return result


def scalar(value: object) -> float:
    if isinstance(value, (int, float)) and not isinstance(value, bool):
        return float(value)
    if value in ("NaN", "+Infinity", "-Infinity"):
        return {"NaN": math.nan, "+Infinity": math.inf, "-Infinity": -math.inf}[value]
    raise ValueError(f"Invalid scalar: {value!r}")


def encode(value: float) -> float | str:
    if math.isnan(value):
        return "NaN"
    if math.isinf(value):
        return "+Infinity" if value > 0 else "-Infinity"
    return value


def original_copy() -> tuple[bytes, list[dict]]:
    original = (SOURCE / "src/EnergyPlus/Psychrometrics.hh").read_bytes()
    lines = original.splitlines(keepends=True)
    additions = dict(OBSERVERS)
    result = bytearray()
    patches = []
    for number, line in enumerate(lines, 1):
        if number in additions:
            insertion = additions[number].encode("utf-8")
            result.extend(insertion)
            patches.append({"operation": "insert_before_original_line", "line": number,
                            "original_line": line.decode("utf-8").rstrip("\n"),
                            "inserted_text": additions[number]})
        result.extend(line)
    # Replay removal to prove every original byte is unchanged.
    stripped = bytes(result)
    for _, insertion in OBSERVERS:
        if stripped.count(insertion.encode()) != 1:
            raise ValueError("Observer insertion isn't unique")
        stripped = stripped.replace(insertion.encode(), b"", 1)
    if stripped != original:
        raise ValueError("Instrumentation changed original source bytes")
    return bytes(result), patches


def source_contract() -> dict:
    selections = [
        ("src/EnergyPlus/Psychrometrics.hh", "PsyRhoAirFnPbTdbW state overload", 512, 547),
        ("src/EnergyPlus/Psychrometrics.hh", "PsyRhoAirFnPbTdbW constexpr overload", 549, 574),
        ("src/EnergyPlus/Psychrometrics.hh", "PsyRhoAirFnPbTdbW_fast", 576, 591),
        ("src/EnergyPlus/Psychrometrics.hh", "PsyHFnTdbW", 648, 666),
        ("src/EnergyPlus/Psychrometrics.hh", "PsyHFnTdbW_fast", 668, 677),
        ("src/EnergyPlus/Psychrometrics.hh", "PsyCpAirFnW", 679, 716),
        ("src/EnergyPlus/Psychrometrics.hh", "PsyCpAirFnW_fast", 718, 741),
        ("src/EnergyPlus/Psychrometrics.hh", "PsyTdbFnHW", 743, 762),
        ("src/EnergyPlus/Psychrometrics.hh", "PsyWFnTdbH induced consumer compatibility helper", 962, 998),
        ("src/EnergyPlus/PurchasedAirManager.cc", "positive-flow cooling supply H and sensible capacity inverse T", 2191, 2204),
        ("src/EnergyPlus/PurchasedAirManager.cc", "ConstantSensibleHeatRatio initial H-to-W consumer", 2213, 2227),
        ("src/EnergyPlus/PurchasedAirManager.cc", "dehumidifying capacity consumer control branches", 2263, 2306),
        ("src/EnergyPlus/PurchasedAirManager.cc", "post-saturation H-to-W-to-H consumer (upstream saturation excluded)", 2313, 2325),
        ("src/EnergyPlus/DataGlobalConstants.hh", "Constant::Kelvin", 611, 611),
        ("src/EnergyPlus/api/TypeDefs.h", "Real64", 52, 52),
        ("src/EnergyPlus/EnergyPlus.hh", "using ObjexxFCL::max", 165, 165),
        ("third_party/ObjexxFCL/src/ObjexxFCL/Fmath.hh", "max(double,double)", 338, 344),
        ("src/EnergyPlus/api/func.cc", "native API dispatcher", 165, 199),
        ("src/EnergyPlus/api/state.cc", "stateNew", 56, 60),
        ("src/EnergyPlus/api/state.cc", "stateDelete", 76, 79),
    ]
    rows = []
    for file, symbol, start, end in selections:
        selected = b"".join((SOURCE / file).read_bytes().splitlines(keepends=True)[start - 1:end])
        rows.append({"file": file, "symbol": symbol, "start_line": start, "end_line": end,
                     "range_sha256": hashlib.sha256(selected).hexdigest()})
    copied, patches = original_copy()
    return {
        "schema": "psy01-source-contract.v1", "energyplus_commit": PIN,
        "named_card_functions": ["PsyCpAirFnW", "PsyRhoAirFnPbTdbW", "PsyHFnTdbW", "PsyTdbFnHW"],
        "induced_integration_helpers": [{"function": "PsyWFnTdbH",
                                         "reason": "Existing H-to-W consumers must use original J-based expression and negative-result guard after canonical H delegation",
                                         "not_a_PSY_02_completion_claim": True,
                                         "caller_branches": "Source boundaries document existing consumers; CSHR/dehumidification remains outside CON-01 B admitted None controls"}],
        "source_files": [{"path": path, "sha256": digest} for path, digest in SOURCE_PINS.items()],
        "selected_ranges": rows,
        "reference": {"method": "full original header with read-only Cp cache observers",
                      "copy_sha256": hashlib.sha256(copied).hexdigest(), "patches": patches,
                      "original_byte_restoration_checked": True,
                      "state_provider": {"path": str(DLL.relative_to(ROOT)).replace("\\", "/"), "sha256": DLL_HASH,
                                         "exports": ["stateNew", "stateDelete"],
                                         "abi": "Opaque DLL-owned state; compiled rho/W bodies never dereference fields with EP_psych_errors and EP_psych_stats undefined"}},
        "numerical_contract": {
            "real_type": "IEEE-754 binary64 (original Real64 typedef double)",
            "humidity_floor_kg_per_kg": 1e-5, "kelvin_offset": 273.15,
            "density_constants": {"dry_air_gas_constant": 287.0, "water_air_mass_ratio": 1.6077687},
            "enthalpy_constants": {"dry_air_cp": 1004.84, "vapor_enthalpy_intercept": 2500940.0, "vapor_cp": 1858.95},
            "normal_humidity": "ObjexxFCL max: (a < b ? b : a); negative/zero W floors, NaN remains NaN",
            "fast_precondition": "W >= 1e-5 asserted; NaN and -Infinity fail; no pre-floor inserted by dispatcher",
            "induced_inverse_humidity": {"expression_grouping": "(H - 1004.84*T) / (2500940 + 1858.95*T)",
                                         "guard": "Only computed W<0 returns1e-5; zero, positive subfloor and NaN are returned unchanged",
                                         "mutable_state": "none with warning/statistics aggregation excluded"},
            "cp_state": {"normal_and_fast": "independent process-static last-input/last-output pairs",
                         "initial_dw_save": -100.0, "initial_cpa_save": -100.0,
                         "key": "raw W exact floating equality before floor; -0 == +0, NaN != NaN",
                         "cold_sentinel": "First normal W=-100 returns -100 before floor; after another W it computes floor Cp",
                         "timing": "read cache -> hit return, or compute -> save raw input/output -> return",
                         "reset": "fresh reference process only; sequence IDs don't reset state",
                         "thread_contract": "single bounded execution thread; original cross-thread data race outside scope"},
        },
        "native_api": {"rho": "fast", "enthalpy": "fast", "cp": "normal", "inverse_temperature": "normal",
                       "negative_or_subfloor_rho_h_not_valid_reference": True},
        "production_replay": {"input_schema": "psy01-tuples.v2", "output_schema": "psy01-results.v2",
                              "storage": "lossless dictionaries and ordered u32 indices; event sequence=index+1",
                              "execution": "Every ordered event invokes the original body; dictionary duplicates/cache hits are never skipped",
                              "reference_intern_key": "source input ID plus computed result bits and actual before/after Cp cache bits/hit",
                              "state": "no from_call or per-event previous-output map in production; original caches persist",
                              "comparison": "exact ordered input identity; unique actual/reference pair counts weight errors/RMSE by real event repetitions",
                              "unit_protocol_unchanged": "394 calls, psy01-tuples.v1/psy01-results.v1"},
        "exclusions": ["file I/O/warning/statistics aggregation/EP_psych_errors/EP_psych_stats", "PSY-02 saturation/relative humidity helpers",
                       "invalid fast arguments", "production scope admission of arbitrary IEEE/helper-domain probes",
                       "cross-thread original static cache behavior"],
    }


def cases_contract() -> dict:
    processes = []
    calls = []

    def add(sequence: str, function: str, inputs: dict, label: str) -> int:
        index = len(calls)
        calls.append({"sequence_id": sequence, "call_index": index, "function": function, "inputs": inputs,
                      "context": {"caller": "PSY-01.unit", "phase": "unit", "case_id": label}})
        return index

    def state(sequence: str, t: float, w: float, p: float, label: str) -> None:
        add(sequence, "PsyCpAirFnW", {"w_kg_per_kg": w}, label)
        add(sequence, "PsyRhoAirFnPbTdbW", {"p_pa": p, "t_db_c": t, "w_kg_per_kg": w}, label)
        h = add(sequence, "PsyHFnTdbW", {"t_db_c": t, "w_kg_per_kg": w}, label)
        add(sequence, "PsyTdbFnHW", {"h_j_per_kg": {"from_call": h}, "w_kg_per_kg": w}, label)
        if w >= 1e-5:
            add(sequence, "PsyCpAirFnW_fast", {"w_kg_per_kg": w}, label)
            add(sequence, "PsyRhoAirFnPbTdbW_fast", {"p_pa": p, "t_db_c": t, "w_kg_per_kg": w}, label)
            h = add(sequence, "PsyHFnTdbW_fast", {"t_db_c": t, "w_kg_per_kg": w}, label)
            add(sequence, "PsyTdbFnHW", {"h_j_per_kg": {"from_call": h}, "w_kg_per_kg": w}, label)

    # Establish physical Cp before testing negative W, independently of cold process.
    state("physical-first", 20.0, 0.008, 101325.0, "physical-first")
    temperatures = [-40.0, -1e-9, -0.0, 1e-9, 20.0, 60.0]
    humidities = [-0.005, 0.0, math.nextafter(1e-5, -math.inf), 1e-5, math.nextafter(1e-5, math.inf), 0.008, 0.03]
    for ti, t in enumerate(temperatures):
        for wi, w in enumerate(humidities):
            state("temperature-humidity-grid", t, w, 101325.0, f"grid-t{ti}-w{wi}")
    for index, p in enumerate([75000.0, 84300.0, 101325.0, 110000.0]):
        state("pressure-grid", 26.0, 0.012, p, f"pressure-{index}")
    for w in [-0.005, 1e-5, 0.008]:
        for h in [-100000.0, 0.0, 1e-9, 30000.0, 1000000.0]:
            add("independent-inverse", "PsyTdbFnHW", {"h_j_per_kg": h, "w_kg_per_kg": w}, f"inverse-{w}-{h}")
    for w in [0.008, 0.008, -0.0, 0.0, -100.0, -100.0, -0.005, -0.005, "NaN", "NaN", "+Infinity", "+Infinity"]:
        add("normal-cp-cache-sequence", "PsyCpAirFnW", {"w_kg_per_kg": w}, "normal-cp-cache")
    for w in [0.008, 0.008, 1e-5, 1e-5, "+Infinity", "+Infinity", 0.008]:
        add("fast-cp-cache-sequence", "PsyCpAirFnW_fast", {"w_kg_per_kg": w}, "fast-cp-cache")
    ieee = [
        ("PsyRhoAirFnPbTdbW", {"p_pa": 0.0, "t_db_c": 20.0, "w_kg_per_kg": 0.008}),
        ("PsyRhoAirFnPbTdbW", {"p_pa": -0.0, "t_db_c": 20.0, "w_kg_per_kg": 0.008}),
        ("PsyRhoAirFnPbTdbW", {"p_pa": -100.0, "t_db_c": 20.0, "w_kg_per_kg": 0.008}),
        ("PsyRhoAirFnPbTdbW", {"p_pa": 101325.0, "t_db_c": -273.15, "w_kg_per_kg": 0.008}),
        ("PsyRhoAirFnPbTdbW", {"p_pa": 101325.0, "t_db_c": 20.0, "w_kg_per_kg": "NaN"}),
        ("PsyRhoAirFnPbTdbW", {"p_pa": 101325.0, "t_db_c": 20.0, "w_kg_per_kg": "-Infinity"}),
        ("PsyHFnTdbW", {"t_db_c": 0.0, "w_kg_per_kg": "NaN"}),
        ("PsyHFnTdbW", {"t_db_c": 0.0, "w_kg_per_kg": "-Infinity"}),
        ("PsyHFnTdbW_fast", {"t_db_c": 0.0, "w_kg_per_kg": "+Infinity"}),
        ("PsyTdbFnHW", {"h_j_per_kg": "NaN", "w_kg_per_kg": 0.008}),
        ("PsyTdbFnHW", {"h_j_per_kg": 0.0, "w_kg_per_kg": "NaN"}),
        ("PsyTdbFnHW", {"h_j_per_kg": "+Infinity", "w_kg_per_kg": 0.008}),
    ]
    for index, (function, inputs) in enumerate(ieee):
        add("ieee-source-semantics", function, inputs, f"ieee-{index}")
    # Induced integration correction only: the existing consumer must invert
    # canonical original enthalpy with original PsyWFnTdbH grouping and guard.
    inverse_humidity_states = [(-20.0, -0.005), (-1e-9, 0.0), (0.0, 1e-5), (14.0, 0.008),
                               (20.0, 0.008), (26.0, 0.03), (60.0, 0.001), (20.0, math.nextafter(1e-5, -math.inf))]
    for index, (t, w) in enumerate(inverse_humidity_states):
        h = add("induced-H-to-W-consumer", "PsyHFnTdbW", {"t_db_c": t, "w_kg_per_kg": w}, f"H-to-W-{index}")
        add("induced-H-to-W-consumer", "PsyWFnTdbH", {"t_db_c": t, "h_j_per_kg": {"from_call": h}}, f"H-to-W-{index}")
    independent_humidity_inputs = [(0.0, h) for h in [-1e-9, -0.0, 0.0, 1e-9, 1.0, 25.0094, 100000.0]]
    independent_humidity_inputs += [(20.0, h) for h in [math.nextafter(20096.8, -math.inf), 20096.8,
                                                        math.nextafter(20096.8, math.inf), 0.0, 1000000.0]]
    independent_humidity_inputs += [(20.0, h) for h in ["NaN", "+Infinity", "-Infinity"]]
    for index, (t, h) in enumerate(independent_humidity_inputs):
        add("induced-W-guard-boundaries", "PsyWFnTdbH", {"t_db_c": t, "h_j_per_kg": h}, f"W-guard-{index}")
    processes.append({"id": "normal-boundary-and-ieee", "tuples": {"schema": "psy01-tuples.v1", "calls": calls}})
    calls = []
    for w in [-100.0, -100.0, 0.008, -100.0, -100.0, -0.0, 0.0, "NaN", "NaN"]:
        add("cold-normal-cp-cache", "PsyCpAirFnW", {"w_kg_per_kg": w}, "cold-cp-sentinel")
    for w in [0.008, 0.008, 1e-5, 1e-5]:
        add("independent-cold-fast-cp-cache", "PsyCpAirFnW_fast", {"w_kg_per_kg": w}, "cold-fast-cp-cache")
    processes.append({"id": "cold-cp-sentinel", "tuples": {"schema": "psy01-tuples.v1", "calls": calls}})
    return {"schema": "psy01-cases.v1", "card": "PSY-01", "scope": ["A", "B"],
            "contract_kind": "helper unit tuples; does not admit new production inputs",
            "process_reset_policy": "Each process is fresh; sequence IDs within a process preserve state",
            "processes": processes, "call_count": sum(len(p["tuples"]["calls"]) for p in processes),
            "coverage": ["dry/moist", "zero C +/-", "negative/zero/subfloor/floor/nextafter W", "pressure changes",
                         "own-result h-to-T chaining", "independent inverse enthalpy", "Cp cache hits/misses/cold sentinel",
                         "signed zero and IEEE source classifications", "induced own-result H-to-W compatibility and W<0 guard neighbors"],
            "named_card_scope_unchanged": True, "PSY_02_gates_updated": False, "gates_updated": False}


def tolerance_contract() -> dict:
    return {"schema": "psy01-tolerances.v1", "frozen_before_comparison": True,
            "rows": [{"function": name, "unit": unit, "atol": atol, "rtol": 1e-13}
                     for name, (_, unit, atol) in FUNCTIONS.items()],
            "policy": {"classification": "exact", "zero_sign": "exact when reference and candidate are zero",
                       "nan_payload_bits": "recorded; not required equal", "identity_inputs_context_unit_order_count": "exact",
                       "cache_hit": "exact", "cache_dw_save": "exact bits except NaN payload",
                       "cache_cpa_save": "Cp value tolerance/classification plus exact zero sign",
                       "from_call_resolved_h": "enthalpy tolerance; each implementation consumes its own earlier result",
                       "roundtrip_temperature_atol_degC": 1e-10,
                       "nonfinite_helper_probes": "class comparison only, no physical-validity claim"}}


def verify_pins() -> None:
    for path, expected in SOURCE_PINS.items():
        if sha(SOURCE / path) != expected:
            raise ValueError(f"Original source pin mismatch: {path}")
    if sha(DLL) != DLL_HASH:
        raise ValueError("Original DLL pin mismatch")


def frozen_contracts(check: bool) -> list[dict]:
    artifacts = {"PSY-01-source.json": source_contract(), "PSY-01-cases.json": cases_contract(),
                 "PSY-01-tolerances.json": tolerance_contract()}
    for name, value in artifacts.items():
        path = CONTRACTS / name
        content = data_bytes(value)
        if check:
            if not path.exists() or path.read_bytes() != content:
                raise ValueError(f"Frozen contract mismatch: {name}; review --prepare changes before comparison")
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(content)
    return [ref(CONTRACTS / name) for name in artifacts]


def execute(command: list[str], directory: Path, stem: str, env: dict | None = None) -> dict:
    completed = subprocess.run(command, cwd=ROOT, env=env, capture_output=True, check=False)
    write_json(directory / f"{stem}-command.json", command)
    (directory / f"{stem}-stdout.log").write_bytes(completed.stdout)
    (directory / f"{stem}-stderr.log").write_bytes(completed.stderr)
    receipt = {"command": command, "exit_code": completed.returncode,
               "artifacts": [ref(directory / f"{stem}-{suffix}") for suffix in ["command.json", "stdout.log", "stderr.log"]]}
    if completed.returncode != 0:
        raise RuntimeError(f"Command failed ({completed.returncode}): {command}; see {directory / (stem + '-stderr.log')}")
    return receipt


def build_reference(directory: Path) -> tuple[Path, dict, dict]:
    tools_manifest = read_json(ROOT / "tools/porting/reference_tools.json")
    llvm = next(t for t in tools_manifest["tools"] if t["name"] == "llvm-mingw")
    compiler = ROOT / ".runtime/reference-tools" / f"llvm-mingw-{llvm['version']}" / llvm["executable"]
    copied, _ = original_copy()
    header = directory / "psy01_original_instrumented.hh"
    header.write_bytes(copied)
    exe = directory / "psy01_reference.exe"
    command = [str(compiler), "-std=c++20", "-O0", "-ffp-contract=off", "-UEP_psych_errors", "-UEP_psych_stats", "-UNDEBUG",
               "-MMD", "-MF", str(directory / "reference.d"), "-I" + str(directory)]
    for include in ["src", "third_party", "third_party/ObjexxFCL/src", "third_party/fmt-8.0.1/include", "third_party/valijson/include"]:
        command.append("-I" + str(SOURCE / include))
    command += [str(ROOT / "tools/porting/psy01_reference.cpp"), "-o", str(exe)]
    env = os.environ.copy()
    env["PATH"] = str(compiler.parent) + os.pathsep + env.get("PATH", "")
    receipt = execute(command, directory, "build", env)
    dependency_text = (directory / "reference.d").read_text(encoding="utf-8").replace("\\\n", "")
    dependency_paths = dependency_text.split(": ", 1)[1].split()
    dependency_manifest = directory / "reference-dependencies.json"
    write_json(dependency_manifest, [ref(Path(path)) for path in dependency_paths])
    receipt.update({"compiler": ref(compiler), "tool_manifest": ref(ROOT / "tools/porting/reference_tools.json"),
                    "source": ref(ROOT / "tools/porting/psy01_reference.cpp"), "instrumented_original_header": ref(header),
                    "dependency_file": ref(directory / "reference.d"), "binary": ref(exe),
                    "dependency_manifest": ref(dependency_manifest), "dependency_count": len(dependency_paths),
                    "flags": {"assertions_enabled": True, "EP_psych_errors": False, "EP_psych_stats": False, "fp_contraction": "off", "optimization": "O0"}})
    return exe, receipt, env


def validate_tuples(tuples: dict) -> None:
    if tuples.get("schema") == "psy01-tuples.v2":
        dictionary = tuples["dictionary"]
        if any(index >= len(dictionary) for index in tuples["ordered_ids"]):
            raise ValueError("Compressed input dictionary ID out of range")
        for index, row in enumerate(dictionary):
            if any(isinstance(value, dict) for value in row["inputs"].values()):
                raise ValueError("Compressed production input cannot use from_call")
            validate_tuples({"schema": "psy01-tuples.v1", "calls": [dict(row, call_index=index)]})
        return
    if tuples.get("schema") != "psy01-tuples.v1" or not isinstance(tuples.get("calls"), list):
        raise ValueError("Invalid tuple schema")
    previous = {}
    for call in tuples["calls"]:
        index = call["call_index"]
        function = call["function"]
        if not isinstance(index, int) or index in previous or function not in FUNCTIONS:
            raise ValueError("Invalid/duplicate call index or function")
        inputs = call["inputs"]
        if set(inputs) != FUNCTIONS[function][0]:
            raise ValueError("Incorrect function input fields")
        for key, value in inputs.items():
            if isinstance(value, dict):
                dependency = value.get("from_call")
                if key != "h_j_per_kg" or set(value) != {"from_call"} or previous.get(dependency) not in ["PsyHFnTdbW", "PsyHFnTdbW_fast"]:
                    raise ValueError("from_call must point to an earlier enthalpy call")
            else:
                scalar(value)
        if function.endswith("_fast") and not scalar(inputs["w_kg_per_kg"]) >= 1e-5:
            raise ValueError("Fast inputs require W >= 1e-5")
        previous[index] = function


def compare(reference: dict, candidate: dict) -> dict:
    if reference.get("schema") != "psy01-results.v1" or candidate.get("schema") != "psy01-results.v1":
        raise ValueError("Result schema mismatch")
    left, right = reference["calls"], candidate["calls"]
    failures = []
    errors = {name: {"count": 0, "finite_count": 0, "max_abs_error": 0.0, "sum_square_error": 0.0} for name in FUNCTIONS}
    if len(left) != len(right):
        failures.append({"kind": "count", "reference": len(left), "candidate": len(right)})

    def numeric(a: object, b: object, atol: float, rtol: float) -> tuple[bool, float]:
        av, bv = scalar(a), scalar(b)
        if math.isnan(av) or math.isnan(bv):
            return math.isnan(av) and math.isnan(bv), 0.0
        if math.isinf(av) or math.isinf(bv):
            return av == bv, 0.0
        error = abs(av - bv)
        zero_sign_ok = not (av == bv == 0.0) or math.copysign(1.0, av) == math.copysign(1.0, bv)
        return error <= atol + rtol * abs(av) and zero_sign_ok, error

    for a, b in zip(left, right):
        index = a["call_index"]
        function = a["function"]
        _, unit, atol = FUNCTIONS[function]
        for key in ["sequence_id", "call_index", "function", "inputs", "context", "unit", "value_class"]:
            if a.get(key) != b.get(key):
                failures.append({"call_index": index, "field": key, "reference": a.get(key), "candidate": b.get(key)})
        if a["unit"] != unit:
            raise ValueError("Reference units don't match frozen contract")
        good, error = numeric(a["value"], b["value"], atol, 1e-13)
        row = errors[function]
        row["count"] += 1
        if a["value_class"] == b["value_class"] == "finite":
            row["finite_count"] += 1
            row["max_abs_error"] = max(row["max_abs_error"], error)
            row["sum_square_error"] += error * error
        if not good:
            failures.append({"call_index": index, "field": "value", "reference": a["value"], "candidate": b["value"], "abs_error": encode(error)})
        if set(a["resolved_inputs"]) != set(b["resolved_inputs"]):
            failures.append({"call_index": index, "field": "resolved_inputs.keys"})
        else:
            for key, av in a["resolved_inputs"].items():
                chained = isinstance(a["inputs"][key], dict)
                good, _ = numeric(av, b["resolved_inputs"][key], 1e-7 if chained else 0.0, 1e-13 if chained else 0.0)
                if not good:
                    failures.append({"call_index": index, "field": "resolved_inputs." + key})
        if function.startswith("PsyCpAirFnW"):
            if a["cache_hit"] != b.get("cache_hit"):
                failures.append({"call_index": index, "field": "cache_hit"})
            for phase in ["cache_before", "cache_after"]:
                for key in ["dw_save", "cpa_save"]:
                    if phase not in b or key not in b[phase]:
                        failures.append({"call_index": index, "field": phase + "." + key})
                        continue
                    good, _ = numeric(a[phase][key], b[phase][key], 0.0 if key == "dw_save" else atol, 0.0 if key == "dw_save" else 1e-13)
                    if key == "dw_save" and not math.isnan(scalar(a[phase][key])):
                        good = good and a[phase][key + "_bits"] == b[phase].get(key + "_bits")
                    if not good:
                        failures.append({"call_index": index, "field": phase + "." + key})
    for row in errors.values():
        row["rmse"] = math.sqrt(row.pop("sum_square_error") / row["finite_count"]) if row["finite_count"] else 0.0
    return {"call_count": len(left), "mismatch_count": len(failures), "failures": failures,
            "per_function": errors, "checks_passed": not failures}


def production_tuples(trace: dict) -> tuple[dict, dict, dict]:
    if trace.get("schema") != "psychrometrics-calls.v1":
        raise ValueError("Production trace schema mismatch")
    calls = []
    actual_results = []
    order = {"PsyCpAirFnW": ["w_kg_per_kg"], "PsyRhoAirFnPbTdbW": ["p_pa", "t_db_c", "w_kg_per_kg"],
             "PsyHFnTdbW": ["t_db_c", "w_kg_per_kg"], "PsyTdbFnHW": ["h_j_per_kg", "w_kg_per_kg"],
             "PsyWFnTdbH": ["t_db_c", "h_j_per_kg"]}
    for row in trace["calls"]:
        function = row["routine"]
        base = function.removesuffix("_fast")
        keys = order[base]
        if len(keys) != len(row["input_bits"]):
            raise ValueError("Production input bit-vector length mismatch")
        inputs = {key: encode(struct.unpack(">d", bytes.fromhex(bits))[0]) for key, bits in zip(keys, row["input_bits"])}
        call = {"sequence_id": "production", "call_index": row["sequence"], "function": function, "inputs": inputs,
                "context": {"phase": row["phase"], "caller": row["caller"], "production_context": row.get("context")}}
        calls.append(call)
        value = struct.unpack(">d", bytes.fromhex(row["result_bits"]))[0]
        result = dict(call)
        result.update({"resolved_inputs": inputs, "value": encode(value), "value_bits": row["result_bits"],
                       "value_class": "nan" if math.isnan(value) else ("+infinity" if value == math.inf else ("-infinity" if value == -math.inf else "finite")),
                       "unit": FUNCTIONS[function][1]})
        if function.startswith("PsyCpAirFnW"):
            cache = row["cp_cache"]
            if not isinstance(cache, dict):
                raise ValueError("Actual Cp production trace lacks cache state")
            for phase in ["before", "after"]:
                observed = dict(cache[phase])
                for key in ["dw_save", "cpa_save"]:
                    observed[key] = encode(struct.unpack(">d", bytes.fromhex(observed[key + "_bits"]))[0])
                result["cache_" + phase] = observed
            result["cache_hit"] = cache["hit"]
        actual_results.append(result)
    if len(calls) != trace["recorded_call_count"]:
        raise ValueError("Production recorded count mismatch")
    if trace["total_call_count"] != trace["recorded_call_count"] + trace["omitted_call_count"]:
        raise ValueError("Production total/recorded/omitted counts inconsistent")
    if sum(trace["routine_counts"].values()) != trace["total_call_count"]:
        raise ValueError("Production routine counts inconsistent")
    if any(a["call_index"] >= b["call_index"] for a, b in zip(calls, calls[1:])):
        raise ValueError("Production call sequence not strictly increasing")
    tuples = {"schema": "psy01-tuples.v1", "calls": calls}
    receipt = {key: trace[key] for key in ["total_call_count", "recorded_call_count", "omitted_call_count", "record_limit", "routine_counts"]}
    receipt["complete_call_trace"] = trace["omitted_call_count"] == 0
    receipt["scope"] = "recorded prefix only; omitted calls are not compared"
    receipt["capture_source"] = trace["capture_source"]
    receipt["input_origin"] = trace["input_origin"]
    receipt["thread_coverage"] = trace["thread_coverage"]
    receipt["phase_contract"] = trace["phase_contract"]
    return tuples, receipt, {"schema": "psy01-results.v1", "calls": actual_results}


def production_compressed(trace: dict) -> tuple[dict, dict, dict]:
    if trace.get("schema") != "psychrometrics-calls.v2":
        raise ValueError("Compressed production schema mismatch")
    dictionary, results = [], []
    order = {"PsyCpAirFnW": ["w_kg_per_kg"], "PsyRhoAirFnPbTdbW": ["p_pa", "t_db_c", "w_kg_per_kg"],
             "PsyHFnTdbW": ["t_db_c", "w_kg_per_kg"], "PsyTdbFnHW": ["h_j_per_kg", "w_kg_per_kg"],
             "PsyWFnTdbH": ["t_db_c", "h_j_per_kg"]}
    for index, row in enumerate(trace["dictionary"]):
        function = row["routine"]
        keys = order[function.removesuffix("_fast")]
        if len(keys) != len(row["input_bits"]):
            raise ValueError("Production input bit-vector length mismatch")
        inputs = {key: encode(struct.unpack(">d", bytes.fromhex(bits))[0]) for key, bits in zip(keys, row["input_bits"])}
        call = {"function": function, "inputs": inputs,
                "context": {"phase": row["phase"], "caller": row["caller"], "production_context": row.get("context")}}
        dictionary.append(call)
        value = struct.unpack(">d", bytes.fromhex(row["result_bits"]))[0]
        result = dict(call, input_id=index, resolved_inputs=inputs, value=encode(value), value_bits=row["result_bits"],
                      value_class="nan" if math.isnan(value) else ("+infinity" if value == math.inf else ("-infinity" if value == -math.inf else "finite")),
                      unit=FUNCTIONS[function][1])
        if function.startswith("PsyCpAirFnW"):
            cache = row["cp_cache"]
            if not isinstance(cache, dict):
                raise ValueError("Actual Cp production trace lacks cache state")
            for phase in ["before", "after"]:
                observed = dict(cache[phase])
                for key in ["dw_save", "cpa_save"]:
                    observed[key] = encode(struct.unpack(">d", bytes.fromhex(observed[key + "_bits"]))[0])
                result["cache_" + phase] = observed
            result["cache_hit"] = cache["hit"]
        results.append(result)
    ordered = trace["ordered_ids"]
    if any(index >= len(dictionary) for index in ordered):
        raise ValueError("Production dictionary index out of range")
    if len(ordered) != trace["recorded_call_count"] or trace["total_call_count"] != len(ordered) + trace["omitted_call_count"]:
        raise ValueError("Production total/recorded/omitted counts inconsistent")
    if sum(trace["routine_counts"].values()) != trace["total_call_count"]:
        raise ValueError("Production routine counts inconsistent")
    observed_counts = Counter()
    for index, count in Counter(ordered).items():
        observed_counts[trace["dictionary"][index]["routine"]] += count
    if any(count > trace["routine_counts"].get(function, 0) for function, count in observed_counts.items()):
        raise ValueError("Recorded routine counts exceed reported total")
    if trace["omitted_call_count"] == 0 and dict(observed_counts) != trace["routine_counts"]:
        raise ValueError("Complete trace routine counts don't match ordered events")
    receipt = {key: trace.get(key) for key in ["total_call_count", "recorded_call_count", "omitted_call_count", "event_limit", "unique_tuple_limit",
                                               "truncation_reason", "routine_counts", "capture_source", "input_origin", "thread_coverage", "phase_contract"]}
    receipt.update({"complete_call_trace": trace["omitted_call_count"] == 0, "dictionary_count": len(dictionary),
                    "scope": "every recorded ordered event; omitted suffix not compared", "schema": trace["schema"]})
    return ({"schema": "psy01-tuples.v2", "dictionary": dictionary, "ordered_ids": ordered}, receipt,
            {"schema": "psy01-results.v2", "dictionary": results, "ordered_ids": ordered})


def compare_compressed(reference: dict, candidate: dict) -> dict:
    if reference.get("schema") != "psy01-results.v2" or candidate.get("schema") != "psy01-results.v2":
        raise ValueError("Compressed result schema mismatch")
    ref_order, actual_order = reference["ordered_ids"], candidate["ordered_ids"]
    failures, mismatch_count = [], 0
    if len(ref_order) != len(actual_order) or reference["executed_event_count"] != len(ref_order):
        failures.append({"field": "event_count", "reference": len(ref_order), "actual": len(actual_order)})
        mismatch_count += 1
    pairs = Counter(zip(actual_order, ref_order))
    stats = {name: {"count": 0, "finite_count": 0, "max_abs_error": 0.0, "sum_square_error": 0.0} for name in FUNCTIONS}
    pair_receipts = []
    for (actual_id, ref_id), weight in pairs.items():
        if actual_id >= len(candidate["dictionary"]) or ref_id >= len(reference["dictionary"]):
            raise ValueError("Compressed result dictionary index out of range")
        actual, original = candidate["dictionary"][actual_id], reference["dictionary"][ref_id]
        if original["input_id"] != actual_id or actual["input_id"] != actual_id:
            failures.append({"field": "ordered_input_id", "actual_id": actual_id, "reference_id": ref_id, "repeat_count": weight})
            mismatch_count += weight
        comparison = compare({"schema": "psy01-results.v1", "calls": [dict(original, sequence_id="production", call_index=actual_id)]},
                             {"schema": "psy01-results.v1", "calls": [dict(actual, sequence_id="production", call_index=actual_id)]})
        mismatch_count += comparison["mismatch_count"] * weight
        for failure in comparison["failures"]:
            failures.append(dict(failure, actual_id=actual_id, reference_id=ref_id, repeat_count=weight))
        function = original["function"]
        measured = comparison["per_function"][function]
        aggregate = stats[function]
        aggregate["count"] += weight
        aggregate["finite_count"] += measured["finite_count"] * weight
        aggregate["max_abs_error"] = max(aggregate["max_abs_error"], measured["max_abs_error"])
        aggregate["sum_square_error"] += measured["rmse"] ** 2 * weight
        pair_receipts.append({"actual_id": actual_id, "reference_id": ref_id, "repeat_count": weight,
                              "mismatch_fields": comparison["mismatch_count"]})
    for row in stats.values():
        squared = row.pop("sum_square_error")
        row["rmse"] = math.sqrt(squared / row["finite_count"]) if row["finite_count"] else 0.0
    return {"event_count": len(actual_order), "unique_pair_count": len(pairs), "actual_dictionary_count": len(candidate["dictionary"]),
            "reference_dictionary_count": len(reference["dictionary"]), "weighted_mismatch_count": mismatch_count,
            "mismatch_count": mismatch_count, "failures": failures, "per_function": stats,
            "ordered_input_identity_checked_every_event": True, "pair_repeat_counts": pair_receipts,
            "checks_passed": mismatch_count == 0}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--prepare", action="store_true", help="Write deterministic contracts for review before comparisons")
    parser.add_argument("--check", action="store_true", help="Read-only source/contract pin check, no compile/run")
    parser.add_argument("--calls", type=Path, help="Replay external psy01-tuples.v1 using original functions")
    parser.add_argument("--production-trace", type=Path, help="Replay actual Rust psychrometrics-calls.v1/v2 recorded input bits")
    parser.add_argument("--rust-runner", type=Path, help="Built psy01_tuples executable: INPUT.json -> JSON stdout")
    parser.add_argument("--rust-results", type=Path, help="Compare one external psy01-results.v1 to --calls")
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_RUNTIME)
    args = parser.parse_args()
    verify_pins()
    contracts = frozen_contracts(check=not args.prepare)
    if args.prepare or args.check:
        print(json.dumps({"mode": "prepare" if args.prepare else "check", "contract_count": len(contracts),
                          "unit_call_count": cases_contract()["call_count"], "checks_passed": True}))
        return 0
    if args.calls and args.production_trace:
        raise ValueError("Select --calls or --production-trace")
    if args.rust_results and not args.calls:
        raise ValueError("--rust-results requires one --calls input")
    if args.production_trace and (args.rust_runner or args.rust_results):
        raise ValueError("Production replay compares observed Rust results; do not replace with a synthetic runner")
    directory = args.output_dir.resolve()
    directory.mkdir(parents=True, exist_ok=True)
    exe, build, env = build_reference(directory)
    trace_receipt = None
    actual_production = None
    if args.calls:
        processes = [{"id": "external", "tuples": read_json(args.calls)}]
    elif args.production_trace:
        raw_trace = read_json(args.production_trace)
        adapter = production_compressed if raw_trace.get("schema") == "psychrometrics-calls.v2" else production_tuples
        tuples, trace_receipt, actual_production = adapter(raw_trace)
        trace_receipt["original_trace"] = ref(args.production_trace)
        processes = [{"id": "production", "tuples": tuples}]
    else:
        processes = read_json(CONTRACTS / "PSY-01-cases.json")["processes"]
    results = []
    all_passed = True
    for process in processes:
        name = process["id"]
        tuples = process["tuples"]
        validate_tuples(tuples)
        compressed = tuples["schema"] == "psy01-tuples.v2"
        if compressed and (args.rust_runner or args.rust_results):
            raise ValueError("Compressed replay uses actual production observations; Rust unit protocol remains v1")
        input_file, output_file = directory / f"{name}-tuples.json", directory / f"{name}-reference.json"
        write_json(input_file, tuples)
        receipt = execute([str(exe), str(input_file), str(output_file), str(DLL)], directory, name + "-reference", env)
        original = read_json(output_file)
        count = len(tuples["ordered_ids"]) if compressed else len(tuples["calls"])
        row = {"process_id": name, "call_count": count, "tuples": ref(input_file),
               "original_results": ref(output_file), "execution": receipt}
        if args.rust_runner:
            runner = args.rust_runner.resolve()
            rust_receipt = execute([str(runner), str(input_file)], directory, name + "-rust")
            rust_file = directory / (name + "-rust-stdout.log")
            comparison = compare(original, read_json(rust_file))
            row.update({"rust_binary": ref(runner), "rust_execution": rust_receipt, "comparison": comparison})
            all_passed &= comparison["checks_passed"]
        elif args.rust_results:
            comparison = compare(original, read_json(args.rust_results))
            row.update({"rust_results": ref(args.rust_results), "comparison": comparison})
            all_passed &= comparison["checks_passed"]
        elif actual_production:
            if compressed:
                comparison = compare_compressed(original, actual_production)
                row.update({"actual_production_trace": ref(args.production_trace), "comparison": comparison})
            else:
                production_results_file = directory / "production-observed-results.json"
                write_json(production_results_file, actual_production)
                comparison = compare(original, actual_production)
                row.update({"actual_production_results": ref(production_results_file), "comparison": comparison})
            all_passed &= comparison["checks_passed"]
        # Retain just the salient original cold-cache values in the checked-in receipt.
        if name == "cold-cp-sentinel":
            row["cold_cache_observations"] = original["calls"]
        results.append(row)
    summary = {"schema": "psy01-source-reference.v1", "energyplus_commit": PIN,
               "execution_repository_head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
               "contracts": contracts, "reference_build": build, "state_library": ref(DLL),
               "python_tool": ref(Path(__file__)), "results": results,
               "reference_processes": len(processes), "production_trace": trace_receipt,
               "comparisons_executed": bool(args.rust_runner or args.rust_results or actual_production),
               "checks_passed": all_passed, "gates_updated": False,
               "limitations": ["warning aggregation/I/O excluded", "unit IEEE probes don't expand A/B admitted inputs",
                               "downstream ZON-02/HVAC-04/05 numerical gates require their actual same-tuple/call-order traces"]}
    summary_file = directory / "source-reference.json"
    write_json(summary_file, summary)
    if not args.calls and not args.production_trace:
        write_json(EVIDENCE / "source-reference.json", summary)
    print(json.dumps({"summary": str(summary_file.relative_to(ROOT)), "reference_processes": len(processes),
                      "call_count": sum(r["call_count"] for r in results), "comparisons_executed": summary["comparisons_executed"],
                      "checks_passed": all_passed}))
    return 0 if all_passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
