//! Original BEPS local allocation/copy, collapse, conversion and primary heat scan.
use super::helpers::{category, count};
use super::{Error, SourceBepsReportOwners, SourceTabularReports};
use crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray;

fn allocated<const N: usize>(shape: [usize; N]) -> Result<SourceSolarArray<f64, N>, Error> {
    let mut a = SourceSolarArray::default();
    a.dimension(shape, None)?;
    Ok(a)
}
pub(super) struct SourceBepsLocals {
    pub(super) use_val: SourceSolarArray<f64, 2>,
    pub(super) normal_val: SourceSolarArray<f64, 2>,
    pub(super) collapsed_total: SourceSolarArray<f64, 1>,
    pub(super) collapsed_end_use: SourceSolarArray<f64, 2>,
    pub(super) collapsed_sub: SourceSolarArray<f64, 3>,
    pub(super) sub_other: SourceSolarArray<f64, 2>,
    pub(super) collapsed_space: SourceSolarArray<f64, 3>,
    pub(super) gt_power_fuel_fire_gen: f64,
    pub(super) gt_power_pv: f64,
    pub(super) gt_power_wind: f64,
    pub(super) gt_power_ht_geothermal: f64,
    pub(super) gt_power_conversion: f64,
    pub(super) gt_electric_produced: f64,
    pub(super) gt_electric_purchased: f64,
    pub(super) gt_electric_surplus_sold: f64,
    pub(super) gt_water_heat_recovery: f64,
    pub(super) gt_air_heat_recovery_cool: f64,
    pub(super) gt_air_heat_recovery_heat: f64,
    pub(super) gt_heat_ht_geothermal: f64,
    pub(super) gt_heat_solar_water: f64,
    pub(super) gt_heat_solar_air: f64,
}
pub(super) struct SourceBepsPass {
    pub(super) large: f64,
    pub(super) k: f64,
    pub(super) water: f64,
    pub(super) area: f64,
    pub(super) ip_electricity: f64,
    pub(super) primary_heating: usize,
}
impl SourceTabularReports {
    pub(super) fn allocate_source_beps_locals(
        &self,
        o: &SourceBepsReportOwners<'_, '_>,
    ) -> Result<SourceBepsLocals, Error> {
        let use_val = allocated([14, 15])?;
        let normal_val = allocated([14, 4])?;
        let collapsed_total = allocated([14])?;
        let collapsed_end_use = allocated([14, 14])?;
        let collapsed_sub = allocated([count(o.registration.max_num_subcategories)?, 14, 14])?;
        let sub_other = allocated([14, 14])?;
        let collapsed_space =
            allocated([count(o.registration.max_num_end_use_space_types)?, 14, 14])?;
        let s = &self.input_state;
        let gt_power_fuel_fire_gen = s.gather_power_fuel_fire_gen;
        let gt_power_pv = s.gather_power_pv;
        let gt_power_wind = s.gather_power_wind;
        let gt_power_ht_geothermal = s.gather_power_ht_geothermal;
        let gt_power_conversion = s.gather_power_conversion;
        let gt_electric_produced = s.gather_electric_produced;
        let gt_electric_purchased = s.gather_electric_purchased;
        let gt_electric_surplus_sold = s.gather_electric_surplus_sold;
        let gt_water_heat_recovery = s.gather_water_heat_recovery;
        let gt_air_heat_recovery_cool = s.gather_air_heat_recovery_cool;
        let gt_air_heat_recovery_heat = s.gather_air_heat_recovery_heat;
        let gt_heat_ht_geothermal = s.gather_heat_ht_geothermal;
        let gt_heat_solar_water = s.gather_heat_solar_water;
        let gt_heat_solar_air = s.gather_heat_solar_air;
        Ok(SourceBepsLocals {
            use_val,
            normal_val,
            collapsed_total,
            collapsed_end_use,
            collapsed_sub,
            sub_other,
            collapsed_space,
            gt_power_fuel_fire_gen,
            gt_power_pv,
            gt_power_wind,
            gt_power_ht_geothermal,
            gt_power_conversion,
            gt_electric_produced,
            gt_electric_purchased,
            gt_electric_surplus_sold,
            gt_water_heat_recovery,
            gt_air_heat_recovery_cool,
            gt_air_heat_recovery_heat,
            gt_heat_ht_geothermal,
            gt_heat_solar_water,
            gt_heat_solar_air,
        })
    }
    pub(super) fn prepare_source_beps_pass(
        &mut self,
        pass: usize,
        first_style: bool,
        l: &mut SourceBepsLocals,
        o: &SourceBepsReportOwners<'_, '_>,
    ) -> Result<SourceBepsPass, Error> {
        self.determine_source_building_floor_area(o)?;
        // Actual14 assignments per end use retain the Source resource permutation.
        const SOURCE: [usize; 14] = [1, 2, 6, 8, 9, 10, 11, 12, 13, 14, 3, 4, 5, 7];
        for end_use in 1..=14 {
            for (resource, &source) in SOURCE.iter().enumerate() {
                l.collapsed_end_use.set(
                    [resource + 1, end_use],
                    self.input_state
                        .gather_end_use_beps
                        .read([source, end_use])?,
                )?;
            }
        }
        for (resource, &source) in SOURCE.iter().enumerate() {
            l.collapsed_total.set(
                [resource + 1],
                self.input_state.gather_totals_beps[source - 1],
            )?;
        }
        if self.input_state.tabular_report_passes[pass].produce_tabular && o.globals.create_perf_log
        {
            return Err(Error::Unavailable(
                "actual BEPS7894 AppendPerfLog first formatted energy field",
            ));
        }
        for end_use in 1..=14 {
            for sub in 1..=category(o.registration, end_use)?.num_subcategories {
                for (resource, &source) in SOURCE.iter().enumerate() {
                    l.collapsed_sub.set(
                        [sub as usize, end_use, resource + 1],
                        self.input_state.gather_end_use_sub_beps.read([
                            sub as usize,
                            end_use,
                            source,
                        ])?,
                    )?;
                }
            }
            for space in 1..=category(o.registration, end_use)?.num_space_types {
                for (resource, &source) in SOURCE.iter().enumerate() {
                    l.collapsed_space.set(
                        [space as usize, end_use, resource + 1],
                        self.input_state.gather_end_use_space_type_beps.read([
                            space as usize,
                            end_use,
                            source,
                        ])?,
                    )?;
                }
            }
        }
        // Actual ipElectricityConversionFactor=1 precedes the units switch.
        let ip_electricity = 1.0;
        let (large, k, water, area) = match self.input_state.tabular_report_passes[pass].units_style
        {
            1 => (3_600_000.0, 1.0, 1.0, 1.0),
            4 | 5 => {
                return Err(Error::Unavailable(
                    "actual BEPS7974 getSpecificUnitDivider J/kBtu operation",
                ));
            }
            _ => (1_000_000_000.0, 1000.0, 1.0, 1.0),
        };
        for resource in 1..=13 {
            let adjust = if resource == 1 { ip_electricity } else { 1.0 };
            for end_use in 1..=14 {
                let value = l.collapsed_end_use.read([resource, end_use])? / (large / adjust);
                l.collapsed_end_use.set([resource, end_use], value)?;
                for sub in 1..=category(o.registration, end_use)?.num_subcategories {
                    let index = [sub as usize, end_use, resource];
                    let value = l.collapsed_sub.read(index)? / (large / adjust);
                    l.collapsed_sub.set(index, value)?;
                }
                for space in 1..=category(o.registration, end_use)?.num_space_types {
                    let index = [space as usize, end_use, resource];
                    let value = l.collapsed_space.read(index)? / (large / adjust);
                    l.collapsed_space.set(index, value)?;
                }
            }
            let value = l.collapsed_total.read([resource])? / (large / adjust);
            l.collapsed_total.set([resource], value)?;
        }
        for end_use in 1..=14 {
            let value = l.collapsed_end_use.read([14, end_use])? / water;
            l.collapsed_end_use.set([14, end_use], value)?;
            for sub in 1..=category(o.registration, end_use)?.num_subcategories {
                let index = [sub as usize, end_use, 14];
                let value = l.collapsed_sub.read(index)? / water;
                l.collapsed_sub.set(index, value)?;
            }
            for space in 1..=category(o.registration, end_use)?.num_space_types {
                let index = [space as usize, end_use, 14];
                let value = l.collapsed_space.read(index)? / water;
                l.collapsed_space.set(index, value)?;
            }
        }
        let value = l.collapsed_total.read([14])? / water;
        l.collapsed_total.set([14], value)?;
        let s = &mut self.input_state;
        if first_style {
            l.gt_power_fuel_fire_gen = s.gather_power_fuel_fire_gen;
            l.gt_power_pv = s.gather_power_pv;
            l.gt_power_wind = s.gather_power_wind;
            l.gt_power_ht_geothermal = s.gather_power_ht_geothermal;
            l.gt_power_conversion = s.gather_power_conversion;
            l.gt_electric_produced = s.gather_electric_produced;
            l.gt_electric_purchased = s.gather_electric_purchased;
            l.gt_electric_surplus_sold = s.gather_electric_surplus_sold;
        } else {
            s.gather_power_fuel_fire_gen = l.gt_power_fuel_fire_gen;
            s.gather_power_pv = l.gt_power_pv;
            s.gather_power_wind = l.gt_power_wind;
            s.gather_power_ht_geothermal = l.gt_power_ht_geothermal;
            s.gather_power_conversion = l.gt_power_conversion;
            s.gather_electric_produced = l.gt_electric_produced;
            s.gather_electric_purchased = l.gt_electric_purchased;
            s.gather_electric_surplus_sold = l.gt_electric_surplus_sold;
        }
        let factor = ip_electricity / large;
        s.gather_power_fuel_fire_gen *= factor;
        s.gather_power_pv *= factor;
        s.gather_power_wind *= factor;
        s.gather_power_ht_geothermal *= factor;
        s.gather_power_conversion *= factor;
        s.gather_electric_produced *= factor;
        s.gather_electric_purchased *= factor;
        s.gather_electric_surplus_sold *= factor;
        if o.facility.number_of_electric_storage_devices > 0 {
            s.overall_net_energy_from_storage = s.gather_electric_storage;
            s.overall_net_energy_from_storage *= factor;
        } else {
            s.overall_net_energy_from_storage = 0.0;
        }
        let mut primary_heating = 0;
        let mut maximum = 0.0;
        for resource in 1..=13 {
            if l.collapsed_end_use.read([resource, 1])? > maximum {
                maximum = l.collapsed_end_use.read([resource, 1])?;
                primary_heating = resource;
            }
        }
        Ok(SourceBepsPass {
            large,
            k,
            water,
            area,
            ip_electricity,
            primary_heating,
        })
    }
}
