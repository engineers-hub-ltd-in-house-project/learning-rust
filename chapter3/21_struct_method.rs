/*
 * 構造体のメソッド
 *
 * この例では、Rustの構造体にメソッドを実装する方法を示しています。
 * メソッドは構造体に関連付けられた関数で、構造体のインスタンスに対して操作を行います。
 * 
 * 構造体メソッドの特徴：
 * 1. implブロック内で定義される
 * 2. 第一引数に &self, &mut self, self などを取る
 * 3. ドット記法 (instance.method()) で呼び出す
 * 4. データとそれに対する操作を一緒に扱うことができる
 * 
 * この例では、Person構造体に挨拶を行うprintメソッドと、
 * 年齢を変更するchange_ageメソッドを実装しています。
 */

// 人物を表す構造体
struct Person {
    name: String,
    email: String,
    age: u32,
}

// Personのコンストラクタ関数
fn person(name: String, email: String, age: u32) -> Person {
    Person { name, email, age }
}

// Person構造体にメソッドを実装
impl Person {
    // 自己紹介メソッド（不変参照）
    fn print(&self) {
        println!("私は{}（{}歳）です。連絡先: {}", self.name, self.age, self.email);
    }
    
    // 年齢を変更するメソッド（可変参照）
    fn change_age(&mut self, new_age: u32) {
        self.age = new_age;
        println!("{}の年齢を{}歳に変更しました", self.name, self.age);
    }
    
    // フルネームを返すメソッド（値を返す）
    fn full_name(&self, family_name: &str) -> String {
        format!("{} {}", family_name, self.name)
    }
}

fn main() {
    // Personのインスタンスを作成
    let mut taro = person(
        String::from("太郎"),
        String::from("taro@example.com"),
        30,
    );
    
    // メソッドを呼び出す
    taro.print();
    
    // 可変参照を使うメソッド
    taro.change_age(31);
    
    // 変更後の状態を表示
    taro.print();
    
    // 値を返すメソッド
    let full_name = taro.full_name("山田");
    println!("フルネーム: {}", full_name);
}
