use chrono::{ Datelike, Days, NaiveDate };

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    let date = NaiveDate::from_yo_opt(year as i32, 365)?;
    if date.leap_year() {
        return None;
    }
    let middle_day = date.checked_sub_days(Days::new(182)).unwrap().weekday();
    Some(middle_day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(middle_day(1022), Some(chrono::Weekday::Tue));
    }
}
