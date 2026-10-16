//! Complete CheckWarmupConvergence3059-3226; actual diagnostics precede transfers/resets.
use super::*;
impl SourceHeatBalanceManager {
    pub(crate) fn check_source_warmup_convergence(
        &mut self,
        o: &mut SourceWarmupCheckOwners<'_, '_>,
    ) -> Result<(), Error> {
        if o.globals.num_of_zones <= 0 {
            o.clock.warmup_flag = false;
            return Ok(());
        }
        let mut failed = false;
        for number in 1..=o.globals.num_of_zones {
            let i = index(number)?;
            let maximum = (get(&self.max_temp_prev_day, i)? - get(&self.max_temp_zone, i)?).abs();
            convergence(&mut self.convergence, i)?.test_values[0] = maximum;
            let minimum = (get(&self.min_temp_prev_day, i)? - get(&self.min_temp_zone, i)?).abs();
            convergence(&mut self.convergence, i)?.test_values[1] = minimum;
            if convergence(&mut self.convergence, i)?.test_values[0]
                <= o.control.temperature_convergence_tolerance
            {
                convergence(&mut self.convergence, i)?.pass_flags[0] = 2;
            } else {
                failed = true;
                convergence(&mut self.convergence, i)?.pass_flags[0] = 1;
            }
            if convergence(&mut self.convergence, i)?.test_values[1]
                <= o.control.temperature_convergence_tolerance
            {
                convergence(&mut self.convergence, i)?.pass_flags[1] = 2;
            } else {
                failed = true;
                convergence(&mut self.convergence, i)?.pass_flags[1] = 1;
            }
            check_load(
                &mut self.max_heat_load_zone,
                &mut self.max_heat_load_prev_day,
                convergence(&mut self.convergence, i)?,
                i,
                2,
                o.control.loads_convergence_tolerance,
                &mut failed,
            )?;
            check_load(
                &mut self.max_cool_load_zone,
                &mut self.max_cool_load_prev_day,
                convergence(&mut self.convergence, i)?,
                i,
                3,
                o.control.loads_convergence_tolerance,
                &mut failed,
            )?;
            if o.clock.day_of_sim >= o.control.max_number_of_warmup_days
                && o.clock.warmup_flag
                && convergence(&mut self.convergence, i)?
                    .pass_flags
                    .iter()
                    .sum::<i32>()
                    != 8
            {
                output::zone_failure(self, o, i)?;
            }
            set(
                &mut self.max_heat_load_prev_day,
                i,
                get(&self.max_heat_load_zone, i)?,
            )?;
            set(
                &mut self.max_cool_load_prev_day,
                i,
                get(&self.max_cool_load_zone, i)?,
            )?;
            set(&mut self.max_temp_prev_day, i, get(&self.max_temp_zone, i)?)?;
            set(&mut self.min_temp_prev_day, i, get(&self.min_temp_zone, i)?)?;
            set(&mut self.max_heat_load_zone, i, -9999.0)?;
            set(&mut self.max_cool_load_zone, i, -9999.0)?;
            set(&mut self.max_temp_zone, i, -9999.0)?;
            set(&mut self.min_temp_zone, i, 1000.0)?;
        }
        if o.clock.day_of_sim >= o.control.max_number_of_warmup_days
            && o.clock.warmup_flag
            && failed
            && o.control.max_number_of_warmup_days < 25
        {
            o.backend.show_severe_error(&format!(
                "CheckWarmupConvergence: User supplied maximum warmup days={} is insufficient.",
                o.control.max_number_of_warmup_days
            ))?;
            o.backend.show_continue_error(
                "Suggest setting maximum number of warmup days to at least 25.",
            )?;
        }
        if !failed && o.clock.day_of_sim >= o.project.min_number_of_warmup_days {
            o.clock.warmup_flag = false;
        } else if !failed && o.clock.day_of_sim < o.project.min_number_of_warmup_days {
            o.clock.warmup_flag = true;
        }
        if o.clock.day_of_sim >= o.control.max_number_of_warmup_days && o.clock.warmup_flag {
            o.clock.warmup_flag = false;
        }
        Ok(())
    }
}
fn check_load(
    current: &mut Option<Vec<f64>>,
    previous: &mut Option<Vec<f64>>,
    conv: &mut SourceWarmupConvergence,
    i: usize,
    test: usize,
    tolerance: f64,
    failed: &mut bool,
) -> Result<(), Error> {
    if get(current, i)? > 1.0e-4 {
        set(current, i, cpp_max(get(current, i)?, 100.0).abs())?;
        set(previous, i, cpp_max(get(previous, i)?, 100.0).abs())?;
        conv.test_values[test] = ((get(current, i)? - get(previous, i)?) / get(current, i)?).abs();
        if conv.test_values[test] <= tolerance {
            conv.pass_flags[test] = 2;
        } else {
            *failed = true;
            conv.pass_flags[test] = 1;
        }
    } else {
        conv.pass_flags[test] = 2;
    }
    Ok(())
}
