/*!
Astonomical time types and utilities
 */

use crate::angle::{Angle, TWO_PI};

pub use chrono::DateTime;
use chrono::{Datelike, Timelike, Utc};

/**
Earth Rotation Angle

<https://en.wikipedia.org/wiki/Sidereal_time>
 */
#[allow(clippy::excessive_precision)] // NOTE: actual equation has that much precision
pub fn earth_rotation_angle(time_julian_ut1: JulianDate) -> Angle {
    Angle::Radian(
        TWO_PI * (0.7790572732640 + 1.00273781191135448 * (time_julian_ut1.0 - 2451545.0)),
    )
}

/// Julian Date (J2000 epoch)
#[derive(Debug)]
pub struct JulianDate(
    /// Seconds since J2000
    pub f64,
);

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
            "Datetime must be between year 1801 and 2099" // TODO: Find an algorithm that has a wider range.
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

/// Greenwich Mean Sidereal Time
pub struct GMST(
    /// Hour
    pub Angle,
);

impl From<JulianDate> for GMST {
    fn from(julian_date: JulianDate) -> Self {
        // https://en.wikipedia.org/wiki/Sidereal_time
        Self(Angle::Hour(earth_rotation_angle(julian_date).to_hr()))
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use crate::time::*;

    #[test]
    fn test_juliandate() {
        // From: https://en.wikipedia.org/wiki/Epoch_(astronomy)#J2000
        // Definition of J2000 epoch
        assert_eq!(
            JulianDate::from(Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap()).0,
            2451545.0
        );

        // From: https://en.wikipedia.org/wiki/Julian_day
        // 00:30:00.0 UT January 1, 2013, is 2_456_293.520_833
        assert_float_absolute_eq!(
            JulianDate::from(Utc.with_ymd_and_hms(2013, 1, 1, 0, 30, 0).unwrap()).0,
            2_456_293.520_833,
            1e-6
        );
    }
}
