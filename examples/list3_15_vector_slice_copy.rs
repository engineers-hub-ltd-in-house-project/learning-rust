fn main() {
    let mut data = vec![12, 34, 56, 78, 90];
    let mut part = data[2..4].to_vec();
    data.insert(3, 999);
    part.push(-1);
    println!("{:?} in {:?}", part, data);
}
