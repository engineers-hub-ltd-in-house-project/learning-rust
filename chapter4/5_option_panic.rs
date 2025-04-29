fn print(item: Option<i32>) {
    match item {
        None => panic!("NODATA!!"),
        Some(n) => println!("No, {}.", n),
    }
}
