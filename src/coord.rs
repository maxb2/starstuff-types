use crate::angle::{Angle, AngleTrig, Hour, Radian, PI, PI_FOURTH, PI_HALF};
use crate::util::GMST;

pub trait ConstrainedAngle {
    fn new(angle: &Angle) -> Self;
    fn value(&self) -> Angle;
}

#[derive(Debug)]
pub struct ZenithAngle(Angle);

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

#[derive(Debug)]
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

#[derive(Debug)]
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
#[derive(Debug)]
pub struct Latitude(Angle);

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

#[derive(Debug)]
pub struct RightAscension(pub Angle);

#[derive(Debug)]
pub struct Azimuth(pub Angle);

#[derive(Debug)]
pub struct Longitude(pub Angle);

#[derive(Debug)]
#[allow(dead_code)]
pub struct Cartesian {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
pub struct Geographic {
    latitude: Latitude,
    longitude: Longitude,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Polar {
    radius: f64,
    angle: Angle,
}

#[derive(Debug)]
pub struct Equitorial {
    pub right_ascension: RightAscension,
    pub declination: Declination,
}

#[derive(Debug)]
pub struct Horizontal {
    altitude: Altitude,
    azimuth: Azimuth,
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
