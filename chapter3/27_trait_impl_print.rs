fn person(name: &str, mail: &str, age: i32) -> impl Print {
    Person {
        name: String::from(name),
        mail: String::from(mail),
        age: age,
    }
}

fn student(name: &str, mail: &str, grade: i32) -> impl Print {
    Student {
        name: String::from(name),
        mail: String::from(mail),
        grade,
    }
}
