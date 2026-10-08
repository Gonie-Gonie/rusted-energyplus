"""Freeze the eleven existing live Products for a forward-only retention policy.

Metadata and hashes only: no target build, strip, engine, copying of binaries,
deletion or historical proof rewriting. New targets start as an empty registry.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[2]
RUNTIME = (ROOT / ".runtime").resolve()
BUILD_SHA = "4fd88058f53aae38b49dc774550cb4b875ec3516275dc323c97fb24f8039bdd9"
REVIEW_SHA = "119811aea18571e2a939690e465c026bbf6e96f3a7e934ea48d631157227e8ee"
POLICY_SHA = "b9aaf18cc2415ace6a8fe6e7c25185f197044ab5ca85b060148d04190459c982"
PIN = "6f2e40d10250a105b49966baa24d843711e61048"
PRIOR = {"clk01_reference", "geo01_reference", "geo02_reference", "geo02_reference_helper",
         "geo03_reference", "geo03_reference_helper", "psy02_reachability", "psy02_reference",
         "psy02_reference_replay"}
OWN = ("zon01_reference", "zon01_reference_helper")


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha(path):
    before = path.stat()
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        while chunk := stream.read(16 * 1024 * 1024):
            digest.update(chunk)
    after = path.stat()
    require((before.st_size, before.st_mtime_ns) == (after.st_size, after.st_mtime_ns),
            "File changed while hashed: " + str(path))
    return digest.hexdigest()


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


def read(path):
    return json.loads(path.read_text(encoding="utf-8"))


def archive(path, output):
    with output.open("xb") as stream:
        stream.write(path.read_bytes())
    require(sha(path) == sha(output), "Metadata/source changed while archiving")
    return {"historical_path": path.relative_to(ROOT).as_posix(), "sha256": sha(output),
            "matching_archive": ref(output)}


def write(path, value):
    with path.open("x", encoding="utf-8", newline="\n") as stream:
        json.dump(value, stream, indent=2, allow_nan=False)
        stream.write("\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("native-build", "build-review", "forward-policy", "output-dir"):
        parser.add_argument("--" + name, required=True)
    args = parser.parse_args()
    require(RUNTIME.is_relative_to(ROOT), "Runtime root resolves outside repository")
    build_path, review_path, policy_path = [contained(value) for value in
        (args.native_build, args.build_review, args.forward_policy)]
    require([sha(path) for path in (build_path, review_path, policy_path)]
            == [BUILD_SHA, REVIEW_SHA, POLICY_SHA], "Unreviewed transition basis")
    build, review = read(build_path), read(review_path)
    require(build["schema"] == "zon01-native-driver-build.v1" and build["checks_passed"] is True
            and build["energyplus_commit"] == PIN and build["scientific_source_patches"] is False,
            "Successful unchanged original build required")
    require(review["schema"] == "zon01-independent-native-build-final-review.v1"
            and review["status"] == "pass-build-identity-before-original-numerical-execution"
            and review["reviewed_build"] == ref(build_path), "Independent build binding differs")
    core_path = bound(build["core_build"])
    core = read(core_path)
    require(core["schema"] == "native-core-build.v1" and core["checks_passed"] is True
            and core["energyplus_commit"] == PIN, "Original core identity differs")
    prior = build["previous_card_products"]
    require(type(prior) is list and len(prior) == 9
            and {Path(row["path"]).stem for row in prior} == PRIOR, "Nine legacy target selectors differ")
    require(set(build["binaries"]) == set(OWN), "Two ZON linked target selectors differ")
    bindings, origins = [], []
    for index, row in enumerate(prior):
        bindings.append(row)
        origins.append({"originating_receipt": ref(build_path),
                        "receipt_selector": f"previous_card_products[{index}]"})
    for name in OWN:
        row = build["binaries"][name]
        require(row["binary"] == row["matching_archive"], "Historical ZON execution archive differs")
        linked = row["linked_build_binary"]
        require(type(linked["historical_path"]) is str and linked["sha256"] == row["binary"]["sha256"],
                "Historical ZON linked SHA differs")
        bound(row["matching_archive"])
        bindings.append({"path": linked["historical_path"], "sha256": linked["sha256"]})
        origins.append({"originating_receipt": ref(build_path),
                        "receipt_selector": f"binaries.{name}.linked_build_binary",
                        "original_historical_identity": linked, "existing_execution_archive": row["binary"]})
    product_paths = [contained(row["path"], RUNTIME) for row in bindings]
    require(len(set(product_paths)) == 11 and len({path.parent for path in product_paths}) == 1
            and all(path.suffix.lower() == ".exe" and path.parent.name == "Products" for path in product_paths),
            "Legacy live Products extent differs")
    protected = [core["artifacts"][key] for key in ("core_library", "api_library")]
    protected.append(build["linked_container_library"])
    for row in protected:
        bound(row)
    for row in bindings:
        bound(row)
    out = contained(args.output_dir, RUNTIME)
    require(not out.exists() and out.parent.is_dir(), "Fresh registry output directory required")
    out.mkdir()
    started = datetime.now(timezone.utc).isoformat()
    sources = {"writer": archive(Path(__file__).resolve(), out / "freeze_native_registry.py"),
        "forward_policy": archive(policy_path, out / "retention-forward-only.md"),
        "native_build": archive(build_path, out / "legacy-native-build.json"),
        "independent_build_review": archive(review_path, out / "legacy-build-review.json"),
        "core_receipt": archive(core_path, out / "core-build.json")}
    legacy = [{"target": path.stem, "retention_class": "legacy-live-product",
               "binary": row, "size_bytes": path.stat().st_size, **origin}
              for row, path, origin in zip(bindings, product_paths, origins)]
    for row in bindings + protected:
        bound(row)
    require(all(sha(contained(row["historical_path"])) == row["sha256"] for row in sources.values()),
            "Source/metadata changed during transition freeze")
    registry = {"schema": "native-retained-artifact-registry.v1", "status": "frozen-legacy-baseline-only",
        "started_utc": started, "finished_utc": datetime.now(timezone.utc).isoformat(),
        "actual_writer_argv": sys.orig_argv, "actual_cwd": str(Path.cwd().resolve()),
        "energyplus_commit": PIN, "transition_sources": sources,
        "legacy_products": legacy, "legacy_product_count": len(legacy), "new_targets": [],
        "protected_core_API_and_container": protected, "live_files_hash_verified_before_and_after": True,
        "source_bytes_match_before_and_after": True, "original_proofs_rewritten": False,
        "binaries_copied": False, "strip_or_build_or_engine_executed": False,
        "Cargo_or_Git_executed": False, "removal_authorized": False,
        "scientific_execution_certified": False, "gates_updated": False,
        "new_target_admission": "A future append must bind genuine link, validated derivative, independent review and actual retention/run receipts; this baseline admits none."}
    write(out / "registry.json", registry)
    print(json.dumps({"registry": ref(out / "registry.json"), "legacy_products": len(legacy), "new_targets": 0}))


if __name__ == "__main__":
    main()
