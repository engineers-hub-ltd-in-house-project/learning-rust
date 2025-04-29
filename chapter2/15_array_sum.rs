fn main() {
    let data = [12, 34, 56, 78, 90];
    let mut ans = 0;
    for item in data {
        ans += item;
    }
    println!("データの合計は、{} です。", ans);
}
