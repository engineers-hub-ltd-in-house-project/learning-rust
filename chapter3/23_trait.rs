/*
 * トレイトの実装
 *
 * トレイトは、Rustでインターフェースに相当する機能です。
 * 複数の型に共通の振る舞いを定義することができます。
 * 
 * この例では：
 * 1. Printableトレイトを定義し、print()メソッドを要求しています
 * 2. 異なる構造体（PersonとBlog）にこのトレイトを実装しています
 * 3. 各構造体は独自の方法でprint()メソッドを実装しています
 * 
 * トレイトを使うメリット：
 * - コードの再利用性が高まる
 * - 異なる型に共通のインターフェースを提供できる
 * - ジェネリックプログラミングの基盤となる
 * - コードの抽象化レベルを上げられる
 */

// プリント可能なことを表すトレイト
trait Printable {
    fn print(&self);
}

struct Person {
    name: String,
    mail: String,
    age: i32,
}

impl Printable for Person {
    fn print(&self) {
        println!("{}<{}>({}).", self.name, self.mail, self.age);
    }
}

fn person(name: String, mail: String, age: i32) -> Person {
    Person { name, mail, age }
}

struct Student {
    name: String,
    mail: String,
    grade: i32,
}

impl Printable for Student {
    fn print(&self) {
        println!("grade{}: {}<{}>.", self.grade, self.name, self.mail);
    }
}

fn student(name: String, mail: String, grade: i32) -> Student {
    Student { name, mail, grade }
}

fn main() {
    let taro = person(String::from("Taro"), String::from("taro@yamada"), 39);
    let hanako = student(String::from("Hanako"), String::from("hanako@flower"), 2);
    taro.print();
    hanako.print();
}
