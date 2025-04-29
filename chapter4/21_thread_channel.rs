use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    println!("Main: start!");
    let h1 = thread::spawn(move || {
        let mut num = 1;
        println!("H1: start!");
        for n in 1..5 {
            num += n;
            tx.send(num).unwrap();
            println!("H1: num={}.", num);
            thread::sleep(Duration::from_millis(10));
        }
        println!("H1: End.");
    });
    let h2 = thread::spawn(move || {
        println!("H2: start!");
        for _n in 1..5 {
            let num_recv = rx.recv().unwrap();
            println!("H2: num_recv={}.", num_recv);
            thread::sleep(Duration::from_millis(20));
        }
        println!("H2: End.");
    });
    let _res = h1.join();
    let _res = h2.join();
    println!("Main: End.");
}
