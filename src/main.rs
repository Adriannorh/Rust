/*
fn main() {
    let message = "Hello, Adrian ";
    let weigiht = 20.0;

    let kilos = weigiht / 2.0;
    println!("{}{}", message, weigiht);
    println!("{}{}", message, kilos)
} */

/* fn main() {
    let process = false;
    if process {
        println!("Proceeding");
    } else {
        println!("Not proceeding")
    }

    let height = 190;
    if height > 180 {
        println!("Tall");
    } else if height > 170 {
        println!("Medium");
    } else {
        println!("Short");
    }

    let age = 0;
    if age > 18 {
        println!("Adult");
    } else if age > 12 {
        println!("Teenager");
    } else {
        println!("Child");
    }
}
 */

/* fn main() {
    let mut height = 190;
    height = height - 20;
    let result = if height < 180 {
        "Tall"
    } else if height > 170 {
        "Medium"
    } else {
        "Short"
    };

    println!("Result is {}", result);

    let health = if height < 180 { "Good" } else { "Bad" };
    println!("Health is {}", health);

    let health = if height < 180 { true } else { false };
} */

// remember to use break to exit the loop!!
/* fn main() {
    let mut x = 1;
    loop {
        println!("x is {}", x);
        x += 1;
        if x > 10 {
            break;
        }
    }
} */

/* fn main() {
    let maybe_number = Some(42);
    if let Some(number) = maybe_number {
        println!("number is {}", number);
    } else {
        println!("number is not present");
    }
} */
/*
fn main() {
    for i in 1..11 {
        println!("i is {}", i)
    }
} */
/*
fn main() {
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        }
        println!("i is {}", i);
        if i == 7 {
            break;
        }
    }
} */
/* use std::io;

fn main() {
    println!("Please enter a  greeting");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    match name.trim() {
        "Good bye" => println!("Good bye to you too!"),
        "hello" => println!("Hello to you too!"),
        _ => println!("I don't understand you"),
    }
} */

// Spilt string
/*
fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);
    result.expect("oops! something went wrong").to_string()
}

fn main() {
    let chunk = split_string("Hello, World".to_string(), ',', 1);
    println!("chunk is {}", chunk)
}
 */

// sum of numbers
/* use std::io::{self, Write};

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    let mut numbers = String::new();

    print!("Please enter numbers separated by spaces: ");
    io::stdout().flush().unwrap(); // Flush the output

    io::stdin()
        .read_line(&mut numbers)
        .expect("Failed to read line");

    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();

    let results = sum(&numbers);
    println!("Result is {}", results);
} */

fn loop_and_panic(numbers: Vec<i32>) {
    for number in numbers {
        if number == 0 {
            panic!("Zero is not a valid number");
        }
        println!("The reciprocal of {} is {}", number, 1 / number);
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 0, -5];
    loop_and_panic(numbers);
}
