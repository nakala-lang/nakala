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

#[test]
fn fib_recursive_works() {
    utils::compare_output(
        vec![],
        Some(
            "
        fn fib(x) {
            if x <= 1 {
                ret x
            } else {
                ret call fib(x - 1) + call fib(x - 2)
            }
        }

        call fib(15)",
        ),
        "610",
    )
}

#[test]
fn func_propagates_changes_to_outside_env() {
    utils::compare_output(
        vec![],
        Some(
            "
        let global_counter = 0

        fn change_global() {
            global_counter = 1
        }

        call change_global()

        global_counter",
        ),
        "1",
    );
}

#[test]
fn recursive_func_propagates_changes_to_outside_env() {
    let input = "
        let global_counter = 0

        fn rec_change_global(x) {
            if x < 1 {
                ret 0
            } else {
                global_counter = global_counter + 1
                call rec_change_global(x - 1)
            }
        }

        call rec_change_global(5)

        global_counter";

    utils::compare_output(vec![], Some(input), "5");
}
