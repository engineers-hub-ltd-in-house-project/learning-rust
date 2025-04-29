use std::fs::File;
use std::io::Write;

fn main() {
    let data = [
        "Hello world!",
        "これはサンプルのデータです。",
        "テストテスト！",
    ];
    let str_data = data.join("\n");
    let mut file = File::create("backup.txt").unwrap();
    file.write_all(str_data.as_bytes()).unwrap();
}
