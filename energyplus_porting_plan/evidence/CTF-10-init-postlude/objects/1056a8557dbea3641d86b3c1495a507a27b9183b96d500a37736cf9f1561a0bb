use super::*;

type TestResult = Result<(), CtfReportUnavailable>;

#[test]
fn fresh_value_initialized_owner_has_all_named_members_and_no_allocations() -> TestResult {
    let owner = PredefinedRegistry::default();
    assert!(owner.reports.is_none() && owner.subtables.is_none() && owner.columns.is_none());
    assert_eq!(
        (owner.report_count, owner.subtable_count, owner.column_count),
        (0, 0, 0)
    );
    assert!(owner.named_indices.values().all(|value| *value == 0));
    for name in [
        "pdchAWHPDesSizeRefAirTemp",
        "pdchAWHPDesEnterWaterTemp",
        "pdchAWHPDesLeaveWaterTemp",
    ] {
        assert_eq!(owner.named_indices.get(name), Some(&0));
    }
    assert!(owner.opaque_layer_columns().is_empty());
    Ok(())
}

#[test]
fn first_allocation_resets_stale_counters_and_grows_with_retained_slots() -> TestResult {
    let mut owner = PredefinedRegistry {
        report_capacity: -9,
        report_count: 81,
        ..Default::default()
    };
    assert_eq!(owner.new_report("first", "F", "First")?, 1);
    assert_eq!(owner.report_capacity, 100);
    for _ in 1..100 {
        owner.new_report("next", "N", "Next")?;
    }
    owner
        .reports
        .as_mut()
        .ok_or(CtfReportUnavailable::TableOwner)?[0]
        .show = true;
    assert_eq!(owner.new_report("grown", "G", "Grown")?, 101);
    assert_eq!(owner.report_capacity, 200);
    let reports = owner
        .reports
        .as_ref()
        .ok_or(CtfReportUnavailable::TableOwner)?;
    assert_eq!(reports.len(), 200);
    assert!(reports[0].show);
    assert_eq!(reports[0].name, "first");
    assert_eq!(reports[100].name, "grown");
    assert_eq!(reports[199], PredefinedReportName::default());
    Ok(())
}

#[test]
fn helpers_write_only_source_members_in_existing_next_slots() -> TestResult {
    let mut owner = PredefinedRegistry::default();
    owner.new_report("first", "F", "First")?;
    owner
        .reports
        .as_mut()
        .ok_or(CtfReportUnavailable::TableOwner)?[1]
        .show = true;
    owner.new_report("second", "S", "Second")?;
    assert!(
        !owner
            .reports
            .as_ref()
            .ok_or(CtfReportUnavailable::TableOwner)?[1]
            .show
    );
    owner.new_subtable(2, "first subtable")?;
    owner
        .subtables
        .as_mut()
        .ok_or(CtfReportUnavailable::TableOwner)?[1]
        .footnote = "retained next-slot footnote".to_owned();
    assert_eq!(owner.new_subtable(1, "second subtable")?, 2);
    let subtable = &owner
        .subtables
        .as_ref()
        .ok_or(CtfReportUnavailable::TableOwner)?[1];
    assert_eq!(subtable.report_index, 1);
    assert_eq!(subtable.footnote, "retained next-slot footnote");
    assert_eq!(owner.new_column(2, "second subtable heading")?, 1);
    assert_eq!(
        owner
            .columns
            .as_ref()
            .ok_or(CtfReportUnavailable::TableOwner)?[0]
            .subtable_index,
        2
    );
    Ok(())
}

#[test]
fn full_registration_retains_real_parent_links_and_layer_column_ids() -> TestResult {
    let mut owner = PredefinedRegistry::default();
    owner.set_predefined_tables(false, false)?;
    let report = owner.named_indices["pdrEnvelope"];
    let subtable = owner.named_indices["pdstOpqConsLayers"];
    assert_eq!(
        owner
            .subtables
            .as_ref()
            .ok_or(CtfReportUnavailable::TableOwner)?
            [usize::try_from(subtable - 1).map_err(|_| CtfReportUnavailable::TableOwner)?]
        .report_index,
        report
    );
    let columns = owner
        .columns
        .as_ref()
        .ok_or(CtfReportUnavailable::TableOwner)?;
    assert_eq!(owner.opaque_layer_columns().len(), 10);
    for (offset, index) in owner.opaque_layer_columns().iter().enumerate() {
        let column =
            &columns[usize::try_from(*index - 1).map_err(|_| CtfReportUnavailable::TableOwner)?];
        assert_eq!(column.subtable_index, subtable);
        assert_eq!(column.heading, format!("Layer {}", offset + 1));
    }
    assert_eq!(owner.named_indices["pdrStd62"], 0);
    assert_eq!(
        owner
            .reports
            .as_ref()
            .ok_or(CtfReportUnavailable::TableOwner)?[0]
            .name,
        "ClimaticDataSummary"
    );
    Ok(())
}

#[test]
fn either_sizing_flag_enters_same_catalog_branch() -> TestResult {
    let mut zone = PredefinedRegistry::default();
    let mut system = zone.clone();
    let mut both = zone.clone();
    zone.set_predefined_tables(true, false)?;
    system.set_predefined_tables(false, true)?;
    both.set_predefined_tables(true, true)?;
    assert_eq!(zone, system);
    assert_eq!(zone, both);
    assert!(zone.named_indices["pdrStd62"] > 0);
    let subtable = zone.named_indices["pdstS62sysVentReqCool"];
    assert_eq!(
        zone.subtables
            .as_ref()
            .ok_or(CtfReportUnavailable::TableOwner)?
            [usize::try_from(subtable - 1).map_err(|_| CtfReportUnavailable::TableOwner)?]
        .report_index,
        zone.named_indices["pdrStd62"]
    );
    Ok(())
}

#[test]
fn repeated_set_appends_and_skipped_sizing_members_retain_previous_ids() -> TestResult {
    let mut owner = PredefinedRegistry::default();
    owner.set_predefined_tables(true, false)?;
    let std62 = owner.named_indices["pdrStd62"];
    let first_report_count = owner.report_count;
    let first_layers = owner.opaque_layer_columns().to_vec();
    owner
        .reports
        .as_mut()
        .ok_or(CtfReportUnavailable::TableOwner)?[0]
        .show = true;
    owner.set_predefined_tables(false, false)?;
    assert_eq!(owner.named_indices["pdrStd62"], std62);
    assert_eq!(owner.named_indices["pdrClim"], first_report_count + 1);
    assert!(
        owner
            .reports
            .as_ref()
            .ok_or(CtfReportUnavailable::TableOwner)?[0]
            .show
    );
    assert_eq!(&owner.opaque_layer_columns()[..10], first_layers.as_slice());
    assert!(owner.opaque_layer_columns()[10] > first_layers[9]);
    assert_eq!(owner.opaque_layer_columns().len(), 20);
    Ok(())
}

#[test]
fn footnote_range_guard_and_zero_representation_are_distinct() -> TestResult {
    let mut owner = PredefinedRegistry::default();
    owner.new_subtable(1, "actual")?;
    owner.add_footnote(1, "first")?;
    let before = owner.clone();
    owner.add_footnote(-1, "ignored")?;
    owner.add_footnote(2, "ignored")?;
    assert_eq!(owner, before);
    assert_eq!(
        owner.add_footnote(0, "unrepresentable"),
        Err(CtfReportUnavailable::TableOwner)
    );
    assert_eq!(owner, before);
    owner.add_footnote(1, "replaced")?;
    assert_eq!(
        owner
            .subtables
            .as_ref()
            .ok_or(CtfReportUnavailable::TableOwner)?[0]
            .footnote,
        "replaced"
    );
    Ok(())
}

#[test]
fn malformed_imported_owners_do_not_become_fresh_registry_success() -> TestResult {
    let mut owner = PredefinedRegistry::default();
    owner.named_indices.remove("pdrClim");
    let before = owner.clone();
    assert_eq!(
        owner.set_predefined_tables(false, false),
        Err(CtfReportUnavailable::TableOwner)
    );
    assert_eq!(owner, before);
    let mut owner = PredefinedRegistry {
        columns: Some(vec![PredefinedColumn::default()]),
        column_capacity: 2,
        ..Default::default()
    };
    assert_eq!(
        owner.new_column(1, "missing capacity slot"),
        Err(CtfReportUnavailable::TableOwner)
    );
    assert_eq!(owner.column_count, 0);
    Ok(())
}
