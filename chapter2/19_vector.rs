/*
 * ベクタの基本
 *
 * この例では、Rustのベクタ（Vec<T>）の基本的な使い方を示しています。
 * ベクタは同じ型の要素を可変長で格納できるコレクションです。
 *
 * ベクタの特徴：
 * 1. 同じ型の値を複数格納できる
 * 2. サイズが動的に変更可能（追加・削除ができる）
 * 3. メモリ上で連続して配置される
 * 4. インデックスによるアクセスが高速
 *
 * この例では、ベクタの作成、要素の追加、インデックスによるアクセスを示しています。
 */

fn main() {
    // 空のベクタを作成（型注釈を明示）
    let mut numbers: Vec<i32> = Vec::new();

    // 要素を追加
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    // インデックスでアクセス
    println!("最初の要素: {}", numbers[0]);
    println!("2番目の要素: {}", numbers[1]);
    println!("3番目の要素: {}", numbers[2]);

    // ベクタの長さを取得
    let length: usize = numbers.len();
    println!("ベクタの長さ: {}", length);

    // ベクタの全要素を表示
    println!("ベクタの内容: {:?}", numbers);

    // vec!マクロを使った初期化（型注釈付き）
    let fibonacci: Vec<i32> = vec![1, 1, 2, 3, 5, 8, 13, 21];
    println!("フィボナッチ数列: {:?}", fibonacci);
}
