//! Genuine thermostat setpoint6558 and schedule6674 predefined reports.
use super::*;
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_schedule_manager::{SelectedScheduleManager, SourceScheduleIndex};
use crate::weather::day::WeatherOwnerState;

struct Report<'a> {
    schedules: &'a SelectedScheduleManager,
    environment: &'a WeatherEnvironmentState,
    weather: &'a WeatherOwnerState,
    clock: &'a WeatherGlobalState,
    registry: &'a PredefinedRegistry,
    table: &'a mut PredefinedStringTable,
}
fn column(registry: &PredefinedRegistry, name: &'static str) -> Result<i32, Error> {
    registry.named_indices.get(name).copied().ok_or(Error::Unavailable(name))
}
fn text(table: &mut PredefinedStringTable, registry: &PredefinedRegistry,
    column_name: &'static str, object: &str, value: &str) -> Result<(), Error> {
    table.string_entry(column(registry, column_name)?, object, value).map_err(Error::from)
}
fn pointer(index: Option<SourceScheduleIndex>) -> Result<SourceScheduleIndex, Error> {
    index.ok_or(Error::Unavailable("actual thermostat predefined null schedule dereference"))
}
fn schedule(manager: &SelectedScheduleManager, index: SourceScheduleIndex)
    -> Result<&crate::heat_balance::selected_schedule_manager::SelectedSchedule, Error> {
    manager.schedules.get(index.0).ok_or(Error::Unavailable(
        "actual thermostat predefined retained schedule pointer",
    ))
}
impl Report<'_> {
    fn times(&mut self, index: SourceScheduleIndex, summer: bool, object: &str) -> Result<String, Error> {
        let (at_11, days, _) = self.schedules.source_value_and_count_on_day(
            index, summer, 4, 11, self.environment, self.weather, self.clock,
        )?;
        self.table.fixed_real_entry_2(column(self.registry, "pdchLeedSchStPt11amWednesday")?, object, at_11)?;
        self.table.integer_entry(column(self.registry, "pdchLeedSchStPt11amWedCnt")?, object, days)?;
        let (at_23, days, month) = self.schedules.source_value_and_count_on_day(
            index, summer, 4, 23, self.environment, self.weather, self.clock,
        )?;
        self.table.fixed_real_entry_2(column(self.registry, "pdchLeedSchStPt11pmWednesday")?, object, at_23)?;
        self.table.integer_entry(column(self.registry, "pdchLeedSchStPt11pmWedCnt")?, object, days)?;
        Ok(month)
    }
    fn first(&mut self, index: SourceScheduleIndex, setpoint: &str) -> Result<(), Error> {
        text(self.table, self.registry, "pdChLeedSchStPtFirstObjUsed",
            &schedule(self.schedules, index)?.name, setpoint)
    }
    fn ordinary(&mut self, index: SourceScheduleIndex, summer: bool, setpoint: &str) -> Result<(), Error> {
        self.first(index, setpoint)?;
        let schedules = self.schedules;
        let name = &schedule(schedules, index)?.name;
        let month = self.times(index, summer, name)?;
        text(self.table, self.registry, "pdChLeedSchStPtMonthUsed", name, &month)
    }
}
impl SourceZoneAirPredictorCorrector {
    pub(crate) fn fill_source_predefined_table_on_thermostat_setpoints(
        &self, schedules: &SelectedScheduleManager, environment: &WeatherEnvironmentState,
        weather: &WeatherOwnerState, clock: &WeatherGlobalState,
        registry: &PredefinedRegistry, table: &mut PredefinedStringTable,
    ) -> Result<(), Error> {
        let mut unique_schedules = Vec::<i32>::new();
        let integer = |kind| i32::try_from(self.flags.number_temperature_controls[kind])
            .map_err(|_| Error::Unavailable("actual thermostat NumTempControls int domain6565"));
        let heating = integer(1)?;
        let cooling = integer(2)?;
        let single = integer(3)?;
        let dual = integer(4)?;
        let capacity = heating.checked_add(cooling)
            .and_then(|v| v.checked_add(single))
            .and_then(|v| v.checked_add(dual.checked_mul(2)?))
            .ok_or(Error::Unavailable("actual thermostat reserve signed expression6565"))?;
        unique_schedules.reserve_exact(usize::try_from(capacity)
            .map_err(|_| Error::Unavailable("actual thermostat reserve size_t domain"))?);
        let mut report = Report { schedules, environment, weather, clock, registry, table };
        // Native traverses the allocated Array1D rows in enum-kind order.
        for kind in [1, 2, 3, 4] {
            for setpoint in self.flags.temperature_setpoint_schedules[kind].as_deref().unwrap_or(&[]) {
                let first = pointer(if kind == 2 { setpoint.cool_schedule } else { setpoint.heat_schedule })?;
                let number = i32::try_from(schedule(schedules, first)?.number.0)
                    .map_err(|_| Error::Unavailable("actual thermostat schedule Num int domain"))?;
                if !unique_schedules.contains(&number) {
                    unique_schedules.push(number);
                    if kind == 3 {
                        report.first(first, &setpoint.name)?;
                        let name = &schedule(schedules, first)?.name;
                        let summer = report.times(first, true, &format!("{name} (summer)"))?;
                        let winter = report.times(first, false, &format!("{name} (winter)"))?;
                        text(report.table, registry, "pdChLeedSchStPtMonthUsed", name,
                            &format!("{summer} and {winter}"))?;
                    } else {
                        report.ordinary(first, kind == 2, &setpoint.name)?;
                    }
                }
                // Dual's cool pointer is consumed independently AFTER heat's
                // complete report, including when heat is already in uniqSch.
                if kind == 4 {
                    let cool = pointer(setpoint.cool_schedule)?;
                    let number = i32::try_from(schedule(schedules, cool)?.number.0)
                        .map_err(|_| Error::Unavailable("actual thermostat cool Num int domain"))?;
                    if !unique_schedules.contains(&number) {
                        unique_schedules.push(number);
                        report.ordinary(cool, true, &setpoint.name)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn fill_source_predefined_table_on_thermostat_schedules(
        &self, schedules: &SelectedScheduleManager,
        registry: &PredefinedRegistry, table: &mut PredefinedStringTable,
    ) -> Result<(), Error> {
        #[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
        struct ControlTypeInfo {
            thermostat_type: String,
            control_type_name: String,
            heat_schedule_name: String,
            cool_schedule_name: String,
        }
        fn join(infos: &[ControlTypeInfo], member: fn(&ControlTypeInfo) -> &str) -> String {
            let mut result = Vec::with_capacity(infos.len());
            for info in infos {
                let value = member(info).to_owned();
                if value.is_empty() { continue; }
                result.push(value);
            }
            result.join(", ")
        }
        const TYPE_NAMES: [&str; 5] = [
            "Uncontrolled", "SingleHeating", "SingleCooling", "SingleHeatCool", "DualSetPointWithDeadBand",
        ];
        for index in 0..self.controls.number_temperature_zones {
            let zone = self.controls.temperature_controlled_zones.as_deref()
                .and_then(|v| v.get(index as usize)).ok_or(Error::Unavailable(
                    "actual thermostat predefined TempControlledZone(idx)",
                ))?;
            text(table, registry, "pdchStatName", &zone.zone_name, &zone.name)?;
            text(table, registry, "pdchStatCtrlTypeSchd", &zone.zone_name,
                &schedule(schedules, pointer(zone.control_type_schedule)?)?.name)?;
            let mut infos = vec![ControlTypeInfo::default(); TYPE_NAMES.len()];
            for (kind, type_name) in TYPE_NAMES.iter().enumerate().skip(1) {
                let setpoint = &zone.setpoints[kind];
                let info = &mut infos[kind];
                if setpoint.name.is_empty() { continue; }
                info.thermostat_type = (*type_name).to_owned();
                info.control_type_name = setpoint.name.clone();
                match kind {
                    3 | 4 => {
                        info.cool_schedule_name = schedule(schedules, pointer(setpoint.cool_schedule)?)?.name.clone();
                        info.heat_schedule_name = schedule(schedules, pointer(setpoint.heat_schedule)?)?.name.clone();
                    }
                    2 => info.cool_schedule_name = schedule(schedules, pointer(setpoint.cool_schedule)?)?.name.clone(),
                    1 => info.heat_schedule_name = schedule(schedules, pointer(setpoint.heat_schedule)?)?.name.clone(),
                    _ => return Err(Error::Unavailable("actual controlledSetptTypes enum domain")),
                }
                // Native emplace_back(std::move(info)) moves the four strings
                // out of the original enum slot; pinned libstdc++ leaves empty.
                let moved = std::mem::take(info);
                infos.push(moved);
            }
            infos.sort_unstable();
            text(table, registry, "pdchStatSchdType1", &zone.zone_name,
                &join(&infos, |x| &x.thermostat_type))?;
            text(table, registry, "pdchStatSchdTypeName1", &zone.zone_name,
                &join(&infos, |x| &x.control_type_name))?;
            let heat = join(&infos, |x| &x.heat_schedule_name);
            if !heat.is_empty() { text(table, registry, "pdchStatSchdHeatName", &zone.zone_name, &heat)?; }
            let cool = join(&infos, |x| &x.cool_schedule_name);
            if !cool.is_empty() { text(table, registry, "pdchStatSchdCoolName", &zone.zone_name, &cool)?; }
        }
        Ok(())
    }
}
