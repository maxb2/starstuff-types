/*!
Coordinate Systems

> See [converting astronomical coordinates](https://en.wikipedia.org/wiki/Astronomical_coordinate_systems#Converting_coordinates) for more details.
 */

use crate::angle::{Angle, PI, PI_FOURTH, PI_HALF};
use crate::time::GMST;

/// Trait for constrained angles
pub trait ConstrainedAngle {
    /// Angle constructor that panics when the underlying angle does not satisfy constraints.
    fn new(angle: &Angle) -> Self;
    /// Get underlying angle
    fn value(&self) -> Angle;
}

/**
Zenith Angle

<https://en.wikipedia.org/wiki/Zenith>
 */
#[derive(Debug, Copy, Clone)]
pub struct ZenithAngle(Angle);

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

/**
Declination

<https://en.wikipedia.org/wiki/Declination>
 */
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

/**
Altitude

<https://en.wikipedia.org/wiki/Horizontal_coordinate_system>
 */
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

/**
Latitude

<https://en.wikipedia.org/wiki/Latitude>
 */
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

/**
Right Ascension

<https://en.wikipedia.org/wiki/Right_ascension>
 */
#[derive(Debug, Copy, Clone)]
pub struct RightAscension(pub Angle);

/**
Azimuth

<https://en.wikipedia.org/wiki/Azimuth>
 */
#[derive(Debug, Copy, Clone)]
pub struct Azimuth(pub Angle);

/**
Longitude

<https://en.wikipedia.org/wiki/Longitude>
 */
#[derive(Debug, Copy, Clone)]
pub struct Longitude(pub Angle);

/**
Cartesian Coordinates

<https://en.wikipedia.org/wiki/Cartesian_coordinate_system>
 */
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct Cartesian {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/**
Geographic Coordinates (Latitude/Longitude)

<https://en.wikipedia.org/wiki/Geographic_coordinate_system>
 */
#[derive(Debug, Copy, Clone)]
pub struct Geographic {
    pub latitude: Latitude,
    pub longitude: Longitude,
}

/**
Two-dimensional Polar Coordinates

<https://en.wikipedia.org/wiki/Polar_coordinate_system>
 */
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct Polar {
    pub radius: f64,
    pub angle: Angle,
}

/**
Equitorial Astronomical Coordinates

<https://en.wikipedia.org/wiki/Astronomical_coordinate_systems#Equatorial_system>
 */
#[derive(Debug, Copy, Clone)]
pub struct Equitorial {
    pub right_ascension: RightAscension,
    pub declination: Declination,
}

/**
Horizontal Astronomical Coordinates

<https://en.wikipedia.org/wiki/Astronomical_coordinate_systems#Horizontal_system>
 */
#[derive(Debug, Copy, Clone)]
pub struct Horizontal {
    pub altitude: Altitude,
    pub azimuth: Azimuth,
}

impl Horizontal {
    /**
    Convert equitorial coordinates to horizontal given a place and time.

    <https://en.wikipedia.org/wiki/Astronomical_coordinate_systems#Equatorial_%E2%86%94_horizontal>
     */
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

    /**
    Stereographic map projection of coordinates.

    <https://en.wikipedia.org/wiki/Stereographic_map_projection>
     */
    pub fn stereo_project(&self) -> Polar {
        Polar {
            radius: 2.0 * (PI_FOURTH - self.altitude.0.to_rad() / 2.0).tan(),
            angle: self.azimuth.0,
        }
    }
}
