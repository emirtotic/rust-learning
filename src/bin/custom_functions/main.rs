use std::io;

fn main() {
    println!("Let's do some functions!");
    let mut input = String::new();
    println!("Enter operation (+, -, *, /): ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let operation = input.trim();

    let result = match operation {
        "+" => sum(10, 10),
        "-" => subtraction(10, 10),
        "*" => multiplication(10, 10),
        "/" => division(10, 10),
        _ => panic!("Unsupported operation: {}", operation),
    };

    println!("The result is: {}", result);
}

fn sum(num1: i32, num2: i32) -> i32 {
    println!("Choice is: addition");
    num1 + num2
}

fn subtraction(num1: i32, num2: i32) -> i32 {
    println!("Choice is: subtraction");
    num1 - num2
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("Choice is: multiplication");
    num1 * num2
}

fn division(num1: i32, num2: i32) -> i32 {
    if num2 != 0 {
        println!("Choice is: division");
        num1 / num2
    } else {
        panic!("Division by zero is not allowed.");
    }
}
