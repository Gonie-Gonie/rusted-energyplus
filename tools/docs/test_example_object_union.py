"""Lexer regressions for factual inventories, with no engine execution."""

from pathlib import Path
import tempfile
import unittest

from example_object_union import catalog_from_schema, idf_records, object_counts


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


if __name__ == "__main__":
    unittest.main()
