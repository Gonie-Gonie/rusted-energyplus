use super::*;
use crate::heat_balance::state::HeatBalanceState;
use ep_model::SurfaceType;
use crate::heat_balance::initialization::initialize_heat_balance_state;
use ep_model::{
    AutoOrNumber, Construction, ConstructionId, ConstructionKind, Material, MaterialDefinition,
    MaterialId, MaterialSurfaceRoughness, NormalizedName, OpaqueSurfaceProperties,
    OutsideBoundaryCondition, Point3, RegularMaterial, SimulationModel, Space, SpaceId,
    SpaceOrigin, SpaceTypeId, SunExposure, Surface, WindExposure, Zone, ZoneId,
};

type TestResult<T = ()> = Result<T, String>;

fn model() -> SimulationModel {
    // Authored input primitives only. Public coefficients and retry workspaces
    // must come from the actual ordinary initializer, never a Native endpoint.
    let mut model = TypedModel::default();
    model.timestep.number_of_timesteps_per_hour = 3;
    model.materials.push(Material {
        id: MaterialId(5),
        name: NormalizedName::new("M5"),
        definition: MaterialDefinition::Regular(RegularMaterial {
            roughness: MaterialSurfaceRoughness::MediumRough,
            thickness_m: 0.2,
            conductivity_w_per_m_k: 1.0,
            density_kg_per_m3: 1000.0,
            specific_heat_j_per_kg_k: 1000.0,
            thermal_resistance_m2_k_per_w: 0.2,
            surface: OpaqueSurfaceProperties::default(),
        }),
    });
    model.constructions.push(Construction {
        id: ConstructionId(30),
        name: NormalizedName::new("C30"),
        kind: ConstructionKind::Opaque,
        outside_layer: Some(MaterialId(5)),
        layers: vec![MaterialId(5)],
        thermochromic_master: None,
        ground_factor: None,
        air_boundary: None,
        complex_fenestration: None,
        window_equivalent_layer: None,
        internal_heat_source: None,
    });
    model.zones.push(Zone {
        id: ZoneId(9),
        name: NormalizedName::new("Z9"),
        direction_of_relative_north_deg: 0.0,
        origin: Point3 {
            x_m: 0.0,
            y_m: 0.0,
            z_m: 0.0,
        },
        zone_type: 1,
        multiplier: 1,
        list_multiplier: 1,
        list_group: None,
        ceiling_height: AutoOrNumber::Value(1.0),
        volume: AutoOrNumber::Value(1.0),
        floor_area: AutoOrNumber::Value(1.0),
        inside_convection_algorithm: ep_model::ZoneConvectionAlgorithm::Inherited(
            ep_model::InsideSurfaceConvectionAlgorithm::Tarp,
        ),
        outside_convection_algorithm: ep_model::ZoneConvectionAlgorithm::Inherited(
            ep_model::OutsideSurfaceConvectionAlgorithm::Doe2,
        ),
        is_part_of_total_floor_area: true,
        is_nominal_controlled: false,
        linked_outdoor_air_node: None,
        spaces: vec![SpaceId(3)],
    });
    model.spaces.push(Space {
        id: SpaceId(3),
        name: NormalizedName::new("S3"),
        zone: ZoneId(9),
        ceiling_height: AutoOrNumber::AutoCalculate,
        volume: AutoOrNumber::AutoCalculate,
        floor_area: AutoOrNumber::AutoCalculate,
        space_type: NormalizedName::new("GENERAL"),
        space_type_id: SpaceTypeId(0),
        tags: Vec::new(),
        origin: SpaceOrigin::Authored,
    });
    model.surfaces.push(Surface {
        id: SurfaceId(500),
        name: NormalizedName::new("W500"),
        surface_type: SurfaceType::Wall,
        construction: ConstructionId(30),
        zone: ZoneId(9),
        space: SpaceId(3),
        outside_boundary_condition: OutsideBoundaryCondition::Outdoors,
        outside_boundary_condition_object: None,
        sun_exposure: SunExposure::NoSun,
        wind_exposure: WindExposure::NoWind,
        view_factor_to_ground: AutoOrNumber::Value(0.0),
        vertices: vec![
            Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            Point3 {
                x_m: 1.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            Point3 {
                x_m: 1.0,
                y_m: 0.0,
                z_m: 1.0,
            },
            Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 1.0,
            },
        ],
        computed_geometry: None,
    });
    SimulationModel::from_typed(model)
}

fn state(model: &SimulationModel) -> TestResult<HeatBalanceState> {
    initialize_heat_balance_state(model, 20.0).map_err(|error| format!("initializer: {error:?}"))
}

fn clock(step: i32, begin: bool) -> ProductionCtfHistoryClock {
    ProductionCtfHistoryClock {
        phase: crate::weather::day::WeatherDayPhase::Warmup { day: 1 },
        record_index: 0,
        begin_simulation: begin,
        begin_environment: begin,
        day_of_sim: 1,
        hour_of_day: 1,
        time_step: step,
        sim_time_steps: step,
        time_step_zone: 1.0 / 3.0,
        time_step_zone_seconds: 1200.0,
    }
}
