//! Genuine GatherBEPSResultsForTimestep3824–3960 on the sole TAB and OP owners.
use super::*;
impl SourceTabularReports {
    pub(crate) fn gather_source_beps_results(
        &mut self,
        time_step: TimeStepKind,
        o: &SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        if !(self.display_tabular_beps || self.display_leed_summary)
            || time_step != TimeStepKind::Zone
        {
            return Ok(());
        }
        let s = &mut self.input_state;
        for resource in 1..=14 {
            let meter = s.meter_num_totals_beps[resource - 1];
            if meter > -1 {
                let value = o.registration.current_source_meter_value(meter)?;
                s.gather_totals_beps[resource - 1] += value;
            }
            for end_use in 1..=14 {
                let meter = s.meter_num_end_use_beps.read([resource, end_use])?;
                if meter > -1 {
                    let value = o.registration.current_source_meter_value(meter)?;
                    let old = s.gather_end_use_beps.read([resource, end_use])?;
                    s.gather_end_use_beps
                        .set([resource, end_use], old + value)?;
                    let category = o
                        .registration
                        .end_use_categories
                        .as_ref()
                        .and_then(|v| v.get(end_use - 1))
                        .ok_or(Error::Unavailable("actual BEPS EndUseCategory3913"))?;
                    for ordinal in 1..=category.num_subcategories {
                        let subcategory = usize::try_from(ordinal)
                            .map_err(|_| Error::Unavailable("actual BEPS subcategory ordinal"))?;
                        let indices = [subcategory, end_use, resource];
                        let meter = s.meter_num_end_use_sub_beps.read(indices)?;
                        if meter > -1 {
                            let value = o.registration.current_source_meter_value(meter)?;
                            let old = s.gather_end_use_sub_beps.read(indices)?;
                            s.gather_end_use_sub_beps.set(indices, old + value)?;
                        }
                    }
                    for ordinal in 1..=category.num_space_types {
                        let space_type = usize::try_from(ordinal)
                            .map_err(|_| Error::Unavailable("actual BEPS space-type ordinal"))?;
                        let indices = [space_type, end_use, resource];
                        let meter = s.meter_num_end_use_space_type_beps.read(indices)?;
                        if meter > -1 {
                            let value = o.registration.current_source_meter_value(meter)?;
                            let old = s.gather_end_use_space_type_beps.read(indices)?;
                            s.gather_end_use_space_type_beps.set(indices, old + value)?;
                        }
                    }
                }
            }
        }
        for resource in 1..=12 {
            let meter = s.meter_num_totals_source[resource - 1];
            if meter > -1 {
                let value = o.registration.current_source_meter_value(meter)?;
                s.gather_totals_source[resource - 1] += value;
            }
        }
        s.gather_power_fuel_fire_gen += o
            .registration
            .current_source_meter_value(s.meter_num_power_fuel_fire_gen)?;
        s.gather_power_pv += o
            .registration
            .current_source_meter_value(s.meter_num_power_pv)?;
        s.gather_power_wind += o
            .registration
            .current_source_meter_value(s.meter_num_power_wind)?;
        s.gather_power_ht_geothermal += o
            .registration
            .current_source_meter_value(s.meter_num_power_ht_geothermal)?;
        s.gather_electric_produced += o
            .registration
            .current_source_meter_value(s.meter_num_electric_produced)?;
        s.gather_electric_purchased += o
            .registration
            .current_source_meter_value(s.meter_num_electric_purchased)?;
        s.gather_electric_surplus_sold += o
            .registration
            .current_source_meter_value(s.meter_num_electric_surplus_sold)?;
        s.gather_electric_storage += o
            .registration
            .current_source_meter_value(s.meter_num_electric_storage)?;
        s.gather_power_conversion += o
            .registration
            .current_source_meter_value(s.meter_num_power_conversion)?;
        s.gather_water_heat_recovery += o
            .registration
            .current_source_meter_value(s.meter_num_water_heat_recovery)?;
        s.gather_air_heat_recovery_cool += o
            .registration
            .current_source_meter_value(s.meter_num_air_heat_recovery_cool)?;
        s.gather_air_heat_recovery_heat += o
            .registration
            .current_source_meter_value(s.meter_num_air_heat_recovery_heat)?;
        s.gather_heat_ht_geothermal += o
            .registration
            .current_source_meter_value(s.meter_num_heat_ht_geothermal)?;
        s.gather_heat_solar_water += o
            .registration
            .current_source_meter_value(s.meter_num_heat_solar_water)?;
        s.gather_heat_solar_air += o
            .registration
            .current_source_meter_value(s.meter_num_heat_solar_air)?;
        s.gather_rain_water += o
            .registration
            .current_source_meter_value(s.meter_num_rain_water)?;
        s.gather_condensate += o
            .registration
            .current_source_meter_value(s.meter_num_condensate)?;
        s.gather_wellwater += o
            .registration
            .current_source_meter_value(s.meter_num_groundwater)?;
        s.gather_mains += o
            .registration
            .current_source_meter_value(s.meter_num_mains)?;
        s.gather_water_end_use_total += o
            .registration
            .current_source_meter_value(s.meter_num_water_end_use_total)?;
        Ok(())
    }
}
