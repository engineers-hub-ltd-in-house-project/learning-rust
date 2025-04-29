fn main() {
    let mut msg = String::from("Hello!");
    msg = print_msg(msg);
    println!("msg: {}", msg);
}

fn print_msg(msg: String) -> String {
    println!("Message is \"{}\".", msg);
    msg
}
