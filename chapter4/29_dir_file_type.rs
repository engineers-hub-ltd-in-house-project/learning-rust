use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let entry = path.unwrap();
        let ftype = entry.file_type().unwrap();
        if ftype.is_file() {
            println!("{:?} file", entry.path())
        } else if ftype.is_dir() {
            println!("{:?} dir", entry.path())
        } else if ftype.is_symlink() {
            println!("{:?} link", entry.path())
        } else {
            println!("{:?}", entry.path())
        }
    }
}
