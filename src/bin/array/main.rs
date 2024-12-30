fn main () {

    let numbers = [10, 20, 30, 40, 50];
    let mut updated_numbers = [0; 5];

    for i in 0..numbers.len() {
        updated_numbers[i] = numbers[i] + 5;
    }

    println!("Original array: {:?}", numbers);
    println!("Updated array: {:?}", updated_numbers);


}