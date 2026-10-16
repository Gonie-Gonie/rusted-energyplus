//! Genuine same-manager RecKeep/CheckWarmup/report consumers.
//! Actual ManageHeatBalance caller order and real air-demand producer remain prerequisites.
use super::{SourceHeatBalanceManager, SourceWarmupConvergence};
use crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance;
use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirDemand;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
use crate::heat_balance::selected_project_data::SelectedProjectData;
use crate::heat_balance::warmup::report::{
    DETAILED_HEADER, SUMMARY_HEADER, WarmupReportState, WarmupReportSummary,
};
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use std::io::Write;
mod check;
mod output;
mod record;
pub(crate) struct SourceWarmupRecordOwners<'a> {
    pub(crate) air: &'a SourceAirHeatBalance,
    pub(crate) demand: &'a SourceZoneAirDemand,
    pub(crate) geometry: &'a SurfaceGeometryInputState,
    pub(crate) history: &'a CtfThermalHistoryState,
    pub(crate) radiation: &'a mut OrdinarySolarRadiation,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) project: &'a SelectedProjectData,
    pub(crate) eio: &'a mut dyn Write,
}
pub(crate) struct SourceWarmupCheckOwners<'a, 'b> {
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) project: &'a SelectedProjectData,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a mut WeatherGlobalState,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) backend: &'a mut SourceErrorBackend<'b>,
}
pub(crate) struct SourceWarmupOutputOwners<'a> {
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) eio: &'a mut dyn Write,
}
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable("actual positive warmup Source ordinal"))
}
fn get(a: &Option<Vec<f64>>, i: usize) -> Result<f64, Error> {
    a.as_ref()
        .and_then(|a| a.get(i))
        .copied()
        .ok_or(Error::Unavailable("actual allocated warmup array cell"))
}
fn set(a: &mut Option<Vec<f64>>, i: usize, v: f64) -> Result<(), Error> {
    *a.as_mut()
        .and_then(|a| a.get_mut(i))
        .ok_or(Error::Unavailable("actual allocated warmup array output"))? = v;
    Ok(())
}
fn convergence(
    a: &mut Option<Vec<SourceWarmupConvergence>>,
    i: usize,
) -> Result<&mut SourceWarmupConvergence, Error> {
    a.as_mut()
        .and_then(|a| a.get_mut(i))
        .ok_or(Error::Unavailable(
            "actual allocated WarmupConvergenceValues",
        ))
}
fn report_array(report: &WarmupReportState, slot: usize) -> Result<(), Error> {
    if report.source_allocations.and_then(|a| a.get(slot).copied()) == Some(true) {
        Ok(())
    } else {
        Err(Error::Unavailable("actual Source warmup report allocation"))
    }
}
fn zone_name(zones: &SelectedZoneInputOwners, i: usize) -> Result<&str, Error> {
    Ok(zones
        .zones
        .as_ref()
        .and_then(|a| a.get(i))
        .ok_or(Error::Unavailable("actual Source warmup Zone name"))?
        .name
        .as_str())
}
fn cpp_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}
fn round(v: f64, p: usize) -> Result<String, Error> {
    crate::heat_balance::source_round_format::round(v, p)
        .map_err(|e| Error::Format(format!("actual warmup R formatter: {e:?}")))
}
fn pass_fail(flag: i32) -> Result<&'static str, Error> {
    match flag {
        1 => Ok("Fail"),
        2 => Ok("Pass"),
        _ => Err(Error::Unavailable("actual PassFail1..2 ordinal")),
    }
}
