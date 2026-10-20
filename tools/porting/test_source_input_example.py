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
