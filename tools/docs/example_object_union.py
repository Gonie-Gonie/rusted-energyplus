"""Inventory example input occurrences; presence never establishes support."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
import tomllib
from collections import Counter
from pathlib import Path
from typing import Any


CATALOG = "data/schema/energyplus-26.1-object-index.json"
OUTPUT = "docs/src/generated/example-object-union.md"
SCHEMA_SHA = "60b24d904e6fa553f773b38037a8a09730e6da1538647df552cd77db8d6704a9"
EP_COMMIT = "6f2e40d10250a105b49966baa24d843711e61048"
ASCII_FOLD = str.maketrans("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "abcdefghijklmnopqrstuvwxyz")



OBJECT_STATUS_LABELS = {
    "typed": "Reviewed input projection", "typed_graph_only": "Typed graph only",
    "diagnostic": "Diagnostic input projection", "raw_source_getter": "Raw Source getter",
    "partial_input": "Partial input projection", "unimplemented": "No connected implementation",
}
SOURCE_ADMISSION_LABELS = {
    "bounded_a_b": "Bounded A/B", "bounded_a": "Bounded A only", "bounded_b": "Bounded B only",
    "inactive_only": "Inactive declaration only", "blocked": "Blocked", "unreviewed": "Not reviewed",
}


def compiler_typed_names(repo_root: Path) -> set[str]:
    """Read the compiler's explicit whole-object bitmap, including string constants.

    This is documentation of typed_coverage_status, never a runtime inference.
    Refuse a changed expression grammar rather than silently dropping entries.
    """
    text = (repo_root / "crates/ep_compiler/src/compiler.rs").read_text(encoding="utf8")
    constants = dict(re.findall(r'const ([A-Z_]+): &str = "([^"\\]*)";', text))
    match = re.search(r'const TYPED_OBJECT_TYPES: &\[&str\] = &\[(.*?)\];', text, re.S)
    if match is None:
        raise ValueError("compiler typed-object bitmap is missing")
    result: set[str] = set()
    for token in (part.strip() for part in match[1].split(",")):
        if not token:
            continue
        if re.fullmatch(r'"[^"\\]+"', token):
            name = json.loads(token)
        elif token in constants:
            name = constants[token]
        else:
            raise ValueError(f"unreviewed compiler bitmap expression: {token}")
        if name in result:
            raise ValueError(f"duplicate compiler typed object: {name}")
        result.add(name)
    return result


def object_registry(repo_root: Path) -> dict[str, dict[str, Any]]:
    with (repo_root / "specs/object_coverage.toml").open("rb") as stream:
        spec = tomllib.load(stream)
    if spec.get("coverage_schema") != 2:
        raise ValueError("object registry classification schema differs")
    typed = compiler_typed_names(repo_root)
    result: dict[str, dict[str, Any]] = {}
    for row in spec.get("object", []):
        name, status = row["name"], row["status"]
        if name in result:
            raise ValueError(f"duplicate object registry entry: {name}")
        if status not in OBJECT_STATUS_LABELS:
            raise ValueError(f"unknown object handling status: {name}: {status}")
        actual = "typed" if name in typed else "raw_only"
        if row.get("compiler_coverage") != actual:
            raise ValueError(f"compiler coverage differs from actual bitmap: {name}")
        if status == "typed_graph_only" and actual != "typed":
            raise ValueError(f"typed graph entry has no typed compiler model: {name}")
        if status in {"raw_source_getter", "partial_input", "unimplemented"} and actual != "raw_only":
            raise ValueError(f"raw/partial/missing entry incorrectly hides a typed model: {name}")
        admission = row.get("source_admission")
        if admission not in SOURCE_ADMISSION_LABELS:
            raise ValueError(f"unknown Source admission: {name}: {admission}")
        if admission != "unreviewed":
            refs = row.get("source_refs")
            if not row.get("source_boundary") or not isinstance(refs, list) or not refs:
                raise ValueError(f"reviewed Source admission needs boundary and refs: {name}")
            for ref in refs:
                path = (repo_root / ref).resolve()
                if not path.is_relative_to(repo_root.resolve()) or not path.is_file():
                    raise ValueError(f"missing/escaping Source review reference: {name}: {ref}")
        if status == "unimplemented" and admission not in {"blocked", "unreviewed"}:
            raise ValueError(f"missing implementation cannot have positive admission: {name}")
        result[name] = row
    return result


def input_typing(row: dict[str, Any]) -> str:
    if row["compiler_coverage"] == "typed":
        return "Typed model; graph only" if row["status"] == "typed_graph_only" else "Typed model"
    if row["status"] == "partial_input":
        return "Raw only; partial projection"
    if row["status"] == "raw_source_getter" or row["source_admission"] in {"bounded_a_b", "bounded_a", "bounded_b", "inactive_only"}:
        return "Raw only; Source getter"
    return "Raw only"


def source_review_cell(row: dict[str, Any]) -> str:
    boundary = row.get("source_boundary", "Not reviewed for the ordinary Source route.")
    refs = row.get("source_refs", [])
    links = ", ".join(f"[`{Path(ref).name}`](../../../{ref})" for ref in refs)
    return (boundary + (" Review: " + links + "." if links else "")).replace("|", "\\|").replace("\n", " ")


def catalog_from_schema(path: Path) -> dict[str, Any]:
    raw = path.read_bytes()
    if hashlib.sha256(raw).hexdigest() != SCHEMA_SHA:
        raise ValueError("schema bytes differ from the pinned EnergyPlus 26.1 distribution")
    schema = json.loads(raw)
    names = {name: row.get("group", "") for name, row in sorted(schema["properties"].items())}
    if len(names) != 858:
        raise ValueError("pinned schema object count differs")
    return {
        "schema": "energyplus-object-name-index.v1",
        "authority": {
            "energyplus_version": "26.1.0", "energyplus_commit": EP_COMMIT,
            "schema_source": "Energy+.schema.epJSON from the EnergyPlus 26.1.0 distribution",
            "schema_sha256": SCHEMA_SHA, "schema_bytes": len(raw), "json_schema": schema["$schema"],
            "source_url": f"https://github.com/NatLabRockies/EnergyPlus/tree/{EP_COMMIT}/idd",
        },
        "purpose": "Canonical object type names and official groups only; no field, value, parser, or runtime support claim.",
        "object_types": names,
    }


def idf_records(text: str) -> list[list[str]]:
    """Scan a corpus record, retaining blank fields and quoted delimiters."""
    records: list[list[str]] = []
    fields: list[str] = []
    token: list[str] = []
    quote = ""
    comment = False
    index = 0
    text = text.removeprefix("\ufeff")
    while index < len(text):
        char = text[index]
        index += 1
        if comment:
            if char in "\r\n":
                comment = False
            continue
        if quote:
            if char == quote:
                if index < len(text) and text[index] == quote:
                    token.append(char)
                    index += 1
                else:
                    quote = ""
            else:
                token.append(char)
            continue
        if char == "!":
            comment = True
        elif char in "\"'" and not "".join(token).strip():
            token.clear()
            quote = char
        elif char in ",;":
            fields.append("".join(token).strip())
            token.clear()
            if char == ";":
                if not fields[0]:
                    raise ValueError("IDF record has no object type")
                records.append(fields)
                fields = []
        else:
            token.append(char)
    if quote:
        raise ValueError("unterminated quoted IDF field")
    if fields or "".join(token).strip():
        raise ValueError("unterminated IDF record")
    return records


def unique_json_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def object_counts(path: Path) -> Counter[str]:
    text = path.read_text(encoding="utf-8-sig")
    if path.suffix.casefold() == ".idf":
        return Counter(record[0] for record in idf_records(text))
    model = json.loads(text, object_pairs_hook=unique_json_object)
    if not isinstance(model, dict) or not all(isinstance(row, dict) for row in model.values()):
        raise ValueError("epJSON must map object types to instance maps")
    return Counter({name: len(instances) for name, instances in model.items() if instances})


def sample_paths(repo_root: Path) -> list[Path]:
    """Select tracked and ordinary untracked samples, respecting Git ignores."""
    output = subprocess.check_output(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", "data", "docs/validation"],
        cwd=repo_root,
    )
    selected = set()
    for name in output.decode("utf-8").split("\0"):
        if not name:
            continue
        relative = Path(name)
        if relative.suffix.casefold() not in {".idf", ".epjson"}:
            continue
        if {".git", ".runtime", ".reference", "objects"} & set(relative.parts):
            continue
        path = repo_root / relative
        if path.is_file():
            if not path.resolve().is_relative_to(repo_root.resolve()):
                raise ValueError(f"sample escapes repository: {name}")
            selected.add(relative.as_posix())
    return [repo_root / name for name in sorted(selected)]


def dependency_group(name: str) -> str:
    """Review buckets, not an executable order or inferred dependency proof."""
    if name.startswith(("Output:", "OutputControl:")):
        return "07 Reporting"
    if name.startswith(("Plant", "Branch", "Connector", "Boiler:", "Chiller:", "Pump:", "Pipe:", "LoadProfile:", "District")):
        return "06 Plant and loops"
    if name.startswith(("Schedule", "ExternalInterface:")):
        return "02 Schedules"
    if name.startswith(("Material", "WindowMaterial:", "WindowGap:", "Construction")):
        return "03 Materials and constructions"
    if name in {"Zone", "ZoneList", "ZoneGroup", "Space", "SpaceList", "People", "Lights", "OtherEquipment", "ElectricEquipment", "GasEquipment"} or name.startswith(("BuildingSurface:", "FenestrationSurface:", "SurfaceProperty:", "SurfaceProperties:", "ZoneProperty:", "Shading:", "WindowProperty:")):
        return "04 Zones, surfaces and gains"
    if name.startswith(("ZoneHVAC:", "ZoneControl:", "Thermostat", "NodeList", "OutdoorAir:", "DesignSpecification:", "AirLoop", "AirTerminal:", "Fan:", "Coil:", "Controller:", "SetpointManager:", "AvailabilityManager:", "EnergyManagementSystem:")):
        return "05 Controls, nodes and equipment"
    if name in {"Version", "Building", "Timestep", "SimulationControl", "GlobalGeometryRules", "HeatBalanceAlgorithm", "ZoneAirHeatBalanceAlgorithm", "ShadowCalculation"} or name.startswith(("RunPeriod", "SizingPeriod:", "Site:", "SurfaceConvectionAlgorithm:")):
        return "01 Project, time and site"
    return "08 Other dependencies to review"


def collect_union(repo_root: Path) -> dict[str, Any]:
    catalog = json.loads((repo_root / CATALOG).read_text(encoding="utf-8"))
    authority = catalog["authority"]
    names = catalog["object_types"]
    if catalog["schema"] != "energyplus-object-name-index.v1" or authority["energyplus_version"] != "26.1.0" or authority["schema_sha256"] != SCHEMA_SHA or authority["energyplus_commit"] != EP_COMMIT or len(names) != 858:
        raise ValueError("object-name catalog differs from the pinned EnergyPlus 26.1 authority")
    canonical = {name.translate(ASCII_FOLD): name for name in names}
    if len(canonical) != len(names):
        raise ValueError("case-ambiguous canonical object names")
    reviewed = object_registry(repo_root)
    rows: dict[str, dict[str, Any]] = {}
    paths = sample_paths(repo_root)
    formats: Counter[str] = Counter()
    for path in paths:
        formats[path.suffix.casefold()] += 1
        try:
            counts = object_counts(path)
        except (ValueError, OSError) as error:
            raise ValueError(f"{path.relative_to(repo_root).as_posix()}: {error}") from error
        per_file: Counter[str] = Counter()
        for raw, count in counts.items():
            per_file[canonical.get(raw.translate(ASCII_FOLD), raw)] += count
        for name, count in per_file.items():
            row = rows.setdefault(name, {
                "name": name, "schema_known": name in names, "files": 0, "instances": 0,
                "group": dependency_group(name), "schema_group": names.get(name),
                "review": reviewed.get(name),
            })
            row["files"] += 1
            row["instances"] += count
    for row in rows.values():
        if row["schema_known"] and (row["review"] is None or row["review"]["source_admission"] == "unreviewed"):
            raise ValueError(f"canonical corpus object lacks reviewed classification: {row['name']}")
    # Accepted domain expansions can introduce a type in an ignored, retained
    # same-input fixture. Use their published review record so regeneration does
    # not depend on local runtime files, and preserve the original file counts.
    evidence = json.loads((repo_root / "docs/validation/evidence/object-input-domains.json").read_text(encoding="utf-8"))
    additional = []
    for name in sorted(evidence["reviewed_domains"]):
        if name in rows:
            continue
        if name not in names or name not in reviewed or reviewed[name]["source_admission"] == "unreviewed":
            raise ValueError(f"accepted domain object lacks canonical reviewed classification: {name}")
        additional.append({"name": name, "review": reviewed[name]})
    return {
        "files": len(paths), "idf_files": formats[".idf"], "epjson_files": formats[".epjson"],
        "rows": sorted(rows.values(), key=lambda row: (row["group"], row["name"])),
        "additional_reviewed_types": additional,
    }


def markdown(repo_root: Path) -> str:
    inventory = collect_union(repo_root)
    known = [row for row in inventory["rows"] if row["schema_known"]]
    unknown = [row for row in inventory["rows"] if not row["schema_known"]]
    additional = inventory["additional_reviewed_types"]
    lines = [
        "<!-- DO NOT EDIT. Generated from repository samples, accepted domain reviews, the pinned object catalog and specs/object_coverage.toml by tools/docs/generate_docs.py. -->",
        "", "# Example Objects and Source Boundaries", "",
        f"The selected corpus has **{inventory['files']} files** ({inventory['idf_files']} IDF, {inventory['epjson_files']} epJSON), **{len(known)} canonical object types**, and **{sum(row['instances'] for row in known)} instances** of those types.",
        "",
        f"Including the additional types in published [accepted domain expansions](../../validation/evidence/object-input-domains.json), the combined inventory contains **{len(known) + len(additional)} canonical object types**. The additional types are listed separately below; they do not change the original corpus occurrence counts.",
        "",
        "Selection includes tracked and ordinary untracked input files under `data/` and `docs/validation/`, respecting Git ignores. It excludes runtime/reference trees and immutable `objects` stores. Invalid-field and negative fixtures remain included; embedded code literals and compressed artifacts are outside this file corpus.",
        "",
        "This public corpus list separates occurrence counts, whole-object compiler typing and actual ordinary Source admission. **No row means full-object or arbitrary-combination support.** Counts include negative fixtures, inactive declarations and unused objects. Compiler typing comes from `Compiler::typed_coverage_status`; raw-only objects can still have genuine Source getters or partial projections. The [full registry](object-coverage.md) retains other reviewed types outside this corpus. [Capabilities](../../../specs/capabilities.toml), raw admission guards and connected consumers jointly bound execution; legacy/fixed-case evidence is not automatically Source support.",
        "",
        "Review groups provide a development queue, not IDF declaration order or runtime execution order. Cross-group references and cycles require explicit register, resolve and consume phases. The [roadmap](../current/roadmap.md) defines the expansion work and [project contract](../current/project-contract.md#reviewing-a-porting-change) defines its evidence requirements.",
        "",
        "Canonical names and official groups come from the compact [EnergyPlus 26.1 catalog](../../../data/schema/energyplus-26.1-object-index.json). It records the original schema SHA256 and pinned source commit; installed EnergyPlus is not needed to regenerate this page.",
        "",
        "Source labels are bounded A/B, bounded A only, bounded B only, inactive declaration only, blocked, or not reviewed. B is the declared direct-Zone sensible no-OA IdealLoads route. An admitted family still requires every applicable field/reference/consumer condition. Review references are code/contract facts, not additional numerical conformance cases; promoted identities and tolerances are unchanged.",
        "", "| Object type | Input files | Instances | Dependency review group | Compiler input typing | Source admission | Scope and review references |", "|---|---|---|---|---|---|---|",
    ]
    lines.extend(f"| `{row['name']}` | {row['files']} | {row['instances']} | {row['group']} | {input_typing(row['review'])} | {SOURCE_ADMISSION_LABELS[row['review']['source_admission']]} | {source_review_cell(row['review'])} |" for row in known)
    if additional:
        lines += ["", "## Additional Types From Accepted Domain Expansions", "", "These types occur in the retained domain-expansion inputs and have published review records. Their local runtime inputs are excluded from the file corpus above; the combined inventory includes each type once. Admission remains bounded by the [full registry](object-coverage.md).", "", "| Object type | Compiler input typing | Source admission |", "|---|---|---|"]
        lines.extend(f"| `{row['name']}` | {input_typing(row['review'])} | {SOURCE_ADMISSION_LABELS[row['review']['source_admission']]} |" for row in additional)
    if unknown:
        lines += ["", "## Unknown Input Types", "", "These names are absent from the pinned schema. `Unknown:Diagnostic` is an intentional local diagnostic fixture, not an EnergyPlus object type.", "", "| Input type | Input files | Instances |", "|---|---|---|"]
        lines.extend(f"| `{row['name']}` | {row['files']} | {row['instances']} |" for row in unknown)
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--check", action="store_true", help="check the generated union page")
    parser.add_argument("--update-catalog-from", type=Path, help="rebuild the compact catalog from the exact pinned schema")
    args = parser.parse_args()
    root = args.repo_root.resolve()
    try:
        if args.update_catalog_from:
            if args.check:
                raise ValueError("catalog updates cannot be combined with --check")
            payload = catalog_from_schema(args.update_catalog_from)
            destination = root / CATALOG
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
            print(f"Updated {CATALOG}: {len(payload['object_types'])} canonical names.")
            return 0
        content = markdown(root)
        if args.check:
            if not (root / OUTPUT).is_file() or (root / OUTPUT).read_text(encoding="utf-8") != content:
                raise ValueError("example object union is stale; run docs-generate")
            print("Example object union is current.")
        else:
            print(content, end="")
        return 0
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f"Object union rejected: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
