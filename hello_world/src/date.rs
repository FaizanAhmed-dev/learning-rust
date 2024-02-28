extern crate chrono;
use chrono::{DateTime, Local, Utc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Function to get the current date as a UNIX timestamp (seconds since Jan 1, 1970)
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backward")
        .as_secs()
}

// Function to format a UNIX timestamp as a human-readable date
fn format_timestamp(timestamp: u64) -> String {
    let dt = UNIX_EPOCH + Duration::from_secs(timestamp);
    dt.to_string()
}


// Function to get the current local date and time
fn current_local_datetime() -> DateTime<Local> {
    Local::now()
}

// Function to format a DateTime as a string
fn format_datetime(dt: DateTime<Local>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_date() {
    let timestamp = current_timestamp();
    let formatted_date = format_timestamp(timestamp);

    println!("Current UNIX Timestamp: {}", timestamp);
    println!("Formatted Date: {}", formatted_date);
}
