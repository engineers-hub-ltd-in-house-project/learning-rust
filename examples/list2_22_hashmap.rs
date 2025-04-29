use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("first"), 123);
    map.insert(String::from("second"), 456);
    map.insert(String::from("third"), 789);
    map.remove("second");
    println!("{:?}", map);
}
