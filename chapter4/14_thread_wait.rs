use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for n in 1..100 {
            println!("Thread:No,{}.", n);
        }
    });
    thread::sleep(Duration::from_millis(1));
    for n in 1..100 {
        println!("Main: No,{}.", n);
    }
}
