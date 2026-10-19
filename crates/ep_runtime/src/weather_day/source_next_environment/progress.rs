//! Actual DisplaySimDaysProgress159-194 over the two sole native callbacks.
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_project_data::ProjectSystemVariables;

/// Actual free function pointer identity; no external API writer is supplied here.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SourceProgressFunction(pub(crate) fn(i32));
impl PartialEq for SourceProgressFunction {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::fn_addr_eq(self.0, other.0)
    }
}
/// A positive captured std::function is distinct from its genuine null constructor.
/// Its invocation is unavailable until an actual API callback owner is transported.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SourceCapturedProgressCallback {
    _unported: (),
}
/// Actual DataGlobals.hh167/169 members, retained on the same global owner.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceProgressCallbacks {
    pub(crate) f_progress: Option<SourceProgressFunction>,
    pub(crate) progress_callback: Option<SourceCapturedProgressCallback>,
}

// Keep Original constructor/local initialization and later assignment order, including unconsumed suffix writes.
#[allow(unused_assignments)]
pub(super) fn display_source_simulation_days(
    globals: &SelectedHeatBalanceGlobals,
    system: &ProjectSystemVariables,
    current: i32,
    total: i32,
) -> Result<(), Error> {
    let mut percent = 0;
    if globals.kickoff_simulation && !system.developer_flag {
        return Ok(());
    }
    if total > 0 {
        // The float division is promoted by Native's double literal100.0;
        // ObjexxFCL therefore instantiates nint(T=double), after that product.
        let value = f64::from(current as f32 / total as f32) * 100.0_f64;
        let sign = if value >= 0.0 { 1.0_f64 } else { -1.0_f64 };
        let rounded = value + sign * 0.5_f64;
        let truncated = rounded.trunc();
        if !rounded.is_finite()
            || truncated < f64::from(i32::MIN)
            || truncated > f64::from(i32::MAX)
        {
            return Err(Error::Unavailable(
                "actual DisplaySimDaysProgress nint signed-int domain182",
            ));
        }
        percent = rounded as i32;
        percent = if 100 < percent { 100 } else { percent };
    } else {
        percent = 0;
    }
    if let Some(callback) = globals.progress_callbacks.f_progress {
        (callback.0)(percent);
    }
    if globals.progress_callbacks.progress_callback.is_some() {
        return Err(Error::Unavailable(
            "actual captured progressCallback invocation192",
        ));
    }
    Ok(())
}
