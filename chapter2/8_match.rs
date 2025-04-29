/*
 * match式の基本
 *
 * この例では、Rustのmatch式について紹介しています。
 * match式はC言語のswitch文に似ていますが、より強力です。
 * 全てのパターンを網羅する必要があり、コンパイラが確認します。
 *
 * この例では、数値に対するmatch処理とenumに対するmatch処理の両方を示しています。
 * enumと組み合わせることで、型安全なパターンマッチングが実現できます。
 *
 * 特徴：
 * 1. 値に応じて異なる処理を実行できる
 * 2. 全てのケースを網羅する必要がある
 * 3. 複数の値を一つのケースで処理できる
 * 4. match式自体が値を返せる
 */

// 列挙型の定義
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    // 数値のmatch
    let num = 5;
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

    // 列挙型のmatch
    let coin = Coin::Quarter;
    let value = value_in_cents(coin);
    println!("コインの価値は {} セントです", value);
}

// 列挙型を受け取り、値を返す関数
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
