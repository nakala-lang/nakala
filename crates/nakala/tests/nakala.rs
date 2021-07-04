mod utils;

#[test]
fn version_number_matches() {
    let version_str = format!("nakala {}", env!("CARGO_PKG_VERSION"));
    utils::compare_output(vec!["--version"], None, version_str.as_str());
}
