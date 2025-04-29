use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        println!("Thread:Start!");
        thread::sleep(Duration::from_millis(10));
        println!("Thread:End.");
    });

    println!("Main:Start!");
    thread::sleep(Duration::from_millis(100));
    println!("Main:End.");
}
