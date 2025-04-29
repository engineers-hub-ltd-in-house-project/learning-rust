trait Print {
    fn print(&self);
}

struct Person {
    name: String,
    mail: String,
    age: i32,
}

impl Print for Person {
    fn print(&self) {
        println!("{}<{}>({}).", self.name, self.mail, self.age);
    }
}

fn person(name: String, mail: String, age: i32) -> Person {
    Person { name, mail, age }
}

struct Student {
    name: String,
    mail: String,
    grade: i32,
}

impl Print for Student {
    fn print(&self) {
        println!("grade{}: {}<{}>.", self.grade, self.name, self.mail);
    }
}

fn student(name: String, mail: String, grade: i32) -> Student {
    Student { name, mail, grade }
}

fn main() {
    let taro = person(String::from("Taro"), String::from("taro@yamada"), 39);
    let hanako = student(String::from("Hanako"), String::from("hanako@flower"), 2);
    taro.print();
    hanako.print();
}
