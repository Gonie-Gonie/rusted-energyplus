//! Ancillary classification of raw inputs on the admitted ordinary Source lane.
//! This token never bypasses an input getter or provides Native runtime state.
use crate::porting_scope::{PortingScope, PortingScopeTrace};
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
use std::collections::BTreeSet;

pub(crate) struct SourceSupportAdmission {
    covered_objects: BTreeSet<&'static str>,
    /// Exact local alpha fields consumed by genuine GetZoneData and the live
    /// owning-Zone dispatchers. This is not a general Zone-feature exemption.
    zone_convection_overrides_consumed: bool,
}

impl SourceSupportAdmission {
    /// Only the fully validated finite scope/environment and existing raw
    /// admission may classify these actual Source-consumed input semantics.
    // Retain the existing nested input/trace guards and their error or diagnostic order.
    #[allow(clippy::collapsible_if)]
    pub(crate) fn from_validated_scope(raw: &RawModel, trace: &PortingScopeTrace) -> Option<Self> {
        if !trace.admissible || !trace.violations.is_empty() || trace.environment.is_none() {
            return None;
        }
        match trace.scope {
            PortingScope::A => crate::ordinary_equipment_input::resolve_source_a(raw),
            PortingScope::B => crate::ordinary_equipment_input::resolve_source_b(raw),
        }
        .ok()?;
        // The existing raw admission checks every field/frequency/schedule;
        // the same reporting pre-scan must retain every actual request object.
        let reporting = crate::ordinary_input_processor::resolve_reporting_prescan(raw).ok()?;
        let requested = raw.objects.get(&ObjectType("Output:Variable".into()));
        if reporting.variables.len() != requested.map_or(0, |objects| objects.len()) {
            return None;
        }
        if let Some(objects) = requested {
            if !objects
                .keys()
                .zip(&reporting.variables)
                .all(|(name, request)| name.0 == request.object_name)
            {
                return None;
            }
        }
        // The same real IP/raw projection retains every normal meter request
        // (including unnamed blank keys) for UpdateMeterReporting. This proves
        // custody, not arbitrary meter-resource/SQLite or cumulative support.
        if ["Output:Meter", "Output:Meter:MeterFileOnly"]
            .iter()
            .any(|family| {
                raw.objects
                    .get(&ObjectType((*family).into()))
                    .is_some_and(|rows| !rows.is_empty())
            })
        {
            let input = match trace.scope {
                PortingScope::A => crate::ordinary_input_processor::resolve_source_a(raw),
                PortingScope::B => crate::ordinary_input_processor::resolve_source_b(raw),
            }
            .ok()?;
            let projected = crate::ordinary_heat_balance_input::resolve(raw, &input).ok()?;
            for family in ["Output:Meter", "Output:Meter:MeterFileOnly"] {
                let raw_names = raw
                    .objects
                    .get(&ObjectType(family.into()))
                    .map(|rows| {
                        rows.keys()
                            .map(|key| key.0.as_str())
                            .collect::<BTreeSet<_>>()
                    })
                    .unwrap_or_default();
                let names = projected
                    .objects
                    .get(family)
                    .map(|rows| {
                        rows.iter()
                            .map(|row| row.object_name.as_str())
                            .collect::<BTreeSet<_>>()
                    })
                    .unwrap_or_default();
                if raw_names != names {
                    return None;
                }
            }
        }
        let covered_objects = [
            "HeatBalanceAlgorithm",
            "SimulationControl",
            "SurfaceConvectionAlgorithm:Inside",
            "SurfaceConvectionAlgorithm:Outside",
            // This monthly weather input is consumed by the actual Source ground
            // model; admitted thermal boundary branches remain scope-validated.
            "Site:GroundTemperature:BuildingSurface",
            "Output:Variable",
            "Output:Meter",
            "Output:Meter:MeterFileOnly",
            // Actual Source schedule reporting consumes real key_field and EIO
            // owners; this token never substitutes a TypedModel schedule.
            "Output:Schedules",
            // Actual comfort initialization consumes all declared lists through
            // its shared IP getter; this remains compiler RawOnly.
            "ComfortViewFactorAngles",
            // Actual local getters populate the same retained Day/Week/Detailed
            // graph. Typed legacy schedules alone do not grant this token.
            "Schedule:Day:Hourly",
            "Schedule:Day:Interval",
            "Schedule:Day:List",
            "Schedule:Week:Compact",
            "Schedule:Week:Daily",
            "Schedule:Year",
        ]
        .into_iter()
        .filter(|family| {
            raw.objects
                .get(&ObjectType((*family).into()))
                .is_some_and(|objects| !objects.is_empty())
        })
        .collect();
        // Scope and the same raw resolver already check every effective enum
        // and every supplied scalar. Keep authored Override provenance in the
        // typed model; only a nonempty, actually present local alpha is covered.
        let zone_convection_overrides_consumed = raw
            .objects
            .get(&ObjectType("Zone".into()))
            .is_some_and(|objects| {
                objects.values().any(|object| {
                    [
                        "zone_inside_convection_algorithm",
                        "zone_outside_convection_algorithm",
                    ]
                    .iter()
                    .any(|field| {
                        matches!(object.fields.get(&FieldName((*field).into())),
                        Some(RawValue::String(value)) if !value.is_empty())
                    })
                })
            });
        Some(Self {
            covered_objects,
            zone_convection_overrides_consumed,
        })
    }

    pub(super) fn consumes_zone_convection_overrides(&self) -> bool {
        self.zone_convection_overrides_consumed
    }

    pub(super) fn consumes(&self, family: &str) -> bool {
        self.covered_objects.contains(family)
    }
}
