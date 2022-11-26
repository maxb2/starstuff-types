use auto_ops::*;

pub use std::f64::consts::PI;

pub const TWO_PI: f64 = 2.0 * PI;
pub const PI_HALF: f64 = PI / 2.0;
pub const PI_FOURTH: f64 = PI / 4.0;

pub trait AngleTrig {
    fn sin(&self) -> f64;
    fn asin(&self) -> f64;
    fn cos(&self) -> f64;
    fn acos(&self) -> f64;
    // self.atan2(other) => atan2(self, other) => atan(self/other)
    fn atan2(&self, other: &Self) -> f64;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DegData(pub f64);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RadData(pub f64);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HourData(pub f64);

impl_op_ex!(+ |a: &DegData, b: &DegData| -> DegData { DegData(a.0 + b.0) });
impl_op_ex!(+ |a: &HourData, b: &HourData| -> HourData { HourData(a.0 + b.0) });
impl_op_ex!(+ |a: &RadData, b: &RadData| -> RadData { RadData(a.0 + b.0) });

impl_op_ex!(-|a: &DegData, b: &DegData| -> DegData { DegData(a.0 - b.0) });
impl_op_ex!(-|a: &HourData, b: &HourData| -> HourData { HourData(a.0 - b.0) });
impl_op_ex!(-|a: &RadData, b: &RadData| -> RadData { RadData(a.0 - b.0) });

impl AngleTrig for RadData {
    fn sin(&self) -> f64 {
        self.0.sin()
    }
    fn asin(&self) -> f64 {
        self.0.sin()
    }
    fn cos(&self) -> f64 {
        self.0.cos()
    }
    fn acos(&self) -> f64 {
        self.0.acos()
    }
    fn atan2(&self, other: &Self) -> f64 {
        self.0.atan2(other.0)
    }
}

impl AngleTrig for DegData {
    fn sin(&self) -> f64 {
        RadData::from(*self).sin()
    }
    fn asin(&self) -> f64 {
        RadData::from(*self).sin()
    }
    fn cos(&self) -> f64 {
        RadData::from(*self).cos()
    }
    fn acos(&self) -> f64 {
        RadData::from(*self).acos()
    }
    fn atan2(&self, other: &Self) -> f64 {
        RadData::from(*self).atan2(&RadData::from(*other))
    }
}

impl AngleTrig for HourData {
    fn sin(&self) -> f64 {
        RadData::from(*self).sin()
    }
    fn asin(&self) -> f64 {
        RadData::from(*self).sin()
    }
    fn cos(&self) -> f64 {
        RadData::from(*self).cos()
    }
    fn acos(&self) -> f64 {
        RadData::from(*self).acos()
    }
    fn atan2(&self, other: &Self) -> f64 {
        RadData::from(*self).atan2(&RadData::from(*other))
    }
}

impl AngleTrig for Angle {
    fn sin(&self) -> f64 {
        RadData::from(*self).sin()
    }
    fn asin(&self) -> f64 {
        RadData::from(*self).sin()
    }
    fn cos(&self) -> f64 {
        RadData::from(*self).cos()
    }
    fn acos(&self) -> f64 {
        RadData::from(*self).acos()
    }
    fn atan2(&self, other: &Self) -> f64 {
        RadData::from(*self).atan2(&RadData::from(*other))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Angle {
    Degree(DegData),
    Radian(RadData),
    Hour(HourData),
}

impl From<RadData> for DegData {
    fn from(rad: RadData) -> Self {
        const RAD_TO_DEG: f64 = 180.0 / PI;
        Self(rad.0 * RAD_TO_DEG)
    }
}

impl From<HourData> for DegData {
    fn from(hr: HourData) -> Self {
        const HR_TO_DEG: f64 = 15.0;
        Self(hr.0 * HR_TO_DEG)
    }
}

impl From<RadData> for HourData {
    fn from(rad: RadData) -> Self {
        const RAD_TO_HR: f64 = 180.0 / PI / 15.0;
        Self(rad.0 * RAD_TO_HR)
    }
}

impl From<DegData> for HourData {
    fn from(deg: DegData) -> Self {
        const DEG_TO_HR: f64 = 1.0 / 15.0;
        Self(deg.0 * DEG_TO_HR)
    }
}

impl From<HourData> for RadData {
    fn from(hr: HourData) -> Self {
        const HR_TO_RAD: f64 = 15.0 * PI / 180.0;
        Self(hr.0 * HR_TO_RAD)
    }
}

impl From<DegData> for RadData {
    fn from(deg: DegData) -> Self {
        const DEG_TO_RAD: f64 = PI / 180.0;
        Self(deg.0 * DEG_TO_RAD)
    }
}

impl From<DegData> for Angle {
    fn from(deg: DegData) -> Self {
        Self::Degree(deg)
    }
}

impl From<RadData> for Angle {
    fn from(rad: RadData) -> Self {
        Self::Radian(rad)
    }
}

impl From<HourData> for Angle {
    fn from(hr: HourData) -> Self {
        Self::Hour(hr)
    }
}

impl From<Angle> for DegData {
    fn from(angle: Angle) -> Self {
        match angle {
            Angle::Degree(a) => a,
            Angle::Radian(a) => DegData::from(a),
            Angle::Hour(a) => DegData::from(a),
        }
    }
}

impl From<Angle> for RadData {
    fn from(angle: Angle) -> Self {
        match angle {
            Angle::Degree(a) => RadData::from(a),
            Angle::Radian(a) => a,
            Angle::Hour(a) => RadData::from(a),
        }
    }
}

impl From<Angle> for HourData {
    fn from(angle: Angle) -> Self {
        match angle {
            Angle::Degree(a) => HourData::from(a),
            Angle::Radian(a) => HourData::from(a),
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
        Self::angle_to_ams(DegData::from(angle).0)
    }
}

impl From<DegData> for Dms {
    fn from(angle: DegData) -> Self {
        Self::angle_to_ams(angle.0)
    }
}

impl From<RadData> for Dms {
    fn from(angle: RadData) -> Self {
        Self::angle_to_ams(DegData::from(angle).0)
    }
}

impl From<HourData> for Dms {
    fn from(angle: HourData) -> Self {
        Self::angle_to_ams(DegData::from(angle).0)
    }
}

impl From<Angle> for Hms {
    fn from(angle: Angle) -> Self {
        Self::angle_to_ams(HourData::from(angle).0)
    }
}

impl From<DegData> for Hms {
    fn from(angle: DegData) -> Self {
        Self::angle_to_ams(HourData::from(angle).0)
    }
}

impl From<RadData> for Hms {
    fn from(angle: RadData) -> Self {
        Self::angle_to_ams(HourData::from(angle).0)
    }
}

impl From<HourData> for Hms {
    fn from(angle: HourData) -> Self {
        Self::angle_to_ams(angle.0)
    }
}

// TODO: DRY this somehow.

impl From<Hms> for HourData {
    fn from(angle: Hms) -> HourData {
        let angle_abs: f64 = angle.1 as f64 + angle.2 as f64 / 60.0 + angle.3 as f64 / 3600.0;
        match angle.0 {
            Sign::Plus => HourData(angle_abs),
            Sign::Minus => HourData(-angle_abs),
        }
    }
}

impl From<Dms> for DegData {
    fn from(angle: Dms) -> DegData {
        let angle_abs: f64 = angle.1 as f64 + angle.2 as f64 / 60.0 + angle.3 as f64 / 3600.0;
        match angle.0 {
            Sign::Plus => DegData(angle_abs),
            Sign::Minus => DegData(-angle_abs),
        }
    }
}

impl From<Dms> for Angle {
    fn from(dms: Dms) -> Angle {
        Angle::from(DegData::from(dms))
    }
}

impl From<Hms> for Angle {
    fn from(hms: Hms) -> Angle {
        Angle::from(HourData::from(hms))
    }
}

#[cfg(test)]
mod tests {
    use crate::angle::*;

    #[test]
    fn test_angle() {
        assert_eq!(DegData(90.0), DegData::from(RadData(PI / 2.0)));
        assert_eq!(DegData(90.0), DegData::from(HourData(6.0)));
        assert_eq!(HourData(6.0), HourData::from(RadData(PI / 2.0)));
        assert_eq!(HourData(6.0), HourData::from(DegData(90.0)));
        assert_eq!(RadData(PI / 2.0), RadData::from(HourData(6.0)));
        assert_eq!(RadData(PI / 2.0), RadData::from(DegData(90.0)));
    }

    #[test]
    fn test_ams() {
        // Dms
        assert_eq!(Dms::from(DegData(0.0)), Dms::new(Sign::Plus, 0, 0, 0.0));
        assert_eq!(Dms::from(DegData(-0.0)), Dms::new(Sign::Minus, 0, 0, 0.0));
        assert_eq!(Dms::from(DegData(180.0)), Dms::new(Sign::Plus, 180, 0, 0.0));
        assert_eq!(Dms::from(RadData(0.0)), Dms::new(Sign::Plus, 0, 0, 0.0));
        assert_eq!(Dms::from(RadData(-0.0)), Dms::new(Sign::Minus, 0, 0, 0.0));
        assert_eq!(Dms::from(RadData(PI)), Dms::new(Sign::Plus, 180, 0, 0.0));
        assert_eq!(Dms::from(HourData(12.0)), Dms::new(Sign::Plus, 180, 0, 0.0));
        // Hms
        assert_eq!(Hms::from(DegData(0.0)), Hms::new(Sign::Plus, 0, 0, 0.0));
        assert_eq!(Hms::from(DegData(-0.0)), Hms::new(Sign::Minus, 0, 0, 0.0));
        assert_eq!(Hms::from(HourData(12.0)), Hms::new(Sign::Plus, 12, 0, 0.0));
        assert_eq!(Hms::from(RadData(0.0)), Hms::new(Sign::Plus, 0, 0, 0.0));
        assert_eq!(Hms::from(RadData(-0.0)), Hms::new(Sign::Minus, 0, 0, 0.0));
        assert_eq!(Hms::from(RadData(PI)), Hms::new(Sign::Plus, 12, 0, 0.0));
        assert_eq!(Hms::from(DegData(180.0)), Hms::new(Sign::Plus, 12, 0, 0.0));
    }
}
