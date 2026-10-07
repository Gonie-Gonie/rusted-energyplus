#!/usr/bin/env python3
"""Record a real command and deduplicated, immutable Rust source bytes.

This records provenance only. It neither computes reference values nor changes
porting gates. Each receipt retains the historical command and repository state.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
RAW = ROOT / ".runtime/porting"
OBJECTS = RAW / "source-objects"


def sha(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def ref(path: Path) -> dict:
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": sha(path)}


def contained(path: Path, parent: Path) -> Path:
    path, parent = path.resolve(), parent.resolve()
    if not path.is_relative_to(parent) or path == parent:
        raise ValueError(f"Expected a child of {parent}: {path}")
    return path


def store_bytes(data: bytes) -> dict:
    digest = hashlib.sha256(data).hexdigest()
    path = contained(OBJECTS / digest[:2] / digest, RAW)
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.exists():
        if path.read_bytes() != data:
            raise ValueError(f"Immutable source object differs: {path}")
    else:
        with path.open("xb") as stream:
            stream.write(data)
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": digest}


def json_bytes(value: dict) -> bytes:
    return (json.dumps(value, ensure_ascii=False, indent=2, allow_nan=False) + "\n").encode("utf-8")


def source_paths() -> list[Path]:
    crates = ROOT / "crates"
    paths = set(crates.rglob("*.rs")) | set(crates.rglob("Cargo.toml"))
    for name in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", ".cargo/config.toml"):
        path = ROOT / name
        if path.is_file():
            paths.add(path)
    return sorted(paths)


def source_snapshot() -> dict:
    rows = []
    for path in source_paths():
        contained(path, ROOT)
        data = path.read_bytes()
        rows.append({"historical_path": path.relative_to(ROOT).as_posix(),
                     "size_bytes": len(data), "archive": store_bytes(data)})
    return store_bytes(json_bytes({
        "schema": "rust-command-source-snapshot.v1",
        "scope": "all crates Rust sources/Cargo manifests and existing workspace Cargo/toolchain configuration",
        "bytes_normalized": False,
        "files": rows,
    }))


def snapshot_still_matches(snapshot: dict) -> bool:
    path = ROOT / snapshot["path"]
    if sha(path) != snapshot["sha256"]:
        raise ValueError("Source snapshot changed")
    manifest = json.loads(path.read_text(encoding="utf-8"))
    rows = manifest["files"]
    if {path.relative_to(ROOT).as_posix() for path in source_paths()} != {row["historical_path"] for row in rows}:
        return False
    for row in rows:
        path = contained(ROOT / row["historical_path"], ROOT)
        if not path.is_file() or sha(path) != row["archive"]["sha256"]:
            return False
    return True


def git_state() -> dict:
    head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    status = subprocess.check_output(["git", "status", "--porcelain=v1", "-z"], cwd=ROOT)
    return {"head": head, "worktree_clean": not status,
            "porcelain_v1_nul_utf8": status.decode("utf-8")}


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.command
    if command and command[0] == "--":
        command = command[1:]
    if not command:
        parser.error("Supply a command after --")
    directory = contained(ROOT / args.output_dir, RAW)
    directory.mkdir(parents=True, exist_ok=False)
    snapshot = source_snapshot()
    launcher = store_bytes(Path(__file__).read_bytes())
    before = git_state()
    stdout, stderr = directory / "stdout.log", directory / "stderr.log"
    start_utc, started = utc_now(), time.perf_counter()
    launch_error = None
    with stdout.open("xb") as out, stderr.open("xb") as err:
        try:
            result = subprocess.run(command, cwd=ROOT, stdout=out, stderr=err)
            exit_code = result.returncode
        except OSError as error:
            launch_error, exit_code = str(error), 127
            err.write((launch_error + "\n").encode("utf-8"))
    end_utc, elapsed = utc_now(), time.perf_counter() - started
    after = git_state()
    receipt = {
        "schema": "recorded-porting-command.v1", "command": command,
        "cwd": ROOT.as_posix(), "exit_code": exit_code, "launch_error": launch_error,
        "started_utc": start_utc, "finished_utc": end_utc, "elapsed_seconds": elapsed,
        "repository_before": before, "repository_after": after,
        "source_snapshot": snapshot, "source_bytes_match_before_and_after": snapshot_still_matches(snapshot),
        "executed_launcher": {"historical_path": Path(__file__).relative_to(ROOT).as_posix(), "archive": launcher},
        "python": {"version": sys.version, "executable": str(Path(sys.executable)), "sha256": sha(Path(sys.executable))},
        "stdout": ref(stdout), "stderr": ref(stderr),
        "recorder_updates_gates": False, "recorder_supplies_reference_answers": False,
    }
    receipt_path = directory / "receipt.json"
    with receipt_path.open("xb") as stream:
        stream.write(json_bytes(receipt))
    for path in (stdout, stderr):
        with path.open("rb") as stream:
            stream.seek(max(0, path.stat().st_size - 8000))
            print(stream.read().decode("utf-8", errors="replace"), end="")
    print(json.dumps({"receipt": ref(receipt_path), "exit_code": exit_code,
                      "source_bytes_match_before_and_after": receipt["source_bytes_match_before_and_after"]}))
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
