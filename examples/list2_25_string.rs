fn main() {
    let s1 = String::new();
    let s2 = String::from("Hello");
    let s3 = "World";
    let s4 = s1 + &s2 + &s3;
    println!("{}", s4);
}
