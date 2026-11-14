//! Native SizingManager2176-2433 and simple DataSizing635-1394 DSOA owners.
//! Positive OA equipment admission, outdoor Nodes, CO2 and SpaceList remain separate.
use super::{
    LocalArrays, SelectedOAFlowMethod as Method, SelectedOARequirement, SelectedSizingError as E,
    SelectedSizingManager, local_arrays,
};
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as FlowError, SelectedHeatBalanceInputState,
    zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputStatisticsValue as V,
};
use crate::heat_balance::selected_schedule_manager::{
    SelectedScheduleError, SelectedScheduleManager,
};
use crate::weather::day::WeatherGlobalState;

const FAMILY: &str = "DesignSpecification:OutdoorAir";
const FIELDS: [&str; 8] = [
    "name",
    "outdoor_air_method",
    "outdoor_air_flow_per_person",
    "outdoor_air_flow_per_zone_floor_area",
    "outdoor_air_flow_per_zone",
    "outdoor_air_flow_air_changes_per_hour",
    "outdoor_air_schedule_name",
    "proportional_control_minimum_outdoor_air_flow_rate_schedule_name",
];

fn json_number(token: &str) -> Result<f64, E> {
    if token.bytes().any(|byte| matches!(byte, b'.' | b'e' | b'E')) {
        token
            .parse()
            .map_err(|_| E::Unavailable("actual DSOA JSON double"))
    } else if let Ok(value) = token.parse::<i64>() {
        Ok(value as f64)
    } else if let Ok(value) = token.parse::<u64>() {
        // Native setObjectItemValue842 gets int64 even for an unsigned JSON integer.
        Ok((value as i64) as f64)
    } else {
        // Native nlohmann lexer8310 falls from overflowing integer storage to double.
        token
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
            .ok_or(E::Unavailable(
                "actual DSOA overflowing integer JSON double",
            ))
    }
}

fn get_object(
    input: &mut SelectedInputProcessor,
    number: usize,
    local: &mut LocalArrays,
) -> Result<(usize, usize), E> {
    let name = input
        .source_raw_object_names(FAMILY)?
        .get(
            number
                .checked_sub(1)
                .ok_or(E::Unavailable("actual DSOA getObjectItem ordinal"))?,
        )
        .cloned()
        .ok_or(E::Unavailable("actual DSOA getObjectItem row"))?;
    let fields = input.source_raw_object_fields(FAMILY, &name)?;
    let maximum = FIELDS.iter().enumerate().fold(1, |maximum, (at, field)| {
        if fields.contains_key(*field) {
            maximum.max(at + 1)
        } else {
            maximum
        }
    });
    let (alphas, alpha_names, numeric_names, numbers, alpha_blanks, numeric_blanks) = local;
    if alphas.len() < 4
        || alpha_names.len() < 4
        || alpha_blanks.len() < 4
        || numbers.len() < 4
        || numeric_names.len() < 4
        || numeric_blanks.len() < 4
    {
        return Err(E::Unavailable("actual DSOA local argument allocation"));
    }
    alphas.fill(String::new());
    numbers.fill(0.0);
    numeric_blanks.fill(true);
    alpha_blanks.fill(true);
    alpha_names.fill(String::new());
    numeric_names.fill(String::new());
    input.mark_object_as_used(FAMILY, &name);
    let fields = input.source_raw_object_fields(FAMILY, &name)?;
    let (mut alpha, mut numeric, mut num_alphas, mut num_numbers) = (0, 0, 0, 0);
    for (at, field) in FIELDS.iter().enumerate() {
        if at == 0 {
            alphas[alpha] = name.to_ascii_uppercase();
            alpha_blanks[alpha] = name.is_empty();
            alpha_names[alpha] = (*field).into();
            alpha += 1;
            num_alphas = alpha;
        } else if matches!(at, 1 | 6 | 7) {
            let default = if at == 1 { "Flow/Person" } else { "" };
            let (value, blank) = match fields.get(*field) {
                None => (default, true),
                Some(V::String(value)) if value.is_empty() => (default, true),
                Some(V::String(value)) => (value.as_str(), false),
                _ => return Err(E::Unavailable("actual DSOA alpha string scalar")),
            };
            alphas[alpha] = value.to_ascii_uppercase();
            alpha_blanks[alpha] = blank;
            alpha_names[alpha] = (*field).into();
            alpha += 1;
            if at < maximum {
                num_alphas = alpha;
            }
        } else {
            let default = if at == 2 { 0.00944 } else { 0.0 };
            let (value, blank) = match fields.get(*field) {
                None => (default, true),
                Some(V::String(value)) if value.is_empty() => (default, true),
                Some(V::String(_)) => (-99999.0, false),
                Some(V::Number(token)) => (json_number(token)?, false),
                _ => return Err(E::Unavailable("actual DSOA numeric scalar")),
            };
            numbers[numeric] = value;
            numeric_blanks[numeric] = blank;
            numeric_names[numeric] = (*field).into();
            numeric += 1;
            if at < maximum {
                num_numbers = numeric;
            }
        }
    }
    Ok((num_alphas, num_numbers))
}

impl Method {
    fn from_alpha(value: &str) -> Self {
        match value {
            "FLOW/PERSON" => Self::PerPerson,
            "FLOW/ZONE" => Self::PerZone,
            "FLOW/AREA" => Self::PerArea,
            "AIRCHANGES/HOUR" => Self::AirChanges,
            "SUM" => Self::Sum,
            "MAXIMUM" => Self::Maximum,
            "INDOORAIRQUALITYPROCEDURE" => Self::IndoorAirQualityProcedure,
            "PROPORTIONALCONTROLBASEDONOCCUPANCYSCHEDULE" => Self::ProportionalOccupancySchedule,
            "PROPORTIONALCONTROLBASEDONDESIGNOCCUPANCY" => Self::ProportionalDesignOccupancy,
            _ => Self::Invalid,
        }
    }
    fn uses_people(self) -> bool {
        matches!(
            self,
            Self::PerPerson
                | Self::Sum
                | Self::Maximum
                | Self::IndoorAirQualityProcedure
                | Self::ProportionalOccupancySchedule
                | Self::ProportionalDesignOccupancy
        )
    }
    fn uses_area(self) -> bool {
        matches!(
            self,
            Self::PerArea
                | Self::Sum
                | Self::Maximum
                | Self::IndoorAirQualityProcedure
                | Self::ProportionalOccupancySchedule
                | Self::ProportionalDesignOccupancy
        )
    }
    fn uses_zone(self) -> bool {
        matches!(
            self,
            Self::PerZone | Self::Sum | Self::Maximum | Self::IndoorAirQualityProcedure
        )
    }
    fn uses_air_changes(self) -> bool {
        matches!(
            self,
            Self::AirChanges | Self::Sum | Self::Maximum | Self::IndoorAirQualityProcedure
        )
    }
    fn co2_prerequisite_name(self) -> Option<&'static str> {
        match self {
            Self::IndoorAirQualityProcedure => Some("IndoorAirQualityProcedure"),
            // The two literal Native prerequisite diagnostic labels are reversed.
            Self::ProportionalOccupancySchedule => {
                Some("ProportionalControlBasedOnDesignOccupancy")
            }
            Self::ProportionalDesignOccupancy => {
                Some("ProportionalControlBasedOnOccupancySchedule")
            }
            _ => None,
        }
    }
}

impl SelectedSizingManager {
    pub(crate) fn get_oa_requirements(
        &mut self,
        input: &mut SelectedInputProcessor,
        schedules: &mut SelectedScheduleManager,
        clock: &WeatherGlobalState,
        source_is_epjson: bool,
    ) -> Result<(), E> {
        let count = input.get_num_objects_found(FAMILY);
        let args = input.get_object_def_max_args(FAMILY)?;
        let lists = input.get_num_objects_found("DesignSpecification:OutdoorAir:SpaceList");
        self.number_of_oa_requirements = count
            .checked_add(lists)
            .ok_or(E::Unavailable("OA count size"))?;
        let mut local = local_arrays(args);
        self.number_of_oa_requirements = self
            .number_of_oa_requirements
            .checked_add(1)
            .ok_or(E::Unavailable("default OA count size"))?;
        self.oa_requirements = Some(vec![
            SelectedOARequirement::default();
            self.number_of_oa_requirements
        ]);
        self.default_oa_requirement = self.number_of_oa_requirements;
        let default = self
            .oa_requirements
            .as_mut()
            .and_then(|rows| rows.last_mut())
            .ok_or(E::Unavailable("OA allocation"))?;
        default.name = "Default-0.00944[m3/s-person]".into();
        default.flow_method = Method::PerPerson;
        default.flow_per_person = 0.00944;
        default.flow_fraction_schedule = Some(schedules.always_on()?);
        if count > 0 && !source_is_epjson {
            return Err(E::Unavailable("actual DSOA IDF declaration/count metadata"));
        }
        for number in 1..=count {
            let (num_alphas, num_numbers) = get_object(input, number, &mut local)?;
            let (alphas, _, _, numbers, alpha_blanks, _) = &local;
            let row = self
                .oa_requirements
                .as_mut()
                .and_then(|rows| rows.get_mut(number - 1))
                .ok_or(E::Unavailable("actual DSOA input row"))?;
            row.name.clone_from(&alphas[0]);
            row.flow_method = if num_alphas > 1 {
                Method::from_alpha(&alphas[1])
            } else {
                Method::PerPerson
            };
            if row.flow_method == Method::Invalid {
                return Err(E::Unavailable(
                    "actual DSOA invalid-method severe/continuation child2360",
                ));
            }
            row.flow_per_person = if num_numbers > 0 { numbers[0] } else { 0.00944 };
            if !row.flow_method.uses_people() {
                row.flow_per_person = 0.0;
            }
            if num_numbers > 1 {
                row.flow_per_area = if row.flow_method.uses_area() {
                    numbers[1]
                } else {
                    0.0
                };
            }
            if num_numbers > 2 {
                row.flow_per_zone = if row.flow_method.uses_zone() {
                    numbers[2]
                } else {
                    0.0
                };
            }
            if num_numbers > 3 {
                row.flow_air_changes_per_hour = if row.flow_method.uses_air_changes() {
                    numbers[3]
                } else {
                    0.0
                };
            }
            if num_alphas <= 2 || alpha_blanks[2] {
                row.flow_fraction_schedule = Some(schedules.always_on()?);
            } else {
                row.flow_fraction_schedule = schedules
                    .get_source_schedule(&alphas[2])
                    .map_err(SelectedScheduleError::from)?;
                let schedule = row.flow_fraction_schedule.ok_or(E::Unavailable(
                    "actual DSOA missing flow-fraction schedule severe2418",
                ))?;
                if !schedules
                    .source_check_min_max_inclusive(schedule, 0.0, 1.0, clock)
                    .map_err(SelectedScheduleError::from)?
                {
                    return Err(E::Unavailable(
                        "actual DSOA bad flow-fraction schedule severe2421",
                    ));
                }
            }
            if num_alphas > 3 && !alpha_blanks[3] {
                row.proportional_control_minimum_schedule = schedules
                    .get_source_schedule(&alphas[3])
                    .map_err(SelectedScheduleError::from)?;
                let schedule = row
                    .proportional_control_minimum_schedule
                    .ok_or(E::Unavailable(
                        "actual DSOA missing proportional minimum schedule severe2427",
                    ))?;
                if !schedules
                    .source_check_min_max_inclusive(schedule, 0.0, 1.0, clock)
                    .map_err(SelectedScheduleError::from)?
                {
                    return Err(E::Unavailable(
                        "actual DSOA bad proportional minimum schedule severe2430",
                    ));
                }
            }
        }
        if lists > 0 {
            return Err(E::Unavailable("actual positive DSOA SpaceList input2263"));
        }
        Ok(())
    }
}

/// Original helper parameters, not copied globals, derived counts or cached values.
#[allow(
    dead_code,
    reason = "The owned positive DSOA entry awaits the separate OA consumer; admission remains closed."
)]
pub(crate) struct SourceOutdoorAirFlowOptions {
    pub(crate) use_occupancy_schedule: bool,
    pub(crate) use_minimum_oa_schedule: bool,
    pub(crate) per_person_not_set: bool,
    pub(crate) maximum_volume_flow: bool,
    pub(crate) space_number: i32,
    pub(crate) calculate_iaq_methods: bool,
}

/// Temporary loans of the actual upstream owners, never a design/occupancy facade.
#[allow(
    dead_code,
    reason = "The owned positive DSOA entry awaits the separate OA consumer; admission remains closed."
)]
pub(crate) struct SourceOutdoorAirFlowOwners<'a, 'backend> {
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) gains: &'a SelectedInternalHeatGains,
    pub(crate) schedules: &'a mut SelectedScheduleManager,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}

fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

impl SelectedSizingManager {
    /// Owned simple DSOA body; old pointer-only helper and all current callers stay literal.
    #[allow(
        dead_code,
        reason = "The owned positive DSOA entry awaits the separate OA consumer; admission remains closed."
    )]
    pub(crate) fn calculate_source_design_specification_outdoor_air_owned(
        &mut self,
        dsoa_pointer: i32,
        actual_zone: i32,
        options: SourceOutdoorAirFlowOptions,
        owners: &mut SourceOutdoorAirFlowOwners<'_, '_>,
    ) -> Result<f64, FlowError> {
        if dsoa_pointer == 0 {
            return Ok(0.0);
        }
        let at = dsoa_pointer
            .checked_sub(1)
            .and_then(|number| usize::try_from(number).ok())
            .ok_or(FlowError::Unavailable("actual DSOA pointer ordinal650"))?;
        let row = self
            .oa_requirements
            .as_mut()
            .and_then(|rows| rows.get_mut(at))
            .ok_or(FlowError::Unavailable("actual DSOA pointer owner650"))?;
        if row.number_of_dsoa != 0 {
            return Err(FlowError::Unavailable("actual DSOA SpaceList child659"));
        }
        let zone_at = actual_zone
            .checked_sub(1)
            .and_then(|number| usize::try_from(number).ok())
            .ok_or(FlowError::Unavailable("actual DSOA Zone ordinal1048"))?;
        let zone = owners
            .zones
            .zones
            .as_ref()
            .and_then(|rows| rows.get(zone_at))
            .ok_or(FlowError::Unavailable("actual DSOA Zone owner1048"))?;
        if options.space_number > 0 {
            return Err(FlowError::Unavailable(
                "actual DSOA positive Space operands1055",
            ));
        }
        let floor_area = zone.floor_area;
        let volume = zone.volume;
        let design_occupants = zone.total_occupants;
        let current_occupants = *owners.gains.source_zone_current_occupants(actual_zone)?;
        let maximum_occupants = zone.max_occupants;
        if let Some(method_name) = row.flow_method.co2_prerequisite_name() {
            if row.my_environment {
                // Native1069-1102 executes this prerequisite before calcIAQMethods.
                if !owners.control.carbon_dioxide_simulation {
                    owners.backend.show_severe_error(&format!(
                        "DesignSpecification:OutdoorAir=\"{}\" valid Outdoor Air Method =\" {}\" requires CO2 simulation.",
                        row.name, method_name,
                    ))?;
                    owners.backend.show_continue_error("The choice must be Yes for the field Carbon Dioxide Concentration in ZoneAirContaminantBalance")?;
                    return match owners.backend.show_fatal_error("CalcDesignSpecificationOutdoorAir: Errors found in input. Preceding condition(s) cause termination.") {
                        Ok(never) => match never {},
                        Err(error) => Err(FlowError::Fatal(error)),
                    };
                }
                row.my_environment = false;
            }
            if !options.calculate_iaq_methods {
                return Ok(0.0);
            }
            return Err(FlowError::Unavailable(
                "actual DSOA positive CO2 demand/generation/control calculation1173",
            ));
        }
        let mut people = match row.flow_method {
            Method::PerPerson | Method::Sum | Method::Maximum => {
                if options.use_occupancy_schedule {
                    if options.maximum_volume_flow {
                        maximum_occupants * row.flow_per_person
                    } else {
                        current_occupants * row.flow_per_person
                    }
                } else {
                    design_occupants * row.flow_per_person
                }
            }
            _ => 0.0,
        };
        if options.per_person_not_set {
            people = 0.0;
        }
        let mut flow = match row.flow_method {
            Method::PerPerson => people,
            Method::PerZone => row.flow_per_zone,
            Method::PerArea => row.flow_per_area * floor_area,
            Method::AirChanges => row.flow_air_changes_per_hour * volume / 3600.0,
            Method::Sum | Method::Maximum => {
                let per_zone = row.flow_per_zone;
                let per_area = row.flow_per_area * floor_area;
                let air_changes = row.flow_air_changes_per_hour * volume / 3600.0;
                if row.flow_method == Method::Maximum {
                    // Native ObjexxFCL four-way max: paired comparisons, first on equality.
                    max(max(people, per_zone), max(per_area, air_changes))
                } else {
                    people + per_zone + per_area + air_changes
                }
            }
            Method::Invalid => 0.0, // Native default branch1376, not an invented valid enum.
            Method::IndoorAirQualityProcedure
            | Method::ProportionalOccupancySchedule
            | Method::ProportionalDesignOccupancy => {
                return Err(FlowError::Unavailable(
                    "actual unowned advanced DSOA calculation",
                ));
            }
        };
        let multiplier =
            zone.multiplier
                .checked_mul(zone.list_multiplier)
                .ok_or(FlowError::Unavailable(
                    "actual DSOA integer multiplier product1383",
                ))?;
        flow *= f64::from(multiplier);
        if let Some(schedule) = row.flow_fraction_schedule {
            if options.use_minimum_oa_schedule {
                let value = if options.maximum_volume_flow {
                    owners
                        .schedules
                        .source_max_with_clock(schedule, owners.clock)
                } else {
                    owners.schedules.source_current_schedule_value(schedule)
                }
                .map_err(|_| {
                    FlowError::Unavailable("actual DSOA flow-fraction schedule owner1390")
                })?;
                flow *= value;
            }
        }
        Ok(flow)
    }
}
