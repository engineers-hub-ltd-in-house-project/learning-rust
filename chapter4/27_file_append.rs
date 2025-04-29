use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let str_data = "This is sample!\n";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("append.txt")
        .unwrap();
    file.write_all(str_data.as_bytes()).unwrap();
}
