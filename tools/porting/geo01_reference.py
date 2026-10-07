#!/usr/bin/env python3
"""Input-only GEO-01 contracts and read-only, original native model observations.

No geometry formula, expected answer, Rust edit or automatic gate update belongs here.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
import math
import os
from pathlib import Path
import re
import shutil
import struct
import subprocess
import sys
import time

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / ".reference/energyplus-src/26.1.0"
CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
EVIDENCE = ROOT / "energyplus_porting_plan/evidence/GEO-01"
RAW = ROOT / ".runtime/porting/GEO-01"
BUILD = ROOT / ".runtime/ep261-gcc13-o0"
CORE_RECEIPT = ROOT / ".runtime/porting/reference-energyplus-26.1.0/native-core-build.json"
PIN = "6f2e40d10250a105b49966baa24d843711e61048"
RANGES = [
    ("SurfaceGeometry.cc", "SetupZoneGeometry original trig cache and input caller", 251, 311, ["GetSurfaceData", "std::cos", "std::sin"], "selected"),
    ("SurfaceGeometry.cc", "GetSurfaceData one-time control and GGR caller", 1051, 1064, ["GetGeometryParameters"], "selected-input-context"),
    ("SurfaceGeometry.cc", "GetSurfaceData detailed surface input caller", 1154, 1167, ["GetHTSurfaceData"], "selected-input-context"),
    ("SurfaceGeometry.cc", "GetGeometryParameters valid GGR branch", 3225, 3302, ["InputProcessor::getObjectItem", "Util::FindItem", "Util::SameString"], "selected"),
    ("SurfaceGeometry.cc", "GetHTSurfaceData Detailed parser and class", 3888, 3936, ["InputProcessor::getObjectItem"], "selected-input-context"),
    ("SurfaceGeometry.cc", "GetHTSurfaceData construction and zone binding", 3938, 3986, ["Util::FindItemInList"], "selected-input-context"),
    ("SurfaceGeometry.cc", "GetHTSurfaceData vertex count allocation and GetVertices", 4294, 4348, ["GetVertices", "CheckConvexity"], "selected"),
    ("SurfaceGeometry.cc", "GetVertices declaration, counter, copy, winding, corner and coordinates", 9169, 9314, ["SetupZoneGeometry trig cache"], "selected"),
    ("SurfaceGeometry.cc", "checkPopCoincidentVertex inactive coincident removal", 9115, 9167, [], "inactive-domain-guard"),
    ("SurfaceGeometry.cc", "GetVertices downstream removal/orientation/aspect/geometry", 9316, 9485, ["checkPopCoincidentVertex", "ReverseAndRecalculate", "TransformVertsByAspect", "Vectors::CreateNewellSurfaceNormalVector", "Vectors::CreateNewellAreaVector", "Vectors::DetermineAzimuthAndTilt"], "downstream-original-context-not-GEO02-GEO03-closure"),
    ("SurfaceGeometry.cc", "GetSurfaceData final owned array relocation", 1504, 1530, [], "immutable-array-relay"),
    ("SurfaceGeometry.cc", "GetSurfaceData temporary array deallocation", 1729, 1735, [], "state-lifetime-context"),
    ("SurfaceGeometry.cc", "SetupZoneGeometry processed vertex consumer", 343, 356, ["ProcessSurfaceVertices"], "GEO02-GEO03-consumer-context-only"),
    ("SurfaceGeometry.cc", "ProcessSurfaceVertices base-surface vertex readers", 12899, 12977, [], "GEO02-GEO03-consumer-context-only"),
    ("SurfaceGeometry.cc", "TransformVertsByAspect absent-object early return", 14391, 14474, [], "inactive-domain-guard"),
    ("SurfaceGeometry.hh", "selected routine declarations", 89, 130, [], "declarations"),
    ("SurfaceGeometry.hh", "GetVertices declaration", 278, 282, [], "declarations"),
    ("SurfaceGeometry.hh", "geometry initialization owner and flags", 442, 478, [], "state-contract"),
    ("SurfaceGeometry.hh", "geometry clear-state defaults", 509, 534, [], "state-contract"),
    ("DataSurfaces.hh", "four corner enum constants", 304, 307, [], "constants"),
    ("DataSurfaces.hh", "owned surface name/class/sides/vertices", 687, 746, [], "state-contract"),
    ("DataSurfaces.hh", "counter and geometry flags constructor defaults", 1489, 1500, [], "state-contract"),
    ("DataHeatBalance.hh", "zone relative north and origin defaults", 573, 581, [], "state-contract"),
    ("DataHeatBalance.hh", "actual selected zone Volume consumer field", 438, 443, [], "GEO03-consumer-context-only"),
    ("DataHeatBalance.hh", "BuildingAzimuth", 1809, 1813, [], "state-contract"),
    ("DataHeatBalance.hh", "BuildingRotationAppendixG", 1837, 1840, [], "state-contract"),
    ("DataErrorTracking.hh", "inactive geometry error counters", 214, 220, [], "state-contract"),
    ("DataGlobalConstants.hh", "distance and angle constants", 582, 591, [], "constants"),
    ("HeatBalanceManager.cc", "Building angle modulo prerequisite", 559, 565, [], "CON01-prerequisite"),
    ("HeatBalanceManager.cc", "Zone rotation and origin parsed input", 2311, 2326, [], "selected-input-context"),
    ("HeatBalanceManager.cc", "SetupZoneGeometry initialization call", 1815, 1823, ["SetupZoneGeometry"], "selected-input-context"),
    ("HeatBalanceManager.cc", "geometry input precedes before-init callback", 147, 198, [], "callback-phase-proof"),
    ("api/runtime.cc", "public native before-init callback", 187, 197, [], "opaque-public-API"),
    ("InputProcessing/InputProcessor.hh", "genuine parsed epJSON", 307, 319, [], "input-bit-observation"),
    ("CMakeLists.txt", "original directory compile definitions", 25, 45, [], "ABI-provenance"),
]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha(path):
    h = hashlib.sha256()
    with Path(path).open("rb") as f:
        for block in iter(lambda: f.read(1048576), b""):
            h.update(block)
    return h.hexdigest()


def ref(path):
    p = Path(path).resolve()
    return {"path": p.relative_to(ROOT).as_posix(), "sha256": sha(p)}


def read(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def write(path, data):
    p = Path(path)
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(data, ensure_ascii=False, indent=2, allow_nan=False) + "\n", encoding="utf-8", newline="\n")


def bits(value):
    return struct.pack(">d", float(value)).hex()


def objects(text):
    clean = re.sub(r"!.*", "", text)
    return [[v.strip() for v in row.split(",")] for row in clean.split(";") if row.strip()]


def named(objs, kind):
    rows = [x for x in objs if x[0].casefold() == kind.casefold()]
    require(len(rows) == 1, f"Need exactly one {kind}")
    return rows[0]


def input_projection(objs):
    building = named(objs, "Building")
    zone = named(objs, "Zone")
    rules = named(objs, "GlobalGeometryRules")
    surfaces = []
    for row in objs:
        if row[0].casefold() != "buildingsurface:detailed":
            continue
        # 26.1 Detailed includes an optional Space field before boundary fields.
        require(len(row) == 24 and row[11] == "4", "Only explicit exact four-vertex surfaces in this contract")
        require(row[4].upper() == zone[1].upper(), "Detailed surface zone binding differs")
        coords = [float(x) for x in row[12:]]
        require(all(math.isfinite(x) for x in coords), "Finite coordinates required")
        surfaces.append({"name": row[1], "canonical_name": row[1].upper(), "zone_name": row[4], "class": row[2],
                         "source_order": len(surfaces), "declared_number_of_vertices": 4,
                         "input_vertices_m": [coords[i:i+3] for i in range(0, 12, 3)],
                         "input_vertex_bits": [[bits(x) for x in coords[i:i+3]] for i in range(0, 12, 3)],
                         "input_vertex_lexemes": [row[12+i:15+i] for i in range(0, 12, 3)]})
    require(len(surfaces) == 6, "Need six opaque bounded faces")
    require(not any(x[0].casefold() in {"geometrytransform", "compliance:building"} for x in objs), "Excluded transform control")
    return {"building_raw_north_axis_deg": float(building[2]), "appendix_g_rotation_deg": 0,
            "rules": {"starting_vertex_position": rules[1], "vertex_entry_direction": rules[2], "coordinate_system": rules[3]},
            "zones": [{"name": zone[1], "source_order": 0, "relative_north_deg": float(zone[2]), "origin_m": [float(x) for x in zone[3:6]]}],
            "surfaces": surfaces}


def diagnostic_specs():
    cases = [
        ("WORLD-IGNORED-AXES", "World", "UpperLeftCorner", "CounterClockWise", 37.25, -23.5, (3.125, -4.75, 1.5), None),
        ("WORLD-NEGATIVE-ZERO", "World", "UpperLeftCorner", "CounterClockWise", 0, 0, (0, 0, 0), "all"),
        ("WORLD-X-NEGATIVE-ZERO", "World", "UpperLeftCorner", "CounterClockWise", 0, 0, (0, 0, 0), "x"),
        ("RELATIVE-ZERO", "Relative", "UpperLeftCorner", "CounterClockWise", 0, 0, (0, 0, 0), None),
        ("RELATIVE-BUILDING90", "Relative", "UpperLeftCorner", "CounterClockWise", 90, 0, (0, 0, 0), None),
        ("RELATIVE-ZONE90", "Relative", "UpperLeftCorner", "CounterClockWise", 0, 90, (0, 0, 0), None),
        ("RELATIVE-TRANSLATE", "Relative", "UpperLeftCorner", "CounterClockWise", 0, 0, (3.125, -4.75, 1.5), None),
        ("RELATIVE-COMPOSED", "Relative", "UpperLeftCorner", "CounterClockWise", 37.25, -23.5, (3.125, -4.75, 1.5), None),
        ("RELATIVE-BUILDING-NEG450", "Relative", "UpperLeftCorner", "CounterClockWise", -450, 0, (0, 0, 0), None),
        ("RELATIVE-BUILDING765", "Relative", "UpperLeftCorner", "CounterClockWise", 765, 0, (0, 0, 0), None),
        ("RELATIVE-ZONE450", "Relative", "UpperLeftCorner", "CounterClockWise", 0, 450, (0, 0, 0), None),
        ("RELATIVE-NEAR-ZERO", "Relative", "UpperLeftCorner", "CounterClockWise", 1e-12, -1e-12, (1e-12, -1e-12, 0), None),
        ("RELATIVE-NEGATIVE-ZERO", "Relative", "UpperLeftCorner", "CounterClockWise", "-0.0", "-0.0", ("-0.0", "-0.0", "-0.0"), "all"),
    ]
    for corner in ["UpperLeftCorner", "LowerLeftCorner", "LowerRightCorner", "UpperRightCorner"]:
        for direction in ["CounterClockWise", "ClockWise"]:
            cases.append((f"WORLD-{corner.upper()}-{direction.upper()}", "World", corner, direction, 0, 0, (0, 0, 0), None))
    cases.append(("RELATIVE-CW-LOWERRIGHT", "Relative", "LowerRightCorner", "ClockWise", 37.25, -23.5, (3.125, -4.75, 1.5), None))
    return cases


def source_contract():
    selected = []
    files = {}
    for filename, symbol, start, end, helpers, role in RANGES:
        path = "src/EnergyPlus/" + filename
        lines = (SOURCE / path).read_text(encoding="utf-8").splitlines(keepends=True)
        require(end <= len(lines), f"Invalid source range {symbol}")
        files[path] = {"path": path, "sha256": sha(SOURCE / path)}
        body = "".join(lines[start-1:end])
        tokens = ["SetupZoneGeometry", "GetGeometryParameters", "GetHTSurfaceData", "GetVertices", "checkPopCoincidentVertex", "TransformVertsByAspect", "ProcessSurfaceVertices", "SurfaceGeometryData", "clear_state", "UpperLeftCorner", "SurfaceData", "MaxVerticesPerSurface", "RelNorth", "BuildingAzimuth", "BuildingRotationAppendixG", "TotalCoincidentVertices", "DegToRad", "callbackBeginZoneTimeStepBeforeInitHeatBalance", "epJSON", "EP_psych_errors", "Volume", "surfTemp", "ZoneLoop", "GetObjectItem"]
        owner = symbol.split(" ", 1)[0]
        literal = owner if owner in body else next((x for x in tokens if x in body), None)
        if literal is None:
            literal = "SurfaceTmp" if "SurfaceTmp" in body else "GetSurfaceDataOneTimeFlag"
        require(literal in body, "Source symbol absent from range " + symbol)
        selected.append({"path": path, "symbol": literal, "description": symbol, "function_owner": owner,
                         "start_line": start, "end_line": end,
                         "helpers": helpers, "role": role,
                         "range_sha256": hashlib.sha256("".join(lines[start-1:end]).encode()).hexdigest()})
    for path in ["src/EnergyPlus/Data/EnergyPlusData.hh", "src/EnergyPlus/Data/EnergyPlusData.cc"]:
        files[path] = {"path": path, "sha256": sha(SOURCE / path)}
    return {"schema": "geo01-source-contract.v1", "card": "GEO-01", "energyplus_commit": PIN,
            "named_card_functions": ["GetVertices", "GetHTSurfaceData"],
            "source_files": list(files.values()), "selected_ranges": selected,
            "coordinate_order": ["original XYZ buffer copy", "CW reverses vertices2..N preservingvertex1", "original corner swap loop",
                "Relative: zone rotation then XY origin then building rotation; Z adds zone origin", "World: AppendixG-only XY multiply/add still executes, fixedangle0"],
            "angle_constants": "Original Constant::Pi/180.0 DegToRad multiply; building raw angle source mod(value,360.0) is CON01 prerequisite",
            "state_contract": {"owner": "original constructor/GetGeometryParameters/SetupZoneGeometry/GetHTSurfaceData initialization then retained owned Surface.Vertex",
                "first_phase": "first callbackBeginZoneTimeStepBeforeInitHeatBalance after original geometry input/setup, possibly warmup; full GetVertices has already returned",
                "final_phase": "energyplus return before stateDelete; arrays compared to first initialized bytes",
                "cold_defaults": {"Corner": 0, "CCW": False, "WorldCoordSystem": False, "MaxVerticesPerSurface": 4,
                    "geometry_coefficients": 0.0, "firstTime": True, "noTransform": True, "GetSurfaceDataOneTimeFlag": False},
                "native_only": ["MaxVerticesPerSurface before/after (no Rust lifecycle counter owner; four-vertex inputs keepgrowth inactive)", "source trig coefficients, zone trig arrays allocated then genuinelydeallocated; never reconstructed", "TotalCoincidentVertices/TotalDegenerateSurfaces", "geometry flags"],
                "paired_projection": "case-normalized surface/zone names, exact cardinality/order, actual flags and stored world coordinates; own indices retained independently"},
            "reference": {"method": "unchanged full original dynamic API run; genuine same-GCC DLL constructs/parses/initializes/destroys its own EnergyPlusData; const private reads only",
                "core_receipt": ref(CORE_RECEIPT), "ABI": "same GCC13.2 headers/options; original directory+target COMPILE_DEFINITIONS; project_options/project_fp_options/project_warnings",
                "prohibited": ["rewritten geometry equations as oracle", "fake EnergyPlusData", "MSVC private-state cast", "expectedvertices in Rust inputs", "observer geometry/probe calls or state mutation"]},
            "domain": {"production": "unchanged CON15 World/UpperLeft/CCW zero Building/Zone controls; onlyselectedinput topology, not universalCLI admission predicate",
                "diagnostics": "same A24 valid closed opaque six-face model with declared4=actual4; Relative, axes/origins/corners/CW and finitezero inputvariants; no production scope expansion",
                "inactive_required": ["coincident/degenerate removal", "roof/floor auto-reversal", "GeometryTransform", "AppendixG nonzero", "subsurface/detached/simple geometry branches"],
                "max_counter_growth": "inactive with explicit four-vertex domain; true native default4 retained, never surrogateRust0"},
            "integration_connection": "Selected CON physical snapshots compare actual consumed area/azimuth/tilt and zonevolume with original final source consumers only; this establishes vertices connection, never GEO02/GEO03 algorithm closure or diagnostic dry-run consumerproof",
            "excluded": ["GEO02/GEO03 area/normal/azimuth/tilt/centroid/volume algorithm closure", "invalid/missing GGR and invalid/extra/auto vertex-count parsing", "triangle/5+ vertex corner and counter-growth branches", "nonfinite geometry inputs", "aspect/AppendixG transform and full shading/windows/air-boundary families", "whole building/HVAC numerical equivalence"],
            "comparison_status": "not_run", "gates_updated": False}


def prepare():
    rows = []
    scope = read(CONTRACTS / "scope.json")
    for case in scope["cases"]:
        row = {k: copy.deepcopy(case[k]) for k in ["id", "scope", "duration", "input", "weather"]}
        row["kind"] = "con-production-input"
        row["metadata"] = copy.deepcopy(case["metadata"])
        row["geometry_input"] = input_projection(objects((ROOT / case["input"]["path"]).read_text(encoding="utf-8")))
        rows.append(row)
    base_path = ROOT / scope["cases"][0]["input"]["path"]
    base = objects(base_path.read_text(encoding="utf-8"))
    corners = ["UpperLeftCorner", "LowerLeftCorner", "LowerRightCorner", "UpperRightCorner"]
    for ident, coord, corner, direction, building_angle, zone_angle, origin, zero_policy in diagnostic_specs():
        objs = copy.deepcopy(base)
        named(objs, "Building")[2] = str(building_angle)
        named(objs, "Zone")[2:6] = [str(zone_angle), *[str(v) for v in origin]]
        named(objs, "GlobalGeometryRules")[1:4] = [corner, direction, coord]
        for row in objs:
            if row[0].casefold() != "buildingsurface:detailed":
                continue
            vertices = [row[12+i:15+i] for i in range(0, 12, 3)]
            offset = corners.index(corner)
            vertices = vertices[offset:] + vertices[:offset]
            if direction == "ClockWise":
                vertices[1:] = vertices[1:][::-1]
            for vertex in vertices:
                for axis, token in enumerate(vertex):
                    if float(token) == 0 and (zero_policy == "all" or (zero_policy == "x" and axis == 0)):
                        vertex[axis] = "-0.0"
            row[12:] = [v for vertex in vertices for v in vertex]
        patches = []
        for before, after in zip(base, objs):
            if before != after:
                patches.append({"object_type": before[0], "name": before[1], "before_fields": before[1:], "after_fields": after[1:]})
        target = ROOT / f"energyplus_porting_plan/cases/GEO-01/{ident}/input.idf"
        text = "! GEO-01 input-only geometry diagnostic; no expected vertices.\n" + "\n\n".join(x[0]+",\n  "+",\n  ".join(x[1:])+";" for x in objs) + "\n"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text, encoding="utf-8", newline="\n")
        metadata = {"schema": "geo01-case-metadata.v1", "id": ident, "kind": "source-unit-diagnostic", "base_input": ref(base_path),
            "input": ref(target), "weather": scope["cases"][0]["weather"], "patches": patches,
            "physical_scope_expansion": False, "construction_schedule_weather_controls_unchanged": True,
            "input_polygon_policy": "cyclic starting-corner permutation then clockwise reversal preserving first vertex; no geometry transform or expected answer",
            "geometry_input": input_projection(objs)}
        write(target.parent / "metadata.json", metadata)
        rows.append({"id": ident, "kind": "source-unit-diagnostic", "scope": None, "duration": "24H",
            "input": ref(target), "weather": scope["cases"][0]["weather"], "metadata": ref(target.parent / "metadata.json"),
            "geometry_input": metadata["geometry_input"]})
    case_contract = {"schema": "geo01-input-cases.v1", "card": "GEO-01", "production_case_count": 15,
        "diagnostic_case_count": len(diagnostic_specs()), "case_count": len(rows), "cases": rows,
        "request_policy": "input-only same IDFs; original native parser and normal Rust IDF→source-assisted lexical epJSON conversion→compiler; no independently assembled models/expectedanswers",
        "zero_policy": "negativezero lexemes are inputs; actual original/Rust parsed input bits must be retained, not assume signsurvival from text",
        "gates_updated": False}
    tolerance = {"schema": "geo01-tolerances.v1", "card": "GEO-01", "frozen_before_comparison": True,
        "profiles": {"world_vertices_m": {"unit": "m", "absolute_tolerance": 1e-10, "relative_tolerance": 1e-13,
            "rule": "abs(actual-reference) <= atol+rtol*abs(reference)", "finite_class": "exact",
            "zero_sign": "exact when both numerical outputs zero and same actual parsed inputsign established; inputparser discrepancies separatelyfail identity, never infer preservednegativezero from lexeme",
            "nonzero_bits": "recorded; no blanketbit-equality requirement across compiler libm"},
            "connection_area_m2": {"unit": "m2", "absolute_tolerance": 1e-10, "relative_tolerance": 1e-13, "scope": "selected CON actual physical consumers only; no GEO02 algorithm closure"},
            "connection_zone_volume_m3": {"unit": "m3", "absolute_tolerance": 1e-9, "relative_tolerance": 1e-13, "scope": "selected CON actual physical consumers only; no GEO03 algorithm closure"},
            "connection_azimuth_deg": {"unit": "deg", "absolute_tolerance": 1e-10, "relative_tolerance": 1e-13, "scope": "selected CON actual physical consumers only; no GEO02 algorithm closure"},
            "connection_tilt_deg": {"unit": "deg", "absolute_tolerance": 1e-10, "relative_tolerance": 1e-13, "scope": "selected CON actual physical consumers only; no GEO02 algorithm closure"}},
        "exact_metadata": ["canonical names", "surface/zone binding", "vertex cardinality and arrayorder", "typed GGR flags", "recorded input coordinates after lexicalconversion", "first/final original retained vertex bits", "inactive-domain predicates"],
        "native_only_unpaired": ["engine IDs", "MaxVerticesPerSurface lifecycle", "original trig owner state/caches/deallocation", "normal/centroid and diagnostic-only area/azimuth/tilt/volume consumers; selected CON physical area/volume/azimuth/tilt are paired stored-field connection projections under the separate profiles"],
        "forbidden": ["tolerance widening after failure", "expectedoriginal outputs used as Rust input", "precomputed geometry fixture answers", "GEO02/GEO03 or wholephysics gate inference"], "gates_updated": False}
    write(CONTRACTS / "GEO-01-source.json", source_contract())
    write(CONTRACTS / "GEO-01-cases.json", case_contract)
    write(CONTRACTS / "GEO-01-tolerances.json", tolerance)
    return {"case_count": len(rows), "contracts": [ref(CONTRACTS / f"GEO-01-{name}.json") for name in ["source", "cases", "tolerances"]]}


def check():
    source = read(CONTRACTS / "GEO-01-source.json")
    require(source == source_contract(), "Source contract differs from pinned source/policy")
    cases = read(CONTRACTS / "GEO-01-cases.json")
    require(cases["case_count"] == len(cases["cases"]) == 37 and cases["production_case_count"] == 15, "Fixed case cardinality changed")
    for row in cases["cases"]:
        for key in ["input", "weather", "metadata"]:
            require(ref(ROOT / row[key]["path"]) == row[key], f"{row['id']} {key} changed")
        require(input_projection(objects((ROOT / row["input"]["path"]).read_text(encoding="utf-8"))) == row["geometry_input"], "Recorded geometryinputs changed")
    require(len({x["id"] for x in cases["cases"]}) == 37, "Duplicate input identity")
    return {"status": "pass", "case_count": 37, "source_files": len(source["source_files"]), "selected_ranges": len(source["selected_ranges"]), "comparison_run": False}


def process(command, directory, name, env):
    start = time.perf_counter()
    with (directory / f"{name}-stdout.log").open("wb") as out, (directory / f"{name}-stderr.log").open("wb") as err:
        p = subprocess.run(command, cwd=ROOT, env=env, stdout=out, stderr=err, check=False)
    receipt = {"command": command, "exit_code": p.returncode, "elapsed_seconds": round(time.perf_counter()-start, 3),
               "stdout": ref(directory / f"{name}-stdout.log"), "stderr": ref(directory / f"{name}-stderr.log")}
    write(directory / f"{name}-command.json", receipt)
    return receipt


def environment():
    env = os.environ.copy()
    env.update(PYTHONUTF8="1", PYTHONIOENCODING="utf-8")
    env["PATH"] = str(ROOT / ".runtime/reference-tools/winlibs-gcc13.2.0-ucrt-r3/mingw64/bin") + os.pathsep + env.get("PATH", "")
    return env


def build_driver(directory):
    require(not directory.exists(), "Use fresh native driver build-receipt directory")
    check()
    core = read(CORE_RECEIPT)
    require(core["checks_passed"] is True and core["energyplus_commit"] == PIN, "Unverified native core")
    baseline = read(ROOT / ".runtime/porting/PSY-02/native-driver-replay-target-frozen/configure-command.json")
    command = ["-DCMAKE_PROJECT_INCLUDE=" + str(ROOT / "tools/porting/geo01_reference.cmake") if x.startswith("-DCMAKE_PROJECT_INCLUDE=") else x for x in baseline["command"]]
    before = read(BUILD / "compile_commands.json")
    frozen_path = ROOT / ".runtime/porting/reference-energyplus-26.1.0/provenance/final-core/compile_commands.json"
    frozen = read(frozen_path)
    require(all(row in before for row in frozen), "Frozen original compilecommands changed")
    old_products = [p for p in (BUILD / "Products").glob("*.exe") if p.name.startswith(("clk01_", "psy02_"))]
    old_refs = [ref(p) for p in old_products]
    objexx = BUILD / "Products/libobjexx.a"
    objexx_ref = {**ref(objexx), "bytes": objexx.stat().st_size}
    directory.mkdir(parents=True)
    sources = []
    for filename in ["geo01_reference.cpp", "geo01_reference.py", "geo01_reference.cmake", "psy02_reference.cmake", "psy02_reachability.cmake", "clk01_reference.cmake"]:
        p = ROOT / "tools/porting" / filename
        q = directory / "source" / filename
        q.parent.mkdir(exist_ok=True)
        shutil.copyfile(p, q)
        sources.append(ref(q))
    cfg = process(command, directory, "configure", environment())
    require(cfg["exit_code"] == 0, "Configurefailed; rawlogs retained")
    rows = read(BUILD / "compile_commands.json")
    row = next(x for x in rows if Path(x["file"]).name == "geo01_reference.cpp")
    require(all(x in rows for x in before), "An existing target compilecommand changed")
    for flag in ["-DEP_psych_errors", "-UNDEBUG", "-Werror", "-O0", "-ffp-contract=off"]:
        require(flag in row["command"], "Missing originaldriverflag " + flag)
    for flag in ["-D_GLIBCXX_DEBUG", "-ffast-math", "-DEP_psych_stats"]:
        require(flag not in row["command"], "IncorrectABI/mode " + flag)
    write(directory / "actual-driver-compile-command.json", row)
    built = process([command[0], "--build", str(BUILD), "--target", "geo01_reference", "--parallel", "1", "--verbose"], directory, "build", environment())
    require(built["exit_code"] == 0, "Driverbuildfailed; rawlogs retained")
    after = read(BUILD / "compile_commands.json")
    require(all(x in after for x in before) and all(x in after for x in frozen), "Originalcompilecommands changed")
    for binding in old_refs + [core["artifacts"][k] for k in ["core_library", "api_library"]]:
        require(sha(ROOT / binding["path"]) == binding["sha256"], "An original/priorcard binary changed")
    require(sha(objexx) == objexx_ref["sha256"] and objexx.stat().st_size == objexx_ref["bytes"], "Original container library changed")
    binary = BUILD / "Products/geo01_reference.exe"
    archived = directory / binary.name
    shutil.copyfile(binary, archived)
    for filename in ["CMakeCache.txt", "compile_commands.json"]:
        shutil.copyfile(BUILD / filename, directory / filename)
    receipt = {"schema": "geo01-native-driver-build.v1", "checks_passed": True, "energyplus_commit": PIN,
        "core_build": ref(CORE_RECEIPT), "source_contract": ref(CONTRACTS / "GEO-01-source.json"), "binary": ref(archived),
        "linked_build_binary": ref(binary), "executed_source_archives": sources,
        "configure": ref(directory / "configure-command.json"), "compile_link": ref(directory / "build-command.json"),
        "actual_compile_command": ref(directory / "actual-driver-compile-command.json"), "cache": ref(directory / "CMakeCache.txt"),
        "compile_commands": ref(directory / "compile_commands.json"), "frozen_original_build_commands": ref(frozen_path),
        "frozen_original_build_command_count": len(frozen), "frozen_original_build_commands_unchanged": True,
        "existing_target_compile_commands_unchanged": True, "previous_card_products": old_refs,
        "linked_container_library": objexx_ref, "container_library_unchanged": True,
        "original_core_and_API_bytes_unchanged": True, "same_compiler_owned_state": True,
        "original_directory_and_target_definitions_inherited": True, "assertions_enabled": True, "fp_contract": "off",
        "scientific_source_patches": False, "gates_updated": False}
    write(directory / "native-driver-build.json", receipt)
    return receipt


def run_original(directory, driver_receipt, requested_ids):
    check()
    require(not directory.exists(), "Use fresh original execution root")
    driver = read(driver_receipt)
    require(driver["checks_passed"] is True and driver["energyplus_commit"] == PIN, "Unverified native driver")
    binary = ROOT / driver["binary"]["path"]
    require(ref(binary) == driver["binary"], "Archived originalbinary changed")
    rows = read(CONTRACTS / "GEO-01-cases.json")["cases"]
    if requested_ids:
        rows = [x for x in rows if x["id"] in requested_ids]
        require(len(rows) == len(requested_ids), "Unknown/duplicate selectedcase")
    directory.mkdir(parents=True)
    source_archive = directory / "source"
    source_archive.mkdir()
    shutil.copyfile(Path(__file__), source_archive / "geo01_reference.py")
    bindings = [ref(CONTRACTS / f"GEO-01-{x}.json") for x in ["source", "cases", "tolerances"]]
    results = []
    idd = ROOT / ".runtime/ep261-gcc13-o0/Products/Energy+.idd"
    for row in rows:
        output = directory / row["id"]
        output.mkdir()
        command = [str(binary), str(ROOT), str(CORE_RECEIPT), row["id"], str(ROOT / row["input"]["path"]), str(ROOT / row["weather"]["path"]), str(idd), str(output / "native")]
        execution = process(command, output, "original", environment())
        require(execution["exit_code"] == 0, "Originalrun failed " + row["id"] + "; raw retained")
        result = output / "native/geometry-observation.json"
        data = read(result)
        require(data["energyplus_exit_code"] == 0 and data["callback_error"] == "", "Originalobservation failed")
        receipt = {"schema": "geo01-original-execution.v1", "case_id": row["id"], "kind": row["kind"],
            "execution": ref(output / "original-command.json"), "binary": driver["binary"], "native_core_build": ref(CORE_RECEIPT),
            "native_driver_build": ref(driver_receipt), "executed_launcher": ref(source_archive / "geo01_reference.py"),
            "contracts": bindings, "input": row["input"], "weather": row["weather"], "idd": ref(idd), "results": ref(result),
            "reference_method": "genuine-full-original-input-parser-and-retained-state", "physical_input_scope_expanded": False,
            "rust_compared": False, "gates_updated": False}
        write(output / "receipt.json", receipt)
        results.append({"case_id": row["id"], "receipt": ref(output / "receipt.json"), "results": ref(result)})
    matrix = {"schema": "geo01-original-matrix.v1", "cases": results, "case_count": len(results), "rust_compared": False, "gates_updated": False}
    write(directory / "matrix.json", matrix)
    return {"matrix": ref(directory / "matrix.json"), "case_count": len(results)}


def validate_ref(binding):
    require(isinstance(binding, dict) and type(binding.get("path")) is str and type(binding.get("sha256")) is str, "Malformed artifact binding")
    path = ROOT / binding["path"]
    require(sha(path) == binding["sha256"], "Artifact hash changed: " + binding["path"])
    return path


def review_original(matrix_path, directory):
    """Check genuine parsed inputs/inactive branches/retention, never transform XYZ."""
    check()
    require(not directory.exists(), "Use fresh original review directory")
    matrix = read(matrix_path)
    frozen = read(CONTRACTS / "GEO-01-cases.json")
    require(matrix["case_count"] == len(matrix["cases"]) == frozen["case_count"], "Original matrix incomplete")
    by_id = {x["id"]: x for x in frozen["cases"]}
    require({x["case_id"] for x in matrix["cases"]} == set(by_id), "Original input identities incomplete")
    directory.mkdir(parents=True)
    archived = directory / "geo01_reference.py"
    shutil.copyfile(Path(__file__), archived)
    summaries = []
    total_physical = 0
    corner_ids = {"upperleftcorner": 1, "lowerleftcorner": 2, "lowerrightcorner": 3, "upperrightcorner": 4}
    warning_patterns = [r"coincident vert", r"degenerate surface", r"(?:roof/ceiling|floor) is (?:upside down|not oriented correctly)",
                        r"automatic fix is attempted", r"geometrytransform", r"non-planar surface"]
    for row in matrix["cases"]:
        case = by_id[row["case_id"]]
        receipt_path = validate_ref(row["receipt"])
        receipt = read(receipt_path)
        results_path = validate_ref(row["results"])
        require(receipt["results"] == row["results"] and receipt["input"] == case["input"] and receipt["weather"] == case["weather"], "Execution/input binding differs")
        for key in ["execution", "binary", "native_core_build", "native_driver_build", "executed_launcher", "input", "weather", "idd"]:
            validate_ref(receipt[key])
        for binding in receipt["contracts"]:
            validate_ref(binding)
        d = read(results_path)
        require(d["schema"] == "geo01-original-geometry.v1" and d["case_id"] == case["id"], "Original output schema/identity differs")
        require(d["energyplus_exit_code"] == 0 and d["callback_error"] == "" and d["geometry_probe_calls_added"] == 0 and d["state_reset_requested"] is False, "Original observer failed or added a probe")
        require(d["first_final_vertex_identity_exact"] is True, "Original final vertices changed")
        require(d["constructor"]["native_only_state"]["max_vertices_per_surface"] == 4, "Original constructor counter differs")
        for phase in ["first_initialized", "first_physical", "final_weather", "final"]:
            snapshot = d[phase]
            require(isinstance(snapshot, dict), "Required initialized/retained phase missing")
            require(snapshot["surface_count"] == 6 and snapshot["zone_count"] == 1, "Actual bounded cardinality differs")
            state = snapshot["native_only_state"]
            require(state["max_vertices_per_surface"] == 4 and state["total_coincident_vertices"] == 0 and state["total_degenerate_surfaces"] == 0, "Excluded cleanup/counter growth active")
            require(state["aspect_transform"] is False and state["no_transform"] is True and state["first_time"] is False and state["input_once_flag"] is True, "Excluded aspect/input branch active")
            require(state["zone_cos_array_allocated"] is False and state["zone_sin_array_allocated"] is False and state["surface_tmp_allocated"] is False, "Original temporary state lifetime differs")
            flags = snapshot["settings"]
            rules = case["geometry_input"]["rules"]
            require(flags["corner"] == corner_ids[rules["starting_vertex_position"].casefold()], "Original parsed corner differs")
            require(flags["counterclockwise"] is (rules["vertex_entry_direction"].casefold() == "counterclockwise"), "Original parsed winding differs")
            require(flags["world_coordinate_system"] is (rules["coordinate_system"].casefold() == "world"), "Original parsed coordinate rule differs")
            require(flags["appendix_g_rotation_deg"] == 0, "Excluded AppendixG rotation active")
            surfaces = {s["name"].upper(): s for s in snapshot["surfaces"]}
            require(set(surfaces) == {s["canonical_name"] for s in case["geometry_input"]["surfaces"]}, "Original surface names differ")
            for s in case["geometry_input"]["surfaces"]:
                actual = surfaces[s["canonical_name"]]
                require(actual["sides"] == s["declared_number_of_vertices"] == len(actual["world_vertices_m"]) == 4, "Excluded cardinality mutation occurred")
                require(actual["zone_name"].upper() == s["zone_name"].upper() and actual["zone_id"] > 0, "Original binding differs")
                require(actual["native_consumer"]["is_degenerate"] is False, "Degenerate surface observed")
        parsed = d["parsed_input"]["objects"]
        require("GeometryTransform" not in parsed and "Compliance:Building" not in parsed, "Excluded transform input observed")
        actual_surfaces = {k.upper(): v for k, v in parsed["BuildingSurface:Detailed"].items()}
        for s in case["geometry_input"]["surfaces"]:
            p = actual_surfaces[s["canonical_name"]]
            require(p["number_of_vertices"] == 4 and len(p["vertices"]) == 4, "Original parsed input count differs")
            parsed_bits = [[bits(v[f"vertex_{axis}_coordinate"]) for axis in "xyz"] for v in p["vertices"]]
            require(parsed_bits == s["input_vertex_bits"], "Original parsed input bits/signs differ from frozen lexical input")
        err_path = results_path.parent / "eplusout.err"
        err = err_path.read_text(encoding="utf-8")
        warnings = [line for line in err.splitlines() if any(re.search(pattern, line, re.I) for pattern in warning_patterns)]
        require(not warnings, "Excluded geometry warning/autofix observed: " + case["id"])
        expected_callbacks = {"24H": 96, "72H": 288, "ANNUAL": 35040}[case["duration"]]
        require(d["physical_zone_callback_count"] == expected_callbacks, "Actual original environment incomplete")
        total_physical += d["physical_zone_callback_count"]
        summaries.append({"case_id": case["id"], "kind": case["kind"], "input": case["input"], "weather": case["weather"],
            "receipt": row["receipt"], "results": row["results"], "geometry_error_log": ref(err_path),
            "parsed_input_bits_checked": True, "physical_zone_callback_count": d["physical_zone_callback_count"],
            "first_final_vertex_identity_exact": True, "inactive_geometry_guards_passed": True,
            "native_counter_before": 4, "native_counter_after": 4, "native_counter_paired_with_Rust": False,
            "source_only_consumer_fields_retained": True})
    report = {"schema": "geo01-original-preparation-review.v1", "status": "pass", "energyplus_commit": PIN,
        "original_matrix": ref(matrix_path), "executed_checker": ref(archived), "cases": summaries,
        "case_count": len(summaries), "surface_count": len(summaries)*6, "coordinate_count": len(summaries)*6*4*3,
        "physical_zone_callbacks": total_physical, "input_bits_checked": True, "inactive_geometry_guards_checked": True,
        "first_final_vertex_identity_checked": True, "rust_compared": False, "gates_updated": False,
        "limitations": ["diagnostic original runs do not expand CON production admission", "native counter has no corresponding Rust lifecycle field", "GEO02/GEO03 algorithms and wholephysics unclaimed"]}
    write(directory / "preparation-review.json", report)
    return {"report": ref(directory / "preparation-review.json"), "case_count": len(summaries), "physical_zone_callbacks": total_physical}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--prepare", action="store_true")
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--build", action="store_true")
    mode.add_argument("--run-original", action="store_true")
    mode.add_argument("--review-original", action="store_true")
    parser.add_argument("--output-dir", type=Path)
    parser.add_argument("--driver-build", type=Path)
    parser.add_argument("--original-matrix", type=Path)
    parser.add_argument("--case", action="append", default=[])
    args = parser.parse_args()
    if args.prepare:
        result = prepare()
    elif args.check:
        result = check()
    else:
        require(args.output_dir is not None, "--output-dir required")
        if args.build:
            result = build_driver(args.output_dir.resolve())
        elif args.review_original:
            require(args.original_matrix is not None, "--original-matrix required")
            result = review_original(args.original_matrix.resolve(), args.output_dir.resolve())
        else:
            require(args.driver_build is not None, "--driver-build required")
            result = run_original(args.output_dir.resolve(), args.driver_build.resolve(), args.case)
    print(json.dumps(result, ensure_ascii=False, allow_nan=False))


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, KeyError, StopIteration) as error:
        print(str(error), file=sys.stderr)
        raise SystemExit(2)
