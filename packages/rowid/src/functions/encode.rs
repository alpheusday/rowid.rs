use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::configs::TIMESTAMP_LENGTH;
use crate::common::errors::ENCODE_ST_INVALID_ERR;
use crate::utils::system_time::system_time_to_timestamp;

pub struct EncodeOptions {
    pub char_list: String,
    pub system_time: SystemTime,
}

pub fn encode(opts: EncodeOptions) -> io::Result<String> {
    if opts.system_time < UNIX_EPOCH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            ENCODE_ST_INVALID_ERR,
        ));
    }

    let char_list: Vec<char> = opts.char_list.chars().collect();
    let char_list_length: usize = char_list.len();

    let mut index = TIMESTAMP_LENGTH;
    let mut encoded: [char; TIMESTAMP_LENGTH] = ['\0'; TIMESTAMP_LENGTH];
    let mut remaining: usize = system_time_to_timestamp(opts.system_time);

    while index > 0 {
        index -= 1;
        encoded[index] = char_list[remaining % char_list_length];
        remaining /= char_list_length;
    }

    Ok(encoded.iter().collect())
}
