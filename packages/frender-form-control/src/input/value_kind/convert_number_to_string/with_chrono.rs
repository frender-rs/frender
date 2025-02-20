use chrono::prelude::*;

fn millis_to_secs_and_nsecs(millis: f64) -> (i64, u32) {
    let secs = millis.div_euclid(1000.0) as i64;
    let nsecs = (millis.rem_euclid(1000.0) * 1_000_000.0) as u32;
    (secs, nsecs)
}

fn millis_to_date_time(millis: f64) -> Option<DateTime<Utc>> {
    let (secs, nsecs) = millis_to_secs_and_nsecs(millis);
    DateTime::from_timestamp(secs, nsecs)
}

fn millis_to_date(millis: f64) -> Option<NaiveDate> {
    millis_to_date_time(millis).map(|dt| dt.date_naive())
}

fn month_to_date(month_num: i32) -> Option<NaiveDate> {
    const DATE_UNIX_EPOCH: NaiveDate = NaiveDateTime::UNIX_EPOCH.date();
    let months = chrono::Months::new(month_num.unsigned_abs());
    if month_num < 0 {
        DATE_UNIX_EPOCH.checked_sub_months(months)
    } else {
        DATE_UNIX_EPOCH.checked_add_months(months)
    }
}

// TODO: this might have leading `-` or `+` but input value doesn't allow that.
fn date_string(dt: NaiveDate) -> String {
    dt.to_string()
}

fn month_string(dt: NaiveDate) -> String {
    let mut s = date_string(dt);
    let index_of_last_hyphen = {
        let s = s.as_bytes();
        let mut i = s.len() - 1;
        while s[i] != b'-' {
            i -= 1;
        }

        i
    };
    s.truncate(index_of_last_hyphen);

    s
}

fn week_string(dt: NaiveDate) -> String {
    let w = dt.iso_week();
    format!("{:?}", w)
}

fn millis_to_time(millis: f64) -> Option<NaiveTime> {
    let (secs, nsecs) = millis_to_secs_and_nsecs(millis);

    let secs = secs.rem_euclid(86_400);

    NaiveTime::from_num_seconds_from_midnight_opt(secs as u32, nsecs)
}

fn time_string(t: NaiveTime) -> String {
    t.to_string()
}

fn datetime_string(dt: DateTime<Utc>) -> String {
    format!("{:?}", dt.naive_utc())
}

pub(super) fn convert_non_nan_number_to_string(input_type: &str, value: f64) -> String {
    match input_type {
        "date" => millis_to_date(value).map(date_string),
        "month" => month_to_date(value as i32).map(month_string),
        "week" => millis_to_date(value).map(week_string),
        "time" => millis_to_time(value).map(time_string),
        "datetime-local" => millis_to_date_time(value).map(datetime_string),
        "number" | "range" => return value.to_string(), // TODO: is this compatible with js implementation?
        _ => None,
    }
    .unwrap_or_default()
}
