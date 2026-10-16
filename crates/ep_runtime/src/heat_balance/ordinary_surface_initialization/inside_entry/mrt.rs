//! Literal CalculateZoneMRT5583-5698 on sole Surface/Zone/Space/enclosure owners.
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance;
use crate::heat_balance::ordinary_surface_initialization::OrdinarySurfaceInitialization;
use crate::heat_balance::ordinary_surface_initialization::inside_ctf::value;
use crate::heat_balance::ordinary_surface_initialization::prefix::index;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
impl OrdinarySurfaceInitialization {
    pub(crate) fn calculate_source_zone_mrt(
        &mut self,
        zone_to_resimulate: Option<i32>,
        geometry: &SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        control: &SelectedHeatBalanceInputState,
        air: &mut SourceAirHeatBalance,
        radiation: &mut OrdinarySolarRadiation,
        globals: &SelectedHeatBalanceGlobals,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let enclosures = radiation.source_radiant_enclosures_mut();
        if self.calculate_zone_mrt_first_time {
            if self.arrays.surface_ae.dimensions.is_some() {
                return Err(Error::Unavailable(
                    "native MRT SurfaceAE5596 requires unallocated owner",
                ));
            }
            let surfaces = usize::try_from(geometry.total_surfaces)
                .map_err(|_| Error::Unavailable("actual MRT TotSurfaces"))?;
            self.arrays.surface_ae.dimension([surfaces], None)?;
            if self.arrays.zone_ae_sum.dimensions.is_some() {
                return Err(Error::Unavailable(
                    "native MRT ZoneAESum5597 requires unallocated owner",
                ));
            }
            let zone_count = usize::try_from(globals.num_of_zones)
                .map_err(|_| Error::Unavailable("actual MRT NumOfZones"))?;
            self.arrays.zone_ae_sum.dimension([zone_count], None)?;
            self.arrays.surface_ae.fill(0.0)?;
            self.arrays.zone_ae_sum.fill(0.0)?;
            for enclosure in enclosures.iter_mut() {
                enclosure.sum_ae = 0.0;
            }
            for number in 1..=geometry.total_surfaces {
                let row = &geometry
                    .final_surfaces
                    .as_ref()
                    .and_then(|v| v.get(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual MRT final Surface owner"))?
                    .row;
                if row.heat_transfer_surface {
                    let emissivity = control
                        .construction_usage
                        .get(index(row.construction)?)
                        .ok_or(Error::Unavailable("actual MRT Surface.Construction owner"))?
                        .inside_absorp_thermal;
                    self.arrays
                        .surface_ae
                        .set([index(number)? + 1], row.area * emissivity)?;
                    let zone = row.zone;
                    if zone > 0 {
                        let at = index(zone)? + 1;
                        let sum = self.arrays.zone_ae_sum.read([at])?
                            + self.arrays.surface_ae.read([index(number)? + 1])?;
                        self.arrays.zone_ae_sum.set([at], sum)?;
                    }
                    if row.rad_encl_index > 0 {
                        enclosures
                            .get_mut(index(row.rad_encl_index)?)
                            .ok_or(Error::Unavailable("actual MRT Surface RadEnclIndex"))?
                            .sum_ae += self.arrays.surface_ae.read([index(number)? + 1])?;
                    }
                }
            }
        }
        if let Some(zone) = zone_to_resimulate {
            let row = zones
                .zones
                .as_ref()
                .and_then(|v| v.get(index(zone).ok()?))
                .ok_or(Error::Unavailable("actual MRT resimulated Zone owner"))?;
            for &space_number in &row.space_indexes {
                let space = zones
                    .spaces
                    .as_ref()
                    .and_then(|v| v.get(index(space_number).ok()?))
                    .ok_or(Error::Unavailable("actual MRT Zone Space owner"))?;
                let enclosure = enclosures
                    .get_mut(index(space.radiant_enclosure_num)?)
                    .ok_or(Error::Unavailable("actual MRT Space radiantEnclosureNum"))?;
                enclosure.sum_aet = 0.0;
                enclosure.recalculate_mrt = true;
            }
        } else {
            for enclosure in enclosures.iter_mut() {
                enclosure.recalculate_mrt = true;
            }
        }
        for zone in 1..=globals.num_of_zones {
            if zone_to_resimulate.is_some_and(|selected| zone != selected) {
                continue;
            }
            let air_zone = air
                .zones
                .as_mut()
                .and_then(|v| v.get_mut(index(zone).ok()?))
                .ok_or(Error::Unavailable("same actual zoneHeatBalance MRT owner"))?;
            if self.arrays.zone_ae_sum.read([index(zone)? + 1])? > 0.01 {
                let mut zone_sum_aet = 0.0;
                let zone_row = zones
                    .zones
                    .as_ref()
                    .and_then(|v| v.get(index(zone).ok()?))
                    .ok_or(Error::Unavailable("actual MRT Zone spaceIndexes"))?;
                for &space_number in &zone_row.space_indexes {
                    let space = zones
                        .spaces
                        .as_ref()
                        .and_then(|v| v.get(index(space_number).ok()?))
                        .ok_or(Error::Unavailable("actual MRT Space HT range"))?;
                    for number in space.ht_surface_first..=space.ht_surface_last {
                        let surf_aet = self.arrays.surface_ae.read([index(number)? + 1])?
                            * value(&self.arrays.surf_temp_in, index(number)?)?;
                        zone_sum_aet += surf_aet;
                        let enclosure_number = geometry
                            .final_surfaces
                            .as_ref()
                            .and_then(|v| v.get(index(number).ok()?))
                            .ok_or(Error::Unavailable("actual MRT Surface RadEnclIndex owner"))?
                            .row
                            .rad_encl_index;
                        enclosures
                            .get_mut(index(enclosure_number)?)
                            .ok_or(Error::Unavailable("actual MRT zone enclosure sumAET"))?
                            .sum_aet += surf_aet;
                    }
                }
                air_zone.air.mrt =
                    zone_sum_aet / self.arrays.zone_ae_sum.read([index(zone)? + 1])?;
            } else {
                if self.calculate_zone_mrt_first_time {
                    let name = &zones
                        .zones
                        .as_ref()
                        .and_then(|v| v.get(index(zone).ok()?))
                        .ok_or(Error::Unavailable("actual MRT Zone warning name"))?
                        .name;
                    backend.show_warning_error(&format!("Zone areas*inside surface emissivities are summing to zero, for Zone=\"{name}\""))?;
                    backend
                        .show_continue_error("As a result, MRT will be set to MAT for that zone")?;
                }
                air_zone.air.mrt = air_zone.air.mat;
            }
        }
        for enclosure in enclosures {
            if !enclosure.recalculate_mrt {
                continue;
            }
            if enclosure.sum_ae > 0.01 {
                enclosure.sum_aet = 0.0;
                let surface_numbers =
                    enclosure
                        .source_surface_ptr
                        .as_ref()
                        .ok_or(Error::Unavailable(
                            "actual MRT enclosure SurfacePtr allocation",
                        ))?;
                for &number in surface_numbers {
                    let surf_aet = self.arrays.surface_ae.read([index(number)? + 1])?
                        * value(&self.arrays.surf_temp_in, index(number)?)?;
                    enclosure.sum_aet += surf_aet;
                }
                enclosure.mean_radiant_temperature = enclosure.sum_aet / enclosure.sum_ae;
            } else {
                if self.calculate_zone_mrt_first_time {
                    backend.show_warning_error(&format!("Enclosure areas*inside surface emissivities are summing to zero, for Enclosure=\"{}\"",enclosure.name))?;
                    backend.show_continue_error("As a result, MRT will be set to the volume weighted average MAT for that enclosure")?;
                }
                let mut sum_mat_vol = 0.0;
                let mut sum_vol = 0.0;
                let mut sum_mat = 0.0;
                for &space_number in &enclosure.space_nums {
                    let volume = zones
                        .spaces
                        .as_ref()
                        .and_then(|v| v.get(index(space_number).ok()?))
                        .ok_or(Error::Unavailable("actual MRT fallback Space.Volume"))?
                        .volume;
                    let mat = air
                        .spaces
                        .as_ref()
                        .and_then(|v| v.get(index(space_number).ok()?))
                        .ok_or(Error::Unavailable("same actual spaceHeatBalance MAT owner"))?
                        .air
                        .mat;
                    sum_vol += volume;
                    sum_mat_vol += mat * volume;
                    sum_mat += mat;
                }
                if sum_vol > 0.01 {
                    enclosure.mean_radiant_temperature = sum_mat_vol / sum_vol;
                } else {
                    let size = i32::try_from(enclosure.space_nums.len()).map_err(|_| {
                        Error::Unavailable("actual MRT native Space vector signed size")
                    })?;
                    enclosure.mean_radiant_temperature = sum_mat / f64::from(size);
                }
            }
            for &space_number in &enclosure.space_nums {
                air.spaces
                    .as_mut()
                    .and_then(|v| v.get_mut(index(space_number).ok()?))
                    .ok_or(Error::Unavailable("same actual spaceHeatBalance MRT owner"))?
                    .air
                    .mrt = enclosure.mean_radiant_temperature;
            }
        }
        self.calculate_zone_mrt_first_time = false;
        Ok(())
    }
}
