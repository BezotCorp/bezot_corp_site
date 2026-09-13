use std::time::{SystemTime, UNIX_EPOCH};

const SECONDS_PER_DAY: u64 = 86_400;

pub fn today_yyyy_mm_dd() -> String {
    let days_since_unix_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() / SECONDS_PER_DAY)
        .unwrap_or_default();
    let (year, month, day) = civil_date_from_unix_days(days_since_unix_epoch as i64);

    format!("{year:04}-{month:02}-{day:02}")
}

fn civil_date_from_unix_days(days_since_unix_epoch: i64) -> (i32, u32, u32) {
    let shifted_days = days_since_unix_epoch + 719_468;
    let era = if shifted_days >= 0 {
        shifted_days
    } else {
        shifted_days - 146_096
    } / 146_097;
    let day_of_era = shifted_days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    let adjusted_year = year + if month <= 2 { 1 } else { 0 };

    (adjusted_year as i32, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use super::civil_date_from_unix_days;

    #[test]
    fn converts_unix_epoch_to_civil_date() {
        assert_eq!(civil_date_from_unix_days(0), (1970, 1, 1));
    }

    #[test]
    fn converts_known_future_date() {
        assert_eq!(civil_date_from_unix_days(20_345), (2025, 9, 14));
    }
}
