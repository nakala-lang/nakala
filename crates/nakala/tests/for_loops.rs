mod utils;

#[test]
fn simple_list_loop_works() {
    let input = "
        let x = 100
        for item in [1,2,3] {
            x = x + item
        }

        call print(x)
    ";
    utils::compare_output(vec![], Some(input), "106");
}

#[test]
fn simple_string_loop_works() {
    let input = r#"
        for char in "hello" {
            call println(char)
        }
        "#;
    utils::compare_output(vec![], Some(input), "h\ne\nl\nl\no\n");
}

#[test]
fn duplicate_loops_work_correclty() {
    let input = "
        let iters = 0

        for item in [1,2,3] {
            iters = iters + 1
        }

        for item in [1,2,3] {
            iters = iters + 1
        }

        call print(iters)
    ";
    utils::compare_output(vec![], Some(input), "6");
}
