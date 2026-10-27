//! Actual eleven Facility declarations323-413 and the live retained field view.
use super::*;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceFacilityOutputField as Field, SourceGeneralMeterArguments,
    SourceGeneralRealRegistrationOptions, SourceMeterEndUse, SourceMeterGroup,
    SourceMeterResource as Resource, SourceOutputDictionaryHost, SourceOutputFileHost,
    SourceOutputStoreType as Store, SourceOutputUnits as Units, SourceRealOutputBinding,
    SourceRealOutputDeclaration, SourceRealOutputOwners, SourceRealOutputTarget as Target,
    SourceReportVariableInput, TimeStepKind,
};
use ep_model::ReportInputs;

/// Host streams/metadata only; registration and IP are borrowed per actual call.
pub(crate) struct SourceFacilityOutputContext<'a, H, D> {
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) reports: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) developer: &'a bool,
    pub(crate) minutes_in_time_step: i32,
    pub(crate) host: &'a mut H,
    pub(crate) eio: &'a mut dyn std::io::Write,
    pub(crate) dictionary_host: &'a mut D,
}
impl SourceRealOutputOwners for FacilityElectricService {
    fn real_output(&self, target: Target) -> Result<&f64, OrdinaryRadiationError> {
        let Target::Facility { field } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        Ok(match field {
            Field::PurchasedElectricityRate => &self.electricity_purchased_rate,
            Field::PurchasedElectricityEnergy => &self.electricity_purchased,
            Field::SurplusElectricityRate => &self.electricity_surplus_rate,
            Field::SurplusElectricityEnergy => &self.electricity_surplus,
            Field::NetPurchasedElectricityRate => &self.electricity_net_rate,
            Field::NetPurchasedElectricityEnergy => &self.electricity_net,
            Field::BuildingElectricityDemandRate => &self.total_building_electric_demand,
            Field::HvacElectricityDemandRate => &self.total_hvac_electric_demand,
            Field::ElectricityDemandRate => &self.total_electric_demand,
            Field::ProducedElectricityRate => &self.electricity_produced_rate,
            Field::ProducedElectricityEnergy => &self.electricity_produced,
        })
    }
}
/// Constructed for one instantaneous read, while this same facility is shared.
pub(crate) struct SourceFacilityAndOtherOutputOwners<'a> {
    pub(crate) facility: &'a FacilityElectricService,
    pub(crate) other: &'a dyn SourceRealOutputOwners,
}
impl SourceRealOutputOwners for SourceFacilityAndOtherOutputOwners<'_> {
    fn real_output(&self, target: Target) -> Result<&f64, OrdinaryRadiationError> {
        match target {
            Target::Facility { .. } => self.facility.real_output(target),
            _ => self.other.real_output(target),
        }
    }
}
impl FacilityElectricService {
    pub(super) fn register_source_facility_outputs<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &self,
        o: &mut SourceFacilityElectricOwners<'_, '_>,
        output: &mut SourceFacilityOutputContext<'_, H, D>,
    ) -> Result<(), Error> {
        for (field, name, units, store, resource) in [
            (
                Field::PurchasedElectricityRate,
                "Facility Total Purchased Electricity Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::PurchasedElectricityEnergy,
                "Facility Total Purchased Electricity Energy",
                Units::J,
                Store::Sum,
                Some(Resource::ElectricityPurchased),
            ),
            (
                Field::SurplusElectricityRate,
                "Facility Total Surplus Electricity Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::SurplusElectricityEnergy,
                "Facility Total Surplus Electricity Energy",
                Units::J,
                Store::Sum,
                Some(Resource::ElectricitySurplusSold),
            ),
            (
                Field::NetPurchasedElectricityRate,
                "Facility Net Purchased Electricity Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::NetPurchasedElectricityEnergy,
                "Facility Net Purchased Electricity Energy",
                Units::J,
                Store::Sum,
                Some(Resource::ElectricityNet),
            ),
            (
                Field::BuildingElectricityDemandRate,
                "Facility Total Building Electricity Demand Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::HvacElectricityDemandRate,
                "Facility Total HVAC Electricity Demand Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::ElectricityDemandRate,
                "Facility Total Electricity Demand Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::ProducedElectricityRate,
                "Facility Total Produced Electricity Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::ProducedElectricityEnergy,
                "Facility Total Produced Electricity Energy",
                Units::J,
                Store::Sum,
                None,
            ),
        ] {
            let binding = SourceRealOutputBinding::new(Target::Facility { field }, self)
                .map_err(Error::Output)?;
            let mut input = SourceReportVariableInput::new(
                output.reports,
                o.input,
                crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::from_owners(output.minimum_report_frequency, o.globals, output.developer, Some(o.clock)),
                o.backend,
            );
            let meter = resource.map(|resource| SourceGeneralMeterArguments {
                resource,
                end_use: SourceMeterEndUse::Cogeneration,
                end_use_subcategory: "",
                group: SourceMeterGroup::Plant,
                zone_name: "",
                space_type: "",
                zone_multiplier: 1,
                zone_list_multiplier: 1,
            });
            o.registration
                .setup_bound_general_real(
                    &mut input,
                    output.prescan,
                    SourceRealOutputDeclaration {
                        name,
                        key: &self.name,
                        units,
                        store_type: store,
                    },
                    output.minutes_in_time_step,
                    output.host,
                    output.eio,
                    binding,
                    output.dictionary_host,
                    SourceGeneralRealRegistrationOptions {
                        time_step: TimeStepKind::System,
                        meter,
                    },
                )
                .map_err(Error::Output)?;
        }
        Ok(())
    }
    pub(super) fn report_source_pv_and_wind_capacity(
        &mut self,
        o: &mut SourceFacilityElectricOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.pv_total_capacity = 0.0;
        self.wind_total_capacity = 0.0;
        for load_center in &self.load_centers {
            if load_center.number_generators > 0 && !load_center.generator_controllers.is_empty() {
                return Err(Error::Unavailable(
                    "actual positive PV/wind generator capacity fields541",
                ));
            }
        }
        let column = *o
            .predefined_registry
            .named_indices
            .get("pdchLeedRenRatCap")
            .ok_or(Error::Unavailable(
                "same actual pdchLeedRenRatCap column549",
            ))?;
        o.predefined_table.fixed_real_entry_2(
            column,
            "Photovoltaic",
            self.pv_total_capacity / 1000.0,
        )?;
        o.predefined_table
            .fixed_real_entry_2(column, "Wind", self.wind_total_capacity / 1000.0)?;
        Ok(())
    }
    pub(super) fn sum_source_storage_devices(&mut self) -> Result<(), Error> {
        self.number_of_electric_storage_devices = 0;
        for load_center in &self.load_centers {
            if load_center.storage.is_some() {
                self.number_of_electric_storage_devices = self
                    .number_of_electric_storage_devices
                    .checked_add(1)
                    .ok_or(Error::Unavailable("actual defined storage count564"))?;
            }
        }
        Ok(())
    }
    pub(super) fn check_source_load_centers(&self) -> Result<(), Error> {
        let storage_names: Vec<String> = Vec::new();
        let generator_list_names: Vec<String> = Vec::new();
        let inverter_names: Vec<String> = Vec::new();
        let converter_names: Vec<String> = Vec::new();
        let transformer_names: Vec<String> = Vec::new();
        for load_center in &self.load_centers {
            if load_center.storage.is_some() {
                return Err(Error::Unavailable("actual positive storage name getter584"));
            }
            if !load_center.generator_controllers.is_empty() {
                return Err(Error::Unavailable(
                    "actual positive generator-list name getter587",
                ));
            }
            if load_center.inverter.is_some() {
                return Err(Error::Unavailable(
                    "actual positive inverter name getter590",
                ));
            }
            if load_center.converter.is_some() {
                return Err(Error::Unavailable(
                    "actual positive converter name getter593",
                ));
            }
            if load_center.transformer.is_some() {
                return Err(Error::Unavailable(
                    "actual positive transformer name getter596",
                ));
            }
        }
        // Same five ordered name vectors are genuinely empty for default rows.
        // No duplicate/severe/fatal arm is claimed without its genuine objects.
        for names in [
            &storage_names,
            &generator_list_names,
            &inverter_names,
            &converter_names,
            &transformer_names,
        ] {
            for i in 0..names.len() {
                for j in 0..names.len() {
                    if names[i] == names[j] && i != j {
                        return Err(Error::Unavailable(
                            "actual positive duplicate load-center severe606",
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
