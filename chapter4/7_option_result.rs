fn print(item: Option<i32>) -> Result<String, String> {
    match item {
        None => Err(String::from("ERROR IS OCCURED.")),
        Some(n) => {
            println!("No, {}.", n);
            Ok(String::from("OK"))
        }
    }
}
