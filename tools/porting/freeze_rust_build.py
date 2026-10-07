#!/usr/bin/env python3
"""Archive an actual recorded Cargo build without running Cargo or an engine.

The source manifest inventories available files, including cfg(test) files.
It does not prove every inventoried file was selected by the compiler.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

ROOT = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip()).resolve()
RAW = ROOT / ".runtime/porting"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def contained(path: Path, parent: Path) -> Path:
    path, parent = path.resolve(), parent.resolve()
    require(path.is_relative_to(parent) and path != parent, f"Expected a child of {parent}: {path}")
    return path


def ref(path: Path) -> dict:
    path = contained(path, ROOT)
    with path.open("rb") as stream:
        digest = hashlib.file_digest(stream, "sha256").hexdigest()
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": digest}


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def read(reference: dict) -> dict:
    path = contained(ROOT / reference["path"], ROOT)
    require(ref(path) == reference, f"Hash/path binding differs: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def committed_precision(rows: list[dict]) -> dict:
    tree = git("ls-tree", "-r", "HEAD", "--", "crates")
    blobs = {
        line.split("\t", 1)[1]: line.split("\t", 1)[0].split()[2]
        for line in tree.splitlines() if line.endswith(".rs")
    }
    require(set(blobs) == {row["historical_path"] for row in rows}, "Available Rust inventory differs from committed paths")
    wanted = [blobs[row["historical_path"]] for row in rows]
    output = subprocess.check_output(
        ["git", "cat-file", "--batch"], input=("\n".join(wanted) + "\n").encode("ascii"), cwd=ROOT,
    )
    cursor, exact = 0, 0
    differences = []
    for row, blob in zip(rows, wanted, strict=True):
        end = output.index(b"\n", cursor)
        object_id, kind, size = output[cursor:end].split()
        require(object_id.decode("ascii") == blob and kind == b"blob", "Git batch identity differs")
        cursor = end + 1
        body = output[cursor:cursor + int(size)]
        require(output[cursor + int(size):cursor + int(size) + 1] == b"\n", "Git batch framing differs")
        cursor += int(size) + 1
        actual = (ROOT / row["path"]).read_bytes()
        if actual == body:
            exact += 1
        else:
            require(actual.replace(b"\r\n", b"\n") == body, f"Archived source content differs from Git: {row['historical_path']}")
            differences.append({
                "historical_path": row["historical_path"], "archive": ref(ROOT / row["path"]),
                "git_blob": blob, "git_blob_sha256": hashlib.sha256(body).hexdigest(),
                "difference": "Working-tree CRLF bytes; exact LF-normalized equality with committed blob",
            })
    require(cursor == len(output), "Unexpected trailing Git batch bytes")
    return {
        "source_count": len(rows), "exact_byte_matches": exact,
        "line_ending_only_differences": differences, "all_other_content_exact": True,
        "scope": "Archived available Rust-source inventory versus committed blobs; not compiler file-selection evidence",
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--command-receipt", required=True, type=Path)
    parser.add_argument("--output-dir", required=True, type=Path)
    kind = parser.add_mutually_exclusive_group(required=True)
    kind.add_argument("--cli", action="store_true")
    kind.add_argument("--example")
    parser.add_argument("--allow-dirty", action="store_true", help="Record a precommit build without committed-source certification")
    args = parser.parse_args()
    execution_path = contained(ROOT / args.command_receipt, RAW)
    output_dir = contained(ROOT / args.output_dir, RAW)
    require(not output_dir.exists(), "Build output directory must be fresh")
    execution_ref = ref(execution_path)
    execution = read(execution_ref)
    require(execution["schema"] == "recorded-porting-command.v1", "Unexpected command receipt schema")
    recorder_archive = execution["executed_launcher"]["archive"]
    require(ref(ROOT / recorder_archive["path"]) == recorder_archive, "Actually executed recorder archive differs")
    require(Path(execution["cwd"]).resolve() == ROOT, "Recorded build was outside this repository")
    require(execution["exit_code"] == 0 and execution["launch_error"] is None, "Cargo command did not succeed")
    require(execution["source_bytes_match_before_and_after"] is True, "Build source bytes changed during recorded command")
    if args.cli:
        expected = ["cargo", "build", "-p", "ep_cli", "--bin", "eplus-rs", "-j", "2", "--message-format=json-render-diagnostics"]
        target_name, cargo_kind, package = "eplus-rs", "bin", "ep_cli"
        target_source = ROOT / "crates/ep_cli/src/main.rs"
        target_kind = "cli"
    else:
        require(args.example.isidentifier() and args.example.isascii(), "Example must be a plain ASCII identifier")
        expected = ["cargo", "build", "-p", "ep_runtime", "--example", args.example, "-j", "2", "--message-format=json-render-diagnostics"]
        target_name, cargo_kind, package = args.example, "example", "ep_runtime"
        target_source = ROOT / f"crates/ep_runtime/examples/{args.example}.rs"
        target_kind = "example"
    require(execution["command"] == expected, "Recorded Cargo target/arguments differ")
    capture = execution["cargo_compiler_artifacts_capture"]
    require(capture["requested"] is True and capture["status"] == "pass", "Cargo executable identity was not captured after the command")
    candidates = [row for row in capture["rows"] if
                  row["cargo_message"]["target"]["name"] == target_name and
                  row["cargo_message"]["target"]["kind"] == [cargo_kind] and
                  Path(row["cargo_message"]["manifest_path"]).resolve() == ROOT / f"crates/{package}/Cargo.toml" and
                  Path(row["cargo_message"]["target"]["src_path"]).resolve() == target_source]
    require(len(candidates) == 1, "Expected exactly one Cargo-emitted executable for this package/source/target")
    artifact = candidates[0]
    binary = contained(ROOT / artifact["executable"]["historical_path"], ROOT)
    require(Path(artifact["cargo_message"]["executable"]).resolve() == binary, "Cargo-emitted executable path differs")
    require(ref(binary)["sha256"] == artifact["executable"]["sha256"], "Executable bytes changed since the command's artifact capture")
    head = git("rev-parse", "HEAD")
    require(head == execution["repository_before"]["head"] == execution["repository_after"]["head"], "Build/current repository revision changed")
    manifest = read(execution["source_snapshot"])
    require(manifest["schema"] == "rust-command-source-snapshot.v1" and manifest["bytes_normalized"] is False, "Unexpected source manifest")
    available = [{**row["archive"], "historical_path": row["historical_path"]} for row in manifest["files"]]
    require(len({row["historical_path"] for row in available}) == len(available), "Duplicate source inventory path")
    for row in available:
        require(ref(ROOT / row["path"])["sha256"] == row["sha256"], "Immutable source object differs")
        require(ref(ROOT / row["historical_path"])["sha256"] == row["sha256"], "Current source differs from recorded build")
    from record_command import source_paths
    require({path.relative_to(ROOT).as_posix() for path in source_paths()} == {row["historical_path"] for row in available}, "Available source inventory changed since build")
    clean = not git("status", "--porcelain", "--", "crates", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", ".cargo")
    require(clean or args.allow_dirty, "Dirty Rust build requires explicit --allow-dirty; no committed certification")
    rust_rows = [row for row in available if row["historical_path"].endswith(".rs")]
    precision = committed_precision(rust_rows) if clean else None
    output_dir.mkdir(parents=True)
    dependency = Path(__file__).resolve().with_name("record_command.py")
    for source in (binary, ROOT / "Cargo.lock", ROOT / "rust-toolchain.toml", Path(__file__).resolve(), dependency):
        target = output_dir / source.name
        shutil.copyfile(source, target)
        require(ref(source)["sha256"] == ref(target)["sha256"], "Archived build artifact differs")
    receipt = {
        "schema": "porting-Rust-build.v1", "kind": target_kind, "example": args.example,
        "binary": ref(output_dir / binary.name),
        "actual_Cargo_executable": {**artifact, "matching_archive": ref(output_dir / binary.name)},
        "build_execution": execution_ref,
        "source_snapshot": execution["source_snapshot"], "repository_head": head,
        "source_worktree_clean": clean, "committed_source_certification": clean,
        "available_Rust_sources": rust_rows, "available_source_inventory_count": len(available),
        "source_inventory_scope": manifest["scope"], "archived_source_vs_committed_blobs": precision,
        "implementation_commit": head if clean else None, "crates_tree": git("rev-parse", "HEAD:crates") if clean else None,
        "cargo_lock": ref(output_dir / "Cargo.lock"), "toolchain": ref(output_dir / "rust-toolchain.toml"),
        "executed_freezer": ref(output_dir / Path(__file__).name),
        "reader_dependency": ref(output_dir / dependency.name),
        "historical_reader_dependency": {"historical_path": dependency.relative_to(ROOT).as_posix(), "sha256": ref(dependency)["sha256"]},
        "actual_executed_recorder": execution["executed_launcher"],
        "metadata_argv": __import__("sys").argv,
        "build_environment_diagnostic_after_command": {key: os.environ.get(key) for key in (
            "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_TARGET", "RUSTUP_TOOLCHAIN", "RUSTC", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER",
        )},
        "build_environment_diagnostic_scope": "Captured after command; not evidence of transient build environment",
        "rustc_version_diagnostic_after_command": subprocess.check_output(["rustc", "-Vv"], cwd=ROOT, text=True).strip(),
        "cargo_version_diagnostic_after_command": subprocess.check_output(["cargo", "-V"], cwd=ROOT, text=True).strip(),
        "toolchain_diagnostic_scope": "Queried during freeze; not proof of compiler executable/flags used by the earlier build",
        "gates_updated": False, "reference_outputs_supplied": False,
        "limitations": ["Source before/after equality cannot detect transient edits.", "Available-source inventory does not prove which files were compiled into the binary."],
    }
    receipt_path = output_dir / "build-receipt.json"
    receipt_path.write_text(json.dumps(receipt, indent=2, allow_nan=False) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"build": ref(receipt_path), "binary": receipt["binary"], "source_worktree_clean": clean}))


if __name__ == "__main__":
    main()
