use rand::{distributions::Alphanumeric, Rng};

pub struct Randomizer;

impl Randomizer {
    pub fn generate_string(length: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    pub fn random_number(range: std::ops::Range<u32>) -> u32 {
        rand::thread_rng().gen_range(range)
    }

    pub fn random_boolean() -> bool {
        rand::thread_rng().gen()
    }
}
