# rust-training

Online rust playground https://play.rust-lang.org/

## When to use rust?

- Political reason according to the company
- For microcontrollers, processors, wasm
- Safety protocal

* Rust is a strongly typed language

## Rustup

- Toolchain manager
- upgrading `rustup update`

## Cargo

- Package manager
- Build tool
- Create a new cargo

```
cargo new --bin <name=of-cargo>
```

```
cargo run
```

## VS Code Extension

- rust-analyzer - auto complete
- CodeLLDB - a debugger

## API Documentation

https://doc.rust-lang.org/stable/std/

## Functions

```rust
fn the_name(arg1: Type1, arg2: Type2, ...) -> ReturnType {
  statement;
  statement;
  expression
}

```

```rust
fn the_name(arg1: Type1, arg2: Type2, ...) -> ReturnType {
  return expression;
}
```

## Built-in types

- u32: unsigned 32-bit integer
- i32: signed 32-bit integer
- f64: floating point number
- String and/or &str: more on these later. String in rust is more complex than it is in other languages
- bool: a boolean
- (T1, T2, ...): a tuple

## if

```rust
if condition { // no () on condition
    // statements + optional expression
} else if another_condition {
    // statements + optional expression
} else {
    // statements + optional expression
}
```

## match

```rust
match expression {
    pattern_1 => expression,
    pattern_2 => {
        statement;
        statement;
        expression
    }
    _ => expression,
}
```
