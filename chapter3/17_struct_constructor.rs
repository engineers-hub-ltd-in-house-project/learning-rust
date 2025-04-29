/*
 * 構造体のコンストラクタ関数
 *
 * この例では、Rustで構造体のインスタンスを作成するためのコンストラクタ関数を
 * 定義する方法を示しています。Rustには専用のコンストラクタ構文はありませんが、
 * 構造体のインスタンスを返す関数を作成することで同様の機能を実現できます。
 *
 * コンストラクタ関数の利点：
 * 1. 構造体の作成ロジックを一箇所にまとめられる
 * 2. デフォルト値の設定やバリデーションを行える
 * 3. コードの重複を避けられる
 * 4. 詳細な実装を隠蔽できる
 *
 * この例では、Person構造体のインスタンスを作成するperson関数を定義しています。
 */

// Person構造体の定義
struct Person {
    name: String,
    email: String,
    age: i32,
}

// Person構造体のコンストラクタ関数
fn person(name: String, email: String, age: i32) -> Person {
    Person { name, email, age }
}

fn main() {
    // コンストラクタ関数を使用してインスタンスを作成
    let taro = person(String::from("太郎"), String::from("taro@example.com"), 30);

    println!(
        "名前: {}, メール: {}, 年齢: {}",
        taro.name, taro.email, taro.age
    );
}
