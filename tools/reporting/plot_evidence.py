from __future__ import annotations

import argparse
import json
import math
import shutil
from pathlib import Path
from typing import Any

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Export evidence-pack plots as PNG assets.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--version", default="0.1.0")
    parser.add_argument("--output-dir", type=Path)
    parser.add_argument("--latest-dir", type=Path)
    return parser.parse_args()


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def evidence_root(repo_root: Path, version: str) -> Path:
    return repo_root / ".runtime" / "release-evidence" / f"v{version}"


def ensure_dir(path: Path) -> None:
    path.mkdir(parents=True, exist_ok=True)


def style_axis(ax: Any, grid_axis: str = "y") -> None:
    ax.grid(axis=grid_axis, color="#e3e7ed", linewidth=0.8)
    ax.set_axisbelow(True)
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.spines["left"].set_color("#9aa7b5")
    ax.spines["bottom"].set_color("#9aa7b5")
    ax.tick_params(axis="x", colors="#17212b", labelsize=8)
    ax.tick_params(axis="y", colors="#5b6775", labelsize=8)


def save_figure(fig: Any, output_dir: Path, latest_dir: Path, name: str, plots: list[dict[str, Any]]) -> None:
    path = output_dir / name
    latest_path = latest_dir / name
    fig.savefig(path, dpi=180, bbox_inches="tight")
    shutil.copy2(path, latest_path)
    plt.close(fig)
    plots.append({"name": name, "path": path.as_posix(), "latest_path": latest_path.as_posix()})


def coverage_status_plot(evidence: dict[str, Any]) -> Any:
    coverage = evidence.get("coverage_snapshot", {})
    labels = ["conformance", "diagnostic", "baseline"]
    values = [
        int(coverage.get("conformance_output_variable_count", 0)),
        int(coverage.get("diagnostic_output_variable_count", 0)),
        int(coverage.get("baseline_output_variable_count", 0)),
    ]
    colors = ["#2f6f9f", "#c77d1a", "#697789"]
    fig, ax = plt.subplots(figsize=(7.0, 3.6))
    ax.bar(labels, values, color=colors, width=0.52)
    max_value = max(values, default=1)
    for index, value in enumerate(values):
        ax.text(index, value + max_value * 0.025, str(value), ha="center", va="bottom", fontsize=9)
    ax.set_ylim(0, max_value * 1.16)
    ax.set_ylabel("Tracked output variables")
    ax.set_title("Variable Coverage Status", loc="left", fontweight="bold")
    style_axis(ax)
    return fig


def declared_vs_passed_plot(evidence: dict[str, Any]) -> Any:
    coverage = evidence.get("coverage_snapshot", {})
    labels = ["declared numerical", "passed evidence"]
    values = [
        int(coverage.get("declared_numerical_series_count", 0)),
        int(coverage.get("passed_numerical_series_count", 0)),
    ]
    fig, ax = plt.subplots(figsize=(7.0, 3.6))
    ax.bar(labels, values, color=["#c9d8e8", "#1f7a5a"], width=0.48)
    max_value = max(values, default=1)
    for index, value in enumerate(values):
        ax.text(index, value + max_value * 0.025, str(value), ha="center", va="bottom", fontsize=9)
    ax.set_ylim(0, max_value * 1.16)
    ax.set_ylabel("Series count")
    ax.set_title("Declared Numerical Scope vs Passed Evidence", loc="left", fontweight="bold")
    style_axis(ax)
    return fig


def find_time_series(evidence: dict[str, Any], system_prefix: str, variable: str) -> dict[str, Any] | None:
    for record in evidence.get("time_series", []):
        if str(record.get("system", "")).startswith(system_prefix) and record.get("variable") == variable:
            return record
    return None


def dynamic_compare_summary(repo_root: Path, evidence: dict[str, Any]) -> dict[str, Any]:
    dynamic = evidence.get("active_dynamic_diagnostic") or {}
    digest = dynamic.get("source_digest_json")
    if not digest:
        return {}
    digest_path = repo_root / str(digest)
    summary_path = digest_path.parent / "compare-summary.json"
    if not summary_path.is_file():
        return {}
    return load_json(summary_path)


def sample_value(row: dict[str, Any], *names: str) -> float | None:
    for name in names:
        value = row.get(name)
        if value is not None:
            return float(value)
    return None


def dynamic_series(summary: dict[str, Any], key: str | None, variable: str) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for series in summary.get("series", []):
        output = series.get("output") or {}
        if output.get("variable") != variable:
            continue
        if key is not None and output.get("key") != key:
            continue
        rows.append(series)
    return rows


def series_rows(series: dict[str, Any]) -> tuple[list[int], list[float], list[float], list[float]]:
    x_values: list[int] = []
    oracle: list[float] = []
    rust: list[float] = []
    delta: list[float] = []
    for row in series.get("sample_rows", []):
        oracle_value = sample_value(row, "oracle", "oracle_c", "oracle_w", "oracle_value")
        rust_value = sample_value(row, "rust", "rust_c", "rust_w", "rust_value")
        if oracle_value is None or rust_value is None:
            continue
        index = int(row.get("index", len(x_values)))
        x_values.append(index)
        oracle.append(oracle_value)
        rust.append(rust_value)
        delta.append(abs(oracle_value - rust_value))
    return x_values, oracle, rust, delta


def plot_record(record: dict[str, Any], title: str, ylabel: str | None = None) -> Any:
    x_values = record.get("x", [])
    fig, (value_ax, delta_ax) = plt.subplots(
        2,
        1,
        figsize=(8.2, 4.8),
        sharex=True,
        gridspec_kw={"height_ratios": [2.7, 1.0], "hspace": 0.08},
    )
    value_ax.plot(x_values, record.get("oracle", []), label="Oracle", color="#1f4e79", linewidth=1.2)
    value_ax.plot(x_values, record.get("rust", []), label="Rust", color="#d97706", linewidth=1.0, linestyle="--")
    delta_ax.plot(x_values, record.get("delta", []), label="Abs delta", color="#7c4d9e", linewidth=1.0)
    value_ax.set_title(title, loc="left", fontweight="bold")
    value_ax.set_ylabel(ylabel or record.get("units") or "value")
    delta_ax.set_ylabel("abs delta")
    delta_ax.set_xlabel("Sample index")
    value_ax.legend(loc="upper right", frameon=False, ncol=2)
    delta_ax.legend(loc="upper right", frameon=False)
    style_axis(value_ax, "x")
    style_axis(delta_ax, "x")
    return fig


def one_zone_surface_temperature_plot(summary: dict[str, Any]) -> Any:
    matches = dynamic_series(summary, None, "Surface Inside Face Temperature")
    selected: list[dict[str, Any]] = []
    wanted = [("FLR", "floor"), ("ROOF", "roof"), ("WALL", "wall")]
    for token, _label in wanted:
        match = next(
            (
                series
                for series in matches
                if token in str((series.get("output") or {}).get("key", "")).upper()
            ),
            None,
        )
        if match is not None:
            selected.append(match)
    fig, ax = plt.subplots(figsize=(8.2, 4.6))
    for series in selected:
        output = series.get("output") or {}
        key = str(output.get("key", "surface"))
        x_values, oracle, rust, _delta = series_rows(series)
        step = max(1, math.ceil(len(x_values) / 720)) if x_values else 1
        sample = slice(None, None, step)
        ax.plot(x_values[sample], oracle[sample], label=f"{key} oracle", linewidth=1.1)
        ax.plot(x_values[sample], rust[sample], label=f"{key} rust", linewidth=0.9, linestyle="--")
    ax.set_title("1Zone Representative Surface Inside Face Temperature", loc="left", fontweight="bold")
    ax.set_xlabel("Sample index")
    ax.set_ylabel("C")
    ax.legend(loc="upper right", frameon=False, fontsize=7, ncol=2)
    style_axis(ax, "x")
    fig.tight_layout()
    return fig


def one_zone_conduction_delta_heatmap(summary: dict[str, Any]) -> Any:
    matches = dynamic_series(summary, None, "Surface Inside Face Conduction Heat Transfer Rate")
    matches = [series for series in matches if str((series.get("output") or {}).get("key", "")).startswith("ZN001:")]
    labels: list[str] = []
    matrix: list[list[float]] = []
    for series in matches[:12]:
        output = series.get("output") or {}
        x_values, _oracle, _rust, delta = series_rows(series)
        if not delta:
            continue
        step = max(1, math.ceil(len(delta) / 480))
        labels.append(str(output.get("key", "surface")))
        matrix.append(delta[::step])
    if not matrix:
        matrix = [[0.0]]
        labels = ["missing"]
    fig, ax = plt.subplots(figsize=(8.4, max(2.8, 0.38 * len(labels) + 1.2)))
    image = ax.imshow(matrix, aspect="auto", cmap="magma")
    ax.set_title("1Zone Surface Conduction Absolute Delta Heatmap", loc="left", fontweight="bold")
    ax.set_xlabel("Downsampled sample index")
    ax.set_ylabel("Surface key")
    ax.set_yticks(range(len(labels)), labels)
    ax.tick_params(axis="y", labelsize=7)
    fig.colorbar(image, ax=ax, fraction=0.028, pad=0.02, label="abs delta W")
    fig.tight_layout()
    return fig


def one_zone_delta_histogram(summary: dict[str, Any]) -> Any:
    targets = [
        ("ZONE ONE", "Zone Mean Air Temperature", "MAT"),
        (None, "Surface Inside Face Temperature", "Surface IFT"),
        (None, "Surface Inside Face Conduction Heat Transfer Rate", "Surface conduction"),
    ]
    fig, ax = plt.subplots(figsize=(8.2, 4.4))
    colors = ["#2f6f9f", "#c77d1a", "#7c4d9e"]
    for color, (key, variable, label) in zip(colors, targets, strict=True):
        values: list[float] = []
        for series in dynamic_series(summary, key, variable):
            _x_values, _oracle, _rust, delta = series_rows(series)
            values.extend(delta)
        if values:
            ax.hist(values, bins=60, alpha=0.42, label=label, color=color)
    ax.set_title("1Zone Delta Distribution", loc="left", fontweight="bold")
    ax.set_xlabel("Absolute delta")
    ax.set_ylabel("Sample count")
    ax.legend(loc="upper right", frameon=False)
    style_axis(ax)
    fig.tight_layout()
    return fig


def ideal_loads_rates_plot(evidence: dict[str, Any]) -> Any:
    heating = find_time_series(evidence, "IdealLoadsAirSystem", "Zone Ideal Loads Zone Total Heating Rate")
    cooling = find_time_series(evidence, "IdealLoadsAirSystem", "Zone Ideal Loads Zone Total Cooling Rate")
    fig, ax = plt.subplots(figsize=(8.2, 4.2))
    if heating:
        ax.plot(heating["x"], heating["oracle"], label="Oracle heating", color="#1f4e79", linewidth=1.2)
        ax.plot(heating["x"], heating["rust"], label="Rust heating", color="#d97706", linewidth=1.0, linestyle="--")
    if cooling:
        ax.plot(cooling["x"], cooling["oracle"], label="Oracle cooling", color="#365f3f", linewidth=1.2)
        ax.plot(cooling["x"], cooling["rust"], label="Rust cooling", color="#7c4d9e", linewidth=1.0, linestyle="--")
    ax.set_title("IdealLoads No-OA Zone Total Rates", loc="left", fontweight="bold")
    ax.set_xlabel("Sample index")
    ax.set_ylabel("W")
    ax.legend(loc="upper right", frameon=False, ncol=2)
    style_axis(ax, "x")
    return fig


def ideal_loads_node_state_plot(evidence: dict[str, Any]) -> Any:
    temp = find_time_series(evidence, "IdealLoadsAirSystem", "System Node Temperature")
    flow = find_time_series(evidence, "IdealLoadsAirSystem", "System Node Mass Flow Rate")
    fig, axes = plt.subplots(2, 1, figsize=(8.2, 5.0), sharex=True)
    for ax, record, ylabel, title in [
        (axes[0], temp, "C", "Supply Node Temperature"),
        (axes[1], flow, "kg/s", "Supply Node Mass Flow Rate"),
    ]:
        if record:
            ax.plot(record["x"], record["oracle"], label="Oracle", color="#1f4e79", linewidth=1.2)
            ax.plot(record["x"], record["rust"], label="Rust", color="#d97706", linewidth=1.0, linestyle="--")
        ax.set_title(title, loc="left", fontsize=10, fontweight="bold")
        ax.set_ylabel(ylabel)
        ax.legend(loc="upper right", frameon=False, ncol=2)
        style_axis(ax, "x")
    axes[1].set_xlabel("Sample index")
    fig.suptitle("IdealLoads No-OA Supply Node State", x=0.01, ha="left", fontweight="bold")
    fig.tight_layout()
    return fig


def stage_timing_plot(evidence: dict[str, Any]) -> Any:
    cases = evidence.get("cases", [])
    phase_names: list[str] = []
    for case in cases:
        for phase in case.get("timing", {}).get("phases", []):
            name = str(phase.get("name", ""))
            if name and name not in phase_names:
                phase_names.append(name)
    if not phase_names:
        phase_names = ["rust_compare_report", "energyplus_oracle"]
    labels = [case.get("case_id", f"C{index + 1}") for index, case in enumerate(cases)]
    fig, ax = plt.subplots(figsize=(8.4, 4.5))
    bottoms = [0.0 for _ in cases]
    colors = ["#2f6f9f", "#c77d1a", "#7c4d9e", "#3d7f5f", "#697789", "#9b6fbd"]
    for phase_index, phase_name in enumerate(phase_names):
        values = []
        for case in cases:
            value = 0.0
            for phase in case.get("timing", {}).get("phases", []):
                if phase.get("name") == phase_name:
                    value = float(phase.get("wall_seconds") or 0.0)
                    break
            values.append(value)
        ax.bar(labels, values, bottom=bottoms, label=phase_name, color=colors[phase_index % len(colors)])
        bottoms = [bottom + value for bottom, value in zip(bottoms, values)]
    ax.set_title("Compare Stage Timing Breakdown", loc="left", fontweight="bold")
    ax.set_ylabel("Seconds")
    ax.tick_params(axis="x", rotation=35)
    ax.legend(loc="upper right", frameon=False, fontsize=7)
    style_axis(ax)
    fig.tight_layout()
    return fig


def ideal_loads_branch_rows(repo_root: Path, version: str) -> list[dict[str, Any]]:
    index_path = evidence_root(repo_root, version) / "conformance-index-report.json"
    if not index_path.is_file():
        return []
    index = load_json(index_path)
    rows: list[dict[str, Any]] = []
    for case in index.get("cases", []):
        case_id = str(case.get("case_id", ""))
        if not case_id.startswith("ideal_loads_"):
            continue
        outputs = case.get("outputs", [])
        meters = case.get("meters", [])
        conformance = sum(1 for output in outputs if output.get("level") == "conformance")
        diagnostic = sum(1 for output in outputs if output.get("level") == "diagnostic")
        baseline = sum(1 for output in outputs if output.get("level") == "baseline")
        conformance += sum(1 for meter in meters if meter.get("level") == "conformance")
        diagnostic += sum(1 for meter in meters if meter.get("level") == "diagnostic")
        branch = case_id
        for suffix in (
            "_conformance_candidate_001",
            "_conformance_001",
            "_diagnostic_001",
            "_candidate_001",
            "_001",
        ):
            branch = branch.replace(suffix, "")
        rows.append(
            {
                "branch": branch.removeprefix("ideal_loads_").replace("_", " "),
                "claim": 1 if case.get("conformance_claim") else 0,
                "conformance": conformance,
                "diagnostic": diagnostic,
                "baseline": baseline,
                "meters": len(meters),
            }
        )
    return rows


def ideal_loads_branch_heatmap_plot(repo_root: Path, version: str) -> Any:
    rows = ideal_loads_branch_rows(repo_root, version)
    labels = [row["branch"][:34] for row in rows] or ["missing"]
    columns = ["claim", "conf", "diag", "base", "meter"]
    matrix = [
        [row["claim"], row["conformance"], row["diagnostic"], row["baseline"], row["meters"]]
        for row in rows
    ] or [[0, 0, 0, 0, 0]]
    height = min(10.0, max(3.2, 0.34 * len(labels) + 1.2))
    fig, ax = plt.subplots(figsize=(8.2, height))
    image = ax.imshow(matrix, aspect="auto", cmap="YlGnBu")
    ax.set_xticks(range(len(columns)), columns)
    ax.set_yticks(range(len(labels)), labels)
    ax.set_title("IdealLoads Branch Status Heatmap", loc="left", fontweight="bold")
    ax.tick_params(axis="y", labelsize=6.4)
    for y, row in enumerate(matrix):
        for x, value in enumerate(row):
            ax.text(x, y, str(value), ha="center", va="center", fontsize=6.4, color="#17212b")
    fig.colorbar(image, ax=ax, fraction=0.028, pad=0.02)
    fig.tight_layout()
    return fig


def ideal_loads_meter_comparison_plot(repo_root: Path) -> Any:
    summaries = sorted(
        (repo_root / ".runtime").glob("ideal-loads-*facility-meter*/26.1.0/*/compare/compare-summary.json")
    )
    rows: list[dict[str, Any]] = []
    for path in summaries:
        summary = load_json(path)
        for meter in summary.get("meter_series", []):
            rows.append(
                {
                    "label": f"{meter.get('name')} {meter.get('frequency')}",
                    "max_abs_delta": float(meter.get("max_abs_delta") or 0.0),
                    "status": meter.get("status"),
                }
            )
    fig, ax = plt.subplots(figsize=(8.4, max(3.2, 0.34 * len(rows) + 1.2)))
    labels = [row["label"] for row in rows] or ["missing"]
    values = [row["max_abs_delta"] for row in rows] or [0.0]
    y_values = list(range(len(labels)))
    ax.barh(y_values, values, color="#3d7f5f", edgecolor="none")
    max_value = max(values, default=1.0) or 1.0
    for y, value in zip(y_values, values):
        ax.text(value + max_value * 0.012, y, f"{value:.3g}", va="center", fontsize=7)
    ax.set_yticks(y_values, labels)
    ax.invert_yaxis()
    ax.set_xlabel("Max absolute delta (J)")
    ax.set_title("IdealLoads Facility Meter Comparison", loc="left", fontweight="bold")
    style_axis(ax, "x")
    fig.tight_layout()
    return fig


def build_plots(repo_root: Path, version: str, output_dir: Path, latest_dir: Path) -> dict[str, Any]:
    ensure_dir(output_dir)
    ensure_dir(latest_dir)
    evidence = load_json(evidence_root(repo_root, version) / "numeric-conformance-evidence.json")
    plots: list[dict[str, Any]] = []
    save_figure(coverage_status_plot(evidence), output_dir, latest_dir, "coverage_status_bar.png", plots)
    save_figure(declared_vs_passed_plot(evidence), output_dir, latest_dir, "declared_vs_passed_series.png", plots)
    mat = find_time_series(evidence, "1Zone", "Zone Mean Air Temperature")
    if mat:
        save_figure(
            plot_record(mat, "1Zone Zone Mean Air Temperature", "C"),
            output_dir,
            latest_dir,
            "1zone_zone_mean_air_temperature.png",
            plots,
        )
    dynamic_summary = dynamic_compare_summary(repo_root, evidence)
    if dynamic_summary:
        save_figure(
            one_zone_surface_temperature_plot(dynamic_summary),
            output_dir,
            latest_dir,
            "1zone_surface_inside_face_temperature.png",
            plots,
        )
        save_figure(
            one_zone_conduction_delta_heatmap(dynamic_summary),
            output_dir,
            latest_dir,
            "1zone_surface_conduction_delta_heatmap.png",
            plots,
        )
        save_figure(
            one_zone_delta_histogram(dynamic_summary),
            output_dir,
            latest_dir,
            "1zone_delta_histogram.png",
            plots,
        )
    save_figure(ideal_loads_rates_plot(evidence), output_dir, latest_dir, "ideal_loads_zone_total_rates.png", plots)
    save_figure(ideal_loads_node_state_plot(evidence), output_dir, latest_dir, "ideal_loads_supply_node_state.png", plots)
    save_figure(
        ideal_loads_branch_heatmap_plot(repo_root, version),
        output_dir,
        latest_dir,
        "ideal_loads_branch_status_heatmap.png",
        plots,
    )
    save_figure(
        ideal_loads_meter_comparison_plot(repo_root),
        output_dir,
        latest_dir,
        "ideal_loads_meter_comparison.png",
        plots,
    )
    save_figure(stage_timing_plot(evidence), output_dir, latest_dir, "stage_timing_stacked_bar.png", plots)
    summary = {
        "schema_version": 1,
        "version": version,
        "source_json": (evidence_root(repo_root, version) / "numeric-conformance-evidence.json").as_posix(),
        "output_dir": output_dir.as_posix(),
        "latest_dir": latest_dir.as_posix(),
        "plots": plots,
    }
    summary_path = output_dir / "plot-evidence-summary.json"
    summary_path.write_text(json.dumps(summary, indent=2), encoding="utf-8")
    shutil.copy2(summary_path, latest_dir / "plot-evidence-summary.json")
    return summary


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    output_dir = args.output_dir or (evidence_root(repo_root, args.version) / "plots")
    latest_dir = args.latest_dir or (repo_root / "reports" / "latest" / "plots")
    summary = build_plots(repo_root, args.version, output_dir.resolve(), latest_dir.resolve())
    print("Evidence plots")
    print(f"  version: {summary['version']}")
    print(f"  plots: {len(summary['plots'])}")
    print(f"  output_dir: {summary['output_dir']}")
    print(f"  latest_dir: {summary['latest_dir']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
