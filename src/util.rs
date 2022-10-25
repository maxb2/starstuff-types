use crate::angle::{Angle, Degree, Hour, Radian, TWO_PI};

pub use chrono::prelude::*;

pub fn earth_rotation_angle(time_julian_ut1: JulianDate) -> Radian {
    // https://en.wikipedia.org/wiki/Sidereal_time

    Radian(TWO_PI * (0.7790572732640 + 1.00273781191135448 * (time_julian_ut1.0 - 2451545.0)))
}

#[derive(Debug)]
pub struct JulianDate(pub f64);

impl<T> From<DateTime<T>> for JulianDate
where
    T: chrono::TimeZone,
    chrono::DateTime<chrono::Utc>: From<chrono::DateTime<T>>,
{
    fn from(date: DateTime<T>) -> Self {
        // https://stackoverflow.com/a/52431241

        // need UTC date
        let date: DateTime<Utc> = date.into();
        assert!(
            1801 <= date.year() && date.year() <= 2099,
            "Datetime must be between year 1801 and 2099"
        );

        Self(
            // Note: all the integer divisions are truncating toward zero *by design*.
            (367 * date.year() - ((7 * (date.year() + (date.month() as i32 + 9) / 12)) / 4)
                + ((275 * date.month() as i32) / 9)) as f64
                + date.day() as f64
                + 1721013.5
                + (date.hour() as f64
                    + date.minute() as f64 / 60.0
                    + date.second() as f64 / 3600.0)
                    / 24.0
                - (0.5_f64).copysign(100.0 * date.year() as f64 + date.month() as f64 - 190002.5)
                + 0.5,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::util::*;

    #[test]
    fn test_juliandate() {
        // From: https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000
        // Definition of J2000 epoch
        assert_eq!(
            JulianDate::from(Local.ymd(2000, 1, 1).and_hms(12, 0, 0)).0,
            2451545.0
        );

        assert_eq!(
            JulianDate::from(Utc.ymd(2000, 1, 1).and_hms(12, 0, 0)).0,
            2451545.0
        );

        // From: https://en.wikipedia.org/wiki/Julian_day
        // 00:30:00.0 UT January 1, 2013, is 2_456_293.520_833
        assert!(
            (JulianDate::from(Local.ymd(2013, 1, 1).and_hms(0, 30, 0)).0 - 2_456_293.520_833).abs()
                < 1_e-6,
        );
        assert!(
            (JulianDate::from(Utc.ymd(2013, 1, 1).and_hms(0, 30, 0)).0 - 2_456_293.520_833).abs()
                < 1_e-6,
        );
    }
}