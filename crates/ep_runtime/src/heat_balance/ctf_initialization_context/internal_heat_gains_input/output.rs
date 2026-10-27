//! Ordered real People/eFuel IHG declarations on the existing OutputProcessor owner.
use super::getter::SourceGainInputOwners;
use super::{E, SelectedInternalHeatGains, index, unavailable};
use crate::heat_balance::source_output_registration::SourceInternalGainOutputField as Field;
use crate::heat_balance::source_output_registration::{
    SourceOutputFileHost, SourceOutputStoreType as Store, SourceOutputUnits as Units,
    SourceRealOutputDeclaration, SourceReportVariableInput,
};

struct GainOutputHost<'a>(&'a mut dyn SourceOutputFileHost);
impl SourceOutputFileHost for GainOutputHost<'_> {
    fn ensure_mtd_open(&mut self) -> std::io::Result<()> {
        self.0.ensure_mtd_open()
    }
}
use crate::heat_balance::source_output_registration::SourcePeopleOutputField as PeopleField;
const PEOPLE: [(&str, PeopleField, Units, Store); 13] = [
    (
        "Occupant Count",
        PeopleField::Count,
        Units::Dimensionless,
        Store::Average,
    ),
    (
        "Radiant Heating Energy",
        PeopleField::RadiantEnergy,
        Units::J,
        Store::Sum,
    ),
    (
        "Radiant Heating Rate",
        PeopleField::RadiantRate,
        Units::W,
        Store::Average,
    ),
    (
        "Convective Heating Energy",
        PeopleField::ConvectiveEnergy,
        Units::J,
        Store::Sum,
    ),
    (
        "Convective Heating Rate",
        PeopleField::ConvectiveRate,
        Units::W,
        Store::Average,
    ),
    (
        "Sensible Heating Energy",
        PeopleField::SensibleEnergy,
        Units::J,
        Store::Sum,
    ),
    (
        "Sensible Heating Rate",
        PeopleField::SensibleRate,
        Units::W,
        Store::Average,
    ),
    (
        "Latent Gain Energy",
        PeopleField::LatentEnergy,
        Units::J,
        Store::Sum,
    ),
    (
        "Latent Gain Rate",
        PeopleField::LatentRate,
        Units::W,
        Store::Average,
    ),
    (
        "Total Heating Energy",
        PeopleField::TotalEnergy,
        Units::J,
        Store::Sum,
    ),
    (
        "Total Heating Rate",
        PeopleField::TotalRate,
        Units::W,
        Store::Average,
    ),
    (
        "Air Temperature",
        PeopleField::AirTemperature,
        Units::C,
        Store::Average,
    ),
    (
        "Air Relative Humidity",
        PeopleField::AirRelativeHumidity,
        Units::Percentage,
        Store::Average,
    ),
];
const TOTAL: [&str; 10] = [
    "Radiant Heating Energy",
    "Radiant Heating Rate",
    "Visible Radiation Heating Energy",
    "Visible Radiation Heating Rate",
    "Convective Heating Energy",
    "Convective Heating Rate",
    "Latent Gain Energy",
    "Latent Gain Rate",
    "Total Heating Energy",
    "Total Heating Rate",
];
const OTHER: [&str; 10] = [
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

fn declare(
    gains: &SelectedInternalHeatGains,
    owners: &mut SourceGainInputOwners<'_, '_>,
    key: &str,
    name: &str,
    declaration_index: usize,
    field: crate::heat_balance::source_output_registration::SourceInternalGainOutputField,
    row_index: usize,
) -> Result<(), E> {
    use crate::heat_balance::source_output_registration::{
        SourceGeneralRealRegistrationOptions, SourceRealOutputBinding, SourceRealOutputTarget,
        TimeStepKind,
    };
    let binding = SourceRealOutputBinding::new(
        SourceRealOutputTarget::InternalGain {
            field,
            index: i32::try_from(row_index + 1)
                .map_err(|_| unavailable("actual internal gain report output index"))?,
        },
        gains,
    )
    .map_err(E::Output)?;
    let mut request = SourceReportVariableInput::new(
        owners.report_inputs,
        owners.input,
        crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::from_owners(
            owners.minimum_report_frequency, owners.globals, owners.developer, owners.clock,
        ),
        owners.backend,
    );
    owners.registration.setup_bound_general_real_optional_dictionary(
        &mut request, owners.prescan,
        SourceRealOutputDeclaration {
            name, key,
            units: if declaration_index.is_multiple_of(2) { Units::J } else { Units::W },
            store_type: if declaration_index.is_multiple_of(2) { Store::Sum } else { Store::Average },
        },
        owners.minutes, &mut GainOutputHost(&mut *owners.host), owners.eio, binding,
        owners.dictionary_host.as_deref_mut().map(|host| host as &mut dyn crate::heat_balance::source_output_registration::SourceOutputDictionaryHost),
        SourceGeneralRealRegistrationOptions::unmetered(TimeStepKind::Zone),
    ).map_err(E::Output)
}

impl SelectedInternalHeatGains {
    pub(super) fn setup_outputs(
        &self,
        owners: &mut SourceGainInputOwners<'_, '_>,
    ) -> Result<(), E> {
        let zones = owners
            .zones
            .zones
            .as_ref()
            .ok_or_else(|| unavailable("actual output Zones"))?;
        let spaces = owners
            .zones
            .spaces
            .as_ref()
            .ok_or_else(|| unavailable("actual output Spaces"))?;
        // These names/values are local borrows of the SAME owners. Copy only
        // names to allow the independent IP/registration mutable borrows.
        let zone_names: Vec<_> = zones.iter().map(|r| r.name.clone()).collect();
        let space_names: Vec<_> = spaces.iter().map(|r| r.name.clone()).collect();
        for (number, name) in zone_names.iter().enumerate() {
            for (i, suffix) in TOTAL.iter().enumerate() {
                declare(
                    self,
                    owners,
                    name,
                    &format!("Zone Total Internal {suffix}"),
                    i,
                    Field::ZoneTotalInternal {
                        field: u8::try_from(i)
                            .map_err(|_| unavailable("actual total report ordinal"))?,
                    },
                    number,
                )?;
            }
        }
        for (number, name) in space_names.iter().enumerate() {
            for (i, suffix) in TOTAL.iter().enumerate() {
                declare(
                    self,
                    owners,
                    name,
                    &format!("Space Total Internal {suffix}"),
                    i,
                    Field::SpaceTotalInternal {
                        field: u8::try_from(i)
                            .map_err(|_| unavailable("actual total report ordinal"))?,
                    },
                    number,
                )?;
            }
        }
        let mut add_zone_outputs = vec![false; zone_names.len()];
        let mut add_space_outputs = vec![false; space_names.len()];
        for people_number in 1..=self.totals[0] {
            let number = index(people_number)?;
            let people = self
                .allocations
                .people
                .get(number)
                .ok_or_else(|| unavailable("actual People output row3728"))?;
            add_zone_outputs[index(people.zone_ptr)?] = true;
            add_space_outputs[index(people.space_index)?] = true;
            for &(suffix, field, units, store) in &PEOPLE {
                self.declare_people(owners, &people.name, &format!("People {suffix}"),
                    number, crate::heat_balance::source_output_registration::SourceInternalGainOutputField::PeopleObject(field),
                    units, store)?;
            }
        }
        for (number, name) in zone_names.iter().enumerate() {
            if add_zone_outputs[number] {
                for &(suffix, field, units, store) in &PEOPLE[..11] {
                    self.declare_people(owners, name, &format!("Zone People {suffix}"),
                        number, crate::heat_balance::source_output_registration::SourceInternalGainOutputField::PeopleZone(field),
                        units, store)?;
                }
            }
            add_zone_outputs[number] = false;
        }
        for (number, name) in space_names.iter().enumerate() {
            if add_space_outputs[number] {
                for &(suffix, field, units, store) in &PEOPLE[..11] {
                    self.declare_people(owners, name, &format!("Space People {suffix}"),
                        number, crate::heat_balance::source_output_registration::SourceInternalGainOutputField::PeopleSpace(field),
                        units, store)?;
                }
            }
            add_space_outputs[number] = false;
        }
        // Lights/electric/gas/hot-water/steam loops remain genuinely empty here.
        if let Some(equipment) = &self.allocations.other_equipment {
            for (row_index, row) in equipment.iter().enumerate() {
                add_zone_outputs[index(row.zone_ptr)?] = true;
                add_space_outputs[index(row.space_index)?] = true;
                if row.fuel != super::SourceOtherFuel::None
                    && row.fuel != super::SourceOtherFuel::Invalid
                {
                    self.declare_fuel(owners, row_index, false)?;
                    self.declare_fuel(owners, row_index, true)?;
                }
                for (i, suffix) in OTHER.iter().enumerate() {
                    declare(
                        self,
                        owners,
                        &row.name,
                        &format!("Other Equipment {suffix}"),
                        i,
                        Field::OtherThermal {
                            field: u8::try_from(i)
                                .map_err(|_| unavailable("actual OtherEquipment report ordinal"))?,
                        },
                        row_index,
                    )?;
                }
            }
        }
        for (number, name) in zone_names.iter().enumerate() {
            if add_zone_outputs[number] {
                let fuels = owners
                    .zones
                    .zones
                    .as_ref()
                    .ok_or_else(|| unavailable("actual zone fuel list"))?[number]
                    .other_equipment_fuels
                    .clone();
                for fuel in fuels {
                    if fuel == -1 || fuel == 14 {
                        continue;
                    }
                    let fuel_index =
                        usize::try_from(fuel)
                            .ok()
                            .filter(|i| *i < 13)
                            .ok_or_else(|| {
                                unavailable("actual valid OtherEquipment fuel total index")
                            })?;
                    let fuel_name = super::SourceOtherFuel::source_name_at(fuel)?;
                    declare(
                        self,
                        owners,
                        name,
                        &format!("Zone Other Equipment {fuel_name} Rate"),
                        1,
                        Field::ZoneOtherFuel {
                            fuel: u8::try_from(fuel_index)
                                .map_err(|_| unavailable("actual fuel report ordinal"))?,
                            energy: false,
                        },
                        number,
                    )?;
                    declare(
                        self,
                        owners,
                        name,
                        &format!("Zone Other Equipment {fuel_name} Energy"),
                        0,
                        Field::ZoneOtherFuel {
                            fuel: u8::try_from(fuel_index)
                                .map_err(|_| unavailable("actual fuel report ordinal"))?,
                            energy: true,
                        },
                        number,
                    )?;
                }
                for (i, suffix) in OTHER.iter().enumerate() {
                    declare(
                        self,
                        owners,
                        name,
                        &format!("Zone Other Equipment {suffix}"),
                        i,
                        Field::ZoneOtherThermal {
                            field: u8::try_from(i)
                                .map_err(|_| unavailable("actual OtherEquipment report ordinal"))?,
                        },
                        number,
                    )?;
                }
            }
            add_zone_outputs[number] = false;
        }
        for (number, name) in space_names.iter().enumerate() {
            if add_space_outputs[number] {
                let fuels = owners
                    .zones
                    .spaces
                    .as_ref()
                    .ok_or_else(|| unavailable("actual space fuel list"))?[number]
                    .other_equipment_fuels
                    .clone();
                for fuel in fuels {
                    if fuel == -1 || fuel == 14 {
                        continue;
                    }
                    let fuel_index =
                        usize::try_from(fuel)
                            .ok()
                            .filter(|i| *i < 13)
                            .ok_or_else(|| {
                                unavailable("actual valid OtherEquipment fuel total index")
                            })?;
                    let fuel_name = super::SourceOtherFuel::source_name_at(fuel)?;
                    declare(
                        self,
                        owners,
                        name,
                        &format!("Space Other Equipment {fuel_name} Rate"),
                        1,
                        Field::SpaceOtherFuel {
                            fuel: u8::try_from(fuel_index)
                                .map_err(|_| unavailable("actual fuel report ordinal"))?,
                            energy: false,
                        },
                        number,
                    )?;
                    declare(
                        self,
                        owners,
                        name,
                        &format!("Space Other Equipment {fuel_name} Energy"),
                        0,
                        Field::SpaceOtherFuel {
                            fuel: u8::try_from(fuel_index)
                                .map_err(|_| unavailable("actual fuel report ordinal"))?,
                            energy: true,
                        },
                        number,
                    )?;
                }
                for (i, suffix) in OTHER.iter().enumerate() {
                    declare(
                        self,
                        owners,
                        name,
                        &format!("Space Other Equipment {suffix}"),
                        i,
                        Field::SpaceOtherThermal {
                            field: u8::try_from(i)
                                .map_err(|_| unavailable("actual OtherEquipment report ordinal"))?,
                        },
                        number,
                    )?;
                }
            }
            add_space_outputs[number] = false;
        }
        // Later ITE/Baseboard loops are also empty; local flags then destruct.
        Ok(())
    }
}

impl SelectedInternalHeatGains {
    fn declare_people(
        &self,
        owners: &mut SourceGainInputOwners<'_, '_>,
        key: &str,
        name: &str,
        row_index: usize,
        field: crate::heat_balance::source_output_registration::SourceInternalGainOutputField,
        units: Units,
        store_type: Store,
    ) -> Result<(), E> {
        use crate::heat_balance::source_output_registration::{
            SourceGeneralRealRegistrationOptions, SourceRealOutputBinding, SourceRealOutputTarget,
            TimeStepKind,
        };
        let target = SourceRealOutputTarget::InternalGain {
            field,
            index: i32::try_from(row_index + 1)
                .map_err(|_| unavailable("actual People output index"))?,
        };
        let binding = SourceRealOutputBinding::new(target, self).map_err(E::Output)?;
        let mut request = SourceReportVariableInput::new(
            owners.report_inputs,
            owners.input,
            crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::from_owners(
                owners.minimum_report_frequency, owners.globals, owners.developer, owners.clock,
            ),
            owners.backend,
        );
        owners
            .registration
            .setup_bound_general_real_optional_dictionary(
                &mut request,
                owners.prescan,
                SourceRealOutputDeclaration {
                    name,
                    key,
                    units,
                    store_type,
                },
                owners.minutes,
                &mut GainOutputHost(&mut *owners.host),
                owners.eio,
                binding,
                owners.dictionary_host.as_mut().map(|host| &mut **host as &mut dyn crate::heat_balance::source_output_registration::SourceOutputDictionaryHost),
                SourceGeneralRealRegistrationOptions::unmetered(TimeStepKind::Zone),
            )
            .map_err(E::Output)
    }
    fn declare_fuel(
        &self,
        owners: &mut SourceGainInputOwners<'_, '_>,
        row_index: usize,
        energy: bool,
    ) -> Result<(), E> {
        use crate::heat_balance::source_output_registration::{
            SourceGeneralMeterArguments, SourceGeneralRealRegistrationOptions,
            SourceInternalGainOutputField, SourceMeterEndUse, SourceMeterGroup,
            SourceRealOutputBinding, SourceRealOutputTarget,
        };
        let row = self
            .allocations
            .other_equipment
            .as_ref()
            .and_then(|rows| rows.get(row_index))
            .ok_or_else(|| unavailable("same OtherEquipment output row"))?;
        let target = SourceRealOutputTarget::InternalGain {
            index: i32::try_from(row_index + 1)
                .map_err(|_| unavailable("actual OtherEquipment output index"))?,
            field: if energy {
                SourceInternalGainOutputField::OtherConsumption
            } else {
                SourceInternalGainOutputField::OtherPower
            },
        };
        let binding = SourceRealOutputBinding::new(target, self).map_err(E::Output)?;
        let zone = owners
            .zones
            .zones
            .as_ref()
            .and_then(|r| index(row.zone_ptr).ok().and_then(|i| r.get(i)))
            .ok_or_else(|| unavailable("same output Zone"))?;
        let space = owners
            .zones
            .spaces
            .as_ref()
            .and_then(|r| index(row.space_index).ok().and_then(|i| r.get(i)))
            .ok_or_else(|| unavailable("same output Space"))?;
        let meter = energy.then_some(SourceGeneralMeterArguments {
            resource: row.fuel.meter_resource()?,
            end_use: SourceMeterEndUse::InteriorEquipment,
            end_use_subcategory: &row.end_use_subcategory,
            group: SourceMeterGroup::Building,
            zone_name: &zone.name,
            space_type: &space.space_type,
            zone_multiplier: zone.multiplier,
            zone_list_multiplier: zone.list_multiplier,
        });
        let mut request = SourceReportVariableInput::new(
            owners.report_inputs,
            owners.input,
            crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::from_owners(
                owners.minimum_report_frequency, owners.globals, owners.developer, owners.clock,
            ),
            owners.backend,
        );
        owners
            .registration
            .setup_bound_general_real_optional_dictionary(
                &mut request,
                owners.prescan,
                SourceRealOutputDeclaration {
                    name: &format!(
                        "Other Equipment {} {}",
                        row.fuel.source_name()?,
                        if energy { "Energy" } else { "Rate" }
                    ),
                    key: &row.name,
                    units: if energy { Units::J } else { Units::W },
                    store_type: if energy { Store::Sum } else { Store::Average },
                },
                owners.minutes,
                &mut GainOutputHost(&mut *owners.host),
                owners.eio,
                binding,
                owners.dictionary_host.as_deref_mut().map(|host| host as &mut dyn crate::heat_balance::source_output_registration::SourceOutputDictionaryHost),
                SourceGeneralRealRegistrationOptions {
                    time_step: crate::heat_balance::source_output_registration::TimeStepKind::Zone,
                    meter,
                },
            )
            .map_err(E::Output)
    }
}
