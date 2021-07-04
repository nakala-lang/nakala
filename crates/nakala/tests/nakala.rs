mod utils;

#[test]
fn version_number_matches() {
    let version_str = format!("nakala {}\n", env!("CARGO_PKG_VERSION"));
    utils::compare_output(vec!["--version"], version_str.as_str());
}
