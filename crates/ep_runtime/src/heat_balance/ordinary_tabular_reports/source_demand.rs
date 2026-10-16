//! Actual source-energy3962–4084 and peak-demand4086–4225 gathers.
use super::*;
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;
fn schedule_current(
    o: &SourceTabularInputOwners<'_, '_>,
    pointer: Option<SourceScheduleIndex>,
) -> Result<f64, Error> {
    let index = pointer.ok_or(Error::Unavailable(
        "actual fuel-factor Schedule nullptr4055",
    ))?;
    let row = o
        .schedules
        .schedules
        .get(index.0)
        .ok_or(Error::Unavailable("same actual fuel-factor Schedule4055"))?;
    Ok(if row.ems_actuated_on {
        row.ems_value
    } else {
        row.current_value
    })
}
fn native_i32(value: f64) -> Result<i32, Error> {
    if !value.is_finite()
        || value.trunc() < f64::from(i32::MIN)
        || value.trunc() > f64::from(i32::MAX)
    {
        return Err(Error::Unavailable(
            "actual DetermineMinute defined int domain",
        ));
    }
    Ok(value.trunc() as i32)
}
pub(super) fn peak_timestamp(o: &SourceTabularInputOwners<'_, '_>) -> Result<i32, Error> {
    // Native truncates CurrentTime before subtraction, then the entire product.
    let minute = native_i32(
        ((o.clock.current_time + o.hvac.system_time_elapsed)
            - f64::from(native_i32(o.clock.current_time)?))
            * 60.0,
    )?;
    o.environment
        .month
        .checked_mul(100)
        .and_then(|n| n.checked_add(o.environment.day_of_month))
        .and_then(|n| n.checked_mul(100))
        .and_then(|n| n.checked_add(o.clock.hour_of_day))
        .and_then(|n| n.checked_mul(100))
        .and_then(|n| n.checked_add(minute))
        .ok_or(Error::Unavailable(
            "actual EncodeMonDayHrMin defined int domain",
        ))
}
impl SourceTabularReports {
    pub(super) fn gather_source_energy_end_use_results(
        &mut self,
        time_step: TimeStepKind,
        o: &SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.display_source_energy_end_use_summary && time_step == TimeStepKind::Zone {
            let s = &mut self.input_state;
            for resource in 1..=14 {
                if s.ff_schedule_used[resource - 1] {
                    let meter = s.meter_num_totals_beps[resource - 1];
                    if meter > -1 {
                        let value = o.registration.current_source_meter_value(meter)?
                            * schedule_current(o, s.ff_schedules[resource - 1])?
                            * s.source_factors[resource - 1];
                        s.gather_totals_by_source_beps[resource - 1] += value;
                    }
                } else {
                    let meter = s.meter_num_totals_beps[resource - 1];
                    if meter > -1 {
                        let value = o.registration.current_source_meter_value(meter)?
                            * s.source_factors[resource - 1];
                        s.gather_totals_by_source_beps[resource - 1] += value;
                    }
                }
                for end_use in 1..=14 {
                    if s.ff_schedule_used[resource - 1] {
                        let meter = s.meter_num_end_use_beps.read([resource, end_use])?;
                        if meter > -1 {
                            let value = o.registration.current_source_meter_value(meter)?
                                * schedule_current(o, s.ff_schedules[resource - 1])?
                                * s.source_factors[resource - 1];
                            let old = s.gather_end_use_by_source_beps.read([resource, end_use])?;
                            s.gather_end_use_by_source_beps
                                .set([resource, end_use], old + value)?;
                        }
                    } else {
                        let meter = s.meter_num_end_use_beps.read([resource, end_use])?;
                        if meter > -1 {
                            let value = o.registration.current_source_meter_value(meter)?
                                * s.source_factors[resource - 1];
                            let old = s.gather_end_use_by_source_beps.read([resource, end_use])?;
                            s.gather_end_use_by_source_beps
                                .set([resource, end_use], old + value)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
    pub(super) fn gather_source_peak_demand(
        &mut self,
        time_step: TimeStepKind,
        o: &SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        // Native assertion precedes both report predicates, also for System.
        if o.clock.time_step_zone_sec <= 0.0 || o.clock.time_step_zone_sec.is_nan() {
            return Err(Error::Unavailable(
                "actual GatherPeakDemand4159 positive zone seconds assertion",
            ));
        }
        let s = &mut self.input_state;
        if self.display_demand_end_use && time_step == TimeStepKind::Zone {
            for resource in 1..=14 {
                let meter = s.meter_num_totals_beps[resource - 1];
                if meter > -1 {
                    let value = o.registration.current_source_meter_value(meter)?
                        / o.clock.time_step_zone_sec;
                    if value > s.gather_demand_total[resource - 1] {
                        s.gather_demand_total[resource - 1] = value;
                        s.gather_demand_time_stamp[resource - 1] = peak_timestamp(o)?;
                        for end_use in 1..=14 {
                            let meter = s.meter_num_end_use_beps.read([resource, end_use])?;
                            if meter > -1 {
                                let value = o.registration.current_source_meter_value(meter)?
                                    / o.clock.time_step_zone_sec;
                                s.gather_demand_end_use.set([resource, end_use], value)?;
                                let category = o
                                    .registration
                                    .end_use_categories
                                    .as_ref()
                                    .and_then(|v| v.get(end_use - 1))
                                    .ok_or(Error::Unavailable(
                                        "actual peak-demand EndUseCategory4186",
                                    ))?;
                                for ordinal in 1..=category.num_subcategories {
                                    let subcategory = usize::try_from(ordinal).map_err(|_| {
                                        Error::Unavailable("actual peak-demand subcategory ordinal")
                                    })?;
                                    let indices = [subcategory, end_use, resource];
                                    let meter = s.meter_num_end_use_sub_beps.read(indices)?;
                                    if meter > -1 {
                                        let value =
                                            o.registration.current_source_meter_value(meter)?
                                                / o.clock.time_step_zone_sec;
                                        s.gather_demand_end_use_sub.set(indices, value)?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if self.display_leed_summary && time_step == TimeStepKind::Zone {
            for resource in 1..=14 {
                for end_use in 1..=14 {
                    let meter = s.meter_num_end_use_beps.read([resource, end_use])?;
                    if meter > -1 {
                        let value = o.registration.current_source_meter_value(meter)?
                            / o.clock.time_step_zone_sec;
                        if value > s.gather_demand_ind_end_use.read([resource, end_use])? {
                            s.gather_demand_ind_end_use
                                .set([resource, end_use], value)?;
                        }
                        let category = o
                            .registration
                            .end_use_categories
                            .as_ref()
                            .and_then(|v| v.get(end_use - 1))
                            .ok_or(Error::Unavailable(
                                "actual individual peak EndUseCategory4211",
                            ))?;
                        for ordinal in 1..=category.num_subcategories {
                            let subcategory = usize::try_from(ordinal).map_err(|_| {
                                Error::Unavailable("actual individual peak subcategory ordinal")
                            })?;
                            let indices = [subcategory, end_use, resource];
                            let meter = s.meter_num_end_use_sub_beps.read(indices)?;
                            if meter > -1 {
                                let value = o.registration.current_source_meter_value(meter)?
                                    / o.clock.time_step_zone_sec;
                                if value > s.gather_demand_ind_end_use_sub.read(indices)? {
                                    s.gather_demand_ind_end_use_sub.set(indices, value)?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
