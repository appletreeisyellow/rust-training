use rayon::prelude::*;
use std::{
    sync::{mpsc, Arc},
    thread,
};

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

// #[derive(Debug)]
// enum Temp {
//     Celsius(f32),
//     Fahrenheit(f32),
// }

// impl Temp {
//     fn convert(self) -> Temp {
//         use Temp::*;
//         match self {
//             Celsius(n) => Fahrenheit(n * 1.8 + 32.0),
//             Fahrenheit(n) => Celsius((n - 32.0) / 1.8),
//         }
//     }
// }

fn valid(n: i32, max: i32) -> Result<i32, ()> {
    if n <= max {
        Ok(n)
    } else {
        Err(())
    }
}

fn add(n1: i32, n2: i32) -> Result<i32, ()> {
    let n1 = valid(n1, 10)?;
    let n2 = valid(n2, 10)?;
    valid(n1 + n2, 15)
}

fn two_strings(s1: &str, s2: &str) -> Result<i32, Box<dyn std::error::Error + 'static>> {
    let a = s1.parse::<bool>()?;
    let b = s2.parse::<i32>()?;

    if a {
        Ok(b * 2)
    } else {
        Ok(b * 3)
    }
}

struct PowersOfTwo(u32);

impl PowersOfTwo {
    fn new() -> PowersOfTwo {
        PowersOfTwo(0)
    }
}

impl Iterator for PowersOfTwo {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        Some(u32::pow(2, self.0 - 1))
    }
}

fn add_them_up<I>(iter: I) -> i32
where
    I: Iterator<Item = i32>,
{
    iter.sum()
}

fn stringify<I>(iter: I) -> String
where
    I: Iterator,
    I::Item: ToString,
{
    iter.map(|x| x.to_string()).collect()
}

trait Calculation {
    fn eval(&self, v: i32) -> i32;
}

struct AddFortyTwo;

impl Calculation for AddFortyTwo {
    fn eval(&self, v: i32) -> i32 {
        v + 42
    }
}

struct DivideBy(i32);

impl Calculation for DivideBy {
    fn eval(&self, v: i32) -> i32 {
        v / self.0
    }
}

fn run_calculations(mut a: i32, b: &[&dyn Calculation]) -> i32 {
    for cal in b {
        a = cal.eval(a);
    }
    a
}

fn adder(values: Vec<i32>) -> i32 {
    let (tx, rx) = mpsc::channel();
    let values = Arc::new(values);
    let thread_len = 5;
    let size = values.len() / thread_len;

    let tls: Vec<_> = (0..thread_len)
        .map(|i| {
            let tx = tx.clone();
            let values = values.clone();

            thread::spawn(move || {
                let start = i * size;
                let end = start + size;
                for chunk in values[start..end].chunks(10) {
                    let local_sum = chunk.iter().sum::<i32>();
                    tx.send(local_sum).expect("unable to send value");
                }
            })
        })
        .collect();
    drop(tx);

    let total_sum = rx.into_iter().sum();

    for t in tls {
        t.join().expect("sending thread panicked")
    }

    total_sum
}

fn adder_scoped(nums: &[i32]) -> i32 {
    let threads = 5;
    let size = nums.len() / threads;

    let sum = thread::scope(|scope| {
        let tls: Vec<_> = nums
            .chunks(size)
            .map(|chunk| scope.spawn(|| chunk.iter().sum::<i32>()))
            .collect(); // make sure all the threads are started

        tls.into_iter()
            .map(|t| t.join().expect("thread panicked"))
            .sum()
    });

    sum
}

fn adder_rayon(nums: &[i32]) -> i32 {
    nums.par_iter().sum()
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

    {
        println!("adding {} and {} -> {:?}", 10, 2, add(10, 2));
        println!("adding {} and {} -> {:?}", 11, 2, add(11, 2));
        println!("adding {} and {} -> {:?}", 10, 6, add(10, 6));
    }

    {
        println!("{:?}", two_strings("true", "2"));
        println!("{:?}", two_strings("false", "2"));
        println!("{:?}", two_strings("random", "2"));
        println!("{:?}", two_strings("true", "random")); // This is for debuging print

        if let Err(e) = two_strings("true", "random") {
            // This is what to show to the user
            println!("{e}");
        }
    }

    {
        assert_eq!(
            PowersOfTwo::new().take(10).collect::<Vec<_>>(),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512]
        );
    }

    {
        assert_eq!(45, add_them_up(0..10));

        let string_slices = ["hello", " ", "world"];
        assert_eq!("hello world", stringify(string_slices.into_iter()));
        let strings = ["hello", " ", "world"].map(String::from);
        assert_eq!("hello world", stringify(strings.into_iter()));
    }

    assert_eq!(run_calculations(0, &[&AddFortyTwo, &DivideBy(10)]), 4);

    {
        let nums: Vec<_> = (0..40_320).collect();
        assert_eq!(812_831_040, adder(nums));
    }

    {
        let nums: Vec<_> = (0..10_000).collect();
        assert_eq!(49_995_000, adder_scoped(&nums));
    }

    {
        let nums: Vec<_> = (0..10_000).collect();
        assert_eq!(49_995_000, adder_rayon(&nums));
    }
}
