"""Fetch the locked EnergyPlus source files referenced by the algorithm ledger."""

from __future__ import annotations

import argparse
import copy
import tomllib
import urllib.error
import urllib.request
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from urllib.parse import urlparse

from validate_algorithm_ledger import is_safe_repo_relative_ref


SUPPORTED_ENERGYPLUS_VERSION = "26.1.0"
SUPPORTED_ENERGYPLUS_TAG = "v26.1.0"
SUPPORTED_ENERGYPLUS_COMMIT = "6f2e40d10250a105b49966baa24d843711e61048"
SUPPORTED_ENERGYPLUS_REPOSITORY = "NREL/EnergyPlus"
SUPPORTED_SOURCE_ARCHIVE_FILE = "EnergyPlus-v26.1.0-source.zip"
SUPPORTED_SOURCE_ARCHIVE_URL = "https://github.com/NREL/EnergyPlus/archive/refs/tags/v26.1.0.zip"


@dataclass(frozen=True)
class EnergyPlusReference:
    version: str
    commit: str
    repository: str
    source_archive_file: str

    @property
    def raw_base_url(self) -> str:
        return f"https://raw.githubusercontent.com/{self.repository}/{self.commit}"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Fetch the locked EnergyPlus ledger source subset.")
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    return parser.parse_args()


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def validate_energyplus_reference_lock(lock: dict[str, Any], source: str) -> EnergyPlusReference:
    oracle = lock.get("oracle")
    if not isinstance(oracle, dict):
        raise ValueError(f"missing [oracle] table in {source}")
    source_archive = oracle.get("source_zip")
    if not isinstance(source_archive, dict):
        raise ValueError(f"missing [oracle.source_zip] table in {source}")

    version = str(oracle.get("version", "")).strip()
    tag = str(oracle.get("tag", "")).strip()
    oracle_commit = str(oracle.get("commit", "")).strip()
    source_commit = str(source_archive.get("commit", "")).strip()
    source_archive_file = str(source_archive.get("file", "")).strip()
    source_archive_url = str(source_archive.get("url", "")).strip()
    if version != SUPPORTED_ENERGYPLUS_VERSION:
        raise ValueError(
            f"unsupported EnergyPlus version in {source}: {version}; "
            f"expected {SUPPORTED_ENERGYPLUS_VERSION}"
        )
    if tag != SUPPORTED_ENERGYPLUS_TAG:
        raise ValueError(
            f"unsupported EnergyPlus tag in {source}: {tag}; expected {SUPPORTED_ENERGYPLUS_TAG}"
        )
    if oracle_commit != SUPPORTED_ENERGYPLUS_COMMIT:
        raise ValueError(
            f"unsupported EnergyPlus commit in {source}: {oracle_commit}; "
            f"expected {SUPPORTED_ENERGYPLUS_COMMIT}"
        )
    if source_commit != SUPPORTED_ENERGYPLUS_COMMIT:
        raise ValueError(
            f"unsupported EnergyPlus source commit in {source}: {source_commit}; "
            f"expected {SUPPORTED_ENERGYPLUS_COMMIT}"
        )
    if source_archive_file != SUPPORTED_SOURCE_ARCHIVE_FILE:
        raise ValueError(
            f"unsupported EnergyPlus source archive file in {source}: {source_archive_file}; "
            f"expected {SUPPORTED_SOURCE_ARCHIVE_FILE}"
        )
    parsed_url = urlparse(source_archive_url)
    path_parts = [part for part in parsed_url.path.split("/") if part]
    if (
        parsed_url.scheme != "https"
        or parsed_url.netloc.lower() != "github.com"
        or len(path_parts) < 3
        or path_parts[2] != "archive"
    ):
        raise ValueError(f"unsupported EnergyPlus source archive URL in {source}: {source_archive_url}")
    repository = "/".join(path_parts[:2])
    if repository != SUPPORTED_ENERGYPLUS_REPOSITORY:
        raise ValueError(
            f"unsupported EnergyPlus repository in {source}: {repository}; "
            f"expected {SUPPORTED_ENERGYPLUS_REPOSITORY}"
        )
    if source_archive_url != SUPPORTED_SOURCE_ARCHIVE_URL:
        raise ValueError(
            f"unsupported canonical EnergyPlus source archive URL in {source}: {source_archive_url}; "
            f"expected {SUPPORTED_SOURCE_ARCHIVE_URL}"
        )
    return EnergyPlusReference(
        version=version,
        commit=source_commit,
        repository=repository,
        source_archive_file=source_archive_file,
    )


def load_energyplus_reference(lock_path: Path) -> EnergyPlusReference:
    return validate_energyplus_reference_lock(load_toml(lock_path), str(lock_path))


def run_reference_contract_self_tests(lock_path: Path) -> int:
    baseline = load_toml(lock_path)
    validate_energyplus_reference_lock(baseline, str(lock_path))
    mutations = [
        ("version", ("oracle", "version"), "26.2.0", "unsupported EnergyPlus version"),
        ("tag", ("oracle", "tag"), "v26.2.0", "unsupported EnergyPlus tag"),
        (
            "oracle_commit",
            ("oracle", "commit"),
            "5f5c37ce79025f0be53868421048617051ba8022",
            "unsupported EnergyPlus commit",
        ),
        (
            "source_commit",
            ("oracle", "source_zip", "commit"),
            "5f5c37ce79025f0be53868421048617051ba8022",
            "unsupported EnergyPlus source commit",
        ),
        (
            "archive_file",
            ("oracle", "source_zip", "file"),
            "EnergyPlus-develop-source.zip",
            "unsupported EnergyPlus source archive file",
        ),
        (
            "repository",
            ("oracle", "source_zip", "url"),
            "https://github.com/example/EnergyPlus/archive/refs/tags/v26.1.0.zip",
            "unsupported EnergyPlus repository",
        ),
        (
            "archive_tag",
            ("oracle", "source_zip", "url"),
            "https://github.com/NREL/EnergyPlus/archive/refs/tags/v26.2.0.zip",
            "unsupported canonical EnergyPlus source archive URL",
        ),
    ]
    passed: list[str] = []
    for name, path, replacement, expected_error in mutations:
        candidate = copy.deepcopy(baseline)
        table: dict[str, Any] = candidate
        for key in path[:-1]:
            nested = table.get(key)
            if not isinstance(nested, dict):
                raise AssertionError(f"{name}: mutation path is not a table: {'.'.join(path)}")
            table = nested
        table[path[-1]] = replacement
        try:
            validate_energyplus_reference_lock(candidate, f"self-test:{name}")
        except ValueError as error:
            if expected_error not in str(error):
                raise AssertionError(
                    f"{name}: expected error containing {expected_error!r}; got {error}"
                ) from error
        else:
            raise AssertionError(f"{name}: unsupported reference mutation was accepted")
        passed.append(name)

    print("EnergyPlus supported-reference self-test")
    print(f"  mutations: {len(passed)}")
    for name in passed:
        print(f"  OK {name}")
    return 0


def ledger_sources(ledger: dict[str, Any]) -> list[str]:
    sources: set[str] = set()
    for algorithm in ledger.get("algorithm", []):
        if not isinstance(algorithm, dict):
            continue
        sources.update(str(value).strip() for value in algorithm.get("energyplus_source", []))
        routine_map = algorithm.get("routine", {})
        if isinstance(routine_map, dict):
            sources.update(
                str(routine.get("source_file", "")).strip()
                for routine in routine_map.values()
                if isinstance(routine, dict)
            )
    for source in sources:
        normalized = source.replace("\\", "/")
        if (
            not is_safe_repo_relative_ref(normalized)
            or not normalized.startswith("src/EnergyPlus/")
            or not normalized.lower().endswith((".cc", ".cpp", ".cxx", ".hh", ".hpp"))
        ):
            raise ValueError(f"unsafe or unsupported EnergyPlus ledger source path: {source}")
    return sorted(sources)


def download_source(
    source: str,
    reference_root: Path,
    raw_base_url: str,
    force: bool,
) -> tuple[str, str]:
    target = reference_root / source
    if target.is_file() and not force:
        return source, "cached"
    url = f"{raw_base_url}/{source}"
    request = urllib.request.Request(url, headers={"User-Agent": "rusted-energyplus-source-gate"})
    last_error: Exception | None = None
    for _attempt in range(3):
        try:
            with urllib.request.urlopen(request, timeout=60) as response:
                content = response.read()
            if not content:
                raise RuntimeError(f"empty response for {url}")
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(content)
            return source, "downloaded"
        except (OSError, RuntimeError, urllib.error.URLError) as error:
            last_error = error
    raise RuntimeError(f"failed to fetch {url}: {last_error}")


def main() -> int:
    args = parse_args()
    repo_root = args.repo_root.resolve()
    lock_path = repo_root / "tools" / "oracle" / "energyplus.lock.toml"
    if args.self_test:
        return run_reference_contract_self_tests(lock_path)
    reference = load_energyplus_reference(lock_path)
    ledger_path = repo_root / "specs" / "algorithm_ledger.toml"
    sources = ledger_sources(load_toml(ledger_path))
    print("EnergyPlus reference subset")
    print(f"  version: {reference.version}")
    print(f"  commit: {reference.commit}")
    print(f"  sources: {len(sources)}")
    if args.dry_run:
        for source in sources:
            print(f"  {source}")
        return 0

    reference_root = repo_root / ".reference" / "energyplus-src" / reference.version
    results: dict[str, str] = {}
    with ThreadPoolExecutor(max_workers=min(8, max(1, len(sources)))) as executor:
        futures = {
            executor.submit(
                download_source,
                source,
                reference_root,
                reference.raw_base_url,
                args.force,
            ): source
            for source in sources
        }
        for future in as_completed(futures):
            source, status = future.result()
            results[source] = status
    print(f"  downloaded: {sum(status == 'downloaded' for status in results.values())}")
    print(f"  cached: {sum(status == 'cached' for status in results.values())}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
