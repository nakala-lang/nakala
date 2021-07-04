mod utils;

#[test]
fn simple_add() {
    utils::compare_output(vec![], Some("1 + 2"), "3");
}

#[test]
fn simple_sub() {
    utils::compare_output(vec![], Some("2 - 1"), "1");
}

#[test]
fn simple_mul() {
    utils::compare_output(vec![], Some("2 * 3"), "6");
}

#[test]
fn simple_div() {
    utils::compare_output(vec![], Some("6 / 3"), "2");
}

#[test]
fn simple_unary() {
    utils::compare_output(vec![], Some("-5"), "-5");
}

#[test]
fn simple_paren() {
    utils::compare_output(vec![], Some("(2 + 3) * 10"), "50");
}

#[test]
fn complex_expression() {
    utils::compare_output(vec![], Some("(100 * (5 * -4) + 100) / 5"), "-380");
}
