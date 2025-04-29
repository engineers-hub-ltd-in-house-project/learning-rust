fn main() {
    let mut x = 10;
    let mut double = || {
        x *= 2;
        x
    };
    println!("x = {}.", double());
    println!("x = {}.", double());
    println!("x = {}.", double());
}
