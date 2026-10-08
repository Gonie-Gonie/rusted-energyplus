"""Author/check input-only CLK-02 drafts; never execute a scientific parser."""

from __future__ import annotations

import argparse
import hashlib
import json
import struct
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / "energyplus_porting_plan"
CASES = PLAN / "cases/CLK-02"
AUDIT = ROOT / ".runtime/porting/CLK-02/source-audit-01/freeze-02/audit.json"
DRAFT = ROOT / ".runtime/porting/CLK-02/input-author-draft-02"
COMMIT = "6f2e40d10250a105b49966baa24d843711e61048"
REAL_FIELDS = [
    "DryBulb", "DewPoint", "RelHum", "AtmPress", "ETHoriz", "ETDirect",
    "IRHoriz", "GLBHoriz", "DirectRad", "DiffuseRad", "GLBHorizIllum",
    "DirectNrmIllum", "DiffuseHorizIllum", "ZenLum", "WindDir", "WindSpeed",
    "TotalSkyCover", "OpaqueSkyCover", "Visibility", "CeilHeight",
]
TAIL_FIELDS = ["PrecipWater", "AerosolOptDepth", "SnowDepth", "DaysSinceLastSnow", "Albedo", "LiquidPrecip"]
DATE_FIELDS = ["WYear", "WMonth", "WDay", "WHour", "WMinute"]
BASE = [
    "2013", "1", "1", "1", "0", "RAW-CANARY",
    "1.25", "-2.5", "50.125", "101325.0", "0.0", "0.0", "300.5",
    "123.25", "321.5", "45.25", "1.0", "2.0", "3.0", "4.0",
    "359.5", "2.5", "5.0", "4.0", "10.0", "2000.0", "0.0",
    "123456789", "10.25", "0.125", "0.0", "88.0", "0.25", "1.5", "99.0",
]


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def ref(path: Path) -> dict:
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": sha(path.read_bytes())}


def bits(value: float) -> str:
    return struct.pack(">d", value).hex()


def write_json(path: Path, value: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    # Compact JSON retains all exact values while keeping each contract <800 lines.
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")


def record(record_id: str, changes: dict[int, str] | None = None, *,
           cut: int | None = None, kind: str = "raw-parser-diagnostic", intent: str) -> dict:
    tokens = BASE.copy()
    for index, text in (changes or {}).items():
        tokens[index] = text
    if cut is not None:
        tokens = tokens[:cut]
    text = ",".join(tokens)
    return {"record_id": record_id, "kind": kind, "branch_intent": intent,
            "line_utf8": text, "line_sha256": sha(text.encode("utf-8")),
            "expected_values_supplied": False, "expected_exit_supplied": False}


def record_sequences() -> list[dict]:
    calls = [
        record("RAW-NORMAL", intent="complete finite raw outputs"),
        record("RAW-SENTINELS", {6: "99.9", 7: "99.9", 8: "999.0", 9: "999999.0",
               10: "9999.0", 11: "9999.0", 12: "9999.0", 13: "9999.0",
               14: "9999.0", 15: "9999.0", 16: "999999.0", 17: "999999.0",
               18: "999999.0", 19: "99999.0", 20: "999.0", 21: "999.0",
               22: "99.0", 23: "99.0", 24: "9999.0", 25: "99999.0",
               **{i: "999.0" for i in range(28, 34)}}, intent="copy sentinels without substitution"),
        record("RAW-NEGATIVE-FINITE", {i: "-1.25" for i in range(6, 26)},
               intent="copy finite negative physical fields without range normalization"),
        record("RAW-SIGNED-ZERO", {i: ("-0.0" if i % 2 else "0.0")
               for i in list(range(6, 27)) + list(range(28, 34))}, intent="source finite zero signs"),
        record("DATE-FEB29-NONLEAP-RECORD-YEAR", {1: "2", 2: "29"}, intent="raw parser does not enforce Gregorian record year"),
        record("DATE-DAY-ZERO", {2: "0"}, intent="raw lower day bound is not checked"),
        record("DATE-YEAR-ZERO", {0: "0"}, intent="raw year copied independently of civil calendar"),
        record("DATE-HOUR25-MINUTE60", {3: "25", 4: "60"}, intent="raw hour/minute copied; daily-reader admission excluded"),
        record("DATE-HOUR24-MINUTE0", {3: "24", 4: "0"}, intent="raw hour-ending convention retained"),
    ]
    for text, name in [("0.49", "POS-BELOW"), ("0.5", "POS-TIE"),
                       ("-0.49", "NEG-BELOW"), ("-0.5", "NEG-TIE"), ("9.0", "NONZERO")]:
        calls.append(record("OBS-" + name, {26: text}, intent="bounded source nint observation indicator"))
    codes = [
        record("CODES-VALID", intent="zero observation, nine digits"),
        record("CODES-QUOTED", {27: "'123456789'"}, intent="quotes replaced and edge spaces stripped"),
        record("CODES-NONDIGIT", {27: "12A45_789"}, intent="nondigit cleanup at length nine"),
        record("CODES-SHORT-FIRST", {27: "123"}, intent="real missed-code owner mutation"),
        record("CODES-BLANK", {27: ""}, intent="empty code field defaults to nine nines"),
        record("CODES-NONZERO-SHORT", {26: "9.0", 27: "123"}, intent="nonzero observation skips code validation"),
        record("CODES-SHORT-SECOND", {27: "123"}, intent="same-owner repeated missed-code mutation"),
    ]
    for count in range(6):
        # The code delimiter always exists, even with no tail tokens.
        row = record(f"TAIL-ABSENT-{count}", cut=28 + count, intent="optional tail exhaustion after declared prefix")
        if count == 0:
            row["line_utf8"] += ","
            row["line_sha256"] = sha(row["line_utf8"].encode())
        calls.append(row)
    for index, name in enumerate(TAIL_FIELDS):
        calls.append(record("TAIL-BLANK-" + name, {28 + index: ""}, intent="truly empty optional token"))
    calls += [
        record("TAIL-SPACES", {i: "   " for i in range(28, 34)}, intent="ProcessNumber literal-space-only tokens"),
        record("TAIL-FORTRAN-D", {i: "1.25D+01" for i in range(28, 34)}, intent="genuine ProcessNumber exponent compatibility"),
        record("TAIL-INVALID", {28: "not-a-number"}, kind="safe-error-diagnostic", intent="actual optional-token error route"),
        record("TAIL-NONFINITE", {28: "nan"}, kind="safe-error-diagnostic", intent="ProcessNumber nonfinite error route"),
        record("TAIL-FINAL-NO-DURATION", cut=34, intent="final optional token without a comma"),
        record("IGNORED-DURATION-35", {34: "IGNORED-NONNUMERIC"}, intent="unconsumed precipitation duration column"),
        record("DATE-INVALID-MONTH0", {1: "0"}, kind="safe-error-diagnostic", intent="actual upper/lower month rejection"),
        record("DATE-INVALID-MONTH13", {1: "13"}, kind="safe-error-diagnostic", intent="actual upper/lower month rejection"),
        record("DATE-INVALID-APRIL31", {1: "4", 2: "31"}, kind="safe-error-diagnostic", intent="actual non-February upper day rejection"),
        record("DATE-INVALID-FEB30", {1: "2", 2: "30"}, kind="safe-error-diagnostic", intent="actual February upper day rejection"),
        record("MANDATORY-INVALID", {6: "not-a-number"}, kind="safe-error-diagnostic", intent="homogeneous cursor failure; safe complete delimiter shape"),
        record("MANDATORY-EMPTY", {6: ""}, kind="safe-error-diagnostic", intent="repeated-comma skip; actual whole-block classification"),
        record("MANDATORY-PREFIX", {6: "1.25suffix"}, kind="safe-error-diagnostic", intent="prefix consumption followed by cursor failure"),
    ]
    mandatory = [1001.125 + i for i in range(20)]
    optional = [2001.25 + i for i in range(6)]
    initial = {"ErrorFound": True, "dates": [-701, -702, -703, -704, -705],
               "mandatory_reals": mandatory, "mandatory_real_bits": [bits(x) for x in mandatory],
               "WObs": -706, "weather_codes": [-7] * 9,
               "optional_reals": optional, "optional_real_bits": [bits(x) for x in optional]}
    sequences = [{"sequence_id": row["record_id"], "operations": [row]} for row in calls]
    sequences.insert(14, {"sequence_id": "CODES-SAME-OWNER-SEVEN", "operations": codes})
    for sequence in sequences:
        sequence.update({"route": "whole-original-InterpretWeatherDataLine",
                         "fresh_owner": True, "initial_outputs": initial,
                         "reset_declared_output_canaries_before_each_call": True,
                         "prepared_weather_code_missed_count": 0,
                         "prepared_EndDayOfMonth": [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]})
    return sequences


def header_cases() -> list[dict]:
    direct = [
        ("LOCATION-NORMAL", "Location", "LOCATION,Mixed City,State,Country,Source,12345,39.74,-105.18,-7.0,1829.0", []),
        ("LOCATION-INVALID-RETAIN", "Location", "LOCATION,Mixed City,State,Country,Source,12345,not-a-number,-105.18,-7.0,1829.0", []),
        ("LOCATION-CONTINUATION", "Location", "LOCATION,Mixed City,State,Country,Source,12345,", ["39.74,-105.18,-7.0,1829.0"]),
        ("HOLIDAYS-ZERO", "HolidaysDST", "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0", []),
        ("HOLIDAYS-DST-LEAP", "HolidaysDST", "HOLIDAYS/DAYLIGHT SAVINGS,Yes,2nd Sunday in March,1st Sunday in November,0", []),
        ("HOLIDAYS-INVALID-START", "HolidaysDST", "HOLIDAYS/DAYLIGHT SAVINGS,No,BAD,0,0", []),
        ("PERIOD-NORMAL", "DataPeriods", "DATA PERIODS,1,1,Data,Sunday,1/1,12/31", []),
        ("PERIOD-CONTINUATION", "DataPeriods", "DATA PERIODS,2,1,First Period,Sunday,1/1,6/30,", ["Second Period,Monday,7/1,12/31"]),
        ("PERIOD-WRAP-LEAP", "DataPeriods", "DATA PERIODS,1,1,Winter,Friday,12/1,1/31", []),
        ("PERIOD-START-YEAR-ONLY", "DataPeriods", "DATA PERIODS,1,1,Actual,Wednesday,1/1/2013,12/31", []),
        ("PERIOD-YEARS", "DataPeriods", "DATA PERIODS,1,1,Actual,Saturday,12/31/2016,1/1/2017", []),
        ("PERIOD-INITIAL-ERROR", "DataPeriods", "DATA PERIODS,1,1,Data,Sunday,1/1,12/31", []),
        ("PERIOD-ORDINAL", "DataPeriods", "DATA PERIODS,1,1,Ordinal,Sunday,1,365", []),
        ("PERIOD-ZERO-COUNT", "DataPeriods", "DATA PERIODS,0,1", []),
        ("PERIOD-ZERO-INTERVAL", "DataPeriods", "DATA PERIODS,1,0,Data,Sunday,1/1,12/31", []),
        ("PERIOD-INVALID-WEEKDAY", "DataPeriods", "DATA PERIODS,1,1,Data,FUNDAY,1/1,12/31", []),
        ("COMMENTS-NO-COMMA", "Comments1", "COMMENTS 1", []),
        ("DESIGN-NO-ACTION", "DesignConditions", "DESIGN CONDITIONS,UNPARSED-TEXT", []),
    ]
    cases = []
    prepared = {"WeatherFileLatitude": 12.25, "WeatherFileLongitude": -71.5,
                "WeatherFileTimeZone": 3.75, "WeatherFileElevation": 123.5,
                "EPWHeaderTitle": "DECLARED-CANARY", "NumEPWTypExtSets": 0,
                "InputProcessorSpecialDaysObjectCount": 0, "LeapYearAdd": 0}
    for case_id, header_type, line, continuation in direct:
        stream = CASES / "headers/direct" / (case_id + ".txt")
        stream.parent.mkdir(parents=True, exist_ok=True)
        stream.write_text("\n".join(continuation + ["UNCONSUMED-SENTINEL"]) + "\n", encoding="utf-8", newline="\n")
        state = prepared.copy()
        if case_id == "PERIOD-WRAP-LEAP":
            state["LeapYearAdd"] = 1
        cases.append({"case_id": case_id, "route": "direct-original-ProcessEPWHeader",
                      "header_type": header_type, "initial_Line": line,
                      "initial_Line_sha256": sha(line.encode()), "stream": ref(stream),
                      "initial_ErrorsFound": case_id == "PERIOD-INITIAL-ERROR",
                      "prepared_state": state, "expected_values_supplied": False,
                      "expected_exit_supplied": False, "whole_file_admission_claimed": False})
    normal = [
        "LOCATION,Mixed City,State,Country,Source,12345,39.74,-105.18,-7.0,1829.0",
        "DESIGN CONDITIONS,0", "TYPICAL/EXTREME PERIODS,0", "GROUND TEMPERATURES,0",
        "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0", "COMMENTS 1,First", "COMMENTS 2,Second",
        "DATA PERIODS,1,1,Data,Sunday,1/1,12/31",
    ]
    variants = {
        "OPEN-NORMAL": normal + [",".join(BASE)],
        "OPEN-CONTINUATION": [direct[2][2], *direct[2][3], *normal[1:], ",".join(BASE)],
        "OPEN-LOWERCASE-LOCATION": [normal[0].replace("LOCATION", "location", 1), *normal[1:], ",".join(BASE)],
        "OPEN-TRUNCATED-HEADER": normal[:7],
    }
    for case_id, lines in variants.items():
        file = CASES / "headers/full" / (case_id + ".epw")
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text("\n".join(lines) + "\n", encoding="utf-8", newline="\n")
        cases.append({"case_id": case_id, "route": "whole-original-OpenEPlusWeatherFile",
                      "file": ref(file), "ProcessHeader": True, "initial_ErrorsFound": False,
                      "prepared_LeapYearAdd": 0, "expected_values_supplied": False,
                      "expected_exit_supplied": False, "later_weather_admission_claimed": False})
    return cases


def author() -> None:
    if CASES.exists() or DRAFT.exists():
        raise RuntimeError("draft authoring requires fresh cases and receipt directory")
    audit = json.loads(AUDIT.read_text(encoding="utf-8"))
    scope_path = PLAN / "contracts/scope.json"
    scope = json.loads(scope_path.read_text(encoding="utf-8"))
    weather = ROOT / scope["cases"][0]["weather"]["path"]
    for row in scope["cases"]:
        if row["weather"] != scope["cases"][0]["weather"]:
            raise RuntimeError("all fixed CON inputs must retain same weather binding")
    if ref(weather) != scope["cases"][0]["weather"]:
        raise RuntimeError("fixed EPW input hash mismatch")
    for row in audit["source_files"]:
        original = ROOT / ".reference/energyplus-src/26.1.0" / row["source_path"]
        if sha(original.read_bytes()) != row["source_sha256"]:
            raise RuntimeError("source discovery pin drift")
    sequences = record_sequences()
    headers = header_cases()
    operations = [op for seq in sequences for op in seq["operations"]]
    if (len(sequences), len(operations), len(headers), sum(op["kind"] == "safe-error-diagnostic" for op in operations)) != (40, 46, 22, 9):
        raise RuntimeError("agreed draft counts differ")
    request_path = CASES / "helper-request.json"
    request = {
        "schema": "clk02-helper-cases.v1", "expected_values_supplied": False,
        "expected_exits_supplied": False, "record_sequences": sequences, "header_cases": headers,
        "fixed_epw": {"case_id": "FIXED-CON-EPW-8760", "file": ref(weather),
                      "route": "whole-original-OpenEPlusWeatherFile-then-ordered-InterpretWeatherDataLine",
                      "ProcessHeader": True, "record_count": 8760,
                      "same_owner_for_all_raw_records": True, "reset_declared_output_canaries_before_each_call": True,
                      "initial_outputs": sequences[0]["initial_outputs"], "initial_ErrorsFound": False,
                      "civil_calendar_supplied_to_raw_parser": False, "physics_executed": False},
    }
    # Each sequence/case occupies one line; raw text is never pre-parsed into answers.
    lines = ["{", '  "schema": "clk02-helper-cases.v1",', '  "expected_values_supplied": false,',
             '  "expected_exits_supplied": false,', '  "record_sequences": [']
    lines += ["    " + json.dumps(row, ensure_ascii=False) + ("," if i + 1 < len(sequences) else "") for i, row in enumerate(sequences)]
    lines += ['  ],', '  "header_cases": [']
    lines += ["    " + json.dumps(row, ensure_ascii=False) + ("," if i + 1 < len(headers) else "") for i, row in enumerate(headers)]
    lines += ['  ],', '  "fixed_epw": ' + json.dumps(request["fixed_epw"], ensure_ascii=False), "}"]
    request_path.write_text("\n".join(lines) + "\n", encoding="utf-8", newline="\n")
    source = {
        "schema": "clk02-source-contract.v1", "card": "CLK-02", "status": "draft-before-independent-review",
        "energyplus_commit": COMMIT, "source_audit": ref(AUDIT),
        "source_files": [{"path": row["source_path"], "sha256": row["source_sha256"]} for row in audit["source_files"]],
        "selected_ranges": [{key: row[key] for key in ("file", "symbol", "description", "start_line", "end_line", "role", "range_sha256")} for row in audit["selected_ranges"]],
        "primary_functions": ["InterpretWeatherDataLine", "ProcessEPWHeader", "OpenEPlusWeatherFile"],
        "source_facts": audit["source_facts"], "existing_CLK01_source_contract": audit["existing_CLK01_source_pin"],
        "paired_raw_outputs": {"date_integer_fields": DATE_FIELDS, "mandatory_real_fields": REAL_FIELDS,
                               "observation_integer_field": "WObs", "weather_codes": 9, "optional_real_fields": TAIL_FIELDS,
                               "private_RField21_observed": False},
        "paired_header_state": ["EPWHeaderTitle", "WeatherFileLocationTitle", "WeatherFileLatitude", "WeatherFileLongitude",
                                "WeatherFileTimeZone", "WeatherFileElevation", "WFAllowsLeapYears", "EPWDaylightSaving",
                                "EPWDST", "DST", "NumSpecialDays", "SpecialDays", "NumDataPeriods", "NumIntervalsPerHour", "DataPeriods"],
        "mutable_parser_state": ["wvarsMissedCounts.WeathCodes"],
        "observed_call_context": ["ErrorFound_before_after", "ErrorsFound_before_after", "Line_before_after", "actual_stream_position_and_state"],
        "constraints": {"complete_delimiter_shape": True, "complete_finite_header_continuations": True,
                        "bounded_finite_WObs_conversion": True, "no_implicit_positive_count_or_interval_validation": True,
                        "raw_year_is_not_civil_year": True, "full_open_gate_distinct_from_direct_enum": True,
                        "only_defined_header_state_observed": True, "uninitialized_source_locals_observed": False,
                        "DST_weekday_copy_requires_weekday_form_or_disabled_zero_route": True,
                        "whole_error_manager_and_IO_peer_parity": False},
        "excluded_science": ["previous/default sentinel replacement", "RH percent-to-fraction normalization", "rain/snow flags and rain fallback",
                             "IR/sky calculations", "weather record selection", "Today/Tomorrow lifecycle", "timestep interpolation",
                             "civil calendar and warmup equality", "general Site location override", "unused Typical/Ground CON consumers"],
        "source_only_context": ["full header Typical/Extreme and Ground branches really execute", "all unused numeric columns really parse",
                                "source error text/global IO remains unpaired", "header input preparation is not whole simulation parser admission"],
        "builder_policy": "same verified GNU13.2 core/API/Objexx and directory+target definitions; unchanged original functions; future debug derivative separately authenticated",
        "references_or_fixture_outputs_supplied_as_inputs": False, "gates_changed": False,
    }
    # Source ranges are one row per line to retain compact literal-symbol metadata.
    source_path = PLAN / "contracts/CLK-02-source.json"
    encoded = json.dumps(source, ensure_ascii=False, indent=2)
    compact = "[\n" + ",\n".join("    " + json.dumps(row, ensure_ascii=False) for row in source["selected_ranges"]) + "\n  ]"
    start = encoded.index('  "selected_ranges":')
    end = encoded.index('\n  "primary_functions":', start)
    encoded = encoded[:start] + '  "selected_ranges": ' + compact + "," + encoded[end:]
    source_path.write_text(encoded + "\n", encoding="utf-8", newline="\n")
    cases = {
        "schema": "clk02-cases-contract.v1", "card": "CLK-02", "status": "draft-before-independent-review",
        "helper_request": ref(request_path), "fixed_CON_scope": ref(scope_path),
        "counts": {"record_sequences": 40, "diagnostic_raw_calls": 46, "safe_error_diagnostics": 9,
                   "fixed_epw_raw_calls": 8760, "total_raw_calls": 8806,
                   "direct_enum_header_cases": 18, "diagnostic_full_open_header_cases": 4,
                   "fixed_epw_full_open_header_cases": 1, "total_header_roots": 23,
                   "unchanged_CON_cases": 15, "proposed_bounded_Rust_physical_commands": 6},
        "record_sequence_ids": [row["sequence_id"] for row in sequences], "header_case_ids": [row["case_id"] for row in headers],
        "native_routes": ["whole unchanged raw parser", "whole direct enum header with real continuation stream", "whole file-open header gate"],
        "safe_error_policy": "observe actual failure/available outputs; no predetermined exit code or error-text equivalence",
        "admission": {"all_finite_primary_raw_values": True, "WObs_finite_int_range": True, "header_counts_nonnegative_bounded": True,
                      "zero_count_and_zero_intervals_direct_cases_admitted": True, "unterminated_continuations_excluded": True,
                      "short_delimiter_remove_prefix_precondition_violations_excluded": True},
        "native_ordinary_input_scope": "all15 frozen CON bindings remain unchanged; extra diagnostic EPWs are parser-only inputs",
        "Rust_physical_cases": [{"case_id": case_id, "trace_levels": ["Full", "Summary"],
                                 "claim": "actual parsed raw owner and literal physical-adapter handoff; downstream science remains unpaired"}
                                for case_id in ("A-24H", "A-72H", "B-BOTH-24H")],
        "whole_epw_parse_is_annual_physics": False, "expected_values_supplied": False, "expected_exits_supplied": False,
        "scientific_execution_performed": False, "gates_changed": False,
    }
    tolerance = {
        "schema": "clk02-tolerances-contract.v1", "card": "CLK-02", "status": "draft-before-independent-review",
        "profiles": {"finite_raw_scalar": {"policy": "exact_ieee_binary64_bits", "atol": 0.0, "rtol": 0.0},
                     "finite_header_numeric": {"policy": "exact_ieee_binary64_bits", "atol": 0.0, "rtol": 0.0},
                     "integer_text_boolean_arrays_order": {"policy": "recursive_exact_type_and_value"},
                     "finite_zero": {"policy": "exact_positive_or_negative_zero_sign"},
                     "copied_physical_adapter_operand": {"policy": "exact_bits_when_same_named_raw_owner_field_is_copied"}},
        "field_groups": {"mandatory_reals": REAL_FIELDS, "optional_reals": TAIL_FIELDS, "date_integers": DATE_FIELDS,
                         "observation": "WObs", "codes": "nine typed integers", "header_dates_counts_weekdays": "typed exact integers"},
        "nonfinite_policy": "primary admitted values finite; safe optional nonfinite error diagnostic not paired as numeric result",
        "source_error_IO_counter_limits": "only actual weather-code missed count paired; global warning/error indices/messages are unpaired",
        "unpaired_downstream": ["replacement/clamping", "fractional RH", "rain/snow", "IR/sky", "selection/interpolation/calendar/warmup"],
        "failure_does_not_relax_policy": True, "no_native_output_injection": True, "gates_changed": False,
    }
    cases_path = PLAN / "contracts/CLK-02-cases.json"
    tol_path = PLAN / "contracts/CLK-02-tolerances.json"
    write_json(cases_path, cases)
    write_json(tol_path, tolerance)
    DRAFT.mkdir(parents=True)
    files = [source_path, cases_path, tol_path, request_path, Path(__file__)]
    archives = []
    for path in files:
        target = DRAFT / path.name
        target.write_bytes(path.read_bytes())
        archives.append({"historical_path": path.relative_to(ROOT).as_posix(), "archive": ref(target)})
    manifest = {"schema": "clk02-input-author-draft.v1", "status": "draft-no-scientific-execution",
                "source_audit": ref(AUDIT), "files": archives, "counts": cases["counts"],
                "engines_executed": False, "Rust_modified": False, "gates_changed": False}
    write_json(DRAFT / "draft.json", manifest)
    print(json.dumps({"draft": ref(DRAFT / "draft.json"), "files": [ref(path) for path in files[:4]], "counts": cases["counts"]}))


def check() -> None:
    manifest = json.loads((DRAFT / "draft.json").read_text(encoding="utf-8"))
    for row in manifest["files"]:
        archive = ROOT / row["archive"]["path"]
        live = ROOT / row["historical_path"]
        if sha(archive.read_bytes()) != row["archive"]["sha256"] or archive.read_bytes() != live.read_bytes():
            raise RuntimeError("draft archive/current byte mismatch")
    request = json.loads((CASES / "helper-request.json").read_text(encoding="utf-8"))
    for case in request["header_cases"]:
        binding = case.get("stream", case.get("file"))
        if ref(ROOT / binding["path"]) != binding:
            raise RuntimeError("header stream identity mismatch")
    for seq in request["record_sequences"]:
        for op in seq["operations"]:
            if sha(op["line_utf8"].encode()) != op["line_sha256"]:
                raise RuntimeError("raw input line mismatch")
    print(json.dumps({"status": "metadata-only-draft-check-pass", "draft": ref(DRAFT / "draft.json"), "counts": manifest["counts"]}))


def preserve() -> None:
    if DRAFT.exists():
        raise RuntimeError("fresh draft preservation required")
    DRAFT.mkdir(parents=True)
    files = [PLAN / f"contracts/CLK-02-{name}.json" for name in ("source", "cases", "tolerances")]
    files += [CASES / "helper-request.json", Path(__file__)]
    rows = []
    for path in files:
        target = DRAFT / path.name
        target.write_bytes(path.read_bytes())
        rows.append({"historical_path": path.relative_to(ROOT).as_posix(), "archive": ref(target)})
    cases = json.loads(files[1].read_text(encoding="utf-8"))
    write_json(DRAFT / "draft.json", {
        "schema": "clk02-input-author-draft.v1", "status": "draft-no-scientific-execution",
        "source_audit": ref(AUDIT), "files": rows, "counts": cases["counts"],
        "previous_draft": ref(ROOT / ".runtime/porting/CLK-02/input-author-draft-01/draft.json"),
        "pre_run_amendment": "genuine weekday DST forms avoid undefined original local PWeekDay; no parser/source math change",
        "engines_executed": False, "Rust_modified": False, "gates_changed": False,
    })
    print(json.dumps({"draft": ref(DRAFT / "draft.json"), "files": [ref(path) for path in files[:4]], "counts": cases["counts"]}))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--write-draft", action="store_true")
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--preserve-draft", action="store_true")
    args = parser.parse_args()
    if sum([args.write_draft, args.check, args.preserve_draft]) != 1:
        parser.error("choose exactly one action")
    author() if args.write_draft else (preserve() if args.preserve_draft else check())
