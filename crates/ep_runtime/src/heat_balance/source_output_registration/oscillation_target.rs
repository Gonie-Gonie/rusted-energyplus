//! Actual DetectOscillatingZoneTemp output field identities and ReportingThisVariable.
use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceOscillationOutputField {
    ZoneTime,
    ZoneOccupiedTime,
    ZoneDeadbandTime,
    FacilityTime,
    FacilityOccupiedTime,
    FacilityDeadbandTime,
}
impl SourceOutputRegistration {
    /// OutputProcessor4571-4607 over the same reqVars and meter periods.
    pub(crate) fn reporting_source_variable(
        &self,
        variable_name: &str,
    ) -> Result<bool, OrdinaryRadiationError> {
        let name = variable_name.to_ascii_uppercase();
        for request in &self.requested_variables {
            if request.name == name {
                return Ok(true);
            }
        }
        if let Some(&number) = self.meters.meter_map.get(&name) {
            let meter = self
                .meters
                .meters
                .get(number)
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
            for frequency in 1..7 {
                // Native TimeStep1 through Num7, excluding Year6.
                if frequency == 6 {
                    continue;
                }
                let period = &meter.periods[frequency];
                if period.report
                    || period.report_file_only
                    || period.accumulated_report
                    || period.accumulated_file_only
                {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
