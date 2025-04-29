fn main() {
    let mut s1 = String::from("Hello,World!");
    s1.insert_str(6, " Rust ");
    s1.insert(7, '*');
    s1.insert(12, '*');
    s1.remove(5);
    println!("{}", s1);
}
