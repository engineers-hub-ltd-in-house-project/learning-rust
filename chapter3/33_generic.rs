#[derive(Debug)]
struct Sample<T> {
    name: String,
    value: T,
}

fn main() {
    let taro = Sample {
        name: String::from("Taro"),
        value: String::from("this is message."),
    };
    let hanako = Sample {
        name: String::from("Hanako"),
        value: 1234,
    };
    println!("{:?}", taro);
    println!("{:?}", hanako);
}
