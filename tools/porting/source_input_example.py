"""Prepare controlled example weather or compare existing physical engine outputs.

No engine is launched by this tool. The composed example has its own identity;
the fixed CON-01 cases and the original B policy/kernel are never changed.
"""
from pathlib import Path
from datetime import datetime
import argparse
import hashlib
import io
import json
import math
import re
import sys

import check_completed_reported_streams as c

ROOT = c.ROOT
DEFAULT_CASE = Path(__file__).resolve().parents[2] / "data/examples/source-input-domains/composed-b102.json"
GOLDEN_SHA = "c184b947cd34d41c6d6474d63d66dbb82bc0e6cae4c888edcf837348282bce7f"
WEATHER_SHA = "134042fbd42aa79d0886d45d852d9a5a60d53b70ffa34b2485ac8f350bc92059"
INPUT_SHA = "04f14fda1e8322d3f156366b832b2d265b53995671160d0665d05a54f856b1c4"
KERNEL_SHA = "f5669cd0217a3f953e08f2c96fd09fcf9067be85f6a9257b3dc1011e9297f833"
MARKER = re.compile(r"\*\*\s+(?:Warning|Continue|Severe|Fatal)\s+\*\*")
require = c.require


def path(value, cwd=ROOT):
    p = Path(value)
    return (p if p.is_absolute() else Path(cwd) / p).resolve()


def reference(p):
    p = path(p)
    return {"path": str(p), "sha256": c.digest(p), "bytes": p.stat().st_size}


def pinned(row):
    p = path(row["path"])
    require(c.digest(p) == row["sha256"], "changed referenced bytes: " + str(p))
    if "bytes" in row:
        require(p.stat().st_size == row["bytes"], "referenced byte count differs")
    return p


def load_case(p):
    p = path(p)
    case = c.read(p)
    require(case["schema"] == "source-input-example.v1" and case["id"] == "COMPOSED-B102-LEAP12"
            and case["fixed_conformance_case"] is False, "unsupported example identity")
    require(case["input_sha256"] == INPUT_SHA and case["weather_sha256"] == WEATHER_SHA
            and case["golden_weather_sha256"] == GOLDEN_SHA and case["policy_sha256"] == c.POLICY_SHA
            and case["comparison_kernel_sha256"] == KERNEL_SHA, "example authority differs")
    expected = {"scope": "B", "policy_profile": "B", "date": "2016-02-29", "weekday": "Monday",
                "hours": 24, "steps_per_hour": 12, "zone_steps": 288, "series": 102,
                "numeric_records": 29376, "meter_series": 0}
    require(all(case.get(k) == v for k, v in expected.items()), "example physical scope differs")
    input_path = path(case["input"], p.parent)
    require(c.digest(input_path) == INPUT_SHA, "example IDF bytes differ")
    require(c.digest(c.__file__) == KERNEL_SHA, "original comparison kernel bytes changed")
    require(c.digest(ROOT / c.POLICY_PATH) == c.POLICY_SHA, "original policy bytes changed")
    return case, input_path


def synthetic_weather(raw):
    """Literal deterministic recipe. The public caller first checks Golden SHA."""
    rows = raw.splitlines(keepends=True)
    require(len(rows) == 8768 and all(r.endswith(b"\r\n") for r in rows), "expected original 8760-hour CRLF EPW")
    require(rows[4] == b"HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0\r\n"
            and rows[7] == b"DATA PERIODS,1,1,Data,Sunday, 1/ 1,12/31\r\n", "original EPW headers differ")
    result = rows[:8]
    result[4] = b"HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0\r\n"
    result[7] = b"DATA PERIODS,1,1,Data,Friday,1/1/2016,12/31/2016\r\n"
    feb28 = []
    for row in rows[8:]:
        fields = row.split(b",", 4)
        require(len(fields) == 5, "malformed weather row")
        month, day, hour = map(int, fields[1:4])
        require((month, day) != (2, 29), "original weather already has February 29")
        rewritten = b"2016," + row.split(b",", 1)[1]
        result.append(rewritten)
        if (month, day) == (2, 28):
            feb28.append((hour, rewritten))
            if hour == 24:
                require([h for h, _ in feb28] == list(range(1, 25)), "incomplete February 28")
                result.extend(b"2016,2,29," + r.split(b",", 3)[3] for _, r in feb28)
    require(len(feb28) == 24 and len(result) == 8792, "missing or duplicate February 28")
    return b"".join(result)


def prepare_weather(original, output):
    original, output = path(original), path(output)
    require(c.digest(original) == GOLDEN_SHA, "requires pinned Golden TMY3 weather")
    derived = synthetic_weather(original.read_bytes())
    require(hashlib.sha256(derived).hexdigest() == WEATHER_SHA, "derived weather bytes differ")
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("xb") as f:
        f.write(derived)
    return {"schema": "controlled-synthetic-weather.v1", "source": reference(original),
            "output": reference(output), "qualification": "2016 years; February 29 copies February 28, not measured AMY."}


def parsed_options(command, allowed):
    options, positional = {}, []
    i = 1
    while i < len(command):
        token = command[i]
        if token.startswith("-"):
            require(token in allowed and token not in options and i + 1 < len(command)
                    and not command[i + 1].startswith("--"), "unexpected/duplicate command option: " + token)
            options[token] = command[i + 1]
            i += 2
        else:
            positional.append(token)
            i += 1
    return options, positional


def native_files(command, cwd):
    options, operands = parsed_options(command, {"--weather", "--output-directory", "--output-prefix", "--output-suffix"})
    require(len(operands) == 1 and set(options) >= {"--weather", "--output-directory", "--output-prefix"}, "Native command input/options differ")
    prefix = options["--output-prefix"]
    require(re.fullmatch(r"[A-Za-z0-9_-]+", prefix) is not None, "unsafe Native output prefix")
    suffix = options.get("--output-suffix", "L")
    require(suffix in ("L", "D", "C"), "unsupported Native output suffix")
    stem = path(options["--output-directory"], cwd) / (prefix + ("out" if suffix == "L" else ""))
    return stem, path(operands[0], cwd), path(options["--weather"], cwd)


def successful_receipt(p, sha, native=False):
    r = c.authenticated(p, sha)
    schema = r["schema"]
    if schema == "recorded-porting-command.v1":
        require(r["exit_code"] == 0 and r["launch_error"] is None
                and r["source_bytes_match_before_and_after"] is True, "unsuccessful/unstable recorded command")
        pinned(r["source_snapshot"])
        pinned(r["executed_launcher"]["archive"])
    else:
        # Compatibility with the accepted development receipt, not an aggregate
        # or cached result dependency. Public reproductions use record_command.py.
        require(native and schema == "actual-native-composed-B102-command.v1"
                and r["actual_executions"] == 1 and r["exit_code"] == 0 and r["exception"] is None
                and r["fixed_authorities_and_input_stable_before_after"] is True
                and r["fixed_authorities_before"] == r["fixed_authorities_after"]
                and r["input_before"] == r["input_after"], "unrecognized/unsuccessful Native receipt")
        pinned(r["input_before"])
        for key in ("energyplus_exe", "actual_weather"):
            pinned(r["fixed_authorities_before"][key])
    started, finished = (datetime.fromisoformat(r[k]) for k in ("started_utc", "finished_utc"))
    require(started.tzinfo is not None and finished >= started and math.isfinite(r["elapsed_seconds"])
            and r["elapsed_seconds"] >= 0, "invalid command chronology")
    for key in ("stdout", "stderr"):
        pinned(r[key])
    require(path(r["cwd"]) == ROOT, "receipt checkout differs")
    return r


def source_state(source, build, executable):
    require(source["source_snapshot"] == build["source_snapshot"], "Source and build snapshots differ")
    require(datetime.fromisoformat(build["finished_utc"]) <= datetime.fromisoformat(source["started_utc"]), "build follows Source command")
    capture = build["cargo_compiler_artifacts_capture"]
    require(capture["requested"] is True and capture["status"] == "pass", "build lacks emitted executable capture")
    require(build["command"][0] in ("cargo", "cargo.exe") and build["command"][1] == "build"
            and "--message-format=json-render-diagnostics" in build["command"], "not a recorded Cargo JSON build")
    command, cwd = source["command"], source["cwd"]
    require(len(command) >= 3 and command[1] == "run", "Source must run emitted CLI directly")
    options, operands = parsed_options([command[0]] + command[2:], {"--weather", "--output-dir", "--mode", "--partial", "--porting-scope", "--oracle-root"})
    require(len(operands) == 1 and options.get("--mode") == "compatibility" and options.get("--partial") == "deny"
            and options.get("--porting-scope") == "B" and {"--weather", "--output-dir"} <= set(options), "Source route/options differ")
    matches = [row["executable"] for row in capture["rows"]
               if path(row["executable"]["historical_path"]) == path(command[0], cwd)]
    require(len(matches) == 1, "Source command differs from emitted executable")
    exe = path(executable) if executable else path(command[0], cwd)
    require(c.digest(exe) == matches[0]["sha256"], "Source executable bytes differ from build")
    directory = path(options["--output-dir"], cwd)
    summary_path = directory / "run-summary.json"
    summary = c.read(summary_path)
    require(summary["status"] == "success" and summary["exit_code"] == 0 and summary["oracle"] is None, "Source did not complete independently")
    config = summary["config"]
    require(config["mode"] == "compatibility" and config["partial_policy"] == "deny" and config["hours"] is None
            and all(config[k] is False for k in ("dry_run", "oracle_baseline", "compare_oracle")), "Source summary route differs")
    runtime = summary["rust_runtime"]
    require(runtime["schema"] == "rusted-energyplus.source-runtime.v1"
            and runtime["execution"] == "actual Source ManageSimulation successful through620"
            and runtime["native_environment_hour_scope"] == 24 and runtime["requested_hours"] is None, "not full completed Source environment")
    selected = [v for v in runtime["native_environment_scope"] if v["selected_by_primary_flags"]]
    require(len(selected) == 1 and selected[0]["kind_of_environment"] == 3 and selected[0]["total_days"] == 1, "Source selected environment differs")
    owners = {v["stream"]: v for v in runtime["reported_streams"]}
    require(len(runtime["reported_streams"]) == len(owners) == 2 and set(owners) == {"eso", "mtr"}, "Source stream owners differ")
    eso, mtr = (path(owners[k]["owned_path"]) for k in ("eso", "mtr"))
    require(eso.parent == mtr.parent == directory and eso.suffix == ".eso" and mtr == eso.with_suffix(".mtr"), "Source physical owner paths differ")
    require(path(summary["artifacts"]["eplusrs_err"]) == eso.with_suffix(".err"), "Source ERR ownership differs")
    require(c.digest(path(summary["input"]["original"])) == INPUT_SHA
            and c.digest(path(summary["input"]["weather"])) == WEATHER_SHA, "Source consumed input/weather copies differ")
    return eso.with_suffix(""), owners, path(operands[0], cwd), path(options["--weather"], cwd), [reference(summary_path), reference(exe)]


def stream_scope(eso):
    lines = eso.read_text(encoding="utf8").splitlines()
    de, end = lines.index("End of Data Dictionary"), lines.index("End of Data")
    floor = [int(line.split(",", 1)[0]) for line in lines[1:de]
             if re.fullmatch(r"\d+,1,FLOOR,Surface Outside Face Temperature \[C\] !TimeStep", line)]
    require(len(floor) == 1, "missing/duplicate named Ground floor series")
    clocks, samples = [], []
    for line in lines[de + 1:end]:
        fields = line.split(",")
        if fields[0] == "2":
            clocks.append(tuple(int(v.strip()) for v in fields[1:6]) + (float(fields[6]), float(fields[7]), fields[8].strip()))
        elif fields[0] == str(floor[0]):
            samples.append(float(fields[1]))
    expected = [(1, 2, 29, 0, hour, step * 5.0, (step + 1) * 5.0, "Monday")
                for hour in range(1, 25) for step in range(12)]
    require(clocks == expected, "physical Feb29/12-per-hour clock differs")
    require(samples == [18.0] * 288, "Ground floor is not 288 exact 18C samples")
    return {"timestamps": len(clocks), "Ground_floor_all288_exact18C": True}


def ancillary(stem, native):
    eio, err, end, shd = (stem.with_suffix("." + ext).read_text(encoding="utf8") for ext in ("eio", "err", "end", "shd"))
    expected = {" Timesteps per Hour,": " Timesteps per Hour, 12,  5", " Version,": " Version, 26.1",
                " Simulation Control,": " Simulation Control, No, No, No, No, Yes, No",
                "Inside Convection Algorithm,": "Inside Convection Algorithm,Simple",
                "Outside Convection Algorithm,": "Outside Convection Algorithm,DOE-2",
                " Zone Summary,": " Zone Summary,1,6,0",
                " Surface Geometry,": " Surface Geometry,LowerRightCorner,Clockwise,RelativeCoordinateSystem,RelativeCoordinateSystem,RelativeToZoneOrigin",
                "Surface Heat Transfer Algorithm,": "Surface Heat Transfer Algorithm, CTF - ConductionTransferFunction,200,0.10,1000.0",
                "Environment,": "Environment,RUN PERIOD 1,WeatherFileRunPeriod,02/29/2016,02/29/2016,Monday,1,Use RunPeriod Specified Day,Yes,Yes,No,Yes,Yes,Clark and Allen"}
    selected = []
    for prefix, value in expected.items():
        found = [line for line in eio.splitlines() if line.startswith(prefix)]
        require(found == [value], "effective EIO differs: " + prefix)
        selected += found
    building, zone, ground = ([line for line in eio.splitlines() if line.startswith(prefix)]
                             for prefix in (" Building Information,", " Zone Information,", " Site:GroundTemperature:BuildingSurface,"))
    require(len(building) == len(zone) == len(ground) == 1 and ",-22.500," in building[0]
            and ",FullInteriorAndExterior," in building[0] and zone[0].startswith(" Zone Information, ZONE ONE,37.5,12.00,-7.00,2.25,")
            and ",CeilingDiffuser,MoWitt," in zone[0] and [float(v) for v in ground[0].split(",")[1:]] == [18.0] * 12, "effective Building/Zone/Ground EIO differs")
    selected += building + zone + ground
    require(not any(line.startswith((" Construction CTF,", " Material CTF Summary,", " Material:Air CTF Summary,")) for line in eio.splitlines())
            and not stem.with_suffix(".dxf").exists(), "unrequested detailed CTF/DXF must remain absent")
    require("..Solar Distribution=FullInteriorAndExterior" in shd
            and len(re.findall(r"^Number of back surfaces=0$", shd, re.M)) == 6
            and len(re.findall(r"^Number of receiving sub surfaces=0$", shd, re.M)) == 6, "opaque SHD facts differ")
    identity = err.splitlines()[0]
    require((re.fullmatch(r"Program Version,EnergyPlus, Version 26\.1\.0-6f2e40d102, YMD=\d{4}\.\d{2}\.\d{2} \d{2}:\d{2},", identity) is not None) if native
            else identity == "Program Version,rusted-energyplus 0.1.0,Rust typed input version 26.1.0", "engine identity differs")
    require(not MARKER.search(err), "unexpected Warning/Continue/Severe/Fatal")
    require(re.fullmatch(r"EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors; Elapsed Time=\d+hr \d+min [ \d.]+sec\r?\n", end) is not None, "END is not clean completion")
    return selected, [reference(stem.with_suffix("." + ext)) for ext in ("eio", "err", "end", "shd", "rdd", "mdd")]


def compare(args):
    case, input_path = load_case(args.case)
    native = successful_receipt(args.native_receipt, args.native_sha256, native=True)
    source = successful_receipt(args.source_receipt, args.source_sha256)
    build = successful_receipt(args.build_receipt, args.build_sha256)
    nstem, ni, nw = native_files(native["command"], native["cwd"])
    sstem, owners, si, sw, source_refs = source_state(source, build, args.source_executable)
    require(c.digest(ni) == c.digest(si) == c.digest(input_path) == INPUT_SHA
            and c.digest(nw) == c.digest(sw) == WEATHER_SHA, "same exact input/weather bytes required")
    neio, nrefs = ancillary(nstem, True)
    seio, srefs = ancillary(sstem, False)
    require(neio == seio and nstem.with_suffix(".shd").read_bytes() == sstem.with_suffix(".shd").read_bytes(), "Source/Native effective EIO/SHD differs")
    scope = {"Native": stream_scope(nstem.with_suffix(".eso")), "Source": stream_scope(sstem.with_suffix(".eso"))}
    policy = c.read(ROOT / c.POLICY_PATH)
    rows, meter_rows = policy["profiles"]["B"], policy["meter_profiles"]["B"]
    require(len(rows) == 102 and meter_rows == [], "original B policy count differs")
    failures, streams = io.StringIO(), []
    rdds = [c.available_variables(stem.with_suffix(".rdd")) for stem in (sstem, nstem)]
    mdds = [set(stem.with_suffix(".mdd").read_text(encoding="utf8").splitlines()) for stem in (sstem, nstem)]
    for kind, policy_rows in (("eso", rows), ("mtr", meter_rows)):
        streams.append(c.completed_stream(kind, sstem.with_suffix("." + kind), nstem.with_suffix("." + kind), owners[kind], policy_rows,
                                         288, 12, *rdds, *mdds, failures))
    require(len(streams[0]["series"]) == 102 and streams[0]["records"] == 29376
            and streams[1]["deleted_empty"] is True, "complete physical output scope differs")
    for ref in nrefs + srefs + source_refs + [stream[key] for stream in streams if not stream["deleted_empty"] for key in ("Source", "Native")]:
        pinned(ref)
    reports = streams[0]["series"]
    totals = {"tolerance_failures": sum(row["tolerance_failures"] for row in reports),
              "rmse_failures": sum(not row["rmse_limit_met"] for row in reports),
              "raw_operand_unequal": sum(row["raw_operand_unequal"] for row in reports),
              "parsed_binary64_unequal": sum(row["parsed_binary64_unequal"] for row in reports)}
    failure_rows = [json.loads(line) for line in failures.getvalue().splitlines()]
    return {"schema": "source-input-example-comparison.v1", "case_id": case["id"], "fixed_conformance_case": False,
            "accepted": all(row["passed"] for row in reports), "failures": failure_rows, "totals": totals, "series": 102, "numeric_records": 29376,
            "input": reference(input_path), "weather": reference(nw), "case": reference(args.case),
            "policy": reference(ROOT / c.POLICY_PATH), "comparison_kernel": reference(c.__file__),
            "receipts": [reference(p) for p in (args.native_receipt, args.source_receipt, args.build_receipt)],
            "source_snapshot": source["source_snapshot"], "source_refs": source_refs, "ancillary": nrefs + srefs,
            "exact_effective_EIO": neio, "physical_scope": scope, "streams": streams,
            "qualification": case["scope_qualification"], "provenance_limit": "Checks existing bytes and recorded build/run linkage. A receipt alone does not prove inputs/executable stayed immutable between execution and this comparison."}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="action", required=True)
    weather = sub.add_parser("prepare-weather")
    weather.add_argument("--golden", required=True)
    weather.add_argument("--output", required=True)
    check = sub.add_parser("check")
    check.add_argument("--case", default=str(DEFAULT_CASE))
    for kind in ("native", "source", "build"):
        check.add_argument("--" + kind + "-receipt", required=True)
        check.add_argument("--" + kind + "-sha256", required=True)
    check.add_argument("--source-executable", help="Optional immutable archived executable matching the recorded Cargo SHA")
    check.add_argument("--output-dir", required=True)
    args = parser.parse_args()
    try:
        if args.action == "prepare-weather":
            print(json.dumps(prepare_weather(args.golden, args.output), indent=2))
            return 0
        output = path(args.output_dir)
        output.mkdir(parents=True, exist_ok=False)
        result = compare(args)
        (output / "comparison.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf8")
        print(json.dumps({k: result[k] for k in ("case_id", "accepted", "series", "numeric_records", "failures")}))
        return 0 if result["accepted"] else 1
    except (ValueError, KeyError, OSError, TypeError, IndexError) as exc:
        print(str(exc), file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
