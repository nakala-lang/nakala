use crate::{error::EngineError, val::Val};

pub(crate) const BUILTINS: [&'static str; 3] = ["print", "println", "len"];

pub(crate) fn dispatch_builtin(
    func_name: &str,
    param_values: Vec<Val>,
) -> Result<Val, EngineError> {
    match func_name {
        "print" => {
            __print(&param_values[0]);
            Ok(Val::Unit)
        }
        "println" => {
            __println(&param_values[0]);
            Ok(Val::Unit)
        }
        "len" => __len(&param_values[0]),
        _ => unreachable!("Dispatched non existent builtin"),
    }
}

/// `print` builtin function
///
/// Prints a value to stdout
pub(crate) fn __print(val: &Val) {
    print!("{}", val);
}

/// `println` builtin function
///
/// Prints a value to stdout with new line
pub(crate) fn __println(val: &Val) {
    println!("{}", val);
}

/// `len` builtin function
///
/// Returns the length of a value. If the value does not have a length then
/// [EngineError::InvalidLenOperation] occurs
pub(crate) fn __len(val: &Val) -> Result<Val, EngineError> {
    match val {
        Val::List(l) => Ok(Val::Number(l.len() as f64)),
        Val::String(s) => Ok(Val::Number(s.len() as f64)),
        _ => Err(EngineError::InvalidLenOperation { x: val.clone() }),
    }
}
