//! Actual GetAirPathData185-411 zero-loop branch, with all reached allocations.
use super::*;
#[derive(Clone, Debug, Default)]
struct AirUniqueNode {
    #[allow(
        dead_code,
        reason = "SimAirServingZones.cc GetAirPathData keeps the original AirUniqueNodes scratch-row layout; selected zero-primary-loop allocation does not read these members."
    )]
    node_name: String,
    #[allow(
        dead_code,
        reason = "SimAirServingZones.cc GetAirPathData keeps the original AirUniqueNodes scratch-row layout; selected zero-primary-loop allocation does not read these members."
    )]
    air_loop_name: String,
    #[allow(
        dead_code,
        reason = "SimAirServingZones.cc GetAirPathData keeps the original AirUniqueNodes scratch-row layout; selected zero-primary-loop allocation does not read these members."
    )]
    field_name: String,
    #[allow(
        dead_code,
        reason = "SimAirServingZones.cc GetAirPathData keeps the original AirUniqueNodes scratch-row layout; selected zero-primary-loop allocation does not read these members."
    )]
    node_name_used: bool,
}
impl SourceAirLoopData {
    pub(super) fn get_source_air_path_data(
        &mut self,
        o: &mut SourceAirLoopOwners<'_>,
    ) -> Result<(), Error> {
        let _match_node_name: [String; 3] = std::array::from_fn(|_| String::new());
        let first = o
            .input
            .get_object_def_max_args("AirLoopHVAC")
            .map_err(|_| Error::Unavailable("actual AirLoopHVAC definition360"))?;
        let mut max_alphas = first.alphas;
        let mut max_numbers = first.numerics;
        let connector = o
            .input
            .get_object_def_max_args("ConnectorList")
            .map_err(|_| Error::Unavailable("actual ConnectorList definition361"))?;
        max_alphas = max_alphas.max(connector.alphas);
        max_numbers = max_numbers.max(connector.numerics);
        let controllers = o
            .input
            .get_object_def_max_args("AirLoopHVAC:ControllerList")
            .map_err(|_| Error::Unavailable("actual ControllerList definition364"))?;
        max_alphas = max_alphas.max(controllers.alphas);
        max_numbers = max_numbers.max(controllers.numerics);
        let mut numbers: Vec<Option<f64>> = vec![None; max_numbers];
        let mut numeric_fields = vec![String::new(); max_numbers];
        let mut numeric_blanks: Vec<Option<bool>> = vec![None; max_numbers];
        let mut alphas = vec![String::new(); max_alphas];
        let mut alpha_fields = vec![String::new(); max_alphas];
        let mut alpha_blanks: Vec<Option<bool>> = vec![None; max_alphas];
        numbers.fill(Some(0.0));
        numeric_fields.fill(String::new());
        numeric_blanks.fill(Some(true));
        alphas.fill(String::new());
        alpha_fields.fill(String::new());
        alpha_blanks.fill(Some(true));
        self.number_time_steps_in_day = o
            .clock
            .time_steps_in_hour
            .checked_mul(24)
            .ok_or(Error::Unavailable("defined actual NumOfTimeStepInDay383"))?;
        let nodes = o
            .input
            .get_object_def_max_args("NodeList")
            .map_err(|_| Error::Unavailable("actual NodeList definition385"))?;
        let node_nums = vec![
            0_i32;
            nodes
                .total()
                .map_err(|_| Error::Unavailable("actual NodeNums NumParams386"))?
        ];
        let count = o.input.get_num_objects_found("AirLoopHVAC");
        o.hvac.number_of_primary_air_systems = i32::try_from(count)
            .map_err(|_| Error::Unavailable("defined actual NumPrimaryAirSys389"))?;
        let test_unique_nodes = vec![
            AirUniqueNode::default();
            count.checked_mul(4).ok_or(Error::Unavailable(
                "defined actual TestUniqueNodes391 extent"
            ))?
        ];
        if count > 0 {
            return Err(Error::Unavailable(
                "positive actual PrimaryAirSystems constructor allocation393",
            ));
        }
        allocate_empty(&mut self.primary_air_systems);
        allocate_empty(&mut self.air_to_zone_node_info);
        allocate_empty(&mut self.air_loop_zone_info);
        allocate_empty(&mut self.air_to_oa_node_info);
        let _packaged_unit: Vec<Option<bool>> = Vec::new();
        allocate_empty(&mut self.control_info);
        allocate_empty(&mut self.flow);
        allocate_empty(&mut o.project.convergence.air_loop_convergence);
        o.sizing.unitary_system_equipment_sizing = Some(Vec::new());
        if *o.distribution_simulated {
            allocate_empty(&mut self.afn_info);
        }
        o.hvac.get_air_path_data_done = true;
        drop(test_unique_nodes);
        drop(node_nums);
        // Other local arrays are destroyed on this actual return. Shared shortcut
        // arguments are never reset or used by these definition-only calls.
        Ok(())
    }
}
