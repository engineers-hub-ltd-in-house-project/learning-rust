use std::fs::File;
use std::io::ErrorKind;
use std::io::{BufRead, BufReader};

fn main() {
    let file = match File::open("data.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("ファイルが見つかりませんでした"),
            ErrorKind::PermissionDenied => panic!("ファイルへのアクセス権限がありません"),
            _ => panic!("ファイルのオープンに失敗しました: {:?}", error),
        },
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
