use std::fs;

fn main() {
    _ = fs::create_dir("./backup");
    let entries = fs::read_dir("./").unwrap();

    for path in entries {
        let entry = path.unwrap();
        if entry.file_type().unwrap().is_file() {
            let file_name = entry.file_name();
            let from_name = format!("./{}", file_name.to_string_lossy());
            let to_name = format!("./backup/_{}", file_name.to_string_lossy());

            _ = fs::copy(&from_name, &to_name);
            println!("backup: {:?} → {}", from_name, to_name);
        } else {
            println!("not copied.({:?})", entry.file_name());
        }
    }
}
