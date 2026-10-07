"""Install pinned, repository-local C++ reference tools without changing system PATH."""

from __future__ import annotations

import argparse
import hashlib
import json
import platform
import stat
import subprocess
import urllib.request
import zipfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
LOCK = Path(__file__).with_name("reference_tools.json")
DIRECTORY = ROOT / ".runtime/reference-tools"


def digest(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def install(tool: dict, check_only: bool) -> dict:
    directory = DIRECTORY / f"{tool['name']}-{tool['version']}"
    executable = directory / tool["executable"]
    receipt = directory / "installation.json"
    if not executable.is_file() or not receipt.is_file():
        if check_only:
            raise ValueError(f"missing {tool['name']}; run without --check")
        if directory.exists():
            raise ValueError(f"incomplete install exists; inspect it before retry: {directory}")
        downloads = DIRECTORY / "downloads"
        downloads.mkdir(parents=True, exist_ok=True)
        archive = downloads / tool["url"].rsplit("/", 1)[1]
        if not archive.exists():
            request = urllib.request.Request(tool["url"], headers={"User-Agent": "rusted-energyplus-porting"})
            print(f"Downloading {tool['name']} {tool['version']}", flush=True)
            with urllib.request.urlopen(request, timeout=60) as response, archive.open("xb") as destination:
                while chunk := response.read(1024 * 1024):
                    destination.write(chunk)
        if digest(archive) != tool["sha256"]:
            raise ValueError(f"download SHA256 mismatch; inspect {archive}")
        with zipfile.ZipFile(archive) as bundle:
            for item in bundle.infolist():
                path = Path(item.filename)
                target = (directory / path).resolve()
                if (path.is_absolute() or ".." in path.parts or "\\" in item.filename or
                        ":" in item.filename or not target.is_relative_to(directory.resolve()) or
                        stat.S_ISLNK(item.external_attr >> 16)):
                    raise ValueError(f"unsafe archive member: {item.filename}")
            print(f"Extracting verified {tool['name']}", flush=True)
            bundle.extractall(directory)
        if not executable.is_file():
            raise ValueError(f"expected executable missing after extraction: {executable}")
        receipt.write_text(json.dumps({"tool": tool, "executable_sha256": digest(executable)}, indent=2) + "\n", encoding="utf-8")
    installed = json.loads(receipt.read_text(encoding="utf-8"))
    if installed.get("tool") != tool or installed.get("executable_sha256") != digest(executable):
        raise ValueError(f"installation receipt differs from pinned tool: {directory}")
    result = subprocess.run([str(executable), "--version"], capture_output=True, text=True, timeout=30, check=True)
    version = result.stdout.strip().splitlines()[0]
    print(f"PASS {tool['name']}: {version}", flush=True)
    return {"name": tool["name"], "path": str(executable.relative_to(ROOT)), "version": version}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="verify an existing install without downloading or writing")
    args = parser.parse_args()
    if platform.system() != "Windows" or platform.machine().upper() not in {"AMD64", "X86_64"}:
        parser.error("this pinned reference toolchain targets Windows x86_64")
    lock = json.loads(LOCK.read_text(encoding="utf-8"))
    for tool in lock["tools"]:
        install(tool, args.check)
    print("Reference tools ready; system PATH unchanged.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
