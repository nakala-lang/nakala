mod utils;

#[test]
fn simple_not() {
    utils::compare_output(vec![], Some("not false"), "true");
}

#[test]
fn simple_or() {
    utils::compare_output(vec![], Some("false or true"), "true");
}

#[test]
fn simple_and() {
    utils::compare_output(vec![], Some("false and true"), "false");
}

#[test]
fn simple_eq() {
    utils::compare_output(vec![], Some("5 == 5"), "true");
    utils::compare_output(vec![], Some(r#""test" == "test""#), "true");
    utils::compare_output(vec![], Some("true == not false"), "true");
}

#[test]
fn greater_than() {
    utils::compare_output(vec![], Some("5 > 0"), "true");
    utils::compare_output(vec![], Some("5 > 10"), "false");
    utils::compare_output(vec![], Some("0 >= 0"), "true");
    utils::compare_output(vec![], Some("0 >= 1"), "false");
}

#[test]
fn less_than() {
    utils::compare_output(vec![], Some("5 < 0"), "false");
    utils::compare_output(vec![], Some("5 < 10"), "true");
    utils::compare_output(vec![], Some("0 <= 0"), "true");
    utils::compare_output(vec![], Some("0 <= 1"), "true");
}
