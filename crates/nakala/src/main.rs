use interpreter::{env::Environment, interpret};
use miette::Result;
use parser::{parse, source::Source, SymbolTable};
use reedline::{DefaultPrompt, Reedline, Signal};
use std::{fs::read_to_string, path::Path};

fn main() -> Result<()> {
    let args = parse_arguments();

    if args.input_files.len() == 0 {
        repl(args)
    } else {
        if args.input_files.len() > 1 {
            todo!("multiple files are not supported yet.");
        }

        for source in args.input_files.into_iter() {
            let parse =
                parse(source.clone(), None).map_err(|error| error.with_source_code(source.clone()))?;

            if args.show_parse {
                println!("{:#?}", parse);
            }

            interpret(parse, None).map_err(|error| error.with_source_code(source))?;
        }

        Ok(())
    }
}

fn repl(args: NakArguments) -> Result<()> {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    let mut symtab: Option<SymbolTable> = None;
    let mut env = Environment::new();

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buffer) => {
                let source = Source::new(0, buffer, "stdin".to_string());

                let parse = parse(source.clone(), symtab)
                    .map_err(|error| error.with_source_code(source))?;

                if args.show_parse {
                    println!("{:#?}", parse);
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

    let mut next_file_id = 0;
    let input_files = args
        .into_iter()
        .filter(|arg| arg.ends_with(".nak"))
        .filter_map(|filepath| {
            let path = Path::new(&filepath);
            if path.exists() {
                if let Ok(contents) = read_to_string(path) {
                    let t = Source::new(next_file_id, contents, filepath);
                    next_file_id += 1;
                    return Some(t);
                }
            }

            None
        })
        .collect();

    NakArguments {
        input_files,
        show_parse,
    }
}
