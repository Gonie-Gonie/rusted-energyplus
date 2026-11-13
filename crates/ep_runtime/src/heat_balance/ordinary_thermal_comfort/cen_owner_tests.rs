//! Boundary operands exercise the production CEN body after real getter,
//! geometry, MRT and Manage(true) initialization in the runtime fixture.
use super::*;

impl SourceThermalComfort {
    // No surrogate CEN calculation: all result writes occur in production body.
    fn cen_owner_operand_call<H, D>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
        target: f64,
        begin_day: bool,
        begin_hour: bool,
        outdoor: f64,
    ) -> Result<(), Error> {
        let mut clock = o.clock.clone();
        clock.begin_day_flag = begin_day;
        clock.begin_hour_flag = begin_hour;
        let mut environment = o.environment.clone();
        environment.current_weather.out_dry_bulb_temp = outdoor;
        let mut hvac = o.hvac.clone();
        hvac.system_time_elapsed = 0.125;
        let mut air = o.air.clone();
        for zone in air
            .zones
            .as_mut()
            .ok_or(Error::Unavailable("CEN test AirHB"))?
        {
            zone.air.ztav_comf = 2.0 * target;
        }
        let mut child = SourceThermalComfortOwners {
            gains: &mut *o.gains,
            schedules: o.schedules,
            developer: o.developer,
            zones: o.zones,
            control: o.control,
            globals: o.globals,
            clock: &clock,
            environment: &environment,
            hvac: &hvac,
            air: &air,
            controller: o.controller,
            fan: &mut *o.fan,
            radiation: o.radiation,
            geometry: &mut *o.geometry,
            history: o.history,
            psychrometrics: &mut *o.psychrometrics,
            recurring_errors: &mut *o.recurring_errors,
            input: &mut *o.input,
            source_is_epjson: o.source_is_epjson,
            shortcut: &mut *o.shortcut,
            registration: &mut *o.registration,
            prescan: o.prescan,
            report_inputs: o.report_inputs,
            minimum_report_frequency: o.minimum_report_frequency,
            host: &mut *o.host,
            dictionary_host: &mut *o.dictionary_host,
            eio: &mut *o.eio,
            backend: &mut *o.backend,
            registry: &mut *o.registry,
            table: &mut *o.table,
            predefined_summary: &mut *o.predefined_summary,
        };
        self.calc_source_adaptive_cen15251(&mut child)?;
        assert_eq!(self.rad_temp.to_bits(), 0.0_f64.to_bits());
        assert_eq!(self.op_temp.to_bits(), target.to_bits());
        Ok(())
    }

    pub(crate) fn verify_cen_owner_boundaries<H, D>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
        occupied: bool,
    ) -> Result<usize, Error> {
        // Fixture declares CEN on only first People; second is an untouched canary.
        assert!(o.gains.allocations.people.rows[0].adaptive_cen15251);
        assert!(!o.gains.allocations.people.rows[1].adaptive_cen15251);
        let person = &o.gains.allocations.people.rows[0];
        let actual_occupancy = person.number_of_people
            * o.schedules.source_current_schedule_value(
                person
                    .sched
                    .ok_or(Error::Unavailable("CEN actual occupancy schedule"))?,
            )?;
        assert_eq!(actual_occupancy > 0.0, occupied);
        let other = self
            .thermal_comfort_data
            .as_ref()
            .ok_or(Error::Unavailable("CEN rows"))?[1]
            .clone();
        let other_hours = cen_hours(o, 1);
        let ash = (
            self.running_average_ash.to_bits(),
            self.avg_dry_bulb_ash.to_bits(),
        );
        assert!(
            !self.first_time_flag,
            "must enter through genuine Manage(true) first"
        );
        for first_day in [false, true] {
            for epw in [false, true] {
                self.first_day_set = first_day;
                self.use_epw_data_cen = epw;
                self.avg_dry_bulb_cen = 9.0;
                self.running_average_cen = 8.0;
                seed_cen_hours(o);
                self.initialize_source_adaptive_cen15251_design_day(25.0);
                assert_eq!(self.running_average_cen, 25.0);
                assert_eq!(self.avg_dry_bulb_cen.to_bits(), 0.0_f64.to_bits());
                assert_eq!(self.first_day_set, first_day);
                assert_eq!(self.use_epw_data_cen, epw);
                assert_eq!(cen_hours(o, 0), [1.0, 2.0, 3.0]);
            }
        }
        let mut calls = 0;
        // Independent binary64 reference constants for Native multiply then add.
        // 25.4 literal is one ULP LOW and must not be used as the rm20 center.
        for (running, center_bits, lower_center) in [
            (
                20.0,
                0x4039666666666667_u64,
                f64::from_bits(0x4039666666666667),
            ),
            (10.0, 0x403619999999999a_u64, 23.75),
        ] {
            let center = f64::from_bits(center_bits);
            for category in 0..3 {
                let width = [2.0, 3.0, 4.0][category];
                for upper in [false, true] {
                    let boundary = if upper {
                        center + width
                    } else {
                        lower_center - width
                    };
                    for side in [-1_i32, 0, 1] {
                        let operand = match side {
                            -1 => boundary.next_down(),
                            0 => boundary,
                            _ => boundary.next_up(),
                        };
                        // Inside vs equality/outside for this one strict boundary.
                        let inside = if upper { side < 0 } else { side > 0 };
                        let failed_categories = category + usize::from(!inside);
                        let expected = std::array::from_fn(|i| {
                            if occupied {
                                i32::from(i >= failed_categories)
                            } else {
                                -1
                            }
                        });
                        self.running_average_cen = running;
                        seed_cen_hours(o);
                        self.cen_owner_operand_call(o, operand, false, false, 0.0)?;
                        calls += 1;
                        check_cen_row(self, o, expected, center_bits, running.to_bits());
                    }
                }
            }
        }
        // Both inclusive applicability endpoints and their immediate outside neighbors.
        for (running, applicable, center_bits) in [
            (10.0_f64.next_down(), false, (-1.0_f64).to_bits()),
            (10.0, true, 0x403619999999999a),
            (30.0, true, 0x403cb33333333334),
            (30.0_f64.next_up(), false, (-1.0_f64).to_bits()),
        ] {
            self.running_average_cen = running;
            seed_cen_hours(o);
            self.cen_owner_operand_call(o, 23.0, false, false, 0.0)?;
            calls += 1;
            // At rm30 op23 is below CatIII lower bound: all fail if occupied.
            let expected = if !applicable || !occupied {
                [-1; 3]
            } else if running == 30.0 {
                [0; 3]
            } else {
                [1; 3]
            };
            check_cen_row(self, o, expected, center_bits, running.to_bits());
        }
        // Exact 15 transition: low center is tComf=23.75; equality fails Cat I.
        self.running_average_cen = 15.0;
        seed_cen_hours(o);
        self.cen_owner_operand_call(o, 21.75, false, false, 0.0)?;
        calls += 1;
        check_cen_row(
            self,
            o,
            if occupied { [0, 1, 1] } else { [-1; 3] },
            23.75_f64.to_bits(),
            15.0_f64.to_bits(),
        );

        // First DD day is updated even though EPW flag is false.
        self.use_epw_data_cen = false;
        self.first_day_set = false;
        self.initialize_source_adaptive_cen15251_design_day(25.0);
        self.cen_owner_operand_call(o, 25.0, true, false, 0.0)?;
        calls += 1;
        assert_eq!(self.running_average_cen.to_bits(), 20.0_f64.to_bits());
        assert_eq!(self.avg_dry_bulb_cen.to_bits(), 0.0_f64.to_bits());
        // Detect forbidden replacement of (1-alpha) with decimal 0.2.
        self.running_average_cen = 0.0;
        self.avg_dry_bulb_cen = 1.0;
        seed_cen_hours(o);
        self.cen_owner_operand_call(o, 25.0, true, false, 0.0)?;
        calls += 1;
        check_cen_row(self, o, [-1; 3], (-1.0_f64).to_bits(), 0x3fc9999999999998);
        assert_eq!(self.avg_dry_bulb_cen.to_bits(), 0.0_f64.to_bits());
        // firstDaySet skip, unconditional clear, independent BeginHour add.
        self.running_average_cen = 25.0;
        self.avg_dry_bulb_cen = 1.0;
        self.first_day_set = true;
        self.cen_owner_operand_call(o, 25.0, true, true, 24.0)?;
        calls += 1;
        assert_eq!(self.running_average_cen, 25.0);
        assert_eq!(self.avg_dry_bulb_cen, 2.0);
        assert!(!self.first_day_set);
        // Non-BeginDay hour adds, non-hour call does not add or reset.
        self.cen_owner_operand_call(o, 25.0, false, true, 24.0)?;
        calls += 1;
        assert_eq!(self.avg_dry_bulb_cen, 3.0);
        self.cen_owner_operand_call(o, 25.0, false, false, 96.0)?;
        calls += 1;
        assert_eq!(self.avg_dry_bulb_cen, 3.0);
        // Next day consumes daily avg before reset, then current hour contributes.
        self.running_average_cen = 0.0;
        self.avg_dry_bulb_cen = 1.0;
        self.cen_owner_operand_call(o, 25.0, true, true, 48.0)?;
        calls += 1;
        assert_eq!(self.running_average_cen.to_bits(), 0x3fc9999999999998);
        assert_eq!(self.avg_dry_bulb_cen, 2.0);
        assert_eq!(self.thermal_comfort_data.as_ref().unwrap()[1], other);
        assert_eq!(cen_hours(o, 1), other_hours);
        assert_eq!(
            (
                self.running_average_ash.to_bits(),
                self.avg_dry_bulb_ash.to_bits()
            ),
            ash
        );
        assert_eq!(calls, 47);
        Ok(calls)
    }
}
fn cen_hours<H, D>(o: &SourceThermalComfortOwners<'_, '_, H, D>, at: usize) -> [f64; 3] {
    let p = &o.gains.allocations.people.rows[at];
    [
        p.time_not_met_cen15251_cat_i,
        p.time_not_met_cen15251_cat_ii,
        p.time_not_met_cen15251_cat_iii,
    ]
}
fn seed_cen_hours<H, D>(o: &mut SourceThermalComfortOwners<'_, '_, H, D>) {
    // Explicit retained-owner input state, never a calculated/output answer.
    let p = &mut o.gains.allocations.people.rows[0];
    p.time_not_met_cen15251_cat_i = 1.0;
    p.time_not_met_cen15251_cat_ii = 2.0;
    p.time_not_met_cen15251_cat_iii = 3.0;
}
fn check_cen_row<H, D>(
    c: &SourceThermalComfort,
    o: &SourceThermalComfortOwners<'_, '_, H, D>,
    status: [i32; 3],
    center: u64,
    running: u64,
) {
    let r = &c.thermal_comfort_data.as_ref().unwrap()[0];
    assert_eq!(
        [
            r.thermal_comfort_adaptive_cen15251_cat_i,
            r.thermal_comfort_adaptive_cen15251_cat_ii,
            r.thermal_comfort_adaptive_cen15251_cat_iii
        ],
        status
    );
    assert_eq!(r.t_comf_cen15251.to_bits(), center);
    assert_eq!(r.cen15251_running_mean_outdoor_temp.to_bits(), running);
    for (i, hours) in cen_hours(o, 0).into_iter().enumerate() {
        assert_eq!(
            hours,
            [1.0, 2.0, 3.0][i] + if status[i] == 0 { 0.125 } else { 0.0 }
        );
    }
}
