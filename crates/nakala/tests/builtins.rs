mod utils;

#[test]
fn print_works() {
    utils::compare_output(vec![], Some("call print(true)"), "true");
    utils::compare_output(vec![], Some("let x = 123 call print(123)"), "123");
    utils::compare_output(
        vec![],
        Some(r#"call print("hello world!")"#),
        "hello world!",
    );
}

#[test]
fn println_works() {
    utils::compare_output(vec![], Some("call println(true)"), "true\n");
    utils::compare_output(vec![], Some("let x = 123 call println(x)"), "123\n");
}

#[test]
fn len_works_with_strings() {
    utils::compare_output(vec![], Some(r#"call len("hello world!")"#), "12");
}

#[test]
fn len_works_with_lists() {
    utils::compare_output(vec![], Some("call len([1,2,3])"), "3");
}
