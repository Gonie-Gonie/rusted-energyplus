//! Genuine HeatBalanceAirManager input and simple mixing, same retained owners.
use super::super::environment::SourceZoneFanHeatBalanceArrays;
use super::*;
use crate::heat_balance::selected_input_processor::SourceInputFamily;
use std::convert::Infallible;
mod manage;
mod mean_air_report;
mod output;
pub(crate) use manage::SourceManageAirHeatBalanceChildren;
pub(crate) use mean_air_report::SourceMeanAirReportOwners;
mod state;
pub(crate) use output::{SourceAirReportOutputOwners, SourceAirReportRegistrationContext};
pub(crate) use state::SourceSimpleAir;

pub(crate) struct SourceSimpleAirInputOwners<'a, 'backend> {
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) zones: &'a mut SelectedZoneInputOwners,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) reporting_diagnostic_flags:
        Option<crate::heat_balance::error_backend::SourceDiagnosticFlags<'a>>,
    pub(crate) space_controls: &'a SelectedHeatBalanceSpaceControls,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) fan: &'a mut SourceZoneFanHeatBalanceArrays,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}

/// The actual empty setupIHGZonesAndSpaces3300 prefix. A prior definition query
/// may have inserted null; exact epJSON.find distinguishes that from absence.
/// Positive schema/alpha/Zone/Space expansion remains at its first real child.
fn setup_empty_air_instances(
    input: &SelectedInputProcessor,
    family: &str,
    num_input_objects: &mut i32,
    num_instances: &mut i32,
) -> Result<Option<Vec<Infallible>>, Error> {
    match input.find_exact_family(family) {
        None => Ok(None),
        Some(SourceInputFamily::Null) => {
            *num_input_objects = 0;
            let objects = Some(Vec::new());
            *num_instances = 0;
            Ok(objects)
        }
        Some(SourceInputFamily::Objects(rows)) => {
            if !rows.is_empty() {
                return Err(Error::Unavailable(
                    "positive setupIHGZonesAndSpaces3316 actual schema/alpha expansion",
                ));
            }
            *num_input_objects = 0;
            let objects = Some(Vec::new());
            *num_instances = 0;
            Ok(objects)
        }
    }
}
fn allocate_empty(
    collection: &mut Option<Vec<Infallible>>,
    total: i32,
    operation: &'static str,
) -> Result<(), Error> {
    if collection.is_some() || total != 0 {
        return Err(Error::Unavailable(operation));
    }
    *collection = Some(Vec::new());
    Ok(())
}
impl SourceZoneAirPredictorCorrector {
    /// Actual GetAirHeatBalanceInput163-189. The caller clears its input flag
    /// only after this entire genuine child returns, never after a prefix.
    pub(crate) fn get_source_air_heat_balance_input<H, D>(
        &mut self,
        owners: &mut SourceSimpleAirInputOwners<'_, '_>,
        output: &mut SourceAirReportRegistrationContext<'_, H, D>,
    ) -> Result<(), Error>
    where
        H: crate::heat_balance::source_output_registration::SourceOutputFileHost,
        D: crate::heat_balance::source_output_registration::SourceOutputDictionaryHost,
    {
        let errors_found = false;
        self.simple_air.air_flow_flag = true; // GetAirFlowFlag205 precedes292.
        self.get_source_simple_air_model_inputs(owners, output)?;
        let total = self
            .simple_air
            .total_infiltration
            .checked_add(self.simple_air.total_ventilation)
            .and_then(|v| v.checked_add(self.simple_air.total_mixing))
            .and_then(|v| v.checked_add(self.simple_air.total_cross_mixing))
            .and_then(|v| v.checked_add(self.simple_air.total_refrigeration_door_mixing))
            .ok_or(Error::Unavailable(
                "defined Source simple airflow count sum208",
            ))?;
        if total > 0 {
            output
                .eio
                .write_all(b"! <AirFlow Model>, Simple\n AirFlow Model, Simple\n")?;
        }
        self.simple_air
            .set_source_mass_conservation_flags(owners.control, owners.fan)?;
        self.subsystems.room_air.get_source_model_parameters(
            owners.zones,
            owners.globals,
            owners.space_controls,
            owners.input,
            owners.shortcut,
            output.eio,
        )?;
        if errors_found {
            return match owners
                .backend
                .show_fatal_error("GetAirHeatBalanceInput: Errors found in getting Air inputs")
            {
                Err(e) => Err(Error::Fatal(e)),
                Ok(never) => match never {},
            };
        }
        Ok(())
    }

    fn get_source_simple_air_model_inputs<H, D>(
        &mut self,
        owners: &mut SourceSimpleAirInputOwners<'_, '_>,
        output: &mut SourceAirReportRegistrationContext<'_, H, D>,
    ) -> Result<(), Error>
    where
        H: crate::heat_balance::source_output_registration::SourceOutputFileHost,
        D: crate::heat_balance::source_output_registration::SourceOutputDictionaryHost,
    {
        let n = usize::try_from(owners.globals.num_of_zones)
            .map_err(|_| Error::Unavailable("actual RepVarSet289 extent"))?;
        let mut rep_var_set = vec![true; n];
        self.reports
            .allocate_source_reports(owners.globals, owners.space_controls)?;
        output.register_source_air_reports(self, owners)?; // actual297-382, same live fields
        let mut max_alpha = 0;
        let mut max_number = 0;
        for family in [
            "ZoneAirBalance:OutdoorAir",
            "ZoneInfiltration:EffectiveLeakageArea",
            "ZoneInfiltration:FlowCoefficient",
            "ZoneInfiltration:DesignFlowRate",
            "ZoneVentilation:DesignFlowRate",
            "ZoneVentilation:WindandStackOpenArea",
            "ZoneMixing",
            "ZoneCrossMixing",
            "ZoneRefrigerationDoorMixing",
        ] {
            let counts = owners
                .input
                .get_object_def_max_args(family)
                .map_err(|_| Error::Unavailable("actual simple air definition385-420"))?;
            max_alpha = max_alpha.max(counts.alphas);
            max_number = max_number.max(counts.numerics);
        }
        let alpha_args = vec![String::new(); max_alpha];
        let alpha_field_names = vec![String::new(); max_alpha];
        let numeric_field_names = vec![String::new(); max_number];
        let numeric_args = vec![0.0; max_number];
        let alpha_blanks = vec![true; max_alpha];
        let numeric_blanks = vec![true; max_number];
        self.simple_air.total_zone_air_balance = i32::try_from(
            owners
                .input
                .get_num_objects_found("ZoneAirBalance:OutdoorAir"),
        )
        .map_err(|_| Error::Unavailable("actual TotZoneAirBalance430"))?;
        if self.simple_air.zone_air_balance.is_some() {
            return Err(Error::Unavailable(
                "repeated native ZoneAirBalance allocate432",
            ));
        }
        self.simple_air.zone_air_balance =
            Some(vec![
                state::SourceZoneAirBalanceData::default();
                usize::try_from(self.simple_air.total_zone_air_balance)
                    .map_err(|_| Error::Unavailable(
                        "actual ZoneAirBalance extent432"
                    ))?
            ]);
        if self.simple_air.total_zone_air_balance > 0 {
            return Err(Error::Unavailable(
                "positive ZoneAirBalance getObjectItem435",
            ));
        }
        let mut infil_design_objects = 0;
        let mut infil_design = 0;
        let _infil_design = setup_empty_air_instances(
            owners.input,
            "ZoneInfiltration:DesignFlowRate",
            &mut infil_design_objects,
            &mut infil_design,
        )?;
        let mut infil_leakage_objects = 0;
        let mut infil_leakage = 0;
        let _infil_leakage = setup_empty_air_instances(
            owners.input,
            "ZoneInfiltration:EffectiveLeakageArea",
            &mut infil_leakage_objects,
            &mut infil_leakage,
        )?;
        let mut infil_coefficient_objects = 0;
        let mut infil_coefficient = 0;
        let _infil_coefficient = setup_empty_air_instances(
            owners.input,
            "ZoneInfiltration:FlowCoefficient",
            &mut infil_coefficient_objects,
            &mut infil_coefficient,
        )?;
        self.simple_air.total_infiltration = infil_design
            .checked_add(infil_leakage)
            .and_then(|v| v.checked_add(infil_coefficient))
            .ok_or(Error::Unavailable("defined TotInfiltration675 sum"))?;
        allocate_empty(
            &mut self.simple_air.infiltration,
            self.simple_air.total_infiltration,
            "actual empty Infiltration allocate676",
        )?;
        // reserve(0) leaves the same constructor-empty unordered-map contents.
        if self.simple_air.total_infiltration != 0 {
            return Err(Error::Unavailable(
                "positive UniqueInfiltrationNames reserve677",
            ));
        }
        rep_var_set.fill(true); //1345 after all actual empty infiltration loops
        let mut vent_design_objects = 0;
        let mut vent_design = 0;
        let _vent_design = setup_empty_air_instances(
            owners.input,
            "ZoneVentilation:DesignFlowRate",
            &mut vent_design_objects,
            &mut vent_design,
        )?;
        let mut vent_wind_objects = 0;
        let mut vent_wind = 0;
        let _vent_wind = setup_empty_air_instances(
            owners.input,
            "ZoneVentilation:WindandStackOpenArea",
            &mut vent_wind_objects,
            &mut vent_wind,
        )?;
        self.simple_air.total_ventilation = vent_design
            .checked_add(vent_wind)
            .ok_or(Error::Unavailable("defined TotVentilation1366 sum"))?;
        allocate_empty(
            &mut self.simple_air.ventilation,
            self.simple_air.total_ventilation,
            "actual empty Ventilation allocate1367",
        )?;
        rep_var_set.fill(true); //2420 after genuine empty ventilation branches
        let mut mixing_objects = 0;
        let _mixing = setup_empty_air_instances(
            owners.input,
            "ZoneMixing",
            &mut mixing_objects,
            &mut self.simple_air.total_mixing,
        )?;
        if self.simple_air.total_mixing > 0 {
            return Err(Error::Unavailable("positive Mixing allocate2434"));
        }
        if self.simple_air.mass_conservation.is_some() {
            return Err(Error::Unavailable(
                "repeated native MassConservation allocate2831",
            ));
        }
        self.simple_air.mass_conservation =
            Some(vec![state::SourceZoneMassConservationData::default(); n]);
        self.simple_air
            .write_source_zone_reorder(owners.globals, owners.fan)?;
        let mut cross_objects = 0;
        let mut cross_total = 0;
        let _cross = setup_empty_air_instances(
            owners.input,
            "ZoneCrossMixing",
            &mut cross_objects,
            &mut cross_total,
        )?;
        let boundary_count = i32::try_from(self.simple_air.air_boundary_mixing.len())
            .map_err(|_| Error::Unavailable("actual airBoundaryMixing size2935"))?;
        self.simple_air.total_cross_mixing = cross_total
            .checked_add(boundary_count)
            .ok_or(Error::Unavailable("defined TotCrossMixing2935 sum"))?;
        if self.simple_air.total_cross_mixing > 0 {
            return Err(Error::Unavailable("positive CrossMixing allocate2939"));
        }
        self.simple_air.total_refrigeration_door_mixing = i32::try_from(
            owners
                .input
                .get_num_objects_found("ZoneRefrigerationDoorMixing"),
        )
        .map_err(|_| Error::Unavailable("actual TotRefDoorMixing3454"))?;
        if self.simple_air.total_refrigeration_door_mixing > 0 {
            return Err(Error::Unavailable("positive RefDoorMixing allocate3456"));
        }
        drop(rep_var_set);
        drop(alpha_args);
        drop(alpha_field_names);
        drop(numeric_field_names);
        drop(numeric_args);
        drop(alpha_blanks);
        drop(numeric_blanks); //3887-3893 exact order
        let total_infil_vent_flow = vec![0.0; n]; //3895; all positive reporting loops empty
        if owners.control.enforce_zone_mass_balance && self.simple_air.total_infiltration > 0 {
            return Err(Error::Unavailable(
                "positive Infiltration MassConservation pointer3963",
            ));
        }
        let total_mixing_flow = vec![0.0; n]; //4057 after empty infiltration and ventilation loops
        for number in 1..=owners.globals.num_of_zones {
            let i = index(number)?;
            let zone = owners
                .zones
                .zones
                .as_mut()
                .and_then(|v| v.get_mut(i))
                .ok_or(Error::Unavailable("actual nominal Zone4157"))?;
            zone.nominal_infiltration_ventilation = total_infil_vent_flow[i];
            zone.nominal_mixing = total_mixing_flow[i];
        }
        if owners.control.enforce_zone_mass_balance {
            // With genuine zero-mixing rows the earlier diagnostic predicates are false.
            // The first nonempty omitted child is the actual mass-balance output4177.
            return Err(Error::Unavailable(
                "positive enforced mass-balance report SetupOutputVariable4177",
            ));
        }
        drop(total_infil_vent_flow);
        drop(total_mixing_flow); //4240/4241
        Ok(())
    }
}

impl SourceSimpleAir {
    fn write_source_zone_reorder(
        &self,
        globals: &SelectedHeatBalanceGlobals,
        fan: &mut SourceZoneFanHeatBalanceArrays,
    ) -> Result<(), Error> {
        let mut next = 0;
        for selection in 0..3 {
            for number in 1..=globals.num_of_zones {
                let row = self
                    .mass_conservation
                    .as_deref()
                    .and_then(|v| v.get(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual MassConservation reorder2905"))?;
                let selected = match selection {
                    0 => !row.is_only_source_zone && !row.is_source_and_receiving_zone,
                    1 => row.is_source_and_receiving_zone,
                    _ => row.is_only_source_zone,
                };
                if selected {
                    *fan.zone_reorder
                        .as_mut()
                        .and_then(|v| v.get_mut(next))
                        .ok_or(Error::Unavailable("actual same ZoneReOrder2908"))? = Some(number);
                    next += 1;
                }
            }
        }
        Ok(())
    }
    fn set_source_mass_conservation_flags(
        &self,
        control: &SelectedHeatBalanceInputState,
        _fan: &mut SourceZoneFanHeatBalanceArrays,
    ) -> Result<(), Error> {
        if control.enforce_zone_mass_balance
            && self.mass_flow_control.zone_flow_adjustment != 4
            && self.total_mixing > 0
        {
            return Err(Error::Unavailable(
                "positive Mixing ZonePtr mass-balance flag229",
            ));
        }
        Ok(())
    }
    /// Actual InitAirHeatBalance4506 -> InitSimpleMixingConvectiveHeatGains.
    pub(crate) fn initialize_source_simple_mixing(
        &mut self,
        control: &SelectedHeatBalanceInputState,
    ) -> Result<(), Error> {
        if self.air_flow_flag {
            if self.mixing.as_deref().is_some_and(|v| !v.is_empty()) {
                return Err(Error::Unavailable(
                    "positive actual mixing schedule getCurrentVal4530",
                ));
            }
            if control.enforce_zone_mass_balance {
                for row in self.mass_conservation.as_mut().into_iter().flatten() {
                    let flow_sum = 0.0;
                    if row.num_receiving_zones_mixing_object > 0 {
                        // No represented positive Mixing row may be read as a zero value.
                        return Err(Error::Unavailable("positive mixing DesignLevel4544"));
                    }
                    if flow_sum > 0.0 {
                        return Err(Error::Unavailable("positive mixing fraction4549"));
                    }
                    // The admitted actual receiving count is zero, so no fraction cell is touched.
                }
            }
            if self.cross_mixing.as_deref().is_some_and(|v| !v.is_empty()) {
                return Err(Error::Unavailable(
                    "positive actual cross-mixing schedule4557",
                ));
            }
            if self.total_refrigeration_door_mixing > 0 {
                return Err(Error::Unavailable("positive refrigeration-door row4571"));
            }
        }
        Ok(())
    }
}
