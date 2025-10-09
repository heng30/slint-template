//! Time and date utilities for formatting, parsing, and calendar operations.

use anyhow::{Context, Result};
use chrono::{Datelike, Duration, Local, NaiveDate, TimeZone, Weekday};

/// Represents a simple date with year, month, and day.
#[derive(Debug, Clone)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

/// Formats the current local time according to the specified format string.
///
/// The format string follows the same syntax as `chrono::format::strftime`.
///
/// # Arguments
///
/// * `format` - The format string (e.g., "%Y-%m-%d %H:%M:%S")
///
/// # Returns
///
/// Returns the formatted time string.
///
/// # Examples
///
/// ```
/// use cutil::time::local_now;
///
/// let formatted = local_now("%Y-%m-%d %H:%M:%S");
/// println!("Current time: {}", formatted);
/// ```
pub fn local_now(format: &str) -> String {
    Local::now().format(format).to_string()
}

/// Gets the current date as a `Date` struct.
///
/// # Returns
///
/// Returns a `Date` struct representing today's date.
///
/// # Examples
///
/// ```
/// use cutil::time::get_current_date;
///
/// let today = get_current_date();
/// println!("Today is {}-{}-{}", today.year, today.month, today.day);
/// ```
pub fn get_current_date() -> Date {
    let now = Local::now();

    Date {
        year: now.year(),
        month: now.month(),
        day: now.day(),
    }
}

/// Parses a date string in "YYYY-MM-DD" format into a `Date` struct.
///
/// # Arguments
///
/// * `date` - The date string in "YYYY-MM-DD" format
///
/// # Returns
///
/// Returns a `Date` struct on success.
///
/// # Examples
///
/// ```
/// use cutil::time::parse_date_str;
///
/// let date = parse_date_str("2023-11-15").unwrap();
/// assert_eq!(date.year, 2023);
/// assert_eq!(date.month, 11);
/// assert_eq!(date.day, 15);
/// ```
pub fn parse_date_str(date: &str) -> Result<Date> {
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;

    let datetime = date
        .and_hms_opt(0, 0, 0)
        .ok_or(anyhow::anyhow!("Invalid time specification"))?
        .and_utc();

    Ok(Date {
        year: datetime.year(),
        month: datetime.month(),
        day: datetime.day(),
    })
}

/// Gets the current Unix timestamp (seconds since epoch).
///
/// # Returns
///
/// Returns the current timestamp as a 64-bit integer.
///
/// # Examples
///
/// ```
/// use cutil::time::timestamp;
///
/// let ts = timestamp();
/// println!("Current timestamp: {}", ts);
/// ```
pub fn timestamp() -> i64 {
    Local::now().timestamp()
}

/// Generates a calendar matrix for a specific year and month.
///
/// The matrix is 6x7 (6 weeks, 7 days per week) and includes dates from
/// the previous and next months to fill out the calendar grid.
///
/// # Arguments
///
/// * `year` - The year
/// * `month` - The month (1-12)
///
/// # Returns
///
/// Returns a 6x7 matrix of `Date` structs representing the calendar.
///
/// # Examples
///
/// ```
/// use cutil::time::get_calendar_matrix;
///
/// let calendar = get_calendar_matrix(2023, 11).unwrap();
/// assert_eq!(calendar.len(), 6);
/// assert!(calendar.iter().all(|week| week.len() == 7));
/// ```
pub fn get_calendar_matrix(year: i32, month: u32) -> Result<Vec<Vec<Date>>> {
    let mut matrix: Vec<Vec<Date>> = vec![vec![]; 6];

    let first_day_month = NaiveDate::from_ymd_opt(year, month, 1)
        .with_context(|| format!("Get first day of month {year}-{month} failed"))?;
    let first_day_weekday = first_day_month.weekday();

    let first_day_col = match first_day_weekday {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    };

    let start_date = first_day_month - Duration::days(first_day_col as i64);

    let mut current_date = start_date;
    for row in 0..6 {
        matrix[row] = Vec::with_capacity(7);
        for _ in 0..7 {
            matrix[row].push(Date {
                year: current_date.year(),
                month: current_date.month(),
                day: current_date.day(),
            });
            current_date = current_date + Duration::days(1);
        }
    }

    Ok(matrix)
}

/// Converts a date string to a Unix timestamp.
///
/// # Arguments
///
/// * `date_str` - The date string in "YYYY-MM-DD" format
///
/// # Returns
///
/// Returns the Unix timestamp (seconds since epoch) for the given date at 00:00:00.
///
/// # Examples
///
/// ```
/// use cutil::time::date_str_to_timestamp;
///
/// let timestamp = date_str_to_timestamp("2023-11-15").unwrap();
/// println!("Timestamp: {}", timestamp);
/// ```
pub fn date_str_to_timestamp(date_str: &str) -> Result<i64> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    let datetime = date
        .and_hms_opt(0, 0, 0)
        .ok_or(anyhow::anyhow!("Invalid time specification"))?;
    Ok(Local.from_local_datetime(&datetime).unwrap().timestamp())
}

/// Calculates the number of days between two dates.
///
/// # Arguments
///
/// * `start_date` - The start date in "YYYY-MM-DD" format
/// * `end_date` - The end date in "YYYY-MM-DD" format
///
/// # Returns
///
/// Returns the number of days between the two dates.
///
/// # Examples
///
/// ```
/// use cutil::time::diff_dates_to_days;
///
/// let days = diff_dates_to_days("2023-11-01", "2023-11-15").unwrap();
/// assert_eq!(days, 14);
/// ```
pub fn diff_dates_to_days(start_date: &str, end_date: &str) -> Result<i64> {
    let start_timestamp = date_str_to_timestamp(start_date)?;
    let end_timestamp = date_str_to_timestamp(end_date)?;

    Ok((end_timestamp - start_timestamp) / (24 * 60 * 60))
}

/// Converts seconds to a media timestamp format (HH:MM:SS or MM:SS).
///
/// For durations less than 1 hour, the format is MM:SS.
/// For durations 1 hour or more, the format is HH:MM:SS.
///
/// # Arguments
///
/// * `seconds` - The duration in seconds
///
/// # Returns
///
/// Returns a formatted timestamp string.
///
/// # Examples
///
/// ```
/// use cutil::time::seconds_to_media_timestamp;
///
/// assert_eq!(seconds_to_media_timestamp(123.0), "02:03");
/// assert_eq!(seconds_to_media_timestamp(3661.0), "01:01:01");
/// ```
pub fn seconds_to_media_timestamp(seconds: f64) -> String {
    let total_seconds = seconds as u32;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}

/// Converts seconds to a media timestamp format with milliseconds.
///
/// For durations less than 1 hour, the format is MM:SS.mmm.
/// For durations 1 hour or more, the format is HH:MM:SS.mmm.
///
/// # Arguments
///
/// * `seconds` - The duration in seconds
///
/// # Returns
///
/// Returns a formatted timestamp string with milliseconds.
///
/// # Examples
///
/// ```
/// use cutil::time::seconds_to_media_timestamp_with_ms;
///
/// assert_eq!(seconds_to_media_timestamp_with_ms(123.456), "02:03.456");
/// assert_eq!(seconds_to_media_timestamp_with_ms(3661.789), "01:01:01.789");
/// ```
pub fn seconds_to_media_timestamp_with_ms(seconds: f64) -> String {
    let total_seconds = seconds as u32;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;
    let ms = ((seconds - total_seconds as f64) * 1000.0) as u32;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, secs, ms)
    } else {
        format!("{:02}:{:02}.{:03}", minutes, secs, ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_matrix() -> Result<()> {
        // 测试2025年7月的日历
        let matrix = get_calendar_matrix(2025, 7)?;

        for row in &matrix {
            for date in row {
                print!("{:2}/{:2} ", date.month, date.day);
            }
            println!();
        }

        assert_eq!(matrix.len(), 6);
        assert!(matrix.iter().all(|row| row.len() == 7));

        assert_eq!(matrix[0][0].month, 6);
        assert_eq!(matrix[0][0].day, 29);

        // 测试2023年11月的日历 (11月1日是星期三)
        let matrix_nov = get_calendar_matrix(2023, 11)?;
        assert_eq!(matrix_nov[0][3].month, 11); // 11月1日应该在第四列(星期三)
        assert_eq!(matrix_nov[0][3].day, 1);
        assert_eq!(matrix_nov[0][0].month, 10); // 前面的应该是10月的日期
        assert_eq!(matrix_nov[4][6].month, 12); // 最后几个应该是12月的日期

        // 测试2023年2月的日历 (28天)
        let matrix_feb = get_calendar_matrix(2023, 2)?;
        assert_eq!(matrix_feb[0][2].month, 1); // 前面几天是1月的
        assert_eq!(matrix_feb[4][6].month, 3); // 最后几天是3月的

        Ok(())
    }

    #[test]
    fn test_date_str_to_timestamp() {
        assert!(date_str_to_timestamp("2005-12-09").is_ok());
        assert!(date_str_to_timestamp("2005-09-xxx").is_err());
    }

    #[test]
    fn test_diff_dates_to_days() {
        assert_eq!(diff_dates_to_days("2005-12-09", "2005-12-10").unwrap(), 1);
    }

    #[test]
    fn test_seconds_to_media_timestamp() {
        assert_eq!("02:03", seconds_to_media_timestamp(123.0));
        assert_eq!("01:01:01", seconds_to_media_timestamp(3661.0));
        assert_eq!("00:00", seconds_to_media_timestamp(0.0));
        assert_eq!("00:59", seconds_to_media_timestamp(59.0));
        assert_eq!("01:00:00", seconds_to_media_timestamp(3600.0));
    }

    #[test]
    fn test_seconds_to_media_timestamp_with_ms() {
        assert_eq!("02:03.456", seconds_to_media_timestamp_with_ms(123.456));
        assert_eq!("00:59.999", seconds_to_media_timestamp_with_ms(59.999));
    }
}
