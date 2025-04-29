fn main() {
    // ここではトレイト実装などは省略していますが、
    // 本来は正しいトレイト実装と変数定義が必要です
    // このファイルは参考用の一部抜粋として使用しています
    let taro = String::from("example");
    let hanako = String::from("example");

    print(&taro);
    print(&hanako);
}

fn print<T>(_ob: &T) {
    // 実装は省略
}
