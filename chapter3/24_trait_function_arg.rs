fn main() {
    let taro = person(String::from("Taro"), String::from("taro@yamada"), 39);
    let hanako = student(String::from("Hanako"), String::from("hanako@flower"), 2);
    print(taro);
    print(hanako);
}

fn print(ob: impl Print) {
    ob.print();
}
