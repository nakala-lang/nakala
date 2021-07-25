use clap::{App, Arg, ArgMatches};
use engine::env::Env;
use engine::error::EngineError;
use engine::val::Val;
use hir::Hir;
use parser::Parse;
use reedline::{DefaultPrompt, FileBackedHistory, Reedline, Signal};
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let matches = App::new("nakala")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Reagan McFarland")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Run a .nak program")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("parse")
                .long("--show-parse")
                .takes_value(false)
                .help("Show the raw parse tree when using the REPL"),
        )
        .arg(
            Arg::with_name("hir")
                .long("--show-hir")
                .takes_value(false)
                .help("Show the HIR tree when using the REPL"),
        )
        .arg(
            Arg::with_name("input")
                .value_name("INPUT")
                .long("--input")
                .short("-i")
                .takes_value(true)
                .help("Parse raw string as a nakala program")
                .long_help("nakala -i \"let x = 5   x\""),
        )
        .get_matches();

    // If a file is passed in
    if let Some(path) = matches.value_of("file") {
        let path = Path::new(path);
        if path.exists() {
            // parse the entire file into a buffer and execute it
            match read_to_string(path) {
                Ok(buf) => {
                    run_without_repl(&buf, matches);
                }
                Err(_) => {
                    eprintln!("Failed to parse file.");
                }
            }
        } else {
            eprintln!("File does not exist.");
        }
    } else if let Some(input) = matches.value_of("input") {
        run_without_repl(input, matches.clone());
    } else {
        cli_main(matches)
    }
}

fn run_without_repl(buffer: &str, matches: ArgMatches) {
    match parse_and_eval_buffer(&buffer, &mut Env::default()) {
        Ok(NakalaResult { parse, hir, val }) => {
            if matches.is_present("parse") {
                println!("{}", parse.debug_tree());
            }

            if matches.is_present("hir") {
                println!("{:#?}", hir.stmts);
            }

            println!("{}", val);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}

fn cli_main(cli_args: ArgMatches) {
    let history = Box::new(
        FileBackedHistory::with_file(10, "~/.config/nakala/history.txt".into())
            .expect("Error configuring history with file"),
    );

    let mut line_editor = Reedline::new()
        .with_history(history)
        .expect("Error configuring reedline with history");
    let prompt = DefaultPrompt::default();

    let mut env = Env::default();

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buffer) => match parse_and_eval_buffer(buffer.as_str(), &mut env) {
                Ok(NakalaResult { parse, hir, val }) => {
                    if cli_args.is_present("parse") {
                        println!("{}", parse.debug_tree());
                    }

                    if cli_args.is_present("hir") {
                        println!("{:#?}", hir.stmts);
                    }

                    println!("{:?}", val);
                }
                Err(e) => eprintln!("{:?}", e),
            },
            Signal::CtrlD | Signal::CtrlC => {
                line_editor.print_crlf().unwrap();
                break;
            }
            Signal::CtrlL => {
                line_editor.clear_screen().unwrap();
            }
        }
    }
}

pub struct NakalaResult {
    parse: Parse,
    hir: Hir,
    val: Val,
}

pub fn parse_and_eval_buffer(
    buffer: &str,
    env: &mut Env,
) -> std::result::Result<NakalaResult, EngineError> {
    let parse = parser::parse(buffer);
    let ast_tree = ast::Root::cast(parse.syntax()).unwrap();
    let hir = hir::lower(ast_tree);

    engine::eval(env, hir.clone()).map(|val| NakalaResult { parse, hir, val })
}
