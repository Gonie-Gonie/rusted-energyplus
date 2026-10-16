//! Reached None-fuel IHG declarations on the existing OutputProcessor owner.
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
            for row in equipment {
                add_zone_outputs[index(row.zone_ptr)?] = true;
                add_space_outputs[index(row.space_index)?] = true;
                if row.fuel != super::SourceOtherFuel::None {
                    return Err(unavailable("positive IHG fuel meter output"));
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
