fn main() {
    let max = 100;
    let mut ans = 0;
    let mut count = 1;
    while count <= max {
        ans += count;
        count += 1;
    }
    println!("1から{}までの合計は、{} です。", max, ans);
}
