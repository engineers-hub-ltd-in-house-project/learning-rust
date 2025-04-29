fn main() {
    let msg = "Hello!";
    let msg_p = &msg;
    let msg_v = *msg_p;
    println!("{}, {}, {}.", msg, msg_p, msg_v);
}
