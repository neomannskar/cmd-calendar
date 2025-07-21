use chrono::{Datelike, Duration, Local, NaiveDate};
use colored::*;

fn main() {
    let today = Local::now().date_naive();
    let (year, month) = (today.year(), today.month());

    // First day of the current month
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let start_weekday = first_day.weekday().num_days_from_sunday(); // 0 = Sunday

    // Calculate start of calendar grid
    let grid_start = first_day - Duration::days(start_weekday as i64);

    // Print the month name
    let month_name = first_day.format("%B").to_string(); // e.g. "June"
    println!("{}", month_name.bold().underline());

    // Print each of 5 weeks (5x7 = 35 days)
    for week in 0..5 {
        for day in 0..7 {
            let offset = week * 7 + day;
            let date = grid_start + Duration::days(offset as i64);

            let mut s = format!("{:>02}", date.day());

            if date == today {
                // Highlight current day
                s = s.bold().bright_blue().to_string();
            } else if date.month() != month {
                // Grey out dates from other months
                s = s.dimmed().truecolor(150, 150, 150).to_string();
            }

            print!("{} ", s);
        }
        println!();
    }
}
