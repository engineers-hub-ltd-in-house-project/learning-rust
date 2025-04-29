fn main() {
    let taro = ("Taro", 39, true);
    let hanako = ("Hanako", 28, false);
    let (name, age, male) = taro;
    println!("name:{}, age:{}, male?:{}", name, age, male);
    let (name, age, male) = hanako;
    println!("name:{}, age:{}, male?:{}", name, age, male);
}
