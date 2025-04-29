/*
 * ファイル読み込み
 *
 * この例では、Rustの標準ライブラリを使用してファイルからテキストを読み込む方法を示しています。
 * ファイル操作は失敗する可能性があるため、Result型を返す関数を使用し適切なエラーハンドリングが必要です。
 *
 * ファイル読み込みの主なステップ：
 * 1. std::fs::Fileの関数でファイルを開く
 * 2. 読み込んだ内容を文字列として格納
 * 3. エラーハンドリングを行う
 *
 * Rustではファイル操作が失敗した場合に明示的なエラーハンドリングが必要であり、
 * これによりプログラムの堅牢性が向上します。
 */

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        count += 1;
        let txt = line.unwrap();
        println!("{}: {}", count, txt);
    }
}
