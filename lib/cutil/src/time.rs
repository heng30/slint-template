use anyhow::{Context, Result};
use chrono::{Datelike, Duration, Local, NaiveDate, TimeZone, Weekday};

#[derive(Debug, Clone)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

// "%Y-%m-%d %H:%M:%S" -> 2023-11-15 14:30:45
pub fn local_now(format: &str) -> String {
    Local::now().format(format).to_string()
}

pub fn get_current_date() -> Date {
    let now = Local::now();

    Date {
        year: now.year(),
        month: now.month(),
        day: now.day(),
    }
}

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

pub fn timestamp() -> i64 {
    Local::now().timestamp()
}

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

pub fn date_str_to_timestamp(date_str: &str) -> Result<i64> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    let datetime = date
        .and_hms_opt(0, 0, 0)
        .ok_or(anyhow::anyhow!("Invalid time specification"))?;
    Ok(Local.from_local_datetime(&datetime).unwrap().timestamp())
}

pub fn diff_dates_to_days(start_date: &str, end_date: &str) -> Result<i64> {
    let start_timestamp = date_str_to_timestamp(start_date)?;
    let end_timestamp = date_str_to_timestamp(end_date)?;

    Ok((end_timestamp - start_timestamp) / (24 * 60 * 60))
}

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
