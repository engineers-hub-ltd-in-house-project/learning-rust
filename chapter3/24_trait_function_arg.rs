/*
 * トレイトを引数に取る関数
 *
 * この例では、トレイトを実装した型を関数の引数として受け取る方法を示しています。
 * Rustでは、impl Trait構文を使って、特定のトレイトを実装する型を引数として指定できます。
 *
 * 特徴:
 * 1. 異なる型に対して同じインターフェースで操作できる
 * 2. コードの再利用性を高める
 * 3. 複数の型を単一の関数で扱える
 *
 * この例では、Printトレイトを実装するどんな型でも引数として受け取る
 * print関数を定義し、personとstudentの両方のインスタンスを渡しています。
 */

fn main() {
    let taro = person(String::from("Taro"), String::from("taro@yamada"), 39);
    let hanako = student(String::from("Hanako"), String::from("hanako@flower"), 2);
    print(taro);
    print(hanako);
}

// Printトレイトを実装した型を引数に取る関数
fn print(ob: impl Print) {
    ob.print();
}
