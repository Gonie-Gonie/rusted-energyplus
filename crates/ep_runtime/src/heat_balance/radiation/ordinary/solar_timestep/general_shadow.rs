//! Actual general-caster candidate construction and CHKGSS on physical rows.
use super::*;

fn vertex(
    geometry: &SurfaceGeometryInputState,
    number: usize,
    at: usize,
) -> Result<[f64; 3], Error> {
    source_surface(geometry, number)?
        .vertex
        .as_ref()
        .and_then(|v| v.get(at))
        .copied()
        .flatten()
        .ok_or(Error::Unavailable("CHKGSS actual allocated written Vertex"))
}
fn minus(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
impl OrdinarySolarRadiation {
    /// SolarShading2851-2943 CHKGSS. True is the genuine CannotShade return.
    fn source_check_general_shadow_surface(
        &self,
        geometry: &SurfaceGeometryInputState,
        receiving: usize,
        casting: usize,
        z_min: f64,
    ) -> Result<bool, Error> {
        if geometry_properties(geometry, casting)?.out_norm[2] > 0.9999 {
            return Ok(true);
        }
        let casting_count = ordinal(source_surface(geometry, casting)?.sides)?;
        let mut z_max = vertex(geometry, casting, 0)?[2];
        for i in 1..casting_count {
            let z = vertex(geometry, casting, i)?[2];
            if z_max < z {
                z_max = z;
            }
        }
        if z_max <= z_min {
            return Ok(true);
        }
        let receiving_second = vertex(geometry, receiving, 1)?;
        let a = minus(vertex(geometry, receiving, 0)?, receiving_second);
        let b = minus(vertex(geometry, receiving, 2)?, receiving_second);
        let c = cross(b, a);
        let mut product = 0.0;
        for i in 0..casting_count {
            product = dot(c, minus(vertex(geometry, casting, i)?, receiving_second));
            if product > self.solar_timestep.tol_value {
                break;
            }
        }
        if product > self.solar_timestep.tol_value {
            let casting_second = vertex(geometry, casting, 1)?;
            let a = minus(vertex(geometry, casting, 0)?, casting_second);
            let b = minus(vertex(geometry, casting, 2)?, casting_second);
            let c = cross(b, a);
            let receiving_count = ordinal(source_surface(geometry, receiving)?.sides)?;
            for i in 0..receiving_count {
                product = dot(c, minus(vertex(geometry, receiving, i)?, casting_second));
                if product > self.solar_timestep.tol_value {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /// The real5398-5516 candidate loop. General storage belongs to the one
    /// Determine call and is reused across receiving surfaces, like Native GSS.
    pub(super) fn source_general_shadow_candidates(
        &mut self,
        geometry: &SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        receiving: usize,
        receiving_is_shadowing: bool,
        z_min: f64,
        general: &mut Vec<i32>,
    ) -> Result<usize, Error> {
        let count = surface_count(geometry)?;
        let receiving_number = i32::try_from(receiving)
            .map_err(|_| Error::Unavailable("actual signed receiving ordinal"))?;
        let mut found = 0_usize;
        for candidate in 1..=count {
            let row = source_surface(geometry, candidate)?;
            if candidate == receiving {
                continue;
            }
            if row.heat_transfer_surface && row.base_surface == receiving_number {
                continue;
            }
            let include = if control.solar_distribution == SelectedSolarDistribution::Minimal {
                row.base_surface == receiving_number
            } else {
                if receiving_is_shadowing
                    && ((candidate == receiving + 1 && row.mirrored_surface)
                        || (candidate + 1 == receiving
                            && source_surface(geometry, receiving)?.mirrored_surface))
                {
                    continue;
                }
                if row.base_surface == receiving_number {
                    true
                } else if row.base_surface == 0
                    || (row.base_surface
                        == i32::try_from(candidate)
                            .map_err(|_| Error::Unavailable("actual signed casting ordinal"))?
                        && (row.exterior_boundary == 0 || row.exterior_boundary == -4))
                {
                    !self.source_check_general_shadow_surface(
                        geometry, receiving, candidate, z_min,
                    )?
                } else {
                    false
                }
            };
            if include {
                found = found
                    .checked_add(1)
                    .ok_or(Error::Unavailable("actual general-caster signed count"))?;
                if found > ordinal(self.solar_timestep.max_gss)? {
                    self.solar_timestep.max_gss = self
                        .solar_timestep
                        .max_gss
                        .checked_mul(2)
                        .ok_or(Error::Unavailable("MaxGSS signed doubling"))?;
                    general.resize(ordinal(self.solar_timestep.max_gss)?, 0);
                }
                *general
                    .get_mut(found - 1)
                    .ok_or(Error::Unavailable("actual allocated GSS cell"))? =
                    i32::try_from(candidate)
                        .map_err(|_| Error::Unavailable("actual signed casting ordinal"))?;
            }
        }
        Ok(found)
    }
}
