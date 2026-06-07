"""Generate mdBook reference pages from repository specs.

The generated files are navigation aids. They are not evidence and should not
be edited by hand.
"""

from __future__ import annotations

import argparse
import difflib
import json
import sys
import tomllib
from pathlib import Path
from typing import Any


GENERATED_NOTICE = """<!-- DO NOT EDIT.
     Generated from specs/ and data/conformance_cases by tools/docs/generate_docs.py. -->

"""


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def table(headers: list[str], rows: list[list[str]]) -> str:
    output = ["| " + " | ".join(headers) + " |"]
    output.append("|" + "|".join(["---"] * len(headers)) + "|")
    for row in rows:
        output.append("| " + " | ".join(markdown_cell(value) for value in row) + " |")
    return "\n".join(output) + "\n"


def markdown_cell(value: str) -> str:
    return value.replace("|", "\\|").replace("\n", "<br>")


def list_value(value: Any) -> str:
    if value is None:
        return ""
    if isinstance(value, list):
        return ", ".join(str(item) for item in value)
    return str(value)


def write_or_check(path: Path, content: str, check: bool) -> bool:
    if check:
        existing = path.read_text(encoding="utf-8") if path.exists() else ""
        if existing != content:
            diff = difflib.unified_diff(
                existing.splitlines(),
                content.splitlines(),
                fromfile=str(path),
                tofile=f"{path} (generated)",
                lineterm="",
            )
            sys.stderr.write("\n".join(diff) + "\n")
            return False
        return True

    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8", newline="\n")
    return True


def milestone_map(repo_root: Path) -> str:
    spec = load_toml(repo_root / "specs" / "milestones.toml")
    rows = []
    for item in spec.get("milestone", []):
        rows.append(
            [
                str(item.get("version", "")),
                str(item.get("title", "")),
                str(item.get("status", "")),
                str(item.get("claim_level", "")),
                list_value(item.get("required_cases", [])),
                list_value(item.get("not_claimed", [])),
            ]
        )

    targets = []
    for item in spec.get("target", []):
        targets.append(
            [
                str(item.get("version", "")),
                str(item.get("title", "")),
                str(item.get("claim_level", "")),
            ]
        )

    return (
        GENERATED_NOTICE
        + "# Milestone Map\n\n"
        + "Milestones are maintained in `specs/milestones.toml`.\n\n"
        + table(
            ["Version", "Title", "Status", "Claim level", "Required cases", "Not claimed"],
            rows,
        )
        + "\n## Long-Term Targets\n\n"
        + table(["Version", "Title", "Claim level"], targets)
    )


def algorithm_ledger(repo_root: Path) -> str:
    spec = load_toml(repo_root / "specs" / "algorithm_ledger.toml")
    rows = []
    for item in spec.get("algorithm", []):
        rows.append(
            [
                str(item.get("id", "")),
                str(item.get("domain", "")),
                str(item.get("status", "")),
                list_value(item.get("energyplus_source", [])),
                list_value(item.get("rust_target", [])),
                str(item.get("first_case", "")),
                list_value(item.get("proof_variables", [])),
                str(item.get("claim_level", "")),
            ]
        )

    return (
        GENERATED_NOTICE
        + "# Algorithm Ledger\n\n"
        + "Algorithm status is maintained in `specs/algorithm_ledger.toml`.\n\n"
        + table(
            [
                "ID",
                "Domain",
                "Status",
                "EnergyPlus source",
                "Rust target",
                "First case",
                "Proof variables",
                "Claim level",
            ],
            rows,
        )
    )


def conformance_case_index(repo_root: Path) -> str:
    rows = []
    for path in sorted((repo_root / "data" / "conformance_cases").glob("*/case.toml")):
        data = load_toml(path)
        manifest = data.get("manifest_v2", {})
        scope = data.get("scope", {})
        outputs = data.get("outputs", [])
        levels = sorted({str(output.get("level", "")) for output in outputs if output.get("level")})
        rows.append(
            [
                str(data.get("id", path.parent.name)),
                str(data.get("milestone", "")),
                str(data.get("comparison_class", "")),
                str(data.get("conformance_claim", False)).lower(),
                str(manifest.get("tier", "")),
                list_value(scope.get("domains", [])),
                ", ".join(levels),
                str(path.relative_to(repo_root)).replace("\\", "/"),
            ]
        )

    return (
        GENERATED_NOTICE
        + "# Conformance Case Index\n\n"
        + "Case metadata is read from `data/conformance_cases/*/case.toml`.\n\n"
        + table(
            [
                "Case",
                "Milestone",
                "Class",
                "Claim",
                "Tier",
                "Domains",
                "Output levels",
                "Manifest",
            ],
            rows,
        )
    )


def object_coverage(repo_root: Path) -> str:
    spec = load_toml(repo_root / "specs" / "object_coverage.toml")
    rows = [
        [
            str(item.get("name", "")),
            str(item.get("family", "")),
            str(item.get("status", "")),
        ]
        for item in spec.get("object", [])
    ]
    return (
        GENERATED_NOTICE
        + "# Object Coverage\n\n"
        + "Object coverage is maintained in `specs/object_coverage.toml`.\n\n"
        + table(["Object", "Family", "Status"], rows)
    )


def variable_coverage(repo_root: Path) -> str:
    spec = load_toml(repo_root / "specs" / "variable_coverage.toml")
    rows = [
        [
            str(item.get("name", "")),
            str(item.get("domain", "")),
            str(item.get("status", "")),
            str(item.get("first_case", "")),
        ]
        for item in spec.get("variable", [])
    ]
    return (
        GENERATED_NOTICE
        + "# Variable Coverage\n\n"
        + "Variable coverage is maintained in `specs/variable_coverage.toml`.\n\n"
        + table(["Variable", "Domain", "Status", "First case"], rows)
    )


def generated_manifest(repo_root: Path) -> str:
    payload = {
        "sources": [
            "specs/milestones.toml",
            "specs/algorithm_ledger.toml",
            "specs/object_coverage.toml",
            "specs/variable_coverage.toml",
            "data/conformance_cases/*/case.toml",
        ],
        "outputs": [
            "docs/src/generated/milestone-map.md",
            "docs/src/generated/algorithm-ledger.md",
            "docs/src/generated/conformance-case-index.md",
            "docs/src/generated/object-coverage.md",
            "docs/src/generated/variable-coverage.md",
        ],
    }
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", type=Path, required=True)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()

    repo_root = args.repo_root.resolve()
    outputs = {
        repo_root / "docs" / "src" / "generated" / "milestone-map.md": milestone_map(repo_root),
        repo_root / "docs" / "src" / "generated" / "algorithm-ledger.md": algorithm_ledger(repo_root),
        repo_root / "docs" / "src" / "generated" / "conformance-case-index.md": conformance_case_index(repo_root),
        repo_root / "docs" / "src" / "generated" / "object-coverage.md": object_coverage(repo_root),
        repo_root / "docs" / "src" / "generated" / "variable-coverage.md": variable_coverage(repo_root),
        repo_root / "tools" / "docs" / "generated-docs.manifest.json": generated_manifest(repo_root),
    }

    ok = True
    for path, content in outputs.items():
        ok = write_or_check(path, content, args.check) and ok

    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
