use ast::ty::Type;
use interpreter::{env::Environment, error::RuntimeError, interpret, Builtin, Value};
use parser::{parse, source::Source, SymbolTable};
use wasm_bindgen::{prelude::*, JsValue};

static mut OUTPUT: String = String::new();

#[wasm_bindgen]
pub fn wasm_interpret(source: &str) -> JsValue {
    let mut builtins = vec![];

    fn print(vals: Vec<Value>, env: &mut Environment) -> Result<Value, RuntimeError> {
        // SAFETY: OUTPUT is only every appended to so we're fine here
        unsafe {
            OUTPUT.push_str("\n");
            OUTPUT.push_str(&format!(
                "{}",
                vals.first()
                    .expect("arity mismatch didn't catch builtin")
                    .to_string(env)
            ));
        }
        Ok(Value::null())
    }
    builtins.push(Builtin::new(
        String::from("print"),
        vec![Type::Any],
        None,
        print,
    ));

    let symbols = builtins
        .clone()
        .into_iter()
        .map(|b| b.as_symbol())
        .collect();

    let mut env = Environment::new(builtins).expect("ICE: failed to create Environment");
    let symtab = SymbolTable::new(symbols);

    let source = Source::new(0, String::from(source), "stdin".to_string());
    match parse(source.clone(), symtab).map_err(|error| error.with_source_code(source.clone())) {
        Ok(parse) => {
            if let Err(e) =
                interpret(parse, &mut env).map_err(|error| error.with_source_code(source.clone()))
            {
                unsafe {
                    OUTPUT.push('\n');
                    OUTPUT.push_str(&format!("{}", e));
                }
            }
        }
        Err(e) => unsafe {
            OUTPUT.push('\n');
            OUTPUT.push_str(&format!("{}", e));
        },
    }
    unsafe { JsValue::from_str(&OUTPUT) }
}
