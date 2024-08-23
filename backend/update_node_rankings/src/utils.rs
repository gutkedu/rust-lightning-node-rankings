use chrono::{SecondsFormat, TimeZone, Utc};

/// Convert a Unix timestamp to an ISO 8601 formatted string without subsecond precision.
pub fn convert_unix_to_iso(unix_timestamp: u64) -> String {
    const MIN_TIMESTAMP: i64 = 0; // 1970-01-01T00:00:00Z
    const MAX_TIMESTAMP: i64 = 253402300799; // 9999-12-31T23:59:59Z

    let timestamp = unix_timestamp as i64;

    if timestamp < MIN_TIMESTAMP || timestamp > MAX_TIMESTAMP {
        return String::new();
    }

    if let Some(datetime) = Utc.timestamp_opt(timestamp, 0).single() {
        datetime.to_rfc3339_opts(SecondsFormat::Secs, true)
    } else {
        String::new()
    }
}

/// Convert a Satoshi value to a Bitcoin value.
pub fn convert_satoshi_to_btc(satoshi: u64) -> f64 {
    satoshi as f64 / 100_000_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_unix_to_iso() {
        let unix_timestamp = 1638316800; // Corresponds to 2021-12-01T00:00:00Z
        let expected_iso = "2021-12-01T00:00:00Z";
        assert_eq!(convert_unix_to_iso(unix_timestamp), expected_iso);

        let invalid_unix_timestamp = u64::MAX; // An invalid timestamp
        assert_eq!(convert_unix_to_iso(invalid_unix_timestamp), String::new());
    }

    #[test]
    fn test_convert_satoshi_to_btc() {
        let satoshi = 100_000_000;
        let expected_btc = 1.0;
        assert_eq!(convert_satoshi_to_btc(satoshi), expected_btc);

        let satoshi = 50_000_000;
        let expected_btc = 0.5;
        assert_eq!(convert_satoshi_to_btc(satoshi), expected_btc);
    }
}
