fn main() {
    let data: Vec<i32> = vec![123, 456, 789];
    let mut result: i32 = 0;
    for item in data {
        result += item;
    }
    println!("データの合計は、{} です。", result);
}
