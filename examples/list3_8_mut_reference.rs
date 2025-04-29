fn main() {
    let mut msg = String::from("Hello!");
    println!("msg: {}", msg);
    print_msg(&mut msg);
    println!("msg: {}", msg);
}

fn print_msg(msg: &mut String) {
    msg.push_str("!!!!");
    println!("Message is \"{}\".", msg);
}
