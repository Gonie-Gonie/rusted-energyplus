#!/usr/bin/env python3
"""Read-only hash and scalar validation for preserved GEO-02 helper runs."""
from __future__ import annotations

from collections import Counter
from datetime import datetime
import hashlib
import json
import math
from pathlib import Path
import re
import struct

ROOT = Path(__file__).resolve().parents[2]
PIN = "6f2e40d10250a105b49966baa24d843711e61048"
HEX64 = re.compile(r"[0-9a-f]{16}\Z")
SHA256 = re.compile(r"[0-9a-f]{64}\Z")


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read(path):
    def invalid(token):
        raise ValueError("Nonstandard JSON numerical token: " + token)
    return json.loads(Path(path).read_text(encoding="utf-8"), parse_constant=invalid)


def sha(path):
    with Path(path).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def path_of(value):
    require(type(value) is str and bool(value), "Expected a nonempty path")
    path = Path(value)
    path = path.resolve() if path.is_absolute() else (ROOT / path).resolve()
    require(path.is_relative_to(ROOT), "Proof path escapes repository: " + value)
    return path


def ref(path):
    path = path_of(str(path))
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": sha(path)}


class Bindings:
    """Validate declared artifact bytes, retaining each distinct checked binding."""
    def __init__(self):
        self.checked = {}

    def check(self, value):
        require(type(value) is dict and type(value.get("path")) is str
                and type(value.get("sha256")) is str
                and bool(SHA256.fullmatch(value["sha256"])), "Malformed SHA256 binding")
        path = path_of(value["path"])
        key = (path, value["sha256"])
        if key not in self.checked:
            require(path.is_file() and sha(path) == value["sha256"],
                    "Changed or missing artifact: " + value["path"])
            self.checked[key] = {"path": path.relative_to(ROOT).as_posix(),
                                 "sha256": value["sha256"]}
        return path

    def recursive(self, value):
        if type(value) is dict:
            if "path" in value and "sha256" in value:
                self.check(value)
            for child in value.values():
                self.recursive(child)
        elif type(value) is list:
            for child in value:
                self.recursive(child)

    def report(self):
        return sorted(self.checked.values(), key=lambda item: (item["path"], item["sha256"]))


def same_binding(actual, expected):
    return (type(actual) is dict and type(expected) is dict
            and actual.get("sha256") == expected.get("sha256")
            and path_of(actual["path"]) == path_of(expected["path"]))


def exact(actual, expected):
    """Metadata equality preserving numerical IEEE bits, including signed zero."""
    if type(actual) is not type(expected):
        return False
    if type(actual) is float:
        return bits(actual) == bits(expected)
    if type(actual) is dict:
        return actual.keys() == expected.keys() and all(exact(actual[k], expected[k]) for k in actual)
    if type(actual) is list:
        return len(actual) == len(expected) and all(exact(a, b) for a, b in zip(actual, expected))
    return actual == expected


def bits(value):
    require(type(value) in (int, float), "Expected numerical observation")
    return struct.pack(">d", float(value)).hex()


def from_bits(value):
    require(type(value) is str and bool(HEX64.fullmatch(value)), "Expected lowercase 16-digit IEEE bits")
    return struct.unpack(">d", bytes.fromhex(value))[0]


def scalar(value, token):
    observed = from_bits(token)
    if math.isnan(observed):
        require(value is None or value == "NaN", "NaN observation encoding differs")
    elif math.isinf(observed):
        require(value is None or value == ("+Infinity" if observed > 0 else "-Infinity"),
                "Infinity observation encoding differs")
    else:
        require(type(value) in (int, float) and bits(value) == token,
                "Observation value and emitted bits differ")
    return observed


def value_class(value):
    if math.isnan(value):
        return "nan"
    if math.isinf(value):
        return "positive_infinity" if value > 0 else "negative_infinity"
    if value == 0:
        return "negative_zero" if math.copysign(1, value) < 0 else "positive_zero"
    return "finite"


def timestamp(value):
    require(type(value) is str, "Missing execution UTC timestamp")
    parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
    require(parsed.tzinfo is not None and parsed.utcoffset().total_seconds() == 0,
            "Execution timestamp must carry UTC")
    return parsed


class Comparison:
    """Compare actual preserved scalars only; never evaluate a geometry formula."""
    def __init__(self):
        self.checks = 0
        self.failures = Counter()
        self.examples = []
        self.errors = {}

    def failed(self, category, label, actual=None, expected=None):
        self.failures[category] += 1
        if len(self.examples) < 50:
            self.examples.append({"category": category, "field": label,
                                  "actual": actual, "reference": expected})

    def metadata(self, actual, expected, label):
        self.checks += 1
        if not exact(actual, expected):
            self.failed("metadata", label, actual, expected)

    def missing(self, label):
        self.checks += 1
        self.failed("missing_required_output", label)

    def numeric(self, actual_token, reference_token, profile, label):
        self.checks += 1
        actual, reference = from_bits(actual_token), from_bits(reference_token)
        if math.isfinite(actual) != math.isfinite(reference):
            self.failed("numerical_class", label, actual_token, reference_token)
            return
        if not math.isfinite(reference):
            if value_class(actual) != value_class(reference):
                self.failed("numerical_class", label, actual_token, reference_token)
            return
        if actual == 0 and reference == 0 and actual_token != reference_token:
            self.failed("zero_sign", label, actual_token, reference_token)
            return
        error = abs(actual - reference)
        limit = profile["absolute_tolerance"] + profile["relative_tolerance"] * abs(reference)
        metrics = self.errors.setdefault(profile["unit"], {"count": 0, "maximum_absolute_error": 0.0,
                                                           "sum_squared_error": 0.0})
        metrics["count"] += 1
        metrics["maximum_absolute_error"] = max(metrics["maximum_absolute_error"], error)
        metrics["sum_squared_error"] += error * error
        if error > limit:
            self.failed("numerical_tolerance", label, actual_token, reference_token)

    def precision(self, actual_token, reference_token, label):
        self.checks += 1
        from_bits(actual_token)
        from_bits(reference_token)
        if actual_token != reference_token:
            self.failed("cen_exact_bits", label, actual_token, reference_token)

    def report(self):
        metrics = {}
        for unit, row in self.errors.items():
            metrics[unit] = {"count": row["count"], "maximum_absolute_error": row["maximum_absolute_error"],
                             "rmse": math.sqrt(row["sum_squared_error"] / row["count"])}
        return {"checks": self.checks, "mismatch_count": sum(self.failures.values()),
                "mismatch_categories": dict(self.failures), "mismatch_examples": self.examples,
                "scalar_error_metrics": metrics}
