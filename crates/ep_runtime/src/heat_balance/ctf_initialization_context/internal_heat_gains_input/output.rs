//! Ordered real eFuel IHG declarations on the existing OutputProcessor owner.
use super::getter::SourceGainInputOwners;
use super::{E, SelectedInternalHeatGains, index, unavailable};
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
    owners: &mut SourceGainInputOwners<'_, '_>,
    key: &str,
    name: &str,
    index: usize,
    _actual_rate_or_energy: &f64,
) -> Result<(), E> {
    // Native's pointer argument is not stored in the no-request/no-meter
    // return. A positive requested OutVar stops in the existing registration API.
    let mut request = SourceReportVariableInput::new(
        owners.report_inputs,
        owners.input,
        owners.minimum_report_frequency,
    );
    let mut host = GainOutputHost(&mut *owners.host);
    owners
        .registration
        .setup_unmetered_zone_real(
            &mut request,
            owners.prescan,
            SourceRealOutputDeclaration {
                name,
                key,
                units: if index.is_multiple_of(2) {
                    Units::J
                } else {
                    Units::W
                },
                store_type: if index.is_multiple_of(2) {
                    Store::Sum
                } else {
                    Store::Average
                },
            },
            owners.minutes,
            &mut host,
            owners.eio,
        )
        .map_err(E::Output)
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
            let report = self
                .allocations
                .zone_report
                .as_ref()
                .and_then(|v| v.get(number))
                .ok_or_else(|| unavailable("actual ZoneRpt allocation"))?;
            for (i, suffix) in TOTAL.iter().enumerate() {
                declare(
                    owners,
                    name,
                    &format!("Zone Total Internal {suffix}"),
                    i,
                    &report.total[i],
                )?;
            }
        }
        for (number, name) in space_names.iter().enumerate() {
            let report = self
                .allocations
                .space_report
                .as_ref()
                .and_then(|v| v.get(number))
                .ok_or_else(|| unavailable("actual spaceRpt allocation"))?;
            for (i, suffix) in TOTAL.iter().enumerate() {
                declare(
                    owners,
                    name,
                    &format!("Space Total Internal {suffix}"),
                    i,
                    &report.total[i],
                )?;
            }
        }
        let mut add_zone_outputs = vec![false; zone_names.len()];
        let mut add_space_outputs = vec![false; space_names.len()];
        // All preceding equipment loops are genuinely empty in this admission.
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
                    let value = if i.is_multiple_of(2) {
                        &row.energies[i / 2 + 1]
                    } else {
                        &row.rates[i / 2 + 1]
                    };
                    declare(
                        owners,
                        &row.name,
                        &format!("Other Equipment {suffix}"),
                        i,
                        value,
                    )?;
                }
            }
        }
        for (number, name) in zone_names.iter().enumerate() {
            if add_zone_outputs[number] {
                let report = self
                    .allocations
                    .zone_report
                    .as_ref()
                    .and_then(|v| v.get(number))
                    .ok_or_else(|| unavailable("same Other ZoneRpt"))?;
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
                        owners,
                        name,
                        &format!("Zone Other Equipment {fuel_name} Rate"),
                        1,
                        &report.other_power[fuel_index],
                    )?;
                    declare(
                        owners,
                        name,
                        &format!("Zone Other Equipment {fuel_name} Energy"),
                        0,
                        &report.other_consumption[fuel_index],
                    )?;
                }
                for (i, suffix) in OTHER.iter().enumerate() {
                    declare(
                        owners,
                        name,
                        &format!("Zone Other Equipment {suffix}"),
                        i,
                        &report.other[i],
                    )?;
                }
            }
            add_zone_outputs[number] = false;
        }
        for (number, name) in space_names.iter().enumerate() {
            if add_space_outputs[number] {
                let report = self
                    .allocations
                    .space_report
                    .as_ref()
                    .and_then(|v| v.get(number))
                    .ok_or_else(|| unavailable("same Other spaceRpt"))?;
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
                        owners,
                        name,
                        &format!("Space Other Equipment {fuel_name} Rate"),
                        1,
                        &report.other_power[fuel_index],
                    )?;
                    declare(
                        owners,
                        name,
                        &format!("Space Other Equipment {fuel_name} Energy"),
                        0,
                        &report.other_consumption[fuel_index],
                    )?;
                }
                for (i, suffix) in OTHER.iter().enumerate() {
                    declare(
                        owners,
                        name,
                        &format!("Space Other Equipment {suffix}"),
                        i,
                        &report.other[i],
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
            owners.minimum_report_frequency,
        );
        owners
            .registration
            .setup_bound_general_real_source_input(
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
                SourceGeneralRealRegistrationOptions {
                    time_step: crate::heat_balance::source_output_registration::TimeStepKind::Zone,
                    meter,
                },
                owners.backend,
            )
            .map_err(E::Output)
    }
}
