//! Ordered InitInternalHeatGains prefix, real module calls and actual sinks.
use super::devices::{sum_all_space, sum_zone_latent_gains};
use super::report::{fuel_index, reset_report};
use super::{
    E, SelectedInternalHeatGains, SourceGainModule, SourceGainStepOwners, index, unavailable,
};

impl SelectedInternalHeatGains {
    pub(super) fn initialize_timestep(
        &mut self,
        owners: &mut SourceGainStepOwners<'_, '_>,
    ) -> Result<(), E> {
        for row in self
            .allocations
            .space_int_gain
            .as_mut()
            .ok_or_else(|| unavailable("same actual spaceIntGain reset owner"))?
        {
            row[0] = 0.0;
            row[1] = 0.0;
        }
        for row in self
            .allocations
            .zone_electric_end_use
            .as_mut()
            .ok_or_else(|| unavailable("same actual ZoneIntEEuse reset owner"))?
        {
            *row = [[0.0; 26]; 4]; // Actual zeroZoneCatEUse member arrays.
        }
        for row in self
            .allocations
            .zone_report
            .as_mut()
            .ok_or_else(|| unavailable("same actual ZoneRpt CO2 reset owner"))?
        {
            row.co2_rate = 0.0;
        }
        for row in self
            .allocations
            .space_report
            .as_mut()
            .ok_or_else(|| unavailable("same actual spaceRpt reset owner"))?
        {
            reset_report(row, true);
        }
        for row in owners
            .zones
            .zone_predefined
            .as_mut()
            .ok_or_else(|| unavailable("same actual ZonePreDefRep reset owner"))?
        {
            row.num_occ = 0.0;
        }
        // Native6631-6696: nonpositive occupancy retains NumOcc/NOFOCC but
        // skips activity and occupied accumulators, with genuine +0 heat rates.
        for person in 0..self.positive_count(0)? {
            let row = self
                .allocations
                .people
                .rows
                .get_mut(person)
                .ok_or_else(|| unavailable("actual People timestep row"))?;
            let mut occupants = row.number_of_people
                * owners.schedules.source_current_schedule_value(
                    row.sched
                        .ok_or_else(|| unavailable("same People occupancy schedule"))?,
                )?;
            if row.ems_people_on {
                occupants = row.ems_number_of_people;
            }
            let mut total = 0.0;
            let mut sensible = 0.0;
            let zone = owners
                .zones
                .zone_predefined
                .as_mut()
                .and_then(|v| v.get_mut(index(row.zone_ptr).ok()?))
                .ok_or_else(|| unavailable("same People ZonePreDefRep"))?;
            if occupants > 0.0 {
                let activity = owners.schedules.source_current_schedule_value(
                    row.activity_level_sched
                        .ok_or_else(|| unavailable("same People activity schedule"))?,
                )?;
                total = occupants * activity;
                if row.user_spec_sens_frac == -99999.0 {
                    // Native6463 coefficients and exact6657-6660 nesting; no
                    // Zone MAT substitution, reassociation or fused operation.
                    const C: [f64; 9] = [
                        6.4611027,
                        0.946892,
                        0.0000255737,
                        7.139322,
                        -0.0627909,
                        0.0000589271,
                        -0.198550,
                        0.000940018,
                        -0.00000149532,
                    ];
                    let air_temp = *owners
                        .latent
                        .as_deref()
                        .ok_or_else(|| unavailable("same actual People Space AirHB MAT owner"))?
                        .space_mat(row.space_index)?;
                    let room_air = owners
                        .room_air
                        .ok_or_else(|| unavailable("same actual People RoomAir owner"))?;
                    if room_air.any_non_mixing {
                        return Err(unavailable(
                            "actual People non-Mixing DispVent/UFAD temperature operands",
                        ));
                    }
                    sensible = occupants
                        * (C[0]
                            + activity * (C[1] + activity * C[2])
                            + air_temp
                                * ((C[3] + activity * (C[4] + activity * C[5]))
                                    + air_temp * (C[6] + activity * (C[7] + activity * C[8]))));
                } else {
                    sensible = total * row.user_spec_sens_frac;
                }
                if sensible > total {
                    sensible = total;
                }
                if sensible < 0.0 {
                    sensible = 0.0;
                }
                zone.is_occupied = true;
                zone.num_occ += occupants;
                zone.num_occ_accum += occupants * owners.clock.time_step_zone;
                zone.num_occ_accum_time += owners.clock.time_step_zone;
            } else {
                zone.is_occupied = false;
            }
            row.num_occ = occupants;
            row.rad_gain_rate = sensible * row.fraction_radiant;
            row.con_gain_rate = sensible * row.fraction_convected;
            row.sen_gain_rate = sensible;
            row.lat_gain_rate = total - sensible;
            row.tot_gain_rate = total;
            row.co2_gain_rate = total * row.co2_rate_factor;
            self.allocations
                .space_int_gain
                .as_mut()
                .and_then(|v| v.get_mut(index(row.space_index).ok()?))
                .ok_or_else(|| unavailable("same People SpaceIntGain"))?[0] += row.num_occ;
            let report = self
                .allocations
                .space_report
                .as_mut()
                .and_then(|v| v.get_mut(index(row.space_index).ok()?))
                .ok_or_else(|| unavailable("same People SpaceRpt"))?;
            report.people_rad_gain_rate += row.rad_gain_rate;
            report.people_con_gain_rate += row.con_gain_rate;
            report.people_sen_gain_rate += row.sen_gain_rate;
            report.people_lat_gain_rate += row.lat_gain_rate;
            report.people_tot_gain_rate += row.tot_gain_rate;
        }
        // Lights, ElectricEquipment and GasEquipment precede OtherEq.
        for family in [1, 2, 3] {
            self.require_empty_family(family)?;
        }
        for equipment in 0..self.positive_count(6)? {
            let row = self
                .allocations
                .other_equipment
                .as_mut()
                .and_then(|rows| rows.get_mut(equipment))
                .ok_or_else(|| unavailable("actual OtherEquipment timestep row"))?;
            let schedule = row
                .schedule
                .and_then(|i| owners.schedules.schedules.get(i.0))
                .ok_or_else(|| unavailable("same actual OtherEquipment Schedule pointer"))?;
            // ScheduleManager.hh266: EMSActuatedOn ? EMSVal : currentVal.
            let current_value = if schedule.ems_actuated_on {
                schedule.ems_value
            } else {
                schedule.current_value
            };
            let mut q = row.design_level * current_value;
            if row.ems_zone_equipment_override_on {
                q = row.ems_equipment_power;
            }
            row.rates[0] = q;
            row.rates[1] = q * row.fraction_radiant;
            row.rates[2] = q * row.fraction_convected;
            row.rates[3] = q * row.fraction_latent;
            row.rates[4] = q * row.fraction_lost;
            row.rates[5] = q - row.rates[4];
            // Source does not write OtherEq.CO2GainRate in this loop.
            let fuel = fuel_index(row.fuel as i32)?;
            let report = self
                .allocations
                .space_report
                .as_mut()
                .and_then(|rows| rows.get_mut(index(row.space_index).ok()?))
                .ok_or_else(|| unavailable("same actual OtherEquipment spaceRpt association"))?;
            report.other_power[fuel] += row.rates[0];
            report.other[9] += row.rates[5];
            report.other[1] += row.rates[1];
            report.other[3] += row.rates[2];
            report.other[5] += row.rates[3];
            report.other[7] += row.rates[4];
        }
        for family in [4, 5, 8, 9, 7] {
            self.require_empty_family(family)?;
        }
        for module in [
            SourceGainModule::WaterThermalTanks,
            SourceGainModule::PipeHeatTransfer,
            SourceGainModule::WaterUse,
            SourceGainModule::FuelCell,
            SourceGainModule::MicroChp,
            SourceGainModule::ElectricFacility,
            SourceGainModule::TubularDaylighting,
            SourceGainModule::Refrigeration,
        ] {
            owners.modules.run_module(
                module,
                self,
                owners.clock,
                owners.globals,
                owners.zones,
                owners.backend,
            )?;
        }
        // Genuine default UpdateInternalGainValues: do radiation, no latent sum.
        self.update_timestep_device_values(owners)?;
        let zones = owners
            .zones
            .zones
            .as_deref()
            .ok_or_else(|| unavailable("same actual latent Zone owner"))?;
        let spaces = owners
            .zones
            .spaces
            .as_deref()
            .ok_or_else(|| unavailable("same actual radiant Space owner"))?;
        let devices = self
            .allocations
            .space_devices
            .as_deref()
            .ok_or_else(|| unavailable("same actual gain device sum owner"))?;
        for zone_number in 1..=owners.globals.num_of_zones {
            let zone = zones
                .get(index(zone_number)?)
                .ok_or_else(|| unavailable("actual latent Zone association"))?;
            sum_zone_latent_gains(
                devices,
                zone_number,
                &zone.space_indexes,
                &mut owners.latent,
            )?;
            if owners.globals.hybrid_model_people_count {
                return Err(unavailable(
                    "positive Source SumAllInternalLatentGainsExceptPeople",
                ));
            }
        }
        let radiation = owners
            .radiation
            .as_deref_mut()
            .ok_or_else(|| unavailable("same actual radiant enclosure/surface gain owners"))?;
        for enclosure_number in 1..=radiation.num_radiant_enclosures() {
            let enclosure = radiation.radiant_enclosure_mut(enclosure_number)?;
            enclosure.rad_q_thermal_rad = 0.0;
            for &space_number in &enclosure.space_nums {
                let space_ql = sum_all_space(devices, space_number, 2)?;
                enclosure.rad_q_thermal_rad += space_ql;
            }
        }
        if owners.globals.component_load_report_requested {
            return Err(unavailable("positive Source AllocateLoadComponentArrays"));
        }
        for zone_number in 1..=owners.globals.num_of_zones {
            let zone = zones
                .get(index(zone_number)?)
                .ok_or_else(|| unavailable("actual surface-distribution Zone association"))?;
            for &space_number in &zone.space_indexes {
                let space = spaces
                    .get(index(space_number)?)
                    .ok_or_else(|| unavailable("actual surface-distribution Space association"))?;
                if space.ht_surface_first <= 0 {
                    continue;
                }
                for surface_number in space.ht_surface_first..=space.ht_surface_last {
                    let enclosure_number = radiation.surface_radiant_enclosure(surface_number)?;
                    let enclosure = radiation.radiant_enclosure_mut(enclosure_number)?;
                    let q = enclosure.rad_q_thermal_rad;
                    let multiplier = enclosure.rad_therm_abs_mult;
                    let floor_area = enclosure.floor_area;
                    let absorptance = *radiation.surface_thermal_absorptance(surface_number)?;
                    if !owners.globals.load_component_pulse_now {
                        *radiation.surface_internal_radiant_gain_mut(surface_number)? =
                            q * multiplier * absorptance;
                    } else {
                        let adjusted_q = q + floor_area * 0.01;
                        *radiation.surface_internal_radiant_gain_mut(surface_number)? =
                            adjusted_q * multiplier * absorptance;
                        return Err(unavailable(
                            "actual radiantPulseTimestep/Received report owners",
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
