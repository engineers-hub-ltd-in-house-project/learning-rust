fn main() {
    let msg = &String::from("Hello!");
    println!("msg: {}", msg);
    {
        let msg = print_msg(msg);
        println!("msg: {}", msg);
    }
    println!("msg: {}", msg);
}

fn print_msg(msg: &String) -> String {
    let msg = String::from("*** ") + msg + " ***";
    println!("Message is \"{}\".", msg);
    msg
}
