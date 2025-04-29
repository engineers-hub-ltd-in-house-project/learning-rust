fn main() {
    let num: u8 = 7;
    match num {
        1 => println!("{}月は、正月です。", num),
        2 => println!("{}月は、節分の月です。", num),
        3 => println!("{}月は、ひな祭りの月です。", num),
        4 => println!("{}月は、入学式があります。", num),
        5 => println!("{}月といえばゴールデンウィークです。", num),
        6 => println!("{}月は、梅雨です。", num),
        7 => println!("{}月は、夏休みが始まります。", num),
        8 => println!("{}月は、お盆休みです。", num),
        9 => println!("{}月は、新学期です。", num),
        10 => println!("{}月は、ハロウィンです。", num),
        11 => println!("{}月は、ブラックフライデーです。", num),
        12 => println!("{}月は、クリスマスです。", num),
        _ => println!("{}月という月はありません。", num),
    }
}
