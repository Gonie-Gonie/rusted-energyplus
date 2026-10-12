//! Heat-balance compatibility and diagnostic selection APIs.

#![allow(
    clippy::collapsible_if,
    clippy::too_many_arguments,
    clippy::type_complexity
)]

pub mod air_manager;
pub mod algorithm;
pub mod convection;
pub mod ctf;
pub mod ctf_caller_clock;
pub mod ctf_direct_initialization;
pub mod ctf_first_assembly_owner;
pub mod ctf_first_exponential_owner;
pub mod ctf_first_final_coefficients_ingress;
pub mod ctf_first_final_coefficients_owner;
pub mod ctf_first_gamma_ingress;
pub mod ctf_first_gamma_owner;
pub mod ctf_first_inverse_owner;
pub mod ctf_first_matrix_owner;
pub mod ctf_geometry_reports;
pub mod ctf_history_capture;
pub mod ctf_init_postlude;
pub mod ctf_initial_owner;
pub mod ctf_initialization_context;
pub mod ctf_public_initialization;
pub mod ctf_reports;
pub mod ctf_retry_driver;
pub mod ctf_retry_initialization;
pub mod ctf_source_diagnostics;
pub mod ctf_surface_binding;
pub mod ctf_surface_lifecycle;
pub mod ctf_thermal_history;
pub mod error_backend;
pub(crate) mod id_slot_index;
pub mod initialization;
pub mod inside_convection;
pub mod inside_convection_dispatcher;
pub mod inside_convection_layout;
pub mod longwave;
pub mod manager;
pub mod ordinary_equipment;
pub(crate) mod ordinary_inside_temperatures;
pub mod predefined_tables;
pub mod radiation;
pub mod report_scan;
pub mod reports;
pub(crate) mod run_period;
pub mod selected_input_processor;
pub mod solar;
pub mod source_output_prescan;
pub(crate) mod source_round_format;
pub mod state;
pub mod summary;
pub mod surface_balance;
pub mod surface_boundary;
pub(crate) mod surface_loop;
pub mod surface_manager;
pub mod surface_weather;
pub mod timestep;
pub(crate) mod trace;
pub(crate) mod warmup;
pub(crate) mod weather_driver;
pub mod zone_air_correction;
pub mod zone_air_initialization;
pub mod zone_air_initialization_trace;
pub mod zone_humidity_correction;
pub mod zone_predictor_corrector;

pub use air_manager::*;
pub use algorithm::*;
pub use convection::*;
pub use ctf::*;
pub use initialization::*;
pub use inside_convection::*;
pub use longwave::*;
pub use manager::*;
pub use radiation::*;
pub use reports::*;
pub use solar::*;
pub use state::*;
pub use summary::*;
pub use surface_balance::*;
pub use surface_boundary::*;
pub use surface_manager::*;
pub use surface_weather::*;
pub use timestep::*;
pub use zone_air_correction::*;
pub use zone_air_initialization::*;
pub use zone_predictor_corrector::*;
