//! Genuine keyed OutVarReal registration and live Which-target resolution.
use super::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable, SourceOutputDictionaryHost,
    SourceOutputFileHost, SourceOutputPreScan, SourceOutputRegistration, SourceOutputStoreType,
    SourceOutputUnits, SourceRealOutputDeclaration, SourceReportFrequency,
    SourceReportVariableInput, TimeStepKind,
};
use crate::{
    RuntimeOutputDefinition, RuntimeOutputFrequency, RuntimeOutputRegistry, RuntimeOutputRequest,
};

/// Source fields retained by the genuine surface heat-balance owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceSurfaceOutputField {
    SurfTempIn,
    SurfTempOut,
    SurfOpaqInsFaceCond,
    SurfOpaqInsFaceCondFlux,
    SurfOpaqOutFaceCond,
    SurfOpaqOutFaceCondFlux,
    SurfOpaqStorageCond,
    SurfOpaqStorageCondFlux,
}

/// Stable actual field identity; Source ordinals remain independent of typed IDs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceRealOutputTarget {
    Node {
        field: super::SourceNodeOutputField,
        index: i32,
    },
    Weather {
        field: super::SourceWeatherRealOutputField,
    },
    Schedule {
        index: crate::heat_balance::selected_schedule_manager::SourceScheduleIndex,
    },
    SystemReports {
        field: super::SourceSystemReportsOutputField,
    },
    ExteriorLight {
        field: super::SourceExteriorLightOutputField,
        index: i32,
    },
    Facility {
        field: super::SourceFacilityOutputField,
    },
    Pollution {
        field: super::SourceCarbonEquivalentField,
    },
    Oscillation {
        field: super::SourceOscillationOutputField,
        index: i32,
    },
    Hvac {
        field: super::SourceHvacRealOutputField,
    },
    SurfaceHeatBalance {
        field: SourceSurfaceOutputField,
        index: i32,
    },
    Air {
        field: super::SourceAirOutputField,
        index: i32,
        space: bool,
    },
    Comfort {
        field: super::SourceComfortOutputField,
        index: i32,
    },
    Solar {
        field: super::SourceSolarOutputField,
        index: i32,
    },
}

/// Implemented by the caller over its sole live moved State/context owners.
pub(crate) trait SourceRealOutputOwners {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError>;
}

/// Literal Source VariableType value used by OutVarReal's constructor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceOutputVariableType {
    Integer = 0,
    Real = 1,
}

/// Evaluates ActualVariable's real reference before the registration call.
/// Only the semantic target survives registration, never this scalar's value.
pub(crate) struct SourceRealOutputBinding<'a> {
    pub(super) target: SourceRealOutputTarget,
    _actual: &'a f64,
}
impl<'a> SourceRealOutputBinding<'a> {
    pub(crate) fn new(
        target: SourceRealOutputTarget,
        owners: &'a impl SourceRealOutputOwners,
    ) -> Result<Self, OrdinaryRadiationError> {
        Ok(Self {
            target,
            _actual: owners.real_output(target)?,
        })
    }
}

/// Actual constructor members of OutputProcessor::OutVarReal/OutVar.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceOutVarReal {
    pub(crate) dictionary_number: Option<usize>,
    pub(crate) variable_type: SourceOutputVariableType,
    pub(crate) time_step_type: TimeStepKind,
    pub(crate) store_type: SourceOutputStoreType,
    pub(crate) value: f64,
    pub(crate) time_step_value: f64,
    pub(crate) external_interface_time_step_value: f64,
    pub(crate) store_value: f64,
    pub(crate) num_stored: f64,
    pub(crate) stored: bool,
    pub(crate) report: bool,
    pub(crate) time_step_stored: bool,
    pub(crate) this_time_step_stored: bool,
    pub(crate) this_time_step_count: i32,
    pub(crate) frequency: SourceReportFrequency,
    pub(crate) max_value: f64,
    pub(crate) min_value: f64,
    pub(crate) max_value_date: i32,
    pub(crate) min_value_date: i32,
    pub(crate) report_id: i32,
    pub(crate) schedule: Option<super::super::selected_schedule_manager::SourceScheduleIndex>,
    pub(crate) zone_multiplier: i32,
    pub(crate) zone_list_multiplier: i32,
    pub(crate) key_colon_name: String,
    pub(crate) key_colon_name_upper: String,
    pub(crate) name: String,
    pub(crate) name_upper: String,
    pub(crate) key: String,
    pub(crate) key_upper: String,
    pub(crate) units: SourceOutputUnits,
    pub(crate) custom_unit_name: String,
    pub(crate) index_group: String,
    pub(crate) index_group_key: i32,
    pub(crate) meter_numbers: Vec<i32>,
    pub(crate) which: Option<SourceRealOutputTarget>,
    pub(crate) integer_which: Option<super::SourceIntegerOutputTarget>,
}
impl Default for SourceOutVarReal {
    fn default() -> Self {
        Self {
            dictionary_number: None,
            variable_type: SourceOutputVariableType::Real,
            time_step_type: TimeStepKind::Zone,
            store_type: SourceOutputStoreType::Average,
            value: 0.0,
            time_step_value: 0.0,
            external_interface_time_step_value: 0.0,
            store_value: 0.0,
            num_stored: 0.0,
            stored: false,
            report: false,
            time_step_stored: false,
            this_time_step_stored: false,
            this_time_step_count: 0,
            frequency: SourceReportFrequency::Hour,
            max_value: -9999.0,
            min_value: 9999.0,
            max_value_date: 0,
            min_value_date: 0,
            report_id: 0,
            schedule: None,
            zone_multiplier: 1,
            zone_list_multiplier: 1,
            key_colon_name: String::new(),
            key_colon_name_upper: String::new(),
            name: String::new(),
            name_upper: String::new(),
            key: String::new(),
            key_upper: String::new(),
            units: SourceOutputUnits::Invalid,
            custom_unit_name: String::new(),
            index_group: String::new(),
            index_group_key: -1,
            meter_numbers: Vec::new(),
            which: None,
            integer_which: None,
        }
    }
}

/// A read borrows the real owner on this call, without a numeric registry copy.
pub(crate) struct SourceRequestedRealValue<'a> {
    pub(crate) out_var_number: usize,
    pub(crate) report_id: i32,
    pub(crate) request: RuntimeOutputRequest,
    pub(crate) units: &'static str,
    pub(crate) target: SourceRealOutputTarget,
    pub(crate) value: &'a f64,
}
impl SourceRequestedRealValue<'_> {
    /// This metadata join does not certify Source aggregation/reporting.
    /// Frequencies must match exactly; TimeStep is never relabeled Hourly.
    pub(crate) fn runtime_definition<'a>(
        &self,
        registry: &'a RuntimeOutputRegistry,
    ) -> Result<&'a RuntimeOutputDefinition, OrdinaryRadiationError> {
        registry
            .find_output(&self.request)
            .filter(|definition| definition.units == self.units)
            .ok_or_else(|| OrdinaryRadiationUnavailable::InputBranch.into())
    }
}

pub(super) fn runtime_frequency(
    frequency: SourceReportFrequency,
) -> Result<RuntimeOutputFrequency, OrdinaryRadiationError> {
    Ok(match frequency {
        SourceReportFrequency::EachCall => {
            // Existing registry has no Detailed/EachCall identity.
            return Err(OrdinaryRadiationUnavailable::InputBranch.into());
        }
        SourceReportFrequency::TimeStep => RuntimeOutputFrequency::Timestep,
        SourceReportFrequency::Hour => RuntimeOutputFrequency::Hourly,
        SourceReportFrequency::Day => RuntimeOutputFrequency::Daily,
        SourceReportFrequency::Month => RuntimeOutputFrequency::Monthly,
        SourceReportFrequency::Simulation => RuntimeOutputFrequency::RunPeriod,
        SourceReportFrequency::Year => RuntimeOutputFrequency::Annual,
    })
}

impl SourceOutputRegistration {
    /// Source real overload, default frequency Hour and no meter arguments.
    pub(crate) fn setup_bound_unmetered_zone_real(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        prescan: &SourceOutputPreScan,
        declaration: SourceRealOutputDeclaration<'_>,
        minutes_in_time_step: i32,
        host: &mut impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
        binding: SourceRealOutputBinding<'_>,
        dictionary_host: &mut impl SourceOutputDictionaryHost,
    ) -> Result<(), OrdinaryRadiationError> {
        let selected = self.prepare_real_registration_details(
            input,
            prescan,
            declaration,
            minutes_in_time_step,
            host,
            eio,
        )?;
        self.finish_bound_real_registration(
            selected,
            declaration,
            super::SourceRealRegistrationOptions::unmetered(TimeStepKind::Zone),
            dictionary_host,
            binding,
            false,
        )
    }

    /// Actual retained Which target, resolved again after any owner update/move.
    pub(crate) fn read_registered_real<'a>(
        &self,
        out_var_number: usize,
        owners: &'a impl SourceRealOutputOwners,
    ) -> Result<&'a f64, OrdinaryRadiationError> {
        let target = self
            .out_vars
            .get(out_var_number)
            .and_then(|var| var.which)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        owners.real_output(target)
    }

    /// Immediate requested-value projection, before the unported report tail.
    /// This never sets Stored/TSValue or simulates UpdateDataandReport completion.
    pub(crate) fn requested_real_values<'a>(
        &self,
        owners: &'a impl SourceRealOutputOwners,
    ) -> Result<Vec<SourceRequestedRealValue<'a>>, OrdinaryRadiationError> {
        self.out_vars
            .iter()
            .enumerate()
            .filter(|(_, var)| var.report && var.variable_type == SourceOutputVariableType::Real)
            .map(|(number, var)| {
                if var.schedule.is_some() {
                    return Err(OrdinaryRadiationUnavailable::InputBranch.into());
                }
                Ok(SourceRequestedRealValue {
                    out_var_number: number,
                    report_id: var.report_id,
                    request: RuntimeOutputRequest {
                        key: var.key.clone(),
                        variable_name: var.name.clone(),
                        frequency: runtime_frequency(var.frequency)?,
                    },
                    units: var.units.symbol(),
                    target: var
                        .which
                        .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?,
                    value: self.read_registered_real(number, owners)?,
                })
            })
            .collect()
    }
}
