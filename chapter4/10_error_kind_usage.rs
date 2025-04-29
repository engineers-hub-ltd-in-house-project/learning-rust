fn print_all(data: Vec<Option<i32>>) {
    for item in data {
        let res = print(item);
        match res {
            Ok(s) => println!("--- {} ---", s),
            Err(k) => match k {
                ErrKind::Caution => {
                    println!("*** CAUTION!! ***");
                }
                ErrKind::Danger => {
                    println!("DANGER!!");
                    panic!("DANGER ERROR.");
                }
            },
        }
    }
}

fn print(item: Option<i32>) -> Result<String, ErrKind> {
    match item {
        None => Err(ErrKind::Danger),
        Some(n) => {
            println!("No, {}.", n);
            if n == 1 {
                Err(ErrKind::Caution)
            } else {
                Ok(String::from("OK"))
            }
        }
    }
}
