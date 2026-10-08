//! Solar interpolation weights shared by the existing weather consumers.

pub(crate) fn weighted_solar_value(
    previous: f64,
    current: f64,
    next: f64,
    previous_weight: f64,
    current_weight: f64,
    next_weight: f64,
) -> f64 {
    previous.max(0.0) * previous_weight
        + current.max(0.0) * current_weight
        + next.max(0.0) * next_weight
}

pub(crate) fn solar_weather_interpolation_weights(
    zone_steps_per_hour: u32,
    timestep: u32,
) -> (f64, f64, f64) {
    let steps = zone_steps_per_hour.max(1);
    let timestep = timestep.clamp(1, steps);
    let current_weight = solar_interpolation_weight(steps, timestep);
    if steps == 1 {
        return (0.0, current_weight, 0.0);
    }
    let timestep_fraction = 1.0 / f64::from(steps);
    if (current_weight - 1.0).abs() <= f64::EPSILON {
        (0.0, current_weight, 0.0)
    } else if f64::from(timestep) * timestep_fraction < 0.5 {
        (1.0 - current_weight, current_weight, 0.0)
    } else {
        (0.0, current_weight, 1.0 - current_weight)
    }
}

fn solar_interpolation_weight(zone_steps_per_hour: u32, timestep: u32) -> f64 {
    let steps = zone_steps_per_hour.max(1);
    let timestep = timestep.clamp(1, steps);
    if steps.is_multiple_of(2) {
        let halfpoint = steps / 2;
        let distance = timestep.abs_diff(halfpoint);
        return 1.0 - f64::from(distance) / f64::from(steps);
    }

    if steps == 1 {
        1.0
    } else if steps == 3 {
        match timestep {
            1 | 2 => 5.0 / 6.0,
            _ => 0.5,
        }
    } else {
        let timestep_weight = 1.0 / f64::from(steps);
        let halfpoint = steps / 2;
        let peak_weight = 1.0 - timestep_weight / 2.0;
        if timestep == halfpoint || timestep == halfpoint + 1 {
            peak_weight
        } else if timestep > halfpoint + 1 {
            peak_weight - f64::from(timestep - (halfpoint + 1)) * timestep_weight
        } else {
            peak_weight - f64::from(halfpoint - timestep) * timestep_weight
        }
    }
}
