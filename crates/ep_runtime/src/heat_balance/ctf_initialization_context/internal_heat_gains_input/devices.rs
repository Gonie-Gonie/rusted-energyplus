//! Safe identities for the real SetupSpaceInternalGain pointer targets.
use super::{E, SelectedInternalHeatGains, index, unavailable};
use crate::heat_balance::error_backend::SourceErrorBackend;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceRateField {
    Convective,
    Radiant,
    Latent,
    CarbonDioxide,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceRateTarget {
    ZeroPointerValue,
    ElectricEquipment { row: usize, field: SourceRateField },
    OtherEquipment { row: usize, field: SourceRateField },
    People { row: usize, field: SourceRateField },
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceGainDevice {
    pub(crate) component_name: String,
    pub(crate) component_type: i32,
    pub(crate) space_gain_fraction: f64,
    // Convection, return-air convection, radiation, latent, return-air latent,
    // CO2, generic contaminant: the exact seven Source pointer fields.
    pub(crate) targets: [Option<SourceRateTarget>; 7],
    pub(crate) cached_rates: [f64; 7],
    pub(crate) return_air_node: i32,
}
impl Default for SourceGainDevice {
    fn default() -> Self {
        Self {
            component_name: String::new(),
            component_type: -1,
            space_gain_fraction: 1.0,
            targets: [None; 7],
            cached_rates: [0.0; 7],
            return_air_node: 0,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceSpaceGainDevices {
    pub(crate) number_of_devices: usize,
    pub(crate) max_number_of_devices: usize,
    pub(crate) devices: Option<Vec<SourceGainDevice>>,
}
impl SelectedInternalHeatGains {
    pub(super) fn setup_space_device(
        &mut self,
        space: i32,
        equipment: usize,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), E> {
        let name = self
            .allocations
            .other_equipment
            .as_ref()
            .and_then(|v| v.get(equipment))
            .ok_or_else(|| unavailable("actual internal-gain rate-field owner"))?
            .name
            .to_ascii_uppercase();
        let owner = self
            .allocations
            .space_devices
            .as_mut()
            .and_then(|v| v.get_mut(index(space).ok()?))
            .ok_or_else(|| unavailable("actual SpaceIntGainDevices association"))?;
        if let Some(rows) = &owner.devices {
            if rows
                .iter()
                .take(owner.number_of_devices)
                .any(|row| row.component_type == 6 && row.component_name == name)
            {
                backend.show_severe_error("SetupZoneInternalGain: developer error, trapped duplicate internal gains sent to SetupZoneInternalGain")?;
                backend.show_continue_error(&format!("The duplicate object user name ={name}"))?;
                backend.show_continue_error("The duplicate object type = OtherEquipment")?;
                backend.show_continue_error(
                    "This internal gain will not be modeled, and the simulation continues",
                )?;
                return Ok(());
            }
        }
        if owner.number_of_devices == 0 {
            owner.devices = Some(vec![SourceGainDevice::default(); 100]);
            owner.max_number_of_devices = 100;
        } else if owner.number_of_devices + 1 > owner.max_number_of_devices {
            owner.max_number_of_devices = owner
                .max_number_of_devices
                .checked_add(100)
                .ok_or_else(|| unavailable("actual gain-device redimension capacity"))?;
            owner
                .devices
                .as_mut()
                .ok_or_else(|| unavailable("actual gain-device allocation"))?
                .resize(owner.max_number_of_devices, SourceGainDevice::default());
        }
        owner.number_of_devices += 1;
        let row = owner
            .devices
            .as_mut()
            .and_then(|v| v.get_mut(owner.number_of_devices - 1))
            .ok_or_else(|| unavailable("actual next gain-device slot"))?;
        row.component_name = name;
        row.component_type = 6; // Genuine IntGainType::OtherEquipment, not a typed ID.
        // OtherEquipment is absent from AdjustTankLossMultipliers.
        row.space_gain_fraction = 1.0;
        row.targets = [
            Some(SourceRateTarget::OtherEquipment {
                row: equipment,
                field: SourceRateField::Convective,
            }),
            Some(SourceRateTarget::ZeroPointerValue),
            Some(SourceRateTarget::OtherEquipment {
                row: equipment,
                field: SourceRateField::Radiant,
            }),
            Some(SourceRateTarget::OtherEquipment {
                row: equipment,
                field: SourceRateField::Latent,
            }),
            Some(SourceRateTarget::ZeroPointerValue),
            Some(SourceRateTarget::ZeroPointerValue),
            Some(SourceRateTarget::ZeroPointerValue),
        ];
        row.return_air_node = 0;
        Ok(())
    }
    pub(super) fn setup_electric_device(
        &mut self,
        space: i32,
        equipment: usize,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), E> {
        let name = self
            .allocations
            .electric_equipment
            .as_ref()
            .and_then(|v| v.get(equipment))
            .ok_or_else(|| unavailable("actual internal-gain rate-field owner"))?
            .name
            .to_ascii_uppercase();
        let owner = self
            .allocations
            .space_devices
            .as_mut()
            .and_then(|v| v.get_mut(index(space).ok()?))
            .ok_or_else(|| unavailable("actual SpaceIntGainDevices association"))?;
        if let Some(rows) = &owner.devices {
            if rows
                .iter()
                .take(owner.number_of_devices)
                .any(|row| row.component_type == 2 && row.component_name == name)
            {
                backend.show_severe_error("SetupZoneInternalGain: developer error, trapped duplicate internal gains sent to SetupZoneInternalGain")?;
                backend.show_continue_error(&format!("The duplicate object user name ={name}"))?;
                backend.show_continue_error("The duplicate object type = ElectricEquipment")?;
                backend.show_continue_error(
                    "This internal gain will not be modeled, and the simulation continues",
                )?;
                return Ok(());
            }
        }
        if owner.number_of_devices == 0 {
            owner.devices = Some(vec![SourceGainDevice::default(); 100]);
            owner.max_number_of_devices = 100;
        } else if owner.number_of_devices + 1 > owner.max_number_of_devices {
            owner.max_number_of_devices = owner
                .max_number_of_devices
                .checked_add(100)
                .ok_or_else(|| unavailable("actual gain-device redimension capacity"))?;
            owner
                .devices
                .as_mut()
                .ok_or_else(|| unavailable("actual gain-device allocation"))?
                .resize(owner.max_number_of_devices, SourceGainDevice::default());
        }
        owner.number_of_devices += 1;
        let row = owner
            .devices
            .as_mut()
            .and_then(|v| v.get_mut(owner.number_of_devices - 1))
            .ok_or_else(|| unavailable("actual next gain-device slot"))?;
        row.component_name = name;
        row.component_type = 2; // Genuine IntGainType::ElectricEquipment, not a typed ID.
        // ElectricEquipment is absent from AdjustTankLossMultipliers.
        row.space_gain_fraction = 1.0;
        row.targets = [
            Some(SourceRateTarget::ElectricEquipment {
                row: equipment,
                field: SourceRateField::Convective,
            }),
            Some(SourceRateTarget::ZeroPointerValue),
            Some(SourceRateTarget::ElectricEquipment {
                row: equipment,
                field: SourceRateField::Radiant,
            }),
            Some(SourceRateTarget::ElectricEquipment {
                row: equipment,
                field: SourceRateField::Latent,
            }),
            Some(SourceRateTarget::ZeroPointerValue),
            Some(SourceRateTarget::ZeroPointerValue),
            Some(SourceRateTarget::ZeroPointerValue),
        ];
        row.return_air_node = 0;
        Ok(())
    }
    pub(super) fn setup_people_device(
        &mut self,
        space: i32,
        equipment: usize,
        backend: &mut SourceErrorBackend<'_>,
        flags: &crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
    ) -> Result<(), E> {
        let name = self
            .allocations
            .people
            .rows
            .get(equipment)
            .ok_or_else(|| unavailable("actual internal-gain rate-field owner"))?
            .name
            .to_ascii_uppercase();
        let owner = self
            .allocations
            .space_devices
            .as_mut()
            .and_then(|v| v.get_mut(index(space).ok()?))
            .ok_or_else(|| unavailable("actual SpaceIntGainDevices association"))?;
        if let Some(rows) = &owner.devices {
            if rows
                .iter()
                .take(owner.number_of_devices)
                .any(|row| row.component_type == 0 && row.component_name == name)
            {
                backend.show_source_severe_error("SetupZoneInternalGain: developer error, trapped duplicate internal gains sent to SetupZoneInternalGain", flags)?;
                backend.show_continue_error(&format!("The duplicate object user name ={name}"))?;
                backend.show_continue_error("The duplicate object type = People")?;
                backend.show_continue_error(
                    "This internal gain will not be modeled, and the simulation continues",
                )?;
                return Ok(());
            }
        }
        if owner.number_of_devices == 0 {
            owner.devices = Some(vec![SourceGainDevice::default(); 100]);
            owner.max_number_of_devices = 100;
        } else if owner.number_of_devices + 1 > owner.max_number_of_devices {
            owner.max_number_of_devices = owner
                .max_number_of_devices
                .checked_add(100)
                .ok_or_else(|| unavailable("actual gain-device redimension capacity"))?;
            owner
                .devices
                .as_mut()
                .ok_or_else(|| unavailable("actual gain-device allocation"))?
                .resize(owner.max_number_of_devices, SourceGainDevice::default());
        }
        owner.number_of_devices += 1;
        let row = owner
            .devices
            .as_mut()
            .and_then(|v| v.get_mut(owner.number_of_devices - 1))
            .ok_or_else(|| unavailable("actual next gain-device slot"))?;
        row.component_name = name;
        row.component_type = 0; // Genuine IntGainType::People, not a typed ID.
        // People is absent from AdjustTankLossMultipliers.
        row.space_gain_fraction = 1.0;
        row.targets = [
            Some(SourceRateTarget::People {
                row: equipment,
                field: SourceRateField::Convective,
            }),
            Some(SourceRateTarget::ZeroPointerValue),
            Some(SourceRateTarget::People {
                row: equipment,
                field: SourceRateField::Radiant,
            }),
            Some(SourceRateTarget::People {
                row: equipment,
                field: SourceRateField::Latent,
            }),
            Some(SourceRateTarget::ZeroPointerValue),
            Some(SourceRateTarget::People {
                row: equipment,
                field: SourceRateField::CarbonDioxide,
            }),
            Some(SourceRateTarget::ZeroPointerValue),
        ];
        row.return_air_node = 0;
        Ok(())
    }
}
