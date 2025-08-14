use chrono::{DateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gigasecond = chrono::Duration::seconds(10_i64.pow(9));
    start + gigasecond
}
