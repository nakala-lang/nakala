use parser::parse;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "-> ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse = parse(&input);
        println!("{}", parse.debug_tree());

        let mut ast_tree = ast::Root::cast(parse.syntax()).unwrap();
        ast_tree = dbg!(ast_tree);

        let hir_tree = hir::lower(ast_tree);
        dbg!(hir_tree);

        input.clear();
    }
}
