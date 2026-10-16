//! Actual selected ManageInternalHeatGains input and timestep owners.
//! Genuine timestep producers retain explicit unported provider boundaries.
use super::{CtfInitializationContext, hb_input_error};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as E;
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;
use ep_model::TypedModel;

mod allocations;
mod comfort_people;
pub(crate) use comfort_people::SourcePeopleArray;
mod devices;
mod getter;
mod nominal;
pub(super) mod observation;
mod output;
mod timestep;
pub(crate) use timestep::{
    SourceGainLatentOwners, SourceGainModule, SourceGainModuleProvider, SourceGainRadiationOwners,
    SourceGainStepOwners, SourceZoneGenericGainAllocation,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceOtherFuel {
    #[default]
    Invalid = -1,
    None = 14,
}

/// One genuine ZoneEquipData row; rates/energies retain their constructor zero.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceOtherEquipment {
    pub(crate) name: String,
    pub(crate) space_index: i32,
    pub(crate) zone_ptr: i32,
    pub(crate) schedule: Option<SourceScheduleIndex>,
    pub(crate) fuel: SourceOtherFuel,
    pub(crate) design_level: f64,
    /// Actual ZoneEquipData constructorfalse/zero; written only by genuine EMS.
    pub(crate) ems_zone_equipment_override_on: bool,
    pub(crate) ems_equipment_power: f64,
    pub(crate) nominal_min_design_level: f64,
    pub(crate) nominal_max_design_level: f64,
    pub(crate) fraction_latent: f64,
    pub(crate) fraction_radiant: f64,
    pub(crate) fraction_lost: f64,
    pub(crate) fraction_convected: f64,
    pub(crate) co2_rate_factor: f64,
    pub(crate) end_use_subcategory: String,
    // Literal ZoneEquipData fields: Power, RadGainRate, ConGainRate,
    // LatGainRate, LostRate, TotGainRate, CO2GainRate.
    pub(crate) rates: [f64; 7],
    // Consumption, RadGainEnergy, ConGainEnergy, LatGainEnergy,
    // LostEnergy, TotGainEnergy; constructorzero, then the genuine report writer.
    pub(crate) energies: [f64; 6],
}

/// Selected fields of the same allocated ZoneReportVars rows, not cached gains.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceGainReport {
    // Each source energy precedes its rate: Radiant, Visible, Convective,
    // Latent, Total. All ten genuine constructor-initialized fields are zero.
    pub(crate) total: [f64; 10],
    // Other equipment: Radiant, Convective, Latent, Lost, Total.
    pub(crate) other: [f64; 10],
    // Reached ZoneReportVars fields omitted by the input-only observation.
    pub(crate) people_rad_gain: f64,         // PeopleRadGain
    pub(crate) people_con_gain: f64,         // PeopleConGain
    pub(crate) people_sen_gain: f64,         // PeopleSenGain
    pub(crate) people_num_occ: f64,          // PeopleNumOcc
    pub(crate) people_lat_gain: f64,         // PeopleLatGain
    pub(crate) people_tot_gain: f64,         // PeopleTotGain
    pub(crate) people_rad_gain_rate: f64,    // PeopleRadGainRate
    pub(crate) people_con_gain_rate: f64,    // PeopleConGainRate
    pub(crate) people_sen_gain_rate: f64,    // PeopleSenGainRate
    pub(crate) people_lat_gain_rate: f64,    // PeopleLatGainRate
    pub(crate) people_tot_gain_rate: f64,    // PeopleTotGainRate
    pub(crate) lts_power: f64,               // LtsPower
    pub(crate) lts_elec_consump: f64,        // LtsElecConsump
    pub(crate) lts_rad_gain: f64,            // LtsRadGain
    pub(crate) lts_vis_gain: f64,            // LtsVisGain
    pub(crate) lts_con_gain: f64,            // LtsConGain
    pub(crate) lts_ret_air_gain: f64,        // LtsRetAirGain
    pub(crate) lts_tot_gain: f64,            // LtsTotGain
    pub(crate) lts_rad_gain_rate: f64,       // LtsRadGainRate
    pub(crate) lts_vis_gain_rate: f64,       // LtsVisGainRate
    pub(crate) lts_con_gain_rate: f64,       // LtsConGainRate
    pub(crate) lts_ret_air_gain_rate: f64,   // LtsRetAirGainRate
    pub(crate) lts_tot_gain_rate: f64,       // LtsTotGainRate
    pub(crate) base_heat_power: f64,         // BaseHeatPower
    pub(crate) base_heat_elec_cons: f64,     // BaseHeatElecCons
    pub(crate) base_heat_rad_gain: f64,      // BaseHeatRadGain
    pub(crate) base_heat_con_gain: f64,      // BaseHeatConGain
    pub(crate) base_heat_tot_gain: f64,      // BaseHeatTotGain
    pub(crate) base_heat_rad_gain_rate: f64, // BaseHeatRadGainRate
    pub(crate) base_heat_con_gain_rate: f64, // BaseHeatConGainRate
    pub(crate) base_heat_tot_gain_rate: f64, // BaseHeatTotGainRate
    pub(crate) elec_power: f64,              // ElecPower
    pub(crate) elec_consump: f64,            // ElecConsump
    pub(crate) elec_rad_gain: f64,           // ElecRadGain
    pub(crate) elec_con_gain: f64,           // ElecConGain
    pub(crate) elec_lat_gain: f64,           // ElecLatGain
    pub(crate) elec_lost: f64,               // ElecLost
    pub(crate) elec_tot_gain: f64,           // ElecTotGain
    pub(crate) elec_rad_gain_rate: f64,      // ElecRadGainRate
    pub(crate) elec_con_gain_rate: f64,      // ElecConGainRate
    pub(crate) elec_lat_gain_rate: f64,      // ElecLatGainRate
    pub(crate) elec_lost_rate: f64,          // ElecLostRate
    pub(crate) elec_tot_gain_rate: f64,      // ElecTotGainRate
    pub(crate) gas_power: f64,               // GasPower
    pub(crate) gas_consump: f64,             // GasConsump
    pub(crate) gas_rad_gain: f64,            // GasRadGain
    pub(crate) gas_con_gain: f64,            // GasConGain
    pub(crate) gas_lat_gain: f64,            // GasLatGain
    pub(crate) gas_lost: f64,                // GasLost
    pub(crate) gas_tot_gain: f64,            // GasTotGain
    pub(crate) gas_rad_gain_rate: f64,       // GasRadGainRate
    pub(crate) gas_con_gain_rate: f64,       // GasConGainRate
    pub(crate) gas_lat_gain_rate: f64,       // GasLatGainRate
    pub(crate) gas_lost_rate: f64,           // GasLostRate
    pub(crate) gas_tot_gain_rate: f64,       // GasTotGainRate
    pub(crate) hw_power: f64,                // HWPower
    pub(crate) hw_consump: f64,              // HWConsump
    pub(crate) hw_rad_gain: f64,             // HWRadGain
    pub(crate) hw_con_gain: f64,             // HWConGain
    pub(crate) hw_lat_gain: f64,             // HWLatGain
    pub(crate) hw_lost: f64,                 // HWLost
    pub(crate) hw_tot_gain: f64,             // HWTotGain
    pub(crate) hw_rad_gain_rate: f64,        // HWRadGainRate
    pub(crate) hw_con_gain_rate: f64,        // HWConGainRate
    pub(crate) hw_lat_gain_rate: f64,        // HWLatGainRate
    pub(crate) hw_lost_rate: f64,            // HWLostRate
    pub(crate) hw_tot_gain_rate: f64,        // HWTotGainRate
    pub(crate) steam_power: f64,             // SteamPower
    pub(crate) steam_consump: f64,           // SteamConsump
    pub(crate) steam_rad_gain: f64,          // SteamRadGain
    pub(crate) steam_con_gain: f64,          // SteamConGain
    pub(crate) steam_lat_gain: f64,          // SteamLatGain
    pub(crate) steam_lost: f64,              // SteamLost
    pub(crate) steam_tot_gain: f64,          // SteamTotGain
    pub(crate) steam_rad_gain_rate: f64,     // SteamRadGainRate
    pub(crate) steam_con_gain_rate: f64,     // SteamConGainRate
    pub(crate) steam_lat_gain_rate: f64,     // SteamLatGainRate
    pub(crate) steam_lost_rate: f64,         // SteamLostRate
    pub(crate) steam_tot_gain_rate: f64,     // SteamTotGainRate
    pub(crate) co2_rate: f64,                // CO2Rate
    pub(crate) gc_rate: f64,                 // GCRate
    // Source eFuel::Num15 storage, reset in Source order before timestep reads.
    pub(crate) other_power: [f64; 15],
    pub(crate) other_consumption: [f64; 15],
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceMiscGainObject {
    pub(crate) name: String,
    pub(crate) space_start_ptr: i32,
    pub(crate) num_spaces: i32,
    pub(crate) space_nums: Vec<i32>,
    pub(crate) names: Vec<String>,
}

/// Actual reached collection allocations; None differs from allocated length0.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceGainAllocations {
    /// Native DataHeatBalance.People EPVector constructor empty; positive allocation is guarded IHG361.
    pub(crate) people: SourcePeopleArray,
    // SpaceZoneSimData ctor NOFOCC/QLTSW.
    pub(crate) zone_int_gain: Option<Vec<[f64; 2]>>,
    pub(crate) space_int_gain: Option<Vec<[f64; 2]>>,
    pub(crate) space_devices: Option<Vec<devices::SourceSpaceGainDevices>>,
    pub(crate) space_power_reduction_factor: Option<Vec<f64>>,
    pub(crate) zone_report: Option<Vec<SourceGainReport>>,
    pub(crate) space_report: Option<Vec<SourceGainReport>>,
    // ZoneCatEUseData: EEConvected/EERadiated/EELost/EELatent, indices0..25.
    pub(crate) zone_electric_end_use: Option<Vec<[[f64; 26]; 4]>>,
    // RefrigCaseCreditData four genuine ctor-zero credit fields.
    pub(crate) refrigeration_credit: Option<Vec<[f64; 4]>>,
    pub(crate) other_equipment: Option<Vec<SourceOtherEquipment>>,
    pub(crate) co2_generation: Option<Vec<SourceOtherEquipment>>,
}

/// Sole retained InternalHeatGains module state and reached DataHeatBalance owners.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedInternalHeatGains {
    pub(crate) get_input_flag: bool,
    pub(crate) errors_found: bool,
    pub(crate) allocations: SourceGainAllocations,
    // Same Source order as getter definition queries; original totals ctor0.
    pub(crate) totals: [i32; 10],
    // Actual module-global misc arrays; other families' misc arrays are locals.
    pub(crate) lights_objects: Option<Vec<SourceMiscGainObject>>,
    pub(crate) electric_objects: Option<Vec<SourceMiscGainObject>>,
    pub(crate) zero_pointer_value: f64,
}
impl Default for SelectedInternalHeatGains {
    fn default() -> Self {
        Self {
            get_input_flag: true,
            errors_found: false,
            allocations: Default::default(),
            totals: [0; 10],
            lights_objects: None,
            electric_objects: None,
            zero_pointer_value: 0.0,
        }
    }
}

fn unavailable(message: &'static str) -> E {
    E::Unavailable(message)
}
fn index(number: i32) -> Result<usize, E> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or_else(|| unavailable("positive Source internal-gain association"))
}

impl CtfInitializationContext<'_> {
    pub(crate) fn manage_selected_internal_heat_gains_input(
        &mut self,
        model: &TypedModel,
        minutes: i32,
        host: &mut dyn crate::heat_balance::source_output_registration::SourceOutputFileHost,
    ) -> Result<(), crate::RuntimeError> {
        let reports = &mut *self.reports;
        if reports.selected_internal_heat_gains.get_input_flag {
            let raw = reports
                .heat_balance_input
                .as_ref()
                .ok_or_else(|| hb_input_error(unavailable("same raw InternalHeatGains input")))?;
            let input = reports
                .selected_input_processor
                .as_mut()
                .ok_or_else(|| hb_input_error(unavailable("same InternalHeatGains IP")))?;
            let globals = reports
                .selected_heat_balance_globals
                .as_ref()
                .ok_or_else(|| hb_input_error(unavailable("same internal-gain global counts")))?;
            let zones = reports.selected_zone_input.as_mut().ok_or_else(|| {
                hb_input_error(unavailable("same internal-gain Zone/Space owners"))
            })?;
            let schedules = reports
                .selected_schedule_manager
                .as_mut()
                .ok_or_else(|| hb_input_error(unavailable("same actual ScheduleManager")))?;
            let minimum = &reports
                .selected_project_data
                .as_ref()
                .ok_or_else(|| hb_input_error(unavailable("same output minimum frequency")))?
                .system
                .minimum_report_frequency;
            let mut owners = getter::SourceGainInputOwners {
                raw,
                input,
                globals,
                zones,
                schedules,
                backend: &mut self.backend,
                registration: &mut reports.output_registration,
                prescan: &reports.output_prescan,
                report_inputs: &model.report_inputs,
                minimum_report_frequency: minimum,
                minutes,
                host,
                eio: self.eio,
            };
            reports
                .selected_internal_heat_gains
                .get_input(&mut owners)
                .map_err(hb_input_error)?;
            // Original210 follows the getter RETURN, including all output/EIO.
            reports.selected_internal_heat_gains.get_input_flag = false;
        }
        // Actual present InitOnly=true returns before Init/Report/CheckReturnAir.
        Ok(())
    }

    pub(crate) fn check_selected_kiva_input_guard(&self) -> Result<(), crate::RuntimeError> {
        if self.reports.surface_geometry_input.any_kiva {
            // Actual next operation is Kiva setup callbacks/setpoints/weather.
            return Err(hb_input_error(unavailable("positive Source Kiva setup")));
        }
        Ok(())
    }
}
