use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::configs::TIMESTAMP_LENGTH;
use crate::common::errors::RowIDError;
use crate::utils::system_time::system_time_to_timestamp;

pub struct EncodeOptions<'a> {
    pub char_list: &'a str,
    pub system_time: SystemTime,
}

pub fn encode(opts: EncodeOptions) -> io::Result<String> {
    if opts.system_time < UNIX_EPOCH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            RowIDError::InvalidEncoded.as_str(),
        ));
    }

    let char_list: Vec<char> = opts.char_list.chars().collect();
    let char_list_length: usize = char_list.len();

    let mut index: usize = TIMESTAMP_LENGTH;
    let mut encoded: [char; TIMESTAMP_LENGTH] = ['\0'; TIMESTAMP_LENGTH];
    let mut remaining: usize = system_time_to_timestamp(opts.system_time);

    while index > 0 {
        index -= 1;
        encoded[index] = char_list[remaining % char_list_length];
        remaining /= char_list_length;
    }

    Ok(encoded.iter().collect())
}
