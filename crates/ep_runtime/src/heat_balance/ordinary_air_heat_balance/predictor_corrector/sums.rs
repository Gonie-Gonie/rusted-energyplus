//! Actual zone-only opaque calcZoneOrSpaceSums and virtual Space→Zone HAT sum.
use super::*;
#[derive(Default)]
struct SumHat {
    gain: f64,
    ha: f64,
    hat_surface: f64,
    hat_reference: f64,
}
impl SourceZoneSpaceHeatBalanceData {
    pub(super) fn calculate_source_zone_sums(
        &mut self,
        corrector: bool,
        number: i32,
        controller: &SourceZoneAirPredictorCorrector,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.sum_ha = 0.0;
        self.sum_ha_tsurf = 0.0;
        self.sum_ha_tref = 0.0;
        self.sum_sys_m_cp = 0.0;
        self.sum_sys_m_cp_t = 0.0;
        self.sum_int_gain =
            owners
                .gains
                .source_zone_convective_gain(owners.zones, number, false)?;
        self.sum_int_gain += value(&owners.fan.sum_conv_ht_rad_sys, index(number)?)?
            + value(&owners.fan.sum_conv_pool, index(number)?)?;
        if zone(owners, number)?.no_heat_to_return_air {
            self.sum_int_gain +=
                owners
                    .gains
                    .source_zone_convective_gain(owners.zones, number, true)?;
        }
        self.sum_m_cp = self.mcpi + self.mcpv + self.mcpm + self.mcpe + self.mcpc + self.m_dot_cpoa;
        self.sum_m_cp_t = self.mcpti
            + self.mcptv
            + self.mcptm
            + self.mcpte
            + self.mcptc
            + self.m_dot_cpoa * zone(owners, number)?.dry_bulb;
        if controller
            .subsystems
            .airflow_network
            .replaces_simple_multizone()
        {
            return Err(Error::Unavailable("positive AFN exchangeData sums5192"));
        }
        if corrector {
            let z = zone(owners, number)?;
            if z.is_controlled {
                return Err(Error::Unavailable(
                    "positive controlled Zone supply sums5203",
                ));
            } else if z.is_return_plenum {
                return Err(Error::Unavailable("positive return plenum sums5215"));
            } else if z.is_supply_plenum {
                return Err(Error::Unavailable("positive supply plenum sums5239"));
            }
            let multiplier =
                z.multiplier
                    .checked_mul(z.list_multiplier)
                    .ok_or(Error::Unavailable(
                        "defined Source int Zone multiplier product5246",
                    ))?;
            if !z.leakage_parallel_piu_numbers.is_empty() {
                return Err(Error::Unavailable("positive parallel PIU leakage5249"));
            }
            self.sum_sys_m_cp /= f64::from(multiplier);
            self.sum_sys_m_cp_t /= f64::from(multiplier);
        }
        let mut total = SumHat::default();
        for &space_number in &zone(owners, number)?.space_indexes {
            let mut results = SumHat::default();
            let space = owners
                .zones
                .spaces
                .as_deref()
                .and_then(|rows| rows.get(index(space_number).ok()?))
                .ok_or(Error::Unavailable("actual HAT Space ordinal"))?;
            for surface_number in space.ht_surface_first..=space.ht_surface_last {
                let i = index(surface_number)?;
                let mut ha = 0.0;
                let surface = owners
                    .geometry
                    .final_surfaces
                    .as_deref()
                    .and_then(|rows| rows.get(i))
                    .ok_or(Error::Unavailable("actual HAT Surface"))?;
                let area = surface.row.area;
                if surface.row.class == SurfaceClass::Window {
                    return Err(Error::Unavailable("positive Window HAT shading5314"));
                }
                ha += value(&owners.surface_arrays.surf_h_conv_int, i)? * area;
                results.hat_surface += value(&owners.surface_arrays.surf_h_conv_int, i)?
                    * area
                    * value(&owners.surface_arrays.surf_temp_in_tmp, i)?;
                let air_reference = *owners
                    .geometry
                    .parallel_arrays
                    .as_ref()
                    .and_then(|a| a.surf_t_air_ref.get(i))
                    .ok_or(Error::Unavailable("actual SurfTAirRef5380"))?;
                match air_reference {
                    0 => results.ha += ha,
                    1 => {
                        results.hat_reference +=
                            ha * value(&owners.surface_arrays.surf_temp_eff_bulk_air, i)?
                    }
                    2 => {
                        if !zone(owners, number)?.is_controlled {
                            let message = format!(
                                "Zones must be controlled for Ceiling-Diffuser Convection model. No system serves zone {}",
                                zone(owners, number)?.name
                            );
                            return match owners.backend.show_fatal_error(&message) {
                                Err(e) => Err(Error::Fatal(e)),
                                Ok(never) => match never {},
                            };
                        }
                        if self.sum_sys_m_cp > 0.0 {
                            results.hat_reference += ha * self.sum_sys_m_cp_t / self.sum_sys_m_cp;
                        } else {
                            results.ha += ha;
                        }
                    }
                    _ => results.ha += ha,
                }
            }
            total.gain += results.gain;
            total.ha += results.ha;
            total.hat_surface += results.hat_surface;
            total.hat_reference += results.hat_reference;
        }
        self.sum_int_gain += total.gain;
        self.sum_ha = total.ha;
        self.sum_ha_tsurf = total.hat_surface;
        self.sum_ha_tref = total.hat_reference;
        Ok(())
    }
}
