struct Person {
    name: String,
    mail: String,
    age: i32,
}

fn person(name: String, mail: String, age: i32) -> Person {
    Person { name, mail, age }
}
