// number generator
//lib.rs

use rand::Rng;

pub fn generate_random_number(digits: usize) -> u64 {
    let mut rng = rand::thread_rng();
    let min = 10u64.pow(digits as u32 - 1);
    let max = 10u64.pow(digits as u32) - 1;
    rng.gen_range(min..=max)
}
