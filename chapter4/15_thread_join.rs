use std::thread;
use std::time::Duration;

fn main() {
    println!("Main:start!");

    let h = thread::spawn(|| {
        thread::spawn(|| {
            for n in 1..6 {
                println!("H1:No,{}.", n);
                thread::sleep(Duration::from_millis(2));
            }
        });

        thread::spawn(|| {
            for n in 1..6 {
                println!("H2:No,{}.", n);
                thread::sleep(Duration::from_millis(2));
            }
        });

        for n in 1..6 {
            println!("Thread:No,{}.", n);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let _res = h.join();
    println!("Main:End.");
}
