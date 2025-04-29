fn main() {
    const TARO: (&str, i32, bool) = ("Taro", 39, true);
    const HANAKO: (&str, i32, bool) = ("Hanako", 28, false);
    let (name, age, male) = TARO;
    println!("name:{}, age:{}, male?:{}", name, age, male);
    let (name, age, male) = HANAKO;
    println!("name:{}, age:{}, male?:{}", name, age, male);
}
