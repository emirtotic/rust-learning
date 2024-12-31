use std::io;

fn main() {

    let mut number = String::new();
    println!("Enter the number:");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    match number.trim().parse::<u32>() {
        Ok(number) => {
            if number % 2 == 0 {
                println!("{} is an even number.", number);
            } else {
                println!("{} is an odd number.", number);
            }
        }
        Err(_) => println!("Invalid input. Please enter a valid number."),
    }


}