from __future__ import annotations

import argparse
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Build a stability evidence summary.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--version", default="0.1.0")
    return parser.parse_args()


def evidence_root(repo_root: Path, version: str) -> Path:
    return repo_root / ".runtime" / "release-evidence" / f"v{version}"


def load_json(path: Path) -> dict[str, Any]:
    if not path.is_file():
        return {}
    return json.loads(path.read_text(encoding="utf-8"))


def result_store_guard(repo_root: Path) -> dict[str, Any]:
    path = (
        repo_root
        / ".runtime"
        / "ideal-loads-no-oa-sensible"
        / "26.1.0"
        / "ideal_loads_no_oa_sensible_conformance_001"
        / "compare"
        / "rust-result-store.json"
    )
    store = load_json(path)
    return {
        "path": path.as_posix(),
        "duplicate_guard": store.get("duplicate_guard"),
        "diagnostic_count": store.get("diagnostic_count"),
        "diagnostics": store.get("diagnostics", []),
    }


def build_summary(repo_root: Path, version: str) -> dict[str, Any]:
    root = evidence_root(repo_root, version)
    numeric = load_json(root / "numeric-conformance-evidence.json")
    support = load_json(root / "support-coverage-report.json")
    guard = result_store_guard(repo_root)
    ideal = next(
        (
            case
            for case in numeric.get("cases", [])
            if case.get("case_id") == "ideal_loads_no_oa_sensible_conformance_001"
        ),
        {},
    )
    inactive = (ideal.get("raw_summary") or {}).get("inactive_branches", [])
    tests = [
        {
            "test": "repeated-run identical summary hash",
            "case": "1Zone + IdealLoads",
            "expected": "identical compare-summary hash across repeated deterministic runs",
            "observed": "not captured by current evidence generator",
            "status": "pending",
        },
        {
            "test": "unsupported IdealLoads branches are not silently approximated",
            "case": ideal.get("case_id", "ideal_loads_no_oa_sensible_conformance_001"),
            "expected": "inactive branches remain outside the no-OA conformance claim",
            "observed": ", ".join(inactive),
            "status": "documented" if inactive else "pending",
        },
        {
            "test": "duplicate ResultStore handle guard",
            "case": "IdealLoads no-OA result store",
            "expected": "duplicate handles surface through ep_runtime::ResultStore::diagnostics",
            "observed": guard.get("duplicate_guard"),
            "status": "pass" if guard.get("duplicate_guard") == "ep_runtime::ResultStore::diagnostics" else "pending",
        },
        {
            "test": "missing node reference returns typed diagnostic",
            "case": "broken fixture",
            "expected": "object/source diagnostic, no panic",
            "observed": "fixture not generated in current evidence pack",
            "status": "pending",
        },
        {
            "test": "unavailable output variable returns typed diagnostic",
            "case": "broken output request fixture",
            "expected": "typed unavailable-output diagnostic, no panic",
            "observed": "fixture not generated in current evidence pack",
            "status": "pending",
        },
        {
            "test": "non-finite guard",
            "case": "runtime numeric guard fixture",
            "expected": "typed diagnostic or guard failure, no panic",
            "observed": "fixture not generated in current evidence pack",
            "status": "pending",
        },
        {
            "test": "no broad compatibility implication from support coverage",
            "case": "support coverage report",
            "expected": "known gaps explicitly state unsupported domains",
            "observed": "; ".join(support.get("known_gaps", [])),
            "status": "pass" if support.get("known_gaps") else "pending",
        },
    ]
    return {
        "schema_version": 1,
        "version": version,
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "claim_boundary": "Stability evidence describes failure behavior only; it does not promote compatibility.",
        "aggregate": {
            "test_count": len(tests),
            "pass_count": sum(1 for test in tests if test["status"] == "pass"),
            "documented_count": sum(1 for test in tests if test["status"] == "documented"),
            "pending_count": sum(1 for test in tests if test["status"] == "pending"),
            "status": "partial" if any(test["status"] == "pending" for test in tests) else "pass",
        },
        "tests": tests,
        "artifacts": {
            "json": f".runtime/release-evidence/v{version}/stability-summary.json",
        },
    }


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    summary = build_summary(repo_root, args.version)
    output_path = evidence_root(repo_root, args.version) / "stability-summary.json"
    output_path.write_text(json.dumps(summary, indent=2), encoding="utf-8")
    print("Stability summary")
    print(f"  status: {summary['aggregate']['status']}")
    print(f"  tests: {summary['aggregate']['test_count']}")
    print(f"  json: {output_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
