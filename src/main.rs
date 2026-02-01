// Test file using chrono 0.3 API that will break with 0.4
//
// Breaking changes when upgrading to 0.4:
// - chrono::naive::date::NaiveDate -> chrono::naive::NaiveDate (or chrono::NaiveDate)
// - chrono::naive::time::NaiveTime -> chrono::naive::NaiveTime (or chrono::NaiveTime)
// - chrono::naive::datetime::NaiveDateTime -> chrono::naive::NaiveDateTime
// - chrono::offset::utc::UTC -> chrono::Utc
// - chrono::offset::local::Local -> chrono::Local
// - chrono::datetime::DateTime -> chrono::DateTime

use chrono::naive::date::NaiveDate;
use chrono::naive::time::NaiveTime;
use chrono::naive::datetime::NaiveDateTime;
use chrono::offset::utc::UTC;
use chrono::offset::local::Local;
use chrono::Datelike;

fn main() {
    // Create a date using the old API
    let date = NaiveDate::from_ymd(2024, 6, 15);
    println!("Date: {}", date);

    // Use isoweekdate() from Datelike trait
    let (year, week, weekday) = date.isoweekdate();
    println!("ISO week date: year={}, week={}, weekday={:?}", year, week, weekday);

    // Get individual date components
    let month = date.month();
    let day = date.day();
    println!("Month: {}, Day: {}", month, day);

    // Create a time
    let time = NaiveTime::from_hms(14, 30, 0);
    println!("Time: {}", time);

    // Create a datetime
    let datetime = NaiveDateTime::new(date, time);
    println!("DateTime: {}", datetime);

    // Use UTC (renamed to Utc in 0.4)
    let utc_now = UTC::now();
    println!("UTC now: {}", utc_now);

    // Use Local (path changes in 0.4)
    let local_now = Local::now();
    println!("Local now: {}", local_now);

    // Format using the old API
    let formatted = utc_now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted UTC: {}", formatted);

    // More date operations
    let tomorrow = date.succ();
    let yesterday = date.pred();
    println!("Tomorrow: {}, Yesterday: {}", tomorrow, yesterday);
}
