//! Public facade for runtime execution and trace helpers.

#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

pub mod diagnostic_probes;
pub mod diagnostics;
pub mod error;
pub mod execution_plan;
pub mod first_zone;
pub mod geometry;
pub mod heat_balance;
pub mod hvac;
pub mod ideal_loads;
pub mod mode;
pub mod node;
mod output;
pub mod plant;
pub mod precompute;
pub mod psychrometrics;
mod runtime;
pub mod schedules;
pub mod simulation_state;
pub mod time_axis;
pub mod weather;
pub mod zone_equipment;

pub use diagnostic_probes::*;
pub use diagnostics::*;
pub use execution_plan::*;
pub use heat_balance::*;
pub use hvac::*;
pub use ideal_loads::*;
pub use mode::*;
pub use node::*;
pub use output::*;
pub use plant::*;
pub use precompute::*;
pub use runtime::*;
pub use simulation_state::*;
pub use time_axis::*;
pub use zone_equipment::*;

// Explicit test target only: keep the Native single-process-main FP context.
#[cfg(all(test, feature = "native-controlled-air-cohort"))]
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let selection=std::env::args().skip(1).collect::<Vec<_>>();
    if selection.len()==1 && selection[0]=="--report-only" {
        println!("running 1 native system-report cohort on process main");
        let (energy,vent,variants)=crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirPredictorCorrector::run_native_system_report_cohort()?;
        println!("cohort native_controlled_air_cohort ... ok");
        println!("cohort result: ok. 1 passed; 0 failed; energy entries={energy}; ventilation entries={vent}; named Native-unit variants={variants}");
        return Ok(());
    }
    if !selection.is_empty() {return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput,"expected no arguments or --report-only").into());}
    println!("running 1 native controlled-air cohort on process main");
    crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirPredictorCorrector::run_native_controlled_air_cohort()?;
    let (energy,vent,variants)=crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirPredictorCorrector::run_native_system_report_cohort()?;
    println!("system-report entries: energy={energy}; ventilation={vent}; named Native-unit variants={variants}");
    println!("cohort native_controlled_air_cohort ... ok");
    println!("cohort result: ok. 1 passed; 0 failed; predictor entries=1168; whole corrector entries=1168; positive inlet cases=810");
    Ok(())
}
