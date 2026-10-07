"""Read decoded claim boundaries from their canonical TOML registries."""

from __future__ import annotations

import argparse
import json
import sys
import tomllib
from pathlib import Path
from typing import Any


REGISTRIES = {
    "algorithm": ("algorithm_ledger.toml", "support_boundary"),
    "capability": ("capabilities.toml", "claim_boundary"),
}


def boundary_text(entry: dict[str, Any], field: str) -> str:
    parts = []
    for key in (field, f"{field}_addendum"):
        value = entry.get(key, "")
        if not isinstance(value, str):
            raise ValueError(f"{entry.get('id', '')}: {key} must be a string")
        if value.strip():
            parts.append(value.strip())
    addenda = entry.get(f"{field}_addenda", [])
    if not isinstance(addenda, list) or not all(isinstance(value, str) for value in addenda):
        raise ValueError(f"{entry.get('id', '')}: {field}_addenda must be a string array")
    parts.extend(value.strip() for value in addenda if value.strip())
    return " ".join(parts)


def registry_records(spec: dict[str, Any], registry: str, field: str) -> dict[str, Any]:
    entries = spec.get(registry)
    if not isinstance(entries, list):
        raise ValueError(f"missing [[{registry}]] registry")
    records = {}
    for entry in entries:
        if not isinstance(entry, dict):
            raise ValueError(f"invalid {registry} record")
        identifier = entry.get("id")
        if not isinstance(identifier, str) or not identifier.strip() or identifier in records:
            raise ValueError(f"invalid or duplicate {registry} ID: {identifier!r}")
        boundary = boundary_text(entry, field)
        if not boundary:
            raise ValueError(f"{identifier}: missing {field}")
        record: dict[str, Any] = {"boundary": boundary}
        if registry == "capability":
            features = entry.get("forbidden_active_features", [])
            if not isinstance(features, list) or not all(isinstance(value, str) for value in features):
                raise ValueError(f"{identifier}: forbidden_active_features must be a string array")
            record["forbidden_active_features"] = features
        records[identifier] = record
    return records


def read_registries(repo_root: Path) -> dict[str, Any]:
    result = {}
    for registry, (filename, field) in REGISTRIES.items():
        with (repo_root / "specs" / filename).open("rb") as handle:
            spec = tomllib.load(handle)
        result[registry] = registry_records(spec, registry, field)
    return result


def self_test() -> None:
    spec = tomllib.loads(r'''
[[algorithm]]
id = "selected"
support_boundary = "Base with \"quoted\" state."
support_boundary_addendum = "One decoded addendum."
support_boundary_addenda = ["Line\nbreak.", "Last boundary."]
notes = "This metadata must not supply boundary evidence."
[[algorithm]]
id = "other"
support_boundary = "Only the other record has another claim."
''')
    records = registry_records(spec, "algorithm", "support_boundary")
    assert records["selected"]["boundary"] == 'Base with "quoted" state. One decoded addendum. Line\nbreak. Last boundary.'
    assert "metadata" not in records["selected"]["boundary"]
    assert "another claim" not in records["selected"]["boundary"]
    assert "missing" not in records
    for invalid in (
        {"algorithm": [spec["algorithm"][0], spec["algorithm"][0]]},
        {"algorithm": [{"id": "bad", "support_boundary": []}]},
        {"algorithm": [{"id": "bad", "support_boundary_addenda": "not an array"}]},
    ):
        try:
            registry_records(invalid, "algorithm", "support_boundary")
        except ValueError:
            pass
        else:
            raise AssertionError("malformed canonical registry was accepted")
    features = registry_records(
        {"capability": [{"id": "selected", "claim_boundary": "Restricted.", "forbidden_active_features": ["Autosizing", "EMS"]}]},
        "capability",
        "claim_boundary",
    )
    assert features["selected"]["forbidden_active_features"] == ["Autosizing", "EMS"]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    try:
        if args.self_test:
            self_test()
            print("Canonical boundary reader self-tests passed.")
        elif args.repo_root is not None:
            print(json.dumps(read_registries(args.repo_root), ensure_ascii=True))
        else:
            parser.error("--repo-root or --self-test is required")
    except (OSError, ValueError) as error:
        print(f"Canonical boundary reader failed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
