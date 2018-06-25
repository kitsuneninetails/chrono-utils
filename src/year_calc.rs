extern crate chrono;

use chrono::{DateTime, Datelike, TimeZone, Utc};
use std::cmp;

/// This trait defines functions which allow for year calculations between two dates.  As
/// the standard DateTime, Date, and Duration types in chrono are unable to do this (due to
/// complications with leap-years, etc.), a utility function must be added to calculate the
/// years between two DateTimes separately.
pub trait YearCalculations {
    /// Returns the number of years between Self and another DateTime as an integer.
    fn years_since<Tz2: TimeZone>(&self, b: &DateTime<Tz2>) -> i32;
}

fn cmp_month_day(a_utc: &DateTime<Utc>, b_utc: &DateTime<Utc>) -> i32 {
    match a_utc.month().cmp(&b_utc.month()) {
        cmp::Ordering::Greater => 0,
        cmp::Ordering::Less => -1,
        cmp::Ordering::Equal => match a_utc.day().cmp(&b_utc.day()) {
            cmp::Ordering::Greater | cmp::Ordering::Equal => 0,
            cmp::Ordering::Less => -1,
        }
    }
}

impl<Tz> YearCalculations for DateTime<Tz> where Tz: TimeZone {
    fn years_since<Tz2: TimeZone>(&self, b: &DateTime<Tz2>) -> i32 {
        let me_utc = self.with_timezone(&Utc);
        let b_utc = b.with_timezone(&Utc);

        let base_years = me_utc.year() - b_utc.year();

        match base_years.cmp(&0) {
            cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => base_years + cmp_month_day(&me_utc, &b_utc),
            cmp::Ordering::Less => base_years - cmp_month_day(&me_utc, &b_utc),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Format of fn name = test_years_ymd_xyz where
    /// x = year (b = before, a = after, s = same)
    /// y = year (b = before, a = after, s = same)
    /// z = year (b = before, a = after, s = same)
    #[test]
    fn test_years_ymd_bbb() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2010-01-11T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 8);
    }

    #[test]
    fn test_years_ymd_bba() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2010-01-21T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 8);
    }

    #[test]
    fn test_years_ymd_bbs() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2010-01-15T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 8);
    }

    #[test]
    fn test_years_ymd_bab() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2010-05-11T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 7);
    }

    #[test]
    fn test_years_ymd_baa() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2010-05-21T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 7);
    }

    #[test]
    fn test_years_ymd_bas() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2010-05-15T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 7);
    }

    #[test]
    fn test_years_ymd_bsb() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2010-03-11T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 8);
    }

    #[test]
    fn test_years_ymd_bsa() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2010-03-21T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 7);
    }

    #[test]
    fn test_years_ymd_bss() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2010-03-15T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 8);
    }

    #[test]
    fn test_years_ymd_abb() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2030-01-11T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), -12);
    }

    #[test]
    fn test_years_ymd_aba() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2030-01-21T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), -12);
    }

    #[test]
    fn test_years_ymd_abs() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2030-01-15T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), -12);
    }

    #[test]
    fn test_years_ymd_aab() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2030-06-11T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), -11);
    }

    #[test]
    fn test_years_ymd_aaa() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2030-06-21T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), -11);
    }

    #[test]
    fn test_years_ymd_aas() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2030-06-15T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), -11);
    }

    #[test]
    fn test_years_ymd_asb() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2030-03-11T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), -12);
    }

    #[test]
    fn test_years_ymd_asa() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2030-03-21T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), -11);
    }

    #[test]
    fn test_years_ymd_ass() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2030-03-15T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), -12);
    }

    #[test]
    fn test_years_ymd_sbb() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2018-01-11T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 0);
    }

    #[test]
    fn test_years_ymd_sba() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2018-01-21T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 0);
    }

    #[test]
    fn test_years_ymd_sbs() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2018-01-15T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 0);
    }

    #[test]
    fn test_years_ymd_sab() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2018-06-11T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 0);
    }

    #[test]
    fn test_years_ymd_saa() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2018-06-21T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 0);
    }

    #[test]
    fn test_years_ymd_sas() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2018-06-15T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 0);
    }

    #[test]
    fn test_years_ymd_ssb() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2018-03-11T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 0);
    }

    #[test]
    fn test_years_ymd_ssa() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2018-03-21T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 0);
    }

    #[test]
    fn test_years_ymd_sss() {
        let test_date1 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        let test_date2 = DateTime::parse_from_rfc3339("2018-03-15T12:00:00Z").unwrap();
        assert_eq!(test_date1.years_since(&test_date2), 0);
    }
}
