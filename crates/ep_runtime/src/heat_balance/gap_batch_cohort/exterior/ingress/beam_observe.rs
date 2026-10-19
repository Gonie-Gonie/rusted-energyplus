//! Passive comparison of real Source beam owners against retained Native states.
use super::*;
use hb::radiation::ordinary::solar_timestep::array::SourceSolarArray;

pub(super) fn same<T: std::fmt::Debug + PartialEq>(a: T, b: T, label: &str) -> Result {
    if a != b {
        return Err(format!("{label}: actual Source {a:?} != Native {b:?}").into());
    }
    Ok(())
}
pub(super) fn bits(a: f64, e: &Value, label: &str) -> Result {
    if !a.is_finite() {
        return Err(format!("{label}: actual Source nonfinite").into());
    }
    same(
        format!("{:016x}", a.to_bits()),
        text(member(e, "value_bits")?)?.to_owned(),
        label,
    )
}
pub(super) fn vector(a: &[f64], e: &Value, label: &str) -> Result {
    let e = array(e)?;
    same(a.len(), e.len(), label)?;
    for (a, e) in a.iter().zip(e) {
        bits(*a, e, label)?;
    }
    Ok(())
}
pub(super) fn address(v: &Value) -> Result<u64> {
    let address = v.as_u64().ok_or("actual Native unsigned address")?;
    if address == 0 {
        return Err(invalid("actual Native nonnull address").into());
    }
    Ok(address)
}
fn solar_array(a: &SourceSolarArray<f64, 1>, e: &Value, label: &str, defined: bool) -> Result {
    same(
        e["allocated"].as_bool(),
        Some(a.dimensions.is_some()),
        label,
    )?;
    let n = a.dimensions.map_or(0, |v| v[0]);
    same(count(member(e, "size")?)?, n, label)?;
    same(int(member(e, "lower")?)?, 1, label)?;
    same(int(member(e, "upper")?)?, i32::try_from(n)?, label)?;
    if defined {
        same(e["values_defined"].as_bool(), Some(true), label)?;
        let expected = array(member(e, "values")?)?;
        same(expected.len(), a.values.len(), label)?;
        for (a, e) in a.values.iter().zip(expected) {
            bits(a.ok_or("actual Source array cell unwritten")?, e, label)?;
        }
    }
    if n > 0 && e.get("data_address").is_some() {
        address(member(e, "data_address")?)?;
    }
    Ok(())
}
fn integer_array(a: &Option<Vec<i32>>, e: &Value, label: &str) -> Result {
    same(e["allocated"].as_bool(), Some(a.is_some()), label)?;
    let n = a.as_ref().map_or(0, Vec::len);
    same(count(member(e, "size")?)?, n, label)?;
    let lower = if a.is_some() { 0 } else { 1 };
    let upper = if a.is_some() {
        i32::try_from(n)? - 1
    } else {
        0
    };
    same(int(member(e, "lower")?)?, lower, label)?;
    same(int(member(e, "upper")?)?, upper, label)?;
    let values = array(member(e, "values")?)?;
    same(values.len(), n, label)?;
    for (a, e) in a.as_deref().unwrap_or_default().iter().zip(values) {
        same(*a, int(e)?, label)?;
    }
    Ok(())
}
pub(crate) struct BeamIdentity {
    native: Vec<u64>,
    source: Vec<usize>,
}
impl GainFactory {
    pub(super) fn assert_diagnostics(&self, e: &Value) -> Result {
        same(
            self.error_state.tracking.total_warning_errors,
            int(member(e, "warnings")?)?,
            "actual warning count",
        )?;
        same(
            self.error_state.tracking.total_severe_errors,
            int(member(e, "severe")?)?,
            "actual severe count",
        )?;
        same(
            self.reports.source_recurring_errors.number,
            int(member(e, "recurring")?)?,
            "actual recurring count",
        )?;
        same(
            std::str::from_utf8(&self.errors)?,
            text(member(e, "text")?)?,
            "entire actual error text",
        )
    }
    fn beam_identity(&self, e: &Value) -> Result<BeamIdentity> {
        let mut native = vec![
            address(member(e, "state_address")?)?,
            address(member(e, "module_address")?)?,
        ];
        let rows = self
            .geometry
            .final_surfaces
            .as_ref()
            .ok_or("actual physical Surface allocation")?;
        let mut source = vec![
            &self.reports as *const _ as usize,
            &self.radiation as *const _ as usize,
            &self.geometry as *const _ as usize,
            rows.as_ptr() as usize,
        ];
        let expected = array(member(e, "surfaces")?)?;
        same(
            rows.len(),
            expected.len(),
            "physical Surface row identity count",
        )?;
        for (r, n) in rows.iter().zip(expected) {
            native.push(address(member(n, "address")?)?);
            source.push(r as *const _ as usize);
            source.push(&r.row as *const _ as usize);
        }
        let combinations = self
            .radiation
            .solar_timestep
            .shadow_combinations
            .as_ref()
            .ok_or("actual ShadowComb allocation")?;
        let expected = array(member(e, "shadow_combinations")?)?;
        same(
            combinations.len(),
            expected.len(),
            "actual ShadowComb identity shape",
        )?;
        source.push(combinations.as_ptr() as usize);
        for (r, n) in combinations.iter().zip(expected) {
            native.push(address(member(n, "address")?)?);
            source.push(r as *const _ as usize);
        }
        let shading = &self.radiation.solar_timestep.shading;
        for (key, values) in [
            ("SurfSunCosTheta_all", &shading.surf_sun_cos_theta),
            ("SurfSunlitArea_all", &shading.surf_sunlit_area),
            ("SurfIntAbsFac", &shading.surf_int_abs_fac),
        ] {
            if values.values.is_empty() {
                return Err(invalid("actual nonempty solar buffer").into());
            }
            native.push(address(member(member(e, key)?, "data_address")?)?);
            source.push(values.values.as_ptr() as usize);
        }
        if source.contains(&0) {
            return Err(invalid("actual Source nonnull owner/storage identity").into());
        }
        Ok(BeamIdentity { native, source })
    }
    pub(crate) fn assert_beam(&self, e: &Value, retained: &mut Option<BeamIdentity>) -> Result {
        let current = self.beam_identity(e)?;
        if let Some(old) = retained {
            same(
                &current.native,
                &old.native,
                "Native retained owner/row/buffer identities",
            )?;
            same(
                &current.source,
                &old.source,
                "Source retained owner/row/buffer identities",
            )?;
        } else {
            *retained = Some(current);
        }
        let s = &self.radiation.solar_timestep;
        let clock = &self.weather.global;
        let globals = self
            .reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or("same globals")?;
        let shadowing = self
            .reports
            .shadowing_input
            .as_ref()
            .ok_or("same shadowing input")?;
        for (key, value) in [
            ("HourOfDay", clock.hour_of_day),
            ("TimeStep", clock.time_step),
            ("NumOfZones", globals.num_of_zones),
            ("numSpaces", globals.num_spaces),
            ("TotSurfaces", self.geometry.total_surfaces),
            (
                "MaxVerticesPerSurface",
                i32::try_from(self.geometry.coordinate.max_vertices_per_surface)?,
            ),
            (
                "SolarDistribution",
                self.reports.heat_balance_input_state.solar_distribution as i32,
            ),
        ] {
            same(value, int(member(e, key)?)?, key)?;
        }
        for (key, value) in [
            ("BeginSimFlag", clock.begin_sim_flag),
            ("BeginEnvrnFlag", clock.begin_envrn_flag),
            ("GetInputFlag", s.get_input_flag),
            ("firstTime", s.first_time),
            ("ShadowOneTimeFlag", s.shadow_one_time_flag),
            (
                "ShadingTransmittanceVaries",
                self.geometry.shading_transmittance_varies,
            ),
            (
                "DetailedSkyDiffuseAlgorithm",
                shadowing.detailed_sky_diffuse_algorithm,
            ),
            (
                "DetailedSolarTimestepIntegration",
                shadowing.detailed_solar_timestep_integration,
            ),
        ] {
            same(value, boolean(member(e, key)?)?, key)?;
        }
        let expected = array(member(e, "SUNCOS")?)?;
        same(expected.len(), 3, "actual SUNCOS shape")?;
        for (value, n) in s.sun_cosines.iter().zip(expected) {
            bits(
                value.ok_or("actual SUNCOS cell unwritten")?,
                n,
                "actual SUNCOS",
            )?;
        }
        let hour = usize::try_from(clock.hour_of_day)?;
        let step = usize::try_from(clock.time_step)?;
        let rows = self
            .geometry
            .final_surfaces
            .as_ref()
            .ok_or("actual physical rows")?;
        for (i, (surface, n)) in rows.iter().zip(array(member(e, "surfaces")?)?).enumerate() {
            let number = i + 1;
            let row = &surface.row;
            let props = row
                .vertex_progress
                .properties
                .ok_or("actual GetVertices property producer")?;
            same(
                count(member(n, "ordinal")?)?,
                number,
                "actual Native Surface ordinal",
            )?;
            same(
                row.name.as_str(),
                text(member(n, "Name")?)?,
                "actual Surface name",
            )?;
            for (key, value) in [
                ("Class", row.class as i32),
                ("Construction", row.construction),
                ("Zone", row.zone),
                ("BaseSurf", row.base_surface),
                ("Sides", row.sides),
            ] {
                same(value, int(member(n, key)?)?, key)?;
            }
            same(
                row.exterior_solar,
                boolean(member(n, "ExtSolar")?)?,
                "actual ExtSolar",
            )?;
            same(
                row.is_shadowing,
                boolean(member(n, "IsShadowing")?)?,
                "actual IsShadowing",
            )?;
            vector(
                &props.out_norm,
                member(n, "OutNormVec")?,
                "actual OutNormVec",
            )?;
            bits(row.area, member(n, "Area")?, "actual Area")?;
            bits(
                row.net_area_shadow_calc,
                member(n, "NetAreaShadowCalc")?,
                "actual NetAreaShadowCalc",
            )?;
            let vertices = row.vertex.as_ref().ok_or("actual written Vertex owner")?;
            let expected = array(member(n, "vertices")?)?;
            same(vertices.len(), expected.len(), "actual Vertex shape")?;
            for (point, expected) in vertices.iter().zip(expected) {
                vector(
                    &point.ok_or("actual Vertex unwritten")?,
                    expected,
                    "actual Vertex",
                )?;
            }
            for (key, value) in [
                (
                    "SurfSunCosTheta",
                    s.shading.surf_sun_cos_theta.read([number])?,
                ),
                ("SurfSunlitArea", s.shading.surf_sunlit_area.read([number])?),
                (
                    "current_SurfCosIncAngHR",
                    s.heat_balance.surf_cos_inc_ang_hr.read([hour, number])?,
                ),
                (
                    "current_SurfCosIncAng",
                    s.heat_balance.surf_cos_inc_ang.read([hour, step, number])?,
                ),
                (
                    "current_SurfSunlitFracHR",
                    s.heat_balance.surf_sunlit_frac_hr.read([hour, number])?,
                ),
                (
                    "current_SurfSunlitFrac",
                    s.heat_balance.surf_sunlit_frac.read([hour, step, number])?,
                ),
            ] {
                bits(value, member(n, key)?, key)?;
            }
            for (key, array) in [
                (
                    "isotropic_ratio_HRTS",
                    &s.shading.surf_dif_shdg_ratio_iso_sky_hrts,
                ),
                (
                    "horizon_ratio_HRTS",
                    &s.shading.surf_dif_shdg_ratio_horiz_hrts,
                ),
            ] {
                let expected = member(n, key)?;
                if array.dimensions.is_some() {
                    bits(array.read([step, hour, number])?, expected, key)?;
                } else {
                    same(expected.is_null(), true, key)?;
                }
            }
        }
        let combinations = s
            .shadow_combinations
            .as_ref()
            .ok_or("actual ShadowComb owner")?;
        for (i, (row, n)) in combinations
            .iter()
            .zip(array(member(e, "shadow_combinations")?)?)
            .enumerate()
        {
            same(
                count(member(n, "ordinal")?)?,
                i + 1,
                "actual ShadowComb ordinal",
            )?;
            same(
                row.use_this_surface,
                boolean(member(n, "UseThisSurf")?)?,
                "actual UseThisSurf",
            )?;
            for (key, value) in [
                ("NumGenSurf", row.num_gen_surface),
                ("NumBackSurf", row.num_back_surface),
                ("NumSubSurf", row.num_sub_surface),
            ] {
                same(value, int(member(n, key)?)?, key)?;
            }
            integer_array(&row.general, member(n, "GenSurf")?, "actual GenSurf")?;
            integer_array(&row.back, member(n, "BackSurf")?, "actual BackSurf")?;
            integer_array(&row.sub, member(n, "SubSurf")?, "actual SubSurf")?;
        }
        solar_array(
            &s.shading.surf_sun_cos_theta,
            member(e, "SurfSunCosTheta_all")?,
            "actual full SurfSunCosTheta",
            true,
        )?;
        solar_array(
            &s.shading.surf_sunlit_area,
            member(e, "SurfSunlitArea_all")?,
            "actual full SurfSunlitArea",
            true,
        )?;
        solar_array(
            &s.shading.surf_int_abs_fac,
            member(e, "SurfIntAbsFac")?,
            "actual full SurfIntAbsFac",
            true,
        )?;
        for (key, value) in [("XVrt", &s.x_vrt), ("YVrt", &s.y_vrt), ("ZVrt", &s.z_vrt)] {
            solar_array(
                value,
                member(member(e, "actual_shadow_scratch")?, key)?,
                key,
                false,
            )?;
        }
        self.assert_diagnostics(member(e, "diagnostics")?)?;
        Ok(())
    }
}
