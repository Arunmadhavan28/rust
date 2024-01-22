use std::io;

fn main() {
    println!("This is a sample calculator");

    println!("Enter the first number:");
    let x: f64 = read_input();

    println!("Enter the second number:");
    let y: f64 = read_input();

    println!("Choose the action:");
    println!("1. Addition (+)");
    println!("2. Subtraction (-)");
    println!("3. Multiplication (*)");
    println!("4. Division (/)");

    let operation_choose: u32 = read_input();

    let result = match operation_choose {
        1 => x + y,
        2 => x - y,
        3 => x * y,
        4 => {
            if y != 0.0 {
                x / y
            } else {
                println!("Error: Division by zero!");
                return;
            }
        }
        _ => {
            println!("Invalid operation choice!");
            return;
        }
    };

    println!("The result is: {}", result);
}

fn read_input<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input! Please enter a valid number.");
            read_input()
        }
    }
}
