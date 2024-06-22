use std::collections::HashMap;
use std::io;
use std::time::SystemTime;

use crate::common::configs::TIMESTAMP_LENGTH;
use crate::common::errors::{DECODE_ECD_INVALID_ERR, DECODE_ECD_LENGTH_ERR};
use crate::utils::system_time::timestamp_to_system_time;

pub struct DecodeOptions {
    pub char_list: String,
    pub encoded: String,
}

pub fn decode(opts: DecodeOptions) -> io::Result<SystemTime> {
    if opts.encoded.len() < TIMESTAMP_LENGTH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            DECODE_ECD_LENGTH_ERR,
        ));
    }

    let char_list: Vec<char> = opts.char_list.chars().collect();
    let char_list_length: usize = char_list.len();
    let char_index_map: HashMap<char, usize> =
        char_list.iter().enumerate().map(|(i, &c)| (c, i)).collect();

    let mut encoded_chars: [char; TIMESTAMP_LENGTH] = ['\0'; TIMESTAMP_LENGTH];

    for (i, c) in
        opts.encoded[..TIMESTAMP_LENGTH].to_uppercase().chars().enumerate()
    {
        encoded_chars[i] = c;
    }

    if !encoded_chars.iter().all(|c| char_index_map.contains_key(c)) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            DECODE_ECD_INVALID_ERR,
        ));
    }

    let mut timestamp: usize = 0;

    for c in encoded_chars {
        if let Some(index) = char_index_map.get(&c) {
            timestamp = timestamp * char_list_length + index;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                DECODE_ECD_INVALID_ERR,
            ));
        }
    }

    Ok(timestamp_to_system_time(timestamp))
}
