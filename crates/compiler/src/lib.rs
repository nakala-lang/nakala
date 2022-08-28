use ast::{
    expr::{Expr, Expression},
    op::{Operator},
    stmt::{Statement, Stmt},
};
use miette::Diagnostic;
use parser::Parse;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum CompileError {}

pub fn compile(parse: Parse) -> miette::Result<String> {
    let mut output = String::with_capacity(8192);
    add_prelude(&mut output);
    output.push_str("int main(int argc, char** argv) {");

    for stmt in parse.stmts {
        output.push_str(&stmt.codegen()?);
    }

    output.push('}');

    Ok(output)
}

trait Codegen {
    fn codegen(&self) -> Result<String, CompileError>;
}

impl Codegen for Statement {
    fn codegen(&self) -> Result<String, CompileError> {
        match &self.stmt {
            Stmt::Variable { name, expr } => {
                let mut res = format!("Value {} = ", name.name.item);

                if let Some(expr) = expr {
                    res.push_str(&expr.codegen()?);
                } else {
                    res.push_str("Value::null()");
                }

                res.push(';');

                Ok(res)
            }
            Stmt::Expr(expr) => Ok(format!("{};", expr.codegen()?)),
            _ => todo!("codegen for {:?}", self),
        }
    }
}

impl Codegen for Expression {
    fn codegen(&self) -> Result<String, CompileError> {
        match &self.expr {
            Expr::Bool(v) => Ok(format!("Value((bool) {})", v)),
            Expr::Int(v) => Ok(format!("Value((int64_t) {})", v)),
            Expr::Float(v) => Ok(format!("Value((float) {})", v)),
            Expr::String(v) => Ok(format!(r#"Value(std::string("{}"))"#, v)),
            Expr::Variable(name) => Ok(name.to_string()),
            Expr::Call { callee, args, .. } => {
                match &callee.expr {
                    // TODO - deal with builtins better
                    Expr::Variable(name) => {
                        if name == "print" {
                            Ok(format!("std::cout << {}", args.get(0).unwrap().codegen()?))
                        } else {
                            todo!("codegen for non print")
                        }
                    }
                    _ => todo!("callee codegen for {:?}", self),
                }
            }
            _ => todo!("codegen for {:?}", self),
        }
    }
}

impl Codegen for Operator {
    fn codegen(&self) -> Result<String, CompileError> {
        todo!("codegen for {:?}", self)
    }
}

fn add_prelude(output: &mut String) {
    output.push_str(r#"
#include<iostream>
#include<vector>

class Value {
public:
  enum class Type {
    Null,
    Int,
    Float,
    String,
    Bool,
  };

  bool is_null() const { return m_type == Type::Null; }
  bool is_int() const { return m_type == Type::Int; }
  bool is_float() const { return m_type == Type::Float; }
  bool is_string() const { return m_type == Type::String; }
  bool is_bool() const { return m_type == Type::Bool; }

  Value() : m_type(Type::Null) {}

  explicit Value(bool value) : m_type(Type::Bool) { m_value.as_bool = value; }
  explicit Value(int64_t value) : m_type(Type::Int) { m_value.as_int = value; }
  explicit Value(float value) : m_type(Type::Float) { m_value.as_float = value; }
  explicit Value(std::string value) : m_type(Type::String) { m_value.as_string = const_cast<char*>(value.c_str()); } 

  std::string to_string() const {
    switch (m_type) {
    case Type::Null:
      return "null";
    case Type::Int:
      return std::to_string(m_value.as_int);
    case Type::Bool:
      return m_value.as_bool ? "true" : "false";
    case Type::String:
      return m_value.as_string;
    case Type::Float:
      return std::to_string(m_value.as_float);
    default:
      std::cout << "nyi" << std::endl;
      exit(2);
    }
  }

private:
  Type m_type{Type::Null};

  union t {
    int64_t as_int;
    float as_float;
    bool as_bool;
    char *as_string;
  } m_value;
};

std::ostream &operator<<(std::ostream &Str, Value const &val) {
  Str << val.to_string() << std::endl;
  return Str;
}"#);
}
