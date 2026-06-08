from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any


ORACLE_VERSION = "26.1.0"
CASE_ID = "official_1zone_uncontrolled_dynamic_diagnostic_001"


@dataclass(frozen=True)
class ProbeLane:
    lane: str
    summary_path: Path


LANES = (
    ProbeLane(
        lane="default",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="all-ctf",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-all-ctf"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
    ProbeLane(
        lane="third-order",
        summary_path=Path(
            ".runtime/official-dynamic-diagnostic-third-order"
        )
        / ORACLE_VERSION
        / CASE_ID
        / "compare/compare-summary.json",
    ),
)


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        payload = json.load(handle)
    if not isinstance(payload, dict):
        raise ValueError(f"Expected object JSON at {path}")
    return payload


def lane_row(repo_root: Path, lane: ProbeLane) -> dict[str, Any] | None:
    summary_path = repo_root / lane.summary_path
    if not summary_path.exists():
        return None
    summary = load_json(summary_path)
    bottlenecks = summary.get("bottlenecks", [])
    top = bottlenecks[0] if bottlenecks else {}
    output = top.get("output", {}) if isinstance(top, dict) else {}
    return {
        "lane": lane.lane,
        "path": str(lane.summary_path).replace("\\", "/"),
        "status": summary.get("status"),
        "zone_air_algorithm": summary.get("zone_air_algorithm", "unknown"),
        "ctf_seed_policy": summary.get("ctf_seed", {}).get("policy", "unknown"),
        "samples": summary.get("samples"),
        "top_key": output.get("key", "none"),
        "top_variable": output.get("variable", "none"),
        "top_rmse_delta_c": top.get("rmse_delta_c"),
        "top_max_abs_delta_c": top.get("max_abs_delta_c"),
        "max_abs_delta_c": summary.get("max_abs_delta_c"),
        "rmse_delta_c": summary.get("rmse_delta_c"),
    }


def build_summary(repo_root: Path) -> dict[str, Any]:
    lanes = [row for lane in LANES if (row := lane_row(repo_root, lane)) is not None]
    return {
        "schema": "rusted-energyplus.dynamic-heat-balance-probe-summary.v1",
        "oracle_version": ORACLE_VERSION,
        "case_id": CASE_ID,
        "lane_count": len(lanes),
        "lanes": lanes,
    }


def fmt_number(value: Any) -> str:
    if isinstance(value, (int, float)):
        return f"{value:.6f}"
    return "none"


def render_markdown(summary: dict[str, Any]) -> str:
    lines = [
        "# Official Dynamic Heat-Balance Probe Summary",
        "",
        f"case_id: {summary['case_id']}",
        f"oracle_version: {summary['oracle_version']}",
        "",
        "| lane | algorithm | CTF seed | top output | top RMSE | top max abs | status |",
        "|---|---|---|---|---:|---:|---|",
    ]
    for lane in summary["lanes"]:
        top_output = f"{lane['top_key']} / {lane['top_variable']}"
        lines.append(
            "| {lane} | {algorithm} | {ctf} | {top} | {rmse} | {max_abs} | {status} |".format(
                lane=lane["lane"],
                algorithm=lane["zone_air_algorithm"],
                ctf=lane["ctf_seed_policy"],
                top=top_output,
                rmse=fmt_number(lane["top_rmse_delta_c"]),
                max_abs=fmt_number(lane["top_max_abs_delta_c"]),
                status=lane["status"],
            )
        )
    lines.append("")
    return "\n".join(lines)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Summarize official dynamic heat-balance diagnostic probe lanes."
    )
    parser.add_argument(
        "--repo-root",
        type=Path,
        default=Path.cwd(),
        help="Repository root containing .runtime artifacts.",
    )
    parser.add_argument("--json-output", type=Path, default=None)
    parser.add_argument("--markdown-output", type=Path, default=None)
    return parser.parse_args()


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    summary = build_summary(repo_root)
    if not summary["lanes"]:
        print("No official dynamic heat-balance probe summaries found.")
        return 1

    markdown = render_markdown(summary)
    if args.json_output:
        write_text(args.json_output, json.dumps(summary, indent=2) + "\n")
    if args.markdown_output:
        write_text(args.markdown_output, markdown)
    print(markdown)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
