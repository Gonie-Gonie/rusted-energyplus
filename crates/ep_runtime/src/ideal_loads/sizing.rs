//! Bounded direct-hard-sized `SizePurchasedAir` legacy route.

use ep_model::{AutosizeOrNumber, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsLimit};

/// EnergyPlus small-load threshold applied to the local heating design value.
pub const PURCHASED_AIR_SMALL_LOAD_W: f64 = 1.0;

/// Dynamic source state required by the bounded legacy route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PurchasedAirHardSizeLegacyContext {
    /// One-based `CurZoneEqNum`; zero is outside the admitted child route.
    pub current_zone_equipment_index: usize,
    /// Whether a Zone sizing run has completed.
    pub zone_sizing_run_done: bool,
}

/// Four direct PurchasedAir sizing fields in source execution order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirHardSizeField {
    /// `MaxHeatVolFlowRate`.
    MaximumHeatingAirFlowRate,
    /// `MaxHeatSensCap`.
    MaximumSensibleHeatingCapacity,
    /// `MaxCoolVolFlowRate`.
    MaximumCoolingAirFlowRate,
    /// `MaxCoolTotCap`.
    MaximumTotalCoolingCapacity,
}

impl PurchasedAirHardSizeField {
    /// Stable typed-model field name.
    #[must_use]
    pub const fn model_field_name(self) -> &'static str {
        match self {
            Self::MaximumHeatingAirFlowRate => "maximum_heating_air_flow_rate_m3_per_s",
            Self::MaximumSensibleHeatingCapacity => "maximum_sensible_heating_capacity_w",
            Self::MaximumCoolingAirFlowRate => "maximum_cooling_air_flow_rate_m3_per_s",
            Self::MaximumTotalCoolingCapacity => "maximum_total_cooling_capacity_w",
        }
    }
}

/// Runtime-owned numeric values retained after the bounded sizing child.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirSizedLimits {
    /// Retained maximum heating volume flow, including explicit zero.
    pub maximum_heating_air_flow_rate_m3_per_s: Option<AutosizeOrNumber>,
    /// Retained maximum sensible heating capacity, including explicit zero.
    pub maximum_sensible_heating_capacity_w: Option<AutosizeOrNumber>,
    /// Retained maximum cooling volume flow, including explicit zero.
    pub maximum_cooling_air_flow_rate_m3_per_s: Option<AutosizeOrNumber>,
    /// Retained maximum total cooling capacity, including explicit zero.
    pub maximum_total_cooling_capacity_w: Option<AutosizeOrNumber>,
}

impl PurchasedAirSizedLimits {
    /// Seeds the four-field runtime overlay without cloning the full model.
    #[must_use]
    pub const fn from_system(system: &IdealLoadsAirSystem) -> Self {
        Self {
            maximum_heating_air_flow_rate_m3_per_s: system.maximum_heating_air_flow_rate_m3_per_s,
            maximum_sensible_heating_capacity_w: system.maximum_sensible_heating_capacity_w,
            maximum_cooling_air_flow_rate_m3_per_s: system.maximum_cooling_air_flow_rate_m3_per_s,
            maximum_total_cooling_capacity_w: system.maximum_total_cooling_capacity_w,
        }
    }
}

/// Source path completed by the bounded sizing call.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirHardSizeLegacyRoute {
    /// `CurZoneEqNum <= 0` suppressed the field body and returned normally.
    NoCurrentZoneEquipment,
    /// `HVACSizingIndex == 0`, no Zone sizing run, direct hard-size route.
    DirectHardSizedNoSizingRun,
}

/// One source-ordered direct-field visit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirHardSizeFieldOutcome {
    /// Field visited at this source position.
    pub field: PurchasedAirHardSizeField,
    /// Numeric input, or `None` for a blank field.
    pub input_value: Option<f64>,
    /// Whether the positive-value child sizer was invoked.
    pub child_sizer_called: bool,
    /// Clean-scratch bounded child result.
    pub child_result: Option<f64>,
    /// Whether the child result was assigned back to the PurchasedAir object.
    pub object_writeback: bool,
    /// Local design value left after this direct hard-size branch.
    pub local_design_value: f64,
    /// User-specified report records emitted inside the source child sizer.
    pub child_user_report_records: usize,
    /// Design/user report records emitted by the source outer routine.
    pub outer_report_records: usize,
    /// Unit suffix passed to the child sizer by the source routine.
    pub child_sizing_label_unit: &'static str,
}

/// Persistent result of the admitted direct hard-sized legacy route.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirHardSizeLegacyOutcome {
    /// Source path taken by this call.
    pub route: PurchasedAirHardSizeLegacyRoute,
    /// Runtime-owned values consumed by environment initialization and Calc.
    pub sized_limits: PurchasedAirSizedLimits,
    /// Field visits retained in exact source order; all are absent on suppression.
    pub fields: [Option<PurchasedAirHardSizeFieldOutcome>; 4],
    /// Entry clears both shared Zone fan-mode flags.
    pub entry_fan_flags_cleared: bool,
}

impl PurchasedAirHardSizeLegacyOutcome {
    /// Number of child sizers called by this route.
    #[must_use]
    pub fn child_sizer_call_count(self) -> usize {
        self.fields
            .iter()
            .flatten()
            .filter(|field| field.child_sizer_called)
            .count()
    }

    /// Source report-record count characterized by this bounded clean route.
    #[must_use]
    pub fn characterized_report_record_count(self) -> usize {
        self.fields
            .iter()
            .flatten()
            .map(|field| field.child_user_report_records + field.outer_report_records)
            .sum()
    }
}

/// Fail-closed boundary errors for the bounded direct legacy route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirHardSizeLegacyError {
    /// The custom `HVACSizingIndex > 0` branch is still unported.
    CustomZoneHvacSizingNotImplemented {
        /// Selected system.
        system: IdealLoadsAirSystemId,
    },
    /// The `ZoneSizingRunDone`/`FinalZoneSizing` branch is still unported.
    ZoneSizingRunNotImplemented {
        /// Selected system.
        system: IdealLoadsAirSystemId,
    },
    /// Autosizing remains outside this direct hard-size slice.
    AutosizingNotImplemented {
        /// Selected system.
        system: IdealLoadsAirSystemId,
        /// Autosized source field.
        field: PurchasedAirHardSizeField,
    },
    /// A supplied hard size was negative, NaN, or infinite.
    InvalidHardSize {
        /// Selected system.
        system: IdealLoadsAirSystemId,
        /// Invalid source field.
        field: PurchasedAirHardSizeField,
    },
    /// An active limit omitted its required direct numeric value.
    MissingRequiredHardSize {
        /// Selected system.
        system: IdealLoadsAirSystemId,
        /// Missing source field.
        field: PurchasedAirHardSizeField,
    },
}

/// Executes the clean-scratch, direct hard-sized `HVACSizingIndex == 0` route.
///
/// The four child sizers reduce to numeric identity functions for this bounded
/// path. The result retains their source-order call, writeback, local-design,
/// and report asymmetries without claiming the underlying autosizing engines.
pub fn size_purchased_air_direct_hard_sized_legacy_route(
    system: &IdealLoadsAirSystem,
    sized_limits: &mut PurchasedAirSizedLimits,
    context: PurchasedAirHardSizeLegacyContext,
) -> Result<PurchasedAirHardSizeLegacyOutcome, PurchasedAirHardSizeLegacyError> {
    if context.current_zone_equipment_index == 0 {
        return Ok(PurchasedAirHardSizeLegacyOutcome {
            route: PurchasedAirHardSizeLegacyRoute::NoCurrentZoneEquipment,
            sized_limits: *sized_limits,
            fields: [None; 4],
            entry_fan_flags_cleared: true,
        });
    }
    if system
        .design_specification_zonehvac_sizing_object_name
        .is_some()
    {
        return Err(
            PurchasedAirHardSizeLegacyError::CustomZoneHvacSizingNotImplemented {
                system: system.id,
            },
        );
    }
    if context.zone_sizing_run_done {
        return Err(
            PurchasedAirHardSizeLegacyError::ZoneSizingRunNotImplemented { system: system.id },
        );
    }

    let heating_flow = resolve_hard_size(
        system.id,
        PurchasedAirHardSizeField::MaximumHeatingAirFlowRate,
        sized_limits.maximum_heating_air_flow_rate_m3_per_s,
        limit_includes_flow(system.heating_limit),
    )?;
    let heating_capacity = resolve_hard_size(
        system.id,
        PurchasedAirHardSizeField::MaximumSensibleHeatingCapacity,
        sized_limits.maximum_sensible_heating_capacity_w,
        limit_includes_capacity(system.heating_limit),
    )?;
    let cooling_flow = resolve_hard_size(
        system.id,
        PurchasedAirHardSizeField::MaximumCoolingAirFlowRate,
        sized_limits.maximum_cooling_air_flow_rate_m3_per_s,
        limit_includes_flow(system.cooling_limit),
    )?;
    let cooling_capacity = resolve_hard_size(
        system.id,
        PurchasedAirHardSizeField::MaximumTotalCoolingCapacity,
        sized_limits.maximum_total_cooling_capacity_w,
        limit_includes_capacity(system.cooling_limit),
    )?;

    let fields = [
        field_outcome(
            PurchasedAirHardSizeField::MaximumHeatingAirFlowRate,
            heating_flow,
            true,
            0.0,
            0,
            "m3/s",
        ),
        field_outcome(
            PurchasedAirHardSizeField::MaximumSensibleHeatingCapacity,
            heating_capacity,
            false,
            heating_capacity
                .filter(|value| *value >= PURCHASED_AIR_SMALL_LOAD_W)
                .unwrap_or(0.0),
            usize::from(heating_capacity.is_some_and(|value| value >= PURCHASED_AIR_SMALL_LOAD_W))
                * 2,
            "m3/s",
        ),
        field_outcome(
            PurchasedAirHardSizeField::MaximumCoolingAirFlowRate,
            cooling_flow,
            true,
            0.0,
            0,
            "m3/s",
        ),
        field_outcome(
            PurchasedAirHardSizeField::MaximumTotalCoolingCapacity,
            cooling_capacity,
            true,
            0.0,
            0,
            "m3/s",
        ),
    ];
    if let Some(result) = fields[0].child_result {
        sized_limits.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(result));
    }
    if let Some(result) = fields[2].child_result {
        sized_limits.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(result));
    }
    if let Some(result) = fields[3].child_result {
        sized_limits.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(result));
    }

    Ok(PurchasedAirHardSizeLegacyOutcome {
        route: PurchasedAirHardSizeLegacyRoute::DirectHardSizedNoSizingRun,
        sized_limits: PurchasedAirSizedLimits {
            maximum_heating_air_flow_rate_m3_per_s: sized_limits
                .maximum_heating_air_flow_rate_m3_per_s,
            maximum_sensible_heating_capacity_w: sized_limits.maximum_sensible_heating_capacity_w,
            maximum_cooling_air_flow_rate_m3_per_s: sized_limits
                .maximum_cooling_air_flow_rate_m3_per_s,
            maximum_total_cooling_capacity_w: sized_limits.maximum_total_cooling_capacity_w,
        },
        fields: [
            Some(fields[0]),
            Some(fields[1]),
            Some(fields[2]),
            Some(fields[3]),
        ],
        entry_fan_flags_cleared: true,
    })
}

fn resolve_hard_size(
    system: IdealLoadsAirSystemId,
    field: PurchasedAirHardSizeField,
    value: Option<AutosizeOrNumber>,
    required: bool,
) -> Result<Option<f64>, PurchasedAirHardSizeLegacyError> {
    match value {
        Some(AutosizeOrNumber::Value(value)) if value.is_finite() && value >= 0.0 => {
            Ok(Some(value))
        }
        Some(AutosizeOrNumber::Autosize) => {
            Err(PurchasedAirHardSizeLegacyError::AutosizingNotImplemented { system, field })
        }
        Some(AutosizeOrNumber::Value(_)) => {
            Err(PurchasedAirHardSizeLegacyError::InvalidHardSize { system, field })
        }
        None if required => {
            Err(PurchasedAirHardSizeLegacyError::MissingRequiredHardSize { system, field })
        }
        None => Ok(None),
    }
}

fn field_outcome(
    field: PurchasedAirHardSizeField,
    input_value: Option<f64>,
    writes_back: bool,
    local_design_value: f64,
    outer_report_records: usize,
    child_sizing_label_unit: &'static str,
) -> PurchasedAirHardSizeFieldOutcome {
    let child_sizer_called = input_value.is_some_and(|value| value > 0.0);
    PurchasedAirHardSizeFieldOutcome {
        field,
        input_value,
        child_sizer_called,
        child_result: child_sizer_called.then_some(input_value.unwrap_or(0.0)),
        object_writeback: child_sizer_called && writes_back,
        local_design_value,
        child_user_report_records: usize::from(child_sizer_called),
        outer_report_records: if child_sizer_called {
            outer_report_records
        } else {
            0
        },
        child_sizing_label_unit,
    }
}

fn limit_includes_flow(limit: IdealLoadsLimit) -> bool {
    matches!(
        limit,
        IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
    )
}

fn limit_includes_capacity(limit: IdealLoadsLimit) -> bool {
    matches!(
        limit,
        IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
    )
}
