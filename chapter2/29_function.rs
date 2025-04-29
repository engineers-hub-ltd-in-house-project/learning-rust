/*
 * 関数の定義と使用
 *
 * この例では、Rustでの関数の定義と呼び出し方を示しています。
 * Rustでは関数を使ってコードを整理し、再利用可能な形にすることができます。
 *
 * 関数の特徴：
 * 1. fnキーワードで定義する
 * 2. 名前はスネークケース（小文字、アンダースコア区切り）を使う慣習がある
 * 3. 引数の型を明示的に指定する必要がある
 * 4. 戻り値の型は -> 記号の後に指定する
 *
 * この例では、挨拶メッセージを生成する関数を定義して呼び出しています。
 */

fn main() {
    // 名前を定義（型注釈を明示）
    let name: String = String::from("山田");

    // 関数を呼び出して結果を取得
    let message: String = create_greeting(name);

    // 結果を表示
    println!("{}", message);
}

// 挨拶を生成する関数
// nameを引数として受け取り、挨拶文を返す
fn create_greeting(name: String) -> String {
    // 最後の式が戻り値になる（returnキーワードは不要）
    format!("こんにちは、{}さん！", name)
}

// 数値計算を行う関数の例（明示的な型注釈付き）
fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b // 最後の式が戻り値
}

// 複数の戻り値を返す関数（タプルを使用）
fn get_statistics(numbers: &[i32]) -> (i32, i32, f64) {
    let sum: i32 = numbers.iter().sum();
    let max: i32 = *numbers.iter().max().unwrap_or(&0);
    let avg: f64 = sum as f64 / numbers.len() as f64;

    (sum, max, avg) // 合計、最大値、平均値をタプルで返す
}
