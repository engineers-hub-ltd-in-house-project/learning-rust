/*
 * 文字列操作
 *
 * この例では、Rustの文字列（String型）の基本的な操作を示しています。
 * Rustには文字列リテラル（&str）とString型の2つの主な文字列型があります。
 * String型は可変長で、所有権を持ち、修正可能です。
 *
 * String型の特徴：
 * 1. ヒープに格納される可変長文字列
 * 2. UTF-8でエンコードされる
 * 3. 所有権の概念に従う
 * 4. 文字列の結合や修正が可能
 *
 * この例では、文字列の作成、結合、変換などの基本的な操作を示しています。
 */

fn main() {
    // 文字列の作成方法（型注釈を明示）
    let s1: String = String::from("こんにちは"); // String::fromで作成
    let s2: String = "ワールド".to_string(); // to_stringで作成

    // 文字列の結合
    let combined: String = s1 + &s2; // s1の所有権が移動することに注意
    println!("結合した文字列: {}", combined);

    // 新しい文字列を作成
    let s3: String = String::from("Rust");
    let s4: String = String::from("プログラミング");

    // formatマクロを使った結合（所有権を移動しない）
    let message: String = format!("{} {}は楽しい！", s3, s4);
    println!("{}", message);

    // s3とs4はまだ使用可能
    println!("s3: {}, s4: {}", s3, s4);

    // 文字列リテラル（&str型）の明示的な型注釈
    let greeting: &str = "こんばんは";
    println!("挨拶: {}", greeting);

    // 文字列の長さ（バイト数）を取得
    let bytes_len: usize = s3.len();
    println!("「{}」のバイト長: {}", s3, bytes_len);

    // 文字数を取得（日本語などのマルチバイト文字に対応）
    let char_count: usize = s4.chars().count();
    println!("「{}」の文字数: {}", s4, char_count);
}
