"""Collect and serialize the repository script call graph."""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any


GENERATED_TOML_NOTICE = """# DO NOT EDIT.
# Generated from scripts/ and scripts/dev/commands.json by tools/docs/generate_docs.py.

"""
SCRIPT_FILE_SUFFIXES = {".cmd", ".cs", ".ps1"}
ARTIFACT_HINT_RE = re.compile(
    r"(?i)(?<![$A-Za-z0-9_])(?:\.runtime|\.reference|target|dist|docs[\\/]book|"
    r"docs[\\/]src[\\/]generated|tools[\\/]docs[\\/]generated-docs\.manifest\.json|"
    r"reports[\\/]latest)(?:[\\/][A-Za-z0-9_.${}()\-]+)*"
)
PYTHON_SCRIPT_RE = re.compile(r"tools[\/](?:docs|reporting)[\/][A-Za-z0-9_.-]+\.py")
DEV_COMMAND_RE = re.compile(
    r"^\s*Invoke-DevCommand\s+-Command\s+['\"]([^'\"]+)['\"]", re.MULTILINE
)
QUOTED_SCRIPT_REFERENCE_RE = re.compile(
    r"['\"]([^'\"\r\n]+\.(?:cmd|cs|ps1))['\"]", re.IGNORECASE
)
SCRIPT_PATH_ASSIGNMENT_RE = re.compile(
    r"^\s*\$([A-Za-z_][A-Za-z0-9_]*)\s*=.*?['\"]([^'\"\r\n]+\.(?:cmd|cs|ps1))['\"]",
    re.IGNORECASE | re.MULTILINE,
)
SCRIPT_ARRAY_ASSIGNMENT_RE = re.compile(
    r"^\s*\$([A-Za-z_][A-Za-z0-9_]*)\s*=\s*@\((.*?)^\s*\)",
    re.DOTALL | re.MULTILINE,
)
CARGO_COMMAND_RE = re.compile(r"cargo\s+(?:build|clippy|fmt|run|test)[^\r\n`|&;]*")


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(read_text(path))


def repo_path(path: Path, repo_root: Path) -> str:
    return str(path.relative_to(repo_root)).replace("\\", "/")


def normalized_path(value: str) -> str:
    return value.replace("\\", "/").strip("`'\",);")


def extract_path_hints(pattern: re.Pattern[str], text: str) -> list[str]:
    normalized = text.replace("\\", "/")
    return sorted({normalized_path(match.group(0)) for match in pattern.finditer(normalized)})


def extract_call_hints(text: str) -> list[str]:
    calls: set[str] = set()
    for match in DEV_COMMAND_RE.finditer(text):
        calls.add(f"dev:{match.group(1)}")
    for match in CARGO_COMMAND_RE.finditer(text):
        calls.add(" ".join(match.group(0).split()))
    for match in PYTHON_SCRIPT_RE.finditer(text.replace("\\", "/")):
        calls.add(normalized_path(match.group(0)))
    if re.search(r"eplus-rs(?:\.exe)?\s+run", text, flags=re.IGNORECASE):
        calls.add("eplus-rs run")
    if "ConvertInputFormat" in text:
        calls.add("EnergyPlus ConvertInputFormat")
    if "energyplus.exe" in text.lower():
        calls.add("EnergyPlus executable")
    return sorted(calls)


def exit_contract(path: Path, text: str) -> str:
    suffix = path.suffix.lower()
    if suffix == ".cs":
        return "compiled helper; exit code set by compiled process"
    if path.name.lower() == "dev.cmd":
        return "propagates PowerShell ERRORLEVEL"
    explicit_exits = sorted(
        {line.strip() for line in text.splitlines() if re.search(r"^\s*exit(?:\s|/b|$)", line)}
    )
    if explicit_exits:
        return "explicit exit: " + "; ".join(explicit_exits)
    if "$LASTEXITCODE" in text:
        return "nonzero when checked child process fails"
    if "throw " in text or "throw\"" in text:
        return "nonzero via PowerShell throw"
    return "PowerShell success unless an uncaught error occurs"


def script_category(relative_path: str, command: dict[str, Any] | None) -> str:
    if command is not None:
        group = str(command.get("group", ""))
        if group == "launcher":
            return "launcher"
        if group in {"setup", "quality", "smoke", "compare", "conformance", "release"}:
            return group
    if (
        relative_path.startswith("scripts/quality/strict-no-false-conformance/")
        or relative_path == "scripts/release/select-release-assets.ps1"
    ):
        return "internal"
    parts = relative_path.split("/")
    if len(parts) < 2:
        return "internal"
    folder = parts[1]
    if folder == "launcher":
        return "launcher"
    if folder in {"setup", "quality", "smoke", "compare", "conformance", "release"}:
        return folder
    return "internal"


def script_source_files(repo_root: Path) -> list[Path]:
    return sorted(
        path
        for path in (repo_root / "scripts").rglob("*")
        if path.is_file() and path.suffix.lower() in SCRIPT_FILE_SUFFIXES
    )


def extract_executable_script_references(text: str) -> list[tuple[str, str]]:
    references: list[tuple[str, str]] = []
    assignment_spans: set[tuple[int, int]] = set()
    for match in SCRIPT_PATH_ASSIGNMENT_RE.finditer(text):
        variable = match.group(1)
        reference = match.group(2)
        assignment_spans.add(match.span(2))
        escaped = re.escape(variable)
        kind = ""
        if re.search(rf"^\s*&[^\r\n]*\${escaped}\b", text, flags=re.MULTILINE):
            kind = "executes"
        elif re.search(rf"-File\s+\${escaped}\b", text, flags=re.IGNORECASE):
            kind = "executes"
        elif re.search(rf"^\s*\.\s+\${escaped}\b", text, flags=re.MULTILINE):
            kind = "dot_sources"
        elif reference.lower().endswith(".cs") and re.search(
            rf"Add-Type[\s\S]*?-Path\s+\${escaped}\b", text, flags=re.IGNORECASE
        ):
            kind = "compiles"
        if kind:
            references.append((reference, kind))

    for line_match in re.finditer(r"^.*$", text, flags=re.MULTILINE):
        line = line_match.group(0)
        stripped = line.lstrip()
        if stripped.startswith("#") or "Assert-ContainsLiteral" in line:
            continue
        direct_kind = ""
        if re.match(r"^[.&]\s", stripped):
            direct_kind = "dot_sources" if stripped.startswith(".") else "executes"
        elif re.search(r"(?:^|\s)-File\s", line, flags=re.IGNORECASE):
            direct_kind = "executes"
        if not direct_kind:
            continue
        for match in QUOTED_SCRIPT_REFERENCE_RE.finditer(line):
            absolute_span = (
                line_match.start() + match.start(1),
                line_match.start() + match.end(1),
            )
            if absolute_span not in assignment_spans:
                references.append((match.group(1), direct_kind))

    array_assignments = list(SCRIPT_ARRAY_ASSIGNMENT_RE.finditer(text))
    executable_arrays: set[str] = set()
    for runner in re.finditer(
        r"foreach\s*\(\s*\$(?P<item>[A-Za-z_][A-Za-z0-9_]*)\s+in\s+"
        r"\$(?P<array>[A-Za-z_][A-Za-z0-9_]*)\s*\)",
        text,
        flags=re.IGNORECASE,
    ):
        item = runner.group("item")
        if re.search(
            rf"&\s*\(\s*Join-Path[^\r\n]*\${re.escape(item)}\b",
            text[runner.end() :],
            flags=re.IGNORECASE,
        ):
            executable_arrays.add(runner.group("array").lower())

    # Follow simple PowerShell array composition into a dynamically executed
    # runner array. This keeps conditional active/closed lane registries in the
    # call graph without treating every quoted script path as executable.
    array_links = [
        (match.group("destination").lower(), match.group("source").lower())
        for match in re.finditer(
            r"(?im)^\s*\$(?P<destination>[A-Za-z_][A-Za-z0-9_]*)\s*(?:=|\+=)\s*"
            r"(?:@\(\s*)?\$(?P<source>[A-Za-z_][A-Za-z0-9_]*)\s*(?:\)\s*)?(?:#.*)?$",
            text,
        )
    ]
    changed = True
    while changed:
        changed = False
        for destination, source in array_links:
            if destination in executable_arrays and source not in executable_arrays:
                executable_arrays.add(source)
                changed = True

    for match in array_assignments:
        if match.group(1).lower() not in executable_arrays:
            continue
        body = match.group(2)
        for reference in QUOTED_SCRIPT_REFERENCE_RE.finditer(body):
            references.append((reference.group(1), "dynamic_executes"))
    return references


def collect_script_inventory(repo_root: Path) -> dict[str, Any]:
    catalog = load_json(repo_root / "scripts" / "dev" / "commands.json")
    commands = list(catalog.get("commands", []))
    groups = {str(group) for group in catalog.get("groups", [])}
    aliases = {str(name): str(target) for name, target in dict(catalog.get("aliases", {})).items()}
    command_by_script: dict[str, dict[str, Any]] = {}
    missing_command_targets: list[str] = []
    duplicate_command_targets: list[str] = []
    catalog_errors: list[str] = []
    command_names: set[str] = set()
    for entry in commands:
        name = str(entry.get("name", "")).strip()
        name_key = name.lower()
        if not name:
            catalog_errors.append("command name must not be empty")
        elif name_key in command_names:
            catalog_errors.append(f"duplicate command name: {name}")
        command_names.add(name_key)
        group = str(entry.get("group", "")).strip()
        if group not in groups:
            catalog_errors.append(f"{name}: command group is not declared: {group}")
        command_path = normalized_path(str(entry.get("path", "")))
        if (
            not command_path.lower().endswith(".ps1")
            or command_path.startswith("/")
            or ":" in command_path
            or ".." in command_path.split("/")
        ):
            catalog_errors.append(f"{name}: invalid command target path: {command_path}")
        relative = "scripts/" + command_path
        key = relative.lower()
        if key in command_by_script:
            duplicate_command_targets.append(relative)
        command_by_script[key] = entry
        if not (repo_root / relative).is_file():
            missing_command_targets.append(f"{name} -> {relative}")
    for alias, target in aliases.items():
        if alias.lower() in command_names:
            catalog_errors.append(f"alias conflicts with command name: {alias}")
        if target.lower() not in command_names:
            catalog_errors.append(f"alias target is not a command: {alias} -> {target}")

    files = script_source_files(repo_root)
    relative_files = {repo_path(path, repo_root): path for path in files}
    text_by_script = {relative: read_text(path) for relative, path in relative_files.items()}
    callers_by_script: dict[str, set[str]] = {relative: set() for relative in relative_files}
    callees_by_script: dict[str, set[str]] = {relative: set() for relative in relative_files}
    edge_kinds: dict[tuple[str, str], set[str]] = {}

    def add_edge(caller: str, target: str, kind: str) -> None:
        if caller == target or caller not in callees_by_script or target not in callers_by_script:
            return
        callers_by_script[target].add(caller)
        callees_by_script[caller].add(target)
        edge_kinds.setdefault((caller, target), set()).add(kind)

    targets_by_basename: dict[str, list[str]] = {}
    for relative in relative_files:
        targets_by_basename.setdefault(Path(relative).name.lower(), []).append(relative)
    duplicate_basenames = {
        basename: targets for basename, targets in targets_by_basename.items() if len(targets) > 1
    }
    if duplicate_basenames:
        details = "; ".join(
            f"{basename}: {', '.join(targets)}"
            for basename, targets in sorted(duplicate_basenames.items())
        )
        raise ValueError(f"script basenames must be unique for caller resolution: {details}")

    for caller, text in text_by_script.items():
        for reference, kind in extract_executable_script_references(text):
            normalized = normalized_path(reference).lower()
            matches = [
                targets[0]
                for basename, targets in targets_by_basename.items()
                if normalized.endswith(basename)
            ]
            if not matches:
                continue
            longest = max(len(Path(target).name) for target in matches)
            best_matches = [target for target in matches if len(Path(target).name) == longest]
            if len(best_matches) == 1:
                add_edge(caller, best_matches[0], kind)

    for caller, text in text_by_script.items():
        for match in DEV_COMMAND_RE.finditer(text):
            command_name = match.group(1)
            command = next(
                (entry for entry in commands if str(entry.get("name", "")) == command_name),
                None,
            )
            if command is not None:
                target = "scripts/" + normalized_path(str(command.get("path", "")))
                add_edge(caller, target, "invoke_dev_command")
    for entry in commands:
        target = "scripts/" + normalized_path(str(entry.get("path", "")))
        add_edge("scripts/dev.ps1", target, "dev_catalog")

    reachable: set[str] = set()
    pending = ["scripts/dev.cmd"]
    while pending:
        current = pending.pop()
        if current in reachable or current not in relative_files:
            continue
        reachable.add(current)
        pending.extend(callees_by_script[current] - reachable)

    records: list[dict[str, Any]] = []
    for relative, path in relative_files.items():
        command = command_by_script.get(relative.lower())
        command_name = str(command.get("name", "")) if command else ""
        callers = callers_by_script[relative]
        entrypoint = ""
        classification = "internal"
        if relative == "scripts/dev.cmd":
            entrypoint = "dev.cmd"
            classification = "public"
            callers.add("user")
        elif relative == "scripts/dev.ps1":
            entrypoint = "dev.cmd"
            classification = "public"
            callers.add("user")
        elif command is not None:
            entrypoint = f"dev:{command_name}"
            classification = "public"
        elif relative not in reachable:
            classification = "removable"

        text = text_by_script[relative]
        calls = set(extract_call_hints(text))
        calls.update(f"script:{target}" for target in callees_by_script[relative])
        caller_evidence = sorted(
            f"{caller}::{kind}"
            for caller in callers
            for kind in (
                {"user_entrypoint"}
                if caller == "user"
                else edge_kinds.get((caller, relative), {"unresolved"})
            )
        )
        callee_evidence = sorted(
            f"{target}::{kind}"
            for target in callees_by_script[relative]
            for kind in edge_kinds.get((relative, target), {"unresolved"})
        )
        artifact_hints = [
            value
            for value in extract_path_hints(ARTIFACT_HINT_RE, text)
            if value.lower() not in {"dist", "target"}
        ]
        records.append(
            {
                "path": relative,
                "kind": path.suffix.lower().lstrip("."),
                "category": script_category(relative, command),
                "classification": classification,
                "command": command_name,
                "entrypoint": entrypoint,
                "callers": sorted(callers),
                "caller_evidence": caller_evidence,
                "callees": sorted(callees_by_script[relative]),
                "callee_evidence": callee_evidence,
                "calls": sorted(calls),
                "generated_artifacts": artifact_hints,
                "reachable_from_public_root": relative in reachable,
                "exit_contract": exit_contract(path, text),
            }
        )

    unused = [record["path"] for record in records if record["classification"] == "removable"]
    public_without_entrypoint = [
        record["path"]
        for record in records
        if record["classification"] == "public" and not record["entrypoint"]
    ]
    return {
        "schema": "rusted-energyplus.script-inventory.v1",
        "schema_version": 1,
        "generated_by": "tools/docs/generate_docs.py",
        "script_count": len(records),
        "dev_command_count": len(commands),
        "unused_script_count": len(unused),
        "unreachable_count": len(unused),
        "public_without_entrypoint_count": len(public_without_entrypoint),
        "unused_scripts": unused,
        "public_without_entrypoint": public_without_entrypoint,
        "missing_command_targets": missing_command_targets,
        "duplicate_command_targets": duplicate_command_targets,
        "catalog_errors": catalog_errors,
        "scripts": records,
    }


def toml_string(value: str) -> str:
    return json.dumps(value, ensure_ascii=False)


def toml_string_array(values: list[str]) -> str:
    return "[" + ", ".join(toml_string(value) for value in values) + "]"


def script_inventory_toml(repo_root: Path) -> str:
    inventory = collect_script_inventory(repo_root)
    if (
        inventory["unused_script_count"]
        or inventory["public_without_entrypoint_count"]
        or inventory["missing_command_targets"]
        or inventory["duplicate_command_targets"]
        or inventory["catalog_errors"]
    ):
        raise ValueError(
            "script inventory contract failed: "
            f"unused={inventory['unused_script_count']}, "
            f"public_without_entrypoint={inventory['public_without_entrypoint_count']}, "
            f"missing_targets={len(inventory['missing_command_targets'])}, "
            f"duplicate_targets={len(inventory['duplicate_command_targets'])}, "
            f"catalog_errors={len(inventory['catalog_errors'])}"
        )
    lines = [
        GENERATED_TOML_NOTICE.rstrip(),
        f"schema = {toml_string(str(inventory['schema']))}",
        f"schema_version = {inventory['schema_version']}",
        f"generated_by = {toml_string(str(inventory['generated_by']))}",
        f"script_count = {inventory['script_count']}",
        f"dev_command_count = {inventory['dev_command_count']}",
        f"unused_script_count = {inventory['unused_script_count']}",
        f"unreachable_count = {inventory['unreachable_count']}",
        f"public_without_entrypoint_count = {inventory['public_without_entrypoint_count']}",
        f"catalog_error_count = {len(inventory['catalog_errors'])}",
        f"unused_scripts = {toml_string_array(inventory['unused_scripts'])}",
        "public_without_entrypoint = "
        + toml_string_array(inventory["public_without_entrypoint"]),
        f"catalog_errors = {toml_string_array(inventory['catalog_errors'])}",
    ]
    for record in inventory["scripts"]:
        lines.extend(
            [
                "",
                "[[script]]",
                f"path = {toml_string(record['path'])}",
                f"kind = {toml_string(record['kind'])}",
                f"category = {toml_string(record['category'])}",
                f"classification = {toml_string(record['classification'])}",
                f"command = {toml_string(record['command'])}",
                f"entrypoint = {toml_string(record['entrypoint'])}",
                f"callers = {toml_string_array(record['callers'])}",
                f"caller_evidence = {toml_string_array(record['caller_evidence'])}",
                f"callees = {toml_string_array(record['callees'])}",
                f"callee_evidence = {toml_string_array(record['callee_evidence'])}",
                f"calls = {toml_string_array(record['calls'])}",
                "generated_artifacts = "
                + toml_string_array(record["generated_artifacts"]),
                "reachable_from_public_root = "
                + ("true" if record["reachable_from_public_root"] else "false"),
                f"exit_contract = {toml_string(record['exit_contract'])}",
            ]
        )
    return "\n".join(lines) + "\n"
