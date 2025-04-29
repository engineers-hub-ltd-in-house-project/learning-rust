fn sample<T>(name: &str, value: T) -> Sample<T> {
    Sample {
        name: String::from(name),
        value: value,
    }
}

fn main() {
    let taro = sample("Taro", "this is message.");
    let hanako = sample("Hanako", 1234);
    println!("{:?}", taro);
    println!("{:?}", hanako);
}
