//! Actual controlled-zone ventilation declarations followed by four facility declarations.
use super::*;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceGeneralRealRegistrationOptions, SourceOutputDictionaryHost, SourceOutputFileHost,
    SourceOutputRegistration, SourceOutputStoreType, SourceOutputUnits, SourceRealOutputBinding,
    SourceRealOutputDeclaration, SourceRealOutputOwners, SourceRealOutputTarget as Target,
    SourceReportVariableInput, SourceSystemReportsOutputField as Field,
    SourceZoneVentReportOutputField as ZoneField, TimeStepKind,
};
use ep_model::ReportInputs;
/// Per-call streams/metadata; the sole registration/IP/backend are separate borrows.
pub(crate) struct SourceSystemReportsOutputContext<'a, H, D> {
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) reports: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) minutes_in_time_step: i32,
    pub(crate) host: &'a mut H,
    pub(crate) eio: &'a mut dyn std::io::Write,
    pub(crate) dictionary_host: &'a mut D,
}
impl SourceRealOutputOwners for SourceSystemReports {
    fn real_output(&self, target: Target) -> Result<&f64, OrdinaryRadiationError> {
        let Target::SystemReports { field } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        Ok(match field {
            Field::Zone { index, field } => {
                let row = self
                    .zone_vent_rep_vars
                    .as_ref()
                    .and_then(|rows| rows.get(at(index).ok()?))
                    .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
                match field {
                    ZoneField::NoLoadCoolingByVent => &row.no_load_cooling_by_vent,
                    ZoneField::CoolingLoadAddedByVent => &row.cooling_load_added_by_vent,
                    ZoneField::OverheatingByVent => &row.overheating_by_vent,
                    ZoneField::CoolingLoadMetByVent => &row.cooling_load_met_by_vent,
                    ZoneField::NoLoadHeatingByVent => &row.no_load_heating_by_vent,
                    ZoneField::HeatingLoadAddedByVent => &row.heating_load_added_by_vent,
                    ZoneField::OvercoolingByVent => &row.overcooling_by_vent,
                    ZoneField::HeatingLoadMetByVent => &row.heating_load_met_by_vent,
                    ZoneField::OaMassFlow => &row.oa_mass_flow,
                    ZoneField::OaMass => &row.oa_mass,
                    ZoneField::OaVolFlowStdRho => &row.oa_vol_flow_std_rho,
                    ZoneField::OaVolStdRho => &row.oa_vol_std_rho,
                    ZoneField::OaVolFlowCrntRho => &row.oa_vol_flow_crnt_rho,
                    ZoneField::OaVolCrntRho => &row.oa_vol_crnt_rho,
                    ZoneField::MechAch => &row.mech_ach,
                    ZoneField::TargetVentilationFlowVoz => &row.target_ventilation_flow_voz,
                    ZoneField::TimeBelowVozDyn => &row.time_below_voz_dyn,
                    ZoneField::TimeAtVozDyn => &row.time_at_voz_dyn,
                    ZoneField::TimeAboveVozDyn => &row.time_above_voz_dyn,
                    ZoneField::TimeVentUnocc => &row.time_vent_unocc,
                }
            }
            Field::AnyZoneBelowTargetTime => &self.any_zone_time_below_voz_dyn,
            Field::AllZonesAtTargetTime => &self.all_zones_time_at_voz_dyn,
            Field::AnyZoneAboveTargetTime => &self.any_zone_time_above_voz_dyn,
            Field::AnyZoneUnoccupiedVentilationTime => &self.any_zone_time_vent_unocc,
        })
    }
}
impl SourceSystemReports {
    /// Native1998-2157, all controlled-zone declarations on the same report owner.
    pub(super) fn register_source_zone_ventilation_outputs<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &self,
        zone: i32,
        name: &str,
        registration: &mut SourceOutputRegistration,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        o: &mut SourceSystemReportsOutputContext<'_, H, D>,
    ) -> Result<(), Error> {
        if self.vent_loads_report_enabled {
            for (field, output_name) in [
                (
                    ZoneField::NoLoadCoolingByVent,
                    "Zone Mechanical Ventilation No Load Heat Removal Energy",
                ),
                (
                    ZoneField::CoolingLoadAddedByVent,
                    "Zone Mechanical Ventilation Cooling Load Increase Energy",
                ),
                (
                    ZoneField::OverheatingByVent,
                    "Zone Mechanical Ventilation Cooling Load Increase Due to Overheating Energy",
                ),
                (
                    ZoneField::CoolingLoadMetByVent,
                    "Zone Mechanical Ventilation Cooling Load Decrease Energy",
                ),
                (
                    ZoneField::NoLoadHeatingByVent,
                    "Zone Mechanical Ventilation No Load Heat Addition Energy",
                ),
                (
                    ZoneField::HeatingLoadAddedByVent,
                    "Zone Mechanical Ventilation Heating Load Increase Energy",
                ),
                (
                    ZoneField::OvercoolingByVent,
                    "Zone Mechanical Ventilation Heating Load Increase Due to Overcooling Energy",
                ),
                (
                    ZoneField::HeatingLoadMetByVent,
                    "Zone Mechanical Ventilation Heating Load Decrease Energy",
                ),
            ] {
                self.register_source_zone_ventilation_output(
                    zone,
                    name,
                    field,
                    output_name,
                    SourceOutputUnits::J,
                    SourceOutputStoreType::Sum,
                    registration,
                    input,
                    backend,
                    o,
                )?;
            }
        }
        for (field, output_name, units, store_type) in [
            (
                ZoneField::OaMassFlow,
                "Zone Mechanical Ventilation Mass Flow Rate",
                SourceOutputUnits::KgPerSecond,
                SourceOutputStoreType::Average,
            ),
            (
                ZoneField::OaMass,
                "Zone Mechanical Ventilation Mass",
                SourceOutputUnits::Kg,
                SourceOutputStoreType::Sum,
            ),
            (
                ZoneField::OaVolFlowStdRho,
                "Zone Mechanical Ventilation Standard Density Volume Flow Rate",
                SourceOutputUnits::M3PerSecond,
                SourceOutputStoreType::Average,
            ),
            (
                ZoneField::OaVolStdRho,
                "Zone Mechanical Ventilation Standard Density Volume",
                SourceOutputUnits::M3,
                SourceOutputStoreType::Sum,
            ),
            (
                ZoneField::OaVolFlowCrntRho,
                "Zone Mechanical Ventilation Current Density Volume Flow Rate",
                SourceOutputUnits::M3PerSecond,
                SourceOutputStoreType::Average,
            ),
            (
                ZoneField::OaVolCrntRho,
                "Zone Mechanical Ventilation Current Density Volume",
                SourceOutputUnits::M3,
                SourceOutputStoreType::Sum,
            ),
            (
                ZoneField::MechAch,
                "Zone Mechanical Ventilation Air Changes per Hour",
                SourceOutputUnits::Ach,
                SourceOutputStoreType::Average,
            ),
            (
                ZoneField::TargetVentilationFlowVoz,
                "Zone Target Voz Ventilation Flow Rate",
                SourceOutputUnits::M3PerSecond,
                SourceOutputStoreType::Average,
            ),
            (
                ZoneField::TimeBelowVozDyn,
                "Zone Ventilation Below Target Voz Time",
                SourceOutputUnits::Hr,
                SourceOutputStoreType::Sum,
            ),
            (
                ZoneField::TimeAtVozDyn,
                "Zone Ventilation At Target Voz Time",
                SourceOutputUnits::Hr,
                SourceOutputStoreType::Sum,
            ),
            (
                ZoneField::TimeAboveVozDyn,
                "Zone Ventilation Above Target Voz Time",
                SourceOutputUnits::Hr,
                SourceOutputStoreType::Sum,
            ),
            (
                ZoneField::TimeVentUnocc,
                "Zone Ventilation When Unoccupied Time",
                SourceOutputUnits::Hr,
                SourceOutputStoreType::Sum,
            ),
        ] {
            self.register_source_zone_ventilation_output(
                zone,
                name,
                field,
                output_name,
                units,
                store_type,
                registration,
                input,
                backend,
                o,
            )?;
        }
        Ok(())
    }

    fn register_source_zone_ventilation_output<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &self,
        zone: i32,
        key: &str,
        field: ZoneField,
        name: &str,
        units: SourceOutputUnits,
        store_type: SourceOutputStoreType,
        registration: &mut SourceOutputRegistration,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        o: &mut SourceSystemReportsOutputContext<'_, H, D>,
    ) -> Result<(), Error> {
        let binding = SourceRealOutputBinding::new(
            Target::SystemReports {
                field: Field::Zone { index: zone, field },
            },
            self,
        )
        .map_err(Error::Output)?;
        let mut report_input =
            SourceReportVariableInput::new(o.reports, input, o.minimum_report_frequency);
        registration
            .setup_bound_general_real(
                &mut report_input,
                o.prescan,
                SourceRealOutputDeclaration {
                    name,
                    key,
                    units,
                    store_type,
                },
                o.minutes_in_time_step,
                o.host,
                o.eio,
                binding,
                o.dictionary_host,
                SourceGeneralRealRegistrationOptions {
                    time_step: TimeStepKind::System,
                    meter: None,
                },
                backend,
            )
            .map_err(Error::Output)?;
        Ok(())
    }

    pub(super) fn register_source_facility_ventilation_outputs<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &self,
        registration: &mut SourceOutputRegistration,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        o: &mut SourceSystemReportsOutputContext<'_, H, D>,
    ) -> Result<(), Error> {
        for (field, name) in [
            (
                Field::AnyZoneBelowTargetTime,
                "Facility Any Zone Ventilation Below Target Voz Time",
            ),
            (
                Field::AllZonesAtTargetTime,
                "Facility All Zones Ventilation At Target Voz Time",
            ),
            (
                Field::AnyZoneAboveTargetTime,
                "Facility Any Zone Ventilation Above Target Voz Time",
            ),
            (
                Field::AnyZoneUnoccupiedVentilationTime,
                "Facility Any Zone Ventilation When Unoccupied Time",
            ),
        ] {
            let binding = SourceRealOutputBinding::new(Target::SystemReports { field }, self)
                .map_err(Error::Output)?;
            let mut report_input =
                SourceReportVariableInput::new(o.reports, input, o.minimum_report_frequency);
            registration
                .setup_bound_general_real(
                    &mut report_input,
                    o.prescan,
                    SourceRealOutputDeclaration {
                        name,
                        key: "Facility",
                        units: SourceOutputUnits::Hr,
                        store_type: SourceOutputStoreType::Sum,
                    },
                    o.minutes_in_time_step,
                    o.host,
                    o.eio,
                    binding,
                    o.dictionary_host,
                    SourceGeneralRealRegistrationOptions {
                        time_step: TimeStepKind::System,
                        meter: None,
                    },
                    backend,
                )
                .map_err(Error::Output)?;
        }
        Ok(())
    }
}
