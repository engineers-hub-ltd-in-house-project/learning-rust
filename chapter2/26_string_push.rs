/*
 * 文字列追加
 *
 * この例では、Rustの文字列（String型）に対して文字列を追加する操作を示しています。
 * push_str()メソッドを使うと、既存の文字列に新しい文字列を追加できます。
 *
 * 文字列追加の特徴：
 * 1. 文字列は可変（mut）である必要がある
 * 2. push_str()メソッドを使って文字列を追加する
 * 3. 追加操作は元の文字列を変更する（所有権は移動しない）
 * 4. 追加した後も元の変数で文字列を参照できる
 *
 * この例では新しい空の文字列を作成し、そこに連続して文字列を追加しています。
 */

fn main() {
    let mut s1: String = String::new();
    s1.push_str("Hello");
    s1.push_str("World!");
    println!("{}", s1);
}
