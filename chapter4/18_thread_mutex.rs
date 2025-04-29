/*
 * スレッド間での共有データの保護（Mutex）
 *
 * この例では、Rustの相互排他ロック（Mutex）を使って複数のスレッド間でデータを安全に共有する方法を示しています。
 * Mutexは一度に一つのスレッドだけがデータにアクセスできるようにするための同期プリミティブです。
 * 
 * Mutexの特徴：
 * 1. データへの排他的アクセスを提供する
 * 2. lockメソッドでロックを取得し、ロックが解放されるまで他のスレッドはブロックされる
 * 3. ロックガードがスコープを抜けると自動的にロックが解放される
 * 4. Arc（アトミック参照カウント）と組み合わせて複数のスレッド間でMutexを共有する
 * 
 * この例では、二つのスレッドが共有の整数値に異なる操作を行い、
 * Mutexを使用して競合状態を防いでいます。
 */

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Mutexで保護された共有整数値を作成し、Arcでラップ
    let num = Arc::new(Mutex::new(1));
    println!("Main: start!");

    // 最初のスレッド用に参照をクローン
    let num_1 = Arc::clone(&num);

    // 加算を行うスレッド
    let h1 = thread::spawn(move || {
        // Mutexのロックを取得
        let mut num_h1 = num_1.lock().unwrap();
        println!("H1: start!");
        for n in 1..5 {
            // 値に加算
            *num_h1 += n;
            println!("H1: num_h={}.", *num_h1);
            thread::sleep(Duration::from_millis(1));
        }
        println!("H1: End.");
        // ここでスコープを抜けると自動的にロックが解放される
    });

    // 二つ目のスレッド用に参照をクローン
    let num_2 = Arc::clone(&num);

    // 乗算を行うスレッド
    let h2 = thread::spawn(move || {
        // Mutexのロックを取得（最初のスレッドが終了するまでブロックされる）
        let mut num_h2 = num_2.lock().unwrap();
        println!("H2: start!");
        for n in 1..5 {
            // 値に乗算
            *num_h2 *= n;
            println!("H2: num_h={}.", *num_h2);
            thread::sleep(Duration::from_millis(1));
        }
        println!("H2: End.");
        // ここでスコープを抜けると自動的にロックが解放される
    });

    // 両方のスレッドの終了を待つ
    let _res = h1.join();
    let _res = h2.join();

    println!("Main: End.");
}
