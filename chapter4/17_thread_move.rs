use std::thread;
use std::time::Duration;

fn main() {
    let mut num = 1;
    println!("Main: start!");

    let h1 = thread::spawn(move || {
        println!("H1: start!");
        for n in 1..5 {
            num = 10 * n;
            println!("H1: num_h={}.", num);
            thread::sleep(Duration::from_millis(10));
        }
        println!("H1: End.");
    });

    let h2 = thread::spawn(move || {
        println!("H2: start!");
        for n in 1..5 {
            num += n;
            println!("H2: num_h={}.", num);
            thread::sleep(Duration::from_millis(10));
        }
        println!("H2: End.");
    });
    let _res = h1.join();
    let _res = h2.join();
    println!("Main: End.");
}
