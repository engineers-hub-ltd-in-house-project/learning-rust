/*
 * Optionのベクタ
 *
 * この例では、Option型の値をベクタに格納して扱う方法を示しています。
 * Option<T>は値が存在するかどうかを表現するRustの基本的な列挙型で、
 * Some(値)またはNoneのいずれかになります。
 *
 * このパターンの利点：
 * 1. 存在しない可能性のある値を安全に扱える
 * 2. コレクション内の欠損値を明示的に表現できる
 * 3. パターンマッチングによる安全な処理ができる
 *
 * この例では、Some(値)とNoneを含むベクタを作成し、
 * その要素を順に処理しています。
 */

fn main() {
    // Option型の値を持つベクタを作成
    let values: Vec<Option<i32>> = vec![Some(1), None, Some(3), Some(4), None, Some(6)];

    // ベクタの要素を順に処理
    for (index, value) in values.iter().enumerate() {
        match value {
            Some(num) => println!("インデックス {}: 値 = {}", index, num),
            None => println!("インデックス {}: 値なし", index),
        }
    }

    // OptionのSome値のみを抽出して新しいベクタを作成
    let filtered: Vec<i32> = values.iter().filter_map(|opt| opt.clone()).collect();

    println!("フィルタ後のベクタ: {:?}", filtered);
}
