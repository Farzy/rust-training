extern crate rand;
extern crate rand_chacha;

use rand::{Rng, SeedableRng};

pub fn random() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    for _ in 1..20 {
        println!("Random: i16: {} f32: {}", rng.gen::<i16>(), rng.gen::<f32>());
    }
}
