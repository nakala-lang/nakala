use clap::{App, Arg, ArgMatches};
use engine::env::Env;
use engine::error::EngineError;
use engine::val::Val;
use hir::Hir;
use parser::{Parse, ParseError};
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
                .help("Show the parse tree when running a program or using the REPL"),
        )
        .arg(
            Arg::with_name("hir")
                .long("--show-hir")
                .takes_value(false)
                .help("Show the HIR when running a program or using the REPL"),
        )
        .arg(
            Arg::with_name("dbg")
            .long("--debug")
            .takes_value(false)
            .help("Enable common debugging settings, like showing values as the underlying Rust data type")
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
    match parse_and_eval_buffer(buffer, &mut Env::new(None)) {
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

    let mut env = Env::new(None);

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buffer) => {
                if buffer == "__env" {
                    println!("{:#?}", env.get_all_bindings());
                } else {
                    match from_string_to_parse(buffer.as_str()) {
                        Ok(parse) => {
                            if cli_args.is_present("parse") {
                                println!("{}", parse.debug_tree());
                            }

                            match from_parse_to_ast(parse) {
                                Some(ast) => {
                                    let hir = from_ast_to_hir(ast);

                                    if cli_args.is_present("hir") {
                                        println!("{:#?}", hir.stmts);
                                    }

                                    match evaluate_hir(hir, &mut env) {
                                        Ok(val) => {
                                            if cli_args.is_present("dbg") {
                                                println!("{:?}", val);
                                            } else {
                                                println!("{}", val);
                                            }
                                        }
                                        Err(e) => eprintln!("{}", e),
                                    }
                                }
                                None => {
                                    // show ast errors
                                    eprintln!("Parse error: unable to parse into AST");
                                }
                            }
                        }
                        Err(parse_errors) => {
                            // show parse errors
                            for error in parse_errors {
                                eprintln!("{}", error)
                            }
                        }
                    }
                }
            }
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

pub fn from_string_to_parse(buffer: &str) -> Result<Parse, Vec<ParseError>> {
    let mut parse = parser::parse(buffer);

    if parse.has_errors() {
        Err(parse.get_errors())
    } else {
        Ok(parse)
    }
}

pub fn from_parse_to_ast(parse: Parse) -> Option<ast::Root> {
    ast::Root::cast(parse.syntax())
}

pub fn from_ast_to_hir(root: ast::Root) -> Hir {
    hir::lower(root)
}

pub fn evaluate_hir(hir: Hir, env: &mut Env) -> Result<Val, EngineError> {
    engine::eval(env, hir)
}

pub fn parse_and_eval_buffer(
    buffer: &str,
    env: &mut Env,
) -> std::result::Result<NakalaResult, EngineError> {
    let parse = parser::parse(buffer);
    let ast = ast::Root::cast(parse.syntax()).unwrap();
    let hir = hir::lower(ast);

    engine::eval(env, hir.clone()).map(|val| NakalaResult { parse, hir, val })
}
