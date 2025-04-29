fn main() {
    hello(String::from("taro"));
    hello(String::from("hanako"));
}

fn hello(name: String) {
    println!("Hello, {}!", name);
}
