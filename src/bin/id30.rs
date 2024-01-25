use id30::Id30;
use rand08::prelude::*;

fn main() {
    for arg in std::env::args().skip(1) {
        let id30: Result<Id30, _> = arg.parse();
        println!("{arg} => {:?}", id30);
    }

    for i in 0..64 {
        println!("{}", Id30::try_from(i).unwrap());
    }

    let mut rng = rand08::thread_rng();
    for _ in 0..10 {
        println!("{}", rng.gen::<Id30>());
    }
}
