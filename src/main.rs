use std::io;

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

fn main() {
    // Note that using unwrap() here isn't the most user friendly way to show errors but hey,
    // this is just a little calculator.
    let operation = get_operation_input("Choose an operation: add, sub, mul, div").unwrap();
    let num1 = get_number_input("Enter first number:").unwrap();
    let num2 = get_number_input("Enter second number:").unwrap();

    let result = calculate(operation, num1, num2);

    println!("The result is {}", result);
}

fn get_operation_input(prompt: &str) -> Result<Operation, &'static str> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    return match input.trim() {
        "add" => Ok(Operation::Add),
        "sub" => Ok(Operation::Sub),
        "mul" => Ok(Operation::Mul),
        "div" => Ok(Operation::Div),
        _ => Err("Invalid operation"),
    };
}

fn get_number_input(prompt: &str) -> Result<f64, &'static str> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().parse().map_err(|_| "Invalid number");
}

fn calculate(operation: Operation, num1: f64, num2: f64) -> f64 {
    return match operation {
        Operation::Add => num1 + num2,
        Operation::Sub => num1 - num2,
        Operation::Mul => num1 * num2,
        Operation::Div => num1 / num2,
    };
}
