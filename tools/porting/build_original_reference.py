#!/usr/bin/env python3
"""Build the pinned original EnergyPlus core with a disclosed O0 test configuration.

Uses repository-local verified GCC13.2/CMake/Ninja and an already prepared original
source checkout. The source must have its own official upstream Git metadata and
byte-identical locked-commit blobs. No checkout, source patch, global PATH change,
warning waiver, physics comparison, or porting gate update is performed.

Fresh build/output directories are mandatory. --check verifies an existing native
core receipt and its live source/tool/artifact bindings without writing or invoking
CMake/Ninja. The default optimized build failures remain separate preserved facts.
"""
from __future__ import annotations

import argparse
import ctypes
import hashlib
import json
import os
from pathlib import Path
import platform
import shlex
import shutil
import subprocess
import sys
import time
import tomllib

sys.dont_write_bytecode = True
from setup_reference_tools import install, tool_directory  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
RUNTIME = ROOT / ".runtime"
PIN = "6f2e40d10250a105b49966baa24d843711e61048"
ORIGIN = "https://github.com/NatLabRockies/EnergyPlus.git"
VERSION = "EnergyPlus, Version 26.1.0-6f2e40d102"
DEFAULT_RECEIPT = RUNTIME / "porting/reference-energyplus-26.1.0/native-core-build.json"
DEFAULT_BUILD = RUNTIME / "ep261-gcc13-o0"
DEFAULT_OUTPUT = RUNTIME / "porting/reference-energyplus-26.1.0/reproduction"
ABSENT_ARCHIVE_METADATA = {
    "third_party/penumbra/vendor/glfw/.gitattributes",
    "third_party/penumbra/vendor/glfw/.gitignore",
}
DEPFILE = "third_party/Windows-CalcEngine/CMakeFiles/Windows-CalcEngine.dir/src/MultiLayerOptics/src/MultiLayerInterRefSingleComponent.cpp.obj.d"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def digest(path: Path, algorithm: str = "sha256") -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, algorithm).hexdigest()


def read(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def write(path: Path, value: dict) -> None:
    path.write_text(json.dumps(value, indent=2, allow_nan=False) + "\n", encoding="utf-8", newline="\n")


def local(path: Path, boundary: Path = ROOT) -> Path:
    resolved, allowed = path.resolve(), boundary.resolve()
    require(allowed.is_relative_to(ROOT) and resolved.is_relative_to(allowed) and resolved != allowed,
            f"Path must remain inside {allowed}: {path}")
    return resolved


def ref(path: Path) -> dict:
    path = local(path)
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": digest(path), "bytes": path.stat().st_size}


def verify_ref(binding: dict) -> Path:
    path = local(ROOT / binding["path"])
    require(path.is_file() and digest(path) == binding["sha256"], f"Changed/missing provenance: {path}")
    require("bytes" not in binding or path.stat().st_size == binding["bytes"], f"Provenance size differs: {path}")
    return path


def verify_bindings(value: object) -> None:
    if isinstance(value, dict):
        if "path" in value and "sha256" in value:
            verify_ref(value)
        for child in value.values():
            verify_bindings(child)
    elif isinstance(value, list):
        for child in value:
            verify_bindings(child)


def git(source: Path, *args: str) -> str:
    return subprocess.check_output(["git", "-C", str(source), *args], text=True, encoding="utf-8").strip()


def source_check() -> tuple[Path, dict]:
    oracle = tomllib.loads((ROOT / "config/default.toml").read_text(encoding="utf-8"))["oracle"]
    require(oracle["energyplus_version"] == "26.1.0" and oracle["source_commit"] == PIN,
            "This reviewed compiler recipe is pinned to the canonical EnergyPlus26.1.0 commit")
    source = local(ROOT / oracle["source_dir"], ROOT / ".reference")
    require((source / ".git").is_dir(), "Original source needs its own verified upstream Git checkout; inherited parent Git is rejected")
    require(Path(git(source, "rev-parse", "--show-toplevel")).resolve() == source, "Source Git toplevel differs")
    require(git(source, "rev-parse", "HEAD") == PIN and git(source, "remote", "get-url", "origin") == ORIGIN,
            "Original source HEAD/origin is not the locked official upstream")
    records = subprocess.check_output(["git", "-C", str(source), "ls-tree", "-r", "-z", "HEAD"])
    missing, changed, count = [], [], 0
    for row in records.split(b"\0"):
        if not row:
            continue
        metadata, raw_name = row.split(b"\t", 1)
        mode, kind, expected = metadata.decode("ascii").split()
        name = os.fsdecode(raw_name)
        require(kind == "blob" and mode in {"100644", "100755", "120000"}, "Unreviewed upstream tree entry")
        path = local(source / name, source)
        if not path.is_file():
            missing.append(name)
            continue
        blob = hashlib.sha1(f"blob {path.stat().st_size}\0".encode("ascii"))
        with path.open("rb") as stream:
            for chunk in iter(lambda: stream.read(1024 * 1024), b""):
                blob.update(chunk)
        if blob.hexdigest() != expected:
            changed.append(name)
        count += 1
    require(not changed and set(missing) <= ABSENT_ARCHIVE_METADATA,
            f"Original source differs from locked blobs: changed={changed[:5]}, missing={missing}")
    untracked = git(source, "ls-files", "--others", "--exclude-standard").splitlines()
    require(set(untracked) <= {"source.sha256"}, f"Unreviewed untracked source input: {untracked[:5]}")
    print(f"PASS original source: {count} byte-identical official blobs", flush=True)
    return source, {"energyplus_commit": PIN, "official_origin": ORIGIN,
                    "source_directory": source.relative_to(ROOT).as_posix(),
                    "tracked_present_files_checked": count, "tracked_content_mismatches": changed,
                    "missing_original_archive_non_scientific_git_metadata": missing,
                    "scientific_source_patches": False, "source_worktree_replaced": False}


def tools_check(query_versions: bool = True) -> dict[str, tuple[dict, Path]]:
    manifest = read(Path(__file__).with_name("reference_tools.json"))
    selected = {}
    for name in ["winlibs-gcc", "cmake", "ninja"]:
        tool = next(item for item in manifest["tools"] if item["name"] == name)
        install(tool, True, launch_version=query_versions)
        selected[name] = tool, tool_directory(tool) / tool["executable"]
    require(selected["winlibs-gcc"][0]["version"] == "13.2.0-ucrt-r3", "Unreviewed compiler version")
    return selected


def environment(tools: dict) -> dict[str, str]:
    env = os.environ.copy()
    env["PATH"] = str(tools["winlibs-gcc"][1].parent) + os.pathsep + str(tools["ninja"][1].parent) + os.pathsep + env.get("PATH", "")
    env["PYTHONUTF8"], env["PYTHONIOENCODING"] = "1", "utf-8"
    return env


def macro_names(text: str) -> set[str]:
    return {line.split(maxsplit=2)[1].split("(", 1)[0] for line in text.splitlines() if line.startswith("#define ")}


def options_check(options: dict) -> None:
    require(options["effective_assertions_enabled"] is True and options["effective_fast_math_enabled"] is False
            and options["effective_finite_math_only"] == 0
            and options["effective_posix_debug_trap_macro_enabled"] is False
            and options["effective_libstdcxx_debug_container_abi_enabled"] is False
            and options["extra_warning_waivers"] is False, "Original target assertion/FP/ABI invariants differ")
    command = options["exact_source_command"]
    require(command.index("-DNDEBUG") < command.index("-UNDEBUG") and " -Werror " in command
            and " -O0 " in command and "-Wa,-mbig-obj" in command and "-ffp-contract=off" in command
            and "-ffast-math" not in command, "Actual source flags differ from reviewed test-reference configuration")
    raw = verify_ref(options["compiler_macro_stdout"]).read_text(encoding="utf-8")
    require(not ({"NDEBUG", "__FAST_MATH__", "DEBUG_ARITHM_GCC_OR_CLANG", "_GLIBCXX_DEBUG"} & macro_names(raw))
            and "#define __FINITE_MATH_ONLY__ 0" in raw, "Actual exact source macro definitions differ")


def smoke(dll: Path, compiler: Path) -> dict:
    # Opaque state is allocated/deleted by this same GCC DLL; no MSVC private ABI.
    handles = [os.add_dll_directory(str(compiler.parent)), os.add_dll_directory(str(dll.parent))]
    api = ctypes.CDLL(str(dll))
    api.energyPlusVersion.argtypes, api.energyPlusVersion.restype = [], ctypes.c_char_p
    version = api.energyPlusVersion().decode("utf-8")
    require(version == VERSION, "Built upstream version/hash differs")
    api.stateNew.argtypes, api.stateNew.restype = [], ctypes.c_void_p
    api.stateDelete.argtypes, api.stateDelete.restype = [ctypes.c_void_p], None
    api.apiVersionFromEPlus.argtypes, api.apiVersionFromEPlus.restype = [ctypes.c_void_p], ctypes.c_char_p
    state = api.stateNew()
    require(bool(state), "Genuine stateNew returned null")
    try:
        api_version = api.apiVersionFromEPlus(state).decode("utf-8")
    finally:
        api.stateDelete(state)
    # Keep DLL search-directory handles alive until all native calls finish.
    for handle in handles:
        handle.close()
    return {"checks_passed": True, "observed_version": version, "api_version": api_version,
            "actual_state_new_nonnull": True, "actual_state_delete_called": True,
            "official_MSVC_private_state_crossing": False}


def check(receipt_path: Path, tools: dict) -> None:
    core = read(local(receipt_path, RUNTIME))
    require(core["schema"] == "native-core-build.v1" and core["checks_passed"] is True and core["energyplus_commit"] == PIN,
            "Core has no successful original-source build proof")
    require(core["scientific_source_patches"] is False and core["extra_warning_waivers"] is False
            and core["configuration"]["reference_optimization_override"] == "-O0 -g -DNDEBUG"
            and core["configuration"]["source_default_optimized_build_passes"] is False,
            "Core must disclose the test configuration and preserved default-optimized failure")
    verify_bindings(core)
    require(core["compiler"]["binary"]["sha256"] == tools["winlibs-gcc"][0]["executable_sha256"], "Core compiler differs")
    proof = read(verify_ref(core["source_preservation"]))
    require(proof["energyplus_commit"] == PIN and proof["tracked_content_mismatches"] == []
            and proof["scientific_source_patches"] is False, "Core source-preservation proof differs")
    verify_bindings(proof)
    options = read(verify_ref(core["configuration"]["actual_options"]))
    verify_bindings(options)
    options_check(options)
    require(core["configure"]["exit_code"] == core["build"]["exit_code"] == 0, "Configure/build failed")
    observed = smoke(verify_ref(core["artifacts"]["api_library"]), tools["winlibs-gcc"][1])
    print(f"PASS native core: {observed['observed_version']}; API{observed['api_version']}; genuine state lifecycle", flush=True)
    print("PASS explicit O0 reference; default optimized failure remains disclosed; no files written.", flush=True)


def run(name: str, command: list[str], output: Path, env: dict, cwd: Path = ROOT) -> dict:
    stdout, stderr = output / (name + "-stdout.log"), output / (name + "-stderr.log")
    started = time.perf_counter()
    print(f"RUN {name}", flush=True)
    with stdout.open("wb") as out, stderr.open("wb") as err:
        completed = subprocess.run(command, cwd=cwd, env=env, stdout=out, stderr=err, check=False)
    result = {"command": command, "working_directory": cwd.relative_to(ROOT).as_posix(),
              "exit_code": completed.returncode, "elapsed_seconds": round(time.perf_counter() - started, 3),
              "stdout": ref(stdout), "stderr": ref(stderr)}
    write(output / (name + "-receipt.json"), result)
    require(completed.returncode == 0, f"{name} failed with exit{completed.returncode}; preserved logs: {output}")
    return result


def build(source: Path, before: dict, tools: dict, directory: Path, output: Path, jobs: int) -> None:
    directory, output = local(directory, RUNTIME), local(output, RUNTIME)
    require(directory != output and not directory.is_relative_to(output) and not output.is_relative_to(directory), "Build/output directories must be distinct")
    require(not directory.exists() and not output.exists(), "Fresh directories required; use --check to verify existing immutable proof")
    require(len(str(directory / DEPFILE)) < 250, "Choose a shorter .runtime build directory for Windows GNU dependency paths")
    output.mkdir(parents=True, exist_ok=False)
    directory.mkdir(parents=True, exist_ok=False)
    write(output / "source-before.json", before)
    gcc = tools["winlibs-gcc"][1]
    cmake = tools["cmake"][1]
    env = environment(tools)
    configure = [str(cmake), "-S", str(source), "-B", str(directory), "-G", "Ninja",
                 "-DCMAKE_MAKE_PROGRAM=" + str(tools["ninja"][1]), "-DCMAKE_C_COMPILER=" + str(gcc.with_name("gcc.exe")),
                 "-DCMAKE_CXX_COMPILER=" + str(gcc), "-DCMAKE_BUILD_TYPE=RelWithDebInfo",
                 "-DCMAKE_CXX_FLAGS_RELWITHDEBINFO=-O0 -g -DNDEBUG", "-DCMAKE_CXX_FLAGS=-Wa,-mbig-obj -ffp-contract=off",
                 "-DFORCE_DEBUG_ARITHM_GCC_OR_CLANG=OFF", "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON", "-DBUILD_TESTING=OFF",
                 "-DBUILD_FORTRAN=OFF", "-DBUILD_PACKAGE=OFF", "-DDOCUMENTATION_BUILD=DoNotBuild", "-DLINK_WITH_PYTHON=OFF",
                 "-DPython_EXECUTABLE=" + sys.executable]
    configured = run("configure", configure, output, env)
    built = run("build", [str(cmake), "--build", str(directory), "--target", "energyplusapi", "--parallel", str(jobs)], output, env)
    _, after = source_check()
    require(before == after, "Source changed during original compilation")
    write(output / "source-preservation.json", after)
    for filename in ["CMakeCache.txt", "compile_commands.json"]:
        shutil.copyfile(directory / filename, output / filename)
    version_source = directory / "src/EnergyPlus/DataStringGlobals.cc"
    require(VERSION in version_source.read_text(encoding="utf-8"), "Generated upstream version differs")
    shutil.copyfile(version_source, output / "DataStringGlobals.cc")
    commands = read(output / "compile_commands.json")
    command = next(item["command"] for item in commands if item["file"].endswith("Psychrometrics.cc"))
    arguments = [token.strip('"') for token in shlex.split(command, posix=False)]
    index = arguments.index("-o")
    del arguments[index:index + 2]
    arguments[arguments.index("-c")] = "-E"
    arguments.insert(-1, "-dM")
    macros = run("source-macros", arguments, output, env, directory)
    options = {"effective_assertions_enabled": True, "effective_fast_math_enabled": False, "effective_finite_math_only": 0,
               "effective_posix_debug_trap_macro_enabled": False, "effective_libstdcxx_debug_container_abi_enabled": False,
               "extra_warning_waivers": False, "exact_source_command": command, "compiler_macro_stdout": macros["stdout"],
               "compiler_macro_stderr": macros["stderr"], "macro_probe": macros,
               "cmake_cache": ref(output / "CMakeCache.txt"), "compile_commands": ref(output / "compile_commands.json"),
               "generated_version_source": ref(output / "DataStringGlobals.cc")}
    options_check(options)
    write(output / "actual-options.json", options)
    dll, archive = directory / "Products/energyplusapi.dll", directory / "Products/libenergypluslib.a"
    observed = smoke(dll, gcc)
    write(output / "api-smoke.json", observed)
    receipt = {"schema": "native-core-build.v1", "checks_passed": True, "energyplus_commit": PIN,
               "purpose": "Original source-built test reference; no porting gate or physics claim",
               "configuration": {"build_type": "RelWithDebInfo", "reference_optimization_override": "-O0 -g -DNDEBUG",
                                 "source_default_optimized_build_passes": False, "actual_options": ref(output / "actual-options.json")},
               "compiler": {"version": subprocess.check_output([str(gcc), "--version"], env=env, text=True).splitlines()[0],
                            "abi": "x86_64 Windows MinGW-w64 UCRT POSIX SEH; non-debug libstdc++ containers", "binary": ref(gcc)},
               "source_preservation": ref(output / "source-preservation.json"),
               "artifacts": {"api_library": ref(dll), "core_library": ref(archive)},
               "configure": configured, "build": built, "smoke": ref(output / "api-smoke.json"),
               "environment_overrides": {"PYTHONUTF8": "1", "PYTHONIOENCODING": "utf-8"},
               "process_only_path_prefix": [str(gcc.parent), str(tools["ninja"][1].parent)],
               "build_directory": directory.relative_to(ROOT).as_posix(), "scientific_source_patches": False,
               "extra_warning_waivers": False, "tool_manifest": ref(Path(__file__).with_name("reference_tools.json")),
               "executed_builder": ref(Path(__file__)), "same_compiler_abi_required_for_private_state": True}
    write(output / "native-core-build.json", receipt)
    print(f"PASS original native reference: {output / 'native-core-build.json'}", flush=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--check", action="store_true", help="read-only verification of an existing receipt; never invokes CMake/Ninja")
    parser.add_argument("--receipt", type=Path, default=DEFAULT_RECEIPT, help="existing core proof used by --check")
    parser.add_argument("--build-dir", type=Path, default=DEFAULT_BUILD, help="fresh, short directory inside .runtime")
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT, help="fresh logs/receipts directory inside .runtime")
    parser.add_argument("--jobs", type=int, default=min(8, os.cpu_count() or 1), choices=range(1, 9))
    args = parser.parse_args()
    require(platform.system() == "Windows" and platform.machine().upper() in {"AMD64", "X86_64"}, "This recipe targets Windows x86_64")
    source, proof = source_check()
    tools = tools_check(query_versions=not args.check)
    if args.check:
        check(args.receipt, tools)
    else:
        build(source, proof, tools, args.build_dir, args.output_dir, args.jobs)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (ValueError, OSError, subprocess.SubprocessError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        raise SystemExit(1)
