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
    // 文字列の作成方法
    let s1 = String::from("こんにちは");  // String::fromで作成
    let s2 = "ワールド".to_string();      // to_stringで作成
    
    // 文字列の結合
    let combined = s1 + &s2;  // s1の所有権が移動することに注意
    println!("結合した文字列: {}", combined);
    
    // 新しい文字列を作成
    let s3 = String::from("Rust");
    let s4 = String::from("プログラミング");
    
    // formatマクロを使った結合（所有権を移動しない）
    let message = format!("{} {}は楽しい！", s3, s4);
    println!("{}", message);
    
    // s3とs4はまだ使用可能
    println!("s3: {}, s4: {}", s3, s4);
}
