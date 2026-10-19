//! Real full AnisoSkyViewFactors and opaque CalcAbsorbed, no beam producer claim.
use super::*;
use crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray;
use crate::heat_balance::selected_heat_balance_input::SelectedSolarDistribution;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};

const OUTPUT_ARRAYS: [&str; 6] = [
    "SurfAnisoSkyMult",
    "SurfMultIsoSky",
    "SurfMultCircumSolar",
    "SurfMultHorizonZenith",
    "SurfCurDifShdgRatioIsoSky",
    "SurfOpaqAO",
];
fn native_address(value: &Value) -> Result<u64> {
    let value = value.as_u64().ok_or("Native address must be unsigned")?;
    if value == 0 {
        return Err("Native address must be nonnull".into());
    }
    Ok(value)
}
fn native_sky_identity(value: &Value) -> Result<Vec<u64>> {
    let mut identity = vec![
        native_address(&value["state_address"])?,
        native_address(&value["module_address"])?,
    ];
    for row in array(&value["surfaces"])? {
        identity.push(native_address(&row["address"])?);
    }
    for key in OUTPUT_ARRAYS {
        identity.push(native_address(&value[key]["data_address"])?);
    }
    Ok(identity)
}
fn source_sky_identity(
    owner: &OrdinarySolarRadiation,
    geometry: &SurfaceGeometryInputState,
) -> Result<Vec<usize>> {
    let rows = geometry
        .final_surfaces
        .as_ref()
        .ok_or("same physical Surface storage")?;
    let shading = &owner.solar_timestep.shading;
    let opaque = &geometry
        .window_arrays
        .as_ref()
        .ok_or("same physical window-array owner")?
        .surf_opaq_ao;
    let mut identity = vec![
        owner as *const _ as usize,
        geometry as *const _ as usize,
        rows.as_ptr() as usize,
    ];
    for row in rows {
        identity.push(row as *const _ as usize);
        identity.push(&row.row as *const _ as usize);
    }
    for values in [
        &shading.surf_aniso_sky_mult,
        &shading.surf_mult_iso_sky,
        &shading.surf_mult_circum_solar,
        &shading.surf_mult_horizon_zenith,
        &shading.surf_cur_dif_shdg_ratio_iso_sky,
        opaque,
    ] {
        if values.values.is_empty() {
            return Err("actual nonempty Source solar output storage".into());
        }
        identity.push(values.values.as_ptr() as usize);
    }
    if identity.contains(&0) {
        return Err("actual Source owner/storage address must be nonnull".into());
    }
    Ok(identity)
}

fn solar_array(v: &SourceSolarArray<f64, 1>, e: &Value, label: &str) -> Result<()> {
    let n = v.dimensions.ok_or("actual solar unit allocated output")?[0];
    if e["allocated"].as_bool() != Some(true)
        || e["size"].as_u64() != Some(n as u64)
        || e["lower"].as_i64() != Some(1)
        || e["upper"].as_u64() != Some(n as u64)
    {
        return Err(format!("{label} storage differs").into());
    }
    if e["values_defined"].as_bool() == Some(false) {
        if !e["values"].is_null() || v.values.iter().any(Option::is_some) {
            return Err(format!("{label} undefined mask differs").into());
        }
        return Ok(());
    }
    if e["values_defined"].as_bool() != Some(true) {
        return Err("Native output definedness absent".into());
    }
    let expected = array(&e["values"])?;
    if expected.len() != n {
        return Err("output extent differs".into());
    }
    for (i, e) in expected.iter().enumerate() {
        bits(v.read([i + 1])?, e, label)?;
    }
    Ok(())
}
fn diagnostics(state: &SourceErrorState, bytes: &[u8], e: &Value) -> Result<()> {
    if e["warnings"].as_i64() != Some(i64::from(state.tracking.total_warning_errors))
        || e["severe"].as_i64() != Some(i64::from(state.tracking.total_severe_errors))
        || e["recurring"].as_i64() != Some(0)
        || e["text"].as_str() != Some(std::str::from_utf8(bytes)?)
        || state.output_error_header != bytes.is_empty()
    {
        return Err("entire Sky/opaque diagnostics differ".into());
    }
    Ok(())
}
impl OrdinarySolarRadiation {
    fn assert_native_gap_sky(
        &self,
        g: &SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        clock: &WeatherGlobalState,
        env: &WeatherEnvironmentState,
        detailed: bool,
        varying: bool,
        errors: &SourceErrorState,
        bytes: &[u8],
        e: &Value,
        retained_source_identity: &[usize],
    ) -> Result<()> {
        if source_sky_identity(self, g)? != retained_source_identity {
            return Err("Source repeated Solar owner/physical rows/output storage differs".into());
        }
        if e["HourOfDay"].as_i64() != Some(i64::from(clock.hour_of_day))
            || e["TimeStep"].as_i64() != Some(i64::from(clock.time_step))
            || e["DetailedSkyDiffuseAlgorithm"].as_bool() != Some(detailed)
            || e["ShadingTransmittanceVaries"].as_bool() != Some(varying)
        {
            return Err("actual solar clock/flags differ".into());
        }
        let distribution = match control.solar_distribution {
            SelectedSolarDistribution::Minimal => 0,
            SelectedSolarDistribution::FullExterior => 1,
            SelectedSolarDistribution::FullInteriorExterior => 2,
        };
        if e["SolarDistribution"].as_i64() != Some(distribution) {
            return Err("actual solar distribution differs".into());
        }
        values(
            &env.current_weather.solar_direction_cosines,
            &e["SOLCOS"],
            "actual sun direction",
        )?;
        for (x, k) in [
            (env.elevation, "Elevation"),
            (env.current_weather.beam_solar_rad, "BeamSolarRad"),
            (env.current_weather.dif_solar_rad, "DifSolarRad"),
        ] {
            bits(x, &e[k], k)?;
        }
        let rows = g.final_surfaces.as_ref().ok_or("actual geometry rows")?;
        let expected = array(&e["surfaces"])?;
        if rows.len() != expected.len() {
            return Err("actual solar row extent differs".into());
        }
        let hour = usize::try_from(clock.hour_of_day)?;
        let step = usize::try_from(clock.time_step)?;
        for (i, (row, e)) in rows.iter().zip(expected).enumerate() {
            if e["ordinal"].as_u64() != Some((i + 1) as u64)
                || e["Name"].as_str() != Some(&row.row.name)
                || e["Construction"].as_i64() != Some(i64::from(row.row.construction))
                || e["Class"].as_i64() != Some(1)
                || e["OriginalClass"].as_i64() != Some(1)
                || row.row.class
                    != crate::heat_balance::selected_heat_balance_input::surface::SurfaceClass::Wall
                || row.original_class
                    != crate::heat_balance::selected_heat_balance_input::surface::SurfaceClass::Wall
                || e["ExtSolar"].as_bool() != Some(row.row.exterior_solar)
            {
                return Err("actual solar row identity/class/exposure differs".into());
            }
            let p = row
                .row
                .vertex_progress
                .properties
                .ok_or("declared scalar geometry")?;
            values(&p.out_norm, &e["OutNormVec"], "declared normal")?;
            let construction = control
                .construction_usage
                .get(i)
                .ok_or("actual constructor construction")?;
            for (x, k) in [
                (p.tilt_deg, "Tilt"),
                (p.sin_tilt, "SinTilt"),
                (row.row.vertex_progress.view_factor_sky, "ViewFactorSky"),
                (construction.outside_absorp_solar, "OutsideAbsorpSolar"),
                (construction.trans_diff, "TransDiff"),
                (
                    self.solar_timestep
                        .heat_balance
                        .surf_cos_inc_ang
                        .read([hour, step, i + 1])?,
                    "current_cosine",
                ),
                (
                    self.solar_timestep
                        .heat_balance
                        .surf_sunlit_frac
                        .read([hour, step, i + 1])?,
                    "current_sunlit",
                ),
                (
                    self.solar_timestep
                        .shading
                        .surf_dif_shdg_ratio_iso_sky
                        .read([i + 1])?,
                    "isotropic_ratio",
                ),
                (
                    self.solar_timestep
                        .shading
                        .surf_dif_shdg_ratio_horiz
                        .read([i + 1])?,
                    "horizon_ratio",
                ),
                (
                    self.solar_timestep
                        .shading
                        .surf_dif_shdg_ratio_iso_sky_hrts
                        .read([step, hour, i + 1])?,
                    "hrts_isotropic_ratio",
                ),
                (
                    self.solar_timestep
                        .shading
                        .surf_dif_shdg_ratio_horiz_hrts
                        .read([step, hour, i + 1])?,
                    "hrts_horizon_ratio",
                ),
            ] {
                bits(x, &e[k], k)?;
            }
        }
        let s = &self.solar_timestep.shading;
        for (a, k) in [
            (&s.surf_aniso_sky_mult, "SurfAnisoSkyMult"),
            (&s.surf_mult_iso_sky, "SurfMultIsoSky"),
            (&s.surf_mult_circum_solar, "SurfMultCircumSolar"),
            (&s.surf_mult_horizon_zenith, "SurfMultHorizonZenith"),
            (
                &s.surf_cur_dif_shdg_ratio_iso_sky,
                "SurfCurDifShdgRatioIsoSky",
            ),
        ] {
            solar_array(a, &e[k], k)?;
        }
        solar_array(
            &g.window_arrays
                .as_ref()
                .ok_or("actual opaque output owner")?
                .surf_opaq_ao,
            &e["SurfOpaqAO"],
            "SurfOpaqAO",
        )?;
        diagnostics(errors, bytes, &e["diagnostics"])
    }
    pub(crate) fn native_gap_sky_opaque_cases(input: &Value, native: &Value) -> Result<usize> {
        let cases = array(&input["solar_surface"])?;
        let expected = array(&native["solar_surface"])?;
        if cases.len() != expected.len() {
            return Err("solar cases extent differs".into());
        }
        let mut count = 0;
        for (case, oracle) in cases.iter().zip(expected) {
            if case != &oracle["input"] || case["name"] != oracle["name"] {
                return Err("whole solar primary differs".into());
            }
            let definitions = array(&case["surfaces"])?;
            let n = definitions.len();
            let steps = usize::try_from(case["steps_per_hour"].as_u64().ok_or("clock extent")?)?;
            if n == 0 || n > 8 || steps == 0 || steps > 60 {
                return Err("declared unit shape domain".into());
            }
            let mut geometry = SurfaceGeometryInputState::native_gap_opaque_radiation_rows(n)?;
            let mut control = SelectedHeatBalanceInputState::default();
            for (i, p) in definitions.iter().enumerate() {
                let normal = array(&p["normal"])?;
                if normal.len() != 3 {
                    return Err("normal component extent".into());
                }
                geometry.native_gap_sky_row_inputs(
                    i,
                    p["name"].as_str().ok_or("surface name")?,
                    [real(&normal[0])?, real(&normal[1])?, real(&normal[2])?],
                    real(&p["tilt"])?,
                    real(&p["sin_tilt"])?,
                    real(&p["sky_view"])?,
                )?;
                control.construction_usage.push(SelectedConstructionUsage {
                    outside_absorp_solar: real(&p["outside_absorptance"])?,
                    ..Default::default()
                });
            }
            let mut zones = SelectedZoneInputOwners::default();
            zones.zones = Some(vec![SelectedZoneData {
                space_indexes: vec![1],
                ..Default::default()
            }]);
            zones.spaces = Some(vec![Default::default()]);
            let space = zones
                .spaces
                .as_mut()
                .and_then(|r| r.first_mut())
                .ok_or("actual Space")?;
            space.ht_surface_first = 1;
            space.ht_surface_last = i32::try_from(n)?;
            let windows = geometry
                .window_arrays
                .as_mut()
                .ok_or("actual DataSurface arrays")?;
            for (i, a) in windows.surf_active_construction.iter_mut().enumerate() {
                *a = i32::try_from(i + 1)?;
            }
            windows.surf_opaq_ao.dimension([n], None)?;
            let inventory = OrdinaryRadiationInput::from_admitted_inventory(
                std::iter::empty::<(&str, usize)>(),
                false,
            )?;
            let mut owner = Self::for_surface_input(inventory);
            let s = &mut owner.solar_timestep.shading;
            for a in [
                &mut s.surf_aniso_sky_mult,
                &mut s.surf_mult_iso_sky,
                &mut s.surf_mult_circum_solar,
                &mut s.surf_mult_horizon_zenith,
                &mut s.surf_cur_dif_shdg_ratio_iso_sky,
                &mut s.surf_dif_shdg_ratio_iso_sky,
                &mut s.surf_dif_shdg_ratio_horiz,
            ] {
                a.dimension([n], None)?;
            }
            s.surf_dif_shdg_ratio_iso_sky_hrts
                .dimension([steps, 24, n], None)?;
            s.surf_dif_shdg_ratio_horiz_hrts
                .dimension([steps, 24, n], None)?;
            owner
                .solar_timestep
                .heat_balance
                .surf_cos_inc_ang
                .dimension([24, steps, n], None)?;
            owner
                .solar_timestep
                .heat_balance
                .surf_sunlit_frac
                .dimension([24, steps, n], None)?;
            let mut clock = WeatherGlobalState::default();
            clock.time_steps_in_hour = i32::try_from(steps)?;
            let mut environment = WeatherEnvironmentState::default();
            let mut errors = cold_errors();
            let mut bytes = Vec::new();
            let mut console = Vec::new();
            let mut native_identity = None;
            let source_identity = source_sky_identity(&owner, &geometry)?;
            let phase_inputs = array(&case["steps"])?;
            let calls = array(&oracle["calls"])?;
            if phase_inputs.len() != calls.len() {
                return Err("retained solar call extent".into());
            }
            for (step, call) in phase_inputs.iter().zip(calls) {
                if step != &call["primary_step"] {
                    return Err("whole solar step differs".into());
                }
                clock.hour_of_day = i32::try_from(step["hour"].as_i64().ok_or("hour")?)?;
                clock.time_step = i32::try_from(step["timestep"].as_i64().ok_or("timestep")?)?;
                let hour = usize::try_from(clock.hour_of_day)?;
                let ts = usize::try_from(clock.time_step)?;
                if hour == 0 || hour > 24 || ts == 0 || ts > steps {
                    return Err("real clock domain".into());
                }
                let direction = array(&step["SOLCOS"])?;
                if direction.len() != 3 {
                    return Err("direction extent".into());
                }
                environment.current_weather.solar_direction_cosines = [
                    real(&direction[0])?,
                    real(&direction[1])?,
                    real(&direction[2])?,
                ];
                environment.elevation = real(&step["elevation"])?;
                environment.current_weather.beam_solar_rad = real(&step["beam"])?;
                environment.current_weather.dif_solar_rad = real(&step["diffuse"])?;
                let detailed = step["detailed_sky"].as_bool().ok_or("detailed flag")?;
                let varying = step["varying_transmittance"]
                    .as_bool()
                    .ok_or("varying flag")?;
                control.solar_distribution = if step["minimal_distribution"]
                    .as_bool()
                    .ok_or("distribution")?
                {
                    SelectedSolarDistribution::Minimal
                } else {
                    SelectedSolarDistribution::FullExterior
                };
                let surface_inputs = array(&step["surface_inputs"])?;
                if surface_inputs.len() != n {
                    return Err("step surface inputs extent".into());
                }
                for (i, p) in surface_inputs.iter().enumerate() {
                    geometry
                        .final_surfaces
                        .as_mut()
                        .ok_or("same Surface rows")?[i]
                        .row
                        .exterior_solar = p["solar_exposed"].as_bool().ok_or("exposure")?;
                    owner
                        .solar_timestep
                        .heat_balance
                        .surf_cos_inc_ang
                        .set([hour, ts, i + 1], real(&p["incidence"])?)?;
                    owner
                        .solar_timestep
                        .heat_balance
                        .surf_sunlit_frac
                        .set([hour, ts, i + 1], real(&p["sunlit"])?)?;
                    owner
                        .solar_timestep
                        .shading
                        .surf_dif_shdg_ratio_iso_sky
                        .set([i + 1], real(&p["isotropic_ratio"])?)?;
                    owner
                        .solar_timestep
                        .shading
                        .surf_dif_shdg_ratio_horiz
                        .set([i + 1], real(&p["horizon_ratio"])?)?;
                    owner
                        .solar_timestep
                        .shading
                        .surf_dif_shdg_ratio_iso_sky_hrts
                        .set([ts, hour, i + 1], real(&p["hrts_isotropic_ratio"])?)?;
                    owner
                        .solar_timestep
                        .shading
                        .surf_dif_shdg_ratio_horiz_hrts
                        .set([ts, hour, i + 1], real(&p["hrts_horizon_ratio"])?)?;
                }
                for k in ["before", "after_anisotropic", "after_opaque"] {
                    let e = &call[k];
                    let ids = native_sky_identity(e)?;
                    if let Some(old) = &native_identity {
                        if old != &ids {
                            return Err("Native repeated Solar identity differs".into());
                        }
                    } else {
                        native_identity = Some(ids);
                    }
                }
                owner.assert_native_gap_sky(
                    &geometry,
                    &control,
                    &clock,
                    &environment,
                    detailed,
                    varying,
                    &errors,
                    &bytes,
                    &call["before"],
                    &source_identity,
                )?;
                owner.source_anisotropic_sky_view_factors(
                    &geometry,
                    &control,
                    &clock,
                    &environment,
                    detailed,
                    varying,
                    &mut backend(&mut errors, &mut bytes, &mut console),
                )?;
                owner.assert_native_gap_sky(
                    &geometry,
                    &control,
                    &clock,
                    &environment,
                    detailed,
                    varying,
                    &errors,
                    &bytes,
                    &call["after_anisotropic"],
                    &source_identity,
                )?;
                owner.source_calc_absorbed_on_exterior_opaque(
                    1,
                    &mut geometry,
                    &zones,
                    &control,
                    &clock,
                )?;
                owner.assert_native_gap_sky(
                    &geometry,
                    &control,
                    &clock,
                    &environment,
                    detailed,
                    varying,
                    &errors,
                    &bytes,
                    &call["after_opaque"],
                    &source_identity,
                )?;
                count += 1;
            }
        }
        Ok(count)
    }
}
