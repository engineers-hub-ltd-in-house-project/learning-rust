use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("first"), 123);
    map.insert(String::from("second"), map["first"] * 2);
    map.insert(
        String::from("third"),
        map.get("first").unwrap() + map.get("second").unwrap(),
    );
    println!("{:?}.", map);
}
