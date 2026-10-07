#!/usr/bin/env python3
"""Verify unchanged geometry input families in reviewed GEO-01 observations.

This compares recorded inputs and coordinates, not new GEO-02 numerical outputs.
Historical commands, implementation revisions and evidence remain unchanged.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / "energyplus_porting_plan"
REVIEW = PLAN / "evidence/GEO-01/independent-review.json"
REVIEW_SHA = "9b57266ab1f1203ff97cdfb80942a1e9aa09fdaf09c727b7b4ad48a5a78dbc2d"
SELECTED = {"A-24H", "A-72H", "B-NOLIMIT-24H", "B-FLOW-24H",
            "B-CAPACITY-24H", "B-BOTH-24H", "B-BOTH-72H"}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def ref(path: Path) -> dict:
    with path.open("rb") as stream:
        digest = hashlib.file_digest(stream, "sha256").hexdigest()
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": digest}


def verify(binding: dict) -> Path:
    path = (ROOT / binding["path"]).resolve()
    require(path.is_relative_to(ROOT), "Evidence path escapes repository")
    require(ref(path) == binding, f"Evidence bytes differ: {path}")
    return path


def read(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def main() -> None:
    require(ref(REVIEW)["sha256"] == REVIEW_SHA, "Reviewed GEO-01 anchor changed")
    review = read(REVIEW)
    require(review["status"] == "pass", "Prior independent review did not pass")
    report_binding = review["reviewed_comparison_report"]
    report = read(verify(report_binding))
    matrix_binding = report["producer_matrix"]
    matrix = read(verify(matrix_binding))
    require(matrix["kind"] == "production", "Expected actual historical production")
    require({case["case_id"] for case in matrix["cases"]} == SELECTED, "Historical case set differs")
    families: dict[str, str] = {}
    rows = []
    for case in matrix["cases"]:
        verify(case["input"])
        full = [run for run in case["runs"] if run["trace_level"] == "full"]
        require(len(full) == 1, "Expected exactly one retained Full projection")
        path = ROOT / full[0]["output_directory"] / "compiled-geometry.json"
        binding = next(item for item in full[0]["artifacts"] if item["path"] == path.relative_to(ROOT).as_posix())
        geometry = read(verify(binding))
        require(geometry["preparation_only"] is True and geometry["physics_executed"] is False,
                "Coordinate projection must retain its preparation-only meaning")
        projection = {key: geometry[key] for key in ("parsed_input", "settings", "zones", "surfaces")}
        require(geometry["parsed_input"]["objects"].get("GeometryTransform") is None,
                "GeometryTransform is excluded")
        require(geometry["parsed_input"]["objects"].get("Compliance:Building") is None,
                "Appendix G control is excluded")
        encoded = json.dumps(projection, sort_keys=True, separators=(",", ":"), allow_nan=False).encode("utf-8")
        digest = hashlib.sha256(encoded).hexdigest()
        family = case["scope"]
        require(family in {"A", "B"}, "Unknown geometry family")
        require(families.setdefault(family, digest) == digest, f"Geometry differs within family {family}")
        rows.append({"case_id": case["case_id"], "scope": family, "input": case["input"],
                     "historical_compile_projection": binding, "canonical_projection_sha256": digest})
    print(json.dumps({
        "schema": "geo02-recorded-input-families.v1", "status": "pass",
        "tool": ref(Path(__file__)), "reviewed_GEO01_anchor": ref(REVIEW),
        "historical_GEO01_comparison": report_binding, "historical_producer_matrix": matrix_binding,
        "historical_implementation_commit": report["implementation_commit"],
        "canonical_encoding": "UTF8 JSON, sorted keys, compact separators, finite values",
        "family_projection_hashes": families, "cases": rows,
        "scope": "Exact parsed geometry objects, settings, own IDs/order/bindings and ordered coordinate values/bits within A and B families",
        "claim_limits": ["No missing GEO-02 normal or centroid field is inferred.",
                         "No new physical CLI execution occurred.",
                         "Independent initialization and absence of orientation schedules still require the GEO-02 source/input review.",
                         "Historical GEO-01 physical coverage does not imply new per-limit GEO-02 consumption coverage."],
        "gates_updated": False,
    }, indent=2, allow_nan=False))


if __name__ == "__main__":
    main()
