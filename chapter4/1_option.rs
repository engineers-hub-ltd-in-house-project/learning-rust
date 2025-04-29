/*
 * Option列挙型
 *
 * Rustでは、値が存在しない可能性を表現するためにOption<T>列挙型を使用します。
 * これにより、nullポインタによるエラーを防ぎ、コンパイル時に安全性を確保できます。
 * 
 * Option<T>は以下の二つの状態を持ちます：
 * - Some(T): 値が存在する場合
 * - None: 値が存在しない場合
 * 
 * OptionはRustの標準ライブラリで定義されており、明示的にインポートしなくても使用できます。
 * OptionはRustの型システムの重要な部分であり、値の有無を安全に扱うための基盤となっています。
 * 
 * 以下はOption<T>の簡易的な定義例です。実際には標準ライブラリで定義されています。
 */

enum Option<T> {
    Some(T),
    None,
}

fn main() {
  // 値がある場合
  let some_value: Option<i32> = Some(42);
  
  // 値がない場合
  let no_value: Option<i32> = None;
  
  // 値の処理例
  match some_value {
    Some(value) => println!("値があります: {}", value),
    None => println!("値がありません"),
  }
  
  match no_value {
    Some(value) => println!("値があります: {}", value),
    None => println!("値がありません"),
  }
}
