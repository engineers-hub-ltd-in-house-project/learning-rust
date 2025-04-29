fn main() {
    let data = vec![123, 456, 789];
    let mut result = 0;
    for item in data {
        result += item;
    }
    println!("データの合計は、{} です。", result);
}
