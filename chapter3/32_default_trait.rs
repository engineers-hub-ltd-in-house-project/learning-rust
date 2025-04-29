trait Print {
    fn print(&self) {
        println!("PRINT is not yet implemented...");
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    mail: String,
    age: i32,
}

#[derive(Debug)]
struct Student {
    name: String,
    mail: String,
    grade: i32,
}

impl Print for Person {
    fn print(&self) {
        println!("{}<{}>({}).", self.name, self.mail, self.age);
    }
}

impl Print for Student {}

fn person(name: &str, mail: &str, age: i32) -> Person {
    Person {
        name: String::from(name),
        mail: String::from(mail),
        age: age,
    }
}

fn student(name: &str, mail: &str, grade: i32) -> Student {
    Student {
        name: String::from(name),
        mail: String::from(mail),
        grade,
    }
}

fn main() {
    let taro = person("Taro", "taro@yamada", 39);
    let hanako = student("Hanako", "hanako@flower", 2);
    taro.print();
    hanako.print();
}
