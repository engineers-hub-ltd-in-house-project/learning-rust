fn main() {
    print_msg(100);
    print_msg(200);
    print_msg(300);
}

fn print_msg(max: i32) {
    println!("{} までの合計は、{} です。", max, calc(max));
}

fn calc(max: i32) -> i32 {
    let mut result = 0;
    for n in 0..max {
        result += n;
    }
    result
}
