use interpreter::{env::Env, interpret};
use miette::Result;
use parser::{parse, source::Source,  SymbolTable};
use reedline::{DefaultPrompt, Reedline, Signal};

fn main() -> Result<()> {
    let args = parse_arguments();

    if args.input_files.len() == 0 {
        repl(args)
    } else {
        todo!("file support")
    }
}

fn repl(args: NakArguments) -> Result<()> {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    let mut symtab: Option<SymbolTable> = None;
    let mut env = Env::new();

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buffer) => {
                let source = Source::new(0, buffer, "stdin".to_string());

                let parse = parse(source.clone(), symtab)
                    .map_err(|error| error.with_source_code(source))?;

                if args.show_parse {
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

#[derive(Debug)]
struct NakArguments {
    input_files: Vec<Source>,
    show_parse: bool,
}

fn parse_arguments() -> NakArguments {
    let args: Vec<String> = std::env::args().collect();

    let is_present = |flags: &[&str]| args.iter().any(|arg| flags.contains(&arg.as_str()));

    let show_parse = is_present(&["-p", "--show-parse"]);

    NakArguments {
        input_files: vec![],
        show_parse,
    }
}
