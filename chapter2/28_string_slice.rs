fn main() {
    let s1 = String::from("Hello,Rust World!");
    let s2 = &s1[0..5];
    let s3 = &s1[6..10];
    let s4 = &s1[11..16];
    let s5 = String::new() + s4 + s3 + s2;
    println!("{}", s5);
}
