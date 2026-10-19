//! Native ZoneEquipmentManager4195-4510/4717-4908, real retained demand/order path.
use super::{Error, SourceHvacOwners, at};
use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::{
    SourceSystemMoistureDemand, SourceSystemSensibleDemand, SourceZoneAirDemand,
};
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;

// Keep the explicit owner-borrow relationship in this Native-facing helper signature.
#[allow(clippy::needless_lifetimes)]
fn required<'a>(
    demand: &'a mut SourceZoneAirDemand,
    zone: i32,
    space: Option<i32>,
) -> Result<
    (
        &'a mut SourceSystemSensibleDemand,
        &'a mut SourceSystemMoistureDemand,
    ),
    Error,
> {
    let (sensible, moisture, number) = match space {
        Some(number) => (&mut demand.spaces, &mut demand.space_moisture, number),
        None => (&mut demand.zones, &mut demand.zone_moisture, zone),
    };
    let i = at(number)?;
    Ok((
        sensible
            .as_mut()
            .and_then(|rows| rows.get_mut(i))
            .ok_or(Error::Unavailable(
                "actual output-required sensible ordinal4300",
            ))?,
        moisture
            .as_mut()
            .and_then(|rows| rows.get_mut(i))
            .ok_or(Error::Unavailable(
                "actual output-required moisture ordinal4307",
            ))?,
    ))
}

fn list(equipment: &OrdinaryEquipment, zone: i32) -> Result<&super::super::EquipmentList, Error> {
    equipment
        .equipment_lists
        .as_deref()
        .and_then(|rows| rows.get(at(zone).ok()?))
        .ok_or(Error::Unavailable(
            "actual output-required ZoneEquipList ordinal4205",
        ))
}

fn scalar<T: Clone>(rows: &Option<Vec<T>>, i: usize) -> Result<T, Error> {
    rows.as_ref()
        .and_then(|rows| rows.get(i))
        .cloned()
        .ok_or(Error::Unavailable(
            "actual ZoneEquipList allocated scalar4210",
        ))
}

fn fill(rows: &mut Option<Vec<f64>>, value: f64) -> Result<(), Error> {
    rows.as_mut()
        .ok_or(Error::Unavailable("actual allocated sequenced demand4340"))?
        .fill(value);
    Ok(())
}

fn cell(rows: &mut Option<Vec<f64>>, i: usize) -> Result<&mut f64, Error> {
    rows.as_mut()
        .and_then(|rows| rows.get_mut(i))
        .ok_or(Error::Unavailable(
            "actual allocated sequenced demand ordinal4369",
        ))
}

fn fill_full(
    e: &mut SourceSystemSensibleDemand,
    m: &mut SourceSystemMoistureDemand,
) -> Result<(), Error> {
    fill(
        &mut e.sequenced_output_required,
        e.base.total_output_required,
    )?;
    fill(
        &mut e.sequenced_output_to_heating_sp,
        e.output_required_to_heating_sp,
    )?;
    fill(
        &mut e.sequenced_output_to_cooling_sp,
        e.output_required_to_cooling_sp,
    )?;
    fill(
        &mut m.sequenced_output_required,
        m.base.total_output_required,
    )?;
    fill(
        &mut m.sequenced_output_to_humid_sp,
        m.output_required_to_humidifying_sp,
    )?;
    fill(
        &mut m.sequenced_output_to_dehumid_sp,
        m.output_required_to_dehumidifying_sp,
    )?;
    Ok(())
}

/// Native4195: overwrite the real prefix, clear only four unused-row fields,
/// then perform the same nested in-place priority swaps. No alternative sort.
fn set_order(o: &mut SourceHvacOwners<'_, '_>, zone: i32) -> Result<(), Error> {
    let i = at(zone)?;
    let equipment = &mut *o.equipment;
    let actual_list = equipment
        .equipment_lists
        .as_deref()
        .and_then(|rows| rows.get(i))
        .ok_or(Error::Unavailable("actual SetZoneEquipSimOrder list4205"))?;
    let count = actual_list.number_of_equipment_types;
    let order = equipment
        .priority_simulation_order
        .as_mut()
        .ok_or(Error::Unavailable("actual PrioritySimOrder4208"))?;
    for number in 0..count {
        let row = order.get_mut(number).ok_or(Error::Unavailable(
            "actual PrioritySimOrder positive ordinal4208",
        ))?;
        row.equipment_type_name = scalar(&actual_list.equipment_type_names, number)?;
        row.equipment_name = scalar(&actual_list.equipment_names, number)?;
        row.equipment_type = scalar(&actual_list.equipment_types, number)?;
        row.cooling_priority = scalar(&actual_list.cooling_priorities, number)?;
        row.heating_priority = scalar(&actual_list.heating_priorities, number)?;
        row.equipment_pointer = i32::try_from(number + 1).map_err(|_| {
            Error::Unavailable("actual PrioritySimOrder EquipPtr integer domain4215")
        })?;
    }
    for row in order.iter_mut().skip(count) {
        row.equipment_type_name.clear();
        row.equipment_name.clear();
        row.equipment_type = -1;
        row.equipment_pointer = 0;
    }
    for number in 0..count {
        let mut heating_priority = order[number].heating_priority;
        let mut cooling_priority = order[number].cooling_priority;
        for compared in number..count {
            let remaining = o
                .controller
                .demand
                .zones
                .as_ref()
                .and_then(|rows| rows.get(i))
                .ok_or(Error::Unavailable(
                    "actual SetOrder RemainingOutputRequired4237",
                ))?
                .base
                .remaining_output_required;
            if (cooling_priority > order[compared].cooling_priority && remaining < 0.0)
                || (heating_priority > order[compared].heating_priority && remaining >= 0.0)
            {
                // SourceSimulationOrder has exactly the six fields swapped by Native4242-4247.
                order.swap(number, compared);
                cooling_priority = order[number].cooling_priority;
                heating_priority = order[number].heating_priority;
            }
        }
    }
    Ok(())
}

fn init_output(
    o: &mut SourceHvacOwners<'_, '_>,
    zone: i32,
    space: Option<i32>,
    first: bool,
) -> Result<(), Error> {
    {
        let (e, m) = required(&mut o.controller.demand, zone, space)?;
        e.base.remaining_output_required = e.base.total_output_required;
        e.base.unadjusted_remaining_output_required = e.base.total_output_required;
        e.remaining_output_to_heat_sp = e.output_required_to_heating_sp;
        e.unadjusted_remaining_output_to_heat_sp = e.output_required_to_heating_sp;
        e.remaining_output_to_cool_sp = e.output_required_to_cooling_sp;
        e.unadjusted_remaining_output_to_cool_sp = e.output_required_to_cooling_sp;
        m.base.remaining_output_required = m.base.total_output_required;
        m.base.unadjusted_remaining_output_required = m.base.total_output_required;
        m.remaining_output_to_humid_sp = m.output_required_to_humidifying_sp;
        m.unadjusted_remaining_output_to_humid_sp = m.output_required_to_humidifying_sp;
        m.remaining_output_to_dehumid_sp = m.output_required_to_dehumidifying_sp;
        m.unadjusted_remaining_output_to_dehumid_sp = m.output_required_to_dehumidifying_sp;
    }
    // The actual caller uses ResetSimOrder=true. The same routine is called for
    // spaces, but Native's spaceNum==0 condition restricts the actual order write.
    if space.is_none() {
        set_order(o, zone)?;
    }
    let allocated = required(&mut o.controller.demand, zone, space)?
        .0
        .sequenced_output_required
        .is_some();
    if allocated {
        let controlled = o
            .zones
            .zones
            .as_ref()
            .and_then(|rows| rows.get(at(zone).ok()?))
            .ok_or(Error::Unavailable(
                "actual initOutputRequired Zone.IsControlled4326",
            ))?
            .is_controlled;
        if !controlled || o.globals.zone_sizing_calculation {
            let (e, m) = required(&mut o.controller.demand, zone, space)?;
            fill_full(e, m)?;
        } else if first {
            let scheme = list(o.equipment, zone)?.load_distribution_scheme;
            if scheme == 0 || scheme == 1 {
                let (e, m) = required(&mut o.controller.demand, zone, space)?;
                fill_full(e, m)?;
            } else if scheme == 2 || scheme == 3 {
                return Err(Error::Unavailable(
                    "actual initOutputRequired FinalZoneSizing design load4350",
                ));
            }
        } else {
            let (e, _) = required(&mut o.controller.demand, zone, space)?;
            *cell(&mut e.sequenced_output_required, 0)? = e.base.total_output_required;
            *cell(&mut e.sequenced_output_to_heating_sp, 0)? = e.output_required_to_heating_sp;
            *cell(&mut e.sequenced_output_to_cooling_sp, 0)? = e.output_required_to_cooling_sp;
            if o.controller.subsystems.duct_loss.simulated {
                return Err(Error::Unavailable(
                    "actual initOutputRequired DuctLoss.SysSen4372",
                ));
            }
            let (_, m) = required(&mut o.controller.demand, zone, space)?;
            *cell(&mut m.sequenced_output_required, 0)? = m.base.total_output_required;
            *cell(&mut m.sequenced_output_to_humid_sp, 0)? = m.output_required_to_humidifying_sp;
            *cell(&mut m.sequenced_output_to_dehumid_sp, 0)? =
                m.output_required_to_dehumidifying_sp;
            if o.controller.subsystems.duct_loss.simulated {
                return Err(Error::Unavailable(
                    "actual initOutputRequired DuctLoss.SysLat4381",
                ));
            }
        }
    }
    let i = at(zone)?;
    let value = o
        .controller
        .demand
        .deadband_or_setback
        .as_ref()
        .and_then(|rows| rows.get(i))
        .copied()
        .ok_or(Error::Unavailable("actual DeadBandOrSetback4387"))?;
    *o.controller
        .demand
        .current_deadband_or_setback
        .as_mut()
        .and_then(|rows| rows.get_mut(i))
        .ok_or(Error::Unavailable("actual CurDeadBandOrSetback4387"))? = value;
    Ok(())
}

fn fraction(
    owner: &OrdinaryEquipment,
    schedules: &crate::heat_balance::selected_schedule_manager::SelectedScheduleManager,
    zone: i32,
    equipment: i32,
    heating: bool,
) -> Result<f64, Error> {
    let row = list(owner, zone)?;
    let schedule = if heating {
        &row.sequential_heating_fraction_schedules
    } else {
        &row.sequential_cooling_fraction_schedules
    };
    let schedule = scalar(schedule, at(equipment)?)?.ok_or(Error::Unavailable(
        "actual EquipList sequential fraction schedule pointer1810",
    ))?;
    schedules.source_current_schedule_value(schedule)
}

fn distribute_output(
    o: &mut SourceHvacOwners<'_, '_>,
    zone: i32,
    space: Option<i32>,
) -> Result<(), Error> {
    let scheme = list(o.equipment, zone)?.load_distribution_scheme;
    match scheme {
        0 => {
            let equipment = o
                .equipment
                .priority_simulation_order
                .as_ref()
                .and_then(|rows| rows.first())
                .ok_or(Error::Unavailable(
                    "actual PrioritySimOrder first equipment4440",
                ))?
                .equipment_pointer;
            // Native always evaluates heating before cooling, even when one
            // ratio is not selected by TotalOutputRequired's later sign test.
            let heating = fraction(o.equipment, o.schedules, zone, equipment, true)?;
            let cooling = fraction(o.equipment, o.schedules, zone, equipment, false)?;
            let (e, _) = required(&mut o.controller.demand, zone, space)?;
            let ratio = if e.base.total_output_required >= 0.0 {
                heating
            } else {
                cooling
            };
            *cell(&mut e.sequenced_output_required, 0)? = e.base.total_output_required * ratio;
            *cell(&mut e.sequenced_output_to_heating_sp, 0)? =
                e.output_required_to_heating_sp * ratio;
            *cell(&mut e.sequenced_output_to_cooling_sp, 0)? =
                e.output_required_to_cooling_sp * ratio;
            if o.controller.subsystems.duct_loss.simulated {
                return Err(Error::Unavailable(
                    "actual distributeOutputRequired DuctLoss.SysSen4454",
                ));
            }
            let (e, m) = required(&mut o.controller.demand, zone, space)?;
            e.base.remaining_output_required = *cell(&mut e.sequenced_output_required, 0)?;
            e.remaining_output_to_heat_sp = *cell(&mut e.sequenced_output_to_heating_sp, 0)?;
            e.remaining_output_to_cool_sp = *cell(&mut e.sequenced_output_to_cooling_sp, 0)?;
            *cell(&mut m.sequenced_output_required, 0)? = m.base.total_output_required * ratio;
            *cell(&mut m.sequenced_output_to_humid_sp, 0)? =
                m.output_required_to_humidifying_sp * ratio;
            *cell(&mut m.sequenced_output_to_dehumid_sp, 0)? =
                m.output_required_to_dehumidifying_sp * ratio;
            if o.controller.subsystems.duct_loss.simulated {
                return Err(Error::Unavailable(
                    "actual distributeOutputRequired DuctLoss.SysLat4469",
                ));
            }
            let (_, m) = required(&mut o.controller.demand, zone, space)?;
            m.base.remaining_output_required = *cell(&mut m.sequenced_output_required, 0)?;
            m.remaining_output_to_humid_sp = *cell(&mut m.sequenced_output_to_humid_sp, 0)?;
            m.remaining_output_to_dehumid_sp = *cell(&mut m.sequenced_output_to_dehumid_sp, 0)?;
        }
        1 => {
            return Err(Error::Unavailable(
                "actual distributeOutputRequired Uniform NumAvailHeatEquip4482",
            ));
        }
        2 => {
            return Err(Error::Unavailable(
                "actual distributeOutputRequired UniformPLR total load4536",
            ));
        }
        3 => {
            return Err(Error::Unavailable(
                "actual distributeOutputRequired SequentialUniformPLR total load4617",
            ));
        }
        _ => {
            return Err(Error::Unavailable(
                "actual distributeOutputRequired illegal scheme fatal4713",
            ));
        }
    }
    Ok(())
}

/// Exact current actual ResetSimOrder=true caller, including the independently
/// retained doSpaceHeatBalance flag (not the simulation-only proxy).
pub(super) fn initialize(
    o: &mut SourceHvacOwners<'_, '_>,
    zone: i32,
    first: bool,
) -> Result<(), Error> {
    init_output(o, zone, None, first)?;
    if o.space_controls.current {
        let count = o
            .zones
            .zones
            .as_ref()
            .and_then(|rows| rows.get(at(zone).ok()?))
            .ok_or(Error::Unavailable(
                "actual InitSystemOutputRequired spaceIndexes4275",
            ))?
            .space_indexes
            .len();
        for i in 0..count {
            let space = o
                .zones
                .zones
                .as_ref()
                .and_then(|rows| rows.get(at(zone).ok()?))
                .and_then(|zone| zone.space_indexes.get(i))
                .copied()
                .ok_or(Error::Unavailable(
                    "actual InitSystemOutputRequired space ordinal4275",
                ))?;
            init_output(o, zone, Some(space), first)?;
        }
    }
    let controlled = o
        .zones
        .zones
        .as_ref()
        .and_then(|rows| rows.get(at(zone).ok()?))
        .ok_or(Error::Unavailable(
            "actual DistributeSystemOutputRequired Zone.IsControlled4396",
        ))?
        .is_controlled;
    if !controlled {
        return Ok(());
    }
    if o.globals.zone_sizing_calculation {
        return Ok(());
    }
    let scheme = list(o.equipment, zone)?.load_distribution_scheme;
    if first && scheme != 1 && scheme != 0 {
        return Ok(());
    }
    distribute_output(o, zone, None)?;
    if o.space_controls.current {
        let count = o
            .zones
            .zones
            .as_ref()
            .and_then(|rows| rows.get(at(zone).ok()?))
            .ok_or(Error::Unavailable(
                "actual DistributeSystemOutputRequired spaceIndexes4416",
            ))?
            .space_indexes
            .len();
        for i in 0..count {
            let space = o
                .zones
                .zones
                .as_ref()
                .and_then(|rows| rows.get(at(zone).ok()?))
                .and_then(|zone| zone.space_indexes.get(i))
                .copied()
                .ok_or(Error::Unavailable(
                    "actual DistributeSystemOutputRequired space ordinal4416",
                ))?;
            distribute_output(o, zone, Some(space))?;
        }
    }
    Ok(())
}

/// Native4717: update the same demand rows after the actual component returns.
pub(super) fn update(
    o: &mut SourceHvacOwners<'_, '_>,
    zone: i32,
    sensible: f64,
    latent: f64,
    priority: i32,
) -> Result<(), Error> {
    let i = at(zone)?;
    let controlled = o
        .zones
        .zones
        .as_ref()
        .and_then(|rows| rows.get(i))
        .ok_or(Error::Unavailable(
            "actual updateOutputRequired Zone.IsControlled4733",
        ))?
        .is_controlled;
    let demand = &mut o.controller.demand;
    let energy = demand
        .zones
        .as_mut()
        .and_then(|rows| rows.get_mut(i))
        .ok_or(Error::Unavailable(
            "actual updateOutputRequired sensible demand",
        ))?;
    let moisture = demand
        .zone_moisture
        .as_mut()
        .and_then(|rows| rows.get_mut(i))
        .ok_or(Error::Unavailable(
            "actual updateOutputRequired moisture demand",
        ))?;
    if !controlled {
        energy.base.unadjusted_remaining_output_required -= sensible;
        energy.base.remaining_output_required = energy.base.unadjusted_remaining_output_required;
        energy.unadjusted_remaining_output_to_heat_sp -= sensible;
        energy.remaining_output_to_heat_sp = energy.unadjusted_remaining_output_to_heat_sp;
        energy.unadjusted_remaining_output_to_cool_sp -= sensible;
        energy.remaining_output_to_cool_sp = energy.unadjusted_remaining_output_to_cool_sp;
        moisture.base.unadjusted_remaining_output_required -= latent;
        moisture.base.remaining_output_required =
            moisture.base.unadjusted_remaining_output_required;
        moisture.unadjusted_remaining_output_to_humid_sp -= latent;
        moisture.remaining_output_to_humid_sp = moisture.unadjusted_remaining_output_to_humid_sp;
        moisture.unadjusted_remaining_output_to_dehumid_sp -= latent;
        moisture.remaining_output_to_dehumid_sp =
            moisture.unadjusted_remaining_output_to_dehumid_sp;
        let control = *o
            .fan
            .thermostat
            .temp_control_type
            .as_ref()
            .and_then(|rows| rows.get(i))
            .ok_or(Error::Unavailable(
                "actual updateOutputRequired TempControlType4750",
            ))?;
        update_current_deadband(&mut demand.current_deadband_or_setback, i, control, energy)?;
        if priority > -1 {
            let next = next_priority(priority)?;
            if next <= energy.base.num_zone_equipment {
                *cell(&mut energy.sequenced_output_required, at(next)?)? =
                    energy.base.remaining_output_required;
                *cell(&mut moisture.sequenced_output_required, at(next)?)? =
                    moisture.base.remaining_output_required;
            }
            let order = o
                .equipment
                .priority_simulation_order
                .as_ref()
                .and_then(|rows| rows.get(at(priority).ok()?))
                .ok_or(Error::Unavailable(
                    "actual updateOutputRequired current PrioritySimOrder4793",
                ))?;
            let heating = next_priority(order.heating_priority)?;
            if heating <= energy.base.num_zone_equipment {
                *cell(&mut energy.sequenced_output_to_heating_sp, at(heating)?)? =
                    energy.remaining_output_to_heat_sp;
                *cell(&mut moisture.sequenced_output_to_humid_sp, at(heating)?)? =
                    moisture.remaining_output_to_humid_sp;
            }
            let cooling = next_priority(order.cooling_priority)?;
            if cooling <= energy.base.num_zone_equipment {
                *cell(&mut energy.sequenced_output_to_cooling_sp, at(cooling)?)? =
                    energy.remaining_output_to_cool_sp;
                *cell(&mut moisture.sequenced_output_to_dehumid_sp, at(cooling)?)? =
                    moisture.remaining_output_to_dehumid_sp;
            }
        }
        return Ok(());
    }
    let actual_list = list(o.equipment, zone)?;
    match actual_list.load_distribution_scheme {
        0 => {
            energy.base.unadjusted_remaining_output_required -= sensible;
            energy.unadjusted_remaining_output_to_heat_sp -= sensible;
            energy.unadjusted_remaining_output_to_cool_sp -= sensible;
            moisture.base.unadjusted_remaining_output_required -= latent;
            moisture.unadjusted_remaining_output_to_humid_sp -= latent;
            moisture.unadjusted_remaining_output_to_dehumid_sp -= latent;
            let count = i32::try_from(actual_list.number_of_equipment_types).map_err(|_| {
                Error::Unavailable("actual updateOutputRequired NumOfEquipTypes integer domain")
            })?;
            if priority > -1 && priority < count {
                let next = next_priority(priority)?;
                let system = o
                    .equipment
                    .priority_simulation_order
                    .as_ref()
                    .and_then(|rows| rows.get(at(next).ok()?))
                    .ok_or(Error::Unavailable(
                        "actual updateOutputRequired next PrioritySimOrder4828",
                    ))?
                    .equipment_pointer;
                let ratio = if energy.base.total_output_required >= 0.0 {
                    fraction(o.equipment, o.schedules, zone, system, true)?
                } else {
                    fraction(o.equipment, o.schedules, zone, system, false)?
                };
                energy.base.remaining_output_required =
                    ratio * energy.base.unadjusted_remaining_output_required;
                energy.remaining_output_to_heat_sp =
                    ratio * energy.unadjusted_remaining_output_to_heat_sp;
                energy.remaining_output_to_cool_sp =
                    ratio * energy.unadjusted_remaining_output_to_cool_sp;
                moisture.base.remaining_output_required =
                    ratio * moisture.base.unadjusted_remaining_output_required;
                moisture.remaining_output_to_humid_sp =
                    ratio * moisture.unadjusted_remaining_output_to_humid_sp;
                moisture.remaining_output_to_dehumid_sp =
                    ratio * moisture.unadjusted_remaining_output_to_dehumid_sp;
                *cell(&mut energy.sequenced_output_required, at(next)?)? =
                    energy.base.remaining_output_required;
                *cell(&mut energy.sequenced_output_to_heating_sp, at(next)?)? =
                    energy.remaining_output_to_heat_sp;
                *cell(&mut energy.sequenced_output_to_cooling_sp, at(next)?)? =
                    energy.remaining_output_to_cool_sp;
                *cell(&mut moisture.sequenced_output_required, at(next)?)? =
                    moisture.base.remaining_output_required;
                *cell(&mut moisture.sequenced_output_to_humid_sp, at(next)?)? =
                    moisture.remaining_output_to_humid_sp;
                *cell(&mut moisture.sequenced_output_to_dehumid_sp, at(next)?)? =
                    moisture.remaining_output_to_dehumid_sp;
            } else {
                energy.base.remaining_output_required =
                    energy.base.unadjusted_remaining_output_required;
                energy.remaining_output_to_heat_sp = energy.unadjusted_remaining_output_to_heat_sp;
                energy.remaining_output_to_cool_sp = energy.unadjusted_remaining_output_to_cool_sp;
                moisture.base.remaining_output_required =
                    moisture.base.unadjusted_remaining_output_required;
                moisture.remaining_output_to_humid_sp =
                    moisture.unadjusted_remaining_output_to_humid_sp;
                moisture.remaining_output_to_dehumid_sp =
                    moisture.unadjusted_remaining_output_to_dehumid_sp;
            }
            let control = *o
                .fan
                .thermostat
                .temp_control_type
                .as_ref()
                .and_then(|rows| rows.get(i))
                .ok_or(Error::Unavailable(
                    "actual updateOutputRequired TempControlType4863",
                ))?;
            update_current_deadband(&mut demand.current_deadband_or_setback, i, control, energy)?;
        }
        1..=3 => {
            if priority > -1 {
                let next = next_priority(priority)?;
                if next <= energy.base.num_zone_equipment {
                    energy.base.remaining_output_required =
                        *cell(&mut energy.sequenced_output_required, at(next)?)?;
                    moisture.base.remaining_output_required =
                        *cell(&mut moisture.sequenced_output_required, at(next)?)?;
                }
                let order = o
                    .equipment
                    .priority_simulation_order
                    .as_ref()
                    .and_then(|rows| rows.get(at(priority).ok()?))
                    .ok_or(Error::Unavailable(
                        "actual updateOutputRequired current PrioritySimOrder4890",
                    ))?;
                let heating = next_priority(order.heating_priority)?;
                if heating <= energy.base.num_zone_equipment {
                    energy.remaining_output_to_heat_sp =
                        *cell(&mut energy.sequenced_output_to_heating_sp, at(heating)?)?;
                    moisture.remaining_output_to_humid_sp =
                        *cell(&mut moisture.sequenced_output_to_humid_sp, at(heating)?)?;
                }
                let cooling = next_priority(order.cooling_priority)?;
                if cooling <= energy.base.num_zone_equipment {
                    energy.remaining_output_to_cool_sp =
                        *cell(&mut energy.sequenced_output_to_cooling_sp, at(cooling)?)?;
                    moisture.remaining_output_to_dehumid_sp =
                        *cell(&mut moisture.sequenced_output_to_dehumid_sp, at(cooling)?)?;
                }
            }
        }
        _ => {
            return Err(Error::Unavailable(
                "actual updateOutputRequired illegal LoadDist fatal4904",
            ));
        }
    }
    Ok(())
}

fn next_priority(value: i32) -> Result<i32, Error> {
    value.checked_add(1).ok_or(Error::Unavailable(
        "actual updateOutputRequired next priority integer domain",
    ))
}

fn update_current_deadband(
    rows: &mut Option<Vec<bool>>,
    i: usize,
    control: i32,
    energy: &SourceSystemSensibleDemand,
) -> Result<(), Error> {
    let value = match control {
        0 => Some(false),
        1 => Some((energy.base.remaining_output_required - 1.0) < 0.0),
        2 => Some((energy.base.remaining_output_required + 1.0) > 0.0),
        3 | 4 => Some(
            energy.remaining_output_to_heat_sp < 0.0 && energy.remaining_output_to_cool_sp > 0.0,
        ),
        _ => None,
    };
    if let Some(value) = value {
        *rows
            .as_mut()
            .and_then(|rows| rows.get_mut(i))
            .ok_or(Error::Unavailable(
                "actual updateOutputRequired CurDeadBandOrSetback ordinal",
            ))? = value;
    }
    Ok(())
}
