fn person(name: &str, mail: &str, age: i32) -> Box<dyn Print> {
    Box::new(Person {
        name: String::from(name),
        mail: String::from(mail),
        age: age,
    })
}

fn student(name: &str, mail: &str, grade: i32) -> Box<dyn Print> {
    Box::new(Student {
        name: String::from(name),
        mail: String::from(mail),
        grade,
    })
}
