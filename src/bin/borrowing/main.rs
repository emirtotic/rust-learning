fn main() {

    let mut _x = 5;
    let _y: &mut i32 = &mut _x;

    *_y += 1;

    println!("x = {}", _x );



}