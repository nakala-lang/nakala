mod utils;

#[test]
fn simple_if() {
    utils::compare_output(vec![], Some("if true {}"), "")
}

#[test]
fn if_true_mutates_outside_variable() {
    let input = "
        let x = 100
        
        if true {
            x = 5
        }

        x
    ";

    utils::compare_output(vec![], Some(input), "5");
}

#[test]
fn if_false_doesnt_mutate_outside_variable() {
    let input = "
        let x = 100
        
        if false {
            x = 5
        }

        x
    ";

    utils::compare_output(vec![], Some(input), "100");
}

#[test]
fn complicated_if() {
    let input = "
        let x = 1

        if 1 >= (-100 * 5 + 3832930812 + { 100 * 123 }) { 
            x = 100
        }

        x
    ";
    utils::compare_output(vec![], Some(input), "1");
}
