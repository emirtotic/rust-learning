fn main() {

    let _x: i32 = 5;

    println!("Value is {}", _x);

    let _x = _x + 1;

    println!("Value is {}", _x);

    let _x = _x + 1;

    println!("Value is {}", _x);

    let _x = _x + 1;

    println!("Value is {}", _x);

    {
        let _x = _x * 2;
        println!("Value in this code block is {}", _x);
    }

    println!("Value out of scope is not affected by code block and it is {}", _x);

}