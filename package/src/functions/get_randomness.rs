use rand::{rngs::ThreadRng, Rng};

pub struct GetRandomnessOptions<'a> {
    pub char_list: &'a str,
    pub randomness_length: usize,
}

pub fn get_randomness(opts: GetRandomnessOptions) -> String {
    let char_list: Vec<char> = opts.char_list.chars().collect();
    let length: usize = opts.randomness_length;
    let mut randomness: Vec<char> = Vec::with_capacity(length);
    let mut rng: ThreadRng = rand::thread_rng();

    for _ in 0..length {
        let index: usize = rng.gen_range(0..char_list.len());
        randomness.push(char_list[index]);
    }

    randomness.into_iter().collect()
}
