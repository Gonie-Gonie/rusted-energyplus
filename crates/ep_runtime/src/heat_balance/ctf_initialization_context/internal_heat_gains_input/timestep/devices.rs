//! Safe resolution of the same seven Source internal-gain pointer targets.
use super::super::SourceOtherEquipment;
use super::super::devices::{
    SourceGainDevice, SourceRateField, SourceRateTarget, SourceSpaceGainDevices,
};
use super::{
    E, SelectedInternalHeatGains, SourceGainLatentOwners, SourceGainStepOwners,
    SourceZoneGenericGainAllocation, index, unavailable,
};

pub(super) fn active_devices(
    spaces: &[SourceSpaceGainDevices],
    space_number: i32,
) -> Result<&[SourceGainDevice], E> {
    let space = spaces
        .get(index(space_number)?)
        .ok_or_else(|| unavailable("same actual SpaceIntGainDevices association"))?;
    if space.number_of_devices == 0 {
        return Ok(&[]);
    }
    space
        .devices
        .as_deref()
        .and_then(|rows| rows.get(..space.number_of_devices))
        .ok_or_else(|| unavailable("actual active SpaceIntGainDevices rows"))
}

pub(super) fn sum_all_space(
    spaces: &[SourceSpaceGainDevices],
    space_number: i32,
    field: usize,
) -> Result<f64, E> {
    let mut sum = 0.0;
    for device in active_devices(spaces, space_number)? {
        sum += device.cached_rates[field];
    }
    Ok(sum)
}

fn sum_all_zone(
    spaces: &[SourceSpaceGainDevices],
    space_numbers: &[i32],
    field: usize,
) -> Result<f64, E> {
    let mut sum = 0.0;
    for &space_number in space_numbers {
        for device in active_devices(spaces, space_number)? {
            sum += device.cached_rates[field];
        }
    }
    Ok(sum)
}

// Source TradIntGainTypes order: People, Lights, Electric, ITE, Gas, HW,
// Steam, Other. ITE47 is the actual DataHeatBalance.hh224-281 enum ordinal.
const TRADITIONAL_TYPES: [i32; 8] = [0, 1, 2, 47, 3, 4, 5, 6];

pub(super) fn sum_by_types(
    spaces: &[SourceSpaceGainDevices],
    zone_spaces: &[i32],
    space_number: Option<i32>,
    field: usize,
) -> Result<f64, E> {
    let mut sum = 0.0;
    if let Some(space_number) = space_number {
        for device in active_devices(spaces, space_number)? {
            for gain_type in TRADITIONAL_TYPES {
                if device.component_type == gain_type {
                    sum += device.cached_rates[field];
                }
            }
        }
    } else {
        for &space_number in zone_spaces {
            for device in active_devices(spaces, space_number)? {
                for gain_type in TRADITIONAL_TYPES {
                    if device.component_type == gain_type {
                        sum += device.cached_rates[field];
                    }
                }
            }
        }
    }
    Ok(sum)
}

fn dereference_rate(
    target: Option<SourceRateTarget>,
    equipment: Option<&[SourceOtherEquipment]>,
    people: &[super::super::comfort_people::SourcePeopleComfortData],
    zero_pointer_value: f64,
) -> Result<f64, E> {
    match target {
        Some(SourceRateTarget::ZeroPointerValue) => Ok(zero_pointer_value),
        Some(SourceRateTarget::OtherEquipment { row, field }) => {
            let row = equipment
                .and_then(|rows| rows.get(row))
                .ok_or_else(|| unavailable("same live OtherEquipment pointer owner"))?;
            Ok(row.rates[match field {
                SourceRateField::Convective => 2,
                SourceRateField::Radiant => 1,
                SourceRateField::Latent => 3,
                SourceRateField::CarbonDioxide => 6,
            }])
        }
        Some(SourceRateTarget::People { row, field }) => {
            let row = people
                .get(row)
                .ok_or_else(|| unavailable("same live People pointer owner"))?;
            Ok(match field {
                SourceRateField::Convective => row.con_gain_rate,
                SourceRateField::Radiant => row.rad_gain_rate,
                SourceRateField::Latent => row.lat_gain_rate,
                SourceRateField::CarbonDioxide => row.co2_gain_rate,
            })
        }
        None => Err(unavailable(
            "actual initialized internal-gain pointer target",
        )),
    }
}

/// Actual SumAllInternalLatentGains8405-8434, shared with InitInternalHeatGains.
pub(super) fn sum_zone_latent_gains(
    devices: &[SourceSpaceGainDevices],
    zone_number: i32,
    space_numbers: &[i32],
    latent: &mut Option<&mut dyn SourceGainLatentOwners>,
) -> Result<(), E> {
    let mut zone_latent = 0.0;
    for &space_number in space_numbers {
        let space_devices = active_devices(devices, space_number)?;
        if space_devices.is_empty() {
            *latent
                .as_deref_mut()
                .ok_or_else(|| unavailable("same actual Space AirHB latentGain sink"))?
                .space_latent_gain_mut(space_number)? = 0.0;
            continue;
        }
        let mut space_latent = 0.0;
        for device in space_devices {
            space_latent += device.cached_rates[3];
        }
        *latent
            .as_deref_mut()
            .ok_or_else(|| unavailable("same actual Space AirHB latentGain sink"))?
            .space_latent_gain_mut(space_number)? = space_latent;
        zone_latent += space_latent;
    }
    *latent
        .as_deref_mut()
        .ok_or_else(|| unavailable("same actual Zone AirHB latentGain sink"))?
        .zone_latent_gain_mut(zone_number)? = zone_latent;
    Ok(())
}

impl SelectedInternalHeatGains {
    pub(super) fn update_timestep_device_values(
        &mut self,
        owners: &mut SourceGainStepOwners<'_, '_>,
    ) -> Result<(), E> {
        self.update_source_internal_gain_values(owners, false, false)
    }

    /// Actual IHG8069, including HVAC260/344 explicit true,true arguments.
    pub(crate) fn update_source_internal_gain_values(
        &mut self,
        owners: &mut SourceGainStepOwners<'_, '_>,
        suppress_radiation_update: bool,
        sum_latent_gains: bool,
    ) -> Result<(), E> {
        let do_radiation_update = !suppress_radiation_update;
        let equipment = self.allocations.other_equipment.as_deref();
        let people = &self.allocations.people.rows;
        let spaces = self
            .allocations
            .space_devices
            .as_mut()
            .ok_or_else(|| unavailable("same actual internal-gain pointer cache owner"))?;
        for space_number in 1..=owners.globals.num_spaces {
            let space = spaces
                .get_mut(index(space_number)?)
                .ok_or_else(|| unavailable("actual pointer cache Space association"))?;
            for device_number in 0..space.number_of_devices {
                let device = space
                    .devices
                    .as_mut()
                    .and_then(|rows| rows.get_mut(device_number))
                    .ok_or_else(|| unavailable("actual pointer cache device slot"))?;
                // Source writes in order, so a later unavailable target retains
                // all preceding cache writes. Default suppressRadiation=false.
                for field in 0..7 {
                    if field == 2 && !do_radiation_update {
                        continue;
                    }
                    device.cached_rates[field] = dereference_rate(
                        device.targets[field],
                        equipment,
                        people,
                        self.zero_pointer_value,
                    )? * device.space_gain_fraction;
                }
            }
        }
        if sum_latent_gains {
            let zones = owners
                .zones
                .zones
                .as_deref()
                .ok_or_else(|| unavailable("same actual HVAC latent Zone owner"))?;
            for zone_number in 1..=owners.globals.num_of_zones {
                let zone = zones
                    .get(index(zone_number)?)
                    .ok_or_else(|| unavailable("actual HVAC latent Zone association"))?;
                sum_zone_latent_gains(
                    spaces,
                    zone_number,
                    &zone.space_indexes,
                    &mut owners.latent,
                )?;
                if owners.globals.hybrid_model_people_count {
                    return Err(unavailable(
                        "positive Source SumAllInternalLatentGainsExceptPeople",
                    ));
                }
            }
        }
        if owners.control.generic_contaminant_simulation {
            match &mut owners.zone_generic_gain {
                SourceZoneGenericGainAllocation::Unavailable => {
                    return Err(unavailable("same actual ZoneGCGain allocation state"));
                }
                SourceZoneGenericGainAllocation::Unallocated => {}
                SourceZoneGenericGainAllocation::Allocated(gains) => {
                    let zones = owners
                        .zones
                        .zones
                        .as_deref()
                        .ok_or_else(|| unavailable("same actual generic gain Zone owner"))?;
                    for zone_number in 1..=owners.globals.num_of_zones {
                        let zone_index = index(zone_number)?;
                        let zone = zones
                            .get(zone_index)
                            .ok_or_else(|| unavailable("actual generic gain Zone association"))?;
                        let value = sum_all_zone(spaces, &zone.space_indexes, 6)?;
                        *gains
                            .get_mut(zone_index)
                            .ok_or_else(|| unavailable("actual ZoneGCGain row"))? = value;
                        self.allocations
                            .zone_report
                            .as_mut()
                            .and_then(|rows| rows.get_mut(zone_index))
                            .ok_or_else(|| unavailable("same actual ZoneRpt generic gain row"))?
                            .gc_rate = value;
                    }
                }
            }
        }
        Ok(())
    }
}
