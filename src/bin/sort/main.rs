fn main () {

    let mut numbers = [60, 20, 30, 10, 50];

    println!("Original array: {:?}", numbers);

    for _ in 0..numbers.len() {
        for j in 0..numbers.len() - 1 {
            if numbers[j] > numbers[j + 1] {
                let temp = numbers[j];
                numbers[j] = numbers[j + 1];
                numbers[j + 1] = temp;
            }
        }
    }

    println!("Sorted array: {:?}", numbers);

}