"""Create a fresh authenticated debug-only derivative; never run/remove its input.

The receipt proves a bounded PE comparison, not scientific execution. Root must
review the receipt before using the derivative in any original-first request.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
import traceback
import zipfile

ROOT = Path(__file__).resolve().parents[2]
RUNTIME = (ROOT / ".runtime").resolve()
STRIP_SHA = "a7e319e63bb15c58c99102460297604fef428dcdf221ae56718d8c079a42722f"
AUDIT_SHA = "5377a49adb9929b5779219689e4ef951197bfd53f2a07f805196c2f2bbf2da5f"
HELP_SHA = "33575e30a7199de605ea8f8db4030a27fdb519d44b28d4fca2010925a519a45a"
PUBLISHER_SHA = "ddc475481139ca95fc520c47bb326bd996ac377919d3fb1a634c8baaca334ab9"
ZIP_SHA = "4889a18dd97d601c1d6c2f141865469979472846751e444840532257dfac44b6"
MEMBER = "mingw64/bin/strip.exe"


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        while chunk := stream.read(16 * 1024 * 1024):
            digest.update(chunk)
    return digest.hexdigest()


def utc():
    return datetime.now(timezone.utc).isoformat()


def contained(value, base=ROOT):
    path = Path(value)
    path = (ROOT / path).resolve() if not path.is_absolute() else path.resolve()
    require(path.is_relative_to(base) and path != base, "Path outside required containment")
    return path


def ref(path):
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": sha(path)}


def bound(item):
    require(type(item) is dict and type(item.get("path")) is str
            and re.fullmatch(r"[0-9a-f]{64}", item.get("sha256", "")), "Malformed artifact ref")
    path = contained(item["path"])
    require(path.is_file() and sha(path) == item["sha256"], "Artifact hash mismatch: " + item["path"])
    return path


def json_file(path):
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path, value):
    with path.open("x", encoding="utf-8", newline="\n") as stream:
        json.dump(value, stream, indent=2, allow_nan=False)
        stream.write("\n")


def archive(path, output):
    with output.open("xb") as stream:
        stream.write(path.read_bytes())
    require(sha(path) == sha(output), "Source/provenance changed while archiving")
    return {"historical_path": path.relative_to(ROOT).as_posix(), "sha256": sha(output),
            "matching_archive": ref(output)}


def observed(path, historical=False):
    if not path.is_file():
        return None
    value = {"historical_path" if historical else "path": path.relative_to(ROOT).as_posix(),
             "sha256": sha(path), "size_bytes": path.stat().st_size}
    if historical:
        value["retention_class"] = "prospective-debug-original"
    return value


def authenticate(args, out):
    audit_path, help_path = contained(args.tool_audit), contained(args.help_receipt)
    require(sha(audit_path) == AUDIT_SHA and sha(help_path) == HELP_SHA, "Unreviewed tool/help receipt")
    audit, help_record = json_file(audit_path), json_file(help_path)
    require(audit["schema"] == "native-reference-efficiency-read-only-audit.v1"
            and help_record["schema"] == "native-strip-local-help-receipt.v1", "Unsupported authentication schema")
    identity = audit["strip_identity"]
    tool = contained(args.strip_tool, RUNTIME)
    require(tool == bound(identity["installed_binary"]) and sha(tool) == STRIP_SHA
            and identity["byte_equal_to_verified_publisher_archive_member"] is True
            and identity["archive_member"] == MEMBER, "Strip publisher-byte identity differs")
    publisher_path = bound(audit["publisher_receipt"])
    require(sha(publisher_path) == PUBLISHER_SHA, "Unreviewed publisher receipt")
    publisher = json_file(publisher_path)
    package = bound(identity["publisher_archive"])
    require(identity["publisher_archive"]["sha256"] == ZIP_SHA
            and publisher["publisher_checksums"]["sha256"] == ZIP_SHA
            and publisher["actual_checksums"]["sha256"] == ZIP_SHA
            and publisher["checksums_match"] is True
            and contained(publisher["archive"]) == package, "Publisher ZIP binding differs")
    with zipfile.ZipFile(package) as zipped, zipped.open(MEMBER) as stream:
        digest = hashlib.sha256()
        while chunk := stream.read(1024 * 1024):
            digest.update(chunk)
    require(digest.hexdigest() == STRIP_SHA, "Actual publisher ZIP member differs")
    require(type(help_record["exit_code"]) is int and help_record["exit_code"] == 0
            and help_record["tool_before"] == help_record["tool_after"] == ref(tool)
            and bound(help_record["prior_publisher_identity_audit"]) == audit_path
            and help_record["actual_argv"] == [str(tool), "--help"]
            and help_record["strip_transformation_executed"] is False, "Help/tool binding differs")
    text = bound(help_record["stdout"]).read_text(encoding="utf-8")
    bound(help_record["stderr"])
    bound(help_record["executed_help_reader"])
    require("--strip-debug" in text and "--preserve-dates" in text and "-o <file>" in text and "pei-x86-64" in text,
            "Recorded tool help lacks required options/format")
    return tool, {"audit": archive(audit_path, out / "tool-audit.json"),
        "help": archive(help_path, out / "tool-help-receipt.json"),
        "publisher": archive(publisher_path, out / "tool-publisher-receipt.json"),
        "publisher_archive": ref(package), "archive_member": MEMBER,
        "actual_member_sha256": digest.hexdigest()}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("original", "original-sha256", "strip-tool", "tool-audit", "help-receipt", "output-dir"):
        parser.add_argument("--" + name, required=True)
    args = parser.parse_args()
    require(RUNTIME.is_relative_to(ROOT), "Runtime root resolves outside repository")
    require(re.fullmatch(r"[0-9a-f]{64}", args.original_sha256), "Invalid original SHA256")
    original = contained(args.original, RUNTIME)
    out = contained(args.output_dir, RUNTIME)
    require(original.is_file() and original.suffix.lower() == ".exe"
            and original.parent.name == "Products", "Only explicit linked Products/*.exe input supported")
    require(not out.exists() and out.parent.is_dir() and not original.is_relative_to(out), "Fresh output directory required")
    out.mkdir()
    derived = out / "derived-helper.exe"
    reader_path = ROOT / "tools/porting/native_pe.py"
    record = {"schema": "validated-debug-derivative.v1", "started_utc": utc(),
        "actual_driver_argv": sys.orig_argv, "cwd": str(Path.cwd().resolve()), "expected_original_sha256": args.original_sha256,
        "executed_driver": archive(Path(__file__).resolve(), out / "derive_native_helper.py"),
        "executed_PE_reader": archive(reader_path, out / "native_pe.py"),
        "original_before": None, "original_after": None, "strip_command": None,
        "tool_before": None, "tool_after": None, "helper_binary": None,
        "maps": {}, "comparison": None, "status": "failed", "failure": None,
        "original_removed": False, "original_removal_authorized": False,
        "full_original_copied": False, "engines_compilers_Cargo_Git_executed": False,
        "scientific_execution_certified": False, "gates_updated": False}
    strip_exit = None
    tool = None
    try:
        sys.dont_write_bytecode = True
        spec = importlib.util.spec_from_file_location("derivative_PE_reader", out / "native_pe.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        record["tool_authentication"] = None
        tool, record["tool_authentication"] = authenticate(args, out)
        record["tool_before"] = ref(tool)
        record["original_before"] = observed(original, historical=True)
        require(record["original_before"]["sha256"] == args.original_sha256, "Linked original SHA256 differs")
        before = module.inspect(original)
        require(before["sha256"] == record["original_before"]["sha256"]
                and before["size_bytes"] == record["original_before"]["size_bytes"], "Original changed before PE map")
        write_json(out / "original-before-pe.json", before)
        record["maps"]["original_before"] = ref(out / "original-before-pe.json")
        require(not derived.exists(), "Derivative output already exists")
        argv = [str(tool), "--strip-debug", "--preserve-dates", "-o", str(derived), str(original)]
        command = {"schema": "native-strip-command.v1", "command": argv, "cwd": str(ROOT),
                   "started_utc": utc(), "exit_code": None}
        record["strip_command"] = command
        try:
            with (out / "strip-stdout.log").open("xb") as stdout, (out / "strip-stderr.log").open("xb") as stderr:
                result = subprocess.run(argv, cwd=ROOT, stdout=stdout, stderr=stderr, check=False, shell=False)
            strip_exit = result.returncode
        finally:
            command.update(finished_utc=utc(), exit_code=strip_exit)
            for key in ("stdout", "stderr"):
                path = out / f"strip-{key}.log"
                command[key] = ref(path) if path.is_file() else None
            write_json(out / "strip-command.json", command)
            record["strip_command_receipt"] = ref(out / "strip-command.json")
        after = module.inspect(original)
        write_json(out / "original-after-pe.json", after)
        record["maps"]["original_after"] = ref(out / "original-after-pe.json")
        require(before == after, "Original PE changed through transformation")
        require(strip_exit == 0, "strip returned nonzero exit")
        derived_map = module.inspect(derived)
        write_json(out / "derived-pe.json", derived_map)
        record["maps"]["derived"] = ref(out / "derived-pe.json")
        record["comparison"] = module.compare(before, derived_map)
        write_json(out / "pe-comparison.json", record["comparison"])
        record["maps"]["comparison"] = ref(out / "pe-comparison.json")
        accepted = observed(derived)
        require(accepted["sha256"] == derived_map["sha256"]
                and accepted["size_bytes"] == derived_map["size_bytes"], "Derivative changed after accepted PE map")
        record["accepted_derivative"] = accepted
        record["helper_binary"] = {"path": accepted["path"], "sha256": accepted["sha256"]}
        record["status"] = "pass-exact-retained-runtime-sections-and-directories"
    except Exception as error:
        record["failure"] = {"type": type(error).__name__, "message": str(error)}
        with (out / "failure.log").open("x", encoding="utf-8", newline="\n") as stream:
            stream.write(traceback.format_exc())
        record["failure_log"] = ref(out / "failure.log")
    finally:
        try:
            record["original_after"] = observed(original, historical=True)
            record["tool_after"] = ref(tool) if tool is not None and tool.is_file() else None
            record["observed_derivative"] = observed(derived)
            sources_exact = all(sha(contained(record[key]["historical_path"])) == record[key]["sha256"]
                                for key in ("executed_driver", "executed_PE_reader"))
        except Exception as error:
            record["final_observation_error"] = {"type": type(error).__name__, "message": str(error)}
            sources_exact = False
        record["original_bytes_unchanged"] = record["original_before"] is not None and record["original_before"] == record["original_after"]
        record["source_bytes_match_before_and_after"] = sources_exact
        record["accepted_derivative_bytes_unchanged"] = record.get("accepted_derivative") is not None and record["accepted_derivative"] == record.get("observed_derivative")
        if not (record["original_bytes_unchanged"] and record["tool_before"] == record["tool_after"] and sources_exact
                and (not record["status"].startswith("pass-") or record["accepted_derivative_bytes_unchanged"])):
            record["status"] = "failed"
            record["helper_binary"] = None
            record["preservation_failure"] = True
        record["finished_utc"] = utc()
        write_json(out / "receipt.json", record)
    print(json.dumps({"receipt": ref(out / "receipt.json"), "status": record["status"]}))
    return 0 if record["status"].startswith("pass-") else strip_exit if strip_exit not in (None, 0) else 1


if __name__ == "__main__":
    raise SystemExit(main())
