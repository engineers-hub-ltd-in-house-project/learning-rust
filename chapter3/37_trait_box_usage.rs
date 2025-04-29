fn print_all<T: Print + ?Sized>(data: Vec<Box<T>>) {
    for item in data {
        item.print();
    }
}

fn main() {
    let taro = person("Taro", "taro@yamada", 39);
    let hanako = student("Hanako", "hanako@flower", 2);
    let jiro = person("Jiro", "jiro@change", 28);
    let sachiko = student("Sachiko", "sachiko@happy", 4);
    let data_p: Vec<Box<Person>> = vec![taro, jiro];
    let data_s: Vec<Box<Student>> = vec![hanako, sachiko];
    print_all(data_p);
    print_all(data_s);
}
