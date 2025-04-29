/*
 * ハッシュマップの使用
 *
 * この例では、Rustのハッシュマップ（HashMap<K, V>）の基本的な使い方を示しています。
 * ハッシュマップはキーと値のペアを格納するコレクションで、キーを使って値を素早く検索できます。
 *
 * ハッシュマップの特徴：
 * 1. キーと値のペアを格納する
 * 2. キーによる高速な検索が可能
 * 3. キーは一意でなければならない（同じキーは1つしか存在できない）
 * 4. 標準ライブラリのstd::collections::HashMapを使用する
 *
 * この例では、ハッシュマップの作成、要素の挿入、キーによる値の取得を示しています。
 */

use std::collections::HashMap;

fn main() {
    // 新しいハッシュマップを作成（型注釈を明示）
    let mut scores: HashMap<&str, i32> = HashMap::new();

    // キーと値のペアを挿入
    scores.insert("Alice", 98);
    scores.insert("Bob", 85);
    scores.insert("Charlie", 92);

    // キーで値を取得
    if let Some(score) = scores.get("Bob") {
        println!("Bobのスコア: {}", score);
    }

    // 存在しないキーの場合
    match scores.get("Dave") {
        Some(score) => println!("Daveのスコア: {}", score),
        None => println!("Daveのスコアはありません"),
    }

    // 全てのキーと値を表示
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // 型を明示した別のハッシュマップ（String型のキーと値）
    let mut user_data: HashMap<String, String> = HashMap::new();
    user_data.insert(String::from("email"), String::from("user@example.com"));
    user_data.insert(String::from("username"), String::from("rust_fan"));

    println!("\nユーザーデータ:");
    for (key, value) in &user_data {
        println!("{}: {}", key, value);
    }
}
