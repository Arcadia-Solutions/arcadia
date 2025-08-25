use chrono::{DateTime, NaiveDate, TimeZone, Utc};

pub fn naive_date_to_utc_midnight(date: NaiveDate) -> DateTime<Utc> {
    Utc.from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
        .unwrap()
}
