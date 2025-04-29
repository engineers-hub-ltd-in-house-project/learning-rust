/*
 * スレッドのスリープ
 *
 * この例では、Rustでスレッドを一時停止（スリープ）する方法を示しています。
 * スレッドのスリープは、処理の遅延や、他のスレッドに実行機会を与えるために使用されます。
 * 
 * スレッドスリープの特徴：
 * 1. std::thread::sleep関数を使用する
 * 2. Duration型で時間を指定する
 * 3. スリープ中はスレッドが一時停止し、CPUリソースを消費しない
 * 4. スリープが終了すると処理を再開する
 * 
 * よく使用されるケース：
 * - 処理の速度を制限する
 * - 定期的なポーリング処理
 * - スレッド間の簡易的な同期
 * - デバッグや、動作確認のための遅延
 */

use std::thread;
use std::time::Duration;

fn main() {
    println!("プログラム開始");
    
    // 処理を開始
    for i in 1..=5 {
        println!("カウント: {}", i);
        
        // 1秒間スリープ
        let sleep_duration = Duration::from_secs(1);
        thread::sleep(sleep_duration);
    }
    
    // より短い時間でのスリープ
    println!("\nミリ秒単位のスリープ:");
    for i in 1..=3 {
        println!("短いスリープ: {}", i);
        
        // 500ミリ秒（0.5秒）スリープ
        let short_sleep = Duration::from_millis(500);
        thread::sleep(short_sleep);
    }
    
    println!("プログラム終了");
}
