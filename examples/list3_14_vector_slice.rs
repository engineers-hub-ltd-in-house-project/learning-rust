fn main() {
    let mut data = vec![12, 34, 56, 78, 90];
    let part = &data[2..4];
    data.insert(1, 999);
    println!("{:?} in {:?}", part, data);
}
