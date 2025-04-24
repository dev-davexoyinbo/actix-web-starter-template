use rand::{Rng, distr::Alphanumeric};

pub fn generate_random_string(length: u8) -> String {
    let mut rng = rand::rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
} // end function generate_random_string
