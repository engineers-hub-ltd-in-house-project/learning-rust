/*
 * Result列挙型
 *
 * この例では、Rustの標準ライブラリに定義されているResult<T, E>列挙型の概念を示しています。
 * Result型は、操作が成功したときの値(Ok)と失敗したときのエラー値(Err)を表現します。
 *
 * Result<T, E>は以下の二つの状態を持ちます：
 * - Ok(T): 操作が成功し、T型の値を含む
 * - Err(E): 操作が失敗し、E型のエラー値を含む
 *
 * Result型の特徴：
 * 1. 関数の成功と失敗を型システムで明示的に表現できる
 * 2. エラーハンドリングを強制する
 * 3. 失敗する可能性のある操作を安全に扱える
 *
 * 以下は、Result<T, E>の簡易的な定義例です。実際には標準ライブラリで定義されています。
 */

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 使用例を示すmain関数
fn main() {
    // ファイルを開く操作（成功または失敗する可能性がある）
    let result = open_file("config.txt");

    // Result値をマッチングで処理
    match result {
        Ok(content) => println!("ファイルの内容: {}", content),
        Err(error) => println!("エラーが発生しました: {}", error),
    }
}

// ファイルを開く関数（実際のコードではなく、概念を示すもの）
fn open_file(path: &str) -> Result<String, String> {
    if path == "config.txt" {
        Ok(String::from("設定データ"))
    } else {
        Err(String::from("ファイルが見つかりません"))
    }
}
