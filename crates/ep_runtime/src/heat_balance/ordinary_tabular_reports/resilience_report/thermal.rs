//! Full Native13417-13656 annual thermal body; SI and same live format passes.
use super::*;
impl SourceTabularReports {
    pub(crate) fn write_source_thermal_resilience_tables(
        &mut self,
        o: &mut SourceResilienceTableOwners<'_, '_>,
    ) -> Result<(), Error> {
        if o.globals.num_of_zones <= 0 {
            return Ok(());
        }
        for pass in 0..self.input_state.tabular_report_passes.len() {
            if matches!(
                self.input_state.tabular_report_passes[pass].units_style,
                4 | 5
            ) {
                return Err(Error::Unavailable(
                    "actual ThermalResilience13429 getSpecificUnitMultiplier Celsius-hour to Fahrenheit-hour first operation",
                ));
            }
            let degree_hour_conversion = 1.0;
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                self.write_source_report_headers(
                    b"Annual Thermal Resilience Summary",
                    b"Entire Facility",
                    SourceOutputStoreType::Average,
                )?;
            }
            let zones = count(o.globals.num_of_zones)?;
            let row_count = count(o.globals.num_of_zones.checked_add(4).ok_or(
                Error::Unavailable("actual thermal row allocation signed extent"),
            )?)?;
            let widths = vec![10; 5];
            let mut rows = vec![Vec::new(); row_count];
            let body = SourceTabularBody::allocate(5, row_count)?;
            let mut c = Columns {
                headings: vec![Vec::new(); 5],
                widths,
                body,
            };
            c.headings(HEAT_INDEX)?;
            self.resilience_bins(
                pass,
                b"Heat Index Hours",
                Field::HeatIndex,
                1.0,
                &mut rows,
                &mut c,
                o,
            )?;
            self.resilience_bins(
                pass,
                b"Heat Index OccupantHours",
                Field::HeatIndexOccu,
                1.0,
                &mut rows,
                &mut c,
                o,
            )?;
            self.resilience_bins(
                pass,
                b"Heat Index OccupiedHours",
                Field::HeatIndexOccupied,
                1.0,
                &mut rows,
                &mut c,
                o,
            )?;
            c.headings(HUMIDEX)?;
            self.resilience_bins(
                pass,
                b"Humidex Hours",
                Field::Humidex,
                1.0,
                &mut rows,
                &mut c,
                o,
            )?;
            self.resilience_bins(
                pass,
                b"Humidex OccupantHours",
                Field::HumidexOccu,
                1.0,
                &mut rows,
                &mut c,
                o,
            )?;
            self.resilience_bins(
                pass,
                b"Humidex OccupiedHours",
                Field::HumidexOccupied,
                1.0,
                &mut rows,
                &mut c,
                o,
            )?;
            let mut has_pierce_set = true;
            if o.gains.totals[0] == 0 {
                has_pierce_set = false;
                if self.display_thermal_resilience_summary_explicitly {
                    o.writer.backend.show_warning_error("Writing Annual Thermal Resilience Summary - SET Degree-Hours reports: Zone Thermal Comfort Pierce Model Standard Effective Temperature is required, but no People object is defined.")?;
                }
            }
            for people in 1..=o.gains.totals[0] {
                let person =
                    o.gains
                        .allocations
                        .people
                        .get(index(people)?)
                        .ok_or(Error::Unavailable(
                            "actual ThermalResilience13511 People.Pierce row",
                        ))?;
                if !person.pierce {
                    has_pierce_set = false;
                    if self.display_thermal_resilience_summary_explicitly {
                        o.writer.backend.show_warning_error(&format!("Writing Annual Thermal Resilience Summary - SET Degree-Hours reports: Zone Thermal Comfort Pierce Model Standard Effective Temperature is required, but no Pierce model is defined in {} object.",person.name))?;
                    }
                }
            }
            if has_pierce_set {
                // Native applies the same >30C column headings to both tables.
                c.headings(SET)?;
                let mut set_rows = vec![
                    Vec::new();
                    count(
                        o.globals
                            .num_of_zones
                            .checked_add(3)
                            .ok_or(Error::Unavailable("actual SET row extent"))?
                    )?
                ];
                let mut set_c = Columns {
                    headings: c.headings.clone(),
                    widths: Vec::new(),
                    body: SourceTabularBody::allocate(5, set_rows.len())?,
                };
                // The very same columnWidth array carries WriteTable mutations
                // from prior thermal tables and both SET helpers into safety.
                std::mem::swap(&mut set_c.widths, &mut c.widths);
                self.resilience_set(
                    pass,
                    b"Heating SET Degree-Hours",
                    Field::LowSet,
                    degree_hour_conversion,
                    &mut set_rows,
                    &mut set_c,
                    o,
                )?;
                self.resilience_set(
                    pass,
                    b"Cooling SET Degree-Hours",
                    Field::HighSet,
                    degree_hour_conversion,
                    &mut set_rows,
                    &mut set_c,
                    o,
                )?;
                std::mem::swap(&mut set_c.widths, &mut c.widths);
            }
            c.headings(SAFETY)?;
            self.resilience_safety(
                pass,
                b"Hours of Safety for Cold Events",
                Field::ColdSafety,
                &mut rows,
                &mut c,
                o,
            )?;
            self.resilience_safety(
                pass,
                b"Hours of Safety for Heat Events",
                Field::HeatSafety,
                &mut rows,
                &mut c,
                o,
            )?;
            let mut unmet = Columns::allocate(6, row_count)?;
            unmet.headings(UNMET)?;
            self.resilience_bins(
                pass,
                b"Unmet Degree-Hours",
                Field::Unmet,
                degree_hour_conversion,
                &mut rows,
                &mut unmet,
                o,
            )?;
            // Native13631-13635 reallocates these same three local arrays.
            c.headings = vec![Vec::new(); 4];
            c.widths = vec![10; 4];
            c.body = SourceTabularBody::allocate(4, row_count)?;
            c.headings(DISCOMFORT)?;
            self.resilience_bins(
                pass,
                b"Discomfort-weighted Exceedance OccupantHours",
                Field::DiscomfortOccu,
                1.0,
                &mut rows,
                &mut c,
                o,
            )?;
            self.resilience_bins(
                pass,
                b"Discomfort-weighted Exceedance OccupiedHours",
                Field::DiscomfortOccupied,
                1.0,
                &mut rows,
                &mut c,
                o,
            )?;
        }
        Ok(())
    }
}
const HEAT_INDEX: &[&str] = &[
    "Safe (≤ 26.7°C) [hr]",
    "Caution (> 26.7°C, ≤ 32.2°C) [hr]",
    "Extreme Caution (> 32.2°C, ≤ 39.4°C) [hr]",
    "Danger (> 39.4°C, ≤ 51.7°C) [hr]",
    "Extreme Danger (> 51.7°C) [hr]",
];
const HUMIDEX: &[&str] = &[
    "Little to no Discomfort (≤ 29) [hr]",
    "Some Discomfort (> 29, ≤ 40) [hr]",
    "Great Discomfort; Avoid Exertion (> 40, ≤ 45) [hr]",
    "Dangerous (> 45, ≤ 50) [hr]",
    "Heat Stroke Quite Possible (> 50) [hr]",
];
const SET: &[&str] = &[
    "SET > 30°C Degree-Hours [°C·hr]",
    "SET > 30°C Occupant-Weighted Degree-Hours [°C·hr]",
    "SET > 30°C Occupied Degree-Hours [°C·hr]",
    "Longest SET > 30°C Duration for Occupied Period [hr]",
    "Start Time of the Longest SET > 30°C Duration for Occupied Period",
];
const SAFETY: &[&str] = &[
    "Hours of Safety [hr]",
    "End Time of the Safety Duration",
    "Safe Temperature Exceedance Hours [hr]",
    "Safe Temperature Exceedance OccupantHours [hr]",
    "Safe Temperature Exceedance OccupiedHours [hr]",
];
const UNMET: &[&str] = &[
    "Cooling Setpoint Unmet Degree-Hours [°C·hr]",
    "Cooling Setpoint Unmet Occupant-Weighted Degree-Hours [°C·hr]",
    "Cooling Setpoint Unmet Occupied Degree-Hours [°C·hr]",
    "Heating Setpoint Unmet Degree-Hours [°C·hr]",
    "Heating Setpoint Unmet Occupant-Weighted Degree-Hours [°C·hr]",
    "Heating Setpoint Unmet Occupied Degree-Hours [°C·hr]",
];
const DISCOMFORT: &[&str] = &[
    "Very-cold Exceedance OccupantHours [hr]",
    "Cool Exceedance OccupantHours [hr]",
    "Warm Exceedance OccupantHours [hr]",
    "Very-hot Exceedance OccupantHours [hr]",
];
