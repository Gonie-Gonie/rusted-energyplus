use ep_model::TypedModel;
use ep_raw_model::ObjectType;

use super::{Compiler, DiagnosticSeverity};

const SURFACE_WITNESS_OBJECT_TYPES: [&str; 2] =
    ["Shading:Site:Detailed", "Shading:Building:Detailed"];

const VALID_NO_ZONE_OBJECT_TYPES: [&str; 8] = [
    "SolarCollector:FlatPlate:Water",
    "Generator:Photovoltaic",
    "Generator:InternalCombustionEngine",
    "Generator:CombustionTurbine",
    "Generator:FuelCell",
    "Generator:MicroCHP",
    "Generator:MicroTurbine",
    "Generator:WindTurbine",
];

impl Compiler<'_> {
    /// Rejects the bounded no-zone case proved by retained raw detached-shading input.
    pub(super) fn check_valid_simulation_objects_bounded(&mut self, model: &TypedModel) {
        if self
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
            || !model.zones.is_empty()
        {
            return;
        }

        let raw_type_is_present = |object_type: &str| {
            self.raw_model
                .objects
                .get(&ObjectType(object_type.to_string()))
                .is_some_and(|instances| !instances.is_empty())
        };

        if !SURFACE_WITNESS_OBJECT_TYPES
            .iter()
            .any(|object_type| raw_type_is_present(object_type))
        {
            return;
        }

        if VALID_NO_ZONE_OBJECT_TYPES
            .iter()
            .any(|object_type| raw_type_is_present(object_type))
        {
            return;
        }

        self.error(
            "InvalidSimulationWithoutZones",
            "GetHeatBalanceInput",
            None,
            None,
            "There are surfaces in input but no zones found. Invalid simulation.".to_string(),
        );
    }
}
