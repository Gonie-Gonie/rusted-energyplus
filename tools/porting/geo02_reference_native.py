#!/usr/bin/env python3
"""Build/execute genuine GEO-02 references, preserving every raw receipt.

This module owns native tooling only. It never starts Rust or changes gates.
"""
from __future__ import annotations
from datetime import datetime, timezone
import os
from pathlib import Path
import re
import shutil
import subprocess
import time
import sys

sys.dont_write_bytecode = True
from geo02_reference import ROOT, SOURCE, RAW, CONTRACTS, PIN, check, read, write, ref, sha, require

BUILD = ROOT / ".runtime/ep261-gcc13-o0"
CORE = ROOT / ".runtime/porting/reference-energyplus-26.1.0/native-core-build.json"
NAMES = ["geo02_reference.cpp", "geo02_reference_helper.cpp", "geo02_reference_fields.hh", "geo02_reference.cmake",
         "geo02_reference.py", "geo02_reference_native.py", "geo01_reference.py", "geo01_reference.cmake",
         "psy02_reference.cmake", "psy02_reachability.cmake", "clk01_reference.cmake"]


def environment():
    env = os.environ.copy()
    env.update(PYTHONUTF8="1", PYTHONIOENCODING="utf-8")
    env["PATH"] = str(ROOT / ".runtime/reference-tools/winlibs-gcc13.2.0-ucrt-r3/mingw64/bin") + os.pathsep + env.get("PATH", "")
    return env


def process(command, directory, name):
    started = datetime.now(timezone.utc).isoformat()
    begin = time.perf_counter()
    with (directory/f"{name}-stdout.log").open("wb") as out, (directory/f"{name}-stderr.log").open("wb") as err:
        result = subprocess.run(command, cwd=ROOT, env=environment(), stdout=out, stderr=err, check=False)
    receipt = {"command": command, "cwd": str(ROOT), "exit_code": result.returncode,
               "started_utc": started, "finished_utc": datetime.now(timezone.utc).isoformat(),
               "elapsed_seconds": round(time.perf_counter()-begin, 3),
               "environment_overrides": {"PYTHONUTF8":"1", "PYTHONIOENCODING":"utf-8"},
               "runtime_path_prefix": str(ROOT/".runtime/reference-tools/winlibs-gcc13.2.0-ucrt-r3/mingw64/bin"),
               "stdout": ref(directory/f"{name}-stdout.log"), "stderr": ref(directory/f"{name}-stderr.log")}
    write(directory/f"{name}-command.json", receipt)
    return receipt


def fresh(directory):
    directory = directory.resolve()
    require(directory.is_relative_to(RAW.resolve()) and directory != RAW.resolve() and not directory.exists(),
            "Use a fresh output directory inside .runtime/porting/GEO-02")
    directory.mkdir(parents=True)
    return directory


def archive_sources(directory):
    target = directory/"source"
    target.mkdir()
    bindings = []
    for filename in NAMES:
        original = ROOT/"tools/porting"/filename
        archived = target/filename
        shutil.copyfile(original, archived)
        bindings.append({"historical_path":original.relative_to(ROOT).as_posix(), "archive":ref(archived)})
    return bindings


def contracts():
    return {name:ref(CONTRACTS/f"GEO-02-{name}.json") for name in ["source","cases","tolerances"]}


def binding_matches(binding):
    return sha(ROOT/binding["path"]) == binding["sha256"]


def original_source_guard(core):
    preservation=core["source_preservation"]
    require(binding_matches(preservation),"Historical source-preservation proof changed")
    proof=read(ROOT/preservation["path"])
    snapshot=proof["archive_snapshot"]
    require(binding_matches(snapshot),"Original archive snapshot changed")
    missing=set(proof["missing_original_archive_non_scientific_git_metadata"])
    rows=read(ROOT/snapshot["path"])["files"]
    for row in rows:
        path=SOURCE/row["path"]
        if not path.exists():
            require(row["path"] in missing,"Original source disappeared: "+row["path"])
        else:
            require(path.stat().st_size==row["bytes"] and sha(path)==row["sha256"],"Original source changed: "+row["path"])
    return {"original_preservation":preservation,"archive_snapshot":snapshot,
            "archive_rows_checked":len(rows),"unchanged_present_rows":len(rows)-len(missing),
            "known_historical_non_scientific_absences":sorted(missing),"source_changes":[]}


def build_driver(directory):
    check()
    core = read(CORE)
    require(core["checks_passed"] is True and core["energyplus_commit"] == PIN and core["scientific_source_patches"] is False,
            "Unverified unchanged native core")
    require(all(binding_matches(core["artifacts"][k]) for k in ["core_library","api_library"]), "Original core/API changed")
    source_before=original_source_guard(core)
    directory = fresh(directory)
    sources = archive_sources(directory)
    # Write exactly the selected original bytes, not a rewritten math fragment.
    body = (SOURCE/"src/EnergyPlus/SurfaceGeometry.cc").read_text(encoding="utf-8").splitlines(keepends=True)[272:289]
    fragment = directory/"source/geo02_setup273-289.inc"
    fragment.write_text("".join(body), encoding="utf-8", newline="\n")
    selected = next(x for x in read(CONTRACTS/"GEO-02-source.json")["selected_ranges"]
                    if x["path"]=="src/EnergyPlus/SurfaceGeometry.cc" and x["start"]==273)
    require(sha(fragment) == selected["range_sha256"], "Verbatim source fragment differs")
    baseline = read(ROOT/".runtime/porting/PSY-02/native-driver-replay-target-frozen/configure-command.json")
    command = ["-DCMAKE_PROJECT_INCLUDE="+str(ROOT/"tools/porting/geo02_reference.cmake")
               if x.startswith("-DCMAKE_PROJECT_INCLUDE=") else x for x in baseline["command"]]
    command.append("-DGEO02_SETUP_FRAGMENT="+fragment.as_posix())
    before = read(BUILD/"compile_commands.json")
    own_names={"geo02_reference.cpp","geo02_reference_helper.cpp"}
    previous=[x for x in before if Path(x["file"]).name not in own_names]
    own_previous=[x for x in before if Path(x["file"]).name in own_names]
    frozen_path = ROOT/".runtime/porting/reference-energyplus-26.1.0/provenance/final-core/compile_commands.json"
    frozen = read(frozen_path)
    require(all(row in before for row in frozen), "Frozen original compile rows changed")
    old = [ref(p) for p in (BUILD/"Products").glob("*.exe") if p.name.startswith(("clk01_","psy02_","geo01_"))]
    objexx = BUILD/"Products/libobjexx.a"
    objexx_binding = {**ref(objexx), "bytes":objexx.stat().st_size}
    configured = process(command,directory,"configure")
    require(configured["exit_code"] == 0,"Native configure failed; preserved raw receipts")
    rows = read(BUILD/"compile_commands.json")
    require(all(row in rows for row in previous),"Existing original/previous-card target compilation changed")
    driver_rows = [next(x for x in rows if Path(x["file"]).name==name)
                   for name in ["geo02_reference.cpp","geo02_reference_helper.cpp"]]
    for row in driver_rows:
        for flag in ["-DEP_psych_errors","-UNDEBUG","-Werror","-O0","-ffp-contract=off"]:
            require(flag in row["command"],"Missing original native flag "+flag)
        for flag in ["-D_GLIBCXX_DEBUG","-ffast-math","-DEP_psych_stats"]:
            require(flag not in row["command"],"Incorrect original ABI/mode "+flag)
    write(directory/"actual-driver-compile-commands.json",driver_rows)
    built = process([command[0],"--build",str(BUILD),"--target","geo02_reference","geo02_reference_helper","--parallel","1","--verbose"],directory,"build")
    require(built["exit_code"] == 0,"Native build/link failed; preserved raw receipts")
    after = read(BUILD/"compile_commands.json")
    require(all(x in after for x in previous) and all(x in after for x in frozen),"Original/previous target compile rows changed")
    require(all(binding_matches(x) for x in old),"A prior card executable changed")
    require(all(binding_matches(core["artifacts"][k]) for k in ["core_library","api_library"]),"Original core/API bytes changed")
    require(sha(objexx)==objexx_binding["sha256"] and objexx.stat().st_size==objexx_binding["bytes"],"Original Objexx library changed")
    source_after=original_source_guard(core)
    require(source_before==source_after,"Original source guard changed during native build")
    binaries = {}
    for name in ["geo02_reference","geo02_reference_helper"]:
        binary = BUILD/"Products"/(name+".exe")
        archived = directory/binary.name
        shutil.copyfile(binary,archived)
        binaries[name] = {"binary":ref(archived),"linked_build_binary":ref(binary)}
    for name in ["CMakeCache.txt","compile_commands.json"]:
        shutil.copyfile(BUILD/name,directory/name)
    receipt = {"schema":"geo02-native-driver-build.v1","checks_passed":True,"energyplus_commit":PIN,
        "core_build":ref(CORE),"source_contract":contracts()["source"],"contracts":contracts(),
        "binary":binaries["geo02_reference"]["binary"],"helper_binary":binaries["geo02_reference_helper"]["binary"],
        "binaries":binaries,"executed_source_archives":sources,
        "setup_fragment":{"artifact":ref(fragment),"source_range":selected,"whole_SetupZoneGeometry_claimed":False},
        "configure":ref(directory/"configure-command.json"),"compile_link":ref(directory/"build-command.json"),
        "actual_compile_commands":ref(directory/"actual-driver-compile-commands.json"),
        "cache":ref(directory/"CMakeCache.txt"),"compile_commands":ref(directory/"compile_commands.json"),
        "frozen_original_build_commands":ref(frozen_path),"frozen_original_build_command_count":len(frozen),
        "frozen_original_build_commands_unchanged":True,"existing_target_compile_commands_unchanged":True,
        "previous_card_compile_row_count":len(previous),"own_driver_prior_attempt_rows":own_previous,
        "own_driver_prior_attempt_policy":"only these two test targets may change their source-fragment archive path between preserved build attempts; original and all previous-card rows remain exact",
        "previous_card_products":old,"linked_container_library":objexx_binding,"container_library_unchanged":True,
        "original_core_and_API_bytes_unchanged":True,"same_compiler_owned_state":True,
        "original_source_guard":source_after,
        "original_directory_and_target_definitions_inherited":True,"assertions_enabled":True,"fp_contract":"off",
        "scientific_source_patches":False,"gates_updated":False}
    write(directory/"native-driver-build.json",receipt)
    return {"driver_build":ref(directory/"native-driver-build.json"),"binary":receipt["binary"],"helper_binary":receipt["helper_binary"]}


def verified_driver(path):
    driver = read(path)
    require(driver["checks_passed"] is True and driver["energyplus_commit"]==PIN,"Unverified driver")
    require(driver["source_contract"]==contracts()["source"] and driver["contracts"]==contracts(),"Driver contract freeze differs")
    require(binding_matches(driver["core_build"]) and driver["core_build"]==ref(CORE),"Driver original core binding differs")
    for key in ["binary","helper_binary"]:
        require(binding_matches(driver[key]),"Archived driver binary changed")
    return driver


def run_helpers(directory,driver_path):
    check()
    driver=verified_driver(driver_path)
    request=read(CONTRACTS/"GEO-02-cases.json")["helper_request"]
    directory=fresh(directory)
    sources=archive_sources(directory)
    execution=process([str(ROOT/driver["helper_binary"]["path"]),str(ROOT/request["path"])],directory,"helper")
    stdout=directory/"helper-stdout.log"
    if stdout.stat().st_size:
        result=read(stdout)
        write(directory/"helper-results.json",result)
    else:
        result=None
    receipt={"schema":"geo02-original-helper-execution.v1","request":request,"results":ref(directory/"helper-results.json") if result else None,
        "execution":ref(directory/"helper-command.json"),"binary":driver["helper_binary"],"native_core_build":ref(CORE),
        "native_driver_build":ref(driver_path),"contracts":contracts(),"executed_source_archives":sources,
        "reference_method":"genuine-prepared-original-state-whole-helper-and-unchanged-header-cen",
        "kernel_input_identity_passed":result is not None and result.get("valid_kernel_input_identity_passed") is True,
        "original_parser_admission_claimed":False,"physics_executed":False,"Rust_compared":False,"gates_updated":False}
    write(directory/"helper-reference.json",receipt)
    require(execution["exit_code"]==0 and receipt["kernel_input_identity_passed"],"Original helper failed/input identity changed; raw proof retained; stop before Rust baseline")
    require(result["schema"]=="geo02-helper-results.v1" and len(result["cases"])==17 and len(result["cen_calls"])==526,"Original helper output count/schema differs")
    return {"receipt":ref(directory/"helper-reference.json"),"results":receipt["results"],"kernel_input_identity_passed":True}


def run_original(directory,driver_path,requested_ids):
    check()
    driver=verified_driver(driver_path)
    frozen=read(CONTRACTS/"GEO-02-cases.json")
    rows=frozen["native_cases"]+frozen["native_invalid_cases"]
    if requested_ids:
        rows=[x for x in rows if x["id"] in requested_ids]
        require(len(rows)==len(requested_ids),"Unknown/duplicate selected original cases")
    directory=fresh(directory)
    sources=archive_sources(directory)
    idd=BUILD/"Products/Energy+.idd"
    output_rows=[]
    for row in rows:
        output=directory/row["id"]
        output.mkdir()
        command=[str(ROOT/driver["binary"]["path"]),str(ROOT),str(CORE),row["id"],
                 str(ROOT/row["input"]["path"]),str(ROOT/row["weather"]["path"]),str(idd),str(output/"native")]
        execution=process(command,output,"original")
        result_path=output/"native/geometry-observation.json"
        data=read(result_path) if result_path.exists() else None
        receipt={"schema":"geo02-original-execution.v1","case_id":row["id"],"kind":row["kind"],
            "execution":ref(output/"original-command.json"),"binary":driver["binary"],"native_core_build":ref(CORE),
            "native_driver_build":ref(driver_path),"executed_source_archives":sources,"contracts":contracts(),
            "input":row["input"],"weather":row["weather"],"idd":ref(idd),
            "results":ref(result_path) if data else None,
            "geometry_error_log":ref(output/"native/eplusout.err") if (output/"native/eplusout.err").exists() else None,
            "reference_method":"genuine-full-original-input-parser-and-retained-state","physical_input_scope_expanded":False,
            "rust_compared":False,"gates_updated":False}
        write(output/"receipt.json",receipt)
        output_rows.append({"case_id":row["id"],"kind":row["kind"],"receipt":ref(output/"receipt.json"),"results":receipt["results"]})
        write(directory/"matrix.json",{"schema":"geo02-original-matrix.v1","cases":output_rows,"case_count":len(output_rows),
              "complete":len(output_rows)==len(rows),"Rust_compared":False,"gates_updated":False})
        require(execution["exit_code"]==0 and data is not None and data["callback_error"]=="","Original wrapper/observation failed; preserved raw proof")
        if row["kind"]=="ordinary-input-negative-not-CON":
            require(type(data["energyplus_exit_code"]) is int and data["energyplus_exit_code"]!=0 and data["physical_zone_callback_count"]==0,
                    "Frozen original negative input was not blocked before physics")
        else:
            require(data["energyplus_exit_code"]==0 and data["first_final_owned_geometry_identity_exact"] is True,"Original valid execution/retention failed")
            require(data["physical_zone_callback_count"]=={"24H":96,"72H":288}[row["duration"]],"Original physical clock coverage incomplete")
            for phase in ["first_initialized","first_physical","final_weather","final"]:
                value=data[phase]
                require(value["surface_count"]==6 and value["zone_count"]==1,"Original valid topology changed")
                flags=value["native_only_state"]
                require(flags["total_coincident_vertices"]==flags["total_degenerate_surfaces"]==0 and flags["aspect_transform"] is False and flags["no_transform"] is True,"Excluded geometry correction active")
                require(all(s["sides"]==4 and s["native_consumer"]["is_degenerate"] is False for s in value["surfaces"]),"Original valid vertex/cardinality changed")
            error=(output/"native/eplusout.err").read_text(encoding="utf-8")
            patterns=[r"coincident vert",r"degenerate surface",r"(?:roof/ceiling|floor) is (?:upside down|not oriented correctly)",r"automatic fix is attempted",r"geometrytransform",r"non-planar surface"]
            require(not any(re.search(p,error,re.I) for p in patterns),"Excluded geometry-specific warning/repair observed")
    return {"matrix":ref(directory/"matrix.json"),"case_count":len(output_rows)}
