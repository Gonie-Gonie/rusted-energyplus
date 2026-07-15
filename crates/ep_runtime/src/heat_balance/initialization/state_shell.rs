//! Final assembly of initialized heat-balance state and cache diagnostics.

use crate::heat_balance::state::{
    HeatBalanceState, HeatBalanceSurfaceIndexes, SurfaceHeatBalanceState, ZoneHeatBalanceState,
};
use crate::heat_balance::surface_manager::ConstructionThermalDataCache;

pub(super) fn finish_heat_balance_state(
    zones: Vec<ZoneHeatBalanceState>,
    surfaces: Vec<SurfaceHeatBalanceState>,
    surface_indexes: HeatBalanceSurfaceIndexes,
    construction_thermal_data: &ConstructionThermalDataCache,
) -> HeatBalanceState {
    HeatBalanceState {
        timestep_index: 0,
        zones,
        surfaces,
        surface_indexes,
        construction_cache_hash: construction_thermal_data.coefficient_cache_hash,
        construction_cache_build_wall_seconds: construction_thermal_data.build_wall_seconds,
        construction_cache_entry_count: construction_thermal_data.len(),
        construction_cache_no_mass_count: construction_thermal_data.no_mass_construction_ids.len(),
        construction_cache_massive_ctf_count: construction_thermal_data
            .massive_ctf_construction_ids
            .len(),
        construction_cache_eio_seeded_count: construction_thermal_data.eio_seeded_count(),
        construction_cache_rust_generated_count: construction_thermal_data.rust_generated_count(),
        variable_system_timestep_placeholder: true,
        hvac_iteration_count: 0,
        plant_iteration_count: 0,
        last_ctf_history_slot_terms: Vec::new(),
        last_ctf_history_slot_terms_after_advance: Vec::new(),
        last_inside_surface_iteration_count: 0,
        last_inside_surface_iteration_max_delta_c: f64::NAN,
        last_inside_surface_iteration_max_delta_surface_name: None,
    }
}
