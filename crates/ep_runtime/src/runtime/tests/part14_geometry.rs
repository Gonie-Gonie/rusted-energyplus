#[test]
fn stored_geometry_height_consumers_ignore_later_typed_vertex_mutation() {
    use crate::geometry::production_trace::{
        self as geometry_trace, GeometryConsumer, GeometryOperandBits,
    };
    use crate::heat_balance::convection::{
        energyplus_stored_surface_outdoor_air_temperature_c,
        energyplus_stored_surface_outside_wind_speed_m_per_s,
    };
    use crate::psychrometrics::production_trace as execution;
    let mut model = SimulationModel::from_typed(cube_model());
    let state = initialize_heat_balance_state(&model, 20.0).expect("valid geometry initializes");
    let surface = &state.surfaces[0];
    let expected_wind = energyplus_stored_surface_outside_wind_speed_m_per_s(
        surface,
        &model.typed.surfaces[0],
        Terrain::Suburbs,
        3.0,
    );
    let expected_temperature = energyplus_stored_surface_outdoor_air_temperature_c(surface, 30.0);
    // A deliberately unrelated later input geometry must not become the
    // physical consumer's height. The one initialized owner remains unchanged.
    for vertex in &mut model.typed.surfaces[0].vertices {
        vertex.z_m += 1000.0;
    }
    let ((actual, trace), _) = execution::capture(true, || {
        geometry_trace::capture(true, || {
            let _zone = execution::zone_step(0, 1, 4, 900.0);
            (
                energyplus_stored_surface_outside_wind_speed_m_per_s(
                    surface,
                    &model.typed.surfaces[0],
                    Terrain::Suburbs,
                    3.0,
                ),
                energyplus_stored_surface_outdoor_air_temperature_c(surface, 30.0),
            )
        })
    });
    assert_eq!(actual, (expected_wind, expected_temperature));
    let trace = trace.expect("enabled geometry trace");
    assert_eq!(trace.total_call_count, 2);
    assert_eq!(trace.omitted_call_count, 0);
    assert_eq!(
        trace.dictionary[0].consumer,
        GeometryConsumer::OutsideWindSpeed
    );
    assert_eq!(
        trace.dictionary[1].consumer,
        GeometryConsumer::OutdoorAirTemperature
    );
    let centroid = surface.geometry.centroid_m;
    let bits = GeometryOperandBits::CentroidHeight(
        [centroid.x_m, centroid.y_m, centroid.z_m, centroid.z_m].map(f64::to_bits),
    );
    assert!(
        trace
            .dictionary
            .iter()
            .all(|call| call.operand_bits == bits && call.surface_id == surface.surface_id)
    );
}

#[test]
fn convection_sum_consumes_the_recorded_area_and_invalid_geometry_stops_initialization() {
    use crate::geometry::production_trace::{
        self as geometry_trace, GeometryConsumer, GeometryOperandBits,
    };
    let mut model = SimulationModel::from_typed(cube_model());
    let mut state =
        initialize_heat_balance_state(&model, 20.0).expect("valid geometry initializes");
    state.surfaces[0].area_m2 = 123.125;
    state.surfaces[0].inside_convection_coefficient_w_per_m2_k = 2.0;
    let (sum, trace) = geometry_trace::capture(true, || {
        crate::heat_balance::inside_convection::zone_surface_convection_sums_for_indices(
            &state.surfaces,
            &[0],
        )
    });
    assert_eq!(sum.0, 246.25);
    let trace = trace.expect("enabled area observation");
    assert_eq!(trace.total_call_count, 1);
    assert_eq!(
        trace.dictionary[0].consumer,
        GeometryConsumer::SurfaceHeatTransfer
    );
    assert_eq!(
        trace.dictionary[0].operand_bits,
        GeometryOperandBits::Area(123.125f64.to_bits())
    );
    // The other cube faces still give a valid zone volume; admission fails on
    // the actual degenerate surface before returning any state to a timestep.
    let name = model.typed.surfaces[0].name.0.clone();
    model.typed.surfaces[0].vertices = vec![point(0., 0., 0.); 4];
    assert!(matches!(initialize_heat_balance_state(&model,20.0),
        Err(RuntimeError::InvalidSurfaceGeometry{surface_name,..}) if surface_name==name));
}

#[test]
fn geometry_report_recomputation_is_unobserved_while_physical_coefficient_calls_remain_live() {
    use crate::geometry::production_trace::{self as geometry_trace, GeometryConsumer};
    use crate::heat_balance::convection::energyplus_dry_exterior_convection_coefficient_w_per_m2_k;
    use crate::psychrometrics::production_trace as execution;
    let model = SimulationModel::from_typed(cube_model());
    let state = initialize_heat_balance_state(&model, 20.0).expect("valid geometry initializes");
    let surface = state
        .surfaces
        .iter()
        .find(|s| s.surface_name == "ROOF")
        .expect("roof");
    let typed = model
        .typed
        .surfaces
        .iter()
        .find(|s| s.id == surface.surface_id)
        .expect("typed roof");
    let mut record = weather_record_with_precipitation(0.0);
    record.wind_speed_m_per_s = 3.0;
    let records = [record];
    let outdoor = energyplus_surface_outdoor_air_temperature_c(typed, 8.0);
    let mut config = HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical.runtime_config();
    config.use_doe2_outside_convection = true;
    config.use_cached_exterior_report_terms = false;
    let physical = || {
        energyplus_dry_exterior_convection_coefficient_w_per_m2_k(
            surface,
            typed,
            20.0,
            outdoor,
            surface.tilt_deg.to_radians(),
            Terrain::Suburbs,
            3.0,
            0.0,
            true,
        )
    };
    let ((values, trace), _) = execution::capture(true, || {
        geometry_trace::capture(true, || {
            let _zone = execution::zone_step(0, 1, 4, 900.0);
            let before = physical();
            let report = surface_exterior_report_terms(
                &model.typed,
                surface,
                8.0,
                20.0,
                Some(HeatBalanceWeatherContext {
                    records: &records,
                    sample: None,
                    owned: None,
                    record_index: 0,
                    zone_steps_per_hour: 4,
                    zone_timestep: Some(1),
                    first_hour_interpolation_starting_values:
                        FirstHourInterpolationStartingValues::Hour24,
                }),
                config,
            );
            (before, report.convection_coefficient_w_per_m2_k, physical())
        })
    });
    assert_eq!(values.0, values.1);
    assert_eq!(values.0, values.2);
    let trace = trace.expect("enabled capture");
    // Two real coefficient calls each hand off wind-height and orientation;
    // the shared report recomputation emits no geometry observations.
    assert_eq!(trace.total_call_count, 4);
    assert_eq!(
        trace.consumer_counts[&GeometryConsumer::OutsideWindSpeed],
        2
    );
    assert_eq!(
        trace.consumer_counts[&GeometryConsumer::OutsideConvection],
        2
    );
    assert_eq!(trace.omitted_call_count, 0);
}
