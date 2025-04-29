use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let num = Arc::new(Mutex::new(1));
    println!("Main: start!");

    let num_1 = Arc::clone(&num);

    let h1 = thread::spawn(move || {
        let mut num_h1 = num_1.lock().unwrap();
        println!("H1: start!");
        for n in 1..5 {
            *num_h1 += n;
            println!("H1: num_h={}.", *num_h1);
            thread::sleep(Duration::from_millis(1));
        }
        println!("H1: End.");
    });

    let num_2 = Arc::clone(&num);

    let h2 = thread::spawn(move || {
        let mut num_h2 = num_2.lock().unwrap();
        println!("H2: start!");
        for n in 1..5 {
            *num_h2 *= n;
            println!("H2: num_h={}.", *num_h2);
            thread::sleep(Duration::from_millis(1));
        }
        println!("H2: End.");
    });

    let _res = h1.join();
    let _res = h2.join();

    println!("Main: End.");
}
