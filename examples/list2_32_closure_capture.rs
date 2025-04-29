fn main() {
    let max = 100;
    let res = calc(max);
    let print_msg = || {
        println!("{} までの合計は、{} です。", max, res);
    };
    print_msg();

    let max = 200;
    let res = calc(max);
    let print_msg = || {
        println!("0-{} Total: {}", max, res);
    };
    print_msg();
}

fn calc(max: i32) -> i32 {
    let mut result = 0;
    for n in 0..max {
        result += n;
    }
    result
}
