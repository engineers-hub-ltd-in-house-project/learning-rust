fn main() {
    let data: [i32; 5] = [12, 34, 56, 78, 90];
    let mut ans: i32 = 0;
    for item in data {
        ans += item;
    }
    println!("データの合計は、{} です。", ans);
}
