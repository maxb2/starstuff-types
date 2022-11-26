// Hipparcos Catalog parser
// https://heasarc.gsfc.nasa.gov/W3Browse/all/hipparcos.html

use crate::angle::{Angle, ArcMinuteSecond, Degree, Dms, Hms, Hour, Sign};
use crate::coord::{Declination, RightAscension};

use std::convert::TryFrom;

macro_rules! parse_hip_field {
    (u32 $s:expr) => {
        // Need to trim because numbers are space padded in YBSC
        match $s.trim().parse::<u32>() {
            Ok(inner) => Some(inner),
            Err(_) => None,
        }
    };
    (usize $s:expr) => {
        // Need to trim because numbers are space padded in YBSC
        match $s.trim().parse::<usize>() {
            Ok(inner) => Some(inner),
            Err(_) => None,
        }
    };
    (f64 $s:expr) => {
        match $s.trim().parse::<f64>() {
            Ok(inner) => Some(inner),
            Err(_) => None,
        }
    };
    (String $s:expr) => {
        $s.to_string()
    };
    (ra $s:expr) => {
        {
        let fields: Vec<&str> = $s.split(" ").collect();
        RightAscension(Angle::from(Hour::from(Hms::new(Sign::Plus, parse_hip_field!(u32 fields[0]).unwrap(), parse_hip_field!(u32 fields[1]).unwrap(), parse_hip_field!(f64 fields[2]).unwrap()))))
        }
    };
    (dec $s:expr) => {
        {
        let fields: Vec<&str> = $s.split(" ").collect();
        let sign: Sign = match &fields[0][0..1] {
            "+" => Sign::Plus,
            "-" => Sign::Minus,
            _ => panic!()
        };
        Declination(Angle::from(Degree::from(Dms::new(sign, parse_hip_field!(u32 fields[0][1..]).unwrap(), parse_hip_field!(u32 fields[1]).unwrap(), parse_hip_field!(f64 fields[2]).unwrap()))))
    }
    };
}

#[derive(Debug)]
struct HipStar {
    catalog: String,
    HIP: usize,
    Proxy: String,
    right_ascension: RightAscension,
    declination: Declination,
}

impl TryFrom<String> for HipStar {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let fields: Vec<&str> = s.split('|').collect();
        Ok(Self {
            catalog: parse_hip_field!(String fields[0]),
            HIP: parse_hip_field!(usize fields[1]).unwrap(),
            Proxy: parse_hip_field!(String fields[2]),
            right_ascension: parse_hip_field!(ra fields[3]),
            declination: parse_hip_field!(dec fields[4]),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_hip::*;
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;

    #[test]
    fn test_yaleparse() {
        let s = String::from("H|           1| |00 00 00.22|+01 05 20.4");
        println!("{}", s);
        let star = HipStar::try_from(s);
        println!("{:?}", star);
        panic!()
    }
}
