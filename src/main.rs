fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Number array: {:?}", numbers);

    let mix = [10, 20, 30, 40];
    println!("Mix array: {:?}", mix);

    for i in mix {
        println!("{}", i);
    }

    let fruits: [&str; 3] = ["apple", "bannana", "orange"];
    println!("Fruits:  {:?}", fruits);
    println!("Fruits 1st:  {}", fruits[0]);
    println!("Fruits 2nd:  {}", fruits[1]);
    println!("Fruits 3rd:  {}", fruits[2]);

    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 32, true, [1, 2, 3, 4, 5]);
    println!("My mix tuple: {:?}", my_mix_tuple);

    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("My number slice: {:?}", number_slices);

    let animal_slice: &[&str] = &["Monkey", "Donkey"];
    println!("My animal slice: {:?}", animal_slice);

    let animal_slice_string: &[&String] = &[&"Monkey".to_string(), &"Donkey".to_string()];
    println!("My animal slice string: {:?}", animal_slice_string);

    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone cold says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone cold says: {}", stone_cold);

    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}
