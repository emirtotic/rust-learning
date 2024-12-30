fn main() {

    let _X = {
        let price = 5;
        let quantity = 10;
        price * quantity
    };

    println!("Result is: {}", _X);

    let _SUM = sum(2, 3);

    println!("Result is: {}", _SUM);

    let _O = operations("*", 2, 3);

    println!("Operation result is: {}", _O);

    let _C = calculation("*", 2, 3);

    println!("Operation result is: {}", _C);

}


fn sum(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}


fn operations(operation: &str, num1: i32, num2: i32) -> i32 {
    if operation == "+" {
        return num1 + num2;
    } else if operation == "-" {
        return num1 - num2;
    } else if operation == "/" {
        return num1 / num2;
    } else if operation == "*" {
        return num1 * num2;
    } else {
        return 0;
    }
}

fn calculation(operation: &str, num1: i32, num2: i32) -> i32 {
    match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0 {
                num1 / num2
            } else {
                panic!("Division by zero is not allowed.");
            }
        },
        _ => panic!("Unsupported operation: {}", operation),
    }
}