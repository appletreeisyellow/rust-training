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

Suggest to add `trim` for the input value

## Reading from files

```rust
use std::fs;

fn main() {
    let data = fs::read_to_string("/path/to/file")
        .expect("Couldn't read the file");
    println!("Read: {data:?}");
}
```

If the file is large, there are lower-level tools you can use

## Writing to files

```rust
use std::fs;

fn main() {
    fs::write("/path/to/file", "This is now in a file")
        .expect("Unable to write to the file");
}
```

## Parsing strings

### With a type on the variable

```rust
 let a: i32 = "42".parse().expect("Not an integer");
```

### With a type on the function call

```rust
let a = "42".parse::<i32>().expect("Not an integer");
```

## Import path

```rust
// Child, cd ./
use module_name::thing;

// Parent, cd ..
use super::thing;

// Crate root, cd ~
use crate::thing;
```

# Option and Result

## Option

```rust
enum Option<T> {
    Some(T),
    None,
}
```

## Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Why don't we use Result with a dummy error value?

#### Semantics

- `Options` is the presence of data
- `Result` is only the outcome of an action

### Options vs Result

**Option**

- Field or function argument
- Retrieving values from collections
- "Null" pointers

**Result**

- Return value of a function
- Annotated such that they must be used

# Iterators

## The Iterator trait

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

## IntoIterator

```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
```

## for loops use IntoIterator

```rust
for v in vec![1, 2, 3] {
    // ...
}
```

```rust
let mut i = vec![1, 2, 3].into_iter();
while let Some(v) = i.next() {
  // ...
}
```

## Closures in Rust

```rust
fn do_thing_3_times<F>(thing: F)
where
    F: Fn(),
{
    thing();
    thing();
    thing();
}

fn main() {
    do_thing_3_times(|| println!("Hello!"));
}
```

## Iterator::map

```rust
trait Iterator {
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Item) -> B,
}
```

Closure given an item, returns a new item.

### Example

```rust
for i in (o..10).map(|v| v * 2) {
  println!("{i}");
}
```

## Iterator::filter

```rust
trait Iterator {
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        P: FnMut(&Self::Item) -> bool,
}
```

Closure given a reference to an item, returns false if this item should be discarded.

### Example

```rust
for i in (0..10).filter(|v| *v < 5>) {
  println!("{i}");
}

for i in (0..10).filter(|v| v < &5>) {
  println!("{i}");
}

for i in (0..10).filter(|&v| v < 5>) {
  println!("{i}");
}
```

## Iterator::collect

```rust
trait Iterator {
    fn collect<B>(self) -> B
    where
        B: FromIterator<Self::Item>,
}
```

Builds a collection of values from the iterator.

### Example

```rust
let nums: Vec<_> = (0..10).collect();
let nums: HashSet<_> = (0..10).collect();
let nums: LinkedList<_> = (0..10).collect();
```

## Iterators vs Indexing

### Avoid

```rust
let scores = vec![1, 2, 3];
for i in 0..scores.len() {
    let v = scores[i];
    println!("{v}");
}
```

### Prefer

```rust
let scores = vec![1, 2, 3];
for v in &scores {
    println!("{v}");
}
```

## Returning iterators

```rust
fn song_names() -> impl Iterator<Item = &'static str> {
    vec!["crash", "wow", "my heart will go on"].into_iter()
}
fn short_song_names() -> impl Iterator<Item = &'static str> {
    song_names().filter(|name| name.len() < 5)
}
fn main() {
    for name in short_song_names() {
        println!("Listen to {name}");
    }
}
```

- `impl Iterator<Item = &'static str>` is a return type that is a type of iterator.
- This allows the function not to expose its implementation details.

## Conditionally returning iterator using trait objects

```rust
fn song_names() -> impl Iterator<Item = &'static str> {
    vec!["crash", "wow", "my heart will go on"].into_iter()
}
fn good_song_names(short: bool) -> Box<dyn Iterator<Item = &'static str>> {
    if short {
        Box::new(song_names().filter(|name| name.len() < 5))
    } else {
        Box::new(song_names().filter(|name| name.len() > 5))
    }
}
fn main() {
    for name in good_song_names(true) {
        println!("Listen to {name}");
    }
}
```

Use `Box<dyn Iterator<Item = &'static str>>` and `Box::new(...)` when returning iterator conditionally

## Implementing an iterator

### Trait definition

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

### Trait implementation

```rust
impl Iterator for MyType {
    type Item = TheTypeOfEachElement;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
```

## Ownership of iterators is often transferred

```rust
fn main() {
    let input = "12 Bananas";
    let letters = input.chars();
    for l in letters {
        if l == ' ' {
            break;
        }
    }
    println!("{}", letters.as_str());
    // this will fail to compile since `letters` is
    // used in the for loop
}
```

```rust
trait Iterator {
    fn by_ref(&mut self) -> &mut Self
}

fn main() {
    let input = "12 Bananas";
    let mut letters = input.chars();
    for l in letters.by_ref() {
        if l == ' ' {
            break;
        }
    }
    println!("{}", letters.as_str());
}

// OR

fn main() {
    let input = "12 Bananas";
    let mut letters = input.chars();
    for l in &mut letters {
        if l == ' ' {
            break;
        }
    }
    println!("{}", letters.as_str());
}
```

## Iterators that reference themselves

```rust
trait LendingIterator {
  type Item<'a>
  where
    Self: 'a;

  fn next(&mut self) -> Option<Self::Item<'_>>;
}

impl LendingIterator for StrangeIter {
  type Item<'a> = &'a i32;

  fn next(&mut self) -> Option<Slef::Item<'_>> {
    Some(&self.data[self.index])
  }
}
```

# Advanced Types and Traits

## Traits are Rust's primary interesting form of abstraction

- Functions provide abstraction, but are well-known
- Macros are a different kind of abstraction, but less common

## Common traits

- Copy / Clone
- Debug / Display
- Iterator / IntoIterator
- Into / From / TryInto / TryFrom
- Fn / FnMut / FnOnce
- Add / Sub / Mul / Div / etc.
- Read / Write

## Bsic structure of a trait

```rust
trait Price {
    fn price(&self) -> i32;
}

struct Apple;

impl Price for Apple {
    fn price(&self) -> i32 {
        1
    }
}

fn main() {
    let a = Apple;
    println!("{}", a.price());
}
```

## Basic structure of generic functions

```rust
fn my_generic<T>(value: T) -> T {
    value
}
```

## Trait bounds

### Inline

```rust
use std::fmt::Display;
fn my_generic<T: Display>(value: T) {
    println!("{value}");
}
```

### Where clause

```rust
use std::fmt::Display;
fn my_generic<T>(value: T)
where
    T: Display,
{
    println!("{value}");
}
```

Suggest to use the where clause than the inline format

## Generic associated types

```rust
trait LendingIterator {
  type Item<'a>
  where
    Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}
```

```rust
struct BufferedData {
  data: [u8; 4],
  idx: usize,
}

impl LendingIterator for BufferedData {
  type Item<'a> = &'a mut u8;

  for next(&mut self) -> Option<Self::Item<'_>> {
    let v = self.data.get_mut(self.idx)?;
    self.idx += 1;
    Some(v)
  }
}

fn main() {
  let mut buf = BufferedData {
    data: [1, 2, 3, 4],
    idx: 0,
  };

  while let Some(v) = buf.next() {
    println!("{v}");
  }
}
```

Key things to remember:

- Compare to the normal trait, this trait can only have one item out at a time. As soon as you have on item, you cannot use it anymore

## `impl Trait` as an argument

```rust
fn lots_of_noise(animal: impl Animal) {
    for _ in 0..10 {
        animal.speak()
    }
}
```

```rust
fn lots_of_noise<A: Animal>(animal: A) {
    for _ in 0..10 {
        animal.speak()
    }
}
```

```rust
fn lots_of_noise<A>(animal: A)
where
    A: Animal,
{
    for _ in 0..10 {
        animal.speak()
    }
}
```

use `impl Animal` if you can

## Rust has three kinds of generics

**Lifetimes**

```rust
fn alpha<'a>(x: &'a u8) {}
```

**Types**

```rust
fn sigma<T>(x: T) {}
```

**Constants**

```rust
fn omega<const N: usize>() {}
```

Example

```rust
fn many_zeroes<const N: usize>() -> [i32; N] {
    [0; N]
}

fn main() {
    let [a, b] = many_zeroes();
    let [c] = many_zeroes();
    let d = many_zeroes::<2>();
    dbg!(a, b, c, d);
}
```

## Lifetimes and traits

### Lifetimes elided

```rust
impl Gradebook {
  fn scores(&self) -> impl Iterator<Item = u8> {
    self.scores.iter().copied()
  }
}
```

### Lifetimes explicit

```rust
impl Gradebook {
  fn scores<'a>(&'a self) -> impl Iterator<Item = u8> + 'static {
    self.scores.iter().copied()
  }
}
```

### Actual types

```rust
impl Gradebook {
    fn scores<'a>(&'a self) -> std::iter::Copied<std::slice::Iter<'a, u8>> {
        self.scores.iter().copied()
    }
}
```

## Indicating lifetimes

### Explicit lifetime

```rust
impl Gradebook {
    fn scores<'a>(&'a self) -> impl Iterator<Item = u8> + 'a {
        self.scores.iter().copied()
    }
}
```

### Anonymous lifetime

```rust
impl Gradebook {
    fn scores(&self) -> impl Iterator<Item = u8> + '_ {
        self.scores.iter().copied()
    }
}
```

## Higher-ranked trait bounds

This is important but infrequently useful

```rust
trait Holder {
    type Value;
    fn get(&self) -> &Self::Value;
}
trait Identify {
    fn identify(&self);
}
struct Container(Example);
impl Holder for Container {
    type Value = Example;
    fn get(&self) -> &Self::Value {
        &self.0
    }
}
struct Example;
impl Identify for Example {
    fn identify(&self) {
        println!("I am an Example")
    }
}
```

Combine the two trait into one

```rust
fn default_and_identify<H>(holder: H)
where
    H: Holder,
    for<'a> &'a H::Value: Identify,
{
    let value = holder.get();
    value.identify();
}
```

Whenever you need to place the reference trait bound on anything you own, you need to use higher-ranked trait bounds.

# Concurrency

## Interior mutability

```rust
use std::cell::Cell;
fn main() {
    let shared = Cell::new(42);
    shared.set(99);
}
```

```rust
use std::cell::RefCell;
fn main() {
    let shared = RefCell::new(42);
    *shared.borrow_mut() += 1;
}
```

- `RefCell` is single-thread read-write lock
- The downside of `Cell` and `RefCell` is that they are only for single-threaded reference-counting pointer

## Applying shared ownership and interior mutability

```rust
use std::{sync::{Arc, Mutex}, thread};
fn main() {
    let scores = Arc::new(Mutex::new(vec![42]));
    let thread_scores = scores.clone();
    let handle = thread::spawn(move || {
        println!("Scores A: {thread_scores:?}");
        thread_scores.lock().expect("lock poisoned").push(1);
        println!("Scores B: {thread_scores:?}");
    });
    scores.lock().expect("lock poisoned").push(2);
    println!("Scores C: {scores:?}");
    handle.join().expect("The thread panicked");
}
```

- Rust can prevent memery leaking
- Rust cannot prevent race condition

## Sharing values while the thread is running

> Do not communicate by sharing memory; instead, share memory by communicating.
