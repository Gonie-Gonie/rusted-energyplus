#!/usr/bin/env python3
"""Prepare input-only GEO-02 drafts and verify pinned source/input bytes.

Preparation outputs contain no expected geometry or searched reference answers.
Native build/execution modes dispatch to a separate receipt-preserving module.
"""
from __future__ import annotations

import argparse
import copy
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import shutil
import struct
import sys

sys.dont_write_bytecode = True
from geo01_reference import input_projection, objects

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / ".reference/energyplus-src/26.1.0"
CONTRACTS = ROOT / "energyplus_porting_plan/contracts"
RAW = ROOT / ".runtime/porting/GEO-02"
PIN = "6f2e40d10250a105b49966baa24d843711e61048"
SELECTED = ["A-24H", "A-72H", "B-NOLIMIT-24H", "B-FLOW-24H", "B-CAPACITY-24H", "B-BOTH-24H", "B-BOTH-72H"]
RANGES = [
    ("src/EnergyPlus/SurfaceGeometry.cc", "SetupZoneGeometry", 251, 355, "initialization-order", []),
    ("src/EnergyPlus/SurfaceGeometry.cc", "CosBldgRelNorth", 273, 289, "verbatim-test-only-initialization-fragment", ["std::cos", "std::sin"]),
    ("src/EnergyPlus/SurfaceGeometry.cc", "CalcSurfaceCentroid", 2589, 2593, "centroid-caller-before-ProcessSurfaceVertices", ["CalcSurfaceCentroid"]),
    ("src/EnergyPlus/SurfaceGeometry.cc", "checkPopCoincidentVertex", 9115, 9167, "ordinary-negative-input-coincident-pop-prerequisite", ["Vectors::VecLength"]),
    ("src/EnergyPlus/SurfaceGeometry.cc", "GetVertices", 9169, 9485, "selected-primary-opaque-quad-producer-with-inactive-corrections", ["Vectors::CreateNewellSurfaceNormalVector", "Vectors::CreateNewellAreaVector", "Vectors::VecLength", "Vectors::DetermineAzimuthAndTilt", "Convect::GetSurfConvOrientation", "TransformVertsByAspect"]),
    ("src/EnergyPlus/SurfaceGeometry.cc", "ProcessSurfaceVertices", 12846, 12985, "selected-base-surface-preamble-and-shape", ["Vectors::CalcCoPlanarNess", "Vectors::VecLength", "isRectangle"]),
    ("src/EnergyPlus/SurfaceGeometry.cc", "CalcCoordinateTransformation", 13324, 13369, "selected-base-surface-transfer-and-processed-flag", ["CalcCoordinateTransformation"]),
    ("src/EnergyPlus/SurfaceGeometry.cc", "CalcCoordinateTransformation", 13398, 13447, "selected-base-surface-local-origin", ["magnitude_squared", "dot"]),
    ("src/EnergyPlus/SurfaceGeometry.cc", "CalcSurfaceCentroid", 14505, 14654, "selected-quad-centroid-and-source-only-degenerate-retention", ["cen", "Vectors::AreaPolygon"]),
    ("src/EnergyPlus/SurfaceGeometry.cc", "isRectangle", 15364, 15394, "selected-quad-shape-predicate", ["Vectors::VecLength", "Vectors::VecNormalize", "dot"]),
    ("src/EnergyPlus/SurfaceGeometry.cc", "TransformVertsByAspect", 14391, 14474, "absent-aspect-prerequisite", []),
    ("src/EnergyPlus/SurfaceGeometry.hh", "ProcessSurfaceVertices", 373, 378, "declaration", []),
    ("src/EnergyPlus/SurfaceGeometry.hh", "CalcSurfaceCentroid", 412, 432, "declarations", []),
    ("src/EnergyPlus/SurfaceGeometry.hh", "ProcessSurfaceVerticesOneTimeFlag", 442, 495, "true-defaults-and-scratch-owner", []),
    ("src/EnergyPlus/SurfaceGeometry.hh", "clear_state", 509, 535, "clear-state-flags-and-deallocation-no-Triangle-reset", []),
    ("src/EnergyPlus/DataSurfaces.hh", "Area", 712, 748, "owned-output-fields", []),
    ("src/EnergyPlus/DataSurfaces.hh", "SurfaceData", 830, 841, "surface-constructor-defaults", []),
    ("src/EnergyPlus/DataGlobalConstants.hh", "DegToRad", 575, 591, "numeric-constants", []),
    ("src/EnergyPlus/Vectors.cc", "AreaPolygon", 106, 136, "ordered-cross-sum-area", ["cross", "dot", "VecNormalize"]),
    ("src/EnergyPlus/Vectors.cc", "VecSquaredLength", 138, 150, "ordered-unfused-length-square", []),
    ("src/EnergyPlus/Vectors.cc", "VecLength", 152, 164, "length", ["VecSquaredLength", "std::sqrt"]),
    ("src/EnergyPlus/Vectors.cc", "VecNormalize", 184, 208, "exact-zero-normalization", ["VecLength"]),
    ("src/EnergyPlus/Vectors.cc", "DetermineAzimuthAndTilt", 224, 285, "normal-and-horizontal-branch-angles", ["VecNormalize", "cross", "dot", "mod", "std::atan2", "std::acos"]),
    ("src/EnergyPlus/Vectors.cc", "PlaneEquation", 287, 327, "coplanarity-prerequisite-and-zero-normal-diagnostic", ["VecLength", "dot"]),
    ("src/EnergyPlus/Vectors.cc", "Pt2Plane", 329, 345, "coplanarity-distance", []),
    ("src/EnergyPlus/Vectors.cc", "CreateNewellAreaVector", 347, 370, "ordered-fan-area-vector", ["cross"]),
    ("src/EnergyPlus/Vectors.cc", "CreateNewellSurfaceNormalVector", 372, 417, "cyclic-normal-separate-from-area-vector", ["VecNormalize"]),
    ("src/EnergyPlus/Vectors.cc", "CalcCoPlanarNess", 445, 486, "coplanarity-predicate", ["PlaneEquation", "Pt2Plane"]),
    ("src/EnergyPlus/Vectors.hh", "AreaPolygon", 76, 115, "declarations", []),
    ("third_party/ObjexxFCL/src/ObjexxFCL/Vector3.hh", "operator +=", 238, 247, "ordered-component-add", []),
    ("third_party/ObjexxFCL/src/ObjexxFCL/Vector3.hh", "operator *=", 419, 440, "scalar-multiply-and-reciprocal-division", []),
    ("third_party/ObjexxFCL/src/ObjexxFCL/Vector3.hh", "magnitude_squared", 617, 622, "ordered-component-magnitude-square", []),
    ("third_party/ObjexxFCL/src/ObjexxFCL/Vector3.hh", "magnitude_squared", 1150, 1157, "magnitude-square-forwarder", ["Vector3::magnitude_squared"]),
    ("third_party/ObjexxFCL/src/ObjexxFCL/Vector3.hh", "operator -", 1435, 1442, "ordered-component-vector-subtraction", []),
    ("third_party/ObjexxFCL/src/ObjexxFCL/Vector3.hh", "operator *", 1471, 1487, "scalar-vector-multiplication", []),
    ("third_party/ObjexxFCL/src/ObjexxFCL/Vector3.hh", "cen", 1683, 1695, "binary64-sum-then-extended-product-cast", []),
    ("third_party/ObjexxFCL/src/ObjexxFCL/Vector3.hh", "dot", 1728, 1748, "ordered-dot-and-cross", []),
    ("third_party/ObjexxFCL/src/ObjexxFCL/Fmath.hh", "mod", 725, 732, "double-mod-is-fmod", []),
    ("src/EnergyPlus/DataSurfaces.cc", "Centroid", 594, 659, "SRC05-height-operand-context-only", []),
    ("src/EnergyPlus/SolarShading.cc", "OutNormVec", 2708, 2718, "current-SOLCOS-incidence-operand-context-only", []),
    ("src/EnergyPlus/SolarShading.cc", "SurfSunCosTheta", 5095, 5111, "shadowing-SUNCOS-incidence-operand-context-only", []),
    ("src/EnergyPlus/HeatBalanceManager.cc", "SetupZoneGeometry", 1815, 1823, "input-initialization-caller", []),
    ("src/EnergyPlus/HeatBalanceManager.cc", "BeginZoneTimestepBeforeInitHeatBalance", 147, 198, "public-callback-after-geometry", []),
    ("src/EnergyPlus/ConvectionCoefficients.cc", "GetSurfConvOrientation", 6596, 6614, "source-orientation-field-context-unpaired", []),
    ("src/EnergyPlus/ConvectionCoefficients.hh", "GetSurfConvOrientation", 745, 745, "orientation-declaration", []),
    ("src/EnergyPlus/CMakeLists.txt", "EP_psych_errors", 25, 45, "same-directory-and-target-definitions", []),
]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha(p):
    h = hashlib.sha256()
    with Path(p).open("rb") as f:
        for b in iter(lambda: f.read(1048576), b""):
            h.update(b)
    return h.hexdigest()


def ref(p):
    p = Path(p).resolve()
    return {"path": p.relative_to(ROOT).as_posix(), "sha256": sha(p)}


def read(p):
    return json.loads(Path(p).read_text(encoding="utf-8"))


def write(p, value):
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(value, ensure_ascii=False, indent=2, allow_nan=False) + "\n", encoding="utf-8", newline="\n")


def bits(v):
    return struct.pack(">d", float(v)).hex()


def quad_cases():
    roof = [[0, 0, 2], [3, 0, 2], [3, 2, 2], [0, 2, 2]]
    wall = [[0, 0, 2], [0, 0, 0], [3, 0, 0], [3, 0, 2]]
    trap = [[0, 0, 2], [0, 0, 0], [4, 0, 0], [2, 0, 2]]
    def reverse(v):
        return [v[0], *v[1:][::-1]]
    values = [
        ("ROOF-HORIZONTAL", "Roof", roof, "horizontal positive normal"),
        ("FLOOR-HORIZONTAL", "Floor", reverse([[x, y, 0] for x, y, _ in roof]), "horizontal negative normal"),
        ("WALL-VERTICAL", "Wall", wall, "vertical negative-Y normal"),
        ("WALL-REVERSED", "Wall", reverse(wall), "reversed-wall normal, no roof/floor repair"),
        ("ROOF-SLOPED", "Roof", [[x, y, z + .2*x + .1*y] for x, y, z in roof], "nonzero oblique normal"),
        ("WALL-TRAPEZOID", "Wall", trap, "unequal triangle areas, convex centroid weighting"),
        ("WALL-TRANSLATED-TRAPEZOID", "Wall", [[x + 3.125, y - 4.75, z + 1.5] for x, y, z in trap], "nonzero centroid XYZ"),
        ("WALL-NEGATIVE-Z", "Wall", [[x, y, z - 4] for x, y, z in trap], "source negative-Z centroid warning diagnostic"),
        ("QUAD-CONCAVE-ALTERNATE", "Roof", [[0, 0, 2], [.5, .5, 2], [3, 0, 2], [0, 3, 2]], "source >1.05 alternate split candidate, diagnostic only"),
    ]
    for label, d, rev in [("AZ180-BELOW", 1e-8, False), ("AZ180-ABOVE", 1e-7, False),
                          ("AZ360-BELOW", 1e-5, True), ("AZ360-ABOVE", 1e-4, True)]:
        v = [[0, 0, 2], [0, 0, 0], [3, d, 0], [3, d, 2]]
        values.append((label, "Wall", reverse(v) if rev else v, "source angle cleanup strict-threshold candidate"))
    for label, k in [("HORIZONTAL-EPS-BELOW", 1e-8), ("HORIZONTAL-EPS-ABOVE", 2e-8)]:
        values.append((label, "Roof", [[x, y, z+k*x] for x, y, z in roof], "source near-horizontal epsilon branch candidate"))
    result = [{"case_id": name, "kind": "valid_quad", "surface_class": cls, "vertices_m": v,
               "initial_centroid_m": [0, 0, 0], "branch_intent": intent}
              for name, cls, v, intent in values]
    for name, v in [("DEGENERATE-COLLINEAR", [[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0]]),
                    ("DEGENERATE-COINCIDENT", [[0, 0, 0]]*4)]:
        result.append({"case_id": name, "kind": "source_only_degenerate", "surface_class": "Wall", "vertices_m": v,
                       "initial_centroid_m": [17, -23, 31], "branch_intent": "PlaneEquation exact-zero error and CalcSurfaceCentroid zeroGross warning/retention; not full parser rejection"})
    require(len(result) == 17, "Quad draft count differs")
    for row in result:
        row["input_vertex_bits"] = [[bits(x) for x in v] for v in row["vertices_m"]]
        if row["kind"] == "valid_quad":
            require(all(x != 0 or bits(x) == "0000000000000000" for v in row["vertices_m"] for x in v), "Valid quad geometric zeros must be positive")
            row["required_kernel_input_identity"] = "original post-GetVertices ordered bits and Rust own stored input bits must equal input_vertex_bits before numerical comparison"
    return result


def cen_calls():
    edge = [0, 1 << 63, 1, (1 << 63) | 1, 1 << 52, (1 << 63) | (1 << 52)]
    edge += [int(bits(v), 16) for v in [1, 2, 3, 4.572, 15.24]]
    state = 0x47454F303243454E
    mask = (1 << 64) - 1
    generated = []
    for _ in range(512):
        state ^= (state << 13) & mask
        state ^= state >> 7
        state ^= (state << 17) & mask
        state &= mask
        exponent = (state >> 52) % 401 - 200
        raw = (state & (1 << 63)) | ((1023 + exponent) << 52) | (state & ((1 << 52) - 1))
        generated.append(raw)
    rows = []
    for i, raw in enumerate(edge + generated):
        token = f"{raw:016x}"
        value = struct.unpack(">d", bytes.fromhex(token))[0]
        rows.append({"case_id": f"CEN-{i:03d}", "x_operands": [value, 0.0, 0.0],
                     "x_operand_bits": [token, bits(0), bits(0)], "kind": "raw-significand-input"})
    for name, operands in [("ALL-NEGATIVE-ZERO", [-0.0, -0.0, -0.0]),
                           ("CANCELLATION-LEFT-LOSS", [1e16, 1.0, -1e16]),
                           ("CANCELLATION-LEFT-EXACT", [1e16, -1e16, 1.0])]:
        rows.append({"case_id": "CEN-" + name, "x_operands": operands,
                     "x_operand_bits": [bits(x) for x in operands], "kind": "ordered-three-operand-boundary"})
    require(len(rows) == 526, "Cen input count differs")
    return rows


def source_contract():
    files, ranges = {}, []
    for rel, symbol, start, end, role, helpers in RANGES:
        p = SOURCE / rel
        content = "".join(p.read_text(encoding="utf-8").splitlines(keepends=True)[start-1:end])
        require(symbol in content, f"Literal source symbol absent: {rel}:{start} {symbol}")
        files[rel] = {"path": rel, "sha256": sha(p)}
        ranges.append({"path": rel, "symbol": symbol, "start": start, "end": end, "role": role,
                       "helpers": helpers, "range_sha256": hashlib.sha256(content.encode()).hexdigest()})
    for rel in ["src/EnergyPlus/Data/EnergyPlusData.hh", "src/EnergyPlus/Data/EnergyPlusData.cc"]:
        files[rel] = {"path": rel, "sha256": sha(SOURCE / rel)}
    return {"schema": "geo02-source-contract.v1", "card": "GEO-02", "status": "draft-before-independent-review",
        "energyplus_commit": PIN, "source_files": list(files.values()), "selected_ranges": ranges,
        "primary_producers": ["SurfaceGeometry::GetVertices selected opaque planar quad geometry", "SurfaceGeometry::CalcSurfaceCentroid selected quad centroid"],
        "card_name_correction": "ProcessSurfaceVertices consumes existing geometry and produces shape/shading coordinates; it is not the primary area/normal/angle producer",
        "call_order": ["SetupZoneGeometry source trig initialization", "GetSurfaceData -> GetVertices", "GetSurfaceData -> CalcSurfaceCentroid", "SetupZoneGeometry deallocates zone trig arrays", "SetupZoneGeometry -> ProcessSurfaceVertices"],
        "geometry_fields": ["NewellAreaVector", "NewellSurfaceNormalVector", "GrossArea", "Area", "NetAreaShadowCalc", "Azimuth", "Tilt", "lcsx/lcsy/lcsz", "SinAzim/CosAzim/SinTilt/CosTilt", "OutNormVec", "Centroid"],
        "normal_policy": "Newell cyclic normalized vector is separate from independently component-snapped OutNormVec; source snap order +1, -1, zero at strict1e-6 without renormalization",
        "centroid_policy": "Quad123/134 area fractions against own GrossArea; >1.05 switches124/234 with own recomputed total; <=0 warns and continues retaining prior centroid",
        "cen_precision_policy": "Original cen3 sums each component in T=f64; third is long double initialized from DOUBLE 1.0/3.0; one extended multiplication then T cast. Unchanged whole-header outputs are authoritative; no copied cen numerical implementation",
        "valid_helper_input_boundary": "raw World/UpperLeft/CounterClockWise/AppG0 quads with finite positive geometric zero only; actual original post-GetVertices ordered vertex bits MUST equal raw request bits before any Rust baseline or numerical comparison. Rust independently stores its own request bits. Any failure requires an independently executed compiler route at unchanged inputs/tolerances, never native-output injection",
        "valid_helper_inactive_predicates": ["Sides remains four", "zero coincident/degenerate cleanup counters", "absent/inactive aspect transform", "no roof/floor automatic reversal", "same retained vertex order/bits", "ProcessSurfaceVertices second call early-return retains first-call owned fields"],
        "state_contract": {"constructor": "actual fresh EnergyPlusData and newly allocated real SurfaceData defaults; quantities/normals/centroid zero and VerticesProcessedfalse; ProcessOneTime true, X/Y/Zpsv unallocated; Triangle arrays declaredsize3",
            "clear_state": "ProcessOneTime true and X/Y/Zpsv deallocated; Triangle1/2 are not reset by selected clear_state body; no invented default or reset element values",
            "unwritten_buffer_policy": "nondebug Vector3 default construction can leave components uninitialized: observe only allocation/dimensions for Triangle or Vertex buffers before actual source assignments; never read unwritten element values or assume zero. Written buffers may be copied after actual producer calls",
            "native_only_unpaired": ["true global maxcounter", "scratch arrays and Triangle buffers", "source shape/shade arrays/local shifts", "oneTime/processed lifecycle without matching Rust global"],
            "retention": "read-only first initialized / first physical / final weather / before stateDelete field snapshots; no observer recomputation"},
        "references": {"native_initializer": "same-GCC original dynamic API parses unchanged IDFs; DLL owns state; const reads only",
            "valid_helper": "same-GCC genuine core state, declared input allocations/GGR/class/vertices; byte-exact source Setup273-289 fragment, original GetVertices->ownedSurface->CalcSurfaceCentroid->ProcessSurfaceVertices twice; not whole Setup or original parser admission",
            "unsafe_helper": "genuine PlaneEquation/normalizer/area and CalcSurfaceCentroid only; explicitly bypass GetVertices/Process, source-only diagnostics",
            "invalid_native_input": "two input-only A24 wall-vertex patches execute ordinary original API in fresh lifetimes; preserve actual nonzero exit, geometry error logs and only available constructor/phase snapshots. No successful-field or physical-hook fabrication, no error-message parity claim; separate from unsafe prepared helper warning/centroid retention",
            "native_core_build": ref(ROOT / ".runtime/porting/reference-energyplus-26.1.0/native-core-build.json"),
            "ABI": "actual inherited project_options/project_fp_options/project_warnings and original directory+target definitions, GCC13/O0 assertions/Werror/no contraction; same compiler-owned state",
            "precision_observation": "read-only sizeof types, FLT/DBL/LDBL_MANT_DIG, FE rounding, GNU x64 fstcw x87 control word including PC24/53/64/reserved and RC, and stmxcsr rounding mode. Capture before/after unchanged-header cen and initialized/final centroid lifetimes. Never write control state or infer actual x87 precision solely from LDBL_MANT_DIG. Optional third literal diagnostic labeled replica, not original private-local state"},
        "consumer_scope": "pair actual geometry operands used for centroid-height and current/period normal-angle consumers; no SRC04 solar-clock or SRC05 lapse/wind physics closure",
        "excluded": ["CON input expansion", "triangle/5+ production", "fenestration/net subtraction/multipliers", "IntMass/shading/air-boundary/complex window branches", "active aspect/AppendixG transformations", "roof/floor automatic reorientation", "whole physics or GEO03 volume closure"],
        "comparison_run": False, "gates_updated": False}


def prepare():
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
    archive = RAW / "draft-history" / stamp
    archive.mkdir(parents=True)
    shutil.copyfile(Path(__file__), archive / Path(__file__).name)
    for name in ["source", "cases", "tolerances"]:
        p = CONTRACTS / f"GEO-02-{name}.json"
        if p.exists():
            shutil.copyfile(p, archive / p.name)
    cases_dir = ROOT / "energyplus_porting_plan/cases/GEO-02"
    if cases_dir.exists():
        shutil.copytree(cases_dir, archive / "prior-cases")
    scope = read(CONTRACTS / "scope.json")
    con = {c["id"]: c for c in scope["cases"]}
    prior = read(CONTRACTS / "GEO-01-cases.json")
    prior_by_id = {c["id"]: c for c in prior["cases"]}
    prior_ref = ref(CONTRACTS / "GEO-01-cases.json")
    prior_indices = {c["id"]: i for i,c in enumerate(prior["cases"])}
    native = [{**copy.deepcopy(con[k]), "kind": "unchanged-CON-input",
               "geometry_input_source": {"artifact": prior_ref, "json_pointer": f"/cases/{prior_indices[k]}/geometry_input"}} for k in SELECTED]
    base_path = ROOT / con["A-24H"]["input"]["path"]
    base = objects(base_path.read_text(encoding="utf-8"))
    for name, transform, description in [
        ("OBLIQUE-AFFINE", lambda x,y,z: (x+.375*y, y+.25*z, z+.2*x), "input-only orientation-preserving affine box shear"),
        ("NONUNIFORM-TRAPEZOID", lambda x,y,z: (x*(1+.08*z), y*(1+.04*z), z), "input-only frustum; convex nonuniform trapezoidal walls"),
        ("SNAP-BELOW", lambda x,y,z: (x+5e-7*z, y, z), "input-only shear; raw normal component source snap threshold candidate below1e-6"),
        ("SNAP-ABOVE", lambda x,y,z: (x+2e-6*z, y, z), "input-only shear; raw normal component source snap threshold candidate above1e-6"),
    ]:
        ident = "GEO02-" + name
        objs = copy.deepcopy(base)
        for row in objs:
            if row[0].casefold() == "buildingsurface:detailed":
                row[12:] = [repr(v) for i in range(0,12,3) for v in transform(*map(float,row[12+i:15+i]))]
        patches = [{"object_type": a[0], "name": a[1], "before_fields": a[1:], "after_fields": b[1:]}
                   for a,b in zip(base,objs) if a != b]
        target = cases_dir / ident / "input.idf"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text("! GEO-02 input-only diagnostic; no expected geometry.\n" + "\n\n".join(x[0]+",\n  "+",\n  ".join(x[1:])+";" for x in objs)+"\n", encoding="utf-8", newline="\n")
        metadata = {"schema": "geo02-idf-metadata.v1", "case_id": ident, "base_input": ref(base_path),
                    "input": ref(target), "weather": con["A-24H"]["weather"], "patches": patches,
                    "input_generation": description, "only_detailed_vertex_fields_changed": True,
                    "physical_scope_expansion": False, "geometry_input": input_projection(objs)}
        write(target.parent / "metadata.json", metadata)
        native.append({"id": ident, "scope": None, "duration": "24H", "kind": "valid-diagnostic-not-CON",
                       "input": ref(target), "weather": con["A-24H"]["weather"], "metadata": ref(target.parent / "metadata.json"),
                       "geometry_input_source": {"artifact": ref(target.parent / "metadata.json"), "json_pointer": "/geometry_input"}})
    helper = {"schema": "geo02-helper-cases.v1", "cases": quad_cases(), "cen_calls": cen_calls()}
    invalid = []
    for name, points in [("COLLINEAR", [[0,0,0],[1,0,0],[2,0,0],[3,0,0]]),
                         ("COINCIDENT", [[0,0,0]]*4)]:
        ident = "GEO02-INVALID-" + name
        objs = copy.deepcopy(base)
        wall = next(row for row in objs if row[0].casefold()=="buildingsurface:detailed" and row[2].casefold()=="wall")
        before = copy.deepcopy(wall)
        wall[12:] = [repr(float(v)) for point in points for v in point]
        target = cases_dir / ident / "input.idf"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text("! GEO-02 ordinary-input negative geometry; no expected answers.\n" + "\n\n".join(x[0]+",\n  "+",\n  ".join(x[1:])+";" for x in objs)+"\n", encoding="utf-8", newline="\n")
        metadata = {"schema":"geo02-idf-metadata.v1", "case_id":ident, "base_input":ref(base_path),
                    "input":ref(target), "weather":con["A-24H"]["weather"],
                    "patches":[{"object_type":before[0],"name":before[1],"before_fields":before[1:],"after_fields":wall[1:]}],
                    "input_generation":"first opaque Wall vertex fields replaced by declared " + name.casefold() + " points only",
                    "only_detailed_vertex_fields_changed":True,"physical_scope_expansion":False,"geometry_input":input_projection(objs)}
        write(target.parent/"metadata.json",metadata)
        invalid.append({"id":ident,"scope":None,"duration":"24H","kind":"ordinary-input-negative-not-CON",
                        "input":ref(target),"weather":con["A-24H"]["weather"],"metadata":ref(target.parent/"metadata.json"),
                        "geometry_input_source":{"artifact":ref(target.parent/"metadata.json"),"json_pointer":"/geometry_input"}})
    request = cases_dir / "helper-request.json"
    write(request, helper)
    case_contract = {"schema": "geo02-input-contract.v1", "card": "GEO-02", "status": "draft-before-independent-review",
        "unchanged_CON_case_count": 15, "unchanged_CON": copy.deepcopy(scope["cases"]),
        "native_case_count": 11, "native_cases": native,
        "native_invalid_case_count": 2, "native_invalid_cases": invalid,
        "native_invalid_policy": "actual ordinary API nonzero exit and zero physical callbacks; preserve error logs and available states without demanding successful output fields. Later Rust normal entrypoint admission is separate, no source error-string equality",
        "Rust_physical_case_ids": ["A-24H", "A-72H", "B-BOTH-24H"], "Rust_physical_trace_levels": ["full", "summary"],
        "physical_scope_statement": "three fresh representative Rust cases, not fresh per-limit/B72/annual consumption; original native selected7 plus4diagnostics; no new CON admission",
        "static_geometry_family_projection": ref(ROOT / "energyplus_porting_plan/evidence/GEO-02/input-families.json"),
        "helper_request": ref(request), "helper_quad_count": 17, "valid_helper_quads": 15, "source_only_degenerate_quads": 2,
        "cen_precision_input_count": 526, "cen_generation": {"edge_count": 11, "random_count": 512, "special_three_operand_count": 3, "seed_hex": "47454f303243454e", "algorithm": "xorshift64: x^=x<<13; x^=x>>7; x^=x<<17 with64bitmask; sign=bit63; exponent=((x>>52)%401)-200; significand=low52; no output search", "call": "unchanged ObjexxFCL::cen(Vector3(a,0,0),Vector3(b,0,0),Vector3(c,0,0)); actual source ordered binary64 sum precedes extended product; first523 operands are [rawvalue,+0,+0]; special triples preserve all-negative-zero and distinct cancellation orders"},
        "valid_helper_domain": {"rules": "World/UpperLeftCorner/CounterClockWise", "appendix_g_rotation_deg": 0, "zero_input_sign": "positive geometric zero only", "required_original_first_guard": "actual post-GetVertices ordered vertex bits == frozen request input_vertex_bits, no repair/aspect/count change", "required_Rust_guard": "own stored TypedModel vertex bits == frozen request input_vertex_bits", "identity_failure": "stop before comparison; retain failure; no native-output seed; unchanged-input compiler adapter required"},
        "request_policy": "input-only finite vertices/state and raw cen bits; C++ and Rust compute own values; no original output is a request field",
        "historical_reuse": {"GEO01_cases": ref(CONTRACTS / "GEO-01-cases.json"), "GEO01_comparison": ref(ROOT / "energyplus_porting_plan/evidence/GEO-01/comparison-report.json"),
            "allowed": "already observed coordinate/input identities and area/azimuth/tilt only", "not_observed": ["raw/snapped normals", "centroid", "gross/net area", "height/incidence operand consumption"]},
        "comparison_run": False, "gates_updated": False}
    profiles = {}
    for field, unit, tolerance in [("area_m2","m2",1e-10), ("newell_area_vector_m2","m2",1e-10), ("centroid_m","m",1e-10),
                                   ("normals_and_lcs","1",1e-12), ("trig","1",1e-12), ("angles_deg","deg",1e-10)]:
        profiles[field] = {"unit":unit, "absolute_tolerance":tolerance, "relative_tolerance":1e-13,
                           "rule":"abs(actual-reference)<=atol+rtol*abs(reference)", "finite_class":"exact", "both_zero_sign":"exact", "angle_wrapping":"forbidden"}
    tolerance = {"schema":"geo02-tolerances.v1", "card":"GEO-02", "status":"draft-before-independent-review", "frozen_before_numerical_comparison":True,
        "profiles":profiles, "cen_precision":{"output_bits":"exact unchanged-header cen3 observation", "input_bits":"exact", "class_and_zero_sign":"exact", "numeric_tolerance_used":False,
            "floating_environment":"read actual GNU x64 x87 control word/PC/RC and MXCSR rounding before/after calls and initialization; no control writes; select source-compatible product precision from actual PC observation rather than LDBL_MANT_DIG assumption"},
        "exact_metadata":["input bits", "canonical names and own-ID bindings", "surface class and vertex cardinality/order", "source phases", "first/final retained field bits", "inactive geometry corrections", "actual observed flags"],
        "source_only_unpaired":["unsafe native warning/centroid retention and scratch history", "native-only globals/shape/shading buffers", "no fabricated Rust error messages or source counters"],
        "exclusions":["tolerance widening after failures", "whole SRC04/SRC05/physics/GEO03 closure", "old GEO01 observations relabeled as new missing-field evidence"], "gates_updated":False}
    for name, value in [("source",source_contract()),("cases",case_contract),("tolerances",tolerance)]:
        write(CONTRACTS / f"GEO-02-{name}.json",value)
    result = {"schema":"geo02-draft-preparation.v1", "executed_preparer":ref(archive/Path(__file__).name),
              "contracts":{k:ref(CONTRACTS/f"GEO-02-{k}.json") for k in ["source","cases","tolerances"]},
              "native_case_count":11,"native_invalid_case_count":2,"helper_quad_count":17,"cen_precision_input_count":526,"numerical_execution":False,"gates_updated":False}
    write(archive/"preparation.json",result)
    return result


def check():
    source=read(CONTRACTS/"GEO-02-source.json")
    require(source==source_contract(),"Source contract differs from original/current declared draft")
    cases=read(CONTRACTS/"GEO-02-cases.json")
    require(len(cases["native_cases"])==cases["native_case_count"]==11,"Native case count differs")
    require(len(cases["native_invalid_cases"])==cases["native_invalid_case_count"]==2,"Native invalid case count differs")
    for row in cases["native_cases"] + cases["native_invalid_cases"]:
        for k in ["input","weather","metadata"]:
            require(ref(ROOT/row[k]["path"])==row[k],"Native input binding differs")
        binding = row["geometry_input_source"]
        require(ref(ROOT/binding["artifact"]["path"]) == binding["artifact"], "Geometry input source binding differs")
        projection = read(ROOT/binding["artifact"]["path"])
        for key in binding["json_pointer"].strip("/").split("/"):
            projection = projection[int(key)] if isinstance(projection,list) else projection[key]
        require(input_projection(objects((ROOT/row["input"]["path"]).read_text(encoding="utf-8")))==projection,"Native input projection differs")
    require(ref(ROOT/cases["helper_request"]["path"])==cases["helper_request"],"Helper request hash differs")
    h=read(ROOT/cases["helper_request"]["path"])
    require(h=={"schema":"geo02-helper-cases.v1","cases":quad_cases(),"cen_calls":cen_calls()},"Input-only helper generation differs")
    require(len(h["cases"])==17 and len(h["cen_calls"])==526,"Helper/precision counts differ")
    require(ref(ROOT/cases["static_geometry_family_projection"]["path"])==cases["static_geometry_family_projection"],"Family proof binding differs")
    return {"status":"pass","source_files":len(source["source_files"]),"selected_ranges":len(source["selected_ranges"]),"native_case_count":11,"native_invalid_case_count":2,"helper_quad_count":17,"cen_precision_input_count":526,"numerical_execution":False}


def main():
    p=argparse.ArgumentParser(description=__doc__)
    choice=p.add_mutually_exclusive_group(required=True)
    choice.add_argument("--prepare-draft",action="store_true")
    choice.add_argument("--check",action="store_true")
    choice.add_argument("--build",action="store_true")
    choice.add_argument("--run-original",action="store_true")
    choice.add_argument("--run-helpers",action="store_true")
    p.add_argument("--output-dir",type=Path)
    p.add_argument("--driver-build",type=Path)
    p.add_argument("--case",action="append",default=[])
    args=p.parse_args()
    if args.prepare_draft:
        result=prepare()
    elif args.check:
        result=check()
    else:
        require(args.output_dir is not None,"--output-dir required")
        from geo02_reference_native import build_driver,run_original,run_helpers
        if args.build:
            result=build_driver(args.output_dir)
        else:
            require(args.driver_build is not None,"--driver-build required")
            result=run_helpers(args.output_dir,args.driver_build.resolve()) if args.run_helpers else run_original(args.output_dir,args.driver_build.resolve(),args.case)
    print(json.dumps(result,ensure_ascii=False))


if __name__=="__main__":
    try:
        main()
    except (ValueError,KeyError,OSError) as e:
        print(str(e),file=sys.stderr)
        raise SystemExit(2)
