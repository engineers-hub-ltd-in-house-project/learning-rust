struct Person {
    name: String,
    mail: String,
    age: i32,
}

fn person(name: String, mail: String, age: i32) -> Person {
    Person { name, mail, age }
}

fn print_person(p: Person) {
    println!("I'm {}({}). Mail to {}.", p.name, p.age, p.mail);
}

fn main() {
    let taro = person(String::from("Taro"), String::from("taro@yamada"), 39);
    let hanako = person(String::from("Hanako"), String::from("hanako@flower"), 28);
    print_person(taro);
    print_person(hanako);
}
