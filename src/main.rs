use chrono::prelude::*;

fn main() {
    until_my_salary();
}



pub fn until_my_salary() {
    let local: DateTime<Local> = Local::now();
    println!("local Date: {}", local);


    let naive_date_time_local_utc: NaiveDateTime = Local::now().naive_utc();
    println!("naive_date_time_local_utc: {}", naive_date_time_local_utc);

    let naive_date_time_utc_utc: NaiveDateTime = Utc::now().naive_utc();
    println!("naive_date_time_utc_utc: {}", naive_date_time_utc_utc);

    let naive_date_time_local_local: NaiveDateTime = Local::now().naive_local();
    println!("naive_date_time_local_local: {}", naive_date_time_local_local);

    let naive_date_time_utc_local: NaiveDateTime = Utc::now().naive_local();
    println!("naive_date_time_utc_local: {}", naive_date_time_utc_local);

    let naive_date_local: NaiveDate = Local::now().date_naive();
    println!("naive_date_local: {}", naive_date_local);


    println!("Date : {:?}", naive_date_local);

    println!("Date from dateTime : {:?}", naive_date_time_utc_local.day());
    println!("Date from dateTime : {:?}", naive_date_time_utc_local.date());

    
    let today = Local::now().naive_utc();

    let is_before_25th = today.day() < 25;

    if is_before_25th {
        println!("Today is before the 25th of the month.");
        //calculation duration to 25th of this month

    } else {
        println!("Today is on or after the 25th of the month.");
        //calculation duration to 25th of next month
    }
    // let dt: chrono::LocalResult<DateTime<Local>> = Local.with_ymd_and_hms(year, month, day, hour, min, sec);
}

// pub fn add_months(date: NaiveDate, num_months: u32) -> NaiveDate {
//     let mut month = date.month() + num_months;
//     let year = date.year() + (month / 12) as i32;
//     month = month % 12;
//     let mut day = date.day();
//     // let max_days = get_days_from_month(year, month);
//     day = if day > max_days { max_days } else { day };
//     NaiveDate::from_ymd(year, month, day)
// }

