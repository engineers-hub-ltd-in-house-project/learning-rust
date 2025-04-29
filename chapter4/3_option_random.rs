use rand::Rng;

fn random() -> Option<i32> {
    let n = rand::thread_rng().gen_range(0..10);
    match n {
        0 => None,
        _ => Some(n),
    }
}
