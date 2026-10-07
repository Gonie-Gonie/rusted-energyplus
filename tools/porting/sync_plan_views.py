"""Keep checklist rows and the offline HTML packet aligned with validated plan.json.

This command never changes gate values or card evidence. Use --check to detect
stale mirrors or --write after manually reviewing and updating a card's plan.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
import tomllib
from pathlib import Path

from check_plan import complete, task_scopes, validate_plan


PLAN_SCRIPT = re.compile(r'(<script id="plan" type="application/json">)(.*?)(</script>)', re.S)
CHECKLIST_ROW = re.compile(r"^(\| )[^|]*( \| \[([A-Z]+-\d{2})\]\(cards/\3\.md\) \|.*)$", re.M)


def render_checklist(text: str, plan: dict) -> str:
    tasks = {task["id"]: task for task in plan["tasks"]}
    found: list[str] = []

    def replace(match: re.Match[str]) -> str:
        card = match[3]
        if card not in tasks:
            raise ValueError(f"unknown checklist card: {card}")
        found.append(card)
        task = tasks[card]
        scopes = sorted(task_scopes(task))
        finished = {scope: complete(task, scope) for scope in scopes}
        if all(finished.values()):
            status = "[x]"
        elif any(finished.values()):
            status = " / ".join(f"{scope} [{'x' if done else ' '}]" for scope, done in finished.items())
        else:
            status = "[ ]"
        return match[1] + status + match[2]

    result = CHECKLIST_ROW.sub(replace, text)
    if len(found) != len(tasks) or set(found) != set(tasks):
        raise ValueError("checklist must contain one row for every card")
    return result


def render_html(text: str, plan: dict) -> str:
    if len(PLAN_SCRIPT.findall(text)) != 1:
        raise ValueError("HTML must contain exactly one embedded plan")
    # Escape characters that could terminate a script element in reviewed prose.
    payload = json.dumps(plan, ensure_ascii=False).replace("<", "\\u003c").replace("&", "\\u0026")
    return PLAN_SCRIPT.sub(lambda match: match[1] + payload + match[3], text)


def main() -> int:
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8")
    parser = argparse.ArgumentParser(description=__doc__)
    action = parser.add_mutually_exclusive_group(required=True)
    action.add_argument("--check", action="store_true")
    action.add_argument("--write", action="store_true")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    directory = root / "energyplus_porting_plan"
    try:
        plan = json.loads((directory / "plan.json").read_text(encoding="utf-8-sig"))
        lock = tomllib.loads((root / "config/default.toml").read_text(encoding="utf-8-sig"))["oracle"]
        errors = validate_plan(plan, lock, root, directory)
        if errors:
            raise ValueError("; ".join(errors))
        changes: list[tuple[Path, str]] = []
        checklist = directory / "CHECKLIST.md"
        original = checklist.read_text(encoding="utf-8")
        rendered = render_checklist(original, plan)
        if original != rendered:
            changes.append((checklist, rendered))
        html = directory / "energyplus_porting_checklist.html"
        original = html.read_text(encoding="utf-8")
        embedded = PLAN_SCRIPT.search(original)
        if embedded is None or json.loads(embedded[2]) != plan:
            changes.append((html, render_html(original, plan)))
        if args.check and changes:
            for path, _ in changes:
                print(f"FAIL: stale plan mirror: {path.relative_to(root)}")
            return 1
        for path, rendered in changes:
            path.write_text(rendered, encoding="utf-8", newline="\n")
            print(f"Updated {path.relative_to(root)}")
    except (OSError, ValueError, KeyError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        return 1
    print("PASS: checklist and embedded HTML plan match validated gate metadata.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
