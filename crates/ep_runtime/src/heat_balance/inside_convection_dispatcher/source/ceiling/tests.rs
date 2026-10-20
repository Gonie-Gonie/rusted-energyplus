use super::*;
use crate::heat_balance::error_backend::SourceErrorState;
use crate::heat_balance::inside_convection_dispatcher::InsideConvectionSurfaceState;
use crate::heat_balance::ordinary_equipment::OrdinaryEquipmentInput;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::heat_balance::source_initialization::SourceRecurringErrorState;
use crate::psychrometrics::{SourcePsychrometricAccess, with_fresh_psychrometric_state};
use crate::weather::day::WeatherEnvironmentState;
use ep_model::{SpaceId, ZoneId};
use std::collections::BTreeMap;
type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Caller {
    globals: SelectedHeatBalanceGlobals,
    clock: WeatherGlobalState,
    environment: WeatherEnvironmentState,
    hvac: SelectedHvacGlobals,
    psychrometrics: SourcePsychrometricAccess,
    recurring: SourceRecurringErrorState,
    errors: SourceErrorState,
    stream: Vec<u8>,
    console: Vec<u8>,
    developer: bool,
}
impl Caller {
    fn new() -> Self {
        let mut environment = WeatherEnvironmentState::default();
        environment.current_weather.out_baro_press = 101325.0;
        let globals = SelectedHeatBalanceGlobals {
            num_of_zones: 1,
            num_spaces: 1,
            ..SelectedHeatBalanceGlobals::default()
        };
        Self {
            globals,
            clock: WeatherGlobalState::default(),
            environment,
            hvac: SelectedHvacGlobals::default(),
            psychrometrics: SourcePsychrometricAccess,
            recurring: SourceRecurringErrorState::default(),
            errors: super::super::super::tests::error_state(),
            stream: Vec::new(),
            console: Vec::new(),
            developer: false,
        }
    }
    fn call<R>(
        &mut self,
        execute: impl FnOnce(
            &SelectedHeatBalanceGlobals,
            &WeatherGlobalState,
            &mut SourceInsideConvectionPropertyOwners<'_>,
            &mut SourceErrorBackend<'_>,
        ) -> R,
    ) -> R {
        let mut backend = SourceErrorBackend {
            state: &mut self.errors,
            error_stream: Some(&mut self.stream),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        let mut properties = SourceInsideConvectionPropertyOwners {
            environment: &self.environment,
            hvac: &self.hvac,
            psychrometrics: &mut self.psychrometrics,
            recurring: &mut self.recurring,
            developer: &self.developer,
        };
        execute(&self.globals, &self.clock, &mut properties, &mut backend)
    }
}
fn equipment() -> Result<OrdinaryEquipment> {
    let input = OrdinaryEquipmentInput::from_family_inventory([], 4)?;
    let mut owner = OrdinaryEquipment::new(input, &[ZoneId(7)], &[SpaceId(3)])?;
    owner.get_zone_equipment(4)?;
    Ok(owner)
}
fn zone() -> SelectedZoneData {
    SelectedZoneData {
        typed_zone_id: Some(ZoneId(7)),
        name: "DECLARED ZONE".into(),
        inside_algorithm: Inside::CeilingDiffuser,
        system_zone_node_number: 1,
        volume: 90.0,
        ceiling_height: 3.0,
        floor_area: 30.0,
        bounds_m: [0.0, 6.0, 0.0, 5.0, 0.0, 3.0],
        space_indexes: vec![1],
        exterior_gross_wall_area: 18.0,
        ..SelectedZoneData::default()
    }
}
fn geometry() -> Result<(
    InsideConvectionLayout,
    SelectedZoneInputOwners,
    SurfaceGeometryInputState,
    Vec<InsideConvectionSurface>,
)> {
    let layout = InsideConvectionLayout::new(
        vec![ZoneId(7)],
        vec![(SpaceId(3), ZoneId(7))],
        vec![
            InsideConvectionAssociation {
                surface_id: SurfaceId(8),
                zone_id: ZoneId(7),
                space_id: SpaceId(3),
            },
            InsideConvectionAssociation {
                surface_id: SurfaceId(2),
                zone_id: ZoneId(7),
                space_id: SpaceId(3),
            },
        ],
        vec![InsideConvectionGroup {
            zone_id: ZoneId(7),
            space_id: SpaceId(3),
            surfaces: vec![SurfaceId(8), SurfaceId(2)],
        }],
    )
    .map_err(|error| format!("layout: {error:?}"))?;
    let mut zones = SelectedZoneInputOwners {
        zones: Some(vec![zone()]),
        spaces: Some(vec![Default::default()]),
        ..SelectedZoneInputOwners::default()
    };
    let space = &mut zones.spaces.as_mut().ok_or("spaces")?[0];
    space.typed_space_id = Some(SpaceId(3));
    space.zone_num = 1;
    space.ht_surface_first = 1;
    space.ht_surface_last = 2;
    let mut geometry = SurfaceGeometryInputState {
        final_surfaces: Some(vec![Default::default(); 2]),
        ..SurfaceGeometryInputState::default()
    };
    let mut surfaces = Vec::new();
    for (i, (surface_id, cos_tilt)) in [(SurfaceId(8), -1.0), (SurfaceId(2), 1.0)]
        .into_iter()
        .enumerate()
    {
        let row = &mut geometry.final_surfaces.as_mut().ok_or("surface rows")?[i];
        row.typed_id = Some(surface_id);
        row.row.vertex_progress.height = 3.0;
        row.row.exterior_boundary = 0;
        surfaces.push(InsideConvectionSurface {
            surface_id,
            class: ep_model::SurfaceType::Wall,
            exterior: true,
            heat_transfer: true,
            construction_is_window: false,
            kiva: false,
            representative_calculations: false,
            ems_override: false,
            outward_cos_tilt: cos_tilt,
            tilt: 90.0,
            azimuth: 0.0,
            area: 9.0,
            gross_area: 9.0,
            width: 3.0,
            perimeter: 12.0,
            vertices: vec![
                [0.0, 0.0, 0.0],
                [3.0, 0.0, 0.0],
                [3.0, 0.0, 3.0],
                [0.0, 0.0, 3.0],
            ],
        });
    }
    Ok((layout, zones, geometry, surfaces))
}

#[test]
fn source_ceiling_correlations_keep_class_thresholds_blend_and_objexx_ach_limits() -> Result {
    let cos45 = 2.0_f64.sqrt() / 2.0;
    for (cosine, forced) in [
        (-1.0, 4.11365377688938),
        (0.0, 3.17299636062606),
        (1.0, 9.35711423763866),
    ] {
        let natural = super::super::super::natural_coefficient_w_per_m2_k(24.0, 20.0, -cosine);
        for ach in [0.0, 0.5] {
            assert_eq!(
                coefficient(ach, 24.0, 20.0, cosine, 0.008, 3.0, false)?.to_bits(),
                natural.to_bits()
            );
        }
        assert_eq!(
            coefficient(1.75, 24.0, 20.0, cosine, 0.008, 3.0, false)?.to_bits(),
            (natural + ((forced - natural) * 0.5)).to_bits()
        );
        assert!(
            (coefficient(3.0, 24.0, 20.0, cosine, 0.008, 3.0, false)? - forced).abs() < 1.0e-13
        );
    }
    let wall = coefficient(3.0, 24.0, 20.0, 0.0, 0.008, 3.0, false)?;
    for cosine in [-cos45, cos45] {
        assert_eq!(
            coefficient(3.0, 24.0, 20.0, cosine, 0.008, 3.0, false)?.to_bits(),
            wall.to_bits()
        );
    }
    assert_eq!(
        coefficient(3.0, 24.0, 20.0, 1.0, 0.008, 3.0, true)?.to_bits(),
        coefficient(3.0, 24.0, 20.0, 1.0, 0.008, 3.0, false)?.to_bits(),
        "forced branch returns before the natural window child"
    );
    assert!(coefficient(0.5, 24.0, 20.0, 1.0, 0.008, 3.0, true).is_err());
    assert_eq!(limit_ach(20.0, 0.009999).to_bits(), 0.0_f64.to_bits());
    assert_eq!(limit_ach(20.0, 0.01), 20.0);
    assert_eq!(limit_ach(200.0, 0.01), 100.0);
    assert_eq!(limit_ach(-1.0, 0.01), 0.0);
    assert_eq!(
        limit_ach(f64::NAN, 0.01),
        100.0,
        "actual Objexx unordered min"
    );
    assert_eq!(limit_ach(-0.0, 0.01).to_bits(), (-0.0_f64).to_bits());
    Ok(())
}

#[test]
fn source_ceiling_ach_uses_physical_node_properties_before_minflow_and_actual_phase() -> Result {
    with_fresh_psychrometric_state(|| -> Result {
        let mut caller = Caller::new();
        let mut equipment = equipment()?;
        let mut zone = zone();
        // No physical allocation: the genuine first ACH boundary is zero.
        zone.system_zone_node_number = 0;
        assert_eq!(
            caller.call(|g, c, p, b| ach(&zone, &equipment, g, c, p, b))?,
            0.0
        );
        assert!(caller.stream.is_empty());
        zone.system_zone_node_number = 1;
        equipment.declare_ceiling_test_node(-101.0, 0.0);
        caller.clock.begin_envrn_flag = true;
        assert_eq!(
            caller.call(|g, c, p, b| ach(&zone, &equipment, g, c, p, b))?,
            0.0
        );
        assert!(
            caller.stream.is_empty(),
            "BeginEnvrn skips properties even with a physical Node"
        );
        caller.clock.begin_envrn_flag = false;
        caller.clock.warmup_flag = false;
        assert_eq!(
            caller.call(|g, c, p, b| ach(&zone, &equipment, g, c, p, b))?,
            0.0
        );
        assert_eq!(
            caller.errors.tracking.total_warning_errors, 1,
            "actual out-of-range Psat miss precedes the zero-flow gate"
        );
        assert_eq!(caller.recurring.number, 1);
        equipment.declare_ceiling_test_node(23.0, 0.06);
        zone.multiplier = 2;
        zone.list_multiplier = 3;
        let actual = caller.call(|g, c, p, b| ach(&zone, &equipment, g, c, p, b))?;
        let dew = SourcePsychrometricAccess
            .evaluate(
                crate::psychrometrics::EnergyPlusPsychrometricFunction::Psat,
                &[23.0],
            )?
            .result;
        let humidity = dew * 0.62198 / (101325.0 - dew);
        let density =
            crate::psychrometrics::energyplus_psy_rho_air_fn_pb_tdb_w(101325.0, 23.0, humidity);
        let expected = (0.06 / (density * 6.0)) / 90.0 * 3600.0;
        assert_eq!(actual.to_bits(), expected.to_bits());
        equipment.declare_ceiling_test_node(23.0, 0.059);
        assert_eq!(
            caller.call(|g, c, p, b| ach(&zone, &equipment, g, c, p, b))?,
            0.0
        );
        equipment.declare_ceiling_test_node(23.0, 1000.0);
        assert_eq!(
            caller.call(|g, c, p, b| ach(&zone, &equipment, g, c, p, b))?,
            100.0
        );
        Ok(())
    })
}

#[test]
fn source_ceiling_getuser_nodecheck_and_zone_prepass_keep_live_owners_once_and_one_clamp() -> Result
{
    with_fresh_psychrometric_state(|| -> Result {
        let (layout, mut zones, geometry, surfaces) = geometry()?;
        let mut caller = Caller::new();
        let mut equipment = equipment()?;
        equipment.declare_ceiling_test_node(23.0, 0.0);
        let ready = equipment.source_convection_readiness(false, false, false);
        let mut convection = SourceInsideConvection {
            state: InsideConvectionState::new_source_control(&layout),
            actual_no_user_input: None,
        };
        let mut metadata = vec![
            InsideConvectionSurfaceState::new(SurfaceId(8), [11, 12]),
            InsideConvectionSurfaceState::new(SurfaceId(2), [21, 22]),
        ];
        let input = SelectedInputProcessor::from_admitted_input(Vec::new(), BTreeMap::new())?;
        let mut scan = ReportScanState::default();
        zones.zones.as_mut().ok_or("zones")?[0].system_zone_node_number = 0;
        let before = static_zone_views(&zones)?;
        caller.call(|g, c, p, b| {
            convection.prepare_source_inside_prefix(
                &layout,
                &before,
                &surfaces,
                metadata.as_mut_slice(),
                &mut zones,
                ready,
                g,
                c,
                p.developer,
                &input,
                &ReportInputs::default(),
                &mut scan,
                b,
            )
        })?;
        assert!(!convection.state.get_user_input && !convection.state.node_check);
        assert!(!scan.get_report_input);
        assert_eq!(
            zones.zones.as_ref().ok_or("zones")?[0].inside_algorithm,
            Inside::Tarp
        );
        assert_eq!(static_zone_views(&zones)?[0].inside_algorithm, Inside::Tarp);
        assert_eq!(caller.errors.tracking.total_severe_errors, 1);
        assert!(
            metadata[0].interior_geometry[0] > 0.0,
            "GetUser metadata precedes NodeCheck"
        );
        let text = String::from_utf8(caller.stream.clone())?;
        assert!(
            text.find("Inside Convection=CeilingDiffuser")
                .ok_or("severe")?
                < text
                    .find("Defaulting inside convection to TARP")
                    .ok_or("continue")?
        );
        // A second call cannot repeat GetUser's geometry writes or NodeCheck.
        metadata[0].interior_geometry = [99.0; 4];
        let after = static_zone_views(&zones)?;
        caller.call(|g, c, p, b| {
            convection.prepare_source_inside_prefix(
                &layout,
                &after,
                &surfaces,
                metadata.as_mut_slice(),
                &mut zones,
                ready,
                g,
                c,
                p.developer,
                &input,
                &ReportInputs::default(),
                &mut scan,
                b,
            )
        })?;
        assert_eq!(metadata[0].interior_geometry, [99.0; 4]);
        assert_eq!(caller.errors.tracking.total_severe_errors, 1);
        // The native global enum remains CeilingDiffuser when a real system
        // Node exists; whole-Zone writes use Zone MAT, independently of Space MAT.
        zones.zones.as_mut().ok_or("zones")?[0].inside_algorithm = Inside::CeilingDiffuser;
        zones.zones.as_mut().ok_or("zones")?[0].system_zone_node_number = 1;
        let globals = &caller.globals;
        let mut air = SourceAirHeatBalance::default();
        air.allocate_source_arrays(globals)?;
        air.zones.as_mut().ok_or("air zones")?[0].air.mat = 20.0;
        air.zones.as_mut().ok_or("air zones")?[0]
            .air
            .air_hum_rat_avg = 0.008;
        air.spaces.as_mut().ok_or("air spaces")?[0].air.mat = 40.0;
        let temperatures = SourceTemperatures {
            owner: InsideSurfaceTemperatureOwner::SurfTempInTmp,
            layout: &layout,
            values: &[20.0, 24.0],
        };
        let mut cells = [-9.0, -10.0];
        let mut coefficients = SourceCoefficients {
            layout: &layout,
            values: &mut cells,
        };
        caller.call(|g, c, p, b| {
            initialize(
                &geometry,
                &zones,
                &air,
                &surfaces,
                &temperatures,
                &mut coefficients,
                &equipment,
                g,
                c,
                p,
                0.75,
                b,
            )
        })?;
        assert_eq!(
            *coefficients.borrow_coefficient(0)?,
            0.75,
            "one raw-zero lower clamp"
        );
        let expected = coefficient(0.0, 24.0, 20.0, 1.0, 0.008, 3.0, false)?;
        assert_eq!(
            coefficients.borrow_coefficient(1)?.to_bits(),
            expected.to_bits()
        );
        let retained = [
            *coefficients.borrow_coefficient(0)?,
            *coefficients.borrow_coefficient(1)?,
        ];
        let zone_views = static_zone_views(&zones)?;
        let space_air = SourceSpaceAirView {
            air: &air,
            metadata: &zones,
        };
        super::super::super::initialize_inside_convection_surfaces(
            &layout,
            &zone_views,
            &surfaces,
            metadata.as_slice(),
            &mut coefficients,
            &space_air,
            &temperatures,
            99.0,
            ConvectionEntry::Source,
        )?;
        assert_eq!(
            [
                *coefficients.borrow_coefficient(0)?,
                *coefficients.borrow_coefficient(1)?
            ],
            retained,
            "SetByZone Ceiling switch is a no-op, including no second clamp"
        );
        assert!(
            metadata
                .iter()
                .all(|row| row.model == InsideConvectionModel::SetByZone)
        );
        assert_eq!(
            super::super::super::validate(
                &layout,
                &zone_views,
                &surfaces,
                &convection.state,
                &[(ZoneId(7), [false; 3])][..],
                &coefficients,
                &space_air,
                &temperatures,
                metadata.as_slice(),
                ConvectionEntry::Legacy
            ),
            Err(Unavailable::AlgorithmOrSurface),
            "legacy cannot borrow missing Source property owners"
        );
        // A Continue write failure retains the completed GetUser, cleared flag,
        // and severe prefix, while the live Zone fallback has not happened.
        struct FailContinue(Vec<u8>);
        impl std::io::Write for FailContinue {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                if bytes
                    .windows(b"Defaulting".len())
                    .any(|part| part == b"Defaulting")
                {
                    return Err(std::io::Error::other("declared Continue write failure"));
                }
                self.0.extend_from_slice(bytes);
                Ok(bytes.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut failure = SourceInsideConvection {
            state: InsideConvectionState::new_source_control(&layout),
            actual_no_user_input: None,
        };
        zones.zones.as_mut().ok_or("zones")?[0].system_zone_node_number = 0;
        let before = static_zone_views(&zones)?;
        let mut errors = super::super::super::tests::error_state();
        let mut stream = FailContinue(Vec::new());
        let mut console = Vec::new();
        let mut backend = SourceErrorBackend {
            state: &mut errors,
            error_stream: Some(&mut stream),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        let result = failure.prepare_source_inside_prefix(
            &layout,
            &before,
            &surfaces,
            metadata.as_mut_slice(),
            &mut zones,
            ready,
            &caller.globals,
            &caller.clock,
            &caller.developer,
            &input,
            &ReportInputs::default(),
            &mut scan,
            &mut backend,
        );
        assert!(matches!(result, Err(Error::Backend(_))));
        assert!(!failure.state.node_check && !failure.state.get_user_input);
        assert_eq!(
            zones.zones.as_ref().ok_or("zones")?[0].inside_algorithm,
            Inside::CeilingDiffuser
        );
        assert_eq!(errors.tracking.total_severe_errors, 1);
        assert!(
            errors
                .tracking
                .last_severe_error
                .contains("Inside Convection=CeilingDiffuser")
        );
        assert!(String::from_utf8(stream.0)?.contains("Inside Convection=CeilingDiffuser"));
        assert_ne!(
            metadata[0].interior_geometry, [99.0; 4],
            "GetUser happened before the failing NodeCheck"
        );
        Ok(())
    })
}
