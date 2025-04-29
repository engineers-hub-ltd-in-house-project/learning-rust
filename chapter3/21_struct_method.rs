struct Person {
    name: String,
    mail: String,
    age: i32,
}

fn person(name: String, mail: String, age: i32) -> Person {
    Person { name, mail, age }
}

impl Person {
    fn print(&self) {
        println!("{}<{}>({}).", self.name, self.mail, self.age);
    }
}

fn main() {
    let taro = person(String::from("Taro"), String::from("taro@yamada"), 39);
    let hanako = person(String::from("Hanako"), String::from("hanako@flower"), 28);
    taro.print();
    hanako.print();
}
