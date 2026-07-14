use super::*;

#[test]
fn report_snapshot_copies_every_reported_calculation_field() {
    let calculation = IdealLoadsSensibleResult {
        mode: IdealLoadsSensibleMode::Cooling,
        cp_air_j_per_kg_k: 1.0,
        supply_temperature_c: 2.0,
        supply_humidity_ratio: 3.0,
        supply_enthalpy_j_per_kg: 4.0,
        supply_mass_flow_rate_kg_per_s: 5.0,
        heating_mass_flow_rate_kg_per_s: 6.0,
        cooling_mass_flow_rate_kg_per_s: 7.0,
        zone_total_heating_rate_w: 8.0,
        zone_total_cooling_rate_w: 9.0,
        zone_sensible_heating_rate_w: 10.0,
        zone_sensible_cooling_rate_w: 11.0,
        zone_latent_heating_rate_w: 12.0,
        zone_latent_cooling_rate_w: 13.0,
        supply_air_sensible_heating_rate_w: 14.0,
        supply_air_sensible_cooling_rate_w: 15.0,
        supply_air_latent_heating_rate_w: 16.0,
        supply_air_latent_cooling_rate_w: 17.0,
        supply_air_total_heating_rate_w: 18.0,
        supply_air_total_cooling_rate_w: 19.0,
    };

    assert_eq!(
        IdealLoadsReportSnapshot::from(calculation),
        IdealLoadsReportSnapshot {
            mode: IdealLoadsSensibleMode::Cooling,
            zone_total_heating_rate_w: 8.0,
            zone_total_cooling_rate_w: 9.0,
            zone_sensible_heating_rate_w: 10.0,
            zone_sensible_cooling_rate_w: 11.0,
            zone_latent_heating_rate_w: 12.0,
            zone_latent_cooling_rate_w: 13.0,
            supply_air_sensible_heating_rate_w: 14.0,
            supply_air_sensible_cooling_rate_w: 15.0,
            supply_air_latent_heating_rate_w: 16.0,
            supply_air_latent_cooling_rate_w: 17.0,
            supply_air_total_heating_rate_w: 18.0,
            supply_air_total_cooling_rate_w: 19.0,
            supply_mass_flow_rate_kg_per_s: 5.0,
            supply_temperature_c: 2.0,
            supply_humidity_ratio: 3.0,
        }
    );
}
