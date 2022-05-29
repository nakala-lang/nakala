use interpreter::{env::Env, interpret};
use miette::Result;
use parser::{parse, source::Source, SymbolTable};
use reedline::{DefaultPrompt, Reedline, Signal};

fn main() -> Result<()> {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    let args: Vec<String> = std::env::args().collect();
    let show_parse = args.contains(&String::from("-p"));

    let mut symtab: Option<SymbolTable> = None;
    let mut env = Env::new();

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buffer) => {
                let source = Source::new(&buffer, "stdin".to_string());

                let parse = parse(source, symtab)?;

                if show_parse {
                    println!("{:#?}", parse.stmts);
                }

                symtab = Some(parse.symtab.clone());

                interpret(parse, Some(&mut env))?;
            }
            Signal::CtrlD | Signal::CtrlC => {
                println!("\nAborted!");
                break Ok(());
            }
        }
    }
}
