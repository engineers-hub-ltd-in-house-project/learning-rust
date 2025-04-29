fn main() {
    let taro: (&str, i32, bool) = ("Taro", 39, true);
    let hanako: (&str, i32, bool) = ("Hanako", 28, false);
    let (name, age, male): (&str, i32, bool) = taro;
    println!("name:{}, age:{}, male?:{}", name, age, male);
    let (name, age, male): (&str, i32, bool) = hanako;
    println!("name:{}, age:{}, male?:{}", name, age, male);
}
