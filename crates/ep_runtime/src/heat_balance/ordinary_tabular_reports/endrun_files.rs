//! Actual outer tariff floor-area prerequisite and CloseOutputTabularFile3157-3209.
use super::{Error, SourceTabularReports};
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;

impl SourceTabularReports {
    pub(crate) fn prepare_source_tariff_report_floor_area(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
        zones: &SelectedZoneInputOwners,
    ) -> Result<(), Error> {
        if self.input_state.building_conditioned_floor_area == 0.0 {
            // Reuse the ONE literal DetermineBuildingFloorArea body. The
            // actual caller may clear/recompute its same totals at this point.
            self.determine_source_building_floor_area_from_owners(globals, zones)?;
        }
        Ok(())
    }

    pub(crate) fn close_source_tabular_files(&mut self) -> Result<(), Error> {
        if self.input_state.write_tabular_files {
            for ordinal in 1..=self.input_state.number_of_styles {
                let at = usize::try_from(ordinal - 1)
                    .map_err(|_| Error::Unavailable("actual CloseTAB style ordinal3194"))?;
                let stream = self
                    .output_streams
                    .streams
                    .get_mut(at)
                    .ok_or(Error::Unavailable(
                        "actual CloseTAB TabularOutputFile pointer3195",
                    ))?;
                let style = *self
                    .input_state
                    .table_styles
                    .get(at)
                    .ok_or(Error::Unavailable("actual CloseTAB TableStyle3196 read"))?;
                if style == 3 {
                    stream.bytes(b"</body>\n")?;
                    stream.bytes(b"</html>\n")?;
                } else if style == 4 {
                    if !self.input_state.prev_report_name.is_empty() {
                        stream.bytes(b"</")?;
                        stream.bytes(&self.input_state.prev_report_name)?;
                        stream.bytes(b">\n")?;
                    }
                    stream.bytes(b"</EnergyPlusTabularReports>\n")?;
                }
                stream.close();
            }
        }
        Ok(())
    }
}
