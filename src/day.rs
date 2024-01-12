#[cfg(not(feature = "chrono"))]
pub fn timestamp() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

#[cfg(feature = "chrono")]
pub fn timestamp() -> i64 {
    use chrono::Utc;
    Utc::now().timestamp()
}
