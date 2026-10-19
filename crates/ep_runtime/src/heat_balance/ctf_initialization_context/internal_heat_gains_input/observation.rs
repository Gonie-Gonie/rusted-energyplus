//! Ephemeral read-only views of actual retained InternalHeatGains owners.
//! This does not invoke getters, schedule accessors, rate targets or calculations.
use super::devices::{SourceRateField, SourceRateTarget};
use super::{SourceMiscGainObject, SourceOtherEquipment};
use crate::heat_balance::HeatBalanceState;
use crate::heat_balance::ctf_initialization_context::CtfInitializationReports;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use crate::heat_balance::source_output_registration::SourceOutputStoreType;

/// A symbolic identity of the actual stored rate target; no pointer is dereferenced.
#[derive(Clone, Copy, Debug)]
pub enum OrdinaryGainRateTarget {
    /// The genuine shared zeroPointerVal target.
    ZeroPointerValue,
    /// A genuine retained OtherEquipment row and rate-field target.
    OtherEquipment {
        /// Actual zero-based OtherEquipment owner position.
        row_index: usize,
        /// Literal Source name of the stored target field.
        field: &'static str,
    },
}

/// Borrowed descriptor of the corresponding retained Source owner.
pub struct OrdinaryGainDevice<'a> {
    /// Actual retained component name.
    pub component_name: &'a str,
    /// Actual retained component type.
    pub component_type: i32,
    /// Actual retained space gain fraction.
    pub space_gain_fraction: f64,
    /// Actual retained targets.
    pub targets: [Option<OrdinaryGainRateTarget>; 7],
    /// Actual retained cached rates.
    pub cached_rates: &'a [f64; 7],
    /// Actual retained return air node.
    pub return_air_node: i32,
}

/// Borrowed descriptor of the corresponding retained Source owner.
pub struct OrdinaryGainDeviceStorage<'a> {
    /// None is genuinely unallocated. Some(0) is allocated-empty storage.
    pub storage_size: Option<usize>,
    /// Actual retained number of devices.
    pub number_of_devices: usize,
    /// Actual retained max number of devices.
    pub max_number_of_devices: usize,
    /// None means the actual active count does not fit its actual storage.
    pub active_values: Option<Vec<OrdinaryGainDevice<'a>>>,
}

/// Borrowed descriptor of the corresponding retained Source owner.
pub struct OrdinaryOtherEquipment<'a> {
    /// Actual retained name.
    pub name: &'a str,
    /// Actual retained space index.
    pub space_index: i32,
    /// Actual retained zone ptr.
    pub zone_ptr: i32,
    /// Actual retained schedule index.
    pub schedule_index: Option<usize>,
    /// Actual retained fuel code.
    pub fuel_code: i32,
    /// Actual retained design level.
    pub design_level: f64,
    /// Actual retained nominal min design level.
    pub nominal_min_design_level: f64,
    /// Actual retained nominal max design level.
    pub nominal_max_design_level: f64,
    /// Actual retained fraction latent.
    pub fraction_latent: f64,
    /// Actual retained fraction radiant.
    pub fraction_radiant: f64,
    /// Actual retained fraction lost.
    pub fraction_lost: f64,
    /// Actual retained fraction convected.
    pub fraction_convected: f64,
    /// Actual retained co2 rate factor.
    pub co2_rate_factor: f64,
    /// Actual retained end use subcategory.
    pub end_use_subcategory: &'a str,
    /// Actual retained rates.
    pub rates: &'a [f64; 7],
    /// Actual retained energies.
    pub energies: &'a [f64; 6],
}

/// Borrowed descriptor of the corresponding retained Source owner.
pub struct OrdinaryGainReport<'a> {
    /// Actual energy/rate pairs: radiant, visible, convective, latent, total.
    pub total: &'a [f64; 10],
    /// Actual energy/rate pairs: radiant, convective, latent, lost, total.
    pub other: &'a [f64; 10],
}

/// Borrowed descriptor of the corresponding retained Source owner.
pub struct OrdinaryGainMiscObject<'a> {
    /// Actual retained name.
    pub name: &'a str,
    /// Actual retained space start ptr.
    pub space_start_ptr: i32,
    /// Actual retained num spaces.
    pub num_spaces: i32,
    /// Actual retained space nums.
    pub space_nums: &'a [i32],
    /// Actual retained names.
    pub names: &'a [String],
}

/// Borrowed descriptor of the corresponding retained Source owner.
pub struct OrdinaryGainZone<'a> {
    /// Actual retained name.
    pub name: &'a str,
    /// Actual retained total occupants.
    pub total_occupants: f64,
    /// Actual retained internal heat gains.
    pub internal_heat_gains: f64,
}

/// Borrowed descriptor of the corresponding retained Source owner.
pub struct OrdinaryGainSpace<'a> {
    /// Actual retained name.
    pub name: &'a str,
    /// Actual retained zone number.
    pub zone_number: i32,
}

/// Borrowed descriptor of the corresponding retained Source owner.
pub struct OrdinaryGainSchedule<'a> {
    /// Actual retained name.
    pub name: &'a str,
    /// Actual retained number.
    pub number: usize,
    /// Actual retained is used.
    pub is_used: bool,
    /// Actual retained min value.
    pub min_value: f64,
    /// Actual retained max value.
    pub max_value: f64,
    /// Actual retained is min max set.
    pub is_min_max_set: bool,
    /// Actual retained schedule type number.
    pub schedule_type_number: Option<usize>,
    /// Actual retained ems actuated on.
    pub ems_actuated_on: bool,
    /// Actual retained ems value.
    pub ems_value: f64,
    /// Actual retained current value.
    pub current_value: f64,
}

/// Borrowed descriptor of the corresponding retained Source owner.
pub struct OrdinaryGainSchedules<'a> {
    /// Actual retained input processed.
    pub input_processed: bool,
    /// Actual retained schedule map.
    pub schedule_map: Vec<(&'a str, usize)>,
    /// Actual retained rows.
    pub rows: Vec<OrdinaryGainSchedule<'a>>,
}

/// Borrowed descriptor of the corresponding retained Source owner.
pub struct OrdinaryGainDictionaryEntry<'a> {
    /// Actual retained storage index.
    pub storage_index: usize,
    /// Actual retained name.
    pub name: &'a str,
    /// Stored Rust enum names, not invented Native integer enum values.
    pub units: &'static str,
    /// Actual retained store type.
    pub store_type: &'static str,
    /// Actual retained integer.
    pub integer: bool,
    /// Actual retained next.
    pub next: Option<usize>,
    /// Actual retained reported on dictionary file.
    pub reported_on_dictionary_file: bool,
    /// Actual retained key out var nums.
    pub key_out_var_nums: &'a [usize],
}

/// Borrowed descriptor of the corresponding retained Source owner.
pub struct OrdinaryGainOutputRegistration<'a> {
    /// Actual retained dictionary storage size.
    pub dictionary_storage_size: usize,
    /// Actual retained dictionary.
    pub dictionary: Vec<OrdinaryGainDictionaryEntry<'a>>,
    /// Actual retained num total real variables.
    pub num_total_real_variables: i32,
    /// Actual retained num real variables.
    pub num_real_variables: i32,
    /// Actual retained num real variables setup.
    pub num_real_variables_setup: i32,
    /// Actual retained num real variables sum.
    pub num_real_variables_sum: i32,
    /// Actual retained num real variables meter.
    pub num_real_variables_meter: i32,
    /// Actual retained report number counter.
    pub report_number_counter: i32,
}

/// All views borrow the existing owners. Option collection states are preserved.
pub struct OrdinaryInternalGains<'a> {
    /// Actual retained get input flag.
    pub get_input_flag: bool,
    /// Actual retained errors found.
    pub errors_found: bool,
    /// Actual retained zero pointer value.
    pub zero_pointer_value: f64,
    /// Actual retained totals.
    pub totals: &'a [i32; 10],
    /// Actual retained zone int gain.
    pub zone_int_gain: Option<&'a [[f64; 2]]>,
    /// Actual retained space int gain.
    pub space_int_gain: Option<&'a [[f64; 2]]>,
    /// Actual retained space devices.
    pub space_devices: Option<Vec<OrdinaryGainDeviceStorage<'a>>>,
    /// Actual retained space power reduction factor.
    pub space_power_reduction_factor: Option<&'a [f64]>,
    /// Actual retained zone report.
    pub zone_report: Option<Vec<OrdinaryGainReport<'a>>>,
    /// Actual retained space report.
    pub space_report: Option<Vec<OrdinaryGainReport<'a>>>,
    /// Fixed Rust inner arrays have bounds0..25, without a dynamic allocation flag.
    pub zone_electric_end_use: Option<&'a [[[f64; 26]; 4]]>,
    /// Actual retained refrigeration credit.
    pub refrigeration_credit: Option<&'a [[f64; 4]]>,
    /// Actual retained other equipment.
    pub other_equipment: Option<Vec<OrdinaryOtherEquipment<'a>>>,
    /// Actual retained co2 generation size.
    pub co2_generation_size: Option<usize>,
    /// Actual retained lights objects.
    pub lights_objects: Option<Vec<OrdinaryGainMiscObject<'a>>>,
    /// Actual retained electric objects.
    pub electric_objects: Option<Vec<OrdinaryGainMiscObject<'a>>>,
    /// Actual retained zones.
    pub zones: Option<Vec<OrdinaryGainZone<'a>>>,
    /// Actual retained spaces.
    pub spaces: Option<Vec<OrdinaryGainSpace<'a>>>,
    /// Actual retained schedules.
    pub schedules: Option<OrdinaryGainSchedules<'a>>,
    /// Actual retained output registration.
    pub output_registration: OrdinaryGainOutputRegistration<'a>,
}

fn equipment(row: &SourceOtherEquipment) -> OrdinaryOtherEquipment<'_> {
    OrdinaryOtherEquipment {
        name: &row.name,
        space_index: row.space_index,
        zone_ptr: row.zone_ptr,
        schedule_index: row.schedule.map(|index| index.0),
        fuel_code: row.fuel as i32,
        design_level: row.design_level,
        nominal_min_design_level: row.nominal_min_design_level,
        nominal_max_design_level: row.nominal_max_design_level,
        fraction_latent: row.fraction_latent,
        fraction_radiant: row.fraction_radiant,
        fraction_lost: row.fraction_lost,
        fraction_convected: row.fraction_convected,
        co2_rate_factor: row.co2_rate_factor,
        end_use_subcategory: &row.end_use_subcategory,
        rates: &row.rates,
        energies: &row.energies,
    }
}

fn misc(row: &SourceMiscGainObject) -> OrdinaryGainMiscObject<'_> {
    OrdinaryGainMiscObject {
        name: &row.name,
        space_start_ptr: row.space_start_ptr,
        num_spaces: row.num_spaces,
        space_nums: &row.space_nums,
        names: &row.names,
    }
}

fn target(value: SourceRateTarget) -> OrdinaryGainRateTarget {
    match value {
        SourceRateTarget::ZeroPointerValue => OrdinaryGainRateTarget::ZeroPointerValue,
        SourceRateTarget::OtherEquipment { row, field } => OrdinaryGainRateTarget::OtherEquipment {
            row_index: row,
            field: match field {
                SourceRateField::Convective => "ConGainRate",
                SourceRateField::Radiant => "RadGainRate",
                SourceRateField::Latent => "LatGainRate",
            },
        },
    }
}

impl CtfInitializationReports {
    /// Borrow reached internal-gain owners on the selected route without mutation.
    /// Legacy unentered geometry returns None. This does not certify HB completion.
    ///
    /// # Errors
    /// Returns unavailable if the actual moved Zone/Space owner is missing.
    pub fn ordinary_internal_gains<'a>(
        &'a self,
        state: &'a HeatBalanceState,
    ) -> Result<Option<OrdinaryInternalGains<'a>>, HeatBalanceInputError> {
        if !self.surface_geometry_input.entered_once {
            return Ok(None);
        }
        let zones =
            state
                .selected_zone_input
                .as_ref()
                .ok_or(HeatBalanceInputError::Unavailable(
                    "same moved internal-gain Zone/Space owner",
                ))?;
        let gains = &self.selected_internal_heat_gains;
        let allocations = &gains.allocations;
        let output = &self.output_registration;
        Ok(Some(OrdinaryInternalGains {
            get_input_flag: gains.get_input_flag,
            errors_found: gains.errors_found,
            zero_pointer_value: gains.zero_pointer_value,
            totals: &gains.totals,
            zone_int_gain: allocations.zone_int_gain.as_deref(),
            space_int_gain: allocations.space_int_gain.as_deref(),
            space_devices: allocations.space_devices.as_ref().map(|rows| {
                rows.iter()
                    .map(|row| {
                        let active = if row.number_of_devices == 0 {
                            Some(&[][..])
                        } else {
                            row.devices
                                .as_ref()
                                .and_then(|v| v.get(..row.number_of_devices))
                        };
                        OrdinaryGainDeviceStorage {
                            storage_size: row.devices.as_ref().map(Vec::len),
                            number_of_devices: row.number_of_devices,
                            max_number_of_devices: row.max_number_of_devices,
                            active_values: active.map(|v| {
                                v.iter()
                                    .map(|device| OrdinaryGainDevice {
                                        component_name: &device.component_name,
                                        component_type: device.component_type,
                                        space_gain_fraction: device.space_gain_fraction,
                                        targets: device.targets.map(|value| value.map(target)),
                                        cached_rates: &device.cached_rates,
                                        return_air_node: device.return_air_node,
                                    })
                                    .collect()
                            }),
                        }
                    })
                    .collect()
            }),
            space_power_reduction_factor: allocations.space_power_reduction_factor.as_deref(),
            zone_report: allocations.zone_report.as_ref().map(|v| {
                v.iter()
                    .map(|r| OrdinaryGainReport {
                        total: &r.total,
                        other: &r.other,
                    })
                    .collect()
            }),
            space_report: allocations.space_report.as_ref().map(|v| {
                v.iter()
                    .map(|r| OrdinaryGainReport {
                        total: &r.total,
                        other: &r.other,
                    })
                    .collect()
            }),
            zone_electric_end_use: allocations.zone_electric_end_use.as_deref(),
            refrigeration_credit: allocations.refrigeration_credit.as_deref(),
            other_equipment: allocations
                .other_equipment
                .as_ref()
                .map(|v| v.iter().map(equipment).collect()),
            co2_generation_size: allocations.co2_generation.as_ref().map(Vec::len),
            lights_objects: gains
                .lights_objects
                .as_ref()
                .map(|v| v.iter().map(misc).collect()),
            electric_objects: gains
                .electric_objects
                .as_ref()
                .map(|v| v.iter().map(misc).collect()),
            zones: zones.zones.as_ref().map(|v| {
                v.iter()
                    .map(|r| OrdinaryGainZone {
                        name: &r.name,
                        total_occupants: r.total_occupants,
                        internal_heat_gains: r.internal_heat_gains,
                    })
                    .collect()
            }),
            spaces: zones.spaces.as_ref().map(|v| {
                v.iter()
                    .map(|r| OrdinaryGainSpace {
                        name: &r.name,
                        zone_number: r.zone_num,
                    })
                    .collect()
            }),
            schedules: self.selected_schedule_manager.as_ref().map(|manager| {
                OrdinaryGainSchedules {
                    input_processed: manager.schedule_input_processed,
                    schedule_map: manager
                        .schedule_map
                        .iter()
                        .map(|(name, index)| (name.as_str(), index.0))
                        .collect(),
                    rows: manager
                        .schedules
                        .iter()
                        .map(|r| OrdinaryGainSchedule {
                            name: &r.name,
                            number: r.number.0,
                            is_used: r.is_used,
                            min_value: r.min_value,
                            max_value: r.max_value,
                            is_min_max_set: r.is_min_max_set,
                            schedule_type_number: r.schedule_type_number,
                            ems_actuated_on: r.ems_actuated_on,
                            ems_value: r.ems_value,
                            current_value: r.current_value,
                        })
                        .collect(),
                }
            }),
            output_registration: OrdinaryGainOutputRegistration {
                dictionary_storage_size: output.dictionary_variables.len(),
                dictionary: output
                    .dictionary_variables
                    .iter()
                    .enumerate()
                    .map(|(index, row)| OrdinaryGainDictionaryEntry {
                        storage_index: index,
                        name: &row.name,
                        units: row.units.symbol(),
                        store_type: match row.store_type {
                            SourceOutputStoreType::Average => "Average",
                            SourceOutputStoreType::Sum => "Sum",
                        },
                        integer: row.integer,
                        next: row.next,
                        reported_on_dictionary_file: row.reported_on_dictionary_file,
                        key_out_var_nums: &row.key_out_var_nums,
                    })
                    .collect(),
                num_total_real_variables: output.num_total_real_variables,
                num_real_variables: output.num_real_variables,
                num_real_variables_setup: output.num_real_variables_setup,
                num_real_variables_sum: output.num_real_variables_sum,
                num_real_variables_meter: output.num_real_variables_meter,
                report_number_counter: output.report_number_counter,
            },
        }))
    }
}

#[cfg(all(test, feature = "native-controlled-air-cohort"))]
impl super::SelectedInternalHeatGains {
    /// Passive views of the actual retained symbolic pointer targets. No rate
    /// getter/calculation is invoked and no owner is copied into a new cache.
    pub(crate) fn native_gap_device_bindings(
        &self,
    ) -> Result<Vec<Vec<[Option<OrdinaryGainRateTarget>; 7]>>, HeatBalanceInputError> {
        self.allocations
            .space_devices
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|owner| {
                if owner.number_of_devices == 0 {
                    return Ok(Vec::new());
                }
                let rows = owner
                    .devices
                    .as_deref()
                    .ok_or(HeatBalanceInputError::Unavailable(
                        "actual device target storage",
                    ))?;
                if rows.len() < owner.number_of_devices {
                    return Err(HeatBalanceInputError::Unavailable(
                        "actual device target active bounds",
                    ));
                }
                Ok(rows[..owner.number_of_devices]
                    .iter()
                    .map(|row| row.targets.map(|v| v.map(target)))
                    .collect())
            })
            .collect()
    }
}
