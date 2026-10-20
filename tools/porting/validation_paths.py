"""Locate current validation assets without rewriting frozen proof identities.

Only the repository's former ``energyplus_porting_plan`` directory is aliased.
Historical JSON bindings retain their original paths and hashes. Callers still
own their containment, hash, schema and execution checks; no missing-file or
archive fallback is provided here.
"""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LEGACY_ROOT = ROOT / "energyplus_porting_plan"
VALIDATION_ROOT = ROOT / "docs/validation"


def resolve_path(value: str | Path) -> Path:
    """Return the actual path for a current or historical repository binding."""
    path = Path(value)
    path = (path if path.is_absolute() else ROOT / path).resolve()
    if path.is_relative_to(LEGACY_ROOT):
        return (VALIDATION_ROOT / path.relative_to(LEGACY_ROOT)).resolve()
    return path


def legacy_path(value: str | Path) -> Path:
    """Keep the original path identity when serializing relocated proof data."""
    path = resolve_path(value)
    if path.is_relative_to(VALIDATION_ROOT):
        return LEGACY_ROOT / path.relative_to(VALIDATION_ROOT)
    return path


def legacy_relative(value: str | Path) -> str:
    """Serialize a repository binding, retaining the caller's containment rule."""
    return legacy_path(value).relative_to(ROOT).as_posix()
