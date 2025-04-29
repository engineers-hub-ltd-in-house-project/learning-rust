/*
 * 構造体の基本
 *
 * この例では、Rustの構造体（struct）の基本的な使い方を示しています。
 * 構造体は複数の異なる型のデータをまとめて一つの意味のある型を作成します。
 *
 * 特徴:
 * 1. フィールド名とその型を明示的に定義します
 * 2. インスタンス作成時にはすべてのフィールドに値を設定する必要があります
 * 3. フィールドにはドット記法（person.name）でアクセスできます
 * 4. 構造体はデフォルトで不変ですが、mut修飾子で可変にできます
 *
 * この例では、Person構造体を定義し、二つのインスタンスを作成して
 * それぞれのフィールドにアクセスする方法を示しています。
 */

// Person構造体の定義
struct Person {
    name: String,
    mail: String,
    age: i32,
}

fn print_person(p: Person) {
    println!("I'm {}({}). Mail to {}.", p.name, p.age, p.mail);
}

fn main() {
    // イミュータブルな構造体インスタンスの作成
    let taro = Person {
        name: String::from("Taro"),
        mail: String::from("taro@yamada"),
        age: 39,
    };
    let hanako = Person {
        name: String::from("Hanako"),
        mail: String::from("hanako@flower"),
        age: 28,
    };
    print_person(taro);
    print_person(hanako);
}
