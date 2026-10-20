//! Ancillary classification of raw inputs on the admitted ordinary Source lane.
//! This token never bypasses an input getter or provides Native runtime state.
use crate::porting_scope::{PortingScope, PortingScopeTrace};
use ep_raw_model::{ObjectType, RawModel};
use std::collections::BTreeSet;

pub(crate) struct SourceSupportAdmission {
    covered_objects: BTreeSet<&'static str>,
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
            PortingScope::A => crate::ordinary_equipment_input::resolve(raw),
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
        let covered_objects = [
            "HeatBalanceAlgorithm",
            "SimulationControl",
            // This monthly weather input is consumed by the actual Source ground
            // model; admitted thermal boundary branches remain scope-validated.
            "Site:GroundTemperature:BuildingSurface",
            "Output:Variable",
        ]
        .into_iter()
        .filter(|family| {
            raw.objects
                .get(&ObjectType((*family).into()))
                .is_some_and(|objects| !objects.is_empty())
        })
        .collect();
        Some(Self { covered_objects })
    }

    pub(super) fn consumes(&self, family: &str) -> bool {
        self.covered_objects.contains(family)
    }
}
