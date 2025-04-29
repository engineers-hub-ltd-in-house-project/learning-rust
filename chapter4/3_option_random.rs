/*
 * ランダムなOption値
 *
 * この例では、Rustでランダムに Some 値または None を生成して
 * Option型の特性を示しています。
 * 
 * この例の重要なポイント：
 * 1. ランダム値の生成（乱数）
 * 2. 条件に基づいたSomeまたはNoneの生成
 * 3. match式によるOption値の処理
 * 
 * 不確定な結果を扱う場合、Option型を使うことで値が存在しない可能性を
 * コード上で明示的に表現し、安全に処理することができます。
 */

use rand::Rng;

fn main() {
    // ランダムな値を生成する乱数生成器
    let mut rng = rand::thread_rng();
    
    // 10回繰り返し
    for i in 1..=10 {
        // ランダムに Option<i32> を生成（50%の確率でSome、50%の確率でNone）
        let value: Option<i32> = if rng.gen_bool(0.5) {
            // Some値ならランダムな数値（1-100）を生成
            Some(rng.gen_range(1..=100))
        } else {
            None
        };
        
        // matchを使ってOption値を処理
        match value {
            Some(num) => println!("試行 {}: 値が存在します - {}", i, num),
            None => println!("試行 {}: 値が存在しません", i),
        }
    }
}
