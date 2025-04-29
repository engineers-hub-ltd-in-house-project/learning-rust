/*
 * 可変参照
 *
 * この例では、Rustの可変参照（mutable reference）の使い方を示しています。
 * 通常の参照は値を変更することができませんが、可変参照を使うと
 * 参照先の値を変更することができます。
 *
 * 可変参照の特徴：
 * 1. &mut キーワードで作成する
 * 2. 元の変数も mut で宣言されている必要がある
 * 3. 一度に一つの可変参照しか存在できない（排他的アクセス）
 * 4. 可変参照が存在する間は不変参照も作れない
 *
 * この例では、文字列を変更するために可変参照を使用しています。
 */

fn main() {
    // 可変文字列を作成（型注釈を明示）
    let mut message: String = String::from("こんにちは");
    println!("元のメッセージ: {}", message);

    // 可変参照を渡して文字列を変更
    modify_string(&mut message);

    // 変更後の文字列を表示
    println!("変更後のメッセージ: {}", message);

    // 数値の可変参照の例
    let mut count: i32 = 10;
    increment(&mut count, 5);
    println!("カウント: {}", count);
}

// 可変参照を引数に取る関数（型注釈を明示）
fn modify_string(s: &mut String) {
    // 可変参照を通じて値を変更する
    s.push_str("、世界！");
    s.insert(0, '【');
    s.push('】');
}

// 数値を増加させる関数
fn increment(value: &mut i32, amount: i32) {
    *value += amount; // 参照外し演算子（*）で値にアクセスして変更
}
