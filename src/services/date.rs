use chrono::Datelike;
use crate::routes::date::Date;

pub fn get_current_date() -> Date {
    let current_utc = chrono::Utc::now();
    let year = current_utc.year();
    let month = current_utc.month();
    let day = current_utc.day();
    let current_date = Date {
        day,
        month,
        year
    };

    current_date
}