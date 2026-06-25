//! Heat-balance reporting source-order ownership notes.

/// Source-order stage that writes zone heat-balance output rows.
pub const ZONE_REPORT_OWNER_STAGE: &str = "ReportHeatBalance";

/// Source-order stage that writes surface heat-balance output rows.
pub const SURFACE_REPORT_OWNER_STAGE: &str = "ReportSurfaceHeatBalance";

use crate::heat_balance::ctf::{
    surface_inside_conduction_rate_w_for_report, surface_outside_conduction_rate_w_for_report,
};
use crate::heat_balance::state::SurfaceHeatBalanceState;
use ep_model::ZoneId;

pub(crate) fn zone_surface_report_conduction_rates_w(
    surfaces: &[SurfaceHeatBalanceState],
    zone_id: ZoneId,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) -> (f64, f64) {
    surfaces
        .iter()
        .filter(|surface| surface.zone_id == zone_id)
        .map(|surface| {
            (
                surface_inside_conduction_rate_w_for_report(
                    surface,
                    use_inside_ctf_outside_temperature_for_conduction_report,
                ),
                surface_outside_conduction_rate_w_for_report(
                    surface,
                    use_inside_ctf_outside_temperature_for_conduction_report,
                ),
            )
        })
        .fold(
            (0.0, 0.0),
            |(inside_sum, outside_sum), (inside, outside)| {
                (inside_sum + inside, outside_sum + outside)
            },
        )
}

pub(crate) fn heat_gain_rate_w(rate_w: f64) -> f64 {
    rate_w.max(0.0)
}

pub(crate) fn heat_loss_rate_w(rate_w: f64) -> f64 {
    (-rate_w).max(0.0)
}
