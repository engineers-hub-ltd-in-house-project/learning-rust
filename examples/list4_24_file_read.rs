use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        count += 1;
        let txt = line.unwrap();
        println!("{}: {}", count, txt);
    }
}
