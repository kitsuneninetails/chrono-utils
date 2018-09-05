extern crate chrono;

use chrono::{DateTime, Datelike, TimeZone};

/// This trait defines functions which allow for by-month calculation and transformations.
/// Implementors of this trait should return new instances of themselves after applying
/// the appropriate month-based transformation.
pub trait MonthCalculations {
    /// Add a positive or negative number of months to self and return a new instance of self
    /// with the transformation applied.
    fn add_months(&self, num_months: i32) -> Self;

    /// Set the day of the month and return the resulting DateTime.  If the day cannot be set,
    /// because there is no such day in the month, etc., it will instead be set to the last
    /// day of the month (for example, using `with_closest_day(30)` on a February DateTime will
    /// result in a DateTime set to February 28 (non-leap year) or February 29 (leap year).
    fn with_closest_day(&self, day: u32) -> Self;
}

impl<Tz> MonthCalculations for DateTime<Tz> where Tz: TimeZone {
    fn add_months(&self, num_months: i32) -> Self {
        let abs_new_month = self.month0() as i32 + num_months;

        // This will be positive to move years forward, negative to move the years back.  In
        // the negative case, a full year will have to be moved back in addition to how many
        // are set here, because a negative value means we have to "borrow" a year (and hence
        // move the clock back an extra year to compensate) in order to make the months value
        // positive again.  The floor function will take care of this by lowering the value to the
        // next lower (i.e. higher absolute value) negative value.
        let years_change = (abs_new_month as f64 / 12f64).floor() as i32;

        // If start month < 0, add 12 to the modulus of the month (to make up for the year we
        // borrowed in the "floor" function above); since start month < 0, this will end up in a
        // value lower than 12).
        let actual_new_month = abs_new_month % 12 + { if abs_new_month >= 0 { 0 } else { 12 }};

        let new_date_year = self.with_year(self.year() + years_change).unwrap();

        new_date_year.with_day(1).unwrap()
            .with_month0(actual_new_month as u32)
            .expect("Value invalid: This means there is a very bad bug in the calculations!")
            .with_closest_day(new_date_year.day())
    }

    fn with_closest_day(&self, day: u32) -> Self {
        // Make sure the limit is 31 (as no month has more than 31 days)
        let check_day = if day > 31 { 31 } else { day };

        // Now check the day.  If the new month is :
        // * 0, 2, 4, 6, 7, 9, 11 (Jan, Mar, May, Jul, Aug, Oct, Dec)=> Use the day as-is,
        // * 3, 5, 8, or 10 (Feb, Apr, Jun, Sept, Nov) => Day is capped at 30,
        // * 1 (Feb) => Check leap year.  If yes, cap the day at 29, otherwise cap at 28.
        let actual_day = match self.month0() {
            0 | 2 | 4 | 6 | 7 | 9 | 11 => check_day,
            3 | 5 | 8 | 10 => if check_day > 30 { 30 } else { check_day },
            1 => {
                let is_leapyear = self
                    .with_day(1).unwrap()
                    .with_month(2).unwrap()
                    .with_day(29).is_some();
                if is_leapyear {
                    if check_day >= 30 { 29 } else { check_day }
                } else {
                    if check_day >= 29 { 28 } else { check_day }
                }
            },
            m => panic!("Month value of {} is invalid!", m),
        };
        self.with_day(actual_day)
            .expect("Value invalid: This means there is a very bad bug in the calculations!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day31() {
        let test_date = DateTime::parse_from_rfc3339("2017-03-31T12:00:00Z").unwrap();
        let new_date = test_date.add_months(1);
        assert_eq!(new_date.month(), 4);
        assert_eq!(new_date.day(), 30);
        assert_eq!(new_date.year(), 2017);
    }

    #[test]
    fn test_day29() {
        let test_date = DateTime::parse_from_rfc3339("2017-01-31T12:00:00Z").unwrap();
        let new_date = test_date.add_months(1);
        assert_eq!(new_date.month(), 2);
        assert_eq!(new_date.day(), 28);
        assert_eq!(new_date.year(), 2017);
    }

    #[test]
    fn test_day29_with_leapyear() {
        let test_date = DateTime::parse_from_rfc3339("2016-01-31T12:00:00Z").unwrap();
        let new_date = test_date.add_months(1);
        assert_eq!(new_date.month(), 2);
        assert_eq!(new_date.day(), 29);
        assert_eq!(new_date.year(), 2016);
    }

    #[test]
    fn test_advance_months_within_year_under_12() {
        let test_date = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let new_date = test_date.add_months(3);
        assert_eq!(new_date.month(), 6);
        assert_eq!(new_date.year(), 2018);
    }

    #[test]
    fn test_advance_months_forward_year_under_12() {
        let test_date = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let new_date = test_date.add_months(11);
        assert_eq!(new_date.month(), 2);
        assert_eq!(new_date.year(), 2019);
    }

    #[test]
    fn test_advance_months_over_12() {
        let test_date = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let new_date = test_date.add_months(23);
        assert_eq!(new_date.month(), 2);
        assert_eq!(new_date.year(), 2020);
    }

    #[test]
    fn test_regress_months_within_year_under_12() {
        let test_date = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let new_date = test_date.add_months(-2);
        assert_eq!(new_date.month(), 1);
        assert_eq!(new_date.year(), 2018);
    }

    #[test]
    fn test_regress_months_rollback_year_under_12() {
        let test_date = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let new_date = test_date.add_months(-7);
        assert_eq!(new_date.month(), 8);
        assert_eq!(new_date.year(), 2017);
    }

    #[test]
    fn test_regress_months_over_12() {
        let test_date = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let new_date = test_date.add_months(-23);
        assert_eq!(new_date.month(), 4);
        assert_eq!(new_date.year(), 2016);
    }

    #[test]
    fn test_zero() {
        let test_date = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let new_date = test_date.add_months(0);
        assert_eq!(new_date.month(), 3);
        assert_eq!(new_date.year(), 2018);
    }
}
