fn main() {


    let _x: bool = false;
    let mut _num: i64 = 1;

    while _x == false {
        println!("{}. Hello World!", _num);
        _num += 100;

        if _num == 4733001 {
            println!("Izlaz!");
            break;
        }
    }



}