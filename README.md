# `dyn-dispatch`

`dyn-dispatch` is an API that provides a dynamic dispatch mechanism for functions with any assortment of arguments and return types. It allows you to register functions with a specific name, parameter types, and return type, and then call them by name with arbitrary arguments at runtime.

## Usage

```rust
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

fn main() -> anyhow::Result<()> {
    // Register the Rust function with the API
    register_function("add", &[Type::Int, Type::Int], Type::Int, add);

    // Call the registered Rust function with different arguments
    let result1 = call_function("add", &[Value::Int(1), Value::Int(2)], Type::Int)?;
    assert_eq!(result1, Value::Int(3));
    let result2 = call_function("add", &[Value::Int(1), Value::Float(2.5)], Type::Int);
    assert!(result2.is_err());
    let result3 = call_function("add", &[Value::Int(1)], Type::Int);
    assert!(result3.is_err());
    Ok(())
}
```

In this example, we register a Rust function add with the API, specifying that it takes two `Int` arguments and returns an `Int`. We then call add with two valid arguments (`Value::Int(1)` and `Value::Int(2)`), and also with an invalid argument (`Value::Float(2.5)`) and an incorrect number of arguments (`Value::Int(1)`).