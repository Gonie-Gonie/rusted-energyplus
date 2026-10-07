"""Read-only porting-plan checks; completion metadata is not numerical certification.

Usage: python tools/porting/check_plan.py --check [--card CON-01] [--scope A]
       python tools/porting/check_plan.py --self-test

Pending imported cards need no evidence. Passed gates use the Korean status
``통과`` (other statuses: ``미확인``, ``실패``). Optional ``scope_gates`` maps
A/B to all four gate values, overriding the shared values for that scope.
Completed cards require ``evidence`` or ``evidence_by_scope[A/B]`` containing:
  scopes; implementation_commit; source_ranges[{path,symbol,start_line,end_line,
  helpers}]; input_hashes[{path,sha256}]; reference{kind,artifacts};
  checks{<gate>:{command,commit,exit_code,artifacts}};
  production{rust_entrypoint,oracle_inputs_used:false,fixture_inputs_used:false};
  comparison{contract,report}.
Every artifact/contract/report is {path,sha256}, relative to the repository.
Commands are recorded, never executed. Reviewers must still assess scientific
validity, source closure, active branches, timing, and numerical tolerances.
"""

from __future__ import annotations

import argparse
import copy
import functools
import hashlib
import json
import re
import subprocess
import sys
import tomllib
from pathlib import Path
from typing import Any, Callable


GATES = ("scope_review", "unit_gate", "integration_gate", "production_gate")
STATUSES = {"미확인", "통과", "실패"}
SCOPES = {"A", "B"}
EXPECTED_IDS = {"CON-01"} | {
    f"{group}-{index:02d}"
    for group, count in (
        ("GEO", 3), ("CLK", 6), ("SCH", 3), ("PSY", 2), ("CTF", 10),
        ("SRC", 8), ("RAD", 4), ("SUR", 7), ("ZON", 6), ("HVAC", 8), ("SYS", 6),
    )
    for index in range(1, count + 1)
}
ReadBytes = Callable[[Path], bytes]


@functools.lru_cache(maxsize=256)
def revision_exists(root: Path, revision: str) -> bool:
    """Accept an existing tested commit/tree, without requiring mutable HEAD."""
    try:
        result = subprocess.run(
            ["git", "cat-file", "-t", revision], cwd=root,
            capture_output=True, text=True, timeout=10, check=False,
        )
    except (OSError, subprocess.TimeoutExpired):
        return False
    return result.returncode == 0 and result.stdout.strip() in {"commit", "tree"}


def task_scopes(task: dict[str, Any]) -> set[str]:
    value = task.get("applicability")
    return set(value.split("/")) if isinstance(value, str) else set()


def gate_values(task: dict[str, Any], scope: str) -> dict[str, Any]:
    overrides = task.get("scope_gates", {})
    if isinstance(overrides, dict) and isinstance(overrides.get(scope), dict):
        return overrides[scope]
    return {gate: task.get(gate) for gate in GATES}


def complete(task: dict[str, Any], scope: str) -> bool:
    return scope in task_scopes(task) and all(
        value == "통과" for value in gate_values(task, scope).values()
    ) and set(gate_values(task, scope)) == set(GATES)


def dependencies(task: dict[str, Any], scope: str) -> list[str]:
    regular = task.get("dependencies", [])
    conditional = task.get("conditional_dependencies", {})
    extras = conditional.get(scope, []) if isinstance(conditional, dict) else []
    return (regular if isinstance(regular, list) else []) + (
        extras if isinstance(extras, list) else []
    )


def check_evidence(
    evidence: Any, card: str, scope: str, root: Path, source_root: Path, read: ReadBytes,
) -> list[str]:
    errors: list[str] = []
    prefix = f"{card}/{scope} evidence"

    def fail(message: str) -> None:
        errors.append(f"{prefix}: {message}")

    def asset(value: Any, label: str) -> None:
        if not isinstance(value, dict):
            fail(f"{label} must contain path and sha256")
            return
        path, digest = value.get("path"), value.get("sha256")
        if not isinstance(path, str) or not path or Path(path).is_absolute():
            fail(f"{label}.path must be repository-relative")
            return
        resolved = (root / path).resolve()
        if not resolved.is_relative_to(root.resolve()):
            fail(f"{label}.path escapes repository")
            return
        if not isinstance(digest, str) or not re.fullmatch(r"[0-9a-f]{64}", digest):
            fail(f"{label}.sha256 must be a full lowercase SHA256")
            return
        try:
            actual = hashlib.sha256(read(resolved)).hexdigest()
        except OSError as error:
            fail(f"{label} cannot be read: {error}")
            return
        if actual != digest:
            fail(f"{label} SHA256 mismatch: {path}")

    def assets(values: Any, label: str) -> None:
        if not isinstance(values, list) or not values:
            fail(f"{label} needs at least one hashed artifact")
            return
        for index, value in enumerate(values):
            asset(value, f"{label}[{index}]")

    if not isinstance(evidence, dict):
        return [f"{prefix}: completed gates require replayable evidence metadata"]
    scopes = evidence.get("scopes")
    if (not isinstance(scopes, list) or not scopes or
            any(not isinstance(value, str) or value not in SCOPES for value in scopes) or
            len(set(scopes)) != len(scopes) or scope not in scopes):
        fail(f"scopes must explicitly include {scope}")
    commit = evidence.get("implementation_commit")
    if not isinstance(commit, str) or not re.fullmatch(r"[0-9a-f]{40}", commit):
        fail("implementation_commit must be a full lowercase Git commit")
    elif not revision_exists(root, commit):
        fail("implementation_commit must identify an existing Git commit/tree")
    ranges = evidence.get("source_ranges")
    if not isinstance(ranges, list) or not ranges:
        fail("source_ranges needs the reviewed EP symbols and exact line ranges")
    else:
        for index, item in enumerate(ranges):
            if not isinstance(item, dict):
                fail(f"source_ranges[{index}] must be an object")
                continue
            source, symbol = item.get("path"), item.get("symbol")
            start, end = item.get("start_line"), item.get("end_line")
            helpers = item.get("helpers")
            if (not isinstance(source, str) or not source.startswith("src/EnergyPlus/") or
                    ".." in Path(source).parts or not isinstance(symbol, str) or not symbol.strip() or
                    type(start) is not int or type(end) is not int or not 1 <= start <= end or
                    not isinstance(helpers, list) or
                    any(not isinstance(helper, str) or not helper.strip() for helper in helpers)):
                fail(f"source_ranges[{index}] requires EP path, symbol, lines, and helpers")
                continue
            source_path = (source_root / source).resolve()
            if not source_path.is_relative_to(root.resolve()):
                fail(f"source_ranges[{index}] escapes the repository's locked reference")
                continue
            try:
                lines = read(source_path).decode("utf-8-sig").splitlines()
            except (OSError, UnicodeError) as error:
                fail(f"source_ranges[{index}] cannot read locked EP source: {error}")
                continue
            if end > len(lines) or not any(symbol in line for line in lines[start - 1:end]):
                fail(f"source_ranges[{index}] lines must include the named symbol in locked EP source")
    assets(evidence.get("input_hashes"), "input_hashes")
    reference = evidence.get("reference", {})
    if (not isinstance(reference, dict) or not isinstance(reference.get("kind"), str) or
            reference["kind"] not in {"cpp-wrapper", "ep-trace"}):
        fail("reference.kind must be cpp-wrapper or ep-trace")
        reference = {}
    assets(reference.get("artifacts"), "reference.artifacts")
    checks = evidence.get("checks", {})
    if not isinstance(checks, dict):
        checks = {}
    for gate in GATES:
        check = checks.get(gate, {})
        if not isinstance(check, dict):
            check = {}
        command, tested_commit = check.get("command"), check.get("commit")
        if not isinstance(command, str) or not command.strip():
            fail(f"checks.{gate}.command is required for replay")
        if not isinstance(tested_commit, str) or not re.fullmatch(r"[0-9a-f]{40}", tested_commit):
            fail(f"checks.{gate}.commit must identify the actual tested commit")
        elif not revision_exists(root, tested_commit):
            fail(f"checks.{gate}.commit must identify an existing Git commit/tree")
        if type(check.get("exit_code")) is not int or check["exit_code"] != 0:
            fail(f"checks.{gate}.exit_code must record successful execution")
        assets(check.get("artifacts"), f"checks.{gate}.artifacts")
    production = evidence.get("production", {})
    if not isinstance(production, dict):
        production = {}
    entrypoint = production.get("rust_entrypoint")
    if not isinstance(entrypoint, str) or not entrypoint.strip():
        fail("production.rust_entrypoint is required")
    for field in ("oracle_inputs_used", "fixture_inputs_used"):
        if production.get(field) is not False:
            fail(f"production.{field} must explicitly be false")
    comparison = evidence.get("comparison", {})
    if not isinstance(comparison, dict):
        comparison = {}
    for field in ("contract", "report"):
        asset(comparison.get(field), f"comparison.{field}")
    return errors


def validate_plan(
    plan: Any, lock: dict[str, Any], root: Path,
    plan_dir: Path, read: ReadBytes = Path.read_bytes,
) -> list[str]:
    """Validate the whole plan in both scopes regardless of display selection."""
    errors: list[str] = []
    if not isinstance(plan, dict):
        return ["plan must be a JSON object"]
    if plan.get("schema") != "energyplus-porting-work-plan.v1":
        errors.append("unexpected plan schema")
    for field, key in (("ep_version", "energyplus_version"), ("ep_commit", "source_commit")):
        if plan.get(field) != lock.get(key) or not lock.get(key):
            errors.append(f"{field} differs from canonical config/default.toml oracle lock")
    if not isinstance(plan.get("scopes"), dict) or set(plan["scopes"]) != SCOPES:
        errors.append("plan scopes must declare A and B")
    tasks = plan.get("tasks")
    if not isinstance(tasks, list) or any(not isinstance(task, dict) for task in tasks):
        return errors + ["tasks must be a list of card objects"]
    ids = [task.get("id") for task in tasks]
    if any(not isinstance(card, str) for card in ids):
        return errors + ["every card must have a string ID"]
    if len(ids) != len(set(ids)):
        errors.append("duplicate card IDs")
    if set(ids) != EXPECTED_IDS or len(ids) != 64:
        errors.append("plan must contain exactly the original 64 card IDs")
    by_id = {task["id"]: task for task in tasks}
    for task in tasks:
        card = task["id"]
        if card not in EXPECTED_IDS:
            continue
        source, url = task.get("source"), task.get("source_url")
        expected_url = f"https://github.com/NatLabRockies/EnergyPlus/blob/{lock.get('source_commit')}/src/EnergyPlus/{source}"
        if not isinstance(source, str) or not source or url != expected_url:
            errors.append(f"{card}: source_url must use the canonical EP commit and source filename")
        try:
            read((plan_dir / "cards" / f"{card}.md").resolve())
        except OSError:
            errors.append(f"{card}: missing readable card file")
        scopes = task_scopes(task)
        if task.get("applicability") not in {"A", "B", "A/B"}:
            errors.append(f"{card}: applicability must be A, B, or A/B")
        for gate in GATES:
            value = task.get(gate)
            if not isinstance(value, str) or value not in STATUSES:
                errors.append(f"{card}: invalid {gate} status")
        scoped = task.get("scope_gates", {})
        if not isinstance(scoped, dict) or not set(scoped) <= scopes:
            errors.append(f"{card}: scope_gates keys must be applicable scopes")
        else:
            for scope, values in scoped.items():
                if (not isinstance(values, dict) or set(values) != set(GATES) or
                        any(not isinstance(v, str) or v not in STATUSES for v in values.values())):
                    errors.append(f"{card}/{scope}: scope_gates requires four valid gate statuses")
        regular = task.get("dependencies")
        conditional = task.get("conditional_dependencies", {})
        if not isinstance(regular, list):
            errors.append(f"{card}: dependencies must be a list")
            regular = []
        if not isinstance(conditional, dict) or not set(conditional) <= scopes:
            errors.append(f"{card}: conditional dependency keys must be applicable scopes")
            conditional = {}
        for label, values in [("dependencies", regular), *conditional.items()]:
            if (not isinstance(values, list) or
                    any(not isinstance(value, str) or value not in by_id for value in values)):
                errors.append(f"{card}: invalid {label} dependency IDs")
        for scope in scopes & SCOPES:
            deps = dependencies(task, scope)
            if any(not isinstance(dep, str) for dep in deps):
                continue
            if len(set(deps)) != len(deps):
                errors.append(f"{card}/{scope}: duplicate dependencies")
            for dep in deps:
                if dep in by_id and scope not in task_scopes(by_id[dep]):
                    errors.append(f"{card}/{scope}: dependency {dep} does not apply to this scope")
            if complete(task, scope):
                pending = [dep for dep in deps if dep in by_id and not complete(by_id[dep], scope)]
                if pending:
                    errors.append(f"{card}/{scope}: completed before predecessors: {', '.join(pending)}")
                scoped_evidence = task.get("evidence_by_scope", {})
                evidence = scoped_evidence.get(scope, task.get("evidence")) if isinstance(
                    scoped_evidence, dict) else None
                source_dir = lock.get("source_dir")
                if not isinstance(source_dir, str) or not source_dir:
                    errors.append(f"{card}/{scope}: canonical oracle source_dir is missing")
                else:
                    errors.extend(check_evidence(evidence, card, scope, root, root / source_dir, read))
    for scope in sorted(SCOPES):
        visiting: set[str] = set()
        visited: set[str] = set()

        def visit(card: str) -> None:
            if card in visiting:
                errors.append(f"{scope}: dependency cycle includes {card}")
                return
            if card in visited:
                return
            visiting.add(card)
            for dep in dependencies(by_id[card], scope):
                if isinstance(dep, str) and dep in by_id and scope in task_scopes(by_id[dep]):
                    visit(dep)
            visiting.remove(card)
            visited.add(card)

        for card, task in by_id.items():
            if scope in task_scopes(task):
                visit(card)
    return errors


def self_test(plan: dict[str, Any], lock: dict[str, Any], root: Path, plan_dir: Path) -> int:
    """Exercise metadata acceptance/rejection entirely in memory, with no writes."""
    base = copy.deepcopy(plan)
    for task in base["tasks"]:
        task.update({gate: "미확인" for gate in GATES})
        for key in ("scope_gates", "evidence", "evidence_by_scope"):
            task.pop(key, None)
    artifact = {"path": "test-evidence.json", "sha256": hashlib.sha256(b"fixture").hexdigest()}
    committed = subprocess.run(["git", "rev-parse", "--verify", "HEAD"], cwd=root,
                               capture_output=True, text=True, timeout=10, check=True).stdout.strip()
    evidence = {
        "scopes": ["A", "B"], "implementation_commit": committed,
        "source_ranges": [{"path": "src/EnergyPlus/HeatBalanceManager.cc", "symbol": "GetProjectControlData",
                           "start_line": 1, "end_line": 2, "helpers": []}],
        "input_hashes": [artifact], "reference": {"kind": "ep-trace", "artifacts": [artifact]},
        "checks": {gate: {"command": "python replay.py", "commit": committed,
                          "exit_code": 0, "artifacts": [artifact]} for gate in GATES},
        "production": {"rust_entrypoint": "ep_run::run", "oracle_inputs_used": False, "fixture_inputs_used": False},
        "comparison": {"contract": artifact, "report": artifact},
    }

    def read(path: Path) -> bytes:
        if path == (root / artifact["path"]).resolve():
            return b"fixture"
        if path == (root / lock["source_dir"] / "src/EnergyPlus/HeatBalanceManager.cc").resolve():
            return b"void GetProjectControlData() {\n}\n"
        return path.read_bytes()

    def check(candidate: dict[str, Any]) -> list[str]:
        return validate_plan(candidate, lock, root, plan_dir, read)

    def task(candidate: dict[str, Any], card: str) -> dict[str, Any]:
        return next(item for item in candidate["tasks"] if item["id"] == card)

    def finish(candidate: dict[str, Any], card: str, scope: str) -> None:
        item = task(candidate, card)
        item.setdefault("scope_gates", {})[scope] = {gate: "통과" for gate in GATES}
        item.setdefault("evidence_by_scope", {})[scope] = copy.deepcopy(evidence)

    cases: list[tuple[str, dict[str, Any], str | None]] = [("pending import", base, None)]
    valid = copy.deepcopy(base)
    finish(valid, "CON-01", "A")
    cases.append(("replayable completed root card", valid, None))
    for label, card, needle in (("false completion", "CON-01", "require replayable"),
                                ("predecessor bypass", "GEO-01", "before predecessors")):
        item = copy.deepcopy(base)
        task(item, card).update({gate: "통과" for gate in GATES})
        cases.append((label, item, needle))
    item = copy.deepcopy(base)
    task(item, "CON-01")["dependencies"] = ["GEO-01"]
    cases.append(("dependency cycle", item, "dependency cycle"))
    for label, mutate, needle in (
        ("unknown status", lambda p: task(p, "CON-01").update(unit_gate=True), "invalid unit_gate"),
        ("EP lock drift", lambda p: p.update(ep_commit="0" * 40), "canonical"),
        ("duplicate card", lambda p: p["tasks"].append(copy.deepcopy(p["tasks"][0])), "duplicate card IDs"),
        ("wrong hash", lambda p: task(p, "CON-01")["evidence_by_scope"]["A"]["input_hashes"][0].update(sha256="0" * 64), "SHA256 mismatch"),
        ("fixture injection", lambda p: task(p, "CON-01")["evidence_by_scope"]["A"]["production"].update(fixture_inputs_used=True), "fixture_inputs_used"),
        ("failed replay", lambda p: task(p, "CON-01")["evidence_by_scope"]["A"]["checks"]["unit_gate"].update(exit_code=1), "successful execution"),
        ("escaped evidence", lambda p: task(p, "CON-01")["evidence_by_scope"]["A"]["comparison"]["report"].update(path="../outside.json"), "escapes repository"),
        ("unknown executed revision", lambda p: task(p, "CON-01")["evidence_by_scope"]["A"]["checks"]["unit_gate"].update(commit="0" * 40), "existing Git commit/tree"),
        ("invalid source range", lambda p: task(p, "CON-01")["evidence_by_scope"]["A"]["source_ranges"][0].update(end_line=999999), "named symbol"),
    ):
        item = copy.deepcopy(valid)
        mutate(item)
        cases.append((label, item, needle))
    applicable = copy.deepcopy(base)
    for card in EXPECTED_IDS:
        if "A" in task_scopes(task(applicable, card)):
            finish(applicable, card, "A")
    cases.append(("A-only conditional dependencies excluded", copy.deepcopy(applicable), None))
    finish(applicable, "SYS-01", "B")
    cases.append(("B conditional predecessor required", copy.deepcopy(applicable), "HVAC-08"))
    failures = 0
    for label, candidate, needle in cases:
        errors = check(candidate)
        passed = not errors if needle is None else any(needle in error for error in errors)
        print(f"{'PASS' if passed else 'FAIL'} self-test: {label}")
        if not passed:
            failures += 1
            print("  " + "; ".join(errors))
    print(f"Self-test: {len(cases) - failures}/{len(cases)} passed; no files written.")
    return 1 if failures else 0


def main() -> int:
    for stream in (sys.stdout, sys.stderr):
        if hasattr(stream, "reconfigure"):
            stream.reconfigure(encoding="utf-8")
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--repo-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--plan", type=Path, help="repository-relative plan JSON (default: energyplus_porting_plan/plan.json)")
    parser.add_argument("--check", action="store_true", help="validate the whole plan (also the default)")
    parser.add_argument("--self-test", action="store_true", help="run in-memory negative fixtures without writes")
    parser.add_argument("--card", action="append", help="show selected card readiness; repeatable")
    parser.add_argument("--scope", choices=("A", "B"), help="restrict readiness display; validation still checks both scopes")
    args = parser.parse_args()
    root = args.repo_root.resolve()
    path = (root / (args.plan or Path("energyplus_porting_plan/plan.json"))).resolve()
    if not path.is_relative_to(root):
        parser.error("--plan must remain inside --repo-root")
    try:
        plan = json.loads(path.read_text(encoding="utf-8-sig"))
        lock = tomllib.loads((root / "config/default.toml").read_text(encoding="utf-8-sig"))["oracle"]
        errors = validate_plan(plan, lock, root, path.parent)
    except (OSError, ValueError, KeyError) as error:
        print(f"FAIL: cannot read porting plan or canonical lock: {error}", file=sys.stderr)
        return 1
    if errors:
        for error in errors:
            print(f"FAIL: {error}", file=sys.stderr)
        return 1
    if args.self_test:
        return self_test(plan, lock, root, path.parent)
    by_id = {task["id"]: task for task in plan["tasks"]}
    if args.card and any(card not in by_id for card in args.card):
        parser.error("--card must name an existing card")
    print(f"PASS: 64 unique cards; EP {plan['ep_version']} / {plan['ep_commit']}; gates and dependencies valid.")
    for scope in [args.scope] if args.scope else sorted(SCOPES):
        active = [task for task in plan["tasks"] if scope in task_scopes(task)]
        finished = sum(complete(task, scope) for task in active)
        ready = [task["id"] for task in active if not complete(task, scope) and
                 all(complete(by_id[dep], scope) for dep in dependencies(task, scope))]
        print(f"Scope {scope}: {finished}/{len(active)} completed; ready: {', '.join(ready) or 'none'}")
        for card in args.card or []:
            task = by_id[card]
            if scope not in task_scopes(task):
                print(f"  {card}: outside scope {scope}")
                continue
            pending = [dep for dep in dependencies(task, scope) if not complete(by_id[dep], scope)]
            state = "complete" if complete(task, scope) else "blocked" if pending else "ready"
            values = ", ".join(f"{gate}={value}" for gate, value in gate_values(task, scope).items())
            print(f"  {card}: {state}; {values}" + (f"; waiting: {', '.join(pending)}" if pending else ""))
    print("Completion evidence metadata checked; numerical correctness requires the recorded scientific review and replay.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
