fn main() {
    let taro = person("Taro", "taro@yamada", 39);
    let hanako = student("Hanako", "hanako@flower", 2);
    let jiro = person("Jiro", "jiro@change", 28);
    let sachiko = student("Sachiko", "sachiko@happy", 4);
    let data: Vec<Box<dyn Print>> = vec![taro, hanako, jiro, sachiko];
    print_all(data);
}
