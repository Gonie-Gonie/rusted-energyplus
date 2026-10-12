//! Extend the existing raw-admission regression at the shared runtime boundary.
use super::*;
use ep_raw_model::parse_epjson_str;
use ep_runtime::heat_balance::ctf_initialization_context::CtfInitializationReports;
use ep_runtime::heat_balance::selected_input_processor::{
    SelectedInputUnavailable, SourceArgumentCounts, SourceObjectDefinition,
};

pub(crate) fn verify_shared_input_boundary() -> Result<(), Box<dyn std::error::Error>> {
    let base = r#""Version":{"v":{"version_identifier":"26.1"}},"Building":{"b":{"solar_distribution":"MinimalShadowing"}},"Timestep":{"t":{"number_of_timesteps_per_hour":4}},"Zone":{"actual zone":{}}"#;
    let diagnostics = r#", "Output:Diagnostics":{"d":{"diagnostics":[{"key":"DisplayAdvancedReportVariables"},{"key":"DisplayAdvancedReportVariables"},{"key":"DisplayAdvancedReportVariables"},{"key":"DisplayAdvancedReportVariables"},{"key":"DisplayAdvancedReportVariables"}]}},"Schedule:Constant":{}"#;
    let raw = parse_epjson_str(&format!("{{{base}{diagnostics}}}"))?;
    let input = resolve(&raw)?;
    assert_eq!(input.get_num_objects_found("zone"), 1);
    assert_eq!(input.get_num_objects_found("Unknown:Count"), 0);
    assert!(input.family("Sizing:Zone").is_none());
    assert_eq!(input.get_num_objects_found("Sizing:Zone"), 0);
    assert!(
        input.family("Sizing:Zone").is_none(),
        "count must not insert"
    );
    assert_eq!(
        input.family("Schedule:Constant"),
        Some(&SourceInputFamily::Objects(vec![]))
    );
    let Some(SourceInputFamily::Objects(shapes)) = input.family("Output:Diagnostics") else {
        return Err("actual diagnostic family shapes missing".into());
    };
    assert_eq!(shapes[0].field_sizes["diagnostics"], 5);
    assert_eq!(shapes[0].object_name, "d");
    assert!(input.has_object("Output:Diagnostics", "d"));
    assert!(input.is_object_unused("Output:Diagnostics", "d"));
    assert!(!input.has_object("Output:Diagnostics", "D"));
    let initial_counts = SourceArgumentCounts {
        alphas: 5,
        numerics: 9,
    };
    assert_eq!(input.get_max_schema_args()?, initial_counts);

    let mut reports = CtfInitializationReports::default();
    reports.input_processor_shortcut.current_module_object = "retained caller".into();
    reports.initialize_input_processor(input)?;
    let scratch = &reports.input_processor_shortcut;
    assert_eq!(scratch.current_module_object, "retained caller");
    for array in [&scratch.alpha_args, &scratch.alpha_field_names] {
        assert_eq!(array.len(), initial_counts.alphas);
        assert!(array.iter().all(String::is_empty));
    }
    assert_eq!(scratch.numeric_args.len(), initial_counts.numerics);
    assert!(
        scratch
            .numeric_args
            .iter()
            .all(|value| value.to_bits() == 0)
    );
    assert_eq!(scratch.numeric_field_names.len(), initial_counts.numerics);
    assert!(scratch.numeric_field_names.iter().all(String::is_empty));
    assert_eq!(
        scratch.alpha_field_blanks,
        vec![false; initial_counts.alphas]
    );
    assert_eq!(
        scratch.numeric_field_blanks,
        vec![false; initial_counts.numerics]
    );

    let Some(input) = reports.selected_input_processor.as_mut() else {
        return Err("actual caller lost moved input".into());
    };
    let sizing_counts = SourceArgumentCounts {
        alphas: 16,
        numerics: 23,
    };
    assert_eq!(input.get_object_def_max_args("sIzInG:zOnE")?, sizing_counts);
    assert_eq!(input.family("Sizing:Zone"), Some(&SourceInputFamily::Null));
    assert_eq!(input.get_num_objects_found("Sizing:Zone"), 0);
    assert_eq!(input.get_max_schema_args()?, sizing_counts);
    assert_eq!(
        reports.input_processor_shortcut.alpha_args.len(),
        5,
        "later null insertion must not resize the already allocated shared arrays"
    );
    assert!(matches!(
        input.get_object_def_max_args("Unknown:Definition"),
        Err(SelectedInputUnavailable::Definition(_))
    ));
    assert!(input.family("Unknown:Definition").is_none());

    for addition in [r#", "Unrepresented:Empty":{}"#, r#", "Branch":{"b":{}}"#] {
        assert!(resolve(&parse_epjson_str(&format!("{{{base}{addition}}}"))?).is_err());
    }
    let previous_scratch = reports.input_processor_shortcut.clone();
    let oversized = SelectedInputProcessor::from_admitted_input(
        vec![SourceObjectDefinition {
            object_type: "oversized".into(),
            alpha_fields: usize::MAX,
            numeric_fields: 1,
            alpha_extensions: 0,
            numeric_extensions: 0,
            extension: None,
        }],
        BTreeMap::from([("oversized".into(), SourceInputFamily::Objects(vec![]))]),
    )?;
    assert_eq!(
        reports.initialize_input_processor(oversized),
        Err(SelectedInputUnavailable::Size)
    );
    assert_eq!(reports.input_processor_shortcut, previous_scratch);
    assert!(
        reports
            .selected_input_processor
            .as_ref()
            .is_some_and(|input| input.family("oversized").is_some()),
        "input must remain retained when allocation sizing stops"
    );
    Ok(())
}
