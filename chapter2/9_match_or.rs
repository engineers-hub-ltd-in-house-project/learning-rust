fn main() {
    let num: u8 = 7;
    match num {
        1 => println!("{}月は、正月です。", num),
        2 => println!("{}月は、冬です。", num),
        3 | 4 | 5 => println!("{}月は、春です。", num),
        6 | 7 | 8 => println!("{}月は、夏です。", num),
        9 | 10 | 11 => println!("{}月は、秋です。", num),
        12 => println!("{}月は、師走です。", num),
        _ => println!("{}月という月はありません。", num),
    }
}
