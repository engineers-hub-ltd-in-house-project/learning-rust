fn main() {
    let taro = person("Taro", "taro@yamada", 39);
    let hanako = student("Hanako", "hanako@flower", 2);
    print(&taro);
    print(&hanako);
}

fn print(ob: &Box<dyn Print>) {
    ob.print();
}
