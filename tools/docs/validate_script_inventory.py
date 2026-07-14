"""Validate the generated script inventory and its public-entrypoint contract."""

from __future__ import annotations

import argparse
import sys
import tomllib
from pathlib import Path
from typing import Any

from script_inventory import (
    DEV_COMMAND_RE,
    collect_script_inventory,
    extract_executable_script_references,
    script_inventory_toml,
)


ALLOWED_CLASSIFICATIONS = {"public", "internal", "removable"}
EXPECTED_SCHEMA = "rusted-energyplus.script-inventory.v1"
PUBLIC_ENTRYPOINTS = {"dev.cmd"}
PUBLIC_ENTRYPOINT_PREFIXES = ("dev:", "rust-cli:")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate scripts inventory metadata.")
    parser.add_argument("--repo-root", required=True, type=Path)
    return parser.parse_args()


def require(condition: bool, errors: list[str], message: str) -> None:
    if not condition:
        errors.append(message)


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def has_supported_public_entrypoint(value: str) -> bool:
    return value in PUBLIC_ENTRYPOINTS or value.startswith(PUBLIC_ENTRYPOINT_PREFIXES)


def validate_parser_contract(errors: list[str]) -> None:
    sample = r'''
. (Join-Path $PSScriptRoot "lib\common.ps1")
$Runner = Join-Path $PSScriptRoot "run.ps1"
& $Runner
$Audit = Join-Path $PSScriptRoot "audit-only.ps1"
Get-Content -LiteralPath $Audit
Assert-ContainsLiteral -Needle 'Invoke-DevCommand -Command "false-command"'
Invoke-DevCommand -Command "real-command"
$lanes = @(
    "lane-a.ps1",
    "lane-b.ps1"
)
$closedLanes = @(
    "lane-closed.ps1"
)
$allLanes = @($lanes)
$allLanes += $closedLanes
foreach ($lane in $allLanes) {
    & (Join-Path $PSScriptRoot $lane)
}
powershell -File "%~dp0dev.ps1"
$Source = Join-Path $PSScriptRoot "launcher.cs"
Add-Type `
    -Path $Source
'''
    expected_references = {
        ("lib\\common.ps1", "dot_sources"),
        ("run.ps1", "executes"),
        ("lane-a.ps1", "dynamic_executes"),
        ("lane-b.ps1", "dynamic_executes"),
        ("lane-closed.ps1", "dynamic_executes"),
        ("%~dp0dev.ps1", "executes"),
        ("launcher.cs", "compiles"),
    }
    actual_references = set(extract_executable_script_references(sample))
    require(
        actual_references == expected_references,
        errors,
        "script reference parser self-test mismatch: "
        f"expected={sorted(expected_references)!r}, actual={sorted(actual_references)!r}",
    )
    require(
        DEV_COMMAND_RE.findall(sample) == ["real-command"],
        errors,
        "Invoke-DevCommand parser must ignore assertion needles",
    )


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    inventory_path = repo_root / "specs" / "script_inventory.toml"
    errors: list[str] = []
    validate_parser_contract(errors)

    require(inventory_path.is_file(), errors, f"missing script inventory: {inventory_path}")
    if not inventory_path.is_file():
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    try:
        expected_text = script_inventory_toml(repo_root)
    except ValueError as error:
        print("Script inventory validation failed:", file=sys.stderr)
        print(f"- {error}", file=sys.stderr)
        return 1
    actual_text = inventory_path.read_text(encoding="utf-8")
    require(
        actual_text == expected_text,
        errors,
        "script inventory is stale; run .\\scripts\\dev.cmd docs-generate",
    )

    inventory = load_toml(inventory_path)
    expected = collect_script_inventory(repo_root)
    records = inventory.get("script", [])
    require(inventory.get("schema") == EXPECTED_SCHEMA, errors, f"script inventory schema must be {EXPECTED_SCHEMA}")
    require(inventory.get("schema_version") == 1, errors, "script inventory schema_version must be 1")
    require(
        inventory.get("generated_by") == "tools/docs/generate_docs.py",
        errors,
        "script inventory generated_by is invalid",
    )
    require(isinstance(records, list), errors, "script inventory must contain [[script]] records")
    if not isinstance(records, list):
        records = []

    expected_paths = {str(record["path"]) for record in expected["scripts"]}
    actual_paths = {str(record.get("path", "")) for record in records}
    require(len(actual_paths) == len(records), errors, "script inventory contains duplicate paths")
    require(actual_paths == expected_paths, errors, "script inventory does not cover the executable script tree exactly")
    require(
        inventory.get("script_count") == len(records),
        errors,
        "script_count does not match [[script]] record count",
    )
    require(
        inventory.get("dev_command_count") == expected["dev_command_count"],
        errors,
        "dev_command_count does not match scripts/dev/commands.json",
    )
    require(
        not expected["missing_command_targets"],
        errors,
        "dev command targets are missing: " + ", ".join(expected["missing_command_targets"]),
    )
    require(
        not expected["duplicate_command_targets"],
        errors,
        "dev command targets are duplicated: " + ", ".join(expected["duplicate_command_targets"]),
    )
    require(
        not expected["catalog_errors"],
        errors,
        "dev command catalog is invalid: " + ", ".join(expected["catalog_errors"]),
    )
    require(
        inventory.get("catalog_error_count") == 0 and inventory.get("catalog_errors") == [],
        errors,
        "catalog_error_count and catalog_errors must be empty",
    )

    removable: list[str] = []
    public_without_entrypoint: list[str] = []
    internal_without_callers: list[str] = []
    unreachable: list[str] = []
    for record in records:
        path = str(record.get("path", ""))
        classification = str(record.get("classification", ""))
        entrypoint = str(record.get("entrypoint", ""))
        callers = record.get("callers", [])
        caller_evidence = record.get("caller_evidence", [])
        callees = record.get("callees", [])
        callee_evidence = record.get("callee_evidence", [])
        artifacts = record.get("generated_artifacts", [])
        require(
            classification in ALLOWED_CLASSIFICATIONS,
            errors,
            f"{path}: unsupported classification {classification!r}",
        )
        require(isinstance(callers, list), errors, f"{path}: callers must be an array")
        require(
            isinstance(caller_evidence, list),
            errors,
            f"{path}: caller_evidence must be an array",
        )
        require(isinstance(callees, list), errors, f"{path}: callees must be an array")
        require(
            isinstance(callee_evidence, list),
            errors,
            f"{path}: callee_evidence must be an array",
        )
        require(
            isinstance(artifacts, list),
            errors,
            f"{path}: generated_artifacts must be an array",
        )
        if classification == "removable":
            removable.append(path)
        elif classification == "internal" and not callers:
            internal_without_callers.append(path)
        elif classification == "public" and not has_supported_public_entrypoint(entrypoint):
            public_without_entrypoint.append(path)
        if record.get("reachable_from_public_root") is not True:
            unreachable.append(path)
        for caller in callers:
            require(
                caller == "user" or caller in actual_paths,
                errors,
                f"{path}: caller is not an inventoried script: {caller}",
            )
        for callee in callees:
            require(
                callee in actual_paths,
                errors,
                f"{path}: callee is not an inventoried script: {callee}",
            )
        require(
            all(value.lower() not in {"dist", "target"} for value in artifacts),
            errors,
            f"{path}: generated_artifacts contains an ambiguous bare hint",
        )

    require(not removable, errors, "uncalled scripts must be removed or connected: " + ", ".join(removable))
    require(
        not internal_without_callers,
        errors,
        "internal scripts must have callers: " + ", ".join(internal_without_callers),
    )
    require(
        not public_without_entrypoint,
        errors,
        "public scripts must use dev.cmd or a Rust CLI entrypoint: "
        + ", ".join(public_without_entrypoint),
    )
    require(
        inventory.get("unused_script_count") == len(removable) == 0,
        errors,
        "unused_script_count must be zero",
    )
    require(not unreachable, errors, "scripts are unreachable from dev.cmd: " + ", ".join(unreachable))
    require(
        inventory.get("unreachable_count") == len(unreachable) == 0,
        errors,
        "unreachable_count must be zero",
    )
    require(
        inventory.get("public_without_entrypoint_count")
        == len(public_without_entrypoint)
        == 0,
        errors,
        "public_without_entrypoint_count must be zero",
    )

    if errors:
        print("Script inventory validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    public_count = sum(record.get("classification") == "public" for record in records)
    internal_count = sum(record.get("classification") == "internal" for record in records)
    print("Script inventory check")
    print(f"  scripts: {len(records)}")
    print(f"  public: {public_count}")
    print(f"  internal: {internal_count}")
    print("  uncalled: 0")
    print("  public_entrypoints: valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
