//! Output-only selected-owner, stored projection and real solver-entry copies.

use crate::{RunConfig, RunError, RunExitCode};
use ep_runtime::heat_balance::ZoneAirInitializationState;
use ep_runtime::heat_balance::zone_air_initialization_trace::{
    LegacyZoneAirInitializationProjection, ZoneAirInitializationTrace,
};
use serde_json::{Value, json};
use std::io::{BufWriter, Write};

fn scalar(value: f64) -> Value {
    let class = if value.is_nan() {
        "nan"
    } else if value == f64::INFINITY {
        "positive_infinity"
    } else if value == f64::NEG_INFINITY {
        "negative_infinity"
    } else if value == 0.0 {
        if value.is_sign_negative() {
            "negative_zero"
        } else {
            "positive_zero"
        }
    } else {
        "finite"
    };
    json!({"value":if value.is_finite(){Some(value)}else{None},
        "value_bits":format!("{:016x}",value.to_bits()),"value_class":class})
}

fn fields(owner: &ZoneAirInitializationState) -> Value {
    json!({
        "MAT":scalar(owner.mat),"ZT":scalar(owner.zt),"ZTAV":scalar(owner.ztav),
        "XMPT":scalar(owner.xmpt),"XMAT":owner.xmat.map(scalar),"DSXMAT":owner.dsxmat.map(scalar),
        "TMX":scalar(owner.tmx),"TM2":scalar(owner.tm2),
        "airHumRat":scalar(owner.air_hum_rat),"airHumRatAvg":scalar(owner.air_hum_rat_avg),
        "ZTM":owner.ztm.map(scalar),"WPrevZoneTS":owner.w_prev_zone_ts.map(scalar),
        "DSWPrevZoneTS":owner.dsw_prev_zone_ts.map(scalar),
        "WPrevZoneTSTemp":owner.w_prev_zone_ts_temp.map(scalar),
        "WTimeMinusP":scalar(owner.w_time_minus_p),"W1":scalar(owner.w1),
        "WMX":scalar(owner.wmx),"WM2":scalar(owner.wm2),
        "airHumRatTemp":scalar(owner.air_hum_rat_temp),"tempIndLoad":scalar(owner.temp_ind_load),
        "tempDepLoad":scalar(owner.temp_dep_load),"airRelHum":scalar(owner.air_rel_hum),
        "AirPowerCap":scalar(owner.air_power_cap),"T1":scalar(owner.t1),
    })
}

fn handoff(value: &LegacyZoneAirInitializationProjection) -> Value {
    json!({
        "legacy_fields":{
            "mean_air_temperature_c":scalar(value.mean_air_temperature_c),
            "zone_timestep_average_air_temperature_c":scalar(value.zone_timestep_average_air_temperature_c),
            "air_humidity_ratio":scalar(value.air_humidity_ratio),
            "zone_timestep_average_air_humidity_ratio":scalar(value.zone_timestep_average_air_humidity_ratio),
            "previous_mean_air_temperatures_c":value.previous_mean_air_temperatures_c.map(scalar),
            "previous_system_mean_air_temperatures_c":value.previous_system_mean_air_temperatures_c.map(scalar),
            "previous_air_humidity_ratios":value.previous_air_humidity_ratios.map(scalar),
            "previous_system_air_humidity_ratios":value.previous_system_air_humidity_ratios.map(scalar),
        },
        "legacy_diagnostic_snapshots":{
            "third_order_temp_independent_load_w":scalar(value.third_order_temp_independent_load_w),
            "third_order_temp_dependent_load_w_per_k":scalar(value.third_order_temp_dependent_load_w_per_k),
            "air_power_cap_w_per_k":scalar(value.air_power_cap_w_per_k),
        },
    })
}

pub(crate) fn write_trace(
    config: &RunConfig,
    trace: &ZoneAirInitializationTrace,
) -> Result<(), RunError> {
    let artifact = json!({
        "schema":"zon01-initialization-trace.v1",
        "capture_source":"actual-Rust-zone-air-initialization-and-shared-solver-entry",
        "thread_coverage":"collecting-thread-only",
        "observer_supplies_inputs":false,"observer_recalculates_initialization":false,
        "observation_limit_per_series":10_000,
        "total_initializer_count":trace.total_initializer_count,
        "retained_initializer_count":trace.initializer_observations.len(),
        "omitted_initializer_count":trace.total_initializer_count-trace.initializer_observations.len() as u64,
        "total_timestep_entry_call_count":trace.total_timestep_entry_call_count,
        "zero_index_timestep_entry_count":trace.zero_index_timestep_entry_count,
        "retained_zero_index_timestep_entry_count":trace.timestep_entry_observations.len(),
        "omitted_zero_index_timestep_entry_count":trace.zero_index_timestep_entry_count-trace.timestep_entry_observations.len() as u64,
        "initializer_observations":trace.initializer_observations.iter().enumerate().map(|(index,row)| {
            let value = &row.observation;
            json!({
                "sequence":index+1,"zone_id":value.zone_id.0,"zone_name":value.zone_name,
                "caller":{"file":row.caller.file(),"line":row.caller.line(),"column":row.caller.column()},
                "phase":row.phase,"context":row.context.map(crate::psychrometrics_trace::execution_context),
                "out_hum_rat":scalar(value.out_hum_rat),
                "external_out_hum_rat_producer":"existing-Rust-weather-entry-provider-unpaired-with-native-stages",
                "caller_temperature_inputs":{
                    "MAT":scalar(value.caller_temperature_inputs.mat),"ZTAV":scalar(value.caller_temperature_inputs.ztav),
                    "XMAT_first_three":value.caller_temperature_inputs.xmat.map(scalar),
                    "DSXMAT_first_three":value.caller_temperature_inputs.dsxmat.map(scalar),
                },
                "constructor":fields(&value.constructor),"after_bulk":fields(&value.after_bulk),
                "before_begin":fields(&value.before_begin),"after_begin":fields(&value.after_begin),
                "guard":{
                    "begin_environment":value.guard.begin_environment,
                    "my_environment_before":value.guard.my_environment_before,
                    "eligible_before":value.guard.eligible_before,
                    "my_environment_after":value.guard.my_environment_after,
                    "initializer_invocations":value.guard.initializer_invocations,
                    "initializer_invocations_are_Rust_only":true,
                },
                "handoff":handoff(&value.handoff),
            })
        }).collect::<Vec<_>>(),
        "timestep_entry_observations":trace.timestep_entry_observations.iter().enumerate().map(|(index,row)|json!({
            "sequence":index+1,"timestep_index":row.timestep_index,
            "caller":{"file":row.caller.file(),"line":row.caller.line(),"column":row.caller.column()},
            "phase":row.phase,"context":row.context.map(crate::psychrometrics_trace::execution_context),
            "MyEnvrnFlag":row.my_environment_flag,
            "zones":row.zones.iter().map(|zone|json!({
                "zone_id":zone.zone_id.0,"zone_name":zone.zone_name,"handoff":handoff(&zone.handoff),
            })).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "source_correspondence":{
            "MAT":"mean_air_temperature_c","ZTAV":"zone_timestep_average_air_temperature_c",
            "airHumRat":"air_humidity_ratio","airHumRatAvg":"zone_timestep_average_air_humidity_ratio",
            "XMAT_first_three":"previous_mean_air_temperatures_c",
            "DSXMAT_first_three":"previous_system_mean_air_temperatures_c",
            "WPrevZoneTS_first_three":"previous_air_humidity_ratios",
            "DSWPrevZoneTS_first_three":"previous_system_air_humidity_ratios",
            "tempIndLoad":"third_order_temp_independent_load_w",
            "tempDepLoad":"third_order_temp_dependent_load_w_per_k","AirPowerCap":"air_power_cap_w_per_k",
        },
        "claim_boundary":"selected transient initialization owner, actual stored projection and zero-index shared solver entry; no native-weather/calendar/warmup-stage pairing or later state retention",
        "unpaired_source_state":["thermostat/day/demand/hybrid/Space siblings","native manager contextual flags and IO","native member invocation counts","later fourth-slot/working-history evolution","AirPowerCap result and J/K capacity physics"],
    });
    let path = config.output_dir.join("zon01-initialization.json");
    let write = || -> Result<(), String> {
        let file = std::fs::File::create(&path).map_err(|error| error.to_string())?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &artifact).map_err(|error| error.to_string())?;
        writer
            .write_all(b"\n")
            .and_then(|()| writer.flush())
            .map_err(|error| error.to_string())
    };
    write().map_err(|message| RunError {
        exit_code: RunExitCode::OutputExport,
        message,
    })
}
