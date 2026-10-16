//! Literal Summary1715-1883 array/name/meter initialization on sole owners.
use super::*;
fn extent(value: i32) -> Result<usize, Error> {
    usize::try_from(value).map_err(|_| Error::Unavailable("actual BEPS array extent"))
}
fn meter(registration: &SourceOutputRegistration, name: &str) -> Result<i32, Error> {
    match registration.meters.meter_map.get(name) {
        Some(&value) => i32::try_from(value)
            .map_err(|_| Error::Unavailable("actual GetMeterIndex signed index")),
        None => Ok(-1),
    }
}
impl SourceTabularReports {
    pub(super) fn initialize_source_beps(
        &mut self,
        o: &SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        use constants::{END_USE_NAMES, RESOURCE_NAMES, SOURCE_NAMES};
        for (i, name) in RESOURCE_NAMES.iter().enumerate() {
            self.input_state.resource_type_names[i] = (*name).into();
        }
        for (i, name) in SOURCE_NAMES.iter().enumerate() {
            self.input_state.source_type_names[i] = (*name).into();
        }
        for (i, name) in END_USE_NAMES.iter().enumerate() {
            self.input_state.end_use_names[i] = (*name).into();
        }
        let subcategories = extent(o.registration.max_num_subcategories)?;
        self.input_state
            .meter_num_end_use_sub_beps
            .dimension([subcategories, 14, 14], None)?;
        let space_types = extent(o.registration.max_num_end_use_space_types)?;
        self.input_state
            .meter_num_end_use_space_type_beps
            .dimension([space_types, 14, 14], None)?;
        for end_use in 1..=14 {
            for resource in 1..=14 {
                for subcategory in 1..=subcategories {
                    self.input_state
                        .meter_num_end_use_sub_beps
                        .set([subcategory, end_use, resource], -1)?;
                }
                for space_type in 1..=space_types {
                    self.input_state
                        .meter_num_end_use_space_type_beps
                        .set([space_type, end_use, resource], -1)?;
                }
            }
        }
        for resource in 1..=14 {
            let mut meter_name = format!(
                "{}:FACILITY",
                self.input_state.resource_type_names[resource - 1]
            );
            let mut meter_number = meter(o.registration, &meter_name.to_ascii_uppercase())?;
            self.input_state.meter_num_totals_beps[resource - 1] = meter_number;
            for end_use in 1..=14 {
                meter_name = format!(
                    "{}:{}",
                    self.input_state.end_use_names[end_use - 1],
                    self.input_state.resource_type_names[resource - 1]
                );
                meter_number = meter(o.registration, &meter_name.to_ascii_uppercase())?;
                self.input_state
                    .meter_num_end_use_beps
                    .set([resource, end_use], meter_number)?;
                let category = o
                    .registration
                    .end_use_categories
                    .as_ref()
                    .and_then(|v| v.get(end_use - 1))
                    .ok_or(Error::Unavailable("same actual OP EndUseCategory1787"))?;
                for subcategory in 1..=extent(category.num_subcategories)? {
                    let name = category
                        .subcategory_names
                        .as_ref()
                        .and_then(|v| v.get(subcategory - 1))
                        .ok_or(Error::Unavailable("same actual OP SubcategoryName1789"))?;
                    meter_name = format!(
                        "{}:{}:{}",
                        name,
                        self.input_state.end_use_names[end_use - 1],
                        self.input_state.resource_type_names[resource - 1]
                    );
                    meter_number = meter(o.registration, &meter_name.to_ascii_uppercase())?;
                    self.input_state
                        .meter_num_end_use_sub_beps
                        .set([subcategory, end_use, resource], meter_number)?;
                }
                for space_type in 1..=extent(category.num_space_types)? {
                    let name = category
                        .space_type_names
                        .as_ref()
                        .and_then(|v| v.get(space_type - 1))
                        .ok_or(Error::Unavailable("same actual OP spaceTypeName1799"))?;
                    meter_name = format!(
                        "{}:{}:SpaceType:{}",
                        self.input_state.end_use_names[end_use - 1],
                        self.input_state.resource_type_names[resource - 1],
                        name
                    );
                    meter_number = meter(o.registration, &meter_name.to_ascii_uppercase())?;
                    self.input_state
                        .meter_num_end_use_space_type_beps
                        .set([space_type, end_use, resource], meter_number)?;
                }
            }
        }
        for resource in 1..=12 {
            let number = meter(
                o.registration,
                &format!(
                    "{}Emissions:Source",
                    self.input_state.source_type_names[resource - 1]
                )
                .to_ascii_uppercase(),
            )?;
            self.input_state.meter_num_totals_source[resource - 1] = number;
        }
        self.input_state.gather_totals_beps.fill(0.0);
        self.input_state.gather_totals_by_source_beps.fill(0.0);
        self.input_state.gather_totals_source.fill(0.0);
        self.input_state.gather_totals_by_source.fill(0.0);
        self.input_state.gather_end_use_beps.fill(0.0)?;
        self.input_state.gather_end_use_by_source_beps.fill(0.0)?;
        self.input_state
            .gather_end_use_sub_beps
            .dimension([subcategories, 14, 14], None)?;
        self.input_state.gather_end_use_sub_beps.fill(0.0)?;
        self.input_state
            .gather_end_use_space_type_beps
            .dimension([space_types, 14, 14], None)?;
        self.input_state.gather_end_use_space_type_beps.fill(0.0)?;
        self.input_state
            .gather_demand_end_use_sub
            .dimension([subcategories, 14, 14], None)?;
        self.input_state.gather_demand_end_use_sub.fill(0.0)?;
        self.input_state
            .gather_demand_ind_end_use_sub
            .dimension([subcategories, 14, 14], None)?;
        self.input_state.gather_demand_ind_end_use_sub.fill(0.0)?;
        self.input_state.meter_num_power_fuel_fire_gen =
            meter(o.registration, "COGENERATION:ELECTRICITYPRODUCED")?;
        self.input_state.meter_num_power_pv =
            meter(o.registration, "PHOTOVOLTAIC:ELECTRICITYPRODUCED")?;
        self.input_state.meter_num_power_wind =
            meter(o.registration, "WINDTURBINE:ELECTRICITYPRODUCED")?;
        self.input_state.meter_num_power_ht_geothermal =
            meter(o.registration, "HTGEOTHERMAL:ELECTRICITYPRODUCED")?;
        self.input_state.meter_num_electric_storage =
            meter(o.registration, "ELECTRICSTORAGE:ELECTRICITYPRODUCED")?;
        self.input_state.meter_num_power_conversion =
            meter(o.registration, "POWERCONVERSION:ELECTRICITYPRODUCED")?;
        self.input_state.meter_num_electric_produced =
            meter(o.registration, "ELECTRICITYPRODUCED:FACILITY")?;
        self.input_state.meter_num_electric_purchased =
            meter(o.registration, "ELECTRICITYPURCHASED:FACILITY")?;
        self.input_state.meter_num_electric_surplus_sold =
            meter(o.registration, "ELECTRICITYSURPLUSSOLD:FACILITY")?;
        if self.input_state.meter_num_electric_purchased == 0 {
            self.input_state.meter_num_electric_purchased =
                meter(o.registration, "ELECTRICITY:FACILITY")?;
        }
        self.input_state.gather_power_fuel_fire_gen = 0.0;
        self.input_state.gather_power_pv = 0.0;
        self.input_state.gather_power_wind = 0.0;
        self.input_state.gather_power_ht_geothermal = 0.0;
        self.input_state.gather_electric_produced = 0.0;
        self.input_state.gather_electric_purchased = 0.0;
        self.input_state.gather_electric_surplus_sold = 0.0;
        self.input_state.gather_electric_storage = 0.0;
        self.input_state.gather_power_conversion = 0.0;
        self.input_state.meter_num_water_heat_recovery =
            meter(o.registration, "HEATRECOVERY:ENERGYTRANSFER")?;
        self.input_state.meter_num_air_heat_recovery_cool =
            meter(o.registration, "HEATRECOVERYFORCOOLING:ENERGYTRANSFER")?;
        self.input_state.meter_num_air_heat_recovery_heat =
            meter(o.registration, "HEATRECOVERYFORHEATING:ENERGYTRANSFER")?;
        self.input_state.meter_num_heat_ht_geothermal =
            meter(o.registration, "HTGEOTHERMAL:HEATPRODUCED")?;
        self.input_state.meter_num_heat_solar_water = meter(o.registration, "SOLARWATER:FACILITY")?;
        self.input_state.meter_num_heat_solar_air = meter(o.registration, "HEATPRODUCED:SOLARAIR")?;
        self.input_state.gather_water_heat_recovery = 0.0;
        self.input_state.gather_air_heat_recovery_cool = 0.0;
        self.input_state.gather_air_heat_recovery_heat = 0.0;
        self.input_state.gather_heat_ht_geothermal = 0.0;
        self.input_state.gather_heat_solar_water = 0.0;
        self.input_state.gather_heat_solar_air = 0.0;
        self.input_state.meter_num_rain_water = meter(o.registration, "RAINWATER:ONSITEWATER")?;
        self.input_state.meter_num_condensate = meter(o.registration, "CONDENSATE:ONSITEWATER")?;
        self.input_state.meter_num_groundwater = meter(o.registration, "WELLWATER:ONSITEWATER")?;
        self.input_state.meter_num_mains = meter(o.registration, "MAINSWATER:FACILITY")?;
        self.input_state.meter_num_water_end_use_total = meter(o.registration, "WATER:FACILITY")?;
        self.input_state.gather_rain_water = 0.0;
        self.input_state.gather_condensate = 0.0;
        self.input_state.gather_wellwater = 0.0;
        self.input_state.gather_mains = 0.0;
        self.input_state.gather_water_end_use_total = 0.0;
        Ok(())
    }
}
