#!/usr/bin/env python3
"""Validate the one-hypothesis/one-lane diagnostic-probe lifecycle contract."""

from __future__ import annotations

import argparse
import json
import posixpath
import re
import sys
import tomllib
from pathlib import Path, PurePosixPath
from typing import Any, Iterable


EXPECTED_SCHEMA = "rusted-energyplus.diagnostic-probe-ledger.v1"
REFERENCE_VERSION = "26.1.0"
ALLOWED_HYPOTHESIS_STATUS = {"unresolved", "resolved", "closed"}
SCALAR_STATE_PATTERN = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")
SUITE_EXCLUSIONS = {
    "scripts/compare/official-dynamic-heat-balance-probe-suite.ps1",
    "scripts/compare/official-dynamic-heat-balance-probe-summary.ps1",
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate the diagnostic-probe lifecycle ledger.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--self-test", action="store_true")
    return parser.parse_args()


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def require(condition: bool, errors: list[str], message: str) -> None:
    if not condition:
        errors.append(message)


def records(spec: dict[str, Any], key: str, errors: list[str]) -> list[dict[str, Any]]:
    value = spec.get(key, [])
    if not isinstance(value, list) or any(not isinstance(item, dict) for item in value):
        errors.append(f"{key} must be an array of tables")
        return []
    return value


def text_field(record: dict[str, Any], key: str) -> str:
    value = record.get(key, "")
    return value.strip() if isinstance(value, str) else ""


def positive_integer_field(record: dict[str, Any], key: str) -> int | None:
    value = record.get(key)
    if isinstance(value, bool) or not isinstance(value, int) or value <= 0:
        return None
    return value


def require_fields(record: dict[str, Any], fields: Iterable[str], prefix: str, errors: list[str]) -> None:
    for field in fields:
        require(bool(text_field(record, field)), errors, f"{prefix}: {field} must not be empty")


def require_unique(values: list[str], label: str, errors: list[str]) -> None:
    seen: set[str] = set()
    duplicates: set[str] = set()
    for value in values:
        if value in seen:
            duplicates.add(value)
        seen.add(value)
    require(not duplicates, errors, f"duplicate {label}: {sorted(duplicates)}")


def normalized_repo_path(value: str) -> str:
    return posixpath.normpath(value.replace("\\", "/"))


def is_safe_repo_path(value: str) -> bool:
    normalized = normalized_repo_path(value)
    path = PurePosixPath(normalized)
    return bool(value) and not path.is_absolute() and ".." not in path.parts and normalized != "."


def validate_evidence_ref(repo_root: Path, value: str, label: str, errors: list[str]) -> None:
    parts = value.split("#")
    require(len(parts) == 2 and all(parts), errors, f"{label} must be a path#anchor reference")
    if len(parts) != 2:
        return
    path_value, anchor = parts
    require(is_safe_repo_path(path_value), errors, f"{label} path must be safe and repository-relative")
    if not is_safe_repo_path(path_value):
        return
    path = repo_root / normalized_repo_path(path_value)
    require(path.is_file(), errors, f"{label} file not found: {path_value}")
    if path.is_file():
        source = path.read_text(encoding="utf-8", errors="replace")
        require(anchor in source, errors, f"{label} anchor not found: {anchor}")


def resolve_suite_lane(suite_script: str, lane: str) -> str:
    parent = posixpath.dirname(normalized_repo_path(suite_script))
    return normalized_repo_path(posixpath.join(parent, lane.replace("\\", "/")))


def set_equality(
    actual: set[str],
    declared: set[str],
    label: str,
    errors: list[str],
) -> None:
    missing = sorted(actual - declared)
    extra = sorted(declared - actual)
    require(
        not missing and not extra,
        errors,
        f"{label} mismatch: unclassified/missing={missing}; stale/extra={extra}",
    )


def parse_rust_diagnostic_selectors(source: str, errors: list[str]) -> set[tuple[str, str]]:
    match = re.search(
        r"diagnostic_heat_balance_selectors!\s*\{(?P<body>.*?)^\}",
        source,
        flags=re.DOTALL | re.MULTILINE,
    )
    if match is None:
        errors.append("Rust diagnostic selector macro invocation was not found")
        return set()
    entries = re.findall(
        r'^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=>\s*"([^"]+)",?\s*$',
        match.group("body"),
        flags=re.MULTILINE,
    )
    baseline = [entry for entry in entries if entry[0] == "SimplifiedAnalytical"]
    require(len(baseline) == 1, errors, "Rust diagnostic selector baseline SimplifiedAnalytical must occur once")
    probes = {entry for entry in entries if entry[0].endswith("Probe")}
    unexpected = [entry[0] for entry in entries if entry[0] != "SimplifiedAnalytical" and not entry[0].endswith("Probe")]
    require(not unexpected, errors, f"Rust diagnostic selector has non-probe variants: {unexpected}")
    require(bool(probes), errors, "Rust diagnostic selector macro must contain at least one *Probe selector")
    return probes


def discover_probe_wrappers(repo_root: Path) -> set[str]:
    wrappers: set[str] = set()
    compare_root = repo_root / "scripts" / "compare"
    if compare_root.is_dir():
        for path in compare_root.rglob("*.ps1"):
            relative = path.relative_to(repo_root).as_posix()
            if "probe" in path.name.lower() and relative not in SUITE_EXCLUSIONS:
                wrappers.add(relative)
    internal_root = repo_root / "scripts" / "internal" / "probes"
    if internal_root.is_dir():
        wrappers.update(path.relative_to(repo_root).as_posix() for path in internal_root.rglob("*.ps1"))
    return wrappers


def parse_suite_array(source: str, variable: str, errors: list[str]) -> list[str]:
    match = re.search(rf"\${re.escape(variable)}\s*=\s*@\((.*?)\)", source, flags=re.DOTALL)
    if match is None:
        errors.append(f"probe suite is missing ${variable} array")
        return []
    body = match.group(1)
    stripped = re.sub(r"(?m)#.*$", "", body)
    remainder = re.sub(r'"[^"]+"\s*,?', "", stripped)
    require(not remainder.strip(), errors, f"probe suite ${variable} must contain only quoted lane paths")
    return re.findall(r'"([^"]+)"', stripped)


def parse_powershell_hashtable(
    source: str,
    variable: str,
    errors: list[str],
) -> dict[str, Any]:
    match = re.search(
        rf"(?ms)^\s*\${re.escape(variable)}\s*=\s*@\{{(?P<body>.*?)^\s*\}}\s*$",
        source,
    )
    if match is None:
        errors.append(f"active probe wrapper is missing ${variable} hashtable")
        return {}
    result: dict[str, Any] = {}
    for raw_line in match.group("body").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        assignment = re.fullmatch(
            r"([A-Za-z_][A-Za-z0-9_]*)\s*=\s*"
            r'(?:"([^"]*)"|\'([^\']*)\'|(-?[0-9]+)|\$(true|false)|\$([A-Za-z_][A-Za-z0-9_]*))',
            line,
            flags=re.IGNORECASE,
        )
        if assignment is None:
            errors.append(f"active probe wrapper has unsupported ${variable} entry: {line}")
            continue
        key = assignment.group(1)
        if key in result:
            errors.append(f"active probe wrapper duplicates ${variable} key: {key}")
            continue
        if assignment.group(2) is not None:
            value: Any = assignment.group(2)
        elif assignment.group(3) is not None:
            value = assignment.group(3)
        elif assignment.group(4) is not None:
            value = int(assignment.group(4))
        elif assignment.group(5) is not None:
            value = assignment.group(5).lower() == "true"
        else:
            value = f"${assignment.group(6)}"
        result[key] = value
    return result


def parse_powershell_string_assignment(
    source: str,
    variable: str,
    errors: list[str],
) -> str:
    matches = re.findall(
        rf'(?m)^\s*\${re.escape(variable)}\s*=\s*"([^"]*)"\s*(?:#.*)?$',
        source,
    )
    require(len(matches) == 1, errors, f"active probe wrapper must assign ${variable} exactly once")
    return matches[0] if len(matches) == 1 else ""


def parse_powershell_number_assignment(
    source: str,
    variable: str,
    errors: list[str],
) -> float | None:
    matches = re.findall(
        rf"(?im)^\s*\${re.escape(variable)}\s*=\s*"
        r"([+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:e[+-]?[0-9]+)?)\s*(?:#.*)?$",
        source,
    )
    require(len(matches) == 1, errors, f"active probe wrapper must assign ${variable} exactly once")
    return float(matches[0]) if len(matches) == 1 else None


def command_catalog(repo_root: Path, errors: list[str]) -> dict[str, list[str]]:
    path = repo_root / "scripts" / "dev" / "commands.json"
    if not path.is_file():
        errors.append("missing command catalog: scripts/dev/commands.json")
        return {}
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        errors.append(f"could not read command catalog: {exc}")
        return {}
    result: dict[str, list[str]] = {}
    for entry in data.get("commands", []):
        if not isinstance(entry, dict):
            continue
        name = str(entry.get("name", "")).strip()
        path_value = str(entry.get("path", "")).strip()
        result.setdefault(name, []).append(normalized_repo_path(posixpath.join("scripts", path_value)))
    return result


def algorithm_index(repo_root: Path, errors: list[str]) -> dict[str, dict[str, Any]]:
    path = repo_root / "specs" / "algorithm_ledger.toml"
    if not path.is_file():
        errors.append("missing algorithm ledger: specs/algorithm_ledger.toml")
        return {}
    algorithms = load_toml(path).get("algorithm", [])
    if not isinstance(algorithms, list):
        errors.append("algorithm ledger [[algorithm]] records are invalid")
        return {}
    result: dict[str, dict[str, Any]] = {}
    for algorithm in algorithms:
        if not isinstance(algorithm, dict):
            continue
        algorithm_id = text_field(algorithm, "id")
        if algorithm_id in result:
            errors.append(f"duplicate algorithm id while validating probe ledger: {algorithm_id}")
        result[algorithm_id] = algorithm
    return result


def source_routine_region(source: str, routine: str) -> str | None:
    definition = re.search(
        rf"(?m)^[A-Za-z_][A-Za-z0-9_:<>,*&]*(?:[ \t]+[A-Za-z_][A-Za-z0-9_:<>,*&]*)*[ \t]+{re.escape(routine)}\s*\(",
        source,
    )
    if definition is None:
        return None
    next_definition = re.search(
        r"(?m)^[A-Za-z_][A-Za-z0-9_:<>,*&]*(?:[ \t]+[A-Za-z_][A-Za-z0-9_:<>,*&]*)*[ \t]+[A-Za-z_][A-Za-z0-9_]*\s*\(",
        source[definition.end() :],
    )
    end = len(source) if next_definition is None else definition.end() + next_definition.start()
    return source[definition.start() : end]


def validate_hypothesis_sources(
    repo_root: Path,
    source_map_path: str,
    hypotheses: list[dict[str, Any]],
    algorithms: dict[str, dict[str, Any]],
    errors: list[str],
) -> None:
    reference_root = repo_root / ".reference" / "energyplus-src" / REFERENCE_VERSION
    require(reference_root.is_dir(), errors, f"missing EnergyPlus reference source root: {reference_root}")
    source_map = repo_root / normalized_repo_path(source_map_path)
    source_map_text = source_map.read_text(encoding="utf-8", errors="replace") if source_map.is_file() else ""
    ledger_hypothesis_ids = {text_field(item, "id") for item in hypotheses}
    source_map_hypothesis_ids = set(
        re.findall(r"diagnostic-probe-hypothesis:([A-Za-z0-9][A-Za-z0-9_-]*)", source_map_text)
    )
    set_equality(
        source_map_hypothesis_ids,
        ledger_hypothesis_ids,
        "source-map hypothesis anchor set",
        errors,
    )

    for hypothesis in hypotheses:
        hypothesis_id = text_field(hypothesis, "id") or "<missing-hypothesis-id>"
        prefix = f"hypothesis {hypothesis_id}"
        algorithm_id = text_field(hypothesis, "algorithm_id")
        owner_routine_id = text_field(hypothesis, "owner_routine_id")
        source_file = text_field(hypothesis, "source_file")
        source_routine = text_field(hypothesis, "source_routine")
        source_state = text_field(hypothesis, "source_state")
        anchor = text_field(hypothesis, "source_map_anchor")

        require(SCALAR_STATE_PATTERN.fullmatch(source_state) is not None, errors, f"{prefix}: source_state must be one scalar identifier")
        require(anchor == f"diagnostic-probe-hypothesis:{hypothesis_id}", errors, f"{prefix}: source_map_anchor must use the hypothesis id")
        require(bool(anchor) and anchor in source_map_text, errors, f"{prefix}: source-map anchor not found: {anchor}")

        algorithm = algorithms.get(algorithm_id)
        require(algorithm is not None, errors, f"{prefix}: algorithm_id is not in algorithm ledger: {algorithm_id}")
        if algorithm is not None:
            routines = algorithm.get("routine", {})
            owner = routines.get(owner_routine_id) if isinstance(routines, dict) else None
            require(isinstance(owner, dict), errors, f"{prefix}: owner_routine_id is not owned by algorithm {algorithm_id}: {owner_routine_id}")
            energyplus_sources = {normalized_repo_path(str(item)) for item in algorithm.get("energyplus_source", [])}
            require(normalized_repo_path(source_file) in energyplus_sources, errors, f"{prefix}: source_file is not mapped by algorithm {algorithm_id}")
            if isinstance(owner, dict):
                require(text_field(owner, "source_file") == source_file, errors, f"{prefix}: source_file must match owner routine")
                require(text_field(owner, "source_routine") == source_routine, errors, f"{prefix}: source_routine must match owner routine")

        source_is_safe = is_safe_repo_path(source_file) and normalized_repo_path(source_file).startswith("src/EnergyPlus/")
        require(source_is_safe, errors, f"{prefix}: source_file must be a safe path below src/EnergyPlus")
        if not source_is_safe or not reference_root.is_dir():
            continue
        source_path = reference_root / normalized_repo_path(source_file)
        require(source_path.is_file(), errors, f"{prefix}: locked EnergyPlus source file not found: {source_file}")
        if not source_path.is_file():
            continue
        source_text = source_path.read_text(encoding="utf-8", errors="replace")
        region = source_routine_region(source_text, source_routine)
        require(region is not None, errors, f"{prefix}: source routine not found in locked EnergyPlus source: {source_routine}")
        require(region is not None and re.search(rf"\b{re.escape(source_state)}\b", region) is not None, errors, f"{prefix}: source_state not found in source routine: {source_state}")


def validate_probe_ledger(repo_root: Path, spec: dict[str, Any]) -> list[str]:
    """Return every diagnostic-probe lifecycle contract error."""

    errors: list[str] = []
    require(spec.get("schema") == EXPECTED_SCHEMA, errors, f"schema must be {EXPECTED_SCHEMA}")
    source_map_path = text_field(spec, "source_map")
    suite_script = text_field(spec, "suite_script")
    rust_selector_source = text_field(spec, "rust_selector_source")
    require_fields(spec, ["source_map", "historical_result_evidence", "suite_script", "rust_selector_source"], "ledger", errors)
    for label, value in [("source_map", source_map_path), ("suite_script", suite_script), ("rust_selector_source", rust_selector_source)]:
        require(is_safe_repo_path(value), errors, f"ledger {label} must be a safe repository-relative path")
        require(bool(value) and (repo_root / normalized_repo_path(value)).is_file(), errors, f"ledger {label} file not found: {value}")
    historical_result_evidence = text_field(spec, "historical_result_evidence")
    validate_evidence_ref(repo_root, historical_result_evidence, "historical_result_evidence", errors)

    lifecycle = spec.get("lifecycle", {})
    require(isinstance(lifecycle, dict), errors, "ledger lifecycle table is required")
    if isinstance(lifecycle, dict):
        require(lifecycle.get("probe_unit") == "full-executable-lane", errors, "lifecycle probe_unit must be full-executable-lane")
        require(lifecycle.get("active_hypothesis_status") == "unresolved", errors, "lifecycle active_hypothesis_status must be unresolved")
        require(lifecycle.get("closed_replay_switch") == "IncludeClosed", errors, "lifecycle closed_replay_switch must be IncludeClosed")
        require(lifecycle.get("default_suite_policy") == "active-only", errors, "lifecycle default_suite_policy must be active-only")

    hypotheses = records(spec, "hypothesis", errors)
    active_probes = records(spec, "active_probe", errors)
    closed_selectors = records(spec, "closed_selector", errors)
    closed_scripts = records(spec, "closed_script", errors)
    require(bool(hypotheses), errors, "ledger must contain at least one [[hypothesis]]")

    hypothesis_fields = (
        "id", "algorithm_id", "owner_routine_id", "source_file", "source_routine",
        "source_state", "statement", "expected_observation", "status", "source_map_anchor",
    )
    for item in hypotheses:
        prefix = f"hypothesis {text_field(item, 'id') or '<missing>'}"
        require_fields(item, hypothesis_fields, prefix, errors)
        status = text_field(item, "status")
        require(status in ALLOWED_HYPOTHESIS_STATUS, errors, f"{prefix}: unsupported status: {status}")
        if status == "unresolved":
            require(not text_field(item, "resolution"), errors, f"{prefix}: unresolved hypothesis cannot have a resolution")
            require(not text_field(item, "evidence_ref"), errors, f"{prefix}: unresolved hypothesis cannot have result evidence")
        elif status in ALLOWED_HYPOTHESIS_STATUS:
            require(bool(text_field(item, "resolution")), errors, f"{prefix}: {status} hypothesis requires a resolution")
            require(bool(text_field(item, "evidence_ref")), errors, f"{prefix}: {status} hypothesis requires result evidence")

    hypothesis_ids = [text_field(item, "id") for item in hypotheses]
    require_unique(hypothesis_ids, "hypothesis id", errors)
    hypothesis_by_id = {text_field(item, "id"): item for item in hypotheses}

    active_fields = (
        "id", "hypothesis_id", "kind", "command", "script", "suite_lane", "selector_kind",
        "selector", "cli_name", "reference_lane", "source_state", "single_change",
        "ctf_seed_policy", "ctf_initial_history_policy", "observation_json_array",
        "observation_key", "observation_oracle_field", "observation_rust_field",
        "observation_delta_field", "report_path",
    )
    for item in active_probes:
        probe_id = text_field(item, "id") or "<missing>"
        prefix = f"active probe {probe_id}"
        require_fields(item, active_fields, prefix, errors)
        require(text_field(item, "kind") == "observation", errors, f"{prefix}: kind must be observation")
        require(text_field(item, "selector_kind") == "compatibility-observation", errors, f"{prefix}: selector_kind must be compatibility-observation")
        require(text_field(item, "reference_lane") == "compatibility-source-order", errors, f"{prefix}: reference_lane must be compatibility-source-order")
        warmup_minimum_days = positive_integer_field(item, "warmup_minimum_days")
        surface_iterations = positive_integer_field(item, "surface_iterations")
        require(warmup_minimum_days is not None, errors, f"{prefix}: warmup_minimum_days must be a positive integer")
        require(surface_iterations is not None, errors, f"{prefix}: surface_iterations must be a positive integer")
        require(item.get("observation_only") is True, errors, f"{prefix}: observation_only must be true")
        minimum_abs_delta_c = item.get("minimum_abs_delta_c")
        require(
            isinstance(minimum_abs_delta_c, (int, float))
            and not isinstance(minimum_abs_delta_c, bool)
            and minimum_abs_delta_c > 0.0,
            errors,
            f"{prefix}: minimum_abs_delta_c must be positive",
        )
        hypothesis_id = text_field(item, "hypothesis_id")
        hypothesis = hypothesis_by_id.get(hypothesis_id)
        require(hypothesis is not None, errors, f"{prefix}: orphan active hypothesis mapping: {hypothesis_id}")
        if hypothesis is not None:
            status = text_field(hypothesis, "status")
            require(status == "unresolved", errors, f"{prefix}: active probe references {status} hypothesis: {hypothesis_id}")
            require(text_field(item, "source_state") == text_field(hypothesis, "source_state"), errors, f"{prefix}: source_state must match hypothesis")
        script = text_field(item, "script")
        require(is_safe_repo_path(script), errors, f"{prefix}: script must be a safe repository-relative path")
        require(bool(script) and (repo_root / normalized_repo_path(script)).is_file(), errors, f"{prefix}: active probe script not found: {script}")
        report_path = text_field(item, "report_path")
        report_path_normalized = normalized_repo_path(report_path)
        require(is_safe_repo_path(report_path) and report_path_normalized.lower().endswith(".json"), errors, f"{prefix}: report_path must be a safe repository-relative .json path")
        script_path = repo_root / normalized_repo_path(script)
        if script_path.is_file() and is_safe_repo_path(report_path):
            wrapper_text = script_path.read_text(encoding="utf-8", errors="replace").replace("\\", "/")
            diagnostic_args = parse_powershell_hashtable(wrapper_text, "diagnosticArgs", errors)
            expected_args = {
                "CtfSeedPolicy": text_field(item, "ctf_seed_policy"),
                "CtfInitialHistoryPolicy": text_field(item, "ctf_initial_history_policy"),
                "ZoneAirAlgorithm": text_field(item, "cli_name"),
                "WarmupMinimumDays": warmup_minimum_days,
                "SurfaceIterations": surface_iterations,
                "OutputRootRelativeOverride": "$OutputRootRelative",
                "ObservationOnly": True,
            }
            set_equality(set(diagnostic_args), set(expected_args), f"{prefix} wrapper diagnosticArgs key set", errors)
            for key, expected_value in expected_args.items():
                require(
                    diagnostic_args.get(key) == expected_value,
                    errors,
                    f"{prefix}: wrapper diagnosticArgs {key} does not match ledger: expected {expected_value!r}, got {diagnostic_args.get(key)!r}",
                )
            require(
                re.search(
                    r'&\s*\(Join-Path\s+\$PSScriptRoot\s+"official-dynamic-heat-balance-diagnostic\.ps1"\)\s+@diagnosticArgs\b',
                    wrapper_text,
                )
                is not None,
                errors,
                f"{prefix}: wrapper must invoke the locked diagnostic script with @diagnosticArgs",
            )
            version_marker = f"/{REFERENCE_VERSION}/"
            if version_marker in report_path_normalized:
                report_root, report_suffix = report_path_normalized.split(version_marker, 1)
                report_root = f"{report_root}/{REFERENCE_VERSION}"
                wrapper_report_root = normalized_repo_path(
                    parse_powershell_string_assignment(wrapper_text, "OutputRootRelative", errors)
                )
                require(wrapper_report_root == report_root, errors, f"{prefix}: wrapper does not contain declared report output root")
                require(report_suffix in wrapper_text, errors, f"{prefix}: wrapper does not contain declared report location")
            else:
                errors.append(f"{prefix}: report_path must include locked reference version {REFERENCE_VERSION}")
            observation_key = text_field(item, "observation_key")
            wrapper_observation_key = parse_powershell_string_assignment(wrapper_text, "ObservationKey", errors)
            require(wrapper_observation_key == observation_key, errors, f"{prefix}: wrapper observation key does not match ledger")
            wrapper_minimum_delta = parse_powershell_number_assignment(wrapper_text, "MinimumAbsDeltaC", errors)
            require(
                wrapper_minimum_delta == minimum_abs_delta_c,
                errors,
                f"{prefix}: wrapper minimum delta does not match ledger",
            )
            observation_array = text_field(item, "observation_json_array")
            require(f"$digest.{observation_array}" in wrapper_text, errors, f"{prefix}: wrapper does not read the declared observation array")
            for field_name in [
                text_field(item, "observation_oracle_field"),
                text_field(item, "observation_rust_field"),
                text_field(item, "observation_delta_field"),
            ]:
                require(f"$_.{field_name}" in wrapper_text, errors, f"{prefix}: wrapper does not read observation field {field_name}")
            require("$_.key -ne $ObservationKey" in wrapper_text, errors, f"{prefix}: wrapper does not restrict the observation key")
            require("-gt $MinimumAbsDeltaC" in wrapper_text, errors, f"{prefix}: wrapper does not enforce the declared minimum delta")

    for field in ["id", "hypothesis_id", "command", "script", "suite_lane"]:
        require_unique([text_field(item, field) for item in active_probes], f"active probe {field}", errors)

    unresolved = {text_field(item, "id") for item in hypotheses if text_field(item, "status") == "unresolved"}
    active_hypotheses = [text_field(item, "hypothesis_id") for item in active_probes]
    missing_active = sorted(unresolved - set(active_hypotheses))
    orphan_active = sorted(set(active_hypotheses) - unresolved)
    require(not missing_active and not orphan_active, errors, f"unresolved hypothesis/active lane bijection failed: missing active={missing_active}; orphan active={orphan_active}")
    require_unique(active_hypotheses, "active hypothesis mapping", errors)

    algorithms = algorithm_index(repo_root, errors)
    validate_hypothesis_sources(repo_root, source_map_path, hypotheses, algorithms, errors)

    rust_path = repo_root / normalized_repo_path(rust_selector_source)
    rust_text = rust_path.read_text(encoding="utf-8", errors="replace") if rust_path.is_file() else ""
    rust_selectors = parse_rust_diagnostic_selectors(rust_text, errors) if rust_text else set()
    closed_selector_pairs: list[tuple[str, str]] = []
    for item in closed_selectors:
        variant = text_field(item, "rust_variant")
        cli_name = text_field(item, "cli_name")
        prefix = f"closed selector {variant or '<missing>'}"
        require_fields(item, ["rust_variant", "cli_name", "status", "evidence_ref"], prefix, errors)
        require(text_field(item, "status") == "closed", errors, f"{prefix}: status must be closed")
        require(text_field(item, "evidence_ref") == historical_result_evidence, errors, f"{prefix}: evidence_ref must match historical_result_evidence")
        closed_selector_pairs.append((variant, cli_name))
    require_unique([item[0] for item in closed_selector_pairs], "closed selector rust_variant", errors)
    require_unique([item[1] for item in closed_selector_pairs], "closed selector cli_name", errors)
    active_diagnostic_pairs = {
        (text_field(item, "selector"), text_field(item, "cli_name"))
        for item in active_probes
        if text_field(item, "selector") in {variant for variant, _ in rust_selectors}
    }
    declared_selector_pairs = set(closed_selector_pairs) | active_diagnostic_pairs
    actual_pairs = {f"{variant}|{cli}" for variant, cli in rust_selectors}
    declared_pairs = {f"{variant}|{cli}" for variant, cli in declared_selector_pairs}
    set_equality(actual_pairs, declared_pairs, "Rust diagnostic selector classification", errors)
    for item in active_probes:
        selector = text_field(item, "selector")
        cli_name = text_field(item, "cli_name")
        selector_pair = rf"Self::{re.escape(selector)}\s*=>\s*{re.escape(chr(34) + cli_name + chr(34))}"
        require(
            re.search(selector_pair, rust_text) is not None,
            errors,
            f"active probe {text_field(item, 'id')}: exact compatibility selector/CLI pair not found in Rust boundary",
        )

    closed_script_paths: list[str] = []
    for item in closed_scripts:
        path = text_field(item, "path")
        prefix = f"closed script {path or '<missing>'}"
        require_fields(item, ["path", "status", "evidence_ref"], prefix, errors)
        require(text_field(item, "status") == "closed", errors, f"{prefix}: status must be closed")
        require(text_field(item, "evidence_ref") == historical_result_evidence, errors, f"{prefix}: evidence_ref must match historical_result_evidence")
        require(is_safe_repo_path(path), errors, f"{prefix}: path must be a safe repository-relative path")
        require(bool(path) and (repo_root / normalized_repo_path(path)).is_file(), errors, f"{prefix}: closed probe script not found")
        closed_script_paths.append(normalized_repo_path(path))
    require_unique(closed_script_paths, "closed script path", errors)
    active_script_paths = {normalized_repo_path(text_field(item, "script")) for item in active_probes}
    require(not (active_script_paths & set(closed_script_paths)), errors, "active and closed probe script classifications overlap")
    discovered = discover_probe_wrappers(repo_root)
    set_equality(discovered, active_script_paths | set(closed_script_paths), "probe wrapper classification", errors)

    suite_path = repo_root / normalized_repo_path(suite_script)
    suite_text = suite_path.read_text(encoding="utf-8", errors="replace") if suite_path.is_file() else ""
    active_lanes = parse_suite_array(suite_text, "activeLanes", errors) if suite_text else []
    closed_lanes = parse_suite_array(suite_text, "closedLanes", errors) if suite_text else []
    require_unique(active_lanes, "active suite lane", errors)
    require_unique(closed_lanes, "closed suite lane", errors)
    declared_active_lanes = {text_field(item, "suite_lane") for item in active_probes}
    set_equality(set(active_lanes), declared_active_lanes, "active suite lane", errors)
    resolved_active_lanes = {resolve_suite_lane(suite_script, lane) for lane in active_lanes}
    resolved_closed_lanes = {resolve_suite_lane(suite_script, lane) for lane in closed_lanes}
    set_equality(active_script_paths, resolved_active_lanes, "active suite script", errors)
    set_equality(set(closed_script_paths), resolved_closed_lanes, "closed suite script", errors)
    require(re.search(r"\$lanes\s*=\s*@\(\$activeLanes\)", suite_text) is not None, errors, "probe suite default must copy only $activeLanes")
    require(re.search(r"if\s*\(\$IncludeClosed\)\s*\{[^}]*\$lanes\s*\+=\s*\$closedLanes", suite_text, flags=re.DOTALL) is not None, errors, "probe suite must add closed lanes only behind -IncludeClosed")

    commands = command_catalog(repo_root, errors)
    for item in active_probes:
        probe_id = text_field(item, "id")
        command = text_field(item, "command")
        mappings = commands.get(command, [])
        require(len(mappings) == 1, errors, f"active probe {probe_id}: active probe command is not in commands.json exactly once: {command}")
        if len(mappings) == 1:
            require(mappings[0] == normalized_repo_path(text_field(item, "script")), errors, f"active probe {probe_id}: command script mapping mismatch")

    return errors


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    if args.self_test:
        from diagnostic_probe_ledger_self_tests import run_self_tests

        return run_self_tests(repo_root)

    ledger_path = repo_root / "specs" / "diagnostic_probe_ledger.toml"
    if not ledger_path.is_file():
        print("Diagnostic probe ledger validation failed:", file=sys.stderr)
        print("- missing diagnostic probe ledger: specs/diagnostic_probe_ledger.toml", file=sys.stderr)
        return 1
    try:
        spec = load_toml(ledger_path)
    except (OSError, tomllib.TOMLDecodeError) as exc:
        print(f"Diagnostic probe ledger validation failed: {exc}", file=sys.stderr)
        return 1
    errors = validate_probe_ledger(repo_root, spec)
    if errors:
        print("Diagnostic probe ledger validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    hypotheses = spec.get("hypothesis", [])
    active = spec.get("active_probe", [])
    closed_scripts = spec.get("closed_script", [])
    closed_selectors = spec.get("closed_selector", [])
    print("Diagnostic probe ledger check")
    print(f"  unresolved hypotheses: {sum(item.get('status') == 'unresolved' for item in hypotheses)}")
    print(f"  active executable lanes: {len(active)}")
    print(f"  closed replay scripts: {len(closed_scripts)}")
    print(f"  closed Rust selectors: {len(closed_selectors)}")
    print("  lifecycle: one unresolved source-state hypothesis per active executable lane")
    print("  status: valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
