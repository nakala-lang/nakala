mod utils;

#[test]
fn simple_block() {
    utils::compare_output(vec![], Some("{ 1 + 2 }"), "3");
}

#[test]
fn multiple_statement_block() {
    utils::compare_output(vec![], Some("{ let x = 5  x }"), "5");
}

#[test]
fn double_def_block() {
    utils::compare_output(vec![], Some("{ let x = 5   let y = 50   x*y }"), "250");
}

#[test]
fn long_block_with_comments() {
    let input = "
        let x = {
            # this is a comment
            let z = 100

            # this is another comment
            let y = z * 100

            # return y
            y
        }

        x
    ";
    utils::compare_output(vec![], Some(input), "10000");
}

#[test]
fn block_with_ref_outside_block() {
    let input = "
        let x = 100

        let y = {
            
            # comments
            let z = x * 5

            z
        }

        y
    ";

    utils::compare_output(vec![], Some(input), "500");
}

#[test]
fn variable_assignment_inside_block_propagates_outside() {
    let input = "
        let x = true

        {
            let y = -504

            x = y > 0

        }

        x
    ";

    utils::compare_output(vec![], Some(input), "false")
}

#[test]
fn variable_assignment_inside_nested_block_propagates_outside() {
    let input = "
        let x = 1
        
        {
            let y = 2

            {
                let z = 3

                y = z
            }

            x = y
        }

        x
    ";

    utils::compare_output(vec![], Some(input), "3");
}

#[test]
fn block_scope_should_not_leak_var_defs_outside() {
    let input = "
        let x = true

        {
            let x = false
        }

        x
    ";

    utils::compare_output(vec![], Some(input), "true");
}
