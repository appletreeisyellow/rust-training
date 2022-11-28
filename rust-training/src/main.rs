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

fn iterators() {
    for i in (0..10).filter(|y| y % 2 == 0).map(|x| x * 2) {
        print!("{i} ");
    }
    println!("");
}

fn iterators_single_value() {
    let values: Vec<i32> = (0..10).collect();
    let total: i32 = (0..10).sum();
    println!("collect {values:?}");
    println!("total {total}");
}

fn exercise_filter_sum() {
    let mut sum = 0;
    for i in (0..100).filter(|x| x % 3 == 0).filter(|x| x % 7 == 0) {
        println!("{i}");
        sum += i;
    }
    println!("sum {sum}");
}

#[derive(Debug)] // This will allow you to print a value with {:?}
struct Fahrenheit {
    value: f32,
}

#[derive(Debug)]
struct Celsius {
    value: f32,
}

fn f_to_c(f: Fahrenheit) -> Celsius {
    Celsius {
        value: (f.value - 32.0) / 1.8,
    }
}

impl Celsius {
    fn to_f(&self) -> Fahrenheit {
        Fahrenheit {
            value: self.value * 1.8 + 32.0,
        }
    }
}

fn fizzbuzz(n: i32) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}

fn main() {
    print_values();

    variables();

    println!("fib(10)={}", fibonacci(10));
    println!("fib2(10)={}", fibonacci2(10));

    vectors();

    iterators();

    iterators_single_value();

    exercise_filter_sum();

    println!(
        "100.0F is {:.1}C",
        f_to_c(Fahrenheit { value: 100.0 }).value
    );

    let c = Celsius { value: 10.0 };
    println!("{:.1}C is {:.1}F", c.value, c.to_f().value);

    for i in 1..=100 {
        println!("{}", fizzbuzz(i));
    }
}
