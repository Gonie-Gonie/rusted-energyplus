//! IHG4283-4567 declarations on the actual ZoneElectric/ZoneRpt/SpaceRpt owners.
use super::{
    E, Field, GainOutputHost, SelectedInternalHeatGains, SourceGainInputOwners, Store, Units,
    declare, index, unavailable,
};
use crate::heat_balance::source_output_registration::{
    SourceGeneralMeterArguments, SourceGeneralRealRegistrationOptions, SourceMeterEndUse,
    SourceMeterGroup, SourceMeterResource, SourceRealOutputBinding, SourceRealOutputDeclaration,
    SourceRealOutputTarget, SourceReportVariableInput, TimeStepKind,
};

const SUFFIXES: [&str; 12] = [
    "Electricity Energy",
    "Electricity Rate",
    "Radiant Heating Energy",
    "Radiant Heating Rate",
    "Convective Heating Energy",
    "Convective Heating Rate",
    "Latent Gain Energy",
    "Latent Gain Rate",
    "Lost Heat Energy",
    "Lost Heat Rate",
    "Total Heating Energy",
    "Total Heating Rate",
];

impl SelectedInternalHeatGains {
    pub(super) fn setup_electric_outputs(
        &self,
        owners: &mut SourceGainInputOwners<'_, '_>,
        zone_names: &[String],
        space_names: &[String],
    ) -> Result<(), E> {
        let mut add_zone = vec![false; zone_names.len()];
        let mut add_space = vec![false; space_names.len()];
        if let Some(rows) = &self.allocations.electric_equipment {
            for (number, row) in rows.iter().enumerate() {
                add_zone[index(row.zone_ptr)?] = true;
                add_space[index(row.space_index)?] = true;
                // Native object declarations begin Rate, Energy; the actual
                // Which ordinal remains the shared Energy/Rate field identity.
                for field in [1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] {
                    let suffix = SUFFIXES[field];
                    if field == 0 {
                        self.declare_electric_consumption(owners, number)?;
                    } else {
                        declare(
                            self,
                            owners,
                            &row.name,
                            &format!("Electric Equipment {suffix}"),
                            field,
                            Field::ElectricObject {
                                field: u8::try_from(field)
                                    .map_err(|_| unavailable("actual electric output ordinal"))?,
                            },
                            number,
                        )?;
                    }
                }
            }
        }
        for (number, name) in zone_names.iter().enumerate() {
            if add_zone[number] {
                for field in [1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] {
                    let suffix = SUFFIXES[field];
                    declare(
                        self,
                        owners,
                        name,
                        &format!("Zone Electric Equipment {suffix}"),
                        field,
                        Field::ElectricZone {
                            field: u8::try_from(field)
                                .map_err(|_| unavailable("actual Zone electric output ordinal"))?,
                        },
                        number,
                    )?;
                }
            }
        }
        for (number, name) in space_names.iter().enumerate() {
            if add_space[number] {
                for field in [1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] {
                    let suffix = SUFFIXES[field];
                    declare(
                        self,
                        owners,
                        name,
                        &format!("Space Electric Equipment {suffix}"),
                        field,
                        Field::ElectricSpace {
                            field: u8::try_from(field)
                                .map_err(|_| unavailable("actual Space electric output ordinal"))?,
                        },
                        number,
                    )?;
                }
            }
        }
        Ok(())
    }

    fn declare_electric_consumption(
        &self,
        owners: &mut SourceGainInputOwners<'_, '_>,
        number: usize,
    ) -> Result<(), E> {
        let row = self
            .allocations
            .electric_equipment
            .as_ref()
            .and_then(|rows| rows.get(number))
            .ok_or_else(|| unavailable("same actual ZoneElectric output row"))?;
        let target = SourceRealOutputTarget::InternalGain {
            field: Field::ElectricObject { field: 0 },
            index: i32::try_from(number + 1)
                .map_err(|_| unavailable("actual electric output index"))?,
        };
        let binding = SourceRealOutputBinding::new(target, self).map_err(E::Output)?;
        let zone = owners
            .zones
            .zones
            .as_ref()
            .and_then(|rows| index(row.zone_ptr).ok().and_then(|i| rows.get(i)))
            .ok_or_else(|| unavailable("same electric meter Zone"))?;
        let space = owners
            .zones
            .spaces
            .as_ref()
            .and_then(|rows| index(row.space_index).ok().and_then(|i| rows.get(i)))
            .ok_or_else(|| unavailable("same electric meter Space"))?;
        let meter = SourceGeneralMeterArguments {
            resource: SourceMeterResource::Electricity,
            end_use: SourceMeterEndUse::InteriorEquipment,
            end_use_subcategory: &row.end_use_subcategory,
            group: SourceMeterGroup::Building,
            zone_name: &zone.name,
            space_type: &space.space_type,
            zone_multiplier: zone.multiplier,
            zone_list_multiplier: zone.list_multiplier,
        };
        let mut request = SourceReportVariableInput::new(owners.report_inputs, owners.input,
            crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::from_owners(
                owners.minimum_report_frequency, owners.globals, owners.developer, owners.clock), owners.backend);
        owners.registration.setup_bound_general_real_optional_dictionary(
            &mut request, owners.prescan,
            SourceRealOutputDeclaration { name: "Electric Equipment Electricity Energy", key: &row.name, units: Units::J, store_type: Store::Sum },
            owners.minutes, &mut GainOutputHost(&mut *owners.host), owners.eio, binding,
            owners.dictionary_host.as_deref_mut().map(|host| host as &mut dyn crate::heat_balance::source_output_registration::SourceOutputDictionaryHost),
            SourceGeneralRealRegistrationOptions { time_step: TimeStepKind::Zone, meter: Some(meter) },
        ).map_err(E::Output)
    }
}
