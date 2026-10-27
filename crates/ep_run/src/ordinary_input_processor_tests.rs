//! Extend the existing raw-admission regression at the shared runtime boundary.
use super::*;
use ep_raw_model::parse_epjson_str;
use ep_runtime::heat_balance::ctf_initialization_context::CtfInitializationReports;
use ep_runtime::heat_balance::error_backend::{
    SourceErrorLevel, SourceErrorState, SourceFatalStop,
};
use ep_runtime::heat_balance::selected_input_processor::{
    SelectedInputUnavailable, SourceArgumentCounts, SourceObjectDefinition,
};
use ep_runtime::heat_balance::source_output_prescan::SourcePreScanError;

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
    let hb = crate::ordinary_heat_balance_input::resolve(&raw, &input)?;
    assert!(hb.source_is_epjson);
    assert_eq!(hb.objects["Building"][0].object_name, "b");
    assert_eq!(hb.objects["Zone"][0].object_name, "actual zone");
    assert_eq!(hb.objects["Building"][0].idf_max_fields, None);
    assert!(hb.objects["Construction"].is_empty());
    let material = crate::ordinary_material_input::resolve(&raw, &input)?;
    assert!(material.source_is_epjson);
    assert!(material.regular.is_empty() && material.no_mass.is_empty());
    assert!(
        input.is_object_unused("Building", "b"),
        "raw custody must not masquerade as a reached getter"
    );
    assert!(input.is_object_unused("Zone", "actual zone"));
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
    // The same sole IP and pre-scan must agree on Native DOM order, including
    // numeric names whose BTree order differs. First inserted spelling is an
    // observable consequence, rather than only a comparison of two adapters.
    let ordered_raw = parse_epjson_str(&format!(
        r#"{{{base},"Output:Variable":{{
        "request10":{{"variable_name":"cASE vAR","key_value":"KEY","reporting_frequency":"Hourly"}},
        "request2":{{"variable_name":"Case Var [W]","key_value":"Key","reporting_frequency":"Hourly"}}
    }}}}"#
    ))?;
    let ordered_projection = resolve_reporting_prescan_source(&ordered_raw)?;
    assert_eq!(
        ordered_projection
            .variables
            .iter()
            .map(|row| row.object_name.as_str())
            .collect::<Vec<_>>(),
        ["request2", "request10"]
    );
    let mut outputs = crate::ctf_initialization_outputs::CtfRunOutputs::new("26.1");
    for input in [
        resolve_source_a(&ordered_raw)?,
        resolve_source_b(&ordered_raw)?,
    ] {
        let Some(SourceInputFamily::Objects(shapes)) = input.family("Output:Variable") else {
            return Err("Source DOM shapes missing".into());
        };
        assert_eq!(
            shapes
                .iter()
                .map(|row| row.object_name.as_str())
                .collect::<Vec<_>>(),
            ["request2", "request10"]
        );
        let mut prescan =
            ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
        prescan.pre_scan_reporting_variables(
            &input,
            &ordered_projection,
            &mut outputs.context().backend,
        )?;
        let first = &prescan.output_variables_for_simulation["case var"];
        assert_eq!(first.variable_name, "Case Var");
        assert_eq!(first.keys["key"].key, "Key");
        assert_eq!(prescan.num_considered_output_variables, 2);
        let mut mismatched = ordered_projection.clone();
        mismatched.variables.swap(0, 1);
        assert!(
            prescan
                .pre_scan_reporting_variables(&input, &mismatched, &mut outputs.context().backend)
                .is_err()
        );
    }
    let legacy = resolve_reporting_prescan(&ordered_raw)?;
    assert_eq!(legacy.variables[0].object_name, "request10");
    let legacy_input = resolve(&ordered_raw)?;
    let mut legacy_prescan =
        ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
    legacy_prescan.pre_scan_reporting_variables(
        &legacy_input,
        &legacy,
        &mut outputs.context().backend,
    )?;

    // Direct-JSON pre-scan keeps original spelling separately from getter alphas.
    let raw = parse_epjson_str(&format!(
        r#"{{{base},"Output:Variable":{{
        "z":{{"variable_name":"case var [W]","key_value":"key"}},
        "a":{{"variable_name":"CaSe VaR  ","key_value":"KeY"}},
        "b":{{"variable_name":"CASE VAR","key_value":"*"}},
        "c":{{"variable_name":"Tabbed\t  ","key_value":"  "}},
        "d":{{"variable_name":"Fallback","key_value":""}}
    }},"Output:Table:SummaryReports":{{"s":{{"reports":[{{"report_name":"AllSummary"}}]}}}}}}"#
    ))?;
    let projection = resolve_reporting_prescan(&raw)?;
    assert_eq!(
        projection
            .variables
            .iter()
            .map(|row| row.object_name.as_str())
            .collect::<Vec<_>>(),
        ["a", "b", "c", "d", "z"]
    );
    assert_eq!(projection.variables[0].variable_name, "CaSe VaR  ");
    assert_eq!(projection.variables[2].key, "  ");
    assert_eq!(projection.variables[4].variable_name, "case var [W]");
    let input = resolve(&raw)?;
    let unused = input.clone();
    let mut utility_events = Vec::new();
    let mut capture_error = |_: &mut SourceErrorState, level, bytes: &[u8]| {
        utility_events.push((level, bytes.to_vec()));
    };
    let mut context = outputs.context();
    context.backend.error_callback = Some(&mut capture_error);
    context.reports.initialize_input_processor(input)?;
    context.reports.reporting_prescan_input = Some(projection.clone());
    context.backend.state.doing_input_processing = true;
    let scratch = context.reports.input_processor_shortcut.clone();
    context.post_input_processing()?;
    assert!(!context.backend.state.doing_input_processing);
    let mut expected_scratch = scratch;
    expected_scratch.current_module_object = "Output:PreprocessorMessage".into();
    assert_eq!(context.reports.input_processor_shortcut, expected_scratch);
    assert_eq!(
        context.reports.selected_input_processor.as_ref(),
        Some(&unused)
    );
    let prescan = &context.reports.output_prescan;
    assert_eq!(prescan.max_considered_output_variables, 10000);
    assert_eq!(prescan.num_considered_output_variables, 5);
    assert_eq!(prescan.output_variables_for_simulation.len(), 3);
    let keys = &prescan.output_variables_for_simulation["case var"];
    assert_eq!(keys.variable_name, "CaSe VaR");
    assert_eq!(keys.keys.len(), 2);
    assert_eq!(keys.keys["key"].key, "KeY");
    assert_eq!(keys.keys["key"].variable_name, "CaSe VaR");
    assert_eq!(keys.keys["*"].variable_name, "CASE VAR");
    assert!(prescan.find_item("unlisted key", "CASE VAR"));
    assert!(prescan.find_item("  ", "tabbed\t"));
    assert!(!prescan.find_item("", "tabbed\t"));
    assert!(!prescan.find_item("  ", "tabbed"));
    assert!(prescan.find_item("anything", "fallback"));
    assert!(!prescan.find_item("anything", "case var [W]"));
    context.post_input_processing()?;
    assert_eq!(
        context
            .reports
            .output_prescan
            .num_considered_output_variables,
        10
    );
    // This formerly unsupported key is valid; both raw and CI matching work.
    context.reports.output_prescan.add_record(
        "bad.regex",
        "Regex retained",
        &mut context.backend,
    )?;
    assert!(
        context
            .reports
            .output_prescan
            .find_item("badXregex", "REGEX RETAINED")
    );
    assert!(
        context
            .reports
            .output_prescan
            .find_item("BADxREGEX", "regex retained")
    );
    assert!(
        !context
            .reports
            .output_prescan
            .find_item("bad/regex/", "Regex retained")
    );
    context.reports.output_prescan.add_record(
        "BAD.REGEX",
        "REGEX RETAINED",
        &mut context.backend,
    )?;
    let regex = &context
        .reports
        .output_prescan
        .output_variables_for_simulation["regex retained"];
    assert_eq!(regex.keys.len(), 1);
    let first = &regex.keys["bad.regex"];
    assert_eq!(first.key, "bad.regex");
    assert_eq!(
        first.pattern.as_ref().map(|pattern| pattern.pattern()),
        Some("bad.regex")
    );
    assert_eq!(
        first
            .case_insensitive_pattern
            .as_ref()
            .map(|pattern| pattern.pattern()),
        Some("(?i)bad.regex")
    );
    context
        .reports
        .output_prescan
        .add_record("[ab]", "Class retained", &mut context.backend)?;
    // DataOutputs' exact literal escape precedes its actual regex comparisons.
    assert!(
        context
            .reports
            .output_prescan
            .find_item("[ab]", "Class retained")
    );
    assert!(
        context
            .reports
            .output_prescan
            .find_item("A", "Class retained")
    );
    assert!(
        !context
            .reports
            .output_prescan
            .find_item("z", "Class retained")
    );
    assert_eq!(
        context
            .reports
            .output_prescan
            .num_considered_output_variables,
        13
    );
    let retained = context.reports.output_prescan.clone();
    let severe_before = context.backend.state.tracking.total_severe_errors;
    assert!(matches!(
        context
            .reports
            .output_prescan
            .add_record("[", "unreached", &mut context.backend),
        Err(SourcePreScanError::Fatal(SourceFatalStop::Fatal(_)))
    ));
    assert_eq!(context.reports.output_prescan, retained);
    assert_eq!(
        context.backend.state.tracking.total_severe_errors,
        severe_before + 1
    );
    let mut bad_projection = projection;
    bad_projection.variables[1].key = "[".into();
    context.reports.reporting_prescan_input = Some(bad_projection);
    context.backend.state.doing_input_processing = true;
    assert!(matches!(
        context.post_input_processing(),
        Err(SourcePreScanError::Fatal(SourceFatalStop::Fatal(_)))
    ));
    assert_eq!(
        context
            .reports
            .output_prescan
            .num_considered_output_variables,
        14
    );
    assert_eq!(
        context
            .reports
            .output_prescan
            .max_considered_output_variables,
        10000
    );
    assert_eq!(
        context
            .reports
            .input_processor_shortcut
            .current_module_object,
        "Output:PreprocessorMessage"
    );
    assert!(!context.backend.state.doing_input_processing);
    assert_eq!(
        context.backend.state.tracking.total_severe_errors,
        severe_before + 2
    );
    assert_eq!(
        context.backend.state.tracking.last_severe_error,
        r#"Regular expression "[" for variable name "CASE VAR" in input file is incorrect"#
    );
    assert!(
        context
            .reports
            .output_prescan
            .find_item("BADxREGEX", "Regex retained")
    );
    assert!(!context.reports.output_prescan.find_item("*", "unreached"));
    assert_eq!(
        context.reports.selected_input_processor.as_ref(),
        Some(&unused)
    );
    assert_eq!(
        utility_events
            .iter()
            .map(|(level, _)| *level)
            .collect::<Vec<_>>(),
        [
            SourceErrorLevel::Severe,
            SourceErrorLevel::Continue,
            SourceErrorLevel::Fatal,
            SourceErrorLevel::Severe,
            SourceErrorLevel::Continue,
            SourceErrorLevel::Fatal
        ]
    );
    assert_eq!(utility_events[1].1.as_slice(), b"missing ]: [");
    assert_eq!(utility_events[4].1.as_slice(), b"missing ]: [");
    assert_eq!(
        utility_events[0].1.as_slice(),
        br#"Regular expression "[" for variable name "unreached" in input file is incorrect"#
    );
    assert_eq!(
        utility_events[2].1.as_slice(),
        b"Error found in regular expression. Previous error(s) cause program termination."
    );
    assert_eq!(utility_events[2], utility_events[5]);
    Ok(())
}
