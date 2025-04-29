fn main() {
    let max = 100;
    let mut ans = 0;
    for item in 1..=max {
        ans += item;
    }
    println!("1から{}までの合計は、{} です。", max, ans);
}
