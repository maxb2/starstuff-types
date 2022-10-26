use crate::angle::{Angle, AngleTrig, Hour, Radian, PI, PI_FOURTH, PI_HALF};
use crate::util::GMST;

pub trait ConstrainedAngle {
    fn new(angle: &Angle) -> Self;
    fn value(&self) -> Angle;
}

#[derive(Debug, Copy, Clone)]
pub struct ZenithAngle(pub Angle);

impl ConstrainedAngle for ZenithAngle {
    fn new(angle: &Angle) -> Self {
        let rad = Radian::from(*angle);
        if rad.0 < 0.0 || rad.0 > PI {
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
        let rad = Radian::from(*angle);
        if rad.0 < -PI_HALF || rad.0 > PI_HALF {
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
        let rad: Radian = Radian::from(*angle);
        if rad.0 < -PI_HALF || rad.0 > PI_HALF {
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
        let rad = Radian::from(*angle);
        if rad.0 < -PI_HALF || rad.0 > PI_HALF {
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
        let hour_local: Hour =
            sidereal_time.0 + Hour::from(geo.longitude.0) - Hour::from(eq.right_ascension.0);
        let x_horiz: f64 = -(geo.latitude.0.sin()) * (eq.declination.0.cos()) * (hour_local.cos())
            + geo.latitude.0.cos() * (eq.declination.0.sin());
        let y_horiz: f64 = eq.declination.0.cos() * hour_local.sin();
        let azimuth_rad: Radian = Radian(-(y_horiz.atan2(x_horiz)));
        let altitude_rad: Radian = Radian(
            (geo.latitude.0.sin() * eq.declination.0.sin()
                + geo.latitude.0.cos() * eq.declination.0.cos() * hour_local.cos())
            .asin(),
        );
        Self {
            altitude: Altitude(Angle::from(azimuth_rad)),
            azimuth: Azimuth(Angle::from(altitude_rad)),
        }
    }

    pub fn stereo_project(&self) -> Polar {
        Polar {
            radius: 2.0 * (PI_FOURTH - Radian::from(self.altitude.0).0 / 2.0).tan(),
            angle: self.azimuth.0,
        }
    }
}
