use dyn_dispatch::{Type, Value, call_function, register_function};

fn add(args: &[Value]) -> anyhow::Result<Value> {
    let a = match &args[0] {
        Value::Int(n) => *n,
        _ => return Err(anyhow::Error::msg("Invalid argument type")),
    };
    let b = match &args[1] {
        Value::Int(n) => *n,
        _ => return Err(anyhow::Error::msg("Invalid argument type")),
    };
    let sum = a + b;
    Ok(Value::Int(sum))
}

fn concat(args: &[Value]) -> anyhow::Result<Value> {
    let s1 = match &args[0] {
        Value::String(s) => s,
        _ => return Err(anyhow::Error::msg("Invalid argument type")),
    };
    let s2 = match &args[1] {
        Value::String(s) => s,
        _ => return Err(anyhow::Error::msg("Invalid argument type")),
    };
    let result = s1.to_owned() + s2;
    Ok(Value::String(result))
}

fn main() -> anyhow::Result<()> {
    // Register the Rust functions with the API
    register_function("add", &[Type::Int, Type::Int], Type::Int, add);
    register_function("concat", &[Type::String, Type::String], Type::String, concat);

    // Call the registered Rust functions with different arguments
    let result1 = call_function("add", &[Value::Int(1), Value::Int(2)], Type::Int)?;
    assert_eq!(result1, Value::Int(3));
    let result2 = call_function("concat", &[Value::String("hello".to_string()), Value::String("world".to_string())], Type::String)?;
    assert_eq!(result2, Value::String("helloworld".to_string()));
    let result3 = call_function("add", &[Value::Int(1), Value::Float(2.5)], Type::Int);
    assert!(result3.is_err());
    let result4 = call_function("add", &[Value::Int(1)], Type::Int);
    assert!(result4.is_err());
    Ok(())
}
