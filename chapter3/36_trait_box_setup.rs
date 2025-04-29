trait Print {
    fn print(&self) {
        println!("not implemented...");
    }
}

struct Person {
    name: String,
    mail: String,
    age: i32,
}

struct Student {
    name: String,
    mail: String,
    grade: i32,
}

impl Print for Person {
    fn print(&self) {
        println!("Person: {}<{}>({})", &self.name, &self.mail, &self.age);
    }
}
impl Print for Student {
    fn print(&self) {
        println!(
            "Student [grade {}] {}<{}>",
            &self.grade, &self.name, &self.mail
        );
    }
}

fn person(name: &str, mail: &str, age: i32) -> Box<Person> {
    Box::new(Person {
        name: String::from(name),
        mail: String::from(mail),
        age: age,
    })
}

fn student(name: &str, mail: &str, grade: i32) -> Box<Student> {
    Box::new(Student {
        name: String::from(name),
        mail: String::from(mail),
        grade,
    })
}
