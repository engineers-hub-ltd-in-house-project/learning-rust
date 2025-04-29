fn main() {
    let mut data: Vec<i32> = vec![123, 456, 789];
    data.remove(1);
    data.insert(2, 100);
    println!("{:?} ", data);
}
