mod utils;

#[test]
fn simple_def() {
    utils::compare_output(vec![], Some("let x = 5"), "");
}

#[test]
fn double_def() {
    utils::compare_output(vec![], Some("let x = 10 let z = 5"), "");
}

#[test]
fn def_uses_ref() {
    utils::compare_output(vec![], Some("let x = 5   let y = x + 5   y"), "10");
}
