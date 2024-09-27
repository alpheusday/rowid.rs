use rand::{rngs::ThreadRng, Rng};

pub struct GetRandomnessOptions {
    pub char_list: String,
    pub randomness_length: usize,
}

pub fn get_randomness(opts: GetRandomnessOptions) -> String {
    let char_list: Vec<char> = opts.char_list.chars().collect();
    let mut rng: ThreadRng = rand::thread_rng();
    let mut randomness: Vec<char> = Vec::with_capacity(opts.randomness_length);

    for _ in 0..opts.randomness_length {
        let index: usize = rng.gen_range(0..char_list.len());
        randomness.push(char_list[index]);
    }

    randomness.into_iter().collect()
}
