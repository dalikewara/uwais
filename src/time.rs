use std::time::{SystemTime, UNIX_EPOCH};

#[inline]
pub fn unix_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}
