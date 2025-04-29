fn main() {
    let mut msg = String::from("Hello, world!");

    let world = String::from(&msg[7..12]);
    println!("`{}` in `{}`.", world, msg);
    msg.insert_str(7, "RUST?");

    let mut world = String::from(&msg[7..12]);
    world.push('!');
    println!("`{}` in `{}`.", world, msg);
}
