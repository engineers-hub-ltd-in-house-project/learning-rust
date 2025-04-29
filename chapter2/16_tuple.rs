fn main() {
    let taro = ("Taro", 39, true);
    let hanako = ("Hanako", 28, false);
    println!("{:?}", taro);
    println!("{:?}", hanako);
    println!("name: {}, {}", taro.0, hanako.0);
    println!("age: {}, {}", taro.1, hanako.1);
    println!("male?: {}, {}", taro.2, hanako.2);
}
