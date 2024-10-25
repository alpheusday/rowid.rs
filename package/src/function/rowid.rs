use std::time::SystemTime;

use crate::function::{
    encode::{encode_unsafe, EncodeOptions},
    get_randomness::{GetRandomnessOptions, _get_randomness},
};

pub struct RowIDOptions<CharList: AsRef<str>> {
    pub char_list: CharList,
    pub randomness_length: usize,
}

pub fn _rowid<CharList: AsRef<str>>(opts: RowIDOptions<CharList>) -> String {
    let char_list: &str = opts.char_list.as_ref();

    encode_unsafe(EncodeOptions { char_list, system_time: SystemTime::now() })
        + &_get_randomness(GetRandomnessOptions {
            char_list,
            randomness_length: opts.randomness_length,
        })
}
