use std::f64::consts::PI;

pub const TWO_PI: f64 = 2.0 * PI;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Degree(pub f64);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Radian(pub f64);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hour(pub f64);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Angle {
    Degree(Degree),
    Radian(Radian),
    Hour(Hour),
}

impl From<Radian> for Degree {
    fn from(rad: Radian) -> Self {
        const RAD_TO_DEG: f64 = 180.0 / PI;
        Self(rad.0 * RAD_TO_DEG)
    }
}

impl From<Hour> for Degree {
    fn from(hr: Hour) -> Self {
        const HR_TO_DEG: f64 = 15.0;
        Self(hr.0 * HR_TO_DEG)
    }
}

impl From<Radian> for Hour {
    fn from(rad: Radian) -> Self {
        const RAD_TO_HR: f64 = 180.0 / PI / 15.0;
        Self(rad.0 * RAD_TO_HR)
    }
}

impl From<Degree> for Hour {
    fn from(deg: Degree) -> Self {
        const DEG_TO_HR: f64 = 1.0 / 15.0;
        Self(deg.0 * DEG_TO_HR)
    }
}

impl From<Hour> for Radian {
    fn from(hr: Hour) -> Self {
        const HR_TO_RAD: f64 = 15.0 * PI / 180.0;
        Self(hr.0 * HR_TO_RAD)
    }
}

impl From<Degree> for Radian {
    fn from(deg: Degree) -> Self {
        const DEG_TO_RAD: f64 = PI / 180.0;
        Self(deg.0 * DEG_TO_RAD)
    }
}

impl From<Degree> for Angle {
    fn from(deg: Degree) -> Self {
        Self::Degree(deg)
    }
}

impl From<Radian> for Angle {
    fn from(rad: Radian) -> Self {
        Self::Radian(rad)
    }
}

impl From<Hour> for Angle {
    fn from(hr: Hour) -> Self {
        Self::Hour(hr)
    }
}

impl From<Angle> for Degree {
    fn from(angle: Angle) -> Self {
        match angle {
            Angle::Degree(a) => a,
            Angle::Radian(a) => Degree::from(a),
            Angle::Hour(a) => Degree::from(a),
        }
    }
}

impl From<Angle> for Radian {
    fn from(angle: Angle) -> Self {
        match angle {
            Angle::Degree(a) => Radian::from(a),
            Angle::Radian(a) => a,
            Angle::Hour(a) => Radian::from(a),
        }
    }
}

impl From<Angle> for Hour {
    fn from(angle: Angle) -> Self {
        match angle {
            Angle::Degree(a) => Hour::from(a),
            Angle::Radian(a) => Hour::from(a),
            Angle::Hour(a) => a,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
pub struct Dms(Sign, u32, u32, f64);

#[derive(Debug, PartialEq)]
pub struct Hms(Sign, u32, u32, f64);

pub trait ArcMinuteSecond {
    fn new(sign: Sign, major: u32, minute: u32, second: f64) -> Self;
    fn angle_to_ams(decimal: f64) -> Self
    where
        Self: std::marker::Sized,
    {
        let major: u32 = decimal as u32;
        let min: f64 = (decimal - (major as f64)) * 60.0;
        let second: f64 = (min - (min as u32 as f64)) * 60.0;
        Self::new(
            if decimal.is_sign_positive() {
                Sign::Plus
            } else {
                Sign::Minus
            },
            major,
            min as u32,
            second,
        )
    }
}

impl ArcMinuteSecond for Dms {
    fn new(sign: Sign, degree: u32, minute: u32, second: f64) -> Self {
        Self(sign, degree, minute, second)
    }
}

impl ArcMinuteSecond for Hms {
    fn new(sign: Sign, hour: u32, minute: u32, second: f64) -> Self {
        Self(sign, hour, minute, second)
    }
}

// I can't figure out how to use generic type parameters in From<T> :sad_face:

impl From<Angle> for Dms {
    fn from(angle: Angle) -> Self {
        Self::angle_to_ams(Degree::from(angle).0)
    }
}

impl From<Degree> for Dms {
    fn from(angle: Degree) -> Self {
        Self::angle_to_ams(angle.0)
    }
}

impl From<Radian> for Dms {
    fn from(angle: Radian) -> Self {
        Self::angle_to_ams(Degree::from(angle).0)
    }
}

impl From<Hour> for Dms {
    fn from(angle: Hour) -> Self {
        Self::angle_to_ams(Degree::from(angle).0)
    }
}

impl From<Angle> for Hms {
    fn from(angle: Angle) -> Self {
        Self::angle_to_ams(Hour::from(angle).0)
    }
}

impl From<Degree> for Hms {
    fn from(angle: Degree) -> Self {
        Self::angle_to_ams(Hour::from(angle).0)
    }
}

impl From<Radian> for Hms {
    fn from(angle: Radian) -> Self {
        Self::angle_to_ams(Hour::from(angle).0)
    }
}

impl From<Hour> for Hms {
    fn from(angle: Hour) -> Self {
        Self::angle_to_ams(angle.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::angle::*;

    #[test]
    fn test_angle() {
        assert_eq!(Degree(90.0), Degree::from(Radian(PI / 2.0)));
        assert_eq!(Degree(90.0), Degree::from(Hour(6.0)));
        assert_eq!(Hour(6.0), Hour::from(Radian(PI / 2.0)));
        assert_eq!(Hour(6.0), Hour::from(Degree(90.0)));
        assert_eq!(Radian(PI / 2.0), Radian::from(Hour(6.0)));
        assert_eq!(Radian(PI / 2.0), Radian::from(Degree(90.0)));
    }

    #[test]
    fn test_ams() {
        // Dms
        assert_eq!(Dms::from(Degree(0.0)), Dms::new(Sign::Plus, 0, 0, 0.0));
        assert_eq!(Dms::from(Degree(-0.0)), Dms::new(Sign::Minus, 0, 0, 0.0));
        assert_eq!(Dms::from(Degree(180.0)), Dms::new(Sign::Plus, 180, 0, 0.0));
        assert_eq!(Dms::from(Radian(0.0)), Dms::new(Sign::Plus, 0, 0, 0.0));
        assert_eq!(Dms::from(Radian(-0.0)), Dms::new(Sign::Minus, 0, 0, 0.0));
        assert_eq!(Dms::from(Radian(PI)), Dms::new(Sign::Plus, 180, 0, 0.0));
        assert_eq!(Dms::from(Hour(12.0)), Dms::new(Sign::Plus, 180, 0, 0.0));
        // Hms
        assert_eq!(Hms::from(Degree(0.0)), Hms::new(Sign::Plus, 0, 0, 0.0));
        assert_eq!(Hms::from(Degree(-0.0)), Hms::new(Sign::Minus, 0, 0, 0.0));
        assert_eq!(Hms::from(Hour(12.0)), Hms::new(Sign::Plus, 12, 0, 0.0));
        assert_eq!(Hms::from(Radian(0.0)), Hms::new(Sign::Plus, 0, 0, 0.0));
        assert_eq!(Hms::from(Radian(-0.0)), Hms::new(Sign::Minus, 0, 0, 0.0));
        assert_eq!(Hms::from(Radian(PI)), Hms::new(Sign::Plus, 12, 0, 0.0));
        assert_eq!(Hms::from(Degree(180.0)), Hms::new(Sign::Plus, 12, 0, 0.0));
    }
}
