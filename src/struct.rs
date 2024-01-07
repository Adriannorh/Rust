// Defining a struct. structs are like classes in other languages
#[derive(Debug)]

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main() {
    println!("{}", fibonacci(10));
    println!(
        "{:?}",
        Person {
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            age: 30,
        }
    );
}

// write a fibonacci function that takes a number n and returns the nth fibonacci number
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
