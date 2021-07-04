mod line_editor;

use clap::{App, Arg, ArgMatches};
use crossterm::event::{read, Event};
use crossterm::Result;
use engine::env::Env;
use engine::error::EngineError;
use engine::val::Val;
use hir::{Database, Hir};
use line_editor::LineEditor;
use parser::Parse;
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
        // Load line editor
        match cli_main(matches) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("An error occurred.");
                crossterm::terminal::disable_raw_mode().unwrap();
            }
        }
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

fn cli_main(cli_args: ArgMatches) -> Result<()> {
    let mut le = LineEditor::new("> ", cli_args)?;
    crossterm::terminal::enable_raw_mode()?;

    loop {
        match read()? {
            Event::Key(key_event) => le.dispatch_key_event(key_event)?,
            Event::Mouse(mouse_event) => le.dispatch_mouse_event(mouse_event)?,
            Event::Resize(new_cols, new_rows) => le.dispatch_resize_event(new_cols, new_rows)?,
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
