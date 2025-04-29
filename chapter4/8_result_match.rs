fn print_all(data: Vec<Option<i32>>) {
    for item in data {
        let res = print(item);
        match res {
            Ok(s) => println!("--- {} ---", s),
            Err(s) => println!("*** {} ***", s),
        }
    }
}
