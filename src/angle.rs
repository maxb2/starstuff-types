/*!
Basic angle structures (Radian, Degree, Arc Hour)
 */

use auto_ops::*;

pub use std::f64::consts::PI;

/// 2π
pub const TWO_PI: f64 = 2.0 * PI;
/// π/2
pub const PI_HALF: f64 = PI / 2.0;
/// π/4
pub const PI_FOURTH: f64 = PI / 4.0;

/// Enum containing common angle variants
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Angle {
    /// Variant containing decimal degree
    Degree(f64),
    /// Variant containing decimal radian
    Radian(f64),
    /// Variant containing decimal hour
    Hour(f64),
}

impl Angle {
    /// Convert to decimal degree
    pub fn to_deg(&self) -> f64 {
        match self {
            Self::Degree(deg) => *deg,
            Self::Hour(hr) => 15.0 * hr,
            Self::Radian(rad) => {
                const RAD_TO_DEG: f64 = 180.0 / PI;
                RAD_TO_DEG * rad
            }
        }
    }
    /// Convert to decimal radian
    pub fn to_rad(&self) -> f64 {
        match self {
            Self::Degree(deg) => {
                const DEG_TO_RAD: f64 = PI / 180.0;
                DEG_TO_RAD * deg
            }
            Self::Hour(hr) => {
                const HOUR_TO_RAD: f64 = 15.0 * PI / 180.0;
                HOUR_TO_RAD * hr
            }
            Self::Radian(rad) => *rad,
        }
    }
    /// Convert to decimal hour
    pub fn to_hr(&self) -> f64 {
        match self {
            Self::Degree(deg) => deg / 15.0,
            Self::Hour(hr) => *hr,
            Self::Radian(rad) => {
                const RAD_TO_HOUR: f64 = 180.0 / PI / 15.0;
                RAD_TO_HOUR * rad
            }
        }
    }
    pub fn sin(&self) -> f64 {
        Self::to_rad(self).sin()
    }
    pub fn cos(&self) -> f64 {
        Self::to_rad(self).cos()
    }
    pub fn tan(&self) -> f64 {
        Self::to_rad(self).tan()
    }
}

impl_op_ex!(+ |a: &Angle, b: &Angle| -> Angle { match a {
    Angle::Degree(deg) => Angle::Degree(deg + b.to_deg()),
    Angle::Radian(rad) => Angle::Degree(rad + b.to_rad()),
    Angle::Hour(hr) => Angle::Degree(hr + b.to_hr()),
} });

impl_op_ex!(-|a: &Angle, b: &Angle| -> Angle {
    match a {
        Angle::Degree(deg) => Angle::Degree(deg - b.to_deg()),
        Angle::Radian(rad) => Angle::Degree(rad - b.to_rad()),
        Angle::Hour(hr) => Angle::Degree(hr - b.to_hr()),
    }
});

/// Enum representing sign of number
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Sign {
    Positive,
    Negative,
}

/// Sexagesimal Degree
#[derive(Debug, PartialEq, Clone)]
pub struct DegMinSec(
    /// Sign of angle
    pub Sign,
    /// Degrees
    pub u32,
    /// Arc minutes
    pub u32,
    /// Arc seconds
    pub f64,
);

/// Sexagesimal Hour
#[derive(Debug, PartialEq, Clone)]
pub struct HourMinSec(
    /// Sign of angle
    pub Sign,
    /// Hours
    pub u32,
    /// Minutes
    pub u32,
    /// Seconds
    pub f64,
);

/**
Angle to sexagesimal angle implementation

> NOTE: this is a macro rather than a public trait so I can implement a type trait without repeating myself for multiple types.

*/
macro_rules! impl_arc_minute_second {
    ($T:ty) => {
        impl $T {
            /**
            Decimal angle to Arc Minute Second struct.
            */
            pub fn angle_to_ams(decimal_angle: f64) -> Self {
                let major: u32 = decimal_angle as u32;
                let min: f64 = (decimal_angle - (major as f64)) * 60.0;
                let second: f64 = (min - (min as u32 as f64)) * 60.0;
                Self(
                    if decimal_angle.is_sign_positive() {
                        Sign::Positive
                    } else {
                        Sign::Negative
                    },
                    major,
                    min as u32,
                    second,
                )
            }
        }
    };
}

impl_arc_minute_second!(DegMinSec);
impl_arc_minute_second!(HourMinSec);

impl From<Angle> for DegMinSec {
    fn from(angle: Angle) -> Self {
        Self::angle_to_ams(angle.to_deg())
    }
}

impl From<Angle> for HourMinSec {
    fn from(angle: Angle) -> Self {
        Self::angle_to_ams(angle.to_hr())
    }
}

impl From<DegMinSec> for Angle {
    fn from(angle: DegMinSec) -> Angle {
        let angle_abs: f64 = angle.1 as f64 + angle.2 as f64 / 60.0 + angle.3 / 3600.0;
        match angle.0 {
            Sign::Positive => Angle::Degree(angle_abs),
            Sign::Negative => Angle::Degree(-angle_abs),
        }
    }
}

impl From<HourMinSec> for Angle {
    fn from(angle: HourMinSec) -> Angle {
        let angle_abs: f64 = angle.1 as f64 + angle.2 as f64 / 60.0 + angle.3 / 3600.0;
        match angle.0 {
            Sign::Positive => Angle::Hour(angle_abs),
            Sign::Negative => Angle::Hour(-angle_abs),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::angle::*;

    const PI_THIRD: f64 = PI / 3.0;
    const PI_SIXTH: f64 = PI / 6.0;

    #[test]
    fn angle_enum() {
        // converting between types
        assert_float_absolute_eq!(
            Angle::Degree(90.0).to_deg(),
            Angle::Radian(PI_HALF).to_deg()
        );
        assert_float_absolute_eq!(Angle::Degree(90.0).to_deg(), Angle::Hour(6.0).to_deg());
        assert_float_absolute_eq!(Angle::Hour(6.0).to_deg(), Angle::Radian(PI_HALF).to_deg());
        assert_float_absolute_eq!(Angle::Hour(6.0).to_deg(), Angle::Degree(90.0).to_deg());
        assert_float_absolute_eq!(Angle::Radian(PI_HALF).to_deg(), Angle::Hour(6.0).to_deg());
        assert_float_absolute_eq!(
            Angle::Radian(PI_HALF).to_deg(),
            Angle::Degree(90.0).to_deg()
        );

        // trigonometry
        assert_float_absolute_eq!(1.0, Angle::Degree(0.0).cos());
        assert_float_absolute_eq!(1.0, Angle::Hour(0.0).cos());
        assert_float_absolute_eq!(1.0, Angle::Radian(0.0).cos());

        assert_float_absolute_eq!(0.0, Angle::Degree(90.0).cos());
        assert_float_absolute_eq!(0.0, Angle::Hour(6.0).cos());
        assert_float_absolute_eq!(0.0, Angle::Radian(PI_HALF).cos());

        assert_float_absolute_eq!(0.5, Angle::Degree(60.0).cos());
        assert_float_absolute_eq!(0.5, Angle::Hour(4.0).cos());
        assert_float_absolute_eq!(0.5, Angle::Radian(PI_THIRD).cos());

        assert_float_absolute_eq!(2_f64.sqrt() / 2.0, Angle::Degree(45.0).cos());
        assert_float_absolute_eq!(2_f64.sqrt() / 2.0, Angle::Hour(3.0).cos());
        assert_float_absolute_eq!(2_f64.sqrt() / 2.0, Angle::Radian(PI_FOURTH).cos());

        assert_float_absolute_eq!(0.0, Angle::Degree(0.0).sin());
        assert_float_absolute_eq!(0.0, Angle::Hour(0.0).sin());
        assert_float_absolute_eq!(0.0, Angle::Radian(0.0).sin());

        assert_float_absolute_eq!(1.0, Angle::Degree(90.0).sin());
        assert_float_absolute_eq!(1.0, Angle::Hour(6.0).sin());
        assert_float_absolute_eq!(1.0, Angle::Radian(PI_HALF).sin());

        assert_float_absolute_eq!(3_f64.sqrt() / 2.0, Angle::Degree(60.0).sin());
        assert_float_absolute_eq!(3_f64.sqrt() / 2.0, Angle::Hour(4.0).sin());
        assert_float_absolute_eq!(3_f64.sqrt() / 2.0, Angle::Radian(PI_THIRD).sin());

        assert_float_absolute_eq!(2_f64.sqrt() / 2.0, Angle::Degree(45.0).sin());
        assert_float_absolute_eq!(2_f64.sqrt() / 2.0, Angle::Hour(3.0).sin());
        assert_float_absolute_eq!(2_f64.sqrt() / 2.0, Angle::Radian(PI_FOURTH).sin());

        assert_float_absolute_eq!(0.5, Angle::Degree(30.0).sin());
        assert_float_absolute_eq!(0.5, Angle::Hour(2.0).sin());
        assert_float_absolute_eq!(0.5, Angle::Radian(PI_SIXTH).sin());

        assert_float_absolute_eq!(1.0, Angle::Degree(45.0).tan());
        assert_float_absolute_eq!(1.0, Angle::Hour(3.0).tan());
        assert_float_absolute_eq!(1.0, Angle::Radian(PI_FOURTH).tan());
    }

    #[test]
    fn arc_min_sec() {
        // DegMinSec
        assert_eq!(
            DegMinSec::from(Angle::Degree(0.0)),
            DegMinSec(Sign::Positive, 0, 0, 0.0)
        );
        assert_eq!(
            DegMinSec::from(Angle::Degree(-0.0)),
            DegMinSec(Sign::Negative, 0, 0, 0.0)
        );
        assert_eq!(
            DegMinSec::from(Angle::Degree(180.0)),
            DegMinSec(Sign::Positive, 180, 0, 0.0)
        );
        assert_eq!(
            DegMinSec::from(Angle::Radian(0.0)),
            DegMinSec(Sign::Positive, 0, 0, 0.0)
        );
        assert_eq!(
            DegMinSec::from(Angle::Radian(-0.0)),
            DegMinSec(Sign::Negative, 0, 0, 0.0)
        );
        assert_eq!(
            DegMinSec::from(Angle::Radian(PI)),
            DegMinSec(Sign::Positive, 180, 0, 0.0)
        );
        assert_eq!(
            DegMinSec::from(Angle::Hour(12.0)),
            DegMinSec(Sign::Positive, 180, 0, 0.0)
        );
        // HourMinSec
        assert_eq!(
            HourMinSec::from(Angle::Degree(0.0)),
            HourMinSec(Sign::Positive, 0, 0, 0.0)
        );
        assert_eq!(
            HourMinSec::from(Angle::Degree(-0.0)),
            HourMinSec(Sign::Negative, 0, 0, 0.0)
        );
        assert_eq!(
            HourMinSec::from(Angle::Hour(12.0)),
            HourMinSec(Sign::Positive, 12, 0, 0.0)
        );
        assert_eq!(
            HourMinSec::from(Angle::Radian(0.0)),
            HourMinSec(Sign::Positive, 0, 0, 0.0)
        );
        assert_eq!(
            HourMinSec::from(Angle::Radian(-0.0)),
            HourMinSec(Sign::Negative, 0, 0, 0.0)
        );
        assert_eq!(
            HourMinSec::from(Angle::Radian(PI)),
            HourMinSec(Sign::Positive, 12, 0, 0.0)
        );
        assert_eq!(
            HourMinSec::from(Angle::Degree(180.0)),
            HourMinSec(Sign::Positive, 12, 0, 0.0)
        );
    }
}
