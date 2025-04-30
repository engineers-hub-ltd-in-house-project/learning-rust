/*
 * トレイト境界を使用した参照渡しの例
 *
 * このサンプルでは、トレイト境界を使用して特定のインターフェース（Printable）を
 * 実装している型のみを受け付ける汎用関数を定義しています。
 *
 * Rustのトレイト境界を使うことで、型安全性を保ちながら柔軟な関数を定義できます。
 * これはジェネリクスとトレイトを組み合わせたRustの強力な機能の一つです。
 */

// 表示可能なことを示すトレイトを定義
// このトレイトは特定の型が「表示可能」であることを保証するためのインターフェース
trait Printable {
    // 自身を文字列として整形するメソッド
    fn format(&self) -> String;
}

// String型に対してPrintableトレイトを実装
// これにより、String型のオブジェクトはPrintableとしての機能を持つようになります
impl Printable for String {
    fn format(&self) -> String {
        // 「文字列: 」という接頭辞を付けて返す
        format!("文字列: {}", self)
    }
}

fn main() {
    // 文字列オブジェクトを作成
    let taro = String::from("太郎");
    let hanako = String::from("花子");

    // print関数に参照を渡す
    // &を使って参照渡しをしています
    print(&taro);
    print(&hanako);
}

// トレイト境界を使って、Printableトレイトを実装している型のみを受け付ける
// <T: Printable>は、「T型はPrintableトレイトを実装していなければならない」という制約
fn print<T: Printable>(obj: &T) {
    // objの参照からformat()メソッドを呼び出し、結果を表示
    println!("{}", obj.format());
}
