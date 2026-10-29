"""Lexer regressions for factual inventories, with no engine execution."""

from pathlib import Path
import tempfile
import unittest

from example_object_union import (
    catalog_from_schema, collect_union, compiler_typed_names, idf_records, input_typing, object_counts, object_registry,
)
from unittest.mock import patch
import json


class ExampleObjectUnionTests(unittest.TestCase):
    def test_comments_bom_blank_fields_and_adjacent_records(self) -> None:
        self.assertEqual(
            idf_records("\ufeff! header\r\nVersion,26.1; Zone, Room, ,0;! end"),
            [["Version", "26.1"], ["Zone", "Room", "", "0"]],
        )

    def test_quoted_delimiters_and_comment_markers_are_data(self) -> None:
        self.assertEqual(
            idf_records('Zone, "Name; with, ! punctuation";'),
            [["Zone", "Name; with, ! punctuation"]],
        )

    def test_single_quotes_doubled_quotes_and_literal_apostrophe(self) -> None:
        self.assertEqual(
            idf_records("Zone,'John''s room';Zone,John's room;Zone,\"a\"\"b\";"),
            [["Zone", "John's room"], ["Zone", "John's room"], ["Zone", 'a"b']],
        )

    def test_empty_or_comment_only_input_has_no_records(self) -> None:
        self.assertEqual(idf_records(" \r\n! comment,; ' \n"), [])

    def test_unterminated_or_nameless_records_reject(self) -> None:
        for text in ["Zone, X", "Zone, 'X;", ", X;", ";"]:
            with self.subTest(text=text), self.assertRaises(ValueError):
                idf_records(text)

    def test_epjson_counts_instances_and_keeps_unknown_types(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "input.epJSON"
            path.write_text('{"Zone":{"A":{},"B":{}},"Unknown:Diagnostic":{"X":{}},"Version":{}}', encoding="utf-8")
            self.assertEqual(object_counts(path), {"Zone": 2, "Unknown:Diagnostic": 1})

    def test_duplicate_json_keys_reject_instead_of_losing_occurrences(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "input.epJSON"
            path.write_text('{"Zone":{"A":{},"A":{}}}', encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "duplicate JSON key"):
                object_counts(path)

    def test_epjson_non_instance_map_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "input.epJSON"
            path.write_text('{"Zone":[]}', encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "instance maps"):
                object_counts(path)

    def test_catalog_requires_exact_pinned_schema_bytes(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "schema.json"
            path.write_text('{"properties":{"Fake:Object":{}}}', encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "schema bytes differ"):
                catalog_from_schema(path)


    def classification_fixture(self, root: Path, rows: str) -> None:
        (root / "specs").mkdir(parents=True)
        (root / "crates/ep_compiler/src").mkdir(parents=True)
        (root / "crates/ep_compiler/src/compiler.rs").write_text(
            'const ALIAS: &str = "Construction:AirBoundary";\n'
            'const TYPED_OBJECT_TYPES: &[&str] = &["Zone", ALIAS];', encoding="utf8")
        (root / "specs/object_coverage.toml").write_text('coverage_schema = 2\n' + rows, encoding="utf8")

    def test_compiler_aliases_and_typed_model_do_not_imply_source_admission(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.classification_fixture(root, '[[' + 'object]]\nname="Zone"\nstatus="typed_graph_only"\ncompiler_coverage="typed"\nsource_admission="blocked"\nsource_boundary="No connected Source consumer"\nsource_refs=["crates/ep_compiler/src/compiler.rs"]\n')
            self.assertEqual(compiler_typed_names(root), {"Zone", "Construction:AirBoundary"})
            row = object_registry(root)["Zone"]
            self.assertEqual(input_typing(row), "Typed model; graph only")
            self.assertEqual(row["source_admission"], "blocked")
            registry_path = root / "specs/object_coverage.toml"
            registry_path.write_text(registry_path.read_text(encoding="utf8").replace(
                'source_admission="blocked"', 'source_admission="bounded_a"'), encoding="utf8")
            admitted = object_registry(root)["Zone"]
            self.assertEqual(admitted["source_admission"], "bounded_a")
            self.assertEqual(input_typing(admitted), "Typed model; graph only")

    def test_raw_getter_is_not_falsely_labelled_typed(self) -> None:
        self.assertEqual(input_typing({"compiler_coverage": "raw_only", "status": "raw_source_getter", "source_admission": "bounded_a_b"}), "Raw only; Source getter")
        self.assertEqual(input_typing({"compiler_coverage": "raw_only", "status": "partial_input", "source_admission": "blocked"}), "Raw only; partial projection")
        self.assertEqual(input_typing({"compiler_coverage": "raw_only", "status": "typed", "source_admission": "bounded_a"}), "Raw only; Source getter")

    def test_registry_rejects_missing_or_false_classification(self) -> None:
        base = '[[object]]\nname="Zone"\nstatus="typed_graph_only"\ncompiler_coverage="typed"\nsource_admission="blocked"\nsource_boundary="No consumer"\nsource_refs=["crates/ep_compiler/src/compiler.rs"]\n'
        variants = [(base + base, "duplicate object"),
                    (base.replace('status="typed_graph_only"', 'status="full_support"'), "unknown object handling"),
                    (base.replace('compiler_coverage="typed"', 'compiler_coverage="raw_only"'), "actual bitmap"),
                    (base.replace('source_admission="blocked"', 'source_admission="full_support"'), "unknown Source admission"),
                    (base.replace('source_boundary="No consumer"', 'source_boundary=""'), "boundary and refs")]
        for rows, message in variants:
            with self.subTest(message=message), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                self.classification_fixture(root, rows)
                with self.assertRaisesRegex(ValueError, message):
                    object_registry(root)

    def test_new_canonical_corpus_object_requires_registry_review_but_unknown_fixture_does_not(self) -> None:
        from example_object_union import SCHEMA_SHA, EP_COMMIT
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.classification_fixture(root, "")
            (root / "data/schema").mkdir(parents=True)
            names = {"Zone": "Thermal Zones", **{f"Unused:{n}": "Other" for n in range(857)}}
            (root / "data/schema/energyplus-26.1-object-index.json").write_text(json.dumps({
                "schema": "energyplus-object-name-index.v1", "authority": {"energyplus_version": "26.1.0", "schema_sha256": SCHEMA_SHA, "energyplus_commit": EP_COMMIT}, "object_types": names}), encoding="utf8")
            sample = root / "data/input.idf"
            sample.write_text("Zone,A;Unknown:Diagnostic,B;", encoding="utf8")
            with patch("example_object_union.sample_paths", return_value=[sample]):
                with self.assertRaisesRegex(ValueError, "canonical corpus object lacks reviewed classification: Zone"):
                    collect_union(root)
                sample.write_text("Unknown:Diagnostic,B;", encoding="utf8")
                self.assertFalse(collect_union(root)["rows"][0]["schema_known"])


if __name__ == "__main__":
    unittest.main()
