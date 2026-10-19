//! Native HeatBalanceIntRadExchange1520-1794 on the finite positive-area domain.
//! The shared kernel owns no scientific state: real module/Zone/backend owners
//! are borrowed by the Source entry; the legacy primitive retains its admission.
use super::ordinary::{
    OrdinaryRadiantExchangeState, OrdinaryRadiationError as Error,
    OrdinaryRadiationUnavailable as Unavailable,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;

/// Actual five mutable function arguments; callers supply their existing cells.
/// No constructor invents values for Native output arguments not yet assigned.
pub(crate) struct SourceViewFactorChecks {
    pub(crate) original: f64,
    pub(crate) fixed: f64,
    pub(crate) final_value: f64,
    pub(crate) iterations: i32,
    pub(crate) row_sum: f64,
}

pub(super) enum Diagnostic {
    #[allow(
        dead_code,
        reason = "Preserve the genuine Native FixViewFactors entry and diagnostic payloads for paired consumers; derived/debug transport does not count as a field read for this lint."
    )]
    Warning(String),
    #[allow(
        dead_code,
        reason = "Preserve the genuine Native FixViewFactors entry and diagnostic payloads for paired consumers; derived/debug transport does not count as a field read for this lint."
    )]
    Continue(String),
    #[allow(
        dead_code,
        reason = "Preserve the genuine Native FixViewFactors entry and diagnostic payloads for paired consumers; derived/debug transport does not count as a field read for this lint."
    )]
    Severe(String),
    #[allow(
        dead_code,
        reason = "Preserve the genuine Native FixViewFactors entry and diagnostic payloads for paired consumers; derived/debug transport does not count as a field read for this lint."
    )]
    Fatal(&'static str),
}

impl OrdinaryRadiantExchangeState {
    /// Full finite Source FixViewFactors, including real persistent writes and
    /// diagnostic requests at their original points. F retains row-major Array2
    /// order (ObjexxFCL Array2.hh608-626), also used by the existing ScriptF code.
    #[expect(
        clippy::too_many_arguments,
        reason = "Keep actual Native arguments and owner borrows distinct"
    )]
    #[allow(
        dead_code,
        reason = "Preserve the genuine Native FixViewFactors entry and diagnostic payloads for paired consumers; derived/debug transport does not count as a field read for this lint."
    )]
    pub(crate) fn fix_source_view_factors(
        &mut self,
        zones: &mut SelectedZoneInputOwners,
        areas: &[f64],
        f: &mut [f64],
        name: &str,
        space_numbers: &[i32],
        checks: &mut SourceViewFactorChecks,
        any_internal_mass: bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let n = areas.len();
        if n == 0 || i32::try_from(n).is_err() || n.checked_mul(n) != Some(f.len()) {
            return Err(Unavailable::StorageDomain.into());
        }
        if areas.iter().any(|v| !v.is_finite() || *v <= 0.0) || f.iter().any(|v| !v.is_finite()) {
            return Err(Unavailable::NumericDomain.into());
        }
        fix_kernel(
            areas,
            f,
            &mut self.largest_surface,
            name,
            space_numbers,
            checks,
            any_internal_mass,
            |message| {
                let result = match message {
                    Diagnostic::Warning(text) => backend.show_warning_error(&text),
                    Diagnostic::Continue(text) => backend.show_continue_error(&text),
                    Diagnostic::Severe(text) => backend.show_severe_error(&text),
                    Diagnostic::Fatal(text) => {
                        return match backend.show_fatal_error(text) {
                            Ok(never) => match never {},
                            Err(error) => Err(Error::Input(Box::new(
                                crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Fatal(error),
                            ))),
                        };
                    }
                };
                result
                    .map_err(|error| Error::Output(crate::error::WarmupOutputError::Backend(error)))
            },
            |space_number| {
                // This lookup is deliberately late: Native commits F first,
                // then consumes each actual Space→Zone and writes that Zone.
                let space_index = space_number
                    .checked_sub(1)
                    .and_then(|v| usize::try_from(v).ok())
                    .ok_or(Unavailable::StorageDomain)?;
                let zone_number = zones
                    .spaces
                    .as_ref()
                    .and_then(|v| v.get(space_index))
                    .ok_or(Unavailable::StorageDomain)?
                    .zone_num;
                let zone_index = zone_number
                    .checked_sub(1)
                    .and_then(|v| usize::try_from(v).ok())
                    .ok_or(Unavailable::StorageDomain)?;
                zones
                    .zones
                    .as_mut()
                    .and_then(|v| v.get_mut(zone_index))
                    .ok_or(Unavailable::StorageDomain)?
                    .enforced_reciprocity = true;
                Ok(())
            },
            |value, precision| {
                crate::heat_balance::source_round_format::round(value, precision).map_err(|error| {
                    Error::Output(crate::error::WarmupOutputError::Format(format!(
                        "FixViewFactors .{precision}R unavailable: {error:?}"
                    )))
                })
            },
        )
    }
}

/// One implementation of the original arithmetic. The legacy warning-free
/// interface calls this same function, with unavailable diagnostic callbacks.
#[expect(
    clippy::too_many_arguments,
    reason = "Original state/arguments and real callbacks are distinct"
)]
pub(super) fn fix_kernel<E>(
    areas: &[f64],
    f: &mut [f64],
    largest_surface: &mut i32,
    name: &str,
    space_numbers: &[i32],
    checks: &mut SourceViewFactorChecks,
    any_internal_mass: bool,
    mut diagnostic: impl FnMut(Diagnostic) -> Result<(), E>,
    mut enforce_reciprocity: impl FnMut(i32) -> Result<(), E>,
    mut round: impl FnMut(f64, usize) -> Result<String, E>,
) -> Result<(), E> {
    let n = areas.len();
    let n_real = n as f64;
    checks.original = (f.iter().sum::<f64>() - n_real).abs();
    let mut fixed_area_factors = f.to_vec();
    let mut convergence_old = 10.0;
    let mut largest_area = areas[0];
    for &area in &areas[1..] {
        if area > largest_area {
            largest_area = area;
        }
    }
    if largest_area > 0.99 * (areas.iter().sum::<f64>() - largest_area) && n > 3 {
        for (i, &area) in areas.iter().enumerate() {
            if largest_area != area {
                continue;
            }
            // Entry bounds already establish a represented i32 array extent.
            *largest_surface = i as i32 + 1;
            break;
        }
        let self_view = 1.2 * largest_area / areas.iter().sum::<f64>();
        let self_view = if 0.9 < self_view { 0.9 } else { self_view };
        let i = (*largest_surface - 1) as usize;
        fixed_area_factors[i * n + i] = self_view;
    }
    let mut area_factors = vec![0.0; f.len()];
    for i in 0..n {
        for j in 0..n {
            area_factors[j * n + i] = fixed_area_factors[j * n + i] * areas[i];
        }
    }
    fixed_area_factors = super::average_with_transpose(&area_factors, n);
    drop(area_factors);
    checks.iterations = 0;
    checks.row_sum = 0.0;
    if n <= 3 {
        let mut fixed_factors = super::area_factors_to_view_factors(&fixed_area_factors, areas);
        diagnostic(Diagnostic::Warning(format!(
            "Surfaces in Zone/Enclosure=\"{name}\" do not define an enclosure."
        )))?;
        for message in [
            "Number of surfaces <= 3, view factors are set to force reciprocity but may not fulfill completeness.",
            "Reciprocity means that radiant exchange between two surfaces will match and not lead to an energy loss.",
            "Completeness means that all of the view factors between a surface and the other surfaces in a zone add up to unity.",
            "So, when there are three or less surfaces in a zone, EnergyPlus will make sure there are no losses of energy but",
            "it will not exchange the full amount of radiation with the rest of the zone as it would if there was a completed enclosure.",
        ] {
            diagnostic(Diagnostic::Continue(message.into()))?;
        }
        checks.row_sum = fixed_factors.iter().sum::<f64>();
        if checks.row_sum > n_real + 0.01 {
            let mut maximum_row_sum = 0.0;
            for i in 0..n {
                let mut row_sum = 0.0;
                for j in 0..n {
                    row_sum += fixed_factors[i * n + j];
                }
                if i == 0 || row_sum > maximum_row_sum {
                    maximum_row_sum = row_sum;
                }
            }
            if maximum_row_sum < 1.0 {
                diagnostic(Diagnostic::Fatal(
                    " FixViewFactors: Three surface or less zone failing ViewFactorFix correction which should never happen.",
                ))?;
            } else {
                let coefficient = 1.0 / maximum_row_sum;
                for value in &mut fixed_factors {
                    *value *= coefficient;
                }
            }
            checks.row_sum = fixed_factors.iter().sum::<f64>();
        }
        checks.fixed = (checks.row_sum - n_real).abs();
        checks.final_value = checks.fixed;
        f.copy_from_slice(&fixed_factors);
        for &space in space_numbers {
            enforce_reciprocity(space)?;
        }
        return Ok(());
    }
    let (fixed_factors, convergence_new) = loop {
        checks.iterations += 1;
        for i in 0..n {
            let sum = (0..n).map(|j| fixed_area_factors[j * n + i]).sum::<f64>();
            let coefficient = if sum.abs() > 1.0e-10 {
                areas[i] / sum
            } else {
                1.0
            };
            for j in 0..n {
                fixed_area_factors[j * n + i] *= coefficient;
            }
        }
        fixed_area_factors = super::average_with_transpose(&fixed_area_factors, n);
        let mut fixed_factors = super::area_factors_to_view_factors(&fixed_area_factors, areas);
        for i in 0..n {
            for j in 0..n {
                let offset = j * n + i;
                if fixed_factors[offset].abs() < 1.0e-10 {
                    fixed_factors[offset] = 0.0;
                    fixed_area_factors[offset] = 0.0;
                }
            }
        }
        let convergence_new = (fixed_factors.iter().sum::<f64>() - n_real).abs();
        let converged =
            (convergence_old - convergence_new).abs() < 1.0e-5 || convergence_new <= 0.001;
        convergence_old = convergence_new;
        if checks.iterations > 400 {
            fixed_area_factors = super::average_with_transpose(&fixed_area_factors, n);
            fixed_factors = super::area_factors_to_view_factors(&fixed_area_factors, areas);
            let sum = fixed_factors.iter().sum::<f64>();
            let tolerance = (sum - n_real).abs();
            checks.fixed = tolerance;
            checks.final_value = checks.fixed;
            checks.row_sum = sum;
            if tolerance > 0.005 {
                if tolerance > 0.1 {
                    diagnostic(Diagnostic::Severe(format!(
                        "FixViewFactors: View factors convergence has failed and will lead to heat balance errors in zone=\"{name}\"."
                    )))?;
                }
                diagnostic(Diagnostic::Warning(format!(
                    "FixViewFactors: View factors not complete. Check for bad surface descriptions or unenclosed zone=\"{name}\"."
                )))?;
                let tolerance_text = round(tolerance, 6)?;
                let row_sum_text = round(checks.row_sum, 2)?;
                diagnostic(Diagnostic::Continue(format!(
                    "Enforced reciprocity has tolerance (ideal is 0)=[{tolerance_text}], Row Sum (ideal is {n})=[{row_sum_text}]."
                )))?;
                diagnostic(Diagnostic::Continue("If zone is unusual or tolerance is on the order of 0.001, view factors might be OK but results should be checked carefully.".into()))?;
                if any_internal_mass {
                    diagnostic(Diagnostic::Continue("For zones with internal mass like this one, thiscan happen when the internal mass has an area thatis much larger than the other surfaces in the zone.".into()))?;
                    diagnostic(Diagnostic::Continue("If a single thermal mass element exists in this zonethat has an area that is larger than the sum of therest of the surface areas, consider breaking it upinto two or more separate internal mass elements.".into()))?;
                }
            }
            if checks.fixed.abs() < checks.original.abs() {
                f.copy_from_slice(&fixed_factors);
                checks.final_value = checks.fixed;
            }
            return Ok(());
        }
        if converged {
            break (fixed_factors, convergence_new);
        }
    };
    checks.fixed = convergence_new;
    if checks.fixed < checks.original {
        f.copy_from_slice(&fixed_factors);
        checks.final_value = checks.fixed;
    } else {
        checks.final_value = checks.original;
        checks.row_sum = fixed_factors.iter().sum::<f64>();
        if (checks.row_sum - n_real).abs() < 0.001 {
            f.copy_from_slice(&fixed_factors);
            checks.final_value = checks.fixed;
        } else {
            diagnostic(Diagnostic::Warning(format!(
                "FixViewFactors: View factors not complete. Check for bad surface descriptions or unenclosed zone=\"{name}\"."
            )))?;
        }
    }
    // Native severeErrorPresent is initialized false and never written. Its
    // final conditional fatal is unreachable; no caller-owned latch is invented.
    Ok(())
}
