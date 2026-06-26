"""Generate mdBook reference pages from repository specs.

The generated files are navigation aids. They are not evidence and should not
be edited by hand.
"""

from __future__ import annotations

import argparse
import difflib
import json
import re
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


def repo_path(path: Path, repo_root: Path) -> str:
    return str(path.relative_to(repo_root)).replace("\\", "/")


def normalized_path(value: str) -> str:
    return value.replace("\\", "/").strip("`'\",);")


def bullet_list(values: list[str]) -> str:
    if not values:
        return "none"
    return "\n".join(f"- `{value}`" for value in values)


def inline_list(values: list[str]) -> str:
    if not values:
        return ""
    return "<br>".join(f"`{value}`" for value in values)


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(read_text(path))


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
                str(item.get("compatibility_boundary", "")),
                list_value(item.get("evidence", [])),
                list_value(item.get("not_claimed", [])),
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
        + table(
            ["Version", "Title", "Claim level", "Boundary", "Evidence", "Not claimed"],
            targets,
        )
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
                str(item.get("source_map", "")),
                list_value(item.get("energyplus_source", [])),
                list_value(item.get("rust_target", [])),
                str(item.get("first_evidence", item.get("first_case", ""))),
                list_value(item.get("proof_variables", [])),
                str(item.get("claim_level", "")),
                str(item.get("support_boundary", "")),
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
                "Source map",
                "EnergyPlus source",
                "Rust target",
                "First evidence",
                "Proof variables",
                "Claim level",
                "Boundary",
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
        meters = data.get("meters", [])
        evidence_requests = [*outputs, *meters]
        levels = sorted(
            {
                str(request.get("level", ""))
                for request in evidence_requests
                if request.get("level")
            }
        )
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
                "Evidence levels",
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
            str(item.get("first_evidence", item.get("first_case", ""))),
            str(item.get("support_boundary", "")),
        ]
        for item in spec.get("object", [])
    ]
    return (
        GENERATED_NOTICE
        + "# Object Coverage\n\n"
        + "Object coverage is maintained in `specs/object_coverage.toml`.\n\n"
        + table(["Object", "Family", "Status", "First evidence", "Boundary"], rows)
    )


def variable_coverage(repo_root: Path) -> str:
    spec = load_toml(repo_root / "specs" / "variable_coverage.toml")
    rows = [
        [
            str(item.get("name", "")),
            str(item.get("domain", "")),
            str(item.get("status", "")),
            str(item.get("first_evidence", item.get("first_case", ""))),
            str(item.get("support_boundary", "")),
        ]
        for item in spec.get("variable", [])
    ]
    return (
        GENERATED_NOTICE
        + "# Variable Coverage\n\n"
        + "Variable coverage is maintained in `specs/variable_coverage.toml`.\n\n"
        + table(["Variable", "Domain", "Status", "First evidence", "Boundary"], rows)
    )


ARTIFACT_HINT_RE = re.compile(
    r"(?i)(?:\.runtime|\.reference|target|dist|docs[\\/]book|docs[\\/]src[\\/]generated|"
    r"tools[\\/]docs[\\/]generated-docs\.manifest\.json|reports[\\/]latest)"
    r"(?:[\\/][A-Za-z0-9_.${}()\-]+)*"
)
INPUT_HINT_RE = re.compile(
    r"(?i)(?:data|specs|config|docs[\\/]src|tools[\\/]oracle|tools[\\/]python)"
    r"[\\/][A-Za-z0-9_.${}()\-\\/]+"
)
PYTHON_SCRIPT_RE = re.compile(r"tools[\\/](?:docs|reporting)[\\/][A-Za-z0-9_.-]+\.py")
DEV_COMMAND_RE = re.compile(r"Invoke-DevCommand\s+-Command\s+['\"]([^'\"]+)['\"]")
CARGO_COMMAND_RE = re.compile(r"cargo\s+(?:build|clippy|fmt|run|test)[^\r\n`|&;]*")
README_DEV_COMMAND_RE = re.compile(r"\.\\scripts\\dev\.(?:cmd|ps1)\s+([A-Za-z0-9_.-]+)")
SUMMARY_LINK_RE = re.compile(r"^- \[[^\]]+\]\(([^)]+)\)")
FRONT_MATTER_RE = re.compile(r"^---\n(.*?)\n---", re.DOTALL)
README_CURRENT_DOC_RE = re.compile(r"`(docs/src/current/[^`]+\.md)`")

CURRENT_DOCS = [
    "docs/src/current/project-contract.md",
    "docs/src/current/current-status.md",
    "docs/src/current/roadmap.md",
    "docs/src/current/verification.md",
    "docs/src/current/architecture-overview.md",
    "docs/src/current/launcher-and-run-framework.md",
]
GENERATED_DOC_OUTPUTS = [
    "docs/src/generated/milestone-map.md",
    "docs/src/generated/algorithm-ledger.md",
    "docs/src/generated/conformance-case-index.md",
    "docs/src/generated/object-coverage.md",
    "docs/src/generated/variable-coverage.md",
    "docs/src/generated/script-index.md",
    "docs/src/generated/docs-inventory.md",
]


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
    if suffix in {".json", ".md"}:
        return "n/a"
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
        if group == "gui":
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
    if folder == "gui":
        return "launcher"
    if folder in {"setup", "quality", "smoke", "compare", "conformance", "release"}:
        return folder
    return "internal"


def script_status(relative_path: str, command: dict[str, Any] | None) -> str:
    if relative_path in {"scripts/dev.cmd", "scripts/dev.ps1"}:
        return "public user command"
    if command is None:
        return "internal helper"

    group = str(command.get("group", ""))
    name = str(command.get("name", ""))
    if group == "release":
        return "release command"
    if group == "gui" and name == "launch-ui":
        return "public user command"
    if group == "setup":
        return "public user command"
    return "developer command"


def script_inventory(repo_root: Path) -> str:
    catalog_path = repo_root / "scripts" / "dev" / "commands.json"
    catalog = load_json(catalog_path)
    commands = list(catalog.get("commands", []))
    aliases = dict(catalog.get("aliases", {}))

    command_by_script: dict[str, dict[str, Any]] = {}
    command_names = {str(entry.get("name", "")) for entry in commands}
    missing_targets: list[str] = []
    duplicate_targets: list[str] = []
    for entry in commands:
        script = "scripts/" + normalized_path(str(entry.get("path", "")))
        key = script.lower()
        if key in command_by_script:
            duplicate_targets.append(script)
        command_by_script[key] = entry
        if not (repo_root / script).exists():
            missing_targets.append(f"{entry.get('name', '')} -> {script}")

    script_files = sorted(path for path in (repo_root / "scripts").rglob("*") if path.is_file())
    rows = []
    unexposed_ps1: list[str] = []
    public_folder_unexposed: list[str] = []
    for path in script_files:
        relative = repo_path(path, repo_root)
        command = command_by_script.get(relative.lower())
        text = read_text(path)
        category = script_category(relative, command)
        status = script_status(relative, command)
        command_name = str(command.get("name", "")) if command else ""
        calls = extract_call_hints(text)
        inputs = extract_path_hints(INPUT_HINT_RE, text)
        artifacts = extract_path_hints(ARTIFACT_HINT_RE, text)
        rows.append(
            [
                relative,
                category,
                status,
                command_name,
                inline_list(calls),
                inline_list(inputs),
                inline_list(artifacts),
                exit_contract(path, text),
            ]
        )
        if path.suffix.lower() == ".ps1" and command is None:
            unexposed_ps1.append(relative)
            if category in {"setup", "quality", "smoke", "compare", "conformance", "release"}:
                public_folder_unexposed.append(relative)

    readme = read_text(repo_root / "README.md")
    readme_commands = sorted({match.group(1) for match in README_DEV_COMMAND_RE.finditer(readme)})
    exposed_or_alias = command_names | set(aliases)
    missing_readme_commands = [
        command for command in readme_commands if command not in exposed_or_alias
    ]

    summary_rows = [
        ["script files", str(len(script_files))],
        ["dev commands", str(len(commands))],
        ["aliases", str(len(aliases))],
        ["missing command targets", str(len(missing_targets))],
        ["duplicate command targets", str(len(duplicate_targets))],
        ["unexposed PowerShell files", str(len(unexposed_ps1))],
        ["unexposed public-folder PowerShell files", str(len(public_folder_unexposed))],
        ["README dev commands missing from catalog", str(len(missing_readme_commands))],
    ]

    return (
        GENERATED_NOTICE
        + "# Script Inventory\n\n"
        + "Script metadata is generated from `scripts/`, `scripts/dev/commands.json`, and README command examples.\n\n"
        + "Status values distinguish public user commands, developer commands, release commands, and internal helpers. "
        + "Call, input, and artifact columns are static hints extracted from script text; wrapper commands remain authoritative in `scripts/dev/commands.json`.\n\n"
        + "## Summary\n\n"
        + table(["Check", "Count"], summary_rows)
        + "\n## Dev Command Catalog Checks\n\n"
        + "**Missing command targets**\n\n"
        + bullet_list(missing_targets)
        + "\n\n**Duplicate command targets**\n\n"
        + bullet_list(duplicate_targets)
        + "\n\n**README dev commands missing from catalog**\n\n"
        + bullet_list(missing_readme_commands)
        + "\n\n**Unexposed PowerShell files in public command folders**\n\n"
        + bullet_list(public_folder_unexposed)
        + "\n\n## Inventory\n\n"
        + table(
            [
                "Path",
                "Category",
                "Status",
                "Dev command",
                "Calls",
                "Input hints",
                "Artifact hints",
                "Exit contract",
            ],
            rows,
        )
    )


def parse_summary_links(repo_root: Path) -> dict[str, str]:
    links: dict[str, str] = {}
    section = ""
    for line in read_text(repo_root / "docs" / "src" / "SUMMARY.md").splitlines():
        if line.startswith("# "):
            section = line[2:].strip()
            continue
        match = SUMMARY_LINK_RE.match(line)
        if match:
            links["docs/src/" + normalized_path(match.group(1))] = section
    return links


def markdown_front_matter(text: str) -> dict[str, str]:
    match = FRONT_MATTER_RE.match(text)
    if not match:
        return {}

    values: dict[str, str] = {}
    for line in match.group(1).splitlines():
        if ":" not in line:
            continue
        key, value = line.split(":", 1)
        values[key.strip()] = value.strip()
    return values


def docs_category(relative_path: str) -> str:
    if relative_path.startswith("docs/src/current/"):
        return "current"
    if (
        relative_path.startswith("docs/src/guides/")
        or relative_path.startswith("docs/src/user-guide/")
        or relative_path == "docs/src/quick-start.md"
    ):
        return "guide"
    if relative_path.startswith("docs/src/generated/"):
        return "generated"
    if relative_path.startswith("docs/src/releases/"):
        return "release-note"
    if relative_path.startswith("docs/src/porting-map/"):
        return "source-map"
    if (
        relative_path.startswith("docs/src/architecture/")
        or relative_path.startswith("docs/src/conformance/")
        or relative_path.startswith("docs/src/operations/")
        or relative_path.startswith("docs/src/adr/")
        or relative_path in {"docs/src/introduction.md", "docs/src/SUMMARY.md"}
    ):
        return "spec-explanation"
    return "removable"


def docs_inventory(repo_root: Path) -> str:
    summary_links = parse_summary_links(repo_root)
    summary_current = [
        path for path, section in summary_links.items() if section == "Current"
    ]
    readme = read_text(repo_root / "README.md")
    readme_current = sorted({normalized_path(match.group(1)) for match in README_CURRENT_DOC_RE.finditer(readme)})
    readme_sections = len(re.findall(r"^## ", readme, flags=re.MULTILINE))

    markdown_files = {repo_path(path, repo_root): path for path in (repo_root / "docs" / "src").rglob("*.md")}
    for expected in GENERATED_DOC_OUTPUTS:
        markdown_files.setdefault(expected, repo_root / expected)

    rows = []
    generated_missing_notice: list[str] = []
    for relative in sorted(markdown_files):
        path = markdown_files[relative]
        text = read_text(path) if path.exists() else ""
        front_matter = markdown_front_matter(text)
        category = docs_category(relative)
        summary_section = summary_links.get(relative, "")
        generated_notice = "n/a"
        if category == "generated":
            generated_notice = "present" if text.startswith(GENERATED_NOTICE) else "missing"
            if generated_notice == "missing":
                generated_missing_notice.append(relative)

        rows.append(
            [
                relative,
                category,
                summary_section or "not in SUMMARY",
                generated_notice,
                "present" if front_matter else "none",
                front_matter.get("status", ""),
                front_matter.get("owner", ""),
                front_matter.get("last_reviewed", ""),
            ]
        )

    expected_current_set = set(CURRENT_DOCS)
    summary_current_set = set(summary_current)
    readme_current_set = set(readme_current)
    current_missing = [path for path in CURRENT_DOCS if path not in summary_current_set]
    current_unexpected = [path for path in summary_current if path not in expected_current_set]
    readme_missing = [path for path in CURRENT_DOCS if path not in readme_current_set]
    readme_unexpected = [path for path in readme_current if path not in expected_current_set]
    release_notes_in_current = [
        path for path in summary_current if path.startswith("docs/src/releases/")
    ]
    unlinked_docs = [
        relative
        for relative in sorted(markdown_files)
        if relative not in summary_links and not relative.startswith("docs/src/generated/")
    ]

    summary_rows = [
        ["docs files", str(len(markdown_files))],
        ["SUMMARY links", str(len(summary_links))],
        ["README h2 sections", str(readme_sections)],
        ["README h2 section limit", "pass" if readme_sections <= 7 else "fail"],
        ["Current nav expected", str(len(CURRENT_DOCS))],
        ["Current nav actual", str(len(summary_current))],
        ["Current nav missing", str(len(current_missing))],
        ["Current nav unexpected", str(len(current_unexpected))],
        ["README current docs missing", str(len(readme_missing))],
        ["README current docs unexpected", str(len(readme_unexpected))],
        ["Generated docs missing notice", str(len(generated_missing_notice))],
        ["Release notes in Current nav", str(len(release_notes_in_current))],
        ["Non-generated docs not in SUMMARY", str(len(unlinked_docs))],
    ]

    current_rows = [
        [
            str(index + 1),
            CURRENT_DOCS[index],
            summary_current[index] if index < len(summary_current) else "",
            "pass" if index < len(summary_current) and CURRENT_DOCS[index] == summary_current[index] else "fail",
        ]
        for index in range(max(len(CURRENT_DOCS), len(summary_current)))
    ]

    return (
        GENERATED_NOTICE
        + "# Docs Inventory\n\n"
        + "Documentation metadata is generated from `docs/src`, `docs/src/SUMMARY.md`, and README current-doc references.\n\n"
        + "## Summary\n\n"
        + table(["Check", "Result"], summary_rows)
        + "\n## Current Navigation Check\n\n"
        + table(["Order", "Expected", "Actual", "Result"], current_rows)
        + "\n**README current docs missing**\n\n"
        + bullet_list(readme_missing)
        + "\n\n**README current docs unexpected**\n\n"
        + bullet_list(readme_unexpected)
        + "\n\n**Generated docs missing notice**\n\n"
        + bullet_list(generated_missing_notice)
        + "\n\n**Release notes in Current navigation**\n\n"
        + bullet_list(release_notes_in_current)
        + "\n\n## Inventory\n\n"
        + table(
            [
                "Path",
                "Category",
                "SUMMARY section",
                "Generated notice",
                "Front matter",
                "Status",
                "Owner",
                "Last reviewed",
            ],
            rows,
        )
    )


def generated_manifest(repo_root: Path) -> str:
    payload = {
        "sources": [
            "specs/milestones.toml",
            "specs/algorithm_ledger.toml",
            "specs/object_coverage.toml",
            "specs/variable_coverage.toml",
            "data/conformance_cases/*/case.toml",
            "scripts/**/*",
            "scripts/dev/commands.json",
            "README.md",
            "docs/src/**/*.md",
            "docs/src/SUMMARY.md",
        ],
        "outputs": [
            *GENERATED_DOC_OUTPUTS,
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
        repo_root / "docs" / "src" / "generated" / "script-index.md": script_inventory(repo_root),
        repo_root / "docs" / "src" / "generated" / "docs-inventory.md": docs_inventory(repo_root),
        repo_root / "tools" / "docs" / "generated-docs.manifest.json": generated_manifest(repo_root),
    }

    ok = True
    for path, content in outputs.items():
        ok = write_or_check(path, content, args.check) and ok

    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
