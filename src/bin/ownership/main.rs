fn main() {
    println!("Hello");

    let s1 = String::from("RUST");
    let length = calculate_length(&s1); // Borrowing a reference from s1!

    // There can be only one owner at the time!
    // When the owner goes out of scope, the value will be dropped.

    println!("Length of the String {} string is {}", s1, length);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn test(s: &string) {

    // cannot find value `s1` in this scope
    // s1 can not be found because it is declared in main, and it is now available out of it
    // To avoid this, call the test method in main and do test(&s1); (Paste it as a reference)

    println!("{}", &s1);
}