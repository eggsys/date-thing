use chrono::prelude::*;


fn main() {
    // let _result = plus_one([7,2,8,5,0,9,1,2,9,5,3,6,6,7,3,2,8,4,3,7,9,5,7,7,4,7,4,9,4,7,0,1,1,1,7,4,0,0,6].to_vec());
    // println!("{}", result)
    until_my_salary();
}

pub fn plus_one(digits: Vec<i32>) ->  Vec<i32> {
    println!("{}", digits[1]);

    let mut str_digits = String::from("");
    for d in digits.iter() {
        str_digits.push_str(d.to_string().as_str());

        // match d {
        //     &1 => println!("There is a rustacean among us!"),
        //     // TODO ^ Try deleting the & and matching just "Ferris"
        //     _ => println!("Hello {}", d),
        // }
    }
    println!("str digits: {}", str_digits);
    // let my_int = str_digits.parse::<u128>().unwrap();
    // let answer: u128 = &my_int + 1;

    let my_int: u128 = str_digits.parse().unwrap();
    let answer: u128 = &my_int + 1;


    let str_again: String = answer.to_string();
    // let mut collected_iterator: Vec<i32>;
    let mut vec_1: Vec<i32> = vec![];
   
    println!("str_again {}", str_again);
    for c in str_again.chars() { 
        // do something with `c`
        let int_c = c.to_string().parse::<i32>().unwrap();
        println!(" push int_c {}", int_c);
        
        vec_1.push(int_c)

    }
    
    println!("vec_1 {:?}", vec_1);
   
   return vec_1;
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

