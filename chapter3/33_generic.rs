/*
 * ジェネリクス型
 *
 * この例では、Rustのジェネリクス（generics）の基本的な使い方を示しています。
 * ジェネリクスを使うと、型パラメータを持つ汎用的なデータ構造や関数を定義できます。
 *
 * ジェネリクスの特徴：
 * 1. 同じコードで異なる型に対応できる
 * 2. コンパイル時に具体的な型が決定されるので実行時のオーバーヘッドがない
 * 3. 型パラメータには制約（トレイト境界）を設定できる
 * 4. コードの重複を減らし再利用性を高める
 *
 * この例では、型パラメータTを持つPoint<T>構造体と、
 * そのメソッドを実装しています。
 */

// ジェネリクス型のPoint構造体
// 型パラメータ T は座標の型を表す
struct Point<T> {
    x: T,
    y: T,
}

// Point<T>に対するメソッド実装
impl<T> Point<T> {
    // 新しいPointを作成するコンストラクタメソッド
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// 数値型（Add, Multiplyトレイトを実装する型）に対する追加メソッド
impl<T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy> Point<T> {
    // 原点からの距離の二乗を計算（ピタゴラスの定理）
    fn distance_squared_from_origin(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

fn main() {
    // i32型の座標を持つPoint
    let p1 = Point::new(5, 10);
    println!("整数の点: ({}, {})", p1.x, p1.y);

    // f64型の座標を持つPoint
    let p2 = Point::new(3.0, 4.0);
    println!("浮動小数点の点: ({}, {})", p2.x, p2.y);

    // 距離の二乗を計算
    let distance_squared = p2.distance_squared_from_origin();
    println!("原点からの距離の二乗: {}", distance_squared);
    println!("原点からの距離: {}", f64::sqrt(distance_squared));
}
