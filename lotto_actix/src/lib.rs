use rand::Rng;

pub fn generate_random_number(digits: usize) -> u32 {
    let mut rng = rand::thread_rng();

    // Calculate the minimum and maximum values based on the number of digits
    let min_value = 10u32.pow(digits as u32 - 1);
    let max_value = 10u32.pow(digits as u32) - 1;

    // Generate a random number within the specified range
    rng.gen_range(min_value..=max_value)
}
