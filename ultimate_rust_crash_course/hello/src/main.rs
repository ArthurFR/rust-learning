use hello::greet;
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let x = rng.gen_range(0..100);
    greet();
    println!("{}", x);
}

