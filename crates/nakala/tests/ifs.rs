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

#[test]
fn if_true_with_else_doesnt_branch() {
    let input = "
        let x = 1

        if true {
            x = 2
        } else {
            x = 10000
        }

        x
    ";
    utils::compare_output(vec![], Some(input), "2");
}

#[test]
fn if_false_with_else_does_branch() {
    let input = "
        let x = 1

        if false {
            x = 2
        } else {
            x = 3
        }

        x
    ";
    utils::compare_output(vec![], Some(input), "3");
}

#[test]
fn if_false_with_else_if_true_does_branch() {
    let input = "
        let x = 1

        if false {
            x = 2
        } else if true{
            x = 3
        }

        x
    ";
    utils::compare_output(vec![], Some(input), "3");
}

#[test]
fn if_false_with_else_if_false_doesnt_branch() {
    let input = "
        let x = 1

        if false {
            x = 2
        } else if false{
            x = 3
        }

        x
    ";
    utils::compare_output(vec![], Some(input), "1");
}

#[test]
fn if_false_with_else_if_false_with_else_does_branch() {
    let input = "
        let x = 1

        if false {
            x = 2
        } else if false{
            x = 3
        } else {
            x = 4
        }

        x
    ";
    utils::compare_output(vec![], Some(input), "4");
}

#[test]
fn if_false_with_else_if_true_with_else_branches_in_the_middle() {
    let input = "
        let x = 1

        if false {
            x = 2
        } else if true {
            x = 3
        } else {
            x = 4
        }

        x
    ";
    utils::compare_output(vec![], Some(input), "3");
}
