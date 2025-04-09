use chrono::{Datelike, NaiveDate, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    let start = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

    let total_days = end.signed_duration_since(start).num_days() + 1;

    if total_days % 2 != 0 {
        let middle_day_ordinal = (total_days / 2) + 1;
        let middle_date = start + chrono::Duration::days(middle_day_ordinal as i64 - 1);
        Some(middle_date.weekday())
    } else {
        None
    }
}