use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let entry = path.unwrap();
        println!("{:?}", entry.path().to_str());
    }
}
