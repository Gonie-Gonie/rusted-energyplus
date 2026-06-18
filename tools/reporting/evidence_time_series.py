from __future__ import annotations

import json
import math
import re
import statistics
from pathlib import Path
from typing import Any


def normalized_lookup_key(value: str | None) -> str:
    return re.sub(r"\s+", " ", str(value or "").strip()).upper()


def parse_eso_variable_field(field: str) -> tuple[str, str | None, str | None]:
    value, _, frequency = field.partition("!")
    value = value.strip()
    units_match = re.search(r"\[([^\]]*)\]\s*$", value)
    units = units_match.group(1).strip() if units_match else None
    variable = re.sub(r"\s*\[[^\]]*\]\s*$", "", value).strip()
    return variable, units, frequency.strip() or None


def load_eso_series(
    eso_path: Path,
    targets: tuple[tuple[str, str, str], ...],
) -> dict[tuple[str, str], dict[str, Any]]:
    target_lookup = {
        (normalized_lookup_key(key), normalized_lookup_key(variable)): (key, variable, group)
        for key, variable, group in targets
    }
    code_lookup: dict[int, tuple[str, str]] = {}
    series = {
        (key, variable): {"key": key, "variable": variable, "group": group, "units": None, "values": []}
        for key, variable, group in targets
    }
    if not eso_path.is_file():
        return series

    in_dictionary = True
    with eso_path.open("r", encoding="utf-8", errors="replace") as handle:
        for raw_line in handle:
            line = raw_line.strip()
            if not line:
                continue
            if in_dictionary:
                if line == "End of Data Dictionary":
                    in_dictionary = False
                    continue
                parts = line.split(",")
                if len(parts) < 4:
                    continue
                try:
                    code_id = int(parts[0])
                    value_count = int(parts[1])
                except ValueError:
                    continue
                if value_count != 1:
                    continue
                key = parts[2].strip()
                variable, units, _frequency = parse_eso_variable_field(",".join(parts[3:]))
                target = target_lookup.get((normalized_lookup_key(key), normalized_lookup_key(variable)))
                if target is None:
                    continue
                target_key = (target[0], target[1])
                code_lookup[code_id] = target_key
                series[target_key]["units"] = units
                continue

            code_text, separator, value_text = line.partition(",")
            if not separator:
                continue
            try:
                code_id = int(code_text)
            except ValueError:
                continue
            target_key = code_lookup.get(code_id)
            if target_key is None:
                continue
            try:
                series[target_key]["values"].append(float(value_text.strip()))
            except ValueError:
                continue
    return series


def load_result_store_series(
    result_store_path: Path,
    targets: tuple[tuple[str, str, str], ...],
) -> dict[tuple[str, str], dict[str, Any]]:
    target_lookup = {
        (normalized_lookup_key(key), normalized_lookup_key(variable)): (key, variable, group)
        for key, variable, group in targets
    }
    series = {
        (key, variable): {"key": key, "variable": variable, "group": group, "units": None, "values": []}
        for key, variable, group in targets
    }
    if not result_store_path.is_file():
        return series
    store = json.loads(result_store_path.read_text(encoding="utf-8"))
    for row in store.get("series", []):
        key = row.get("key")
        variable = row.get("variable_name")
        target = target_lookup.get((normalized_lookup_key(key), normalized_lookup_key(variable)))
        if target is None:
            continue
        target_key = (target[0], target[1])
        values: list[float] = []
        for value in row.get("values", []):
            if value is None:
                continue
            values.append(float(value))
        series[target_key] = {
            "key": target[0],
            "variable": target[1],
            "group": target[2],
            "units": row.get("units"),
            "values": values,
        }
    return series


def downsample_indices(length: int, max_points: int, keep_indices: list[int] | None = None) -> list[int]:
    if length <= 0:
        return []
    keep = {index for index in (keep_indices or []) if 0 <= index < length}
    if length <= max_points:
        return list(range(length))
    step = max(1, math.ceil(length / max_points))
    indices = set(range(0, length, step))
    indices.add(length - 1)
    indices.update(keep)
    return sorted(indices)


def sample_row_numeric(row: dict[str, Any], *names: str) -> float | None:
    for name in names:
        value = row.get(name)
        if value is None:
            continue
        return float(value)
    return None


def build_time_series_record(
    system: str,
    key: str,
    variable: str,
    group: str,
    units: str | None,
    oracle_values: list[float],
    rust_values: list[float],
    source: str,
    max_points: int,
) -> dict[str, Any] | None:
    sample_count = min(len(oracle_values), len(rust_values))
    if sample_count <= 0:
        return None

    oracle = oracle_values[:sample_count]
    rust = rust_values[:sample_count]
    deltas = [abs(left - right) for left, right in zip(oracle, rust)]
    max_delta_index = max(range(sample_count), key=lambda index: deltas[index])
    rmse = math.sqrt(statistics.fmean(delta * delta for delta in deltas))
    mean_abs = statistics.fmean(deltas)
    indices = downsample_indices(sample_count, max_points, [0, max_delta_index, sample_count - 1])
    return {
        "system": system,
        "group": group,
        "key": key,
        "variable": variable,
        "units": units or "",
        "sample_count": sample_count,
        "plotted_points": len(indices),
        "source": source,
        "max_abs_delta": deltas[max_delta_index],
        "mean_abs_delta": mean_abs,
        "rmse_delta": rmse,
        "max_delta_index": max_delta_index,
        "x": indices,
        "oracle": [oracle[index] for index in indices],
        "rust": [rust[index] for index in indices],
        "delta": [deltas[index] for index in indices],
    }
