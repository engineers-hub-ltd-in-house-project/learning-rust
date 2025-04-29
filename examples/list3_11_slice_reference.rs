fn main() {
    let mut msg = String::from("Hello, world!");
    let world = &msg[7..12];
    println!("`{}` in `{}`.", world, msg);
    msg.insert_str(7, "RUST?");
    println!("`{}` in `{}`.", world, msg);
}
