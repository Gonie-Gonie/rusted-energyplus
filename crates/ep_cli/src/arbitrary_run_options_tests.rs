use super::parse_run_options;
use ep_run::PortingScope;

fn args(extra: &[&str]) -> Vec<String> {
    ["model.epJSON", "--output-dir", "out"]
        .into_iter()
        .chain(extra.iter().copied())
        .map(str::to_owned)
        .collect()
}

#[test]
fn bounded_scope_is_preserved_separately_from_run_config() {
    for (token, expected) in [("A", PortingScope::A), ("B", PortingScope::B)] {
        let (config, scope) = parse_run_options(&args(&["--porting-scope", token, "--dry-run"]))
            .expect("valid bounded options");
        assert_eq!(scope, Some(expected));
        assert!(config.dry_run);
        assert!(!config.oracle_baseline);
        assert!(!config.compare_oracle);
    }
    let (_, scope) = parse_run_options(&args(&[])).expect("ordinary run options");
    assert_eq!(scope, None);
}

#[test]
fn bounded_scope_rejects_missing_invalid_and_repeated_selection() {
    for (extra, message) in [
        (vec!["--porting-scope"], "missing value for --porting-scope"),
        (
            vec!["--porting-scope", "C"],
            "unsupported porting scope: C; expected A or B",
        ),
        (
            vec!["--porting-scope", "A", "--porting-scope", "B"],
            "--porting-scope may be specified once",
        ),
    ] {
        let error = parse_run_options(&args(&extra)).expect_err("invalid bounded selection");
        assert_eq!(error.message, message);
    }
}

#[test]
fn a_flag_shaped_path_is_not_interpreted_as_scope_selection() {
    let (config, scope) = parse_run_options(&args(&["--weather", "--porting-scope"]))
        .expect("weather path consumed by its option");
    assert_eq!(scope, None);
    assert_eq!(
        config.weather_path,
        Some(std::path::PathBuf::from("--porting-scope"))
    );
}
