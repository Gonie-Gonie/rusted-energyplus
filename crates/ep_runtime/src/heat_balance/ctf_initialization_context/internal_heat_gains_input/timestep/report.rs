//! Literal ReportInternalHeatGains field writes for the reached ordinary domain.
use super::super::SourceGainReport;
use super::devices::sum_by_types;
use super::{E, SelectedInternalHeatGains, SourceGainStepOwners, index, unavailable};

pub(super) fn fuel_index(fuel: i32) -> Result<usize, E> {
    usize::try_from(fuel)
        .ok()
        .filter(|&i| i < 15)
        .ok_or_else(|| unavailable("actual Source eFuel report index"))
}

// Called in Init for each Space and in Report for each Zone. IT fields are not
// part of these Source reset blocks. Zone CO2Rate is preserved during Report.
pub(super) fn reset_report(row: &mut SourceGainReport, clear_co2: bool) {
    // People
    row.people_rad_gain = 0.0;
    row.people_con_gain = 0.0;
    row.people_sen_gain = 0.0;
    row.people_num_occ = 0.0;
    row.people_lat_gain = 0.0;
    row.people_tot_gain = 0.0;
    row.people_rad_gain_rate = 0.0;
    row.people_con_gain_rate = 0.0;
    row.people_sen_gain_rate = 0.0;
    row.people_lat_gain_rate = 0.0;
    row.people_tot_gain_rate = 0.0;
    // Lights
    row.lts_power = 0.0;
    row.lts_elec_consump = 0.0;
    row.lts_rad_gain = 0.0;
    row.lts_vis_gain = 0.0;
    row.lts_con_gain = 0.0;
    row.lts_ret_air_gain = 0.0;
    row.lts_tot_gain = 0.0;
    row.lts_rad_gain_rate = 0.0;
    row.lts_vis_gain_rate = 0.0;
    row.lts_con_gain_rate = 0.0;
    row.lts_ret_air_gain_rate = 0.0;
    row.lts_tot_gain_rate = 0.0;
    // Baseboard Heat
    row.base_heat_power = 0.0;
    row.base_heat_elec_cons = 0.0;
    row.base_heat_rad_gain = 0.0;
    row.base_heat_con_gain = 0.0;
    row.base_heat_tot_gain = 0.0;
    row.base_heat_rad_gain_rate = 0.0;
    row.base_heat_con_gain_rate = 0.0;
    row.base_heat_tot_gain_rate = 0.0;
    // Electric Equipment
    row.elec_power = 0.0;
    row.elec_consump = 0.0;
    row.elec_rad_gain = 0.0;
    row.elec_con_gain = 0.0;
    row.elec_lat_gain = 0.0;
    row.elec_lost = 0.0;
    row.elec_tot_gain = 0.0;
    row.elec_rad_gain_rate = 0.0;
    row.elec_con_gain_rate = 0.0;
    row.elec_lat_gain_rate = 0.0;
    row.elec_lost_rate = 0.0;
    row.elec_tot_gain_rate = 0.0;
    // Gas Equipment
    row.gas_power = 0.0;
    row.gas_consump = 0.0;
    row.gas_rad_gain = 0.0;
    row.gas_con_gain = 0.0;
    row.gas_lat_gain = 0.0;
    row.gas_lost = 0.0;
    row.gas_tot_gain = 0.0;
    row.gas_rad_gain_rate = 0.0;
    row.gas_con_gain_rate = 0.0;
    row.gas_lat_gain_rate = 0.0;
    row.gas_lost_rate = 0.0;
    row.gas_tot_gain_rate = 0.0;
    // Hot Water Equipment
    row.hw_power = 0.0;
    row.hw_consump = 0.0;
    row.hw_rad_gain = 0.0;
    row.hw_con_gain = 0.0;
    row.hw_lat_gain = 0.0;
    row.hw_lost = 0.0;
    row.hw_tot_gain = 0.0;
    row.hw_rad_gain_rate = 0.0;
    row.hw_con_gain_rate = 0.0;
    row.hw_lat_gain_rate = 0.0;
    row.hw_lost_rate = 0.0;
    row.hw_tot_gain_rate = 0.0;
    // Steam Equipment
    row.steam_power = 0.0;
    row.steam_consump = 0.0;
    row.steam_rad_gain = 0.0;
    row.steam_con_gain = 0.0;
    row.steam_lat_gain = 0.0;
    row.steam_lost = 0.0;
    row.steam_tot_gain = 0.0;
    row.steam_rad_gain_rate = 0.0;
    row.steam_con_gain_rate = 0.0;
    row.steam_lat_gain_rate = 0.0;
    row.steam_lost_rate = 0.0;
    row.steam_tot_gain_rate = 0.0;
    // Other Equipment
    row.other[0] = 0.0;
    row.other[2] = 0.0;
    row.other[4] = 0.0;
    row.other[6] = 0.0;
    row.other[8] = 0.0;
    row.other[1] = 0.0;
    row.other[3] = 0.0;
    row.other[5] = 0.0;
    row.other[7] = 0.0;
    row.other[9] = 0.0;
    // Overall Zone Variables
    row.total[0] = 0.0;
    row.total[2] = 0.0;
    row.total[4] = 0.0;
    row.total[6] = 0.0;
    row.total[8] = 0.0;
    row.total[1] = 0.0;
    row.total[3] = 0.0;
    row.total[5] = 0.0;
    row.total[7] = 0.0;
    row.total[9] = 0.0;
    // Contaminant
    if clear_co2 {
        row.co2_rate = 0.0;
    }
    row.gc_rate = 0.0;
    for i in 0..15 {
        row.other_power[i] = 0.0;
        row.other_consumption[i] = 0.0;
    }
}

impl SelectedInternalHeatGains {
    pub(crate) fn report_timestep(
        &mut self,
        owners: &SourceGainStepOwners<'_, '_>,
    ) -> Result<(), E> {
        let seconds = owners.clock.time_step_zone_sec;
        // Source individual-object loops, before the Zone reset.
        for person in 0..self.positive_count(0)? {
            let row = self
                .allocations
                .people
                .rows
                .get_mut(person)
                .ok_or_else(|| unavailable("actual People report row"))?;
            row.rad_gain_energy = row.rad_gain_rate * seconds;
            row.con_gain_energy = row.con_gain_rate * seconds;
            row.sen_gain_energy = row.sen_gain_rate * seconds;
            row.lat_gain_energy = row.lat_gain_rate * seconds;
            row.tot_gain_energy = row.tot_gain_rate * seconds;
        }
        self.require_empty_family(1)?;
        for equipment in 0..self.positive_count(2)? {
            let row = self
                .allocations
                .electric_equipment
                .as_mut()
                .and_then(|rows| rows.get_mut(equipment))
                .ok_or_else(|| unavailable("actual ZoneElectric report row"))?;
            row.energies[0] = row.rates[0] * seconds;
            row.energies[1] = row.rates[1] * seconds;
            row.energies[2] = row.rates[2] * seconds;
            row.energies[3] = row.rates[3] * seconds;
            row.energies[4] = row.rates[4] * seconds;
            row.energies[5] = row.rates[5] * seconds;
        }
        self.require_empty_family(3)?;
        for equipment in 0..self.positive_count(6)? {
            let row = self
                .allocations
                .other_equipment
                .as_mut()
                .and_then(|v| v.get_mut(equipment))
                .ok_or_else(|| unavailable("actual OtherEquipment report row"))?;
            row.energies[0] = row.rates[0] * seconds;
            row.energies[1] = row.rates[1] * seconds;
            row.energies[2] = row.rates[2] * seconds;
            row.energies[3] = row.rates[3] * seconds;
            row.energies[4] = row.rates[4] * seconds;
            row.energies[5] = row.rates[5] * seconds;
        }
        for family in [4, 5, 8] {
            self.require_empty_family(family)?;
        }
        for row in self
            .allocations
            .zone_int_gain
            .as_mut()
            .ok_or_else(|| unavailable("same actual ZoneIntGain allocation"))?
        {
            row[0] = 0.0;
            row[1] = 0.0;
        }
        let zone_reports = self
            .allocations
            .zone_report
            .as_mut()
            .ok_or_else(|| unavailable("same actual ZoneRpt allocation"))?;
        for row in &mut *zone_reports {
            reset_report(row, false);
        }
        let zones = owners
            .zones
            .zones
            .as_deref()
            .ok_or_else(|| unavailable("same actual Zone report association"))?;
        let spaces = owners
            .zones
            .spaces
            .as_deref()
            .ok_or_else(|| unavailable("same actual Space report association"))?;
        let devices = self
            .allocations
            .space_devices
            .as_deref()
            .ok_or_else(|| unavailable("same actual report device allocation"))?;
        let space_reports = self
            .allocations
            .space_report
            .as_mut()
            .ok_or_else(|| unavailable("same actual spaceRpt allocation"))?;
        let space_int_gains = self
            .allocations
            .space_int_gain
            .as_ref()
            .ok_or_else(|| unavailable("same actual spaceIntGain allocation"))?;
        let zone_int_gains = self
            .allocations
            .zone_int_gain
            .as_mut()
            .ok_or_else(|| unavailable("same actual ZoneIntGain allocation"))?;
        for space_index in 0..usize::try_from(owners.globals.num_spaces)
            .map_err(|_| unavailable("actual report numSpaces"))?
        {
            let space_number = i32::try_from(space_index + 1)
                .map_err(|_| unavailable("actual report Source Space ordinal"))?;
            let zone_index = index(
                spaces
                    .get(space_index)
                    .ok_or_else(|| unavailable("actual report Space row"))?
                    .zone_num,
            )?;
            let space_report = space_reports
                .get_mut(space_index)
                .ok_or_else(|| unavailable("actual Space report row"))?;
            let space_int_gain = space_int_gains
                .get(space_index)
                .ok_or_else(|| unavailable("actual SpaceIntGain row"))?;
            let zone_report = zone_reports
                .get_mut(zone_index)
                .ok_or_else(|| unavailable("actual Zone report row"))?;
            let zone_int_gain = zone_int_gains
                .get_mut(zone_index)
                .ok_or_else(|| unavailable("actual ZoneIntGain row"))?;
            // People
            space_report.people_num_occ = space_int_gain[0];
            space_report.people_rad_gain = space_report.people_rad_gain_rate * seconds;
            space_report.people_con_gain = space_report.people_con_gain_rate * seconds;
            space_report.people_sen_gain = space_report.people_sen_gain_rate * seconds;
            space_report.people_lat_gain = space_report.people_lat_gain_rate * seconds;
            space_report.people_tot_gain = space_report.people_tot_gain_rate * seconds;

            zone_int_gain[0] += space_int_gain[0];
            zone_report.people_rad_gain_rate += space_report.people_rad_gain_rate;
            zone_report.people_con_gain_rate += space_report.people_con_gain_rate;
            zone_report.people_sen_gain_rate += space_report.people_sen_gain_rate;
            zone_report.people_lat_gain_rate += space_report.people_lat_gain_rate;
            zone_report.people_tot_gain_rate += space_report.people_tot_gain_rate;

            // General Lights
            space_report.lts_elec_consump = space_report.lts_power * seconds;
            space_report.lts_ret_air_gain = space_report.lts_ret_air_gain_rate * seconds;
            space_report.lts_rad_gain = space_report.lts_rad_gain_rate * seconds;
            space_report.lts_tot_gain = space_report.lts_tot_gain_rate * seconds;
            space_report.lts_con_gain = space_report.lts_con_gain_rate * seconds;
            space_report.lts_vis_gain = space_report.lts_vis_gain_rate * seconds;

            zone_report.lts_power += space_report.lts_power;
            zone_report.lts_ret_air_gain_rate += space_report.lts_ret_air_gain_rate;
            zone_report.lts_rad_gain_rate += space_report.lts_rad_gain_rate;
            zone_report.lts_tot_gain_rate += space_report.lts_tot_gain_rate;
            zone_report.lts_con_gain_rate += space_report.lts_con_gain_rate;
            zone_report.lts_vis_gain_rate += space_report.lts_vis_gain_rate;
            zone_int_gain[1] += space_int_gain[1];

            // Electric Equipment
            space_report.elec_consump = space_report.elec_power * seconds;
            space_report.elec_con_gain = space_report.elec_con_gain_rate * seconds;
            space_report.elec_rad_gain = space_report.elec_rad_gain_rate * seconds;
            space_report.elec_lat_gain = space_report.elec_lat_gain_rate * seconds;
            space_report.elec_lost = space_report.elec_lost_rate * seconds;
            space_report.elec_tot_gain_rate = space_report.elec_con_gain_rate
                + space_report.elec_rad_gain_rate
                + space_report.elec_lat_gain_rate;
            space_report.elec_tot_gain = space_report.elec_tot_gain_rate * seconds;

            zone_report.elec_power += space_report.elec_power;
            zone_report.elec_con_gain_rate += space_report.elec_con_gain_rate;
            zone_report.elec_rad_gain_rate += space_report.elec_rad_gain_rate;
            zone_report.elec_lat_gain_rate += space_report.elec_lat_gain_rate;
            zone_report.elec_lost_rate += space_report.elec_lost_rate;
            zone_report.elec_tot_gain_rate += space_report.elec_tot_gain_rate;

            // Gas Equipment
            space_report.gas_consump = space_report.gas_power * seconds;
            space_report.gas_con_gain = space_report.gas_con_gain_rate * seconds;
            space_report.gas_rad_gain = space_report.gas_rad_gain_rate * seconds;
            space_report.gas_lat_gain = space_report.gas_lat_gain_rate * seconds;
            space_report.gas_lost = space_report.gas_lost_rate * seconds;
            space_report.gas_tot_gain_rate = space_report.gas_con_gain_rate
                + space_report.gas_rad_gain_rate
                + space_report.gas_lat_gain_rate;
            space_report.gas_tot_gain = space_report.gas_tot_gain_rate * seconds;

            zone_report.gas_power += space_report.gas_power;
            zone_report.gas_con_gain_rate += space_report.gas_con_gain_rate;
            zone_report.gas_rad_gain_rate += space_report.gas_rad_gain_rate;
            zone_report.gas_lat_gain_rate += space_report.gas_lat_gain_rate;
            zone_report.gas_lost_rate += space_report.gas_lost_rate;
            zone_report.gas_tot_gain_rate += space_report.gas_tot_gain_rate;

            // Hot Water Equipment
            space_report.hw_consump = space_report.hw_power * seconds;
            space_report.hw_con_gain = space_report.hw_con_gain_rate * seconds;
            space_report.hw_rad_gain = space_report.hw_rad_gain_rate * seconds;
            space_report.hw_lat_gain = space_report.hw_lat_gain_rate * seconds;
            space_report.hw_lost = space_report.hw_lost_rate * seconds;
            space_report.hw_tot_gain_rate = space_report.hw_con_gain_rate
                + space_report.hw_rad_gain_rate
                + space_report.hw_lat_gain_rate;
            space_report.hw_tot_gain = space_report.hw_tot_gain_rate * seconds;

            zone_report.hw_power += space_report.hw_power;
            zone_report.hw_con_gain_rate += space_report.hw_con_gain_rate;
            zone_report.hw_rad_gain_rate += space_report.hw_rad_gain_rate;
            zone_report.hw_lat_gain_rate += space_report.hw_lat_gain_rate;
            zone_report.hw_lost_rate += space_report.hw_lost_rate;
            zone_report.hw_tot_gain_rate += space_report.hw_tot_gain_rate;

            // Steam Equipment
            space_report.steam_consump = space_report.steam_power * seconds;
            space_report.steam_con_gain = space_report.steam_con_gain_rate * seconds;
            space_report.steam_rad_gain = space_report.steam_rad_gain_rate * seconds;
            space_report.steam_lat_gain = space_report.steam_lat_gain_rate * seconds;
            space_report.steam_lost = space_report.steam_lost_rate * seconds;
            space_report.steam_tot_gain_rate = space_report.steam_con_gain_rate
                + space_report.steam_rad_gain_rate
                + space_report.steam_lat_gain_rate;
            space_report.steam_tot_gain = space_report.steam_tot_gain_rate * seconds;

            zone_report.steam_power += space_report.steam_power;
            zone_report.steam_con_gain_rate += space_report.steam_con_gain_rate;
            zone_report.steam_rad_gain_rate += space_report.steam_rad_gain_rate;
            zone_report.steam_lat_gain_rate += space_report.steam_lat_gain_rate;
            zone_report.steam_lost_rate += space_report.steam_lost_rate;
            zone_report.steam_tot_gain_rate += space_report.steam_tot_gain_rate;

            // Other Equipment
            space_report.other[2] = space_report.other[3] * seconds;
            space_report.other[0] = space_report.other[1] * seconds;
            space_report.other[4] = space_report.other[5] * seconds;
            space_report.other[6] = space_report.other[7] * seconds;
            space_report.other[9] =
                space_report.other[3] + space_report.other[1] + space_report.other[5];
            space_report.other[8] = space_report.other[9] * seconds;

            zone_report.other[3] += space_report.other[3];
            zone_report.other[1] += space_report.other[1];
            zone_report.other[5] += space_report.other[5];
            zone_report.other[7] += space_report.other[7];
            zone_report.other[9] += space_report.other[9];

            for &fuel_type in &spaces[space_index].other_equipment_fuels {
                let fuel_index = fuel_index(fuel_type)?;
                space_report.other_consumption[fuel_index] =
                    space_report.other_power[fuel_index] * seconds;
                zone_report.other_power[fuel_index] += space_report.other_power[fuel_index];
            }

            // Baseboard Heat
            space_report.base_heat_elec_cons = space_report.base_heat_power * seconds;
            space_report.base_heat_con_gain = space_report.base_heat_con_gain_rate * seconds;
            space_report.base_heat_rad_gain = space_report.base_heat_rad_gain_rate * seconds;
            space_report.base_heat_tot_gain_rate =
                space_report.base_heat_con_gain_rate + space_report.base_heat_rad_gain_rate;
            space_report.base_heat_tot_gain = space_report.base_heat_tot_gain_rate * seconds;

            zone_report.base_heat_power += space_report.base_heat_power;
            zone_report.base_heat_con_gain_rate += space_report.base_heat_con_gain_rate;
            zone_report.base_heat_rad_gain_rate += space_report.base_heat_rad_gain_rate;
            zone_report.base_heat_tot_gain_rate += space_report.base_heat_tot_gain_rate;

            // Overall Space Variables

            // these overalls include component gains from devices like water heater, water use, and generators
            //   working vars QFCConv QGenConv QFCRad QGenRad  WaterUseLatentGain WaterThermalTankGain WaterUseSensibleGain

            space_report.total[2] = space_report.lts_vis_gain;
            space_report.total[3] = space_report.lts_vis_gain_rate;

            space_report.total[1] = sum_by_types(
                devices,
                &zones[zone_index].space_indexes,
                Some(space_number),
                2,
            )?;
            space_report.total[0] = space_report.total[1] * seconds;

            space_report.total[5] = sum_by_types(
                devices,
                &zones[zone_index].space_indexes,
                Some(space_number),
                0,
            )?;
            space_report.total[4] = space_report.total[5] * seconds;

            space_report.total[7] = sum_by_types(
                devices,
                &zones[zone_index].space_indexes,
                Some(space_number),
                3,
            )?;
            space_report.total[6] = space_report.total[7] * seconds;

            space_report.total[9] = space_report.total[7]
                + space_report.total[1]
                + space_report.total[5]
                + space_report.total[3];
            space_report.total[8] = space_report.total[9] * seconds;
        }
        // Preserve Original index order and same-row checked accesses across the retained work arrays.
        #[allow(clippy::needless_range_loop)]
        for zone_index in 0..usize::try_from(owners.globals.num_of_zones)
            .map_err(|_| unavailable("actual report NumOfZones"))?
        {
            let zone_report = zone_reports
                .get_mut(zone_index)
                .ok_or_else(|| unavailable("actual Zone report row"))?;
            let zone_int_gain = zone_int_gains
                .get(zone_index)
                .ok_or_else(|| unavailable("actual ZoneIntGain row"))?;
            // People
            zone_report.people_num_occ = zone_int_gain[0];
            zone_report.people_rad_gain = zone_report.people_rad_gain_rate * seconds;
            zone_report.people_con_gain = zone_report.people_con_gain_rate * seconds;
            zone_report.people_sen_gain = zone_report.people_sen_gain_rate * seconds;
            zone_report.people_lat_gain = zone_report.people_lat_gain_rate * seconds;
            zone_report.people_tot_gain = zone_report.people_tot_gain_rate * seconds;

            // General Lights
            zone_report.lts_ret_air_gain = zone_report.lts_ret_air_gain_rate * seconds;
            zone_report.lts_rad_gain = zone_report.lts_rad_gain_rate * seconds;
            zone_report.lts_tot_gain = zone_report.lts_tot_gain_rate * seconds;
            zone_report.lts_con_gain = zone_report.lts_con_gain_rate * seconds;
            zone_report.lts_vis_gain = zone_report.lts_vis_gain_rate * seconds;
            zone_report.lts_elec_consump = zone_report.lts_power * seconds;

            // Electric Equipment
            zone_report.elec_con_gain = zone_report.elec_con_gain_rate * seconds;
            zone_report.elec_rad_gain = zone_report.elec_rad_gain_rate * seconds;
            zone_report.elec_lat_gain = zone_report.elec_lat_gain_rate * seconds;
            zone_report.elec_lost = zone_report.elec_lost_rate * seconds;
            zone_report.elec_consump = zone_report.elec_power * seconds;
            zone_report.elec_tot_gain = zone_report.elec_tot_gain_rate * seconds;

            // Gas Equipment
            zone_report.gas_con_gain = zone_report.gas_con_gain_rate * seconds;
            zone_report.gas_rad_gain = zone_report.gas_rad_gain_rate * seconds;
            zone_report.gas_lat_gain = zone_report.gas_lat_gain_rate * seconds;
            zone_report.gas_lost = zone_report.gas_lost_rate * seconds;
            zone_report.gas_consump = zone_report.gas_power * seconds;
            zone_report.gas_tot_gain = zone_report.gas_tot_gain_rate * seconds;

            // Hot Water Equipment
            zone_report.hw_con_gain = zone_report.hw_con_gain_rate * seconds;
            zone_report.hw_rad_gain = zone_report.hw_rad_gain_rate * seconds;
            zone_report.hw_lat_gain = zone_report.hw_lat_gain_rate * seconds;
            zone_report.hw_lost = zone_report.hw_lost_rate * seconds;
            zone_report.hw_consump = zone_report.hw_power * seconds;
            zone_report.hw_tot_gain = zone_report.hw_tot_gain_rate * seconds;

            // Steam Equipment
            zone_report.steam_con_gain = zone_report.steam_con_gain_rate * seconds;
            zone_report.steam_rad_gain = zone_report.steam_rad_gain_rate * seconds;
            zone_report.steam_lat_gain = zone_report.steam_lat_gain_rate * seconds;
            zone_report.steam_lost = zone_report.steam_lost_rate * seconds;
            zone_report.steam_consump = zone_report.steam_power * seconds;
            zone_report.steam_tot_gain = zone_report.steam_tot_gain_rate * seconds;

            // Other Equipment
            zone_report.other[2] = zone_report.other[3] * seconds;
            zone_report.other[0] = zone_report.other[1] * seconds;
            zone_report.other[4] = zone_report.other[5] * seconds;
            zone_report.other[6] = zone_report.other[7] * seconds;
            zone_report.other[8] = zone_report.other[9] * seconds;
            for &fuel_type in &zones[zone_index].other_equipment_fuels {
                let fuel_index = fuel_index(fuel_type)?;
                zone_report.other_consumption[fuel_index] =
                    zone_report.other_power[fuel_index] * seconds;
            }

            // Baseboard Heat
            zone_report.base_heat_con_gain = zone_report.base_heat_con_gain_rate * seconds;
            zone_report.base_heat_rad_gain = zone_report.base_heat_rad_gain_rate * seconds;
            zone_report.base_heat_elec_cons = zone_report.base_heat_power * seconds;
            zone_report.base_heat_tot_gain = zone_report.base_heat_tot_gain_rate * seconds;

            // Overall Zone Variables

            // these overalls include component gains from devices like water heater, water use, and generators
            //   working vars QFCConv QGenConv QFCRad QGenRad  WaterUseLatentGain WaterThermalTankGain WaterUseSensibleGain

            zone_report.total[2] = zone_report.lts_vis_gain;
            zone_report.total[3] = zone_report.lts_vis_gain_rate;

            zone_report.total[1] =
                sum_by_types(devices, &zones[zone_index].space_indexes, None, 2)?;
            zone_report.total[0] = zone_report.total[1] * seconds;

            zone_report.total[5] =
                sum_by_types(devices, &zones[zone_index].space_indexes, None, 0)?;
            zone_report.total[4] = zone_report.total[5] * seconds;

            zone_report.total[7] =
                sum_by_types(devices, &zones[zone_index].space_indexes, None, 3)?;
            zone_report.total[6] = zone_report.total[7] * seconds;

            zone_report.total[9] = zone_report.total[7]
                + zone_report.total[1]
                + zone_report.total[5]
                + zone_report.total[3];
            zone_report.total[8] = zone_report.total[9] * seconds;
        }
        Ok(())
    }
}
