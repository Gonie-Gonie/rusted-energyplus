//! Actual zero-object getter prefixes/returns, including shared scratch reads and local deallocations.
use super::state::*;
use super::{Error, SourceHvacOwners};
fn zone_count(o: &SourceHvacOwners<'_, '_>) -> Result<usize, Error> {
    usize::try_from(o.globals.num_of_zones)
        .map_err(|_| Error::Unavailable("actual airflow-system NumOfZones allocation"))
}
impl SourceEarthTube {
    pub(super) fn manage(&mut self, o: &mut SourceHvacOwners<'_, '_>) -> Result<(), Error> {
        if self.get_input_flag {
            let _errors_found = false;
            self.get_input(o)?;
            self.get_input_flag = false;
        }
        let empty = match &self.systems {
            None => true,
            Some(rows) => rows.is_empty(),
        };
        if empty {
            return Ok(());
        }
        Err(Error::Unavailable(
            "actual positive initEarthTubeVertical129",
        ))
    }
    fn get_input(&mut self, o: &mut SourceHvacOwners<'_, '_>) -> Result<(), Error> {
        let _report_variable_set = vec![true; zone_count(o)?];
        self.zone_reports = Some(vec![SourceEarthTubeZoneReport::default(); zone_count(o)?]);
        o.shortcut.current_module_object = "ZoneEarthtube:Parameters".into();
        let parameters = o
            .input
            .get_num_objects_found(&o.shortcut.current_module_object);
        if parameters > 0 {
            return Err(Error::Unavailable(
                "actual positive EarthTubePars allocation170",
            ));
        }
        self.parameters = Some(Vec::new());
        o.shortcut.current_module_object = "ZoneEarthtube".into();
        self.total_earth_tubes = o
            .input
            .get_num_objects_found(&o.shortcut.current_module_object);
        if self.total_earth_tubes > 0 {
            return Err(Error::Unavailable(
                "actual positive EarthTubeSys allocation210",
            ));
        }
        self.systems = Some(Vec::new());
        // Actual unconditional CheckEarthTubesInZones arguments574 are still evaluated
        // on zero input; shared cAlphaArgs is not reconstructed or cleared here.
        let _zone_name = o.shortcut.alpha_args.first().ok_or(Error::Unavailable(
            "actual CheckEarthTubes cAlphaArgs(1)574",
        ))?;
        let _field_name = o.shortcut.current_module_object.as_str();
        let number = self
            .systems
            .as_ref()
            .ok_or(Error::Unavailable("actual allocated EarthTubeSys size592"))?
            .len();
        if number > 1 {
            return Err(Error::Unavailable(
                "actual positive duplicate EarthTube Zone pair593",
            ));
        }
        Ok(())
    }
}
impl SourceCoolTower {
    pub(super) fn manage(&mut self, o: &mut SourceHvacOwners<'_, '_>) -> Result<(), Error> {
        if self.get_input_flag {
            self.get_input(o)?;
            self.get_input_flag = false;
        }
        let empty = match &self.systems {
            None => true,
            Some(rows) => rows.is_empty(),
        };
        if empty {
            return Ok(());
        }
        Err(Error::Unavailable("actual positive CalcCoolTower117"))
    }
    fn get_input(&mut self, o: &mut SourceHvacOwners<'_, '_>) -> Result<(), Error> {
        let _errors_found = false;
        // Native CurrentModuleObject is a static local, not the shared shortcut field.
        let current_module_object = "ZoneCoolTower:Shower";
        let counts = o
            .input
            .get_object_def_max_args(current_module_object)
            .map_err(|_| Error::Unavailable("actual ZoneCoolTower:Shower definition162"))?;
        let alphas = vec![String::new(); counts.alphas];
        let alpha_fields = vec![String::new(); counts.alphas];
        let numeric_fields = vec![String::new(); counts.numerics];
        let numbers = vec![0.0; counts.numerics];
        let alpha_blanks = vec![true; counts.alphas];
        let numeric_blanks = vec![true; counts.numerics];
        let number = o.input.get_num_objects_found(current_module_object);
        if number > 0 {
            return Err(Error::Unavailable(
                "actual positive CoolTowerSys allocation176",
            ));
        }
        self.systems = Some(Vec::new());
        // Genuine explicit local deallocate order385-390, before fatal/output loops.
        drop(alphas);
        drop(alpha_fields);
        drop(numeric_fields);
        drop(numbers);
        drop(alpha_blanks);
        drop(numeric_blanks);
        // Actual ErrorsFoundfalse final check and NumCoolTowers0 output loop have
        // no side effects; no output registration or WaterManager setup is fabricated.
        Ok(())
    }
}
impl SourceThermalChimney {
    pub(super) fn manage(&mut self, o: &mut SourceHvacOwners<'_, '_>) -> Result<(), Error> {
        if self.get_input_flag {
            let _errors_found = false;
            self.get_input(o)?;
            self.get_input_flag = false;
        }
        if self.total_chimneys == 0 {
            return Ok(());
        }
        Err(Error::Unavailable("actual positive CalcThermalChimney129"))
    }
    fn get_input(&mut self, o: &mut SourceHvacOwners<'_, '_>) -> Result<(), Error> {
        self.zone_reports = Some(vec![
            SourceThermalChimneyZoneReport::default();
            zone_count(o)?
        ]);
        o.shortcut.current_module_object = "ZoneThermalChimney".into();
        self.total_chimneys = o
            .input
            .get_num_objects_found(&o.shortcut.current_module_object);
        if self.total_chimneys > 0 {
            return Err(Error::Unavailable(
                "actual positive ThermalChimneySys allocation167",
            ));
        }
        self.systems = Some(Vec::new());
        self.reports = Some(Vec::new());
        // Actual TotThermalChimney>1 duplicate traversal and ErrorsFoundfalse
        // fatal check677 have no reached writes on this genuine zero-object route.
        Ok(())
    }
}
