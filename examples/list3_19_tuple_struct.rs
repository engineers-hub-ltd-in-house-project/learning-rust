struct Person(String, String, i32);

fn print_person(p: Person) {
    println!("I'm {}({}). Mail to {}.", p.0, p.2, p.1);
}

fn main() {
    let taro = Person(String::from("Taro"), String::from("taro@yamada"), 39);
    let hanako = Person(String::from("Hanako"), String::from("hanako@flower"), 28);
    print_person(taro);
    print_person(hanako);
}
