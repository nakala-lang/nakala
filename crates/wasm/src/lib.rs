use ast::ty::Type;
use interpreter::{env::Environment, error::RuntimeError, interpret, Builtin, Val, Value};
use parser::{parse, source::Source, SymbolTable};
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen]
pub fn wasm_interpret(source: &str) -> JsValue {
    match helper(source) {
        Ok(val) => JsValue::null(),
        Err(e) => {
            output.push_str(&format!("{:?}", e));
            JsValue::from_str(&output)
        },
    }
}

pub fn helper(source: &str) -> miette::Result<()> {
    let mut output = String::new();

    let mut builtins = vec![];

    fn print(vals: Vec<Value>, env: &mut Environment) -> Result<Value, RuntimeError> {
        output.push_str(&format!(
            "{}",
            vals.first()
                .expect("arity mismatch didn't catch builtin")
                .to_string(env)
        ));
        output.push_str("\n");
        Ok(Value::null())
    }
    builtins.push(Builtin::new(
        String::from("print"),
        vec![Type::Any],
        None,
        print,
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

    let symbols = builtins
        .clone()
        .into_iter()
        .map(|b| b.as_symbol())
        .collect();

    let mut env = Environment::new(builtins).expect("ICE: failed to create Environment");
    let symtab = SymbolTable::new(symbols);

    let source = Source::new(0, String::from(source), "stdin".to_string());
    let parse =
        parse(source.clone(), symtab).map_err(|error| error.with_source_code(source.clone()))?;

    interpret(parse, &mut env).map_err(|error| error.with_source_code(source.clone()))?;

    Ok(())
}
