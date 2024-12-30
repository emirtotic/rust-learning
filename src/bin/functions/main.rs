fn main() {
    greeting("Emir", 32);
    human_id("1111111", "Belgrade");
}

fn greeting(name: &str, age: u32) {
    println!("Hello, I am {} and I am {} years old. :)", name, age);
}

fn human_id(jmbg: &str, city: &str) {
    println!("I am from {} and my JMBG is {}", city, jmbg)
}

// Run in terminal: cargo run --bin functions
