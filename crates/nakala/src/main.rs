mod line_editor;

use clap::{App, Arg, ArgMatches};
use crossterm::event::{read, Event};
use crossterm::Result;
use engine::env::Env;
use engine::error::EngineError;
use engine::val::Val;
use hir::Hir;
use line_editor::LineEditor;
use parser::Parse;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let matches = App::new("nakala")
        .version("0.1.0")
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
        .get_matches();

    // If a file is passed in
    if let Some(path) = matches.value_of("file") {
        let path = Path::new(path);
        if path.exists() {
            // parse the entire file into a buffer and execute it
            match read_to_string(path) {
                Ok(buf) => {
                    parse_and_eval_buffer(&buf, &mut Env::default());
                }
                Err(_) => {
                    eprintln!("Failed to parse file.");
                }
            }
        } else {
            eprintln!("File does not exist.");
        }
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
    buffer: &String,
    env: &mut Env,
) -> std::result::Result<NakalaResult, EngineError> {
    let parse = parser::parse(buffer.as_str());
    let ast_tree = ast::Root::cast(parse.syntax()).unwrap();
    let hir = hir::lower(ast_tree);

    engine::eval(env, hir.clone()).map(|val| NakalaResult { parse, hir, val })
}
