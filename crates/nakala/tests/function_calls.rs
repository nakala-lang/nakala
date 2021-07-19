mod utils;

#[test]
fn simple_call() {
    utils::compare_output(vec![], Some("fn get_const() { 5 }  call get_const()"), "5");
}

#[test]
fn single_param_call() {
    utils::compare_output(
        vec![],
        Some(r#"fn get_const(x) { x } call get_const("hello world")"#),
        r#"hello world"#,
    );
}

#[test]
fn multi_param_call() {
    utils::compare_output(
        vec![],
        Some("fn add_three_numbers(x,y,z) { x + y + z }   call add_three_numbers(1,2,3)"),
        "6",
    );
}

#[test]
fn assign_var_to_call_result() {
    utils::compare_output(
        vec![],
        Some("fn add(x,y) { x + y }     let sum = call add(50, -20)     sum"),
        "30",
    );
}

#[test]
fn use_expr_in_call_param() {
    utils::compare_output(
        vec![],
        Some("fn sub(x,y) { x - y }    let result = call sub({let z = 100   let y = 20     z *  y}, 14)      result"),
        "1986"
    );
}
