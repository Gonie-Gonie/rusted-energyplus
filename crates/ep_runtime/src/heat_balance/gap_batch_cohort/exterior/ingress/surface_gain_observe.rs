//! Full observed positive opaque enclosure/surface outputs, never restored.
use super::beam_observe::{address, bits, same, vector};
use super::gains_observe::GainIdentity;
use super::*;

fn array_values(v: Option<&[f64]>, e: &Value, label: &str) -> Result {
    same(e["allocated"].as_bool(), Some(v.is_some()), label)?;
    let values = v.unwrap_or_default();
    same(count(member(e, "size")?)?, values.len(), label)?;
    same(int(member(e, "lower")?)?, 1, label)?;
    same(
        int(member(e, "upper")?)?,
        i32::try_from(values.len())?,
        label,
    )?;
    same(e["values_defined"].as_bool(), Some(true), label)?;
    vector(values, member(e, "values")?, label)
}
fn matrix(v: Option<&[f64]>, n: usize, e: &Value, label: &str) -> Result {
    same(e["allocated"].as_bool(), Some(v.is_some()), label)?;
    let values = v.unwrap_or_default();
    same(
        values.len(),
        n.checked_mul(n).ok_or("matrix shape overflow")?,
        label,
    )?;
    same(count(member(e, "size")?)?, values.len(), label)?;
    for key in ["lower1", "lower2"] {
        same(int(member(e, key)?)?, 1, label)?;
    }
    for key in ["upper1", "upper2"] {
        same(count(member(e, key)?)?, n, label)?;
    }
    let expected = array(member(e, "values")?)?;
    same(expected.len(), values.len(), label)?;
    // Source matrix scalar index is i*n+j; this observer's Native physical
    // Array2D stream is j-major then i. This is a storage projection only.
    for i in 0..n {
        for j in 0..n {
            bits(values[i * n + j], &expected[j * n + i], label)?;
        }
    }
    Ok(())
}
fn int_values(v: &[i32], e: &Value, label: &str) -> Result {
    let expected = array(e)?;
    same(v.len(), expected.len(), label)?;
    for (a, e) in v.iter().zip(expected) {
        same(*a, int(e)?, label)?;
    }
    Ok(())
}
fn surface_ptr(v: Option<&[i32]>, e: &Value, label: &str) -> Result {
    same(e["allocated"].as_bool(), Some(v.is_some()), label)?;
    let v = v.unwrap_or_default();
    same(count(member(e, "size")?)?, v.len(), label)?;
    same(int(member(e, "lower")?)?, 1, label)?;
    same(int(member(e, "upper")?)?, i32::try_from(v.len())?, label)?;
    int_values(v, member(e, "values")?, label)
}
pub(crate) struct SurfaceGainIdentity {
    native: Vec<u64>,
    source: Vec<usize>,
    core: Option<GainIdentity>,
}
impl GainFactory {
    pub(crate) fn retain_surface_gain_identity(
        &self,
        e: &Value,
        id: &mut Option<SurfaceGainIdentity>,
    ) -> Result {
        let mut native = Vec::new();
        let mut source = vec![
            &self.radiation as *const _ as usize,
            &self.geometry as *const _ as usize,
        ];
        for row in array(member(e, "radiant_enclosures")?)? {
            native.push(address(member(row, "address")?)?);
        }
        for row in self.radiation.radiant_enclosures() {
            source.push(row as *const _ as usize);
            for values in [&row.f, &row.script_f, &row.area, &row.emissivity] {
                let values = values.as_ref().ok_or("actual radiant defined allocation")?;
                if values.is_empty() {
                    return Err(invalid("actual positive radiant buffer").into());
                }
                source.push(values.as_ptr() as usize);
            }
        }
        for row in array(member(e, "solar_enclosures")?)? {
            native.push(address(member(row, "address")?)?);
        }
        for row in self.radiation.enclosures() {
            source.push(row as *const _ as usize);
            for values in [&row.f, &row.area, &row.absorptance] {
                if values.is_empty() {
                    return Err(invalid("actual positive solar buffer").into());
                }
                source.push(values.as_ptr() as usize);
            }
        }
        let rows = self
            .geometry
            .final_surfaces
            .as_ref()
            .ok_or("same actual Surface rows")?;
        let observed = array(member(e, "surface_rows")?)?;
        same(rows.len(), observed.len(), "actual Surface identity shape")?;
        for (row, n) in rows.iter().zip(observed) {
            source.push(row as *const _ as usize);
            source.push(&row.row as *const _ as usize);
            native.push(address(member(n, "address")?)?);
        }
        for values in [
            &self.arrays.surf_abs_thermal_int,
            &self.arrays.surf_abs_solar_int,
            &self.arrays.surf_qdot_rad_int_gains_in_per_area,
        ] {
            let values = values.as_ref().ok_or("same actual Surface scalar arrays")?;
            if values.is_empty() {
                return Err(invalid("actual nonempty Surface scalar allocation").into());
            }
            source.push(values.as_ptr() as usize);
        }
        if source.contains(&0) {
            return Err(invalid("actual Source nonnull surface-gain owner").into());
        }
        if let Some(id) = id {
            same(
                &native,
                &id.native,
                "Native persistent enclosure/Surface identities",
            )?;
            same(
                &source,
                &id.source,
                "Source persistent enclosure/Surface/buffer identities",
            )?;
            self.retain_gain_identity(e, &mut id.core)?;
        } else {
            let mut core = None;
            self.retain_gain_identity(e, &mut core)?;
            *id = Some(SurfaceGainIdentity {
                native,
                source,
                core,
            });
        }
        Ok(())
    }
    pub(crate) fn assert_surface_gains(&self, e: &Value) -> Result {
        self.assert_gains(e)?;
        let expected = array(member(e, "radiant_enclosures")?)?;
        let rows = self.radiation.radiant_enclosures();
        same(expected.len(), rows.len(), "actual radiant enclosure shape")?;
        for (row, n) in rows.iter().zip(expected) {
            same(
                row.name.as_str(),
                text(member(n, "Name")?)?,
                "actual radiant enclosure Name",
            )?;
            same(
                row.num_surfaces,
                int(member(n, "NumOfSurfaces")?)?,
                "actual radiant NumOfSurfaces",
            )?;
            int_values(
                &row.space_nums,
                member(n, "spaceNums")?,
                "actual radiant spaceNums",
            )?;
            surface_ptr(
                row.source_surface_ptr.as_deref(),
                member(n, "SurfacePtr")?,
                "actual radiant SurfacePtr",
            )?;
            array_values(
                row.area.as_deref(),
                member(n, "Area")?,
                "actual radiant Area",
            )?;
            array_values(
                row.emissivity.as_deref(),
                member(n, "Emissivity")?,
                "actual radiant Emissivity",
            )?;
            let size = usize::try_from(row.num_surfaces)?;
            matrix(row.f.as_deref(), size, member(n, "F")?, "actual radiant F")?;
            matrix(
                row.script_f.as_deref(),
                size,
                member(n, "ScriptF")?,
                "actual radiant ScriptF",
            )?;
            same(
                row.rad_recalc,
                boolean(member(n, "radReCalc")?)?,
                "actual radiant radReCalc",
            )?;
            bits(
                row.rad_therm_abs_mult,
                member(n, "radThermAbsMult")?,
                "actual radThermAbsMult",
            )?;
            bits(
                row.rad_q_thermal_rad,
                member(n, "radQThermalRad")?,
                "actual radQThermalRad",
            )?;
        }
        let expected = array(member(e, "solar_enclosures")?)?;
        let rows = self.radiation.enclosures();
        same(expected.len(), rows.len(), "actual solar enclosure shape")?;
        for (row, n) in rows.iter().zip(expected) {
            same(
                row.name.as_str(),
                text(member(n, "Name")?)?,
                "actual solar enclosure Name",
            )?;
            same(
                row.num_surfaces,
                int(member(n, "NumOfSurfaces")?)?,
                "actual solar NumOfSurfaces",
            )?;
            int_values(
                &row.space_nums,
                member(n, "spaceNums")?,
                "actual solar spaceNums",
            )?;
            surface_ptr(
                row.source_surface_ptr.as_deref(),
                member(n, "SurfacePtr")?,
                "actual solar SurfacePtr",
            )?;
            array_values(
                if row.source_array_allocated[1] {
                    Some(row.area.as_slice())
                } else {
                    None
                },
                member(n, "Area")?,
                "actual solar Area",
            )?;
            array_values(
                if row.source_array_allocated[2] {
                    Some(row.absorptance.as_slice())
                } else {
                    None
                },
                member(n, "SolAbsorptance")?,
                "actual solar SolAbsorptance",
            )?;
            matrix(
                if row.source_array_allocated[0] {
                    Some(row.f.as_slice())
                } else {
                    None
                },
                usize::try_from(row.num_surfaces)?,
                member(n, "F")?,
                "actual solar F",
            )?;
            same(
                row.rad_recalc,
                boolean(member(n, "radReCalc")?)?,
                "actual solar radReCalc",
            )?;
            same(
                row.sol_abs_first_calc,
                boolean(member(n, "solAbsFirstCalc")?)?,
                "actual solAbsFirstCalc",
            )?;
            bits(row.sol_vmult, member(n, "solVMULT")?, "actual solVMULT")?;
        }
        let expected = array(member(e, "surface_rows")?)?;
        let rows = self
            .geometry
            .final_surfaces
            .as_ref()
            .ok_or("actual Surface owner")?;
        same(rows.len(), expected.len(), "actual Surface gain shape")?;
        for (i, (row, n)) in rows.iter().zip(expected).enumerate() {
            let row = &row.row;
            same(
                count(member(n, "ordinal")?)?,
                i + 1,
                "actual Surface ordinal",
            )?;
            same(
                row.name.as_str(),
                text(member(n, "Name")?)?,
                "actual Surface Name",
            )?;
            for (key, value) in [
                ("Construction", row.construction),
                ("RadEnclIndex", row.rad_encl_index),
                ("SolarEnclIndex", row.solar_encl_index),
            ] {
                same(value, int(member(n, key)?)?, key)?;
            }
            bits(row.area, member(n, "Area")?, "actual Surface Area")?;
            for (key, values) in [
                (
                    "inside_thermal_absorptance",
                    &self.arrays.surf_abs_thermal_int,
                ),
                ("inside_solar_absorptance", &self.arrays.surf_abs_solar_int),
                (
                    "SurfQdotRadIntGainsInPerArea",
                    &self.arrays.surf_qdot_rad_int_gains_in_per_area,
                ),
            ] {
                bits(
                    *values
                        .as_ref()
                        .and_then(|v| v.get(i))
                        .ok_or("actual observed Surface scalar cell")?,
                    member(n, key)?,
                    key,
                )?;
            }
        }
        Ok(())
    }
}
