use ast::ty::{Type, TypeExpression};
use compiler::compile;
use interpreter::{env::Environment, error::RuntimeError, interpret, Builtin, Val, Value};
use meta::Span;
use miette::Result;
use parser::{parse, source::Source, SymbolTable};
use reedline::{DefaultPrompt, Reedline, Signal};
use std::{fs::read_to_string, path::Path};

fn main() -> Result<()> {
    let args = parse_arguments();

    let builtins = get_builtins();
    let symbols = builtins
        .clone()
        .into_iter()
        .map(|b| b.as_symbol())
        .collect();

    let mut env = Environment::new(builtins)?;
    let symtab = SymbolTable::new(symbols);

    if args.use_interpreter && args.input_files.is_empty() {
        repl(args)
    } else {
        if args.input_files.len() > 1 {
            todo!("multiple files are not supported yet.");
        }

        for source in args.input_files.into_iter() {
            let parse = parse(source.clone(), symtab.clone())
                .map_err(|error| error.with_source_code(source.clone()))?;

            if args.show_parse {
                println!("{:#?}", parse);
            }

            if args.use_compiler {
                let res = compile(parse).map_err(|error| error.with_source_code(source))?;
                println!("{}", res);
            } else {
                interpret(parse, &mut env).map_err(|error| error.with_source_code(source))?;
            }
        }

        Ok(())
    }
}

fn repl(args: NakArguments) -> Result<()> {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    let builtins = get_builtins();
    let symbols = builtins
        .clone()
        .into_iter()
        .map(|b| b.as_symbol())
        .collect();

    let mut env = Environment::new(builtins)?;
    let mut symtab = SymbolTable::new(symbols);

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buffer) => {
                let source = Source::new(0, buffer, "stdin".to_string());

                let parse = parse(source.clone(), symtab)
                    .map_err(|error| error.with_source_code(source.clone()))?;

                if args.show_parse {
                    println!("{:#?}", parse);
                }

                symtab = parse.symtab.clone();

                interpret(parse, &mut env).map_err(|error| error.with_source_code(source))?;
            }
            Signal::CtrlD | Signal::CtrlC => {
                println!("\nAborted!");
                break Ok(());
            }
        }
    }
}

fn get_builtins() -> Vec<Builtin> {
    let mut builtins = vec![];

    // print
    fn print(vals: Vec<Value>, env: &mut Environment) -> Result<Value, RuntimeError> {
        println!(
            "{}",
            vals.first()
                .expect("arity mismatch didn't catch builtin")
                .to_string(env)
        );

        Ok(Value::null())
    }
    builtins.push(Builtin::new(
        String::from("print"),
        vec![Type::Any],
        None,
        print,
    ));

    // exit
    fn exit(vals: Vec<Value>, _: &mut Environment) -> Result<Value, RuntimeError> {
        let code = vals
            .first()
            .expect("arity mismatch didn't catch builtin")
            .as_int()
            .expect("builtin didn't catch type mismatch");

        if let Ok(code) = i32::try_from(code) {
            std::process::exit(code);
        } else {
            panic!("Exit code can't be bigger than i32");
        }
    }
    builtins.push(Builtin::new(
        String::from("exit"),
        vec![Type::Int],
        None,
        exit,
    ));

    // str
    fn str(vals: Vec<Value>, env: &mut Environment) -> Result<Value, RuntimeError> {
        let val = vals.first().expect("arity mismatch didn't catch builtin");
        Ok(Value {
            val: Val::String(val.to_string(env)),
            span: val.span,
            ty: Type::String,
        })
    }
    builtins.push(Builtin::new(
        String::from("str"),
        vec![Type::String],
        Some(Type::String),
        str,
    ));

    // type
    fn type_(vals: Vec<Value>, _: &mut Environment) -> Result<Value, RuntimeError> {
        let val = vals.first().expect("arity mismatch didn't catch builtin");
        Ok(Value {
            val: Val::String(format!("{}", val.ty)),
            ty: Type::String,
            span: val.span,
        })
    }
    builtins.push(Builtin::new(
        String::from("type"),
        vec![Type::Any],
        Some(Type::String),
        type_,
    ));

    //len
    fn len(vals: Vec<Value>, env: &mut Environment) -> Result<Value, RuntimeError> {
        let val = vals.first().expect("arity mismatch didn't catch builtin");
        match val.val {
            Val::List { id } => Ok(env.get_list(id).len()),
            _ => todo!(""),
        }
    }
    builtins.push(Builtin::new(
        String::from("len"),
        vec![Type::Any],
        Some(Type::Int),
        len,
    ));

    // chars
    fn chars(vals: Vec<Value>, env: &mut Environment) -> Result<Value, RuntimeError> {
        let val = vals.first().expect("arity mismatch didn't catch builtin");
        match &val.val {
            Val::String(s) => {
                let new_list = env.new_list(
                    s.chars()
                        .map(|c| Value {
                            span: Span::garbage(),
                            ty: Type::String,
                            val: Val::String(String::from(c)),
                        })
                        .collect(),
                    Type::String,
                );
                Ok(new_list)
            }
            _ => unreachable!("ICE: buliltin typechecking failed"),
        }
    }

    builtins.push(Builtin::new(
        String::from("chars"),
        vec![Type::String],
        Some(Type::List(Box::new(TypeExpression::string()))),
        chars
    ));

    builtins
}

#[derive(Debug)]
struct NakArguments {
    input_files: Vec<Source>,
    use_interpreter: bool,
    use_compiler: bool,
    show_parse: bool,
}

fn parse_arguments() -> NakArguments {
    let args: Vec<String> = std::env::args().collect();

    let is_present = |flags: &[&str]| args.iter().any(|arg| flags.contains(&arg.as_str()));

    let show_parse = is_present(&["-p", "--show-parse"]);
    let use_compiler = is_present(&["-c", "--compile"]);

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
        use_interpreter: !use_compiler,
        use_compiler,
        show_parse,
    }
}
