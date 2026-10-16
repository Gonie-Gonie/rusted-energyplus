//! Genuine WaterManager ReportRainfall1674-1715 on existing retained arrays.
use super::{Error, SourceRainfallMode, SourceWaterManager};
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInputState;

fn column(registry: &PredefinedRegistry, name: &str) -> Result<i32, Error> {
    registry
        .named_indices
        .get(name)
        .copied()
        .ok_or(Error::Unavailable(
            "actual monthly precipitation predefined column",
        ))
}
impl SourceWaterManager {
    /// Report the same current rainfall history even when tank count is zero.
    /// Real-entry formatting retains the existing finite fixed-format domain.
    pub(crate) fn report_source_rainfall(
        &self,
        registry: &PredefinedRegistry,
        table: &mut PredefinedStringTable,
        control: &SelectedHeatBalanceInputState,
    ) -> Result<(), Error> {
        const MONTHS: [&str; 12] = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        for (i, &month) in MONTHS.iter().enumerate() {
            table.fixed_real_entry_2(
                column(registry, "pdchMonthlyTotalPrecInWeather")?,
                month,
                self.data.rainfall.monthly_total_precipitation_weather[i],
            )?;
            table.integer_entry(
                column(registry, "pdchMonthlyTotalHrRain")?,
                month,
                self.data.rainfall.number_rainy_hours_weather[i],
            )?;
        }
        if self.data.rainfall.mode == SourceRainfallMode::RainSchedDesign {
            for (i, &month) in MONTHS.iter().enumerate() {
                table.fixed_real_entry_2(
                    column(registry, "pdchMonthlyTotalPrecInSitePrec")?,
                    month,
                    self.data.rainfall.monthly_total_precipitation_site[i],
                )?;
            }
        }
        if self.data.number_storage_tanks > 0 {
            for month in MONTHS {
                // Actual Native1697 local initialization, not a supplied value
                // substituted for a real collector record or getter.
                let acc_volume_collected_monthly = 0.0;
                if self.data.number_rain_collectors > 0 {
                    return Err(Error::Unavailable(
                        "actual positive RainCollector.VolCollectedMonthly1699",
                    ));
                }
                table.fixed_real_entry_2(
                    column(registry, "pdchMonthlyTotalRainCol")?,
                    month,
                    acc_volume_collected_monthly,
                )?;
            }
        }
        for construction in &control.construction_usage {
            if construction.type_is_eco_roof {
                // This selected EcoRoof compartment has only the genuine
                // CurrentPrecipitation member, not MonthlyIrrigation. Preserve
                // every earlier table write and stop before the first read.
                return Err(Error::Unavailable(
                    "actual positive EcoRoof MonthlyIrrigation1710",
                ));
            }
        }
        Ok(())
    }
}
