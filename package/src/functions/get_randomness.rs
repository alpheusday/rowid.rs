use rand::{rngs::ThreadRng, Rng};

pub struct GetRandomnessOptions<CharList: AsRef<str>> {
    pub char_list: CharList,
    pub randomness_length: usize,
}

pub fn _get_randomness<CharList: AsRef<str>>(
    opts: GetRandomnessOptions<CharList>
) -> String {
    let char_list: Vec<char> = opts.char_list.as_ref().chars().collect();
    let length: usize = opts.randomness_length;
    let mut randomness: Vec<char> = Vec::with_capacity(length);
    let mut rng: ThreadRng = rand::rng();

    for _ in 0..length {
        let index: usize = rng.random_range(0..char_list.len());
        randomness.push(char_list[index]);
    }

    randomness.into_iter().collect()
}
