use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("first"), 123);
    map.insert(String::from("second"), 456);
    map.insert(String::from("third"), 789);
    let mut result: i32 = 0;
    for (ky, val) in map {
        println!("{}: {}.", ky, val);
        result += val;
    }
    println!("total: {}.", result);
}
