"""Read-only finite CON01 comparison; no simulation, fallback, or policy changes.

Only receipt0, completed Source outcomes are accepted. The paired physical ESO/MTR
scan retains per-series metrics, not sample histories or exported JSON trees.
"""
from pathlib import Path
import argparse
import hashlib
import json
import math
import re
import struct
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
POLICY_SHA = "d3cd3e5956ce7e02dbcebe9e8c48799673b66c7b181d932f57514a949dcbf698"
POLICY_REV = "2fffe68fb9131c80efcd3232de3a81d5290045c6"
POLICY_PATH = "energyplus_porting_plan/contracts/output-tolerances.json"
PUBLIC_CHECKPOINT_PATH = "energyplus_porting_plan/evidence/SRC-06-canonical-completed-source-activation-11/summary.json"
PUBLIC_CHECKPOINT_SHA = "49bc529e949c23e3ccf92880db6dccdef42decd183c3ed8a6fa30d32a7adba60"


def require(ok, message):
    if not ok:
        raise ValueError(message)


def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def ref(path):
    path = path.resolve()
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": digest(path), "bytes": path.stat().st_size}


def read(path):
    return json.loads(path.read_bytes())


def authenticated(path, sha):
    path = path.resolve()
    require(digest(path) == sha, f"SHA256 mismatch: {path}")
    return read(path)


def project_ref(row):
    p = ROOT / row["path"]
    require(digest(p) == row["sha256"], f"changed input contract: {p}")
    return p.resolve()


def normalized_path(value):
    return str(Path(value).resolve()).replace("\\", "/").casefold()


def option(command, name):
    require(command.count(name) == 1, f"expected exactly one {name}")
    index = command.index(name)
    require(index + 1 < len(command), f"missing {name} operand")
    return command[index + 1]


def receipt(path, sha, case, source):
    r = authenticated(path, sha)
    require(r["schema"] == "recorded-porting-command.v1", "unrecognized receipt")
    require(r["exit_code"] == 0 and r["launch_error"] is None, "requires actual successful receipt0")
    require(r["source_bytes_match_before_and_after"] is True, "unstable producer source")
    command = r["command"]
    require(normalized_path(project_ref(case["input"])) in [normalized_path(x) for x in command], "receipt input differs from case")
    require(normalized_path(option(command, "--weather")) == normalized_path(project_ref(case["weather"])), "receipt weather differs from case")
    if source:
        require(option(command, "--porting-scope") == case["scope"], "Source scope differs from case")
        output = Path(option(command, "--output-dir")).resolve()
    else:
        require(option(command, "--output-prefix") == "energyplus", "unexpected Native stream prefix")
        output = Path(option(command, "--output-directory")).resolve()
    return r, output


class PhysicalLines:
    def __init__(self, path):
        self.path = path
        self.file = path.open("rb")
        self.line = 0
        self.sha = hashlib.sha256()
        self.bytes = 0
        self.endings = {"CRLF": 0, "LF": 0}

    def next(self):
        raw = self.file.readline()
        require(raw != b"", f"unexpected EOF: {self.path}:{self.line + 1}")
        self.sha.update(raw)
        self.bytes += len(raw)
        self.line += 1
        require(raw.endswith(b"\n"), f"unterminated physical row: {self.path}:{self.line}")
        ending = "CRLF" if raw.endswith(b"\r\n") else "LF"
        self.endings[ending] += 1
        value = raw[:-2] if ending == "CRLF" else raw[:-1]
        require(b"\r" not in value, "embedded CR")
        return value.decode("utf8"), ending

    def finish(self):
        require(self.file.read(1) == b"", f"bytes after footer: {self.path}")
        self.file.close()
        return {"path": self.path.relative_to(ROOT).as_posix(), "sha256": self.sha.hexdigest(), "bytes": self.bytes,
                "physical_lines": self.line, "line_endings": self.endings}


def paired(a, b, exact=True):
    x, xe = a.next()
    y, ye = b.next()
    require(xe == ye, f"physical line endings differ at {a.line}")
    if exact:
        require(x == y, f"exact physical row differs at {a.line}: {x!r} versus {y!r}")
    return x, y


def available_variables(path):
    rows = {}
    for line in path.read_text(encoding="utf8").splitlines():
        if line.startswith("!"):
            continue
        m = re.fullmatch(r"Output:Variable,\*,(.+),hourly; !- (Zone|HVAC) (Average|Sum) \[(.*)\]", line)
        require(m is not None, f"unsupported RDD row: {line}")
        identity = m[1], m[4]
        require(identity not in rows, "duplicate RDD identity")
        rows[identity] = {"raw_line": line, "timestep_type": m[2], "store_type": m[3]}
    return rows


def norm(value):
    # Existing accepted EnergyPlus/ep_compare named-policy binding only.
    return value.strip().upper()


def binding(d, rows, kind):
    if kind == "eso":
        matches = [(i, r) for i, r in enumerate(rows)
                   if (norm(r["key"]), norm(r["variable"]), norm(r["frequency"]))
                   == (norm(d["key"]), norm(d["name"]), norm(d["frequency"]))]
    else:
        matches = [(i, r) for i, r in enumerate(rows)
                   if (r["meter"], r["units"], r["frequency"])
                   == (d["name"], d["units"], d["frequency"])]
    require(len(matches) == 1, f"not one original named policy row: {d}")
    index, row = matches[0]
    if kind == "eso" and row["units"] == "scalar (declared schedule type retained)":
        require(row["variable"] == "Schedule Value" and d["units"] == "", "unexpected scalar label binding")
    elif kind == "eso" and row["variable"] == "Zone Air Humidity Ratio":
        require(row["units"] == "kgWater/kgDryAir" and d["units"] == "", "unexpected humidity label binding")
    else:
        require(row["units"] == d["units"], "policy physical unit mismatch")
    return index, row


def completed_stream(kind, source_path, native_path, owner, policy_rows, expected_steps,
                     steps_per_hour, source_rdd, native_rdd, source_mdd, native_mdd, failures):
    if not source_path.exists() or not native_path.exists():
        require(not source_path.exists() and not native_path.exists(), "only one physical stream is absent")
        require(not policy_rows and owner["deleted_empty"] is True and owner["reported_numeric_records"] == 0
                and owner["series"] == 0 and owner["series_samples"] == [], "missing stream lacks genuine deleted-empty ownership")
        return {"stream": kind, "deleted_empty": True, "series": [], "records": 0}
    require(owner["deleted_empty"] is False, "present stream marked deleted-empty")
    s, n = PhysicalLines(source_path), PhysicalLines(native_path)
    source_version, native_version = paired(s, n, False)
    require(source_version.startswith("Program Version,") and native_version.startswith("Program Version,"), "missing Program Version header")
    dictionary, declarations, ids = [], {}, set()
    while True:
        line, _ = paired(s, n)
        if line == "End of Data Dictionary":
            break
        fields = line.split(",", 2)
        require(len(fields) == 3, "invalid dictionary row")
        rid, count = int(fields[0]), int(fields[1])
        require(rid > 0 and rid not in ids, "invalid or duplicate report ID")
        ids.add(rid)
        dictionary.append(line)
        if rid <= 6:
            continue
        pattern = (r"(\d+),(\d+),([^,]*),(.+) \[(.*)\] !(TimeStep|Hourly)" if kind == "eso"
                   else r"(\d+),(\d+),(.+) \[(.*)\] !(TimeStep|Hourly)")
        m = re.fullmatch(pattern, line)
        require(m is not None and count == 1, "finite comparison requires declared one-operand Timestep/Hourly reports")
        d = {"report_id": rid, "key": m[3] if kind == "eso" else None,
             "name": m[4] if kind == "eso" else m[3], "units": m[5] if kind == "eso" else m[4],
             "frequency": m[6] if kind == "eso" else m[5], "raw_dictionary": line}
        index, tolerance = binding(d, policy_rows, kind)
        if kind == "eso":
            identity = d["name"], d["units"]
            require(identity in source_rdd and source_rdd[identity] == native_rdd.get(identity), "selected Source/Native RDD metadata differs")
            type_authority = source_rdd[identity]
            store_type = type_authority["store_type"].lower()
        else:
            selected = f"Output:Meter,{d['name']},hourly; !- [{d['units']}]"
            require(selected in source_mdd and selected in native_mdd, "selected Source/Native MDD declaration missing")
            type_authority = {"raw_line": selected, "store_type": "Sum",
                              "authority": "accepted actual Source meter declaration StoreType::Sum; not inferred from units/name"}
            store_type = "sum"
        declarations[rid] = {**d, "policy_index": index, "policy": tolerance,
                             "type_authority": type_authority, "store_type": store_type,
                             "samples": 0, "sum_squared_delta": 0.0, "max_abs_delta": 0.0,
                             "max_relative_delta": 0.0, "max_abs_sample_index": 0,
                             "raw_operand_unequal": 0, "parsed_binary64_unequal": 0,
                             "tolerance_failures": 0, "last_stamp_index": -1}
    require(set(range(1, 7)) <= ids, "missing native clock dictionary IDs")
    require(dictionary[0] == "1,5,Environment Title[],Latitude[deg],Longitude[deg],Time Zone[],Elevation[m]", "unexpected environment definition")
    require(dictionary[1] == "2,8,Day of Simulation[],Month[],Day of Month[],DST Indicator[1=yes 0=no],Hour[],StartMinute[],EndMinute[],DayType", "unexpected interval clock definition")
    indices = [d["policy_index"] for d in declarations.values()]
    require(sorted(indices) == list(range(len(policy_rows))), "named profile not bound exactly once")
    identities = [(d["key"], d["name"], d["units"], d["frequency"]) for d in declarations.values()]
    require(len(identities) == len(set(identities)), "duplicate declared identity")
    owner_rows = owner["series_samples"]
    require(owner["series"] == len(declarations) == len(owner_rows), "actual Source catalog series count mismatch")
    require([r["report_id"] for r in owner_rows] == list(declarations), "actual Source catalog/report ID order differs")
    for r in owner_rows:
        d = declarations[r["report_id"]]
        require((r["key"], r["name"], r["units"], r["frequency"], r["kind"], r["store_type"])
                == (d["key"], d["name"], d["units"], d["frequency"].lower(), "variable" if kind == "eso" else "meter", d["store_type"]), "actual Source catalog identity/type differs")
    records = stamps = environments = 0
    environment = fields = None
    stamp_index = -1
    previous_clock = None
    structure_sha = hashlib.sha256()
    while True:
        sl, nl = paired(s, n, False)
        if sl == "End of Data" or nl == "End of Data":
            require(sl == nl == "End of Data", "End of Data placement differs")
            break
        sf, nf = sl.split(","), nl.split(",")
        require(sf[0] == nf[0], "physical report ID order differs")
        rid = int(sf[0])
        require(rid in ids, "undeclared physical report ID")
        if rid <= 6:
            require(sl == nl, "native environment/timestamp raw tuple differs")
            structure_sha.update((sl + "\n").encode())
            if rid == 1:
                require(len(sf) == 6, "invalid environment tuple")
                require(all(math.isfinite(float(x)) for x in sf[2:]), "nonfinite environment metadata")
                environments += 1
                environment = tuple(sf[1:])
                fields = previous_clock = None
            elif rid == 2:
                require(environment is not None and len(sf) == 9, "invalid interval tuple")
                fields = tuple(sf[1:])
                for x in fields[:5]:
                    int(x)
                for x in fields[5:7]:
                    require(math.isfinite(float(x)), "nonfinite clock operand")
                clock = int(fields[0]), int(fields[4]), float(fields[6])
                require(previous_clock is None or previous_clock < clock, "duplicate or nonmonotonic native interval tuple")
                previous_clock = clock
                stamps += 1
                stamp_index += 1
            else:
                raise ValueError("unsupported noninterval data clock in finite selected policy")
            continue
        require(fields is not None and len(sf) == len(nf) == 2, "missing clock or incorrect operand count")
        d = declarations[rid]
        require(d["last_stamp_index"] < stamp_index, "duplicate series sample for native interval")
        if d["frequency"] == "Hourly":
            require(float(fields[5]) == 0.0 and float(fields[6]) == 60.0, "Hourly sample lacks actual whole-hour clock")
        x, y = float(nf[1]), float(sf[1])
        require(math.isfinite(x) and math.isfinite(y), "nonfinite reported value")
        delta = abs(x - y)
        require(math.isfinite(delta), "nonfinite reported delta")
        scale = max(abs(x), abs(y))
        relative = 0.0 if scale == 0.0 else delta / scale
        if delta > d["max_abs_delta"]:
            d["max_abs_delta"], d["max_abs_sample_index"] = delta, d["samples"]
        d["max_relative_delta"] = max(d["max_relative_delta"], relative)
        # Exact accepted audit02 sequential binary64 sum, no fsum/averaging change.
        d["sum_squared_delta"] += delta * delta
        d["raw_operand_unequal"] += nf[1] != sf[1]
        d["parsed_binary64_unequal"] += struct.pack(">d", x) != struct.pack(">d", y)
        t = d["policy"]
        if not (delta <= t["atol"] or delta <= t["rtol"] * scale):
            d["tolerance_failures"] += 1
            failures.write(json.dumps({"stream": kind, "report_id": rid, "sample_index": d["samples"],
                                       "physical_line": s.line, "environment_ordinal": environments,
                                       "environment": environment, "stamp": fields,
                                       "Native_raw": nf[1], "Source_raw": sf[1], "delta": delta}, ensure_ascii=False) + "\n")
        d["samples"] += 1
        d["last_stamp_index"] = stamp_index
        records += 1
    footer, _ = paired(s, n)
    match = re.fullmatch(r" Number of Records Written=\s*(\d+)", footer)
    require(match is not None and int(match[1]) == records == owner["reported_numeric_records"], "physical footer/actual Source counter mismatch")
    reports = []
    for d, owner_row in zip(declarations.values(), owner_rows):
        expected = expected_steps if d["frequency"] == "TimeStep" else expected_steps // steps_per_hour
        require(d["samples"] == owner_row["samples"] == expected and expected > 0, "actual samples differ from whole case scope")
        d["rmse_delta"] = math.sqrt(d["sum_squared_delta"] / d["samples"])
        require(math.isfinite(d["rmse_delta"]), "nonfinite RMSE")
        d["rmse_limit_met"] = d["rmse_delta"] <= d["policy"]["rmse_max"]
        d["passed"] = d["tolerance_failures"] == 0 and d["rmse_limit_met"]
        del d["last_stamp_index"]
        reports.append(d)
    return {"stream": kind, "deleted_empty": False, "Source": s.finish(), "Native": n.finish(),
            "Source_version_line": source_version, "Native_version_line": native_version,
            "exact_dictionary": dictionary, "environments": environments, "timestamp_rows": stamps,
            "exact_ordered_environment_and_timestamp_rows_sha256": structure_sha.hexdigest(),
            "records": records, "series": reports}


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--case", required=True)
    for side in ("source", "native"):
        p.add_argument(f"--{side}-receipt", type=Path, required=True)
        p.add_argument(f"--{side}-receipt-sha256", required=True)
    p.add_argument("--output", type=Path, required=True, help="new ignored evidence directory")
    args = p.parse_args()
    policy_file = ROOT / POLICY_PATH
    policy = authenticated(policy_file, POLICY_SHA)
    historical = subprocess.check_output(["git", "show", POLICY_REV + ":" + POLICY_PATH], cwd=ROOT)
    require(historical == policy_file.read_bytes(), "policy differs from precomparison historical version")
    require(policy["schema_version"] == "con01-output-tolerances.v2", "wrong policy version")
    require(all(r["rtol"] == 0 for rows in policy["profiles"].values() for r in rows), "unexpected nonzero finite-policy rtol")
    checkpoint = authenticated(ROOT / PUBLIC_CHECKPOINT_PATH, PUBLIC_CHECKPOINT_SHA)
    scope_file = ROOT / "energyplus_porting_plan/contracts/scope.json"
    cases = {c["id"]: c for c in read(scope_file)["cases"]}
    require(args.case in cases, "not a declared finite case")
    case = cases[args.case]
    metadata_path = project_ref(case["metadata"])
    metadata = read(metadata_path)
    profile = case["scope"]
    require(policy["case_profiles"][args.case] == metadata["scope"] == profile, "profile mismatch")
    source_receipt, source_dir = receipt(args.source_receipt, args.source_receipt_sha256, case, True)
    native_receipt, native_dir = receipt(args.native_receipt, args.native_receipt_sha256, case, False)
    summary_path = source_dir / "run-summary.json"
    summary = read(summary_path)
    runtime = summary["rust_runtime"]
    require(summary["status"] == "success" and summary["exit_code"] == 0, "Source summary is unsuccessful")
    require(runtime["schema"] == "rusted-energyplus.source-runtime.v1"
            and runtime["execution"] == "actual Source ManageSimulation successful through620", "not genuine completed Source outcome")
    require(not {"result_store_json", "selected_outputs_csv", "meters_csv"} & summary["artifacts"].keys(), "legacy artifact keys on Source outcome")
    expected_steps = metadata["expected_zone_steps_excluding_warmup"]
    steps_per_hour = metadata["expected_settings"]["timesteps_per_hour"]
    require(expected_steps % steps_per_hour == 0 and runtime["native_environment_hour_scope"] == expected_steps // steps_per_hour, "actual Source whole hour scope differs")
    require(runtime["requested_hours"] in (None, expected_steps // steps_per_hour), "requested hours differ from completed scope")
    selected = [e for e in runtime["native_environment_scope"] if e["selected_by_primary_flags"]]
    require(len(selected) == metadata["expected_weather_environments"]
            and all(e["kind_of_environment"] == 3 for e in selected)
            and sum(e["total_days"] for e in selected) * 24 == runtime["native_environment_hour_scope"], "actual Source selected environment scope differs")
    require(digest(Path(summary["input"]["original"])) == case["input"]["sha256"], "Source original input artifact differs")
    source_rdd_path, native_rdd_path = source_dir / "eplusrs.rdd", native_dir / "energyplusout.rdd"
    source_mdd_path, native_mdd_path = source_dir / "eplusrs.mdd", native_dir / "energyplusout.mdd"
    source_rdd, native_rdd = available_variables(source_rdd_path), available_variables(native_rdd_path)
    source_mdd = set(source_mdd_path.read_text(encoding="utf8").splitlines())
    native_mdd = set(native_mdd_path.read_text(encoding="utf8").splitlines())
    owners = {r["stream"]: r for r in runtime["reported_streams"]}
    require(len(runtime["reported_streams"]) == len(owners) == 2 and set(owners) == {"eso", "mtr"}, "wrong actual Source stream owners")
    output = args.output.resolve()
    allowed = (ROOT / ".runtime/porting/SRC-06").resolve()
    require(output.is_relative_to(allowed) and output != allowed and not output.exists(), "output must be a new ignored SRC-06 directory")
    output.mkdir()
    failures_path = output / "all-tolerance-failures.jsonl"
    streams = []
    with failures_path.open("w", encoding="utf8", newline="\n") as failures:
        for kind, rows in (("eso", policy["profiles"][profile]), ("mtr", policy["meter_profiles"][profile])):
            source_path, native_path = source_dir / f"eplusrs.{kind}", native_dir / f"energyplusout.{kind}"
            require(normalized_path(owners[kind]["owned_path"]) == normalized_path(source_path), "summary owner path differs from physical Source stream")
            streams.append(completed_stream(kind, source_path, native_path, owners[kind], rows, expected_steps,
                                            steps_per_hour, source_rdd, native_rdd, source_mdd, native_mdd, failures))
    require(all(x.get("deleted_empty") or x["environments"] == metadata["expected_weather_environments"] for x in streams), "physical environment count differs from case")
    series = [r for stream in streams for r in stream["series"]]
    totals = {"series": len(series), "samples": sum(r["samples"] for r in series),
              "raw_operand_unequal": sum(r["raw_operand_unequal"] for r in series),
              "parsed_binary64_unequal": sum(r["parsed_binary64_unequal"] for r in series),
              "tolerance_failures": sum(r["tolerance_failures"] for r in series),
              "rmse_failures": sum(not r["rmse_limit_met"] for r in series),
              "passed": all(r["passed"] for r in series)}
    report = {"schema": "src06-completed-finite-case-strict-reported-stream-comparison.v1", "case": args.case,
              "profile": profile, "accepted": totals["passed"], "policy": ref(policy_file),
              "unchanged_policy_historical_revision": POLICY_REV, "public_operational_checkpoint": ref(ROOT / PUBLIC_CHECKPOINT_PATH),
              "case_scope": ref(scope_file), "case_metadata": ref(metadata_path),
              "Source_receipt": ref(args.source_receipt), "Native_receipt": ref(args.native_receipt),
              "Source_summary": ref(summary_path), "selected_Source_environment_scope": selected,
              "type_authorities": [ref(x) for x in (source_rdd_path, native_rdd_path, source_mdd_path, native_mdd_path)],
              "streams": streams, "totals": totals, "all_uncapped_tolerance_failures": ref(failures_path),
              "policy_metadata_notes": checkpoint["policy_metadata_notes"],
              "contract": {"exact": "Physical ordered dictionary rows/report IDs/metadata, environment and interval raw tuples, record ordering/counts, line endings, Source catalog StoreType/frequency, complete footers; only Program Version payload may differ.",
                           "numeric": "Original named per-row atol OR rtol acceptance plus original sequential binary64 sum(delta*delta)/count/sqrt RMSE. Reported decimal operands only; no conversion, interpolation, resampling, inference, tolerance change, or accumulator reconstruction.",
                           "bounded_memory": "Paired one-line scan plus declarations/actual summary/per-series scalar metrics only; failures are uncapped streamed JSONL. No per-sample history or annual JSON-tree load.",
                           "limits": "Only the selected completed finite case is compared. ESO/MTR reported values are not internal operation bit conformance or a full15matrix/gate completion. JSON/CSV own-file roundtrip and all ancillary physical streams require separate verification."},
              "author_Source": ref(Path(__file__)), "main_mutations": 0, "Cargo_Native_or_tests_executed": False,
              "new_test_bodies": 0, "policy_changes": 0, "gate_changes": 0}
    report_path = output / "comparison.json"
    report_path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf8", newline="\n")
    print(json.dumps({"comparison": ref(report_path), "totals": totals}))
    return 0 if totals["passed"] else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, KeyError, OSError) as error:
        print(f"strict comparison rejected: {error}", file=sys.stderr)
        sys.exit(2)
