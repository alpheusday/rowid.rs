use std::time::SystemTime;

use rowid::time::{system_time_to_timestamp, timestamp_to_system_time};

// system_time_to_timestamp

#[test]
fn test_system_time_to_timestamp() {
    let timestamp: usize = system_time_to_timestamp(SystemTime::UNIX_EPOCH);
    assert!(timestamp == 0);
}

// timestamp_to_system_time

#[test]
fn test_timestamp_to_system_time() {
    let system_time: SystemTime = timestamp_to_system_time(0);
    assert!(system_time == SystemTime::UNIX_EPOCH);
}

// timestamp_to_system_time + system_time_to_timestamp

#[test]
fn test_timestamp_system_time() {
    let ts: usize = 1_000_000_000;
    let system_time: SystemTime = timestamp_to_system_time(ts);
    let timestamp: usize = system_time_to_timestamp(system_time);
    assert!(ts == timestamp);
}
