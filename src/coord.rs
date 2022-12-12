use crate::angle::{Angle, PI, PI_FOURTH, PI_HALF};
use crate::time::GMST;

pub trait ConstrainedAngle {
    fn new(angle: &Angle) -> Self;
    fn value(&self) -> Angle;
}

#[derive(Debug, Copy, Clone)]
pub struct ZenithAngle(pub Angle);

impl ConstrainedAngle for ZenithAngle {
    fn new(angle: &Angle) -> Self {
        let rad = angle.to_rad();
        if !(0.0..=PI).contains(&rad) {
            panic!("ZenithAngle must be between 0 and pi!")
        }
        Self(*angle)
    }
    fn value(&self) -> Angle {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Declination(pub Angle);

impl ConstrainedAngle for Declination {
    fn new(angle: &Angle) -> Self {
        let rad = angle.to_rad();
        if !(-PI_HALF..=PI_HALF).contains(&rad) {
            panic!("Declination must be between -pi/2 and pi/2!")
        }
        Self(*angle)
    }
    fn value(&self) -> Angle {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Altitude(pub Angle);

impl ConstrainedAngle for Altitude {
    fn new(angle: &Angle) -> Self {
        let rad = angle.to_rad();
        if !(-PI_HALF..=PI_HALF).contains(&rad) {
            panic!("Altitude must be between -pi/2 and pi/2!")
        }
        Self(*angle)
    }
    fn value(&self) -> Angle {
        self.0
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Latitude(pub Angle);

impl ConstrainedAngle for Latitude {
    fn new(angle: &Angle) -> Self {
        let rad = angle.to_rad();
        if !(-PI_HALF..=PI_HALF).contains(&rad) {
            panic!("Latitude must be between -pi/2 and pi/2!")
        }
        Self(*angle)
    }
    fn value(&self) -> Angle {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RightAscension(pub Angle);

#[derive(Debug, Copy, Clone)]
pub struct Azimuth(pub Angle);

#[derive(Debug, Copy, Clone)]
pub struct Longitude(pub Angle);

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct Cartesian {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Geographic {
    pub latitude: Latitude,
    pub longitude: Longitude,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct Polar {
    pub radius: f64,
    pub angle: Angle,
}

#[derive(Debug, Copy, Clone)]
pub struct Equitorial {
    pub right_ascension: RightAscension,
    pub declination: Declination,
}

#[derive(Debug, Copy, Clone)]
pub struct Horizontal {
    pub altitude: Altitude,
    pub azimuth: Azimuth,
}

impl Horizontal {
    pub fn from_equitorial(eq: &Equitorial, geo: &Geographic, sidereal_time: &GMST) -> Self {
        let hour_local: Angle = sidereal_time.0 + geo.longitude.0 - eq.right_ascension.0;
        let x_horiz: f64 = -(geo.latitude.0.sin()) * (eq.declination.0.cos()) * (hour_local.cos())
            + geo.latitude.0.cos() * (eq.declination.0.sin());
        let y_horiz: f64 = eq.declination.0.cos() * hour_local.sin();
        let azimuth_rad: Angle = Angle::Radian(-(y_horiz.atan2(x_horiz)));
        let altitude_rad: Angle = Angle::Radian(
            (geo.latitude.0.sin() * eq.declination.0.sin()
                + geo.latitude.0.cos() * eq.declination.0.cos() * hour_local.cos())
            .asin(),
        );
        Self {
            altitude: Altitude(azimuth_rad),
            azimuth: Azimuth(altitude_rad),
        }
    }

    pub fn stereo_project(&self) -> Polar {
        Polar {
            radius: 2.0 * (PI_FOURTH - self.altitude.0.to_rad() / 2.0).tan(),
            angle: self.azimuth.0,
        }
    }
}
