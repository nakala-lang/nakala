mod utils;

#[test]
fn simple_def() {
    utils::compare_output(vec![], Some("fn nothing() {}"), "");
}

#[test]
fn print_def() {
    utils::compare_output(vec![], Some("fn print(x) { x }"), "");
}

#[test]
fn multiple_param_def() {
    utils::compare_output(
        vec![],
        Some(
            "fn longFunction(x,     y,     z, someOtherVariable) { x + y + z + someOtherVariable }",
        ),
        "",
    );
}
