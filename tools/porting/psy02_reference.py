#!/usr/bin/env python3
"""Freeze PSY-02 input/source/precision contracts and run genuine original peers.

No expected numerical answers, Rust edits, gate updates, or physical-input expansion.
"""
from __future__ import annotations
import argparse
from collections import Counter
import hashlib
import json
import math
import os
from pathlib import Path
import shutil
import struct
import subprocess
import sys
import time

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / ".reference/energyplus-src/26.1.0"
CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
EVIDENCE = ROOT / "energyplus_porting_plan/evidence/PSY-02"
RAW = ROOT / ".runtime/porting/PSY-02"
PIN = "6f2e40d10250a105b49966baa24d843711e61048"
CORE_RECEIPT = ROOT / ".runtime/porting/reference-energyplus-26.1.0/native-core-build.json"
BUILD = ROOT / ".runtime/ep261-gcc13-o0"
SOURCE_FILES = ["src/EnergyPlus/Psychrometrics.cc", "src/EnergyPlus/Psychrometrics.hh", "src/EnergyPlus/PsychCacheData.hh", "src/EnergyPlus/General.cc", "src/EnergyPlus/General.hh", "src/EnergyPlus/DataGlobalConstants.hh", "src/EnergyPlus/Data/EnergyPlusData.cc", "src/EnergyPlus/Data/EnergyPlusData.hh", "src/EnergyPlus/WeatherManager.cc", "src/EnergyPlus/PurchasedAirManager.cc", "src/EnergyPlus/api/func.cc", "src/EnergyPlus/CMakeLists.txt"]
SOURCE_FILES += ["src/EnergyPlus/EnergyPlus.hh", "third_party/ObjexxFCL/src/ObjexxFCL/Fmath.hh"]
RANGES = [
    ("Psychrometrics.hh", "PsyWFnTdbH", 962, 998, []),
    ("Psychrometrics.hh", "PsyWFnTdbRhPb", 1342, 1387, ["PsyPsatFnTemp", "PsyWFnTdbRhPb_error", "ObjexxFCL::max"]),
    ("Psychrometrics.hh", "PsyTsatFnHPb", 1073, 1120, ["PsyTsatFnHPb_raw"]),
    ("Psychrometrics.cc", "PsyTsatFnHPb_raw", 899, 1072, ["F6", "F7", "PsyHFnTdbW", "PsyWFnTdbTwbPb", "ObjexxFCL::max", "ObjexxFCL::min"]),
    ("Psychrometrics.cc", "PsyTwbFnTdbWPb", 279, 345, ["PsyTwbFnTdbWPb_raw"]),
    ("Psychrometrics.cc", "PsyTwbFnTdbWPb_raw", 347, 566, ["PsyTsatFnPb", "PsyPsatFnTemp", "General::Iterate"]),
    ("Psychrometrics.hh", "PsyPsatFnTemp", 1000, 1071, ["PsyPsatFnTemp_raw"]),
    ("Psychrometrics.cc", "PsyPsatFnTemp_raw", 640, 784, ["std::exp", "std::log"]),
    ("Psychrometrics.hh", "PsyTsatFnPb", 1488, 1525, ["PsyTsatFnPb_raw"]),
    ("Psychrometrics.cc", "PsyTsatFnPb_raw", 1264, 1450, ["PsyPsatFnTemp", "General::Iterate"]),
    ("Psychrometrics.hh", "PsyWFnTdbTwbPb", 1408, 1460, ["PsyPsatFnTemp", "PsyWFnTdbRhPb"]),
    ("Psychrometrics.hh", "F6", 1600, 1603, []),
    ("Psychrometrics.hh", "F7", 1605, 1609, []),
    ("EnergyPlus.hh", "using ObjexxFCL::max", 165, 166, ["ObjexxFCL::min"]),
    ("third_party/ObjexxFCL/src/ObjexxFCL/Fmath.hh", "min", 90, 96, []),
    ("third_party/ObjexxFCL/src/ObjexxFCL/Fmath.hh", "max", 338, 344, []),
    ("General.cc", "Iterate", 918, 979, []),
    ("PsychCacheData.hh", "PsychrometricCacheData", 80, 201, []),
    ("Psychrometrics.hh", "PsychrometricsData", 1702, 1735, ["InitializePsychRoutines"]),
    ("Psychrometrics.cc", "InitializePsychRoutines", 110, 134, []),
    ("Psychrometrics.cc", "PsyWFnTdbH_error", 606, 638, []),
    ("Psychrometrics.cc", "PsyWFnTdbRhPb_error", 1228, 1262, []),
    ("Psychrometrics.cc", "PsyWFnTdbTwbPb_temperature_error", 786, 855, []),
    ("WeatherManager.cc", "PsyWFnTdbRhPb", 2125, 2148, ["PsyTwbFnTdbWPb"]),
    ("PurchasedAirManager.cc", "PsyWFnTdbH", 2191, 2330, ["PsyTsatFnHPb", "PsyWFnTdbRhPb"]),
    ("PurchasedAirManager.cc", "PsyWFnTdbRhPb", 2556, 2570, []),
    ("PurchasedAirManager.cc", "PsyWFnTdbRhPb", 2610, 2632, []),
    ("api/func.cc", "PsyTwbFnTdbWPb", 207, 245, ["PsyWFnTdbH", "PsyPsatFnTemp", "PsyTsatFnHPb"]),
    ("api/func.cc", "PsyWFnTdbRhPb", 273, 289, ["PsyWFnTdbTwbPb"]),
]
FUNCTIONS = {
    "PsyTsatFnHPb": (["h_j_per_kg", "p_pa"], "degC"),
    "PsyTsatFnHPb_raw": (["h_j_per_kg", "p_pa"], "degC"),
    "PsyWFnTdbRhPb": (["t_db_c", "rh_fraction", "p_pa"], "kg/kg"),
    "PsyWFnTdbH": (["t_db_c", "h_j_per_kg"], "kg/kg"),
    "PsyTwbFnTdbWPb": (["t_db_c", "w_kg_per_kg", "p_pa"], "degC"),
    "PsyTwbFnTdbWPb_raw": (["t_db_c", "w_kg_per_kg", "p_pa"], "degC"),
    "PsyPsatFnTemp": (["t_db_c"], "Pa"),
    "PsyPsatFnTemp_raw": (["t_db_c"], "Pa"),
    "PsyTsatFnPb": (["p_pa"], "degC"),
    "PsyTsatFnPb_raw": (["p_pa"], "degC"),
    "PsyWFnTdbTwbPb": (["t_db_c", "t_wb_c", "p_pa"], "kg/kg"),
    "PsyHFnTdbW": (["t_db_c", "w_kg_per_kg"], "J/kg"),
}

def read(path): return json.loads(path.read_text(encoding="utf-8"))
def sha(path):
    with path.open("rb") as stream: return hashlib.file_digest(stream, "sha256").hexdigest()
def ref(path): return {"path": path.resolve().relative_to(ROOT).as_posix(), "sha256": sha(path)}
def write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2, allow_nan=False) + "\n", encoding="utf-8", newline="\n")
def require(condition, message):
    if not condition: raise ValueError(message)
def fp_bits(value): return struct.unpack(">Q", struct.pack(">d", value))[0]
def fp_value(bits): return struct.unpack(">d", struct.pack(">Q", bits))[0]

def exact_metadata_equal(a, b):
    """JSON identity retains scalar types; numeric outputs use separate tolerances."""
    if type(a) is not type(b): return False
    if isinstance(a, dict):
        if {(type(k), k) for k in a} != {(type(k), k) for k in b}: return False
        return all(exact_metadata_equal(value, b[key]) for key, value in a.items())
    if isinstance(a, (list, tuple)):
        return len(a) == len(b) and all(exact_metadata_equal(x, y) for x, y in zip(a, b))
    if isinstance(a, float): return fp_bits(a) == fp_bits(b)
    return a == b

def check_metadata():
    probes = [
        ({'cache_hit': True}, {'cache_hit': 1}),
        ({'cache_before': {'index': 0}}, {'cache_before': {'index': False}}),
        ({'cache_before': {'requested_tags': {'i_h': 0}}}, {'cache_before': {'requested_tags': {'i_h': 0.0}}}),
        ({'state_before': {'warmup': False}}, {'state_before': {'warmup': 0}}),
        ({'context': {'zone_timestep': {'hour_index': 1}}}, {'context': {'zone_timestep': {'hour_index': 1.0}}}),
        ({'input_dictionary_id': 0}, {'input_dictionary_id': False}),
        ({'ordered_ids': [0, 1]}, {'ordered_ids': [False, True]}),
        ({'calls': [{'call_index': 0}]}, {'calls': [{'call_index': 0.0}]}),
    ]
    for i, (valid, invalid) in enumerate(probes):
        require(not exact_metadata_equal(valid, invalid), 'Type-invalid metadata accepted: probe ' + str(i))
        require(exact_metadata_equal(valid, json.loads(json.dumps(valid))), 'Valid JSON metadata rejected: probe ' + str(i))
    return {'schema': 'psy02-exact-metadata-selfcheck.v1', 'negative_type_probes_rejected': len(probes), 'positive_payloads_accepted': len(probes), 'numeric_tolerances_changed': False, 'gates_updated': False}

def requests():
    calls = []
    def add(function, inputs, group, **extra):
        index = len(calls)
        calls.append({"call_index": index, "sequence_id": group, "function": function, "inputs": inputs, "context": {"kind": "unit-input-only", "group": group}, **extra})
        return index
    # Cold zero-tag returns exercise the actual allocated array type/defaults.
    add("PsyTwbFnTdbWPb", {"t_db_c": 0., "w_kg_per_kg": 0., "p_pa": 0.}, "cold-zero-tag-source-diagnostic")
    add("PsyTsatFnHPb", {"h_j_per_kg": 0., "p_pa": 0.}, "cold-zero-tag-source-diagnostic")
    add("PsyTsatFnPb", {"p_pa": 0.}, "cold-zero-tag-source-diagnostic")
    temps = [-101., -100., math.nextafter(-100., math.inf), -80., -40., -5., -0.001, -0., 0., 0.001, math.nextafter(.01, -math.inf), .01, math.nextafter(.01, math.inf), 5., 22., 40., 100., 200., math.nextafter(200., math.inf), 201.]
    for t in temps:
        for f in ["PsyPsatFnTemp", "PsyPsatFnTemp_raw"]: add(f, {"t_db_c": t}, "psat-ice-water-clamps")
    pressures = [.0016, .0017, math.nextafter(.0017, math.inf), 10., 100., 611., math.nextafter(611., math.inf), 611.125, math.nextafter(611.25, -math.inf), 611.25, 1500., 50000., 80000., 101325., 101330., 120000., math.nextafter(1555000., -math.inf), 1555000., 1555001.]
    for p in pressures:
        for f in ["PsyTsatFnPb", "PsyTsatFnPb_raw"]: add(f, {"p_pa": p}, "pressure-plateau-bounds-saved-pair")
    for t in [-40., -.001, 0., .01, 22., 40., 100.]:
        for rh in [-.1, 0., 1e-6, .1, .5, 1., 1.05]:
            for p in [80000., 101330.]: add("PsyWFnTdbRhPb", {"t_db_c": t, "rh_fraction": rh, "p_pa": p}, "rh-floor-denominator-and-nonzero")
    psat = add("PsyPsatFnTemp", {"t_db_c": 22.}, "own-output-denominator-clip")
    add("PsyWFnTdbRhPb", {"t_db_c": 22., "rh_fraction": 1., "p_pa": {"from_call": psat}}, "own-output-denominator-clip")
    for t in [-40., 0., 22., 60.]:
        for h in [-1e6, -1000., -0., 0., 5000., 50000., 100000., 1e6]: add("PsyWFnTdbH", {"t_db_c": t, "h_j_per_kg": h}, "h-inverse-negative-floor-and-zero")
    for w in [0., 1e-10, 1e-5, .008, .024, .1]:
        h = add("PsyHFnTdbW", {"t_db_c": 22., "w_kg_per_kg": w}, "own-h-to-w-chain-prerequisite-PSY01")
        add("PsyWFnTdbH", {"t_db_c": 22., "h_j_per_kg": {"from_call": h}}, "own-h-to-w-chain-prerequisite-PSY01")
    for h in ["NaN", "+Infinity", "-Infinity"]: add("PsyWFnTdbH", {"t_db_c": 22., "h_j_per_kg": h}, "uncached-IEEE-classification")
    ranges = [-42400., -22138., -670.12, 27297., 75222., 183790., 475770., 1544500., 3835300., 45866000.]
    for hh in ranges:
        h = hh - 17863.7
        for delta in [-math.inf, None, math.inf]:
            value = h if delta is None else math.nextafter(h, delta)
            for f in ["PsyTsatFnHPb", "PsyTsatFnHPb_raw"]: add(f, {"h_j_per_kg": value, "p_pa": 101330.}, "h-polynomial-case-boundaries")
    for h in [-30000., -1e-10, 0., 1e-10, 10000., 50000., 100000.]:
        for p in [80000., 101330. * .99, 101330. * 1.01, 120000.]:
            for f in ["PsyTsatFnHPb", "PsyTsatFnHPb_raw"]: add(f, {"h_j_per_kg": h, "p_pa": p}, "h-pressure-band-secant-nonzero")
    for t in [-40., -5., -.001, 0., .001, 22., 40., 100.]:
        for w in [-.001, -1e-10, 0., 1e-5, .005, .02, .1]:
            for f in ["PsyTwbFnTdbWPb", "PsyTwbFnTdbWPb_raw"]: add(f, {"t_db_c": t, "w_kg_per_kg": w, "p_pa": 80000. if t < 0 else 101330.}, "wetbulb-ice-liquid-floor-supersaturation")
    for t,tw in [(22., 30.), (22., 22.), (22., -30.), (-10., -20.), (0., 0.), (.01, .01)]:
        add("PsyWFnTdbTwbPb", {"t_db_c": t, "t_wb_c": tw, "p_pa": 101330.}, "direct-saturation-helper-clamp-fallback")
    # Explicit test-state input; does not alter production defaults or loop limits.
    add("PsyTwbFnTdbWPb_raw", {"t_db_c": 30., "w_kg_per_kg": .02, "p_pa": 101330.}, "convergence-failure-test-state", state_overrides={"iconv_tol": 0.})
    add("PsyTwbFnTdbWPb_raw", {"t_db_c": 30., "w_kg_per_kg": .02, "p_pa": 101330.}, "restore-default-tolerance", state_overrides={"iconv_tol": .0001})
    for warmup,suppress in [(False,False),(True,False),(False,True)]:
        add("PsyWFnTdbH", {"t_db_c": 22., "h_j_per_kg": -1000.}, "source-warning-policy", state_overrides={"warmup":warmup}, suppress_warnings=suppress)
    add("PsyWFnTdbH", {"t_db_c": 0., "h_j_per_kg": 0.}, "restore-default-warmup", state_overrides={"warmup":False})
    for i in [
        {"tol":.0001,"x0":0.,"y0":1.,"x1":0.,"y1":0.,"iteration":1},
        {"tol":.0001,"x0":10.,"y0":1.,"x1":0.,"y1":0.,"iteration":1},
        {"tol":.0001,"x0":1.,"y0":0.,"x1":0.,"y1":1.,"iteration":2},
        {"tol":.0001,"x0":1.,"y0":2.,"x1":.5,"y1":2.,"iteration":2},
        {"tol":.0001,"x0":1.,"y0":2.,"x1":1.00001,"y1":1.,"iteration":2}]: add("General::Iterate", i, "source-only-iteration-state")
    # Same-tag first-writer history and different-key/same-slot replacement.
    histories=[]
    for order in ["a-first", "b-first"]:
        sequence=[]
        def item(f,inp,label):sequence.append({"call_index":len(sequence),"sequence_id":label,"function":f,"inputs":inp,"context":{"kind":"cache-history-input-only","order":order}})
        h=50000.; hb=fp_value(fp_bits(h)+1000)
        first,second=(h,hb) if order=="a-first" else (hb,h)
        for value in [first,second,first,second]: item("PsyTsatFnHPb",{"h_j_per_kg":value,"p_pa":101330.},"hpb-same-tag-original-first-writer")
        for f,inp,key,shift in [
            ("PsyTsatFnHPb",{"h_j_per_kg":h,"p_pa":101330.},"h_j_per_kg",24),
            ("PsyPsatFnTemp",{"t_db_c":22.},"t_db_c",28),
            ("PsyTsatFnPb",{"p_pa":101330.},"p_pa",28),
            ("PsyTwbFnTdbWPb",{"t_db_c":22.,"w_kg_per_kg":.008,"p_pa":101330.},"t_db_c",32)]:
            collision={**inp,key:fp_value(fp_bits(inp[key])^(1<<(shift+20)))}
            for args in [inp,inp,collision,inp,collision]: item(f,args,"different-tags-same-slot-replacement")
        histories.append({"id":"cache-history-"+order,"request":{"schema":"psy02-tuples.v1","calls":sequence}})
    return [{"id":"normal-boundary-history","request":{"schema":"psy02-tuples.v1","calls":calls}},*histories]

def source_contract():
    selected=[]
    for file,symbol,start,end,helpers in RANGES:
        path=file if file.startswith('third_party/') else "src/EnergyPlus/"+file; lines=(SOURCE/path).read_text(encoding="utf-8").splitlines(keepends=True)
        require(any(symbol in line for line in lines[start-1:end]),"Source range symbol absent: "+symbol)
        selected.append({"path":path,"symbol":symbol,"start_line":start,"end_line":end,"helpers":helpers,"range_sha256":hashlib.sha256("".join(lines[start-1:end]).encode()).hexdigest()})
    return {"schema":"psy02-source-contract.v1","card":"PSY-02","energyplus_commit":PIN,"source_files":[{"path":p,"sha256":sha(SOURCE/p)} for p in SOURCE_FILES],"selected_ranges":selected,
        "named_functions":["PsyTsatFnHPb","PsyWFnTdbRhPb","PsyWFnTdbH","PsyTwbFnTdbWPb"],"direct_helpers":["PsyPsatFnTemp","PsyTsatFnPb","PsyWFnTdbTwbPb","General::Iterate","F6","F7","PsyHFnTdbW","ObjexxFCL::max","ObjexxFCL::min"],"ordered_min_max_policy":"EnergyPlus.hh imports original ObjexxFCL double overloads: max uses a<b?b:a, min uses a<b?a:b; preserve operand order, NaN and signed-zero behavior rather than substituting numeric max/min intrinsics",
        "reference":{"kind":"unchanged-original-GCC-core-and-original-headers","state":"same-compiler-owned genuine EnergyPlusData; explicit original clear_state then init_constant_state; one fresh state per process","flag_targets":["energypluslib","project_options","project_fp_options","project_warnings"],"header_definitions":"copy BOTH actual energypluslib target and original src/EnergyPlus DIRECTORY COMPILE_DEFINITIONS so EP_psych_errors and any ABI statistics/cache compile modes match the core; validate actual compile row before execution","original_body_patches":False,"private_Msvc_DLL_state_cast":False},
        "cache_contract":{"capacity_each":1048576,"Twb":{"precision_bits":20,"tags":"unsigned IEEE upper bits","miss_operands":"quantized T/W/P representatives","defaults":{"i_tdb":0,"i_w":0,"i_pb":0,"value":0}},"Psat":{"precision_bits":24,"tags":"signed IEEE arithmetic upper bits","miss_operands":"quantized T representative","defaults":{"i_tdb":-1000,"value":0}},"TsatHPb":{"precision_bits":28,"tags":"signed IEEE arithmetic upper bits","miss_operands":"ORIGINAL first-writer H/P; same-tag subsequent caller returns stored result","defaults":{"i_h":0,"i_pb":0,"value":0}},"TsatPb":{"precision_bits":24,"tags":"signed IEEE arithmetic upper bits","miss_operands":"ORIGINAL first-writer P","actual_storage_type":"cached_tsat_h_pb (not unused cached_tsat_pb)","defaults":{"i_h":0,"i_pb":0,"value":0}}},
        "state_contract":{"iconv_tol":.0001,"last_patm":-99999.,"last_t_boil":-99999.,"press_save":-99999.,"t_sat_save":-99999.,"use_interpolation":False,"history":"retained across all calls and sequence IDs until fresh process; no observer reset","observations":"root target cache slot and scalar state before/after; all four genuine arrays scanned ONCE at process end to emit exact sorted NONDEFAULT tables","iteration_locals":"not observable in linked stats-OFF core; do not infer raw iteration count or internal call tree from returned values","source_error_indices_and_callbacks":"genuine source-only diagnostics; ErrorManager recurring-index/logging parity excluded"},
        "constant_and_iteration_policy":{"Psat":"original Hyland-Wexler, triple point 273.16K, clamps below173.15K/above473.15K; IF97 inactive","TsatHPb":"source polynomial F6/F7 CaseRange+17863.7; abs(P-101330)/101330>0.01 secant branch; relative H tolerance1e-5; while IterCount<=30; no-convergence retains polynomial initial T","Twb":"W<0 floor1e-5; state saved boiling pressure/T; boiling clamp tBoil-.1 if WBT>=tBoil-.09; source100-iteration cap and iconvTol; final explicit if(TWB>TDB) TWB=TDB assignment, preserving NaN (not a min helper call)","TsatPb":"default interpolation=false; saved exact-pressure pair; .0017/1555000 clamps and611<P<611.25 zero plateau; 50iterations, .0001 convergence","GeneralIterate":"original .1 perturbation and1e-9 DY floor, X convergence and Y==0 conditions; source-state diagnostics"},
        "production_scope":{"input_family":"unchanged CON15 ordinary2013 weather and sensible no-OA IdealLoads","primary":"actual WeatherManager W/RH and wet-bulb; cooling saturation/W consumers when genuinely invoked","TsatHPb":"supplemental original/Rust unit-state proof until actual admitted cooling guard consumption is independently observed; no artificial call solely to claim production coverage","observer_protocol":"selected OUTERMOST roots only, preserve actual invocation order; original nested helpers execute once naturally in replay; separate PSY02 artifact preserves PSY01 v2 meaning"},
        "excluded":["CON-excluded physical inputs/OA/humidity-control families","IF97 and interpolation=true alternate algorithms","full ErrorManager recurring-warning/logging lifecycle equivalence","whole HVAC05 saturation assembly or CLK04 weather pipeline completion","unobserved raw internal iteration count/nested call tree identity","whole building/HVAC numerical equivalence"],"gates_updated":False}

def prepare():
    source=source_contract(); processes=requests(); scope=read(CONTRACTS/"scope.json")
    case={"schema":"psy02-cases-contract.v1","card":"PSY-02","processes":processes,"fresh_state_per_process":True,"sequence_id_resets_state":False,"inputs_only":True,"expected_numerical_outputs":False,"production_input_matrix_expanded":False,"fixed_production_inputs":[{"case_id":c['id'],"scope":c['scope'],"duration":c['duration'],"input":c['input'],"weather":c['weather']} for c in scope['cases']],"total_calls":sum(len(p['request']['calls']) for p in processes),"source_only_diagnostics":["General::Iterate","source_error_indices","source_error_messages","internal iteration count is unobserved"],"gates_updated":False}
    atol={"degC":1e-10,"kg/kg":1e-12,"Pa":1e-8,"J/kg":1e-7}
    tolerance={"schema":"psy02-tolerances.v1","card":"PSY-02","frozen_before_comparison":True,"rows":[{"function":f,"unit":unit,"atol":atol[unit],"rtol":1e-13} for f,(_,unit) in FUNCTIONS.items()],"state_rows":[{"fields":["last_t_boil","t_sat_save","cached temperature value"],"unit":"degC","atol":1e-10,"rtol":1e-13},{"fields":["last_patm","press_save","cached Psat value"],"unit":"Pa","atol":1e-8,"rtol":1e-13},{"fields":["iconv_tol"],"unit":"degC","atol":0,"rtol":0}],"exact":["input identity/bits/order/context","cache kinds/indices/tags/hits/occupied keys","branch and IEEE class","warmup/interpolation/default/reset policy"],"bits_policy":"input/tag/identity bits exact; floating output/state bits recorded with frozen per-unit numerical comparison; a cache HIT must return its own stored result bit-exactly","zero_policy":"atol governs near0; no tolerance relaxation after failure","warning_lifecycle":"source-only diagnostic, never fabricated Rust parity","General_Iterate":"source-only exact before/after scalar diagnostic unless true existing Rust route is separately paired","gates_updated":False}
    for filename,data in [("PSY-02-source.json",source),("PSY-02-cases.json",case),("PSY-02-tolerances.json",tolerance)]:
        path=CONTRACTS/filename
        if path.exists():require(read(path)==data,"Frozen contract differs; explicit reviewed supersession required: "+filename)
        else:write(path,data)
    return {"contracts":[ref(CONTRACTS/n) for n in ["PSY-02-source.json","PSY-02-cases.json","PSY-02-tolerances.json"]],"process_count":len(processes),"call_count":case['total_calls'],"comparison_run":False}

def check():
    return prepare()

def process(command, directory, stem, env=None, cwd=None):
    started=time.time(); result=subprocess.run(command,cwd=cwd or ROOT,env=env,capture_output=True)
    stdout=directory/(stem+'-stdout.log');stderr=directory/(stem+'-stderr.log');stdout.write_bytes(result.stdout);stderr.write_bytes(result.stderr)
    receipt={'command':command,'exit_code':result.returncode,'elapsed_seconds':round(time.time()-started,3),'stdout':ref(stdout),'stderr':ref(stderr)}
    write(directory/(stem+'-command.json'),receipt);return receipt

def build_driver(directory, driver_target='psy02_reference'):
    require(not directory.exists(),'Use a fresh build-receipt directory')
    check(); core=read(CORE_RECEIPT);require(core['checks_passed'] is True and core['energyplus_commit']==PIN,'Genuine core not verified')
    baseline=read(ROOT/'.runtime/porting/CLK-01/native-driver/configure-receipt.json')
    command=baseline['command'][:]
    command=[('-DCMAKE_PROJECT_INCLUDE='+str(ROOT/'tools/porting/psy02_reference.cmake')) if c.startswith('-DCMAKE_PROJECT_INCLUDE=') else c for c in command]
    env=os.environ.copy();env['PYTHONUTF8']='1';env['PYTHONIOENCODING']='utf-8'
    compiler=ROOT/'.runtime/reference-tools/winlibs-gcc13.2.0-ucrt-r3/mingw64/bin';env['PATH']=str(compiler)+os.pathsep+env.get('PATH','')
    before=read(BUILD/'compile_commands.json');directory.mkdir(parents=True)
    original_commands_path=ROOT/'.runtime/porting/reference-energyplus-26.1.0/provenance/final-core/compile_commands.json'
    original_commands=read(original_commands_path)
    require(all(row in before for row in original_commands),'Frozen original compile commands changed before configure')
    sources=[]
    for filename in ['psy02_reference.cpp','psy02_reference.py','psy02_reference.cmake','clk01_reference.cmake','psy02_reachability.cpp','psy02_reachability.cmake']:
        original=ROOT/'tools/porting'/filename;target=directory/'source'/filename;target.parent.mkdir(exist_ok=True);shutil.copyfile(original,target);sources.append(ref(target))
    configure=process(command,directory,'configure',env);require(configure['exit_code']==0,'Configure failed; raw logs retained')
    configured=read(BUILD/'compile_commands.json');row=next(x for x in configured if Path(x['file']).name=='psy02_reference.cpp' and f'CMakeFiles/{driver_target}.dir/' in x['output'])
    require(all(row in configured for row in original_commands),'Frozen original compile commands changed during configure')
    write(directory/'actual-driver-compile-command.json',row)
    for flag in ['-DEP_psych_errors','-UNDEBUG','-Werror','-O0','-ffp-contract=off']:
        require(flag in row['command'],'Driver missing original mode/flag before compile: '+flag)
    cmake=command[0];compile_link=process([cmake,'--build',str(BUILD),'--target',driver_target,'--parallel','1'],directory,'build',env)
    require(compile_link['exit_code']==0,'Driver compile/link failed; raw logs retained')
    after=read(BUILD/'compile_commands.json');before_core=[x for x in before if 'energypluslib.dir' in x['command']];after_core=[x for x in after if 'energypluslib.dir' in x['command']]
    require(before_core==after_core,'Original core compile commands changed')
    require(all(row in after for row in original_commands),'Frozen original compile commands changed during build')
    row=next(x for x in after if Path(x['file']).name=='psy02_reference.cpp' and f'CMakeFiles/{driver_target}.dir/' in x['output'])
    for flag in ['-DEP_psych_errors','-UNDEBUG','-Werror','-O0','-ffp-contract=off']:
        require(flag in row['command'],'Driver missing original mode/flag: '+flag)
    require('-DEP_psych_stats' not in row['command'] and '-DEP_nocache_Psychrometrics' not in row['command'] and '-ffast-math' not in row['command'],'Unexpected original mode')
    write(directory/'actual-driver-compile-command.json',row)
    for key in ['core_library','api_library']:
        binding=core['artifacts'][key];require(sha(ROOT/binding['path'])==binding['sha256'],'Original core/API bytes changed')
    binary=BUILD/('Products/'+driver_target+'.exe');archived=directory/(driver_target+'.exe');shutil.copyfile(binary,archived)
    shutil.copyfile(BUILD/'CMakeCache.txt',directory/'CMakeCache.txt');shutil.copyfile(BUILD/'compile_commands.json',directory/'compile_commands.json')
    receipt={'schema':'psy02-native-driver-build.v1','checks_passed':True,'energyplus_commit':PIN,'target':driver_target,'core_build':ref(CORE_RECEIPT),'binary':ref(archived),'linked_build_binary':ref(binary),'executed_source_archives':sources,'configure':ref(directory/'configure-command.json'),'compile_link':ref(directory/'build-command.json'),'actual_compile_command':ref(directory/'actual-driver-compile-command.json'),'cache':ref(directory/'CMakeCache.txt'),'compile_commands':ref(directory/'compile_commands.json'),'original_core_compile_commands_unchanged':True,'original_core_compile_command_count':len(after_core),'frozen_original_build_commands':ref(original_commands_path),'frozen_original_build_command_count':len(original_commands),'frozen_original_build_commands_unchanged':True,'original_core_and_API_bytes_unchanged':True,'same_compiler_owned_state':True,'actual_EP_psych_errors':True,'actual_EP_psych_stats':False,'assertions_enabled':True,'fp_contract':'off','gates_updated':False}
    write(directory/'native-driver-build.json',receipt);return receipt

def run_original(directory, driver_receipt):
    require(not directory.exists(),'Use a fresh original execution directory');check()
    build=read(driver_receipt);require(build['checks_passed'] is True and build['energyplus_commit']==PIN,'Driver build unverified')
    binary=ROOT/build['binary']['path'];require(ref(binary)==build['binary'],'Archived original driver changed')
    env=os.environ.copy();compiler=ROOT/'.runtime/reference-tools/winlibs-gcc13.2.0-ucrt-r3/mingw64/bin';env['PATH']=str(compiler)+os.pathsep+env.get('PATH','')
    directory.mkdir(parents=True);executed=directory/'psy02_reference.executed.py';shutil.copyfile(Path(__file__),executed)
    rows=[]
    for item in read(CONTRACTS/'PSY-02-cases.json')['processes']:
        destination=directory/item['id'];destination.mkdir();request=destination/'request.json';write(request,item['request'])
        receipt=process([str(binary),str(request)],destination,'reference',env)
        require(receipt['exit_code']==0,'Original process failed; raw evidence retained: '+item['id'])
        output=read(destination/'reference-stdout.log');require(output['schema']=='psy02-results.v1' and len(output['calls'])==len(item['request']['calls']),'Invalid original result count/schema')
        results=destination/'results.json';write(results,output)
        row={'schema':'psy02-original-reference.v1','process_id':item['id'],'reference_kind':'unchanged-linked-original-and-header-with-genuine-state','command':receipt['command'],'exit_code':receipt['exit_code'],'binary':ref(binary),'driver_build':ref(driver_receipt),'native_core_build':ref(CORE_RECEIPT),'executed_launcher':ref(executed),'request':ref(request),'results':ref(results),'stdout':receipt['stdout'],'stderr':receipt['stderr'],'contract_refs':[ref(CONTRACTS/n) for n in ['PSY-02-source.json','PSY-02-cases.json','PSY-02-tolerances.json']],'call_count':len(output['calls']),'fresh_process':True,'comparison_performed':False,'gates_updated':False}
        write(destination/'reference.json',row);rows.append(ref(destination/'reference.json'))
    result={'schema':'psy02-original-first-preparation.v1','original_processes':rows,'call_count':read(CONTRACTS/'PSY-02-cases.json')['total_calls'],'original_source_math_unchanged':True,'Rust_comparison_performed':False,'gates_updated':False}
    write(directory/'original-first.json',result);return result

class Comparison:
    def __init__(self):
        self.mismatches=[];self.mismatch_count=0;self.count=0;self.stats={}
        self.tolerances={r['function']:r for r in read(CONTRACTS/'PSY-02-tolerances.json')['rows']}
    def exact(self,a,b,label,weight=1):
        self.count+=weight
        if not exact_metadata_equal(a,b):
            self.mismatch_count+=weight
            if len(self.mismatches)<100:self.mismatches.append({'field':label,'original':a,'Rust':b})
    def numeric(self,a,b,label,unit,weight=1):
        self.count+=weight
        a=float(a);b=float(b)
        classify=lambda v: 'nan' if math.isnan(v) else 'positive_infinity' if v==math.inf else 'negative_infinity' if v==-math.inf else 'negative_zero' if v==0 and math.copysign(1,v)<0 else 'positive_zero' if v==0 else 'finite'
        ca,cb=classify(a),classify(b)
        if ca!=cb:
            self.mismatch_count+=weight
            if len(self.mismatches)<100:self.mismatches.append({'field':label,'original_class':ca,'Rust_class':cb})
            return
        if not math.isfinite(a):return
        atol={'degC':1e-10,'kg/kg':1e-12,'Pa':1e-8,'J/kg':1e-7,'exact':0}[unit];rtol=0 if unit=='exact' else 1e-13
        delta=abs(a-b)
        if delta>atol+rtol*abs(a):
            self.mismatch_count+=weight
            if len(self.mismatches)<100:self.mismatches.append({'field':label,'absolute_error':delta,'atol':atol,'rtol':rtol})
        s=self.stats.setdefault(label,{'count':0,'max_absolute_error':0.,'sum_squared_error':0.,'unit':unit})
        s['count']+=weight;s['max_absolute_error']=max(s['max_absolute_error'],delta);s['sum_squared_error']+=weight*delta*delta
    def scalar_bits(self,a,b,key,label,unit,weight=1):
        self.numeric(fp_value(int(a[key+'_bits'],16)),fp_value(int(b[key+'_bits'],16)),label,unit,weight)
    def state(self,a,b,label,weight=1):
        for key in ['warmup','use_interpolation']:self.exact(a[key],b[key],label+'.'+key,weight)
        for key,unit in [('iconv_tol','exact'),('last_patm','Pa'),('last_t_boil','degC'),('press_save','Pa'),('t_sat_save','degC')]:self.scalar_bits(a,b,key,label+'.'+key,unit,weight)
    def cache(self,a,b,label,weight=1):
        if a is None or b is None:self.exact(a,b,label,weight);return
        for key in ['kind','index','i_tdb','i_w','i_pb','i_h','requested_tags']:
            self.exact(a.get(key),b.get(key),label+'.'+key,weight)
        unit='Pa' if a['kind']=='Psat' else 'degC'
        self.scalar_bits(a,b,'value',label+'.value',unit,weight)
    def root(self,a,b,weight=1):
        function=a['function'];unit=self.tolerances[function]['unit']
        for key in ['function','input_bits','phase','caller','context']:self.exact(a.get(key),b.get(key),function+'.'+key,weight)
        self.numeric(fp_value(int(a['value_bits'],16)),fp_value(int(b.get('value_bits',b.get('result_bits')),16)),function+'.value',unit,weight)
        self.exact(a['cache_hit'],b['cache_hit'],function+'.cache_hit',weight)
        for phase in ['before','after']:
            self.state(a['state_'+phase],b['state_'+phase],function+'.state_'+phase,weight)
            self.cache(a['cache_'+phase],b['cache_'+phase],function+'.cache_'+phase,weight)
        if a['cache_hit'] is True:
            self.exact(a['value_bits'],a['cache_before']['value_bits'],function+'.original_hit_stored_bits',weight)
            self.exact(b.get('value_bits',b.get('result_bits')),b['cache_before']['value_bits'],function+'.Rust_hit_stored_bits',weight)
    def final(self,a,b):
        self.state(a['initial_state'],b['initial_state'],'initial_state')
        self.state(a['final_state'],b['final_state'],'final_state')
        self.exact(sorted(a['final_caches']),sorted(b['final_caches']),'final_cache_kinds')
        for kind,rows in a['final_caches'].items():
            other=b['final_caches'][kind];self.exact(len(rows),len(other),'final_caches.'+kind+'.length')
            self.exact([r['index'] for r in rows],sorted(set(r['index'] for r in rows)),'original_final_sorted_unique.'+kind)
            self.exact([r['index'] for r in other],sorted(set(r['index'] for r in other)),'Rust_final_sorted_unique.'+kind)
            for x,y in zip(rows,other):self.cache(x,y,'final_caches.'+kind)
    def report(self):
        for s in self.stats.values():s['rmse']=math.sqrt(s['sum_squared_error']/s['count']) if s['count'] else 0.
        return {'status':'pass' if self.mismatch_count==0 else 'fail','mismatch_count':self.mismatch_count,'mismatch_samples':self.mismatches,'comparison_count':self.count,'statistics':self.stats}

def production_trace(trace_path,directory,driver_receipt):
    require(not directory.exists(),'Use a fresh production-replay directory');check();trace=read(trace_path)
    require(trace['schema']=='psy02-calls.v1' and trace['complete_on_collecting_thread'] is True and trace['omitted_root_count']==0 and trace['truncation_reason'] is None,'Incomplete/invalid root trace')
    require(trace['total_root_count']==len(trace['ordered_ids']),'Root count/order mismatch')
    require(trace['recorded_root_count']==len(trace['ordered_ids']),'Recorded root count mismatch')
    require(all(type(i) is int and 0<=i<len(trace['dictionary']) for i in trace['ordered_ids']),'Invalid actual dictionary ID')
    build=read(driver_receipt);require(build['checks_passed'] is True,'Unverified original driver');binary=ROOT/build['binary']['path'];require(ref(binary)==build['binary'],'Changed archived driver')
    directory.mkdir(parents=True);executed=directory/'psy02_reference.executed.py';shutil.copyfile(Path(__file__),executed)
    env=os.environ.copy();env['PATH']=str(ROOT/'.runtime/reference-tools/winlibs-gcc13.2.0-ucrt-r3/mingw64/bin')+os.pathsep+env.get('PATH','')
    receipt=process([str(binary),str(trace_path)],directory,'reference',env);require(receipt['exit_code']==0,'Original root replay failed; raw logs retained')
    original=read(directory/'reference-stdout.log');require(original['schema']=='psy02-reference-roots.v1' and original['every_ordered_root_evaluated'] is True and original['total_root_count']==trace['total_root_count'],'Invalid original root replay')
    require(len(original['ordered_ids'])==len(trace['ordered_ids']),'Original root order length mismatch')
    require(all(type(i) is int and 0<=i<len(original['dictionary']) for i in original['ordered_ids']),'Invalid original dictionary ID')
    comparison=Comparison();pairs=Counter(zip(trace['ordered_ids'],original['ordered_ids']))
    for (actual_id,original_id),weight in pairs.items():
        actual=trace['dictionary'][actual_id];cpp=original['dictionary'][original_id]
        comparison.exact(cpp['input_dictionary_id'],actual_id,'ordered_input_dictionary_id',weight)
        rust={**actual,'function':actual['routine']}
        comparison.root(cpp,rust,weight)
    comparison.final(original,trace)
    result=comparison.report();result.update({'schema':'psy02-production-root-comparison.v1','total_root_count':trace['total_root_count'],'omitted_root_count':0,'ordered_root_inputs_checked':True,'final_cache_tables_checked':True,'weighted_unique_pairs':len(pairs),'every_original_ordered_root_evaluated':True,'source_warning_lifecycle_is_paired':False,'internal_iteration_count_parity_claimed':False,'trace':ref(trace_path),'original_stdout':receipt['stdout'],'original_stderr':receipt['stderr'],'original_binary':ref(binary),'native_driver_build':ref(driver_receipt),'native_core_build':ref(CORE_RECEIPT),'executed_checker':ref(executed),'contracts':[ref(CONTRACTS/n) for n in ['PSY-02-source.json','PSY-02-cases.json','PSY-02-tolerances.json']],'command':receipt['command'],'command_exit_code':receipt['exit_code'],'gates_updated':False})
    write(directory/'comparison-report.json',result);return result

def scalar_value(value):
    if isinstance(value,str):return {'NaN':math.nan,'+Infinity':math.inf,'-Infinity':-math.inf}[value]
    return float(value)

def compare_units(original_dir,rust_dir,directory):
    require(not directory.exists(),'Use a fresh unit-comparison directory');check();directory.mkdir(parents=True)
    executable=directory/'psy02_reference.executed.py';shutil.copyfile(Path(__file__),executable)
    comparison=Comparison();artifacts=[];paired=0;unpaired=0
    for process in read(CONTRACTS/'PSY-02-cases.json')['processes']:
        name=process['id'];cpp_receipt_path=original_dir/name/'reference.json';receipt=read(cpp_receipt_path)
        require(receipt['exit_code']==0 and receipt['reference_kind']=='unchanged-linked-original-and-header-with-genuine-state','Original first peer unavailable')
        require(receipt['contract_refs']==[ref(CONTRACTS/n) for n in ['PSY-02-source.json','PSY-02-cases.json','PSY-02-tolerances.json']],'Original frozen contract differs')
        for key in ['binary','request','results','stdout','stderr','driver_build','native_core_build','executed_launcher']:
            require(ref(ROOT/receipt[key]['path'])==receipt[key],'Original execution reference changed: '+key)
        require(read(ROOT/receipt['request']['path'])==process['request'],'Original numerical request changed')
        cpp=read(ROOT/receipt['results']['path']);require(cpp==read(ROOT/receipt['stdout']['path']),'Original result differs from actual stdout')
        rust_path=rust_dir/name/'results.json';rust=read(rust_path)
        require(cpp['schema']==rust['schema']=='psy02-results.v1','Unit response schema mismatch')
        require(len(cpp['calls'])==len(rust['calls'])==len(process['request']['calls']),'Unit ordered row count mismatch')
        artifacts.extend([ref(cpp_receipt_path),ref(rust_path)])
        earlier_cpp={};earlier_rust={}
        for expected,a,b in zip(process['request']['calls'],cpp['calls'],rust['calls']):
            f=expected['function'];index=expected['call_index']
            for key in expected:
                comparison.exact(a.get(key),expected[key],f+'.original_request_identity.'+key)
                comparison.exact(b.get(key),expected[key],f+'.Rust_request_identity.'+key)
            if f=='General::Iterate':
                require(b.get('status')=='unsupported_source_only','General diagnostic must remain explicitly unpaired');unpaired+=1;continue
            require(f in FUNCTIONS,'Unfrozen unit routine');paired+=1
            comparison.exact(a['unit'],FUNCTIONS[f][1],f+'.original_output_unit')
            comparison.exact(b['unit'],FUNCTIONS[f][1],f+'.Rust_output_unit')
            for key,value in expected['inputs'].items():
                if isinstance(value,dict) and 'from_call' in value:
                    previous=value['from_call'];require(previous in earlier_cpp and previous in earlier_rust,'Invalid own-result reference')
                    for out,own,label in [(a,earlier_cpp,'original'),(b,earlier_rust,'Rust')]:
                        resolved=scalar_value(out['resolved_inputs'][key]);producer=own[previous]
                        comparison.exact(fp_bits(resolved),int(producer['value_bits'],16),f+'.'+label+'.own_resolved_result_bits')
                    producer_function=earlier_cpp[previous]['function'];unit=FUNCTIONS[producer_function][1]
                    comparison.numeric(scalar_value(a['resolved_inputs'][key]),scalar_value(b['resolved_inputs'][key]),f+'.own_input.'+key,unit)
                else:
                    if isinstance(value,str):
                        comparison.exact(a['resolved_inputs'][key],value,f+'.original_resolved_IEEE.'+key);comparison.exact(b['resolved_inputs'][key],value,f+'.Rust_resolved_IEEE.'+key)
                    else:
                        comparison.exact(fp_bits(float(a['resolved_inputs'][key])),fp_bits(float(value)),f+'.original_literal_input_bits.'+key)
                        comparison.exact(fp_bits(float(b['resolved_inputs'][key])),fp_bits(float(value)),f+'.Rust_literal_input_bits.'+key)
            comparison.root(a,b)
            comparison.exact(a['value_class'],b['value_class'],f+'.value_class')
            earlier_cpp[index]=a;earlier_rust[index]=b
        comparison.final(cpp,rust)
    report=comparison.report();report.update({'schema':'psy02-unit-comparison.v1','paired_numerical_calls':paired,'source_only_General_calls':unpaired,'original_source_warning_lifecycle_paired':False,'original_internal_iteration_count_observed':False,'executed_checker':ref(executable),'artifacts':artifacts,'contracts':[ref(CONTRACTS/n) for n in ['PSY-02-source.json','PSY-02-cases.json','PSY-02-tolerances.json']],'gates_updated':False})
    write(directory/'comparison-report.json',report);return report

def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--prepare',action='store_true');parser.add_argument('--check',action='store_true');parser.add_argument('--check-metadata',action='store_true');parser.add_argument('--build',action='store_true');parser.add_argument('--driver-target',choices=['psy02_reference','psy02_reference_replay'],default='psy02_reference');parser.add_argument('--run-original',action='store_true');parser.add_argument('--production-trace',type=Path);parser.add_argument('--compare-units',action='store_true');parser.add_argument('--original-dir',type=Path);parser.add_argument('--rust-dir',type=Path);parser.add_argument('--output-dir',type=Path);parser.add_argument('--driver-build',type=Path);args=parser.parse_args()
    if sum([args.prepare,args.check,args.check_metadata,args.build,args.run_original,args.production_trace is not None,args.compare_units])!=1:parser.error('Choose exactly one operation')
    if args.build or args.run_original or args.production_trace or args.compare_units:require(args.output_dir is not None,'output-dir required')
    if args.run_original or args.production_trace:require(args.driver_build is not None,'driver-build required')
    if args.compare_units:require(args.original_dir is not None and args.rust_dir is not None,'original-dir and rust-dir required')
    result=build_driver(args.output_dir.resolve(),args.driver_target) if args.build else run_original(args.output_dir.resolve(),args.driver_build.resolve()) if args.run_original else production_trace(args.production_trace.resolve(),args.output_dir.resolve(),args.driver_build.resolve()) if args.production_trace else compare_units(args.original_dir.resolve(),args.rust_dir.resolve(),args.output_dir.resolve()) if args.compare_units else check_metadata() if args.check_metadata else prepare() if args.prepare else check()
    print(json.dumps(result));return 0
if __name__=='__main__':raise SystemExit(main())
