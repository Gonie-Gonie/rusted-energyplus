"""Full-domain claim rules for EnergyPlus routine completion records."""

from __future__ import annotations

from typing import Any


ROUTINE_COMPLETION_RANK = {
    "not_started": 0,
    "source_mapped": 1,
    "state_mapped": 2,
    "implemented": 3,
    "family_gated": 4,
    "complete": 5,
}
DOMAIN_CLAIM_KEYS = {
    "heat_balance": "broad_heat_balance_compatibility",
    "hvac": "hvac_compatibility",
    "plant": "plant_compatibility",
}
MINIMUM_REQUIRED_ROUTINES = {
    "heat_balance": {
        "manage_heat_balance",
        "manage_surface_heat_balance",
        "manage_air_heat_balance",
        "manage_zone_air_updates",
    },
    "hvac": {
        "manage_zone_equipment",
        "sim_zone_equipment",
        "sim_purchased_air",
        "get_purchased_air",
        "init_purchased_air",
        "calc_purch_air_loads",
        "update_purchased_air",
        "report_purchased_air",
    },
    "plant": {"manage_plant_loops"},
}


def require(condition: bool, errors: list[str], message: str) -> None:
    if not condition:
        errors.append(message)


def validate_domain_completion_contract(
    contract: dict[str, Any],
    routines: list[dict[str, Any]],
    errors: list[str],
) -> dict[str, dict[str, Any]]:
    claims = contract.get("claims", {})
    require(isinstance(claims, dict), errors, "project contract claims must be a table")
    if not isinstance(claims, dict):
        claims = {}
    entries = contract.get("domain_claim", [])
    require(isinstance(entries, list), errors, "project contract domain_claim must be an array of tables")
    if not isinstance(entries, list):
        entries = []

    by_domain: dict[str, dict[str, Any]] = {}
    for entry in entries:
        require(isinstance(entry, dict), errors, "domain_claim entries must be TOML tables")
        if not isinstance(entry, dict):
            continue
        domain = str(entry.get("id", "")).strip()
        require(bool(domain), errors, "domain_claim id must not be empty")
        require(domain not in by_domain, errors, f"duplicate domain_claim id: {domain}")
        if domain:
            by_domain[domain] = entry
    require(
        set(by_domain) == set(DOMAIN_CLAIM_KEYS),
        errors,
        "domain_claim ids must be exactly heat_balance, hvac, and plant",
    )

    routine_by_id = {str(routine.get("_id", "")): routine for routine in routines}
    for routine_id, routine in routine_by_id.items():
        status = str(routine.get("completion_status", "")).strip()
        require(
            status in ROUTINE_COMPLETION_RANK,
            errors,
            f"{routine_id}: unsupported routine completion_status {status!r}",
        )
        require(
            isinstance(routine.get("required_for_full_domain"), bool),
            errors,
            f"{routine_id}: required_for_full_domain must be boolean",
        )

    readiness: dict[str, dict[str, Any]] = {}
    for domain, expected_claim_key in DOMAIN_CLAIM_KEYS.items():
        entry = by_domain.get(domain, {})
        claim_key = str(entry.get("claim_key", "")).strip()
        require(
            claim_key == expected_claim_key,
            errors,
            f"{domain}: domain_claim claim_key must be {expected_claim_key}",
        )
        claim_value = claims.get(expected_claim_key)
        require(isinstance(claim_value, bool), errors, f"claims.{expected_claim_key} must be boolean")
        inventory_complete = entry.get("routine_inventory_complete")
        require(
            isinstance(inventory_complete, bool),
            errors,
            f"{domain}: routine_inventory_complete must be boolean",
        )
        raw_required = entry.get("required_routines", [])
        require(isinstance(raw_required, list), errors, f"{domain}: required_routines must be an array")
        if isinstance(raw_required, list):
            require(
                all(isinstance(value, str) for value in raw_required),
                errors,
                f"{domain}: required_routines must contain only string ids",
            )
        required = [str(value).strip() for value in raw_required] if isinstance(raw_required, list) else []
        required_set = set(required)
        require(bool(required), errors, f"{domain}: required_routines must not be empty")
        require(all(required), errors, f"{domain}: required_routines must not contain empty ids")
        require(
            len(required) == len(required_set),
            errors,
            f"{domain}: required_routines must not contain duplicates",
        )
        missing_seed = MINIMUM_REQUIRED_ROUTINES[domain] - required_set
        require(
            not missing_seed,
            errors,
            f"{domain}: required_routines must retain immutable minimum seed: {sorted(missing_seed)}",
        )
        flagged_required = {
            routine_id
            for routine_id, routine in routine_by_id.items()
            if str(routine.get("_domain", "")) == domain
            and routine.get("required_for_full_domain") is True
        }
        require(
            required_set == flagged_required,
            errors,
            f"{domain}: required_routines must exactly match required_for_full_domain routine records; "
            f"listed={sorted(required_set)}, flagged={sorted(flagged_required)}",
        )

        gated = 0
        below_gate: list[str] = []
        for routine_id in dict.fromkeys(required):
            routine = routine_by_id.get(routine_id)
            require(routine is not None, errors, f"{domain}: unknown required routine: {routine_id}")
            if routine is None:
                below_gate.append(f"{routine_id}=missing")
                continue
            routine_domain = str(routine.get("_domain", ""))
            require(
                routine_domain == domain,
                errors,
                f"{domain}: required routine belongs to {routine_domain}: {routine_id}",
            )
            require(
                routine.get("required_for_full_domain") is True,
                errors,
                f"{domain}: required routine is not flagged required_for_full_domain: {routine_id}",
            )
            status = str(routine.get("completion_status", ""))
            if ROUTINE_COMPLETION_RANK.get(status, -1) >= ROUTINE_COMPLETION_RANK["family_gated"]:
                gated += 1
            else:
                below_gate.append(f"{routine_id}={status or 'missing'}")

        blockers: list[str] = []
        if inventory_complete is not True:
            blockers.append("routine inventory incomplete")
        if below_gate:
            blockers.append("below family_gated: " + ", ".join(below_gate))
        ready = not blockers and bool(required)
        if claim_value is True:
            require(
                inventory_complete is True,
                errors,
                f"{domain}: full-domain claim requires routine_inventory_complete=true",
            )
            require(
                not below_gate and bool(required),
                errors,
                f"{domain}: full-domain claim requires every required routine at family_gated or complete",
            )
        readiness[domain] = {
            "claim_key": expected_claim_key,
            "claimed": claim_value is True,
            "inventory_complete": inventory_complete is True,
            "required": len(required),
            "family_gated": gated,
            "ready": ready,
            "blockers": blockers,
        }

    require(
        claims.get("full_runtime_compatibility") is False,
        errors,
        "full runtime compatibility remains locked until every EnergyPlus domain has a complete inventory",
    )
    return readiness
