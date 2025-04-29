fn main() {
    let calc = |max| {
        let mut result = 0;
        for n in 0..max {
            result += n;
        }
        result
    };

    let print_msg = |max| {
        println!("{} までの合計は、{} です。", max, calc(max));
    };

    print_msg(100);
    print_msg(200);
    print_msg(300);
}
