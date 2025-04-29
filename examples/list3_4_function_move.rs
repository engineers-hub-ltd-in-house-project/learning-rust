fn main() {
    let msg = String::from("Hello!");
    print_msg(msg);
    println!("msg: {}", msg);
}

fn print_msg(msg: String) {
    println!("Message is {}", msg);
}
