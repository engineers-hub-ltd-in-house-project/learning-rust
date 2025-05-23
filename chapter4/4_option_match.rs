fn main() {
    let mut data = vec![];
    for _ in 0..10 {
        data.push(random());
    }
    print_all(data);
}

fn print_all(data: Vec<Option<i32>>) {
    for item in data {
        print(item);
    }
}

fn print(item: Option<i32>) {
    match item {
        None => println!("no-data..."),
        Some(n) => println!("No, {}.", n),
    }
}
