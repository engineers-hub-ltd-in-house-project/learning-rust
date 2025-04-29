use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    tx2.send(0).unwrap();
    println!("Main: start!");
    let h1 = thread::spawn(move || {
        println!("H1: start!");
        for _ in 1..5 {
            let val = rx2.recv().unwrap();
            let num = val + 1;
            println!("H1: num={}.", num);
            tx1.send(num).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
        println!("H1: End.");
    });
    thread::sleep(Duration::from_millis(5));
    let h2 = thread::spawn(move || {
        println!("H2: start!");
        for _ in 1..5 {
            let val = rx1.recv().unwrap();
            let num = val * 2;
            println!("H2: num={}.", num);
            tx2.send(num).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
        println!("H2: End.");
    });
    let _res = h1.join();
    let _res = h2.join();
    println!("Main: End.");
}
