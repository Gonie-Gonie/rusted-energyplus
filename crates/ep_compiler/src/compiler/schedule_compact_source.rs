//! Bounded Schedule:Constant/Compact source semantics.

use ep_model::{
    ScheduleCompactDayProfile, ScheduleCompactPeriod, ScheduleCompactSegment, ScheduleDayType,
    ScheduleInterpolation, ScheduleTypeLimitId, TypedModel,
};
use ep_raw_model::RawObject;

use super::{
    ALL_SCHEDULE_DAY_TYPES, CompactSchedulePeriodBuilder, Compiler, process_week_compact_day_types,
};

impl Compiler<'_> {
    pub(super) fn selected_schedule_type_limits(
        &mut self,
        model: &TypedModel,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
    ) -> Option<ScheduleTypeLimitId> {
        const FIELD: &str = "schedule_type_limits_name";
        let reference =
            self.optional_reference_name_checked(object_type, object_name, object, FIELD)?;
        let Some(reference) = reference else {
            self.warning(
                "MissingScheduleTypeLimits",
                object_type,
                Some(object_name),
                Some(FIELD),
                format!("{object_type}/{object_name} has no ScheduleTypeLimits reference; schedule will not be validated"),
            );
            return None;
        };
        if let Some(id) = model.schedule_type_limit_names.resolve(&reference) {
            return Some(id);
        }
        self.warning(
            "MissingReference",
            object_type,
            Some(object_name),
            Some(FIELD),
            format!("{object_type}/{object_name} references missing ScheduleTypeLimits '{reference}'; schedule will not be validated"),
        );
        None
    }

    pub(super) fn compact_schedule_day_types(
        &mut self,
        object_name: &str,
        directive: &str,
        assigned_day_types: &mut [bool; 12],
    ) -> Vec<ScheduleDayType> {
        // ProcessForDayTypes mutates the mask even when the selector reports an error.
        let selection = process_week_compact_day_types(directive, assigned_day_types);
        if selection.duplicate {
            self.error(
                "DuplicateScheduleCompactDayType",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!("Schedule:Compact/{object_name} attempts a duplicate day assignment in '{directive}'"),
            );
        }
        if !selection.recognized {
            self.error(
                "UnsupportedScheduleCompactDayType",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!(
                    "Schedule:Compact/{object_name} has no valid day assignments in '{directive}'"
                ),
            );
        }
        if selection.duplicate || !selection.recognized {
            return Vec::new();
        }
        ALL_SCHEDULE_DAY_TYPES
            .into_iter()
            .zip(selection.selected)
            .filter_map(|(day_type, selected)| selected.then_some(day_type))
            .collect()
    }

    pub(super) fn finish_compact_schedule_period(
        &mut self,
        object_name: &str,
        mut period: CompactSchedulePeriodBuilder,
        periods: &mut Vec<ScheduleCompactPeriod>,
    ) {
        if period.period.day_profiles.is_empty() {
            self.error(
                "MissingScheduleCompactFor",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!("Schedule:Compact/{object_name} Through period requires at least one For profile"),
            );
        }
        let missing_day_types = ALL_SCHEDULE_DAY_TYPES
            .into_iter()
            .zip(period.assigned_day_types)
            .filter_map(|(day_type, assigned)| (!assigned).then_some(day_type))
            .collect::<Vec<_>>();
        if !missing_day_types.is_empty() {
            self.warning(
                "IncompleteScheduleCompactDayTypes",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!("Schedule:Compact/{object_name} has missing day types; missing day types use the zero MissingDay schedule"),
            );
            // Native AddWeekSchedule starts every real pointer at MissingDaySchedule-0.0.
            // Keep an empty/malformed period invalid rather than manufacturing its For rule.
            if !period.period.day_profiles.is_empty() {
                period.period.day_profiles.push(ScheduleCompactDayProfile {
                    day_types: missing_day_types,
                    interpolation: ScheduleInterpolation::No,
                    segments: vec![ScheduleCompactSegment {
                        until_minute_of_day: 1440,
                        value: 0.0,
                    }],
                });
            }
        }
        periods.push(period.period);
    }

    pub(super) fn compact_schedule_until_minute(
        &mut self,
        object_name: &str,
        directive: &str,
        interpolation: ScheduleInterpolation,
        minutes_per_timestep: Option<u32>,
        previous_endpoint: Option<u32>,
    ) -> Option<u32> {
        let Some((minute_of_day, raw_minute, clamped)) =
            decode_compact_until(directive, previous_endpoint)
        else {
            self.error(
                "InvalidScheduleCompactUntil",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!("Schedule:Compact/{object_name} has invalid Until directive '{directive}'"),
            );
            return None;
        };
        // DecodeHHMMField checks the original minute before ProcessIntervalFields clamps 24:mm.
        if interpolation == ScheduleInterpolation::No
            && minutes_per_timestep.is_some_and(|minutes| raw_minute % minutes != 0)
        {
            self.warning(
                "ScheduleCompactUntilNotAlignedToTimestep",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!("Schedule:Compact/{object_name} Until minute {raw_minute} is not a multiple of the minutes per zone timestep"),
            );
        }
        if clamped {
            self.warning(
                "ScheduleCompactUntilClampedToEndOfDay",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!("Schedule:Compact/{object_name} terminates Until '{directive}' at 24:00"),
            );
        }
        Some(minute_of_day)
    }
}

fn decode_compact_until(value: &str, previous_endpoint: Option<u32>) -> Option<(u32, u32, bool)> {
    // Native ProcessIntervalFields2787-2795 requires the keyword at byte0
    // and erases a colon only at byte5. A space before ':' can instead be a
    // genuine blank HH (`Until :30`), not an optional keyword separator.
    if value
        .chars()
        .any(|character| character.is_whitespace() && character != ' ')
        || !value.get(..5)?.eq_ignore_ascii_case("Until")
    {
        return None;
    }
    let field = value.get(5..)?;
    let time = field.strip_prefix(':').unwrap_or(field).trim_matches(' ');
    let (hour, minute) = time.split_once(':')?;
    let integral = |text: &str| {
        let value = ep_model::process_source_number(text).ok()?;
        // Nonintegral positive operands reach Native's separate formatting-
        // fatal path. Keep them outside this completing scalar projection.
        if value < 0.0 || value != value.trunc() || value > f64::from(i32::MAX) {
            return None;
        }
        Some(value as u32)
    };
    // HH at first ':' and an existing empty MM both genuinely mean zero.
    let hour = integral(hour)?;
    let minute = integral(minute)?;
    if hour > 24 || minute > 60 {
        return None;
    }
    // Native MM60 does not assign endHr/endMin. Only a previous real decoded
    // endpoint makes this completing projection defined: None is initial
    // (-1,-1), while Some(0) is a prior explicit0:0's (-1,59).
    if minute == 60 {
        return Some((previous_endpoint?, minute, false));
    }
    let clamped = hour == 24 && minute > 0;
    Some((
        hour * 60 + if clamped { 0 } else { minute },
        minute,
        clamped,
    ))
}
