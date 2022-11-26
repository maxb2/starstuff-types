use serde::{Deserialize, Serialize};
use std::fs;

use crate::angle::{Angle, ArcMinuteSecond, DegData, Dms, Hms, HourData, Sign};
use crate::coord::{Declination, Equitorial, RightAscension};
use crate::star::{Star, StarCoordinates};

#[derive(Serialize, Deserialize, Debug)]
struct _DMS {
    sign: String,
    degree: u32,
    minute: u32,
    second: f64,
}

impl From<_DMS> for Dms {
    fn from(deg: _DMS) -> Dms {
        match deg.sign.as_str() {
            "+" => Dms::new(Sign::Plus, deg.degree, deg.minute, deg.second),
            "-" => Dms::new(Sign::Minus, deg.degree, deg.minute, deg.second),
            _ => panic!("Got an invalid value for sign!"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct _HMS {
    hour: u32,
    minute: u32,
    second: f64,
}

impl From<_HMS> for Hms {
    fn from(hr: _HMS) -> Hms {
        Hms::new(Sign::Plus, hr.hour, hr.minute, hr.second)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct _Star {
    right_ascension: _HMS,
    declination: _DMS,
    harvard: u32,
    name: String,
    v_mag: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct _Catalog {
    catalog: String,
    stars: Vec<_Star>,
}

pub fn parse_catalog(file_path: &String) -> Vec<Star> {
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let catalog: _Catalog = serde_json::from_str(&contents).unwrap();

    let mut stars: Vec<Star> = Vec::new();

    for _star in catalog.stars {
        stars.push(Star {
            harvard: _star.harvard,
            v_mag: _star.v_mag,
            name: _star.name,
            coordinates: StarCoordinates::Equitorial(Equitorial {
                right_ascension: RightAscension(Angle::from(HourData::from(Hms::from(
                    _star.right_ascension,
                )))),
                declination: Declination(Angle::from(DegData::from(Dms::from(_star.declination)))),
            }),
        })
    }
    stars
}
