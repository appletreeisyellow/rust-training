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
    statement;
    statement;
    expression
} else if another_condition {
    statement;
    statement;
    expression
} else {
    statement;
    statement;
    expression
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

rust enforces all the cases are matched

## Structs

```rust
struct TypeName {
    member_name: MemberType,
}

let val = TypeName {
    member_name: member_value,
};
val.member_name;
```

## Tuple structs

```rust
struct TypeName(MemberType, AnotherType);

let val = TypeName(member_value, another_value);
val.0;
```

Don't use tuple if:

- there are a lot of members, limit to 2 or 3 members max
- there is not an order needed

## Reading user input

```rust
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Couldn't read from stdin");
    println!("Read: {input:?}");
}
```
