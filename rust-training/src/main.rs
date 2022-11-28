fn print_values() {
    // Printing Values
    println!("Hello, {}", "alice");
    println!("Hello, {:?}", "bob");

    let name = "peter";
    println!("Hello, {name}");
    println!("Hello, {name:?}");
}

fn variables() -> u32 {
    // Variables are immutable by deafult
    let mut age = 21;
    age += 1;
    age
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci2(n: u32) -> u32 {
    n + if n > 1 { fibonacci2(n - 1) } else { 0 }
}

fn vectors() {
    let mut scores = vec![100, 90, 85];
    scores[0] -= 10;
    scores.push(100);
    println!("scores: {scores:?}");
}

fn main() {
    print_values();

    variables();

    println!("fib(10)={}", fibonacci(10));
    println!("fib2(10)={}", fibonacci2(10));

    vectors();
}
