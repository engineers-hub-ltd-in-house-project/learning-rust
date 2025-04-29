fn main() {
    let mut data = Vec::new();
    data.push(123);
    data.push(456);
    data.push(789);
    println!("0:{}, 1:{}, 2:{}.", data[0], data[1], data.get(2).unwrap());
}
