//! Regular Construction argument/identity operations, reusing existing layer properties.
use super::super::ctf_initialization_context::{original_loader_name, original_material_name};
use super::{
    HeatBalanceInputError, SelectedHeatBalanceInput, SelectedHeatBalanceInputState,
    SelectedInputProcessor, SourceErrorBackend, SourceInputProcessorShortCut,
};
use ep_model::{MaterialKind, TypedModel};

impl SelectedHeatBalanceInputState {
    pub(crate) fn read_regular_construction_arguments(
        &mut self,
        raw: &SelectedHeatBalanceInput,
        model: &TypedModel,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        current_module_object: &mut String,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), HeatBalanceInputError> {
        // Original1341 returns before ANY count or scratch operation.
        if !self.unique_construction_names.is_empty() {
            return Ok(());
        }
        let regular = input.get_num_objects_found("Construction");
        for family in [
            "Construction:AirBoundary",
            "Construction:FfactorGroundFloor",
            "Construction:CfactorUndergroundWall",
            "Construction:ComplexFenestrationState",
            "Construction:WindowDataFile",
            "Construction:WindowEquivalentLayer",
        ] {
            if input.get_num_objects_found(family) != 0 {
                return Err(HeatBalanceInputError::Unavailable(family));
            }
        }
        if i32::try_from(regular).is_err() {
            return Err(HeatBalanceInputError::Unavailable(
                "original construction count",
            ));
        }
        self.total_constructions = regular;
        self.construction_usage = vec![super::SelectedConstructionUsage::default(); regular];
        shortcut.current_module_object = "Construction".into();
        for index in 0..regular {
            // Original local ConstructAlphas lower bound is zero. Our [0] is
            // that same Name slot; material layer1 follows it. DummyProps has5.
            let values = raw.get_object_item::<11, 5>("Construction", index, input, shortcut)?;
            if !self
                .unique_construction_names
                .insert(values.alphas[0].clone())
            {
                return Err(HeatBalanceInputError::Unavailable(
                    "unadmitted duplicate construction",
                ));
            }
            let mut matches = model.constructions.iter().filter(|c| {
                original_loader_name(&model.ctf_report_names.constructions, c.id, &c.name)
                    .is_ok_and(|name| name.to_ascii_uppercase() == values.alphas[0])
            });
            let construction = matches.next().ok_or(HeatBalanceInputError::Unavailable(
                "actual typed construction identity",
            ))?;
            if matches.next().is_some()
                || !construction.is_ordinary_opaque()
                || construction.has_internal_heat_source()
            {
                return Err(HeatBalanceInputError::Unavailable(
                    "unadmitted construction owner",
                ));
            }
            self.construction_order.push(construction.id);
            self.construction_usage[index].construction_id = Some(construction.id);
            let layers =
                values
                    .num_alphas
                    .checked_sub(1)
                    .ok_or(HeatBalanceInputError::Unavailable(
                        "Construction Name argument",
                    ))?;
            if layers != construction.effective_layers().len() || layers == 0 {
                return Err(HeatBalanceInputError::Unavailable(
                    "actual construction layer argument count",
                ));
            }
            for (layer, id) in construction.effective_layers().iter().enumerate() {
                if original_material_name(model, *id)
                    .map_err(|_| {
                        HeatBalanceInputError::Unavailable("actual material loader identity")
                    })?
                    .to_ascii_uppercase()
                    != values.alphas[layer + 1]
                {
                    return Err(HeatBalanceInputError::Unavailable(
                        "actual construction material association",
                    ));
                }
                let material = model.materials.iter().find(|m| m.id == *id).ok_or(
                    HeatBalanceInputError::Unavailable("actual construction material"),
                )?;
                match material.kind() {
                    MaterialKind::Mass => self.no_regular_materials_used = false,
                    MaterialKind::NoMass => {}
                    _ => {
                        return Err(HeatBalanceInputError::Unavailable(
                            "unadmitted construction material family",
                        ));
                    }
                }
                // NominalR and CheckAndSetConstructionProperties are provided
                // by the existing material/property producer. Do not compute
                // a second copy here or claim its partial failure lifecycle.
            }
        }
        *current_module_object = "ConstructionProperty:InternalHeatSource".into();
        if input
            .family("ConstructionProperty:InternalHeatSource")
            .is_some()
        {
            self.any_internal_heat_source_in_input = true;
            if input.get_num_objects_found("ConstructionProperty:InternalHeatSource") != 0 {
                return Err(HeatBalanceInputError::Unavailable(
                    "positive internal heat-source construction",
                ));
            }
        }
        if self.total_constructions > 0 && self.no_regular_materials_used {
            backend.show_warning_error(
                "This building has no thermal mass which can cause an unstable solution.",
            )?;
            backend.show_continue_error("Use Material object for all opaque material definitions except very light insulation layers.")?;
        }
        *current_module_object = "Construction:WindowEquivalentLayer".into();
        *current_module_object = "Construction:WindowDataFile".into();
        // Both actual window loops are empty; the temporary WConstructNames
        // allocation is deallocated here. Existing physical properties follow.
        Ok(())
    }
}
