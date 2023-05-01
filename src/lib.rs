use std::{sync::Mutex, collections::HashMap};
use lazy_static::lazy_static;
use anyhow::Result;

#[derive(Clone, PartialEq)]
pub enum Type {
    Bool,
    Int,
    Float,
    String,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
}

impl Value {
    fn get_type(&self) -> Type {
        match self {
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::String(_) => Type::String,
        }
    }
}

lazy_static! {
    static ref FUNCTION_REGISTRY: Mutex<HashMap<String, Function>> = Mutex::new(HashMap::new());
}

pub struct Function {
    params: Vec<Type>,
    ret_type: Type,
    func_ptr: Box<dyn Fn(&[Value]) -> Result<Value> + Send + Sync>,
}

pub fn register_function(name: &str, params: &[Type], ret_type: Type, func: impl Fn(&[Value]) -> Result<Value> + 'static + Send + Sync) {
    let func_ptr = Box::new(func) as Box<dyn Fn(&[Value]) -> Result<Value> + Send + Sync>;
    let func = Function { params: params.to_vec(), ret_type, func_ptr };
    FUNCTION_REGISTRY.lock().unwrap().insert(name.to_string(), func);
}

pub fn call_function(name: &str, args: &[Value], ret_type: Type) -> Result<Value> {
    let registry = FUNCTION_REGISTRY.lock().unwrap();
    let func = registry.get(name).ok_or(anyhow::Error::msg("Function not found"))?;
    if func.params.len() != args.len() {
        return Err(anyhow::Error::msg("Parameter count mismatch"));
    }
    for (param_type, arg) in func.params.iter().zip(args.iter()) {
        if param_type.clone() != arg.get_type() {
            return Err(anyhow::Error::msg("Parameter type mismatch"));
        }
    }
    if func.ret_type != ret_type {
        return Err(anyhow::Error::msg("Return type mismatch"));
    }
    let result = (func.func_ptr)(args)?;
    Ok(result)
}
