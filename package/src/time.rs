use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// This function converts the [`SystemTime`] to timestamp in milliseconds.
///
/// Be aware that this conversion is **lossy** because
/// it discards part of the [`SystemTime`] precision.
///
/// ## Example
///
/// ```no_run
/// use std::time::SystemTime;
/// use rowid::time::system_time_to_timestamp;
///
/// let now: SystemTime = SystemTime::now();
///
/// let timestamp: usize = system_time_to_timestamp(now);
/// ```
pub fn system_time_to_timestamp<T: Into<SystemTime>>(system_time: T) -> usize {
    system_time
        .into()
        .duration_since(UNIX_EPOCH)
        .map(|d: Duration| d.as_millis() as usize)
        .unwrap_or(0)
}

/// This function converts timestamp in milliseconds to [`SystemTime`].
///
/// ## Example
///
/// ```no_run
/// use std::time::SystemTime;
/// use rowid::time::timestamp_to_system_time;
///
/// let timestamp: usize = 0;
///
/// let system_time: SystemTime = timestamp_to_system_time(timestamp);
/// ```
pub fn timestamp_to_system_time(timestamp: usize) -> SystemTime {
    UNIX_EPOCH + Duration::from_millis(timestamp as u64)
}
