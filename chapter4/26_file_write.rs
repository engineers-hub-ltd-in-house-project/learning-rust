/*
 * ファイル書き込み
 *
 * この例では、Rustでファイルに文字列データを書き込む基本的な方法を示しています。
 * std::fs::Fileモジュールとstd::io::Writeトレイトを使用してファイル操作を行います。
 *
 * ファイル書き込みの特徴：
 * 1. File::create関数で新しいファイルを作成（既存ファイルは上書き）
 * 2. write_allメソッドでバイト列をファイルに書き込む
 * 3. 文字列をバイト列に変換するにはas_bytesメソッドを使用
 * 4. ファイル操作はI/Oエラーの可能性があるためResult型を返す
 *
 * この例では、テキストファイルを作成して文字列データを書き込んでいます。
 */

use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // 書き込む内容を準備
    let content = "Hello, Rust!\nファイル操作を学んでいます。\n複数行のテキストも書き込めます。";

    // ファイルを作成（既存のファイルは上書き）
    let mut file = File::create("output.txt")?;

    // 文字列をバイト列に変換してファイルに書き込む
    file.write_all(content.as_bytes())?;

    println!("ファイルへの書き込みが完了しました。");

    // 正常終了
    Ok(())
}
