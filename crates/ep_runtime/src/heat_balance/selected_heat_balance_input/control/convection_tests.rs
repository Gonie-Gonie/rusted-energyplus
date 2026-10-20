//! Function-entry inputs over the real raw/IP/shortcut/control owners.
use super::*;
use crate::heat_balance::selected_input_processor::{
    SourceInputFamily, SourceObjectDefinition, SourceObjectShape,
};
use crate::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue};
use std::collections::BTreeMap;

#[test]
fn source_convection_getter_preserves_enums_defaults_ip_identity_and_eio()
-> Result<(), Box<dyn std::error::Error>> {
    use InsideSurfaceConvectionAlgorithm as I;
    use OutsideSurfaceConvectionAlgorithm as O;
    // None=absent family, Some(None)=present instance with omitted algorithm.
    for (inside, outside, expected_inside, expected_outside, labels) in [
        (None, None, I::Tarp, O::Doe2, ("TARP", "DOE-2")),
        (
            Some(Some("CeilingDiffuser")),
            Some(Some("DOE-2")),
            I::CeilingDiffuser,
            O::Doe2,
            ("CeilingDiffuser", "DOE-2"),
        ),
        (Some(None), Some(None), I::Tarp, O::Doe2, ("TARP", "DOE-2")),
        (
            Some(Some("")),
            Some(Some("")),
            I::Tarp,
            O::Doe2,
            ("TARP", "DOE-2"),
        ),
        (
            Some(Some("simple")),
            Some(Some("SimpleCombined")),
            I::Simple,
            O::SimpleCombined,
            ("Simple", "SimpleCombined"),
        ),
        (
            Some(Some("TARP")),
            Some(Some("tarp")),
            I::Tarp,
            O::Tarp,
            ("TARP", "TARP"),
        ),
        (
            Some(Some("Simple")),
            Some(Some("MoWiTT")),
            I::Simple,
            O::MoWitt,
            ("Simple", "MoWiTT"),
        ),
        (
            Some(Some("TARP")),
            Some(Some("DOE-2")),
            I::Tarp,
            O::Doe2,
            ("TARP", "DOE-2"),
        ),
    ] {
        let mut raw = SelectedHeatBalanceInput {
            source_is_epjson: true,
            ..Default::default()
        };
        let mut families = BTreeMap::new();
        let definitions = [
            "SurfaceConvectionAlgorithm:Inside",
            "SurfaceConvectionAlgorithm:Outside",
        ]
        .into_iter()
        .map(|family| SourceObjectDefinition {
            object_type: family.into(),
            alpha_fields: 1,
            numeric_fields: 0,
            alpha_extensions: 0,
            numeric_extensions: 0,
            extension: None,
        })
        .collect();
        for (family, authored) in [
            ("SurfaceConvectionAlgorithm:Inside", inside),
            ("SurfaceConvectionAlgorithm:Outside", outside),
        ] {
            if let Some(value) = authored {
                let fields = value
                    .map(|value| {
                        BTreeMap::from([(
                            "algorithm".into(),
                            ProjectInputValue::String(value.into()),
                        )])
                    })
                    .unwrap_or_default();
                let sizes = value
                    .map(|_| BTreeMap::from([("algorithm".into(), 1)]))
                    .unwrap_or_default();
                raw.objects.insert(
                    family.into(),
                    vec![ProjectInputObject {
                        object_name: " ".into(),
                        fields,
                        idf_max_fields: None,
                    }],
                );
                families.insert(
                    family.into(),
                    SourceInputFamily::Objects(vec![SourceObjectShape {
                        object_name: " ".into(),
                        field_sizes: sizes,
                    }]),
                );
            }
        }
        let mut input =
            super::super::SelectedInputProcessor::from_admitted_input(definitions, families)?;
        let mut shortcut = super::super::SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input)?;
        let mut control = SelectedHeatBalanceInputState::default();
        let mut current = String::new();
        let mut eio = Vec::new();
        control.get_source_surface_convection_algorithms(
            &raw,
            &mut input,
            &mut shortcut,
            &mut current,
            &mut eio,
        )?;
        assert_eq!(
            (
                control.inside_convection_algorithm,
                control.outside_convection_algorithm
            ),
            (expected_inside, expected_outside)
        );
        assert_eq!(current, "SurfaceConvectionAlgorithm:Outside");
        assert!(input.source_unused_objects().next().is_none());
        assert_eq!(
            String::from_utf8(eio)?,
            format!(
                "! <Inside Convection Algorithm>, Algorithm {{Simple | TARP | CeilingDiffuser | AdaptiveConvectionAlgorithm}}\nInside Convection Algorithm,{}\n! <Outside Convection Algorithm>, Algorithm {{SimpleCombined | TARP | MoWitt | DOE-2 | AdaptiveConvectionAlgorithm}}\nOutside Convection Algorithm,{}\n",
                labels.0, labels.1
            )
        );
    }
    Ok(())
}
