//! Actual no-positive GetRefrigerationInput/SetupReportInput/zone-gain route.
use super::{
    E, SelectedHeatBalanceGlobals, SelectedInputProcessor, WeatherGlobalState, unavailable,
};

/// Persistent Original counters; NumCompressorLists is local and is not here.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceRefrigerationCounts {
    pub(crate) cascade_condensers: i32,
    pub(crate) cases: i32,
    pub(crate) case_and_walk_in_lists: i32,
    pub(crate) racks: i32,
    pub(crate) secondary_systems: i32,
    pub(crate) transfer_load_lists: i32,
    pub(crate) walk_ins: i32,
    pub(crate) systems: i32,
    pub(crate) transcritical_systems: i32,
    pub(crate) air_condensers: i32,
    pub(crate) evaporative_condensers: i32,
    pub(crate) water_condensers: i32,
    pub(crate) gas_coolers: i32,
    pub(crate) condensers: i32,
    pub(crate) compressors: i32,
    pub(crate) subcoolers: i32,
    pub(crate) chiller_sets: i32,
    pub(crate) air_chillers: i32,
    /// Assigned only by the excluded positive subcooler getter; constructor0.
    pub(crate) mechanical_subcoolers: i32,
}

/// Actual RefrigeratedCaseData constructor members and selected reached array.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceRefrigerationGainModule {
    pub(crate) counts: SourceRefrigerationCounts,
    pub(crate) get_input: bool,
    pub(crate) have_racks: bool,
    pub(crate) have_detailed: bool,
    pub(crate) have_detailed_transcritical: bool,
    pub(crate) have_cases_or_walk_ins: bool,
    pub(crate) have_chillers: bool,
    pub(crate) manage_refrigeration: bool,
    pub(crate) use_system_time_step: bool,
    pub(crate) zone_gains_environment_flag: bool,
    pub(crate) present_in_zone: Option<Vec<bool>>,
}
impl Default for SourceRefrigerationGainModule {
    fn default() -> Self {
        Self {
            counts: SourceRefrigerationCounts::default(),
            get_input: true,
            have_racks: true,
            have_detailed: true,
            have_detailed_transcritical: true,
            have_cases_or_walk_ins: true,
            have_chillers: true,
            manage_refrigeration: true,
            use_system_time_step: false,
            zone_gains_environment_flag: true,
            present_in_zone: None,
        }
    }
}

fn count(input: &SelectedInputProcessor, family: &str) -> Result<i32, E> {
    i32::try_from(input.get_num_objects_found(family))
        .map_err(|_| unavailable("actual refrigeration object-count domain"))
}
fn positive(value: i32, message: &'static str) -> Result<(), E> {
    if value > 0 {
        return Err(unavailable(message));
    }
    Ok(())
}

impl SourceRefrigerationGainModule {
    fn get_refrigeration_input(
        &mut self,
        input: &mut SelectedInputProcessor,
        clock: &WeatherGlobalState,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(), E> {
        let c = &mut self.counts;
        // Original508-538: exact calls/assignments and the intervening sum.
        c.cascade_condensers = count(input, "Refrigeration:Condenser:Cascade")?;
        c.cases = count(input, "Refrigeration:Case")?;
        c.case_and_walk_in_lists = count(input, "Refrigeration:CaseAndWalkInList")?;
        c.racks = count(input, "Refrigeration:CompressorRack")?;
        c.secondary_systems = count(input, "Refrigeration:SecondarySystem")?;
        c.transfer_load_lists = count(input, "Refrigeration:TransferLoadList")?;
        c.walk_ins = count(input, "Refrigeration:WalkIn")?;
        c.systems = count(input, "Refrigeration:System")?;
        c.transcritical_systems = count(input, "Refrigeration:TranscriticalSystem")?;
        c.air_condensers = count(input, "Refrigeration:Condenser:AirCooled")?;
        c.evaporative_condensers = count(input, "Refrigeration:Condenser:EvaporativeCooled")?;
        c.water_condensers = count(input, "Refrigeration:Condenser:WaterCooled")?;
        c.gas_coolers = count(input, "Refrigeration:GasCooler:AirCooled")?;
        c.condensers = c
            .air_condensers
            .checked_add(c.evaporative_condensers)
            .and_then(|v| v.checked_add(c.water_condensers))
            .and_then(|v| v.checked_add(c.cascade_condensers))
            .ok_or_else(|| unavailable("actual refrigeration condenser-count sum"))?;
        c.compressors = count(input, "Refrigeration:Compressor")?;
        c.subcoolers = count(input, "Refrigeration:Subcooler")?;
        let compressor_lists = count(input, "Refrigeration:CompressorList")?;
        c.chiller_sets = count(input, "ZoneHVAC:RefrigerationChillerSet")?;
        c.air_chillers = count(input, "Refrigeration:AirChiller")?;
        if c.racks == 0 {
            self.have_racks = false;
        }
        if c.systems == 0 {
            self.have_detailed = false;
        }
        if c.transcritical_systems == 0 {
            self.have_detailed_transcritical = false;
        }
        if c.cases == 0 && c.walk_ins == 0 {
            self.have_cases_or_walk_ins = false;
        }
        if c.air_chillers == 0 {
            self.have_chillers = false;
        }
        // Stop at the first reached unported allocation, after all prior writes.
        if self.have_cases_or_walk_ins {
            return Err(unavailable("same RefrigCaseCredit allocation owner"));
        }
        positive(c.racks, "actual refrigeration rack allocation")?;
        positive(c.systems, "actual refrigeration system allocation")?;
        positive(
            c.transcritical_systems,
            "actual transcritical system allocation",
        )?;
        positive(
            c.chiller_sets,
            "actual refrigeration chiller-set allocation",
        )?;
        positive(c.condensers, "actual refrigeration condenser allocation")?;
        positive(c.gas_coolers, "actual refrigeration gas-cooler allocation")?;
        positive(c.cases, "actual refrigeration case allocation")?;
        positive(c.walk_ins, "actual refrigeration walk-in allocation")?;
        if c.walk_ins > 0 || c.cases > 0 {
            return Err(unavailable("actual CaseWIZoneReport allocation"));
        } else {
            self.use_system_time_step = true;
        }
        positive(c.secondary_systems, "actual secondary-system allocation")?;
        positive(
            c.air_chillers,
            "actual refrigeration air-chiller allocation",
        )?;
        positive(c.compressors, "actual refrigeration compressor allocation")?;
        positive(c.subcoolers, "actual refrigeration subcooler allocation")?;
        positive(
            c.case_and_walk_in_lists,
            "actual refrigeration case-list allocation",
        )?;
        positive(
            compressor_lists,
            "actual refrigeration compressor-list allocation",
        )?;
        positive(
            c.transfer_load_lists,
            "actual refrigeration transfer-list allocation",
        )?;

        let n = usize::try_from(clock.time_steps_in_hour)
            .map_err(|_| unavailable("actual refrigeration DayValues timestep extent"))?;
        let day_values = vec![
            None::<f64>;
            n.checked_mul(24).ok_or_else(|| unavailable(
                "actual refrigeration DayValues extent"
            ))?
        ];
        let zones = usize::try_from(globals.num_of_zones)
            .map_err(|_| unavailable("actual refrigeration NumOfZones extent"))?;
        self.present_in_zone = Some(vec![false; zones]);
        // Original628-655. These genuine queries insert absent-family nulls.
        let mut definitions = Vec::with_capacity(15);
        for family in [
            "Refrigeration:Case",
            "Refrigeration:CaseAndWalkInList",
            "Refrigeration:CompressorRack",
            "Refrigeration:System",
            "Refrigeration:TranscriticalSystem",
            "Refrigeration:Condenser:AirCooled",
            "Refrigeration:Condenser:EvaporativeCooled",
            "Refrigeration:Condenser:WaterCooled",
            "Refrigeration:GasCooler:AirCooled",
            "Refrigeration:Compressor",
            "Refrigeration:CompressorList",
            "Refrigeration:WalkIn",
            "Refrigeration:SecondarySystem",
            "ZoneHVAC:RefrigerationChillerSet",
            "Refrigeration:AirChiller",
        ] {
            definitions.push(input.get_object_def_max_args(family).map_err(|_| {
                unavailable("same refrigeration selected official schema definition")
            })?);
        }
        let max_alpha = definitions
            .iter()
            .map(|v| v.alphas)
            .max()
            .ok_or_else(|| unavailable("actual fifteen refrigeration alpha definitions"))?;
        let max_numeric = definitions
            .iter()
            .map(|v| v.numerics)
            .max()
            .ok_or_else(|| unavailable("actual fifteen refrigeration numeric definitions"))?;
        let alphas = vec![String::new(); max_alpha];
        let numbers = vec![0.0; max_numeric];
        let alpha_names = vec![String::new(); max_alpha];
        let numeric_names = vec![String::new(); max_numeric];
        let alpha_blanks = vec![true; max_alpha];
        let numeric_blanks = vec![true; max_numeric];
        // All actual positive getter blocks695-7007 have their same zero
        // controlling counts above. Source deallocation7009-7015 stays ordered.
        drop(day_values);
        drop(alphas);
        drop(numbers);
        drop(alpha_names);
        drop(numeric_names);
        drop(alpha_blanks);
        drop(numeric_blanks);
        // Orphan checks7017-7302 are likewise controlled by these zero counts.
        // ReportRefrigerationComponents14059-14790 emits no header or row:
        // each actual output branch is controlled by a zero module counter.
        self.report_components_no_devices()?;
        // Local ErrorsFound remains Sourcefalse on the admitted zero path.
        Ok(())
    }

    fn report_components_no_devices(&self) -> Result<(), E> {
        let c = &self.counts;
        for (value, message) in [
            (c.racks, "actual refrigeration rack EIO header"),
            (c.systems, "actual detailed refrigeration EIO header"),
            (
                c.secondary_systems,
                "actual secondary refrigeration EIO header",
            ),
            (c.chiller_sets, "actual chiller-set EIO header"),
            (c.cases, "actual refrigeration case EIO header"),
            (c.walk_ins, "actual refrigeration walk-in EIO header"),
            (c.air_condensers, "actual air-condenser EIO header"),
            (
                c.evaporative_condensers,
                "actual evaporative-condenser EIO header",
            ),
            (c.water_condensers, "actual water-condenser EIO header"),
            (c.cascade_condensers, "actual cascade-condenser EIO header"),
            (
                c.mechanical_subcoolers,
                "actual mechanical-subcooler EIO header",
            ),
            (
                c.subcoolers - c.mechanical_subcoolers,
                "actual liquid-suction-subcooler EIO header",
            ),
            (c.transcritical_systems, "actual transcritical EIO header"),
        ] {
            positive(value, message)?;
        }
        Ok(())
    }

    fn setup_report_input(&self, globals: &SelectedHeatBalanceGlobals) -> Result<(), E> {
        let c = &self.counts;
        positive(c.cases, "actual refrigeration case output registration")?;
        positive(
            c.walk_ins,
            "actual refrigeration walk-in output registration",
        )?;
        positive(
            c.air_chillers,
            "actual refrigeration air-chiller output registration",
        )?;
        let present = self
            .present_in_zone
            .as_deref()
            .ok_or_else(|| unavailable("same actual RefrigPresentInZone array"))?;
        // Actual7998 iterates every Source zone even when no cases exist.
        for zone in 1..=globals.num_of_zones {
            let at = usize::try_from(zone - 1)
                .map_err(|_| unavailable("actual refrigeration report Zone ordinal"))?;
            if *present
                .get(at)
                .ok_or_else(|| unavailable("actual RefrigPresentInZone domain"))?
            {
                if self.have_cases_or_walk_ins {
                    return Err(unavailable(
                        "actual refrigeration Zone case/walk-in output owner",
                    ));
                }
                if self.have_chillers {
                    return Err(unavailable(
                        "actual refrigeration Zone chiller output owner",
                    ));
                }
            }
        }
        positive(
            c.secondary_systems,
            "actual secondary refrigeration output registration",
        )?;
        positive(c.racks, "actual refrigeration rack output registration")?;
        positive(
            c.systems,
            "actual detailed refrigeration output registration",
        )?;
        positive(
            c.transcritical_systems,
            "actual transcritical output registration",
        )?;
        positive(c.gas_coolers, "actual gas-cooler output registration")?;
        Ok(())
    }

    fn check_input(
        &mut self,
        input: &mut SelectedInputProcessor,
        clock: &WeatherGlobalState,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(), E> {
        if self.get_input {
            self.get_refrigeration_input(input, clock, globals)?;
            self.setup_report_input(globals)?;
            self.get_input = false;
            if !self.have_cases_or_walk_ins && !self.have_chillers {
                self.manage_refrigeration = false;
                return Ok(());
            }
            if !self.have_detailed && !self.have_racks && !self.have_detailed_transcritical {
                self.manage_refrigeration = false;
                return Ok(());
            }
        }
        Ok(())
    }

    pub(super) fn figure_zone_gains(
        &mut self,
        input: &mut SelectedInputProcessor,
        clock: &WeatherGlobalState,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(), E> {
        self.check_input(input, clock, globals)?;
        // Source CheckRefrigerationInput's return does not return this caller.
        if clock.begin_envrn_flag && self.zone_gains_environment_flag {
            let c = &self.counts;
            positive(
                c.systems,
                "actual refrigeration system environment gain reset",
            )?;
            positive(
                c.transcritical_systems,
                "actual transcritical environment gain reset",
            )?;
            positive(c.racks, "actual refrigeration rack environment gain reset")?;
            positive(
                c.secondary_systems,
                "actual secondary refrigeration environment gain reset",
            )?;
            positive(
                c.walk_ins,
                "actual refrigeration walk-in environment gain reset",
            )?;
            positive(c.cases, "actual refrigeration case environment gain reset")?;
            self.zone_gains_environment_flag = false;
        }
        if !clock.begin_envrn_flag {
            self.zone_gains_environment_flag = true;
        }
        Ok(())
    }
}
