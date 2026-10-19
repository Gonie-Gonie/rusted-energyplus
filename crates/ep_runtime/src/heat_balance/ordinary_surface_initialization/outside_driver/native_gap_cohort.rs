//! Whole selected CalcOutsideSurfTemp consumer through the actual private scalar.
//! Named coefficients/history constants are narrow consumer inputs, never CTF outputs.
use super::*;
use crate::heat_balance as hb;
use ep_model::*;
use serde_json::Value;
use std::{collections::BTreeMap, io};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn member<'a>(v: &'a Value, key: &str) -> Result<&'a Value> {
    Ok(v.get(key).ok_or_else(|| invalid(key))?)
}
fn array(v: &Value) -> Result<&Vec<Value>> {
    Ok(v.as_array().ok_or_else(|| invalid("actual array"))?)
}
fn raw(v: &Value) -> Result<f64> {
    let text = member(v, "value_bits")?
        .as_str()
        .ok_or_else(|| invalid("raw16"))?;
    if v.as_object().map_or(0, |m| m.len()) != 1
        || text.len() != 16
        || !text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(invalid("one exact finite raw16 scalar").into());
    }
    let x = f64::from_bits(u64::from_str_radix(text, 16)?);
    if !x.is_finite() {
        return Err(invalid("finite input/observed scalar").into());
    }
    Ok(x)
}
fn same<T: PartialEq + std::fmt::Debug>(a: T, b: T, label: &str) -> Result {
    if a != b {
        return Err(invalid(&format!("{label}: Source {a:?}; Native {b:?}")).into());
    }
    Ok(())
}
fn bits(x: f64, observed: &Value, label: &str) -> Result {
    same(x.to_bits(), raw(observed)?.to_bits(), label)
}
fn retained(identity: &mut Option<Vec<usize>>, now: Vec<usize>, label: &str) -> Result {
    if now.iter().any(|&p| p == 0) {
        return Err(invalid(label).into());
    }
    if let Some(old) = identity {
        same(&now, &*old, label)
    } else {
        *identity = Some(now);
        Ok(())
    }
}
fn vector(source: &Option<Vec<f64>>, native: &Value, label: &str, ids: &mut Vec<usize>) -> Result {
    let source = source
        .as_ref()
        .ok_or_else(|| invalid("actual selected Source array allocation"))?;
    same(member(native, "allocated")?.as_bool(), Some(true), label)?;
    same(
        member(native, "values_defined")?.as_bool(),
        Some(true),
        label,
    )?;
    same(
        member(native, "size")?.as_u64(),
        Some(source.len() as u64),
        label,
    )?;
    same(member(native, "lower")?.as_i64(), Some(1), label)?;
    same(
        member(native, "upper")?.as_i64(),
        Some(source.len() as i64),
        label,
    )?;
    let p = member(native, "data_address")?
        .as_u64()
        .filter(|&p| p != 0)
        .ok_or_else(|| invalid("nonnull actual Native array data"))?;
    ids.push(usize::try_from(p)?);
    let values = array(member(native, "values")?)?;
    same(values.len(), source.len(), label)?;
    for (a, b) in source.iter().zip(values) {
        bits(*a, b, label)?;
    }
    Ok(())
}

fn coefficient_model() -> TypedModel {
    let mut m = TypedModel::default();
    m.timestep.number_of_timesteps_per_hour = 4;
    m.ctf_report_names.materials = vec![(MaterialId(0), "Declared R".into())];
    m.ctf_report_names.constructions = vec![(ConstructionId(0), "Declared C".into())];
    m.materials.push(Material {
        id: MaterialId(0),
        name: NormalizedName::new("Declared R"),
        definition: MaterialDefinition::NoMass(NoMassMaterial {
            roughness: MaterialSurfaceRoughness::MediumRough,
            thermal_resistance_m2_k_per_w: 2.0,
            surface: OpaqueSurfaceProperties::default(),
        }),
    });
    m.constructions.push(Construction {
        id: ConstructionId(0),
        name: NormalizedName::new("Declared C"),
        kind: ConstructionKind::Opaque,
        outside_layer: Some(MaterialId(0)),
        layers: vec![MaterialId(0)],
        thermochromic_master: None,
        ground_factor: None,
        air_boundary: None,
        complex_fenestration: None,
        window_equivalent_layer: None,
        internal_heat_source: None,
    });
    m.surfaces.push(Surface {
        id: SurfaceId(0),
        name: NormalizedName::new("Declared S"),
        surface_type: SurfaceType::Wall,
        construction: ConstructionId(0),
        zone: ZoneId(0),
        space: SpaceId(0),
        outside_boundary_condition: OutsideBoundaryCondition::Adiabatic,
        outside_boundary_condition_object: None,
        sun_exposure: SunExposure::NoSun,
        wind_exposure: WindExposure::NoWind,
        view_factor_to_ground: AutoOrNumber::Value(0.0),
        vertices: Vec::new(),
        computed_geometry: None,
    });
    m
}

struct Factory {
    surface: OrdinarySurfaceInitialization,
    geometry: SurfaceGeometryInputState,
    zones: SelectedZoneInputOwners,
    air: SourceAirHeatBalance,
    control: SelectedHeatBalanceInputState,
    public: OrderedCtfPublicInitialization,
    history: CtfThermalHistoryState,
    radiation: OrdinarySolarRadiation,
    globals: SelectedHeatBalanceGlobals,
    clock: WeatherGlobalState,
    environment: WeatherEnvironmentState,
    convection: SourceInsideConvection,
    input: SelectedInputProcessor,
    reports: ReportInputs,
    scan: ReportScanState,
    errors: hb::error_backend::SourceErrorState,
    text: Vec<u8>,
    console: Vec<u8>,
}
impl Factory {
    fn new(named: &Value) -> Result<Self> {
        let name = member(named, "name")?
            .as_str()
            .ok_or_else(|| invalid("name"))?;
        // Genuine producer creates the normal public owner/return route. Only
        // its three explicitly declared consumer operands are overwritten below.
        let mut cache = hb::surface_manager::ConstructionThermalDataCache::build(
            &coefficient_model(),
            &BTreeMap::new(),
        )?;
        let mut public = cache
            .take_public_initialization()
            .ok_or_else(|| invalid("actual returned CTF public owner"))?;
        same(
            public.constructions.len(),
            1,
            "actual CTF construction count",
        )?;
        let row = &mut public.constructions[0];
        same(
            row.construction_id,
            ConstructionId(0),
            "real Construction identity",
        )?;
        let c = &mut row
            .result
            .as_mut()
            .map_err(|e| invalid(&format!("actual CTF return: {e:?}")))?
            .storage;
        c.inside[0] = raw(member(named, "CTFInside0")?)?;
        c.outside[0] = raw(member(named, "CTFOutside0")?)?;
        c.cross[0] = raw(member(named, "CTFCross0")?)?;
        let geometry = SurfaceGeometryInputState::native_gap_outside_scalar_row(name)?;
        let mut control = SelectedHeatBalanceInputState::default();
        control.construction_usage = vec![Default::default()];
        control.construction_usage[0].construction_id = Some(ConstructionId(0));
        same(
            control.construction_usage[0].source_sink_present,
            false,
            "actual Source Construction ctor",
        )?;
        // Zero-Zone owner is an unconsumed constructor context for the required
        // convection borrow. No convection/getter/latch or full parent is called.
        let mut zones = SelectedZoneInputOwners::default();
        zones.zones = Some(Vec::new());
        let convection = SourceInsideConvection::new(&geometry, &zones)?;
        let mut globals = SelectedHeatBalanceGlobals::default();
        globals.num_spaces = 1;
        let mut air = SourceAirHeatBalance::default();
        air.allocate_source_arrays(&globals)?;
        let mut history = CtfThermalHistoryState::new_source_unallocated(vec![SurfaceId(0)]);
        history.normal.outside_temperature[0] = vec![raw(member(named, "initial_TH11")?)?];
        let mut surface = OrdinarySurfaceInitialization::default();
        surface.arrays.surf_qdot_rad_out_rep_per_area =
            Some(vec![raw(member(named, "initial_radiation_report")?)?]);
        Ok(Self {
            surface,
            geometry,
            zones,
            air,
            control,
            public,
            history,
            radiation: OrdinarySolarRadiation::for_surface_input(
                hb::radiation::ordinary::OrdinaryRadiationInput::from_admitted_inventory(
                    std::iter::empty::<(&str, usize)>(),
                    false,
                )?,
            ),
            globals,
            clock: WeatherGlobalState::default(),
            environment: WeatherEnvironmentState::default(),
            convection,
            input: SelectedInputProcessor::from_admitted_input(Vec::new(), BTreeMap::new())?,
            reports: ReportInputs::default(),
            scan: ReportScanState::default(),
            errors: hb::error_backend::SourceErrorState {
                output_error_header: true,
                version_string: String::new(),
                idd_version_string: String::new(),
                doing_input_processing: false,
                print_console_output: false,
                kickoff_simulation: false,
                developer_flag: false,
                warmup_flag: false,
                doing_sizing: false,
                tracking: hb::error_backend::SourceErrorTracking {
                    ask_for_plant_check_on_abort: false,
                    exit_during_simulations: false,
                    match_counts: [0; 20],
                    abort_processing: false,
                    ask_for_connections_report: false,
                    ask_for_surfaces_report: false,
                    total_multiplied_windows: 0,
                    total_coincident_vertices: 0,
                    total_degenerate_surfaces: 0,
                    total_receiving_non_convex_surfaces: 0,
                    total_casting_non_convex_surfaces: 0,
                    total_severe_errors: 0,
                    total_severe_errors_during_warmup: 0,
                    total_severe_errors_during_sizing: 0,
                    last_severe_error: String::new(),
                    total_warning_errors: 0,
                    total_warning_errors_during_warmup: 0,
                    total_warning_errors_during_sizing: 0,
                },
            },
            text: Vec::new(),
            console: Vec::new(),
        })
    }
    fn load(&mut self, step: &Value) -> Result {
        // Retain each allocation across repeats; modify only primary cells.
        macro_rules! input {
            ($field:ident,$name:literal) => {{
                let x = raw(member(step, $name)?)?;
                let v = self.surface.arrays.$field.get_or_insert_with(|| vec![x]);
                same(v.len(), 1, $name)?;
                v[0] = x;
            }};
        }
        input!(surf_h_conv_int, "SurfHConvInt");
        input!(surf_h_conv_ext, "SurfHConvExt");
        input!(surf_h_air_ext, "SurfHAirExt");
        input!(surf_h_sky_ext, "SurfHSkyExt");
        input!(surf_h_grd_ext, "SurfHGrdExt");
        input!(surf_h_srd_surf_ext, "SurfHSrdSurfExt");
        input!(surf_ctf_const_in_part, "SurfCTFConstInPart");
        input!(surf_ctf_const_out_part, "SurfCTFConstOutPart");
        input!(surf_opaq_q_rad_sw_in_abs, "SurfOpaqQRadSWInAbs");
        input!(surf_opaq_q_rad_sw_out_abs, "SurfOpaqQRadSWOutAbs");
        input!(
            surf_q_additional_heat_source_outside,
            "SurfQAdditionalHeatSourceOutside"
        );
        input!(
            surf_qdot_rad_net_lw_in_per_area,
            "SurfQdotRadNetLWInPerArea"
        );
        input!(surf_temp_in, "SurfTempIn");
        input!(
            surf_qdot_rad_int_gains_in_per_area,
            "SurfQdotRadIntGainsInPerArea"
        );
        self.air
            .spaces
            .as_mut()
            .ok_or_else(|| invalid("actual SpaceHB"))?[0]
            .air
            .mat = raw(member(step, "MAT")?)?;
        self.environment.current_weather.sky_temp = raw(member(step, "SkyTemp")?)?;
        self.environment.current_weather.out_dry_bulb_temp = raw(member(step, "OutDryBulbTemp")?)?;
        Ok(())
    }
    fn observe(
        &self,
        native: &Value,
        error_flag: bool,
        native_ids: &mut Option<Vec<usize>>,
        source_ids: &mut Option<Vec<usize>>,
    ) -> Result {
        let row = &self
            .geometry
            .final_surfaces
            .as_ref()
            .ok_or_else(|| invalid("actual Surface"))?[0];
        let coefficients = &self.public.constructions[0]
            .result
            .as_ref()
            .map_err(|_| invalid("actual CTF public return"))?
            .storage;
        let mut ni = Vec::new();
        for name in [
            "state_address",
            "surface_address",
            "construction_address",
            "space_air_address",
        ] {
            ni.push(usize::try_from(
                member(native, name)?
                    .as_u64()
                    .filter(|&v| v != 0)
                    .ok_or_else(|| invalid("nonnull Native owner"))?,
            )?);
        }
        same(
            member(native, "surface_Name")?.as_str(),
            Some(row.row.name.as_str()),
            "surface Name",
        )?;
        same(row.row.class, SurfaceClass::Wall, "actual Source Wall")?;
        same(
            member(native, "surface_Class")?.as_i64(),
            Some(1),
            "pinned Native Wall1",
        )?;
        same(
            member(native, "OSCMPtr")?.as_i64(),
            Some(row.oscm_ptr as i64),
            "OSCMPtr",
        )?;
        same(
            member(native, "SurfHasSurroundingSurfProperty")?.as_bool(),
            Some(row.has_surrounding_surface_property),
            "surrounding property",
        )?;
        same(
            member(native, "UseSurfPropertyGndSurfTemp")?.as_bool(),
            Some(row.use_ground_surface_temperature),
            "ground property",
        )?;
        same(
            member(native, "SourceSinkPresent")?.as_bool(),
            Some(self.control.construction_usage[0].source_sink_present),
            "SourceSinkPresent",
        )?;
        bits(
            coefficients.inside[0],
            member(native, "CTFInside0")?,
            "CTFInside0",
        )?;
        bits(
            coefficients.outside[0],
            member(native, "CTFOutside0")?,
            "CTFOutside0",
        )?;
        bits(
            coefficients.cross[0],
            member(native, "CTFCross0")?,
            "CTFCross0",
        )?;
        bits(
            *self.air.source_space_mat(1)?,
            member(native, "MAT")?,
            "MAT",
        )?;
        bits(
            self.environment.current_weather.sky_temp,
            member(native, "SkyTemp")?,
            "SkyTemp",
        )?;
        bits(
            self.environment.current_weather.out_dry_bulb_temp,
            member(native, "OutDryBulbTemp")?,
            "OutDryBulbTemp",
        )?;
        let arrays = member(native, "arrays")?;
        macro_rules! compare {
            ($field:ident,$name:literal) => {
                vector(
                    &self.surface.arrays.$field,
                    member(arrays, $name)?,
                    $name,
                    &mut ni,
                )?;
            };
        }
        compare!(surf_h_conv_int, "SurfHConvInt");
        compare!(surf_h_conv_ext, "SurfHConvExt");
        compare!(surf_h_air_ext, "SurfHAirExt");
        compare!(surf_h_sky_ext, "SurfHSkyExt");
        compare!(surf_h_grd_ext, "SurfHGrdExt");
        compare!(surf_h_srd_surf_ext, "SurfHSrdSurfExt");
        compare!(surf_ctf_const_in_part, "SurfCTFConstInPart");
        compare!(surf_ctf_const_out_part, "SurfCTFConstOutPart");
        compare!(surf_opaq_q_rad_sw_in_abs, "SurfOpaqQRadSWInAbs");
        compare!(surf_opaq_q_rad_sw_out_abs, "SurfOpaqQRadSWOutAbs");
        compare!(
            surf_q_additional_heat_source_outside,
            "SurfQAdditionalHeatSourceOutside"
        );
        compare!(
            surf_qdot_rad_net_lw_in_per_area,
            "SurfQdotRadNetLWInPerArea"
        );
        compare!(surf_temp_in, "SurfTempIn");
        compare!(surf_qdot_rad_out_rep_per_area, "SurfQdotRadOutRepPerArea");
        vector(
            &self.surface.arrays.surf_qdot_rad_int_gains_in_per_area,
            member(native, "SurfQdotRadIntGainsInPerArea")?,
            "SurfQdotRadIntGainsInPerArea",
            &mut ni,
        )?;
        vector(
            &Some(self.history.normal.outside_temperature[0].clone()),
            member(native, "SurfOutsideTempHist1")?,
            "SurfOutsideTempHist1",
            &mut ni,
        )?;
        // ErrorFlag is not mutated anywhere in this selected Native branch.
        // Source expresses unsupported positive diagnostics as Result errors;
        // it does not invent a second persistent bool owner for this unit.
        same(
            member(native, "ErrorFlag")?.as_bool(),
            Some(error_flag),
            "Native caller ErrorFlag pass-through",
        )?;
        let d = member(native, "diagnostics")?;
        same(
            member(d, "text")?.as_str(),
            Some(std::str::from_utf8(&self.text)?),
            "whole ERR text",
        )?;
        same(
            member(d, "warnings")?.as_i64(),
            Some(self.errors.tracking.total_warning_errors as i64),
            "warnings",
        )?;
        same(
            member(d, "severe")?.as_i64(),
            Some(self.errors.tracking.total_severe_errors as i64),
            "severe",
        )?;
        same(
            member(d, "recurring")?.as_i64(),
            Some(0),
            "no recurring branch",
        )?;
        retained(
            native_ids,
            ni,
            "within-lifetime actual Native owners/buffers",
        )?;
        let mut si = vec![
            self as *const Self as usize,
            row as *const _ as usize,
            &self.public.constructions[0] as *const _ as usize,
            &self.air.spaces.as_ref().unwrap()[0] as *const _ as usize,
            self.history.normal.outside_temperature[0].as_ptr() as usize,
        ];
        macro_rules! pointer {
            ($field:ident) => {
                si.push(
                    self.surface
                        .arrays
                        .$field
                        .as_ref()
                        .ok_or_else(|| invalid("real allocation"))?
                        .as_ptr() as usize,
                );
            };
        }
        pointer!(surf_h_conv_int);
        pointer!(surf_h_conv_ext);
        pointer!(surf_h_air_ext);
        pointer!(surf_h_sky_ext);
        pointer!(surf_h_grd_ext);
        pointer!(surf_h_srd_surf_ext);
        pointer!(surf_ctf_const_in_part);
        pointer!(surf_ctf_const_out_part);
        pointer!(surf_opaq_q_rad_sw_in_abs);
        pointer!(surf_opaq_q_rad_sw_out_abs);
        pointer!(surf_q_additional_heat_source_outside);
        pointer!(surf_qdot_rad_net_lw_in_per_area);
        pointer!(surf_temp_in);
        pointer!(surf_qdot_rad_out_rep_per_area);
        pointer!(surf_qdot_rad_int_gains_in_per_area);
        retained(
            source_ids,
            si,
            "within-lifetime actual Source owners/buffers",
        )
    }
    fn call(&mut self, step: &Value, h_movable: f64) -> Result {
        let mut backend = SourceErrorBackend {
            state: &mut self.errors,
            error_stream: Some(&mut self.text),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        let mut o = SourceOutsideCtfOwners {
            geometry: &mut self.geometry,
            zones: &self.zones,
            air: &self.air,
            control: &self.control,
            public: &self.public,
            history: &mut self.history,
            radiation: &mut self.radiation,
            physical: &[],
            globals: &self.globals,
            clock: &self.clock,
            environment: &self.environment,
            convection: &mut self.convection,
            input: &self.input,
            reports: &self.reports,
            scan: &mut self.scan,
            carroll_method: false,
            backend: &mut backend,
        };
        scalar::calculate(
            &mut self.surface,
            &mut o,
            1,
            1,
            1,
            h_movable,
            raw(member(step, "TempExt")?)?,
        )?;
        Ok(())
    }
}

impl OrdinarySurfaceInitialization {
    pub(crate) fn run_native_gap_outside_surface_cohort(fixture: &Value) -> Result<usize> {
        let named = member(member(fixture, "named_inputs")?, "outside_surface")?;
        let native = member(
            member(member(fixture, "native_output")?, "groups")?,
            "outside_surface",
        )?;
        same(
            member(named, "schema")?.as_str(),
            Some("src06-outside-surface-gap-input.v1"),
            "named schema",
        )?;
        same(
            member(native, "schema")?.as_str(),
            Some("src06-original-outside-surface-gap-output.v1"),
            "actual Native schema",
        )?;
        let cases = array(member(named, "cases")?)?;
        let outputs = array(member(native, "cases")?)?;
        same(cases.len(), 7, "named lifetimes")?;
        same(outputs.len(), cases.len(), "Native lifetimes")?;
        let mut calls = 0;
        for (case, out) in cases.iter().zip(outputs) {
            same(
                member(out, "input")?,
                case,
                "full actual consumed input tree",
            )?;
            let mut factory = Factory::new(case)?;
            let h_movable = raw(member(case, "HMovInsul")?)?;
            same(h_movable.to_bits(), 0.0f64.to_bits(), "selected no movable")?;
            let error_flag = member(case, "initial_ErrorFlag")?
                .as_bool()
                .ok_or_else(|| invalid("caller ErrorFlag input"))?;
            let (mut native_ids, mut source_ids) = (None, None);
            let steps = array(member(case, "steps")?)?;
            let records = array(member(out, "calls")?)?;
            same(steps.len(), records.len(), "all actual calls")?;
            for (step, record) in steps.iter().zip(records) {
                same(member(record, "primary_step")?, step, "per-step input")?;
                factory.load(step)?;
                factory.observe(
                    member(record, "before")?,
                    error_flag,
                    &mut native_ids,
                    &mut source_ids,
                )?;
                factory.call(step, h_movable)?;
                factory.observe(
                    member(record, "after")?,
                    error_flag,
                    &mut native_ids,
                    &mut source_ids,
                )?;
                calls += 1;
            }
        }
        same(calls, 24, "complete selected Original calls")?;
        println!(
            "Source outside-surface cohort: {calls} complete selected CalcOutsideSurfTemp calls; 7 lifetimes; narrow coefficient/history inputs; no full upstream/outside parent claim"
        );
        Ok(calls)
    }
}
