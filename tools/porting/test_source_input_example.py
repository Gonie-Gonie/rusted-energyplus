"""Portable derivation and fail-closed boundary tests; no engine is launched."""
from datetime import date, timedelta
from pathlib import Path
import json
import tempfile
import unittest

import source_input_example as e


def original_weather():
    headers = [b"header\r\n"] * 8
    headers[4] = b"HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0\r\n"
    headers[7] = b"DATA PERIODS,1,1,Data,Sunday, 1/ 1,12/31\r\n"
    rows = []
    for day in range(365):
        d = date(2013, 1, 1) + timedelta(days=day)
        rows.extend(f"2013,{d.month},{d.day},{hour},60,preserved,{day}:{hour}\r\n".encode() for hour in range(1, 25))
    return b"".join(headers + rows)


class WeatherDerivationTests(unittest.TestCase):
    def test_year_qualification_and_feb29_copy_preserve_physical_fields(self):
        result = e.synthetic_weather(original_weather()).splitlines(keepends=True)
        self.assertEqual(len(result), 8792)
        self.assertEqual(result[4], b"HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0\r\n")
        self.assertEqual(result[7], b"DATA PERIODS,1,1,Data,Friday,1/1/2016,12/31/2016\r\n")
        days = {day: [r.split(b",", 3)[3] for r in result[8:] if r.startswith(f"2016,2,{day},".encode())] for day in (28, 29)}
        self.assertEqual(len(days[29]), 24)
        self.assertEqual(days[28], days[29])
        self.assertTrue(all(r.startswith(b"2016,") and r.endswith(b"\r\n") for r in result[8:]))

    def test_malformed_source_and_incomplete_feb28_are_rejected(self):
        raw = original_weather()
        for changed in (raw.replace(b"Sunday, 1/ 1", b"Monday, 1/ 1"), raw.replace(b"2013,2,28,24,", b"2013,2,28,23,"), raw.replace(b"\r\n", b"\n", 1)):
            with self.subTest(change=changed[:50]), self.assertRaises(ValueError):
                e.synthetic_weather(changed)

    def test_public_preparer_rejects_unpinned_weather_without_writing_output(self):
        with tempfile.TemporaryDirectory() as td:
            source, out = Path(td) / "fake.epw", Path(td) / "out.epw"
            source.write_bytes(original_weather())
            with self.assertRaisesRegex(ValueError, "pinned Golden"):
                e.prepare_weather(source, out)
            self.assertFalse(out.exists())


class ComparisonBoundaryTests(unittest.TestCase):
    def test_native_prefix_suffix_resolution_and_ambiguous_options(self):
        cmd = ["energyplus", "--weather", "weather.epw", "--output-directory", "out", "--output-prefix", "chosen", "input.idf"]
        self.assertEqual(e.native_files(cmd, e.ROOT)[0], e.ROOT / "out/chosenout")
        self.assertEqual(e.native_files(cmd + ["--output-suffix", "D"], e.ROOT)[0], e.ROOT / "out/chosen")
        for changed in (cmd + ["--output-prefix", "again"], [v if v != "chosen" else "../escape" for v in cmd], cmd + ["--annual", "yes"]):
            with self.subTest(command=changed), self.assertRaises(ValueError):
                e.native_files(changed, e.ROOT)

        # Public metadata boundaries only; no kernel, manifest or engine is run.
        import hashlib
        import check_completed_reported_streams_v2 as v2

        allowed = {"--weather", "--output-directory", "--output-prefix"}
        self.assertEqual(v2.options(cmd, allowed), ({"--weather": "weather.epw", "--output-directory": "out", "--output-prefix": "chosen"}, ["input.idf"]))
        for changed in (cmd + ["--output-prefix", "again"], cmd + ["--unknown", "value"], ["energyplus", "--weather"], cmd + [True]):
            with self.subTest(v2_command=changed), self.assertRaises(ValueError):
                v2.options(changed, allowed)

        document = {"schema": "reported-stream-policy.v1", "variables": [
            {"key": "*", "variable": "Surface Outside Face Temperature", "units": "C", "frequency": "TimeStep", "atol": 0, "rtol": 0, "rmse_max": 0},
            {"key": "*", "variable": "Surface Inside Face Temperature", "units": "C", "frequency": "TimeStep", "atol": 0, "rtol": 0, "rmse_max": 0}],
            "meters": [{"meter": "Heating:EnergyTransfer", "units": "J", "frequency": "Hourly", "atol": 0, "rtol": 0, "rmse_max": 0}]}
        with tempfile.TemporaryDirectory() as td:
            path = (Path(td) / "policy.json").resolve()
            name = path.relative_to(e.ROOT).as_posix() if path.is_relative_to(e.ROOT) else path.as_posix()
            raw = json.dumps(document).encode("utf8")
            path.write_bytes(raw)
            pin = {"path": name, "sha256": hashlib.sha256(raw).hexdigest(), "bytes": len(raw)}
            self.assertEqual(v2.auth(pin), path)
            self.assertEqual(v2.policy(pin), document)
            for changed in (dict(pin, bytes=True), dict(pin, path=name.rsplit("/", 1)[0] + "/./policy.json")):
                with self.subTest(v2_reference=changed), self.assertRaises(ValueError):
                    v2.auth(changed)
            path.write_bytes(raw + b" ")
            with self.assertRaisesRegex(ValueError, "Changed referenced file"):
                v2.auth(pin)
            for group, field, value in (("variables", "atol", False), ("variables", "rtol", float("nan")),
                                        ("meters", "rmse_max", float("inf")), ("meters", "atol", -1),
                                        ("variables", "frequency", "Daily"), ("variables", "frequency", "Hourly"),
                                        ("meters", "frequency", "TimeStep")):
                changed = json.loads(json.dumps(document))
                changed[group][0][field] = value
                raw = json.dumps(changed).encode("utf8")
                path.write_bytes(raw)
                pin = {"path": name, "sha256": hashlib.sha256(raw).hexdigest(), "bytes": len(raw)}
                with self.subTest(v2_policy=(group, field, value)), self.assertRaises(ValueError):
                    v2.policy(pin)

    def test_csv_and_idf_mdd_bind_actual_noncumulative_authority(self):
        import check_completed_reported_streams_v3 as v3

        name = "Heating:EnergyTransfer"
        csv_header = "Program Version,fixture\nVar Type (reported time step),Var Report Type,Variable Name [Units]\n"
        idf_header = "! Program Version,fixture\n! Output:Meter Objects (applicable to this run)\n"
        csv_line = "Zone,Meter,Heating:EnergyTransfer [J]"
        idf_line = "Output:Meter,Heating:EnergyTransfer,hourly; !- [J]"
        cumulative = "Output:Meter:Cumulative,Heating:EnergyTransfer,hourly; !- [J]"
        regular = v3.meter_dictionary(csv_header + csv_line + "\n")
        idf = v3.meter_dictionary(idf_header + idf_line + "\n" + cumulative + "\n")
        rows = [{"meter": name, "units": "J", "frequency": "Hourly"}]
        source_pin = {"path": "authored-unit-source.mdd", "sha256": "0" * 64, "bytes": 0}
        native_pin = dict(source_pin, path="authored-unit-native.mdd")
        # Pure authored text fixtures only; these pins are carried, not authenticated
        # receipts or physical dictionaries. Production authenticates both pins.
        for source, native in ((regular, regular), (idf, idf), (regular, idf), (idf, regular)):
            with self.subTest(formats=(source["format"], native["format"])):
                bound = v3.meter_bindings(source, native, rows, source_pin, native_pin)
                authority = bound[(name, "J", "Hourly")]
                self.assertEqual(authority["Source"]["raw_line"], source["rows"][(name, False)]["raw_line"])
                self.assertEqual(authority["Native"]["raw_line"], native["rows"][(name, False)]["raw_line"])
                self.assertNotIn("raw_line", authority)
                self.assertEqual((authority["store_type"], authority["cumulative"]), ("Sum", False))
                for actor, document in (("Source", source), ("Native", native)):
                    companion = authority[actor]["cumulative_companion"]
                    self.assertEqual(companion["raw_line"] if companion is not None else None,
                                     cumulative if document["format"] == "IDF" else None)
                membership = v3.MeterMembership(bound)
                self.assertIn(idf_line, membership)
                for query in (cumulative, idf_line.replace("[J]", "[W]"), idf_line.replace("hourly", "daily"),
                              idf_line.replace(name, "Cooling:EnergyTransfer")):
                    self.assertNotIn(query, membership)
        for text in ("", csv_header + "HVAC,Meter," + name + " [J]\n",
                     csv_header + csv_line + "\n" + csv_line + "\n",
                     csv_header + csv_line + "\n" + csv_line.replace("[J]", "[W]") + "\n",
                     csv_header + csv_line + ",extra\n", idf_header + idf_line.replace("hourly", "daily") + "\n",
                     idf_header + idf_line + "\n" + cumulative + "\n" + cumulative + "\n",
                     idf_header + idf_line + "\n" + cumulative.replace("[J]", "[W]") + "\n"):
            with self.subTest(malformed=text), self.assertRaises(ValueError):
                v3.meter_dictionary(text)
        for changed in (v3.meter_dictionary(idf_header + cumulative + "\n"),
                        v3.meter_dictionary(csv_header + csv_line.replace("[J]", "[W]") + "\n"),
                        v3.meter_dictionary(csv_header + csv_line.replace(name, "Cooling:EnergyTransfer") + "\n")):
            with self.subTest(mismatch=changed), self.assertRaises(ValueError):
                v3.meter_bindings(regular, changed, rows, source_pin, native_pin)
        for policies in (rows + rows, [dict(rows[0], frequency="TimeStep")]):
            with self.subTest(policy=policies), self.assertRaises(ValueError):
                v3.meter_bindings(regular, regular, policies, source_pin, native_pin)

    def test_changed_idf_is_rejected_before_any_comparison(self):
        case = json.loads(e.DEFAULT_CASE.read_text(encoding="utf8"))
        with tempfile.TemporaryDirectory() as td:
            td = Path(td)
            (td / case["input"]).write_text("Version,26.1;\n", encoding="utf8")
            cp = td / "case.json"
            cp.write_text(json.dumps(case), encoding="utf8")
            with self.assertRaisesRegex(ValueError, "IDF bytes"):
                e.load_case(cp)

    def test_archived_executable_cannot_substitute_different_build_bytes(self):
        snapshot = {"path": "same-source", "sha256": "same-sha"}
        source = {"source_snapshot": snapshot, "started_utc": "2026-10-20T02:00:00+00:00", "cwd": str(e.ROOT),
                  "command": [str(e.ROOT / "target/debug/eplus-rs.exe"), "run", "input.idf", "--weather", "weather.epw",
                              "--output-dir", "out", "--mode", "compatibility", "--partial", "deny", "--porting-scope", "B"]}
        build = {"source_snapshot": snapshot, "finished_utc": "2026-10-20T01:00:00+00:00",
                 "command": ["cargo", "build", "--message-format=json-render-diagnostics"],
                 "cargo_compiler_artifacts_capture": {"requested": True, "status": "pass", "rows": [{"executable": {
                     "historical_path": "target/debug/eplus-rs.exe", "sha256": "0" * 64}}]}}
        with tempfile.TemporaryDirectory() as td:
            exe = Path(td) / "archived.exe"
            exe.write_bytes(b"different executable")
            with self.assertRaisesRegex(ValueError, "executable bytes differ"):
                e.source_state(source, build, exe)

    def test_wrong_weekday_or_ground_endpoint_is_rejected(self):
        lines = ["Program Version,fixture", "7,1,FLOOR,Surface Outside Face Temperature [C] !TimeStep", "End of Data Dictionary"]
        for hour in range(1, 25):
            for step in range(12):
                lines += [f"2,1,2,29,0,{hour},{step * 5},{(step + 1) * 5},Monday", "7,18.0"]
        lines += ["End of Data", " Number of Records Written=288"]
        with tempfile.TemporaryDirectory() as td:
            p = Path(td) / "fixture.eso"
            p.write_text("\n".join(lines) + "\n", encoding="utf8")
            self.assertTrue(e.stream_scope(p)["Ground_floor_all288_exact18C"])
            for changed in ("\n".join(lines).replace("Monday", "Tuesday", 1), "\n".join(lines).replace("7,18.0", "7,18.000001", 1)):
                p.write_text(changed + "\n", encoding="utf8")
                with self.assertRaises(ValueError):
                    e.stream_scope(p)


if __name__ == "__main__":
    unittest.main()
