/*!
[Open Source Bright Star Catalog](https://github.com/johanley/star-catalog) parser

> NOTE: run the `get_data.sh` script to get the tests to pass.
*/
use super::parse::sexagesimal_underscore;
use crate::angle::{Dms, Hms, Sign};
use crate::catalog::parse::ws;
use nom::bytes::complete::take;
use nom::character::complete::{multispace0, one_of, u32};
use nom::combinator::{all_consuming, map_parser};
use nom::multi::many1;
use nom::number::complete::double;
use nom::sequence::terminated;
use nom::IResult;

impl From<i32> for Sign {
    fn from(x: i32) -> Self {
        if x >= 0 {
            Self::Positive
        } else {
            Self::Negative
        }
    }
}

/**
[Open Source Bright Star Catalog](https://github.com/johanley/star-catalog) record

From the [original source](https://github.com/johanley/star-catalog/tree/master/catalogs/output/open-source-bsc):

> - All coordinates are in the ICRS.
> - The core astrometry fields are always present.
> - The core data is taken from Hipparcos-2, using Hipparcos-1 as occasional backup.
> - For proper motions, the epoch is J1991.25 = JD2448349.0625 (TT), not J2000.
> - The position and width of each field appears in square brackets.
> - For example, the HIP identifier \[1,6\] starts at the first character, and is 6 characters wide.
> - The unit 'mas' stands for milliarcsecond.
> - (When parsing, you will need to trim leading spaces for some fields.)

*/
#[allow(non_snake_case)] // Copying field names from original data source
#[derive(Debug, Clone)]
pub struct OSBSCStar {
    /// 01: Hipparcos identifier HIP \[1,6\]
    pub Hipparcos_id: Field<usize>,

    /** 02: right Ascension in hours minutes seconds, ICRS \[9,16\]

    - Proper motion epoch is J1991.25
    - An underscore is used to separate the parts.
    - Calculated from the radians in field 04. Included for convenience.
    */
    pub right_ascension_hms: Field<crate::angle::Hms>,

    /** 03: declination degrees minutes seconds, ICRS \[27,16\]

    - Proper motion epoch is J1991.25
    - An underscore is used to separate the parts.
    - Calculated from the radians in field 05. Included for convenience.
    */
    pub declination_dms: Field<crate::angle::Dms>,

    /** 04: right ascension in radians, ICRS. \[45,12\]

    - Proper motion epoch is J1991.25
    */
    pub right_ascension_rad: Field<f64>,

    /** 05: declination in radians, ICRS.  \[59,13\]

    - Proper motion epoch is J1991.25
    */
    pub declination_rad: Field<f64>,

    /// 06: parallax in mas \[73,7\]
    pub parallax: Field<f64>,

    /// 07: proper motion in right ascension in mas/year, * cosine(declination), ICRS \[81,8\]
    pub proper_motion_ra: Field<f64>,

    /// 08: proper motion in declination in mas/year, ICRS \[90,8\]
    pub proper_motion_dec: Field<f64>,

    /// 09: radial velocity in kilometers per second \[99,7\]
    pub radial_velocity: Field<Option<f64>>,

    /// 10: formal error in right ascension in mas \[107,6\]
    pub right_ascension_rad_err: Field<Option<f64>>,

    /// 11: formal error in declination in mas \[114,6\]
    pub declination_rad_err: Field<Option<f64>>,

    /// 12: formal error in parallax in mas \[121,6\]
    pub parallax_err: Field<f64>,

    /// 13: formal error in proper motion in right ascension in mas/year \[128,6\]
    pub proper_motion_ra_err: Field<f64>,

    /// 14: formal error in proper motion in declination in mas/year \[135,6\]
    pub proper_motion_dec_err: Field<f64>,

    /// 15: formal error in radial velocity in kilometers per second \[142,5\]
    pub radial_velocity_err: Field<Option<f64>>,

    /// 16: magnitude in the Johnson V band \[148,5\]
    pub V_magnitude: Field<f64>,

    /** 17: coarse variability flag \[154,1\]

    - Hipparcos-1 field H6.
    - 1: < 0.06mag ; 2: 0.06-0.6mag ; 3: >0.6mag
    */
    pub variability_flag: Field<Option<usize>>,

    /// 18: spectral type \[156,12\]
    pub spectral_type: Field<String>,

    /// 29: color index Johnson B-V magnitude \[169,6\]
    pub BV_magnitude: Field<Option<f64>>,

    /** 20: multiplicity flag \[176,1\]

    - Hipparcos-1 H59, only for C values.
    */
    pub multiplicity_flag: Field<String>,

    /** 21: CCDM identifier \[178,10\]

    - A catalog of double/multiple stars.
    */
    pub CCDM_id: Field<String>,

    /** 22: HD identifier \[189,6\]

    - Henry Draper catalog.
    */
    pub HD_id: Field<Option<usize>>,

    /** 23: HR identifier \[196,4\]

    - Yale Bright Star Catalog, r5.
    */
    pub Yale_id: Field<Option<usize>>,

    /// 24: Bayer identifier \[201,7\]
    pub Bayer_id: Field<String>,

    /// 25: Flamsteed identifier \[209,7\]
    pub Flamsteed_id: Field<String>,

    /** 26: proper name \[217,14\]

    - From an internal list defined by OSBSC.
    */
    pub proper_name: Field<String>,

    /// 27: Constellation abbreviation \[232,3\]
    pub constellation: Field<String>,
}

/// Provenances of data.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Provenance {
    /// Primary source for astrometry -  Hipparcos2
    Hipparcos2,
    /// Secondary source for astrometry - Hipparcos1
    Hipparcos1,
    /// Primary source for radial velocities - Pulkovo
    Pulvoko,
    /// Secondary source for radial velocities - BF
    BF,
    /// Identifiers: Bayer, Flamsteed, and HR - Yale Bright Star Catalog
    Yale,
    /// Backfill for a small number of items - SIMBAD
    SIMBAD,
    /// Open Source Bright Star Catalog custom data for star names
    Custom,
    /// Sexagesimal versions of RA, DEC - calculated fields
    Calculated,
    /// Vmag is the maximum (brightest) magnitude in the Hipparcos Variability Annex
    HipparcosVariabilityAnnex,
    /// Blank fields have no provenance
    None,
}

/// Data field with provenance.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Field<T> {
    value: T,
    provenance: Provenance,
}

/// Parse a provenance character.
pub fn provenance(input: &str) -> IResult<&str, Provenance> {
    let (input, prov) = one_of("ABCDEFGHI-")(input)?;

    Ok((
        input,
        match prov {
            'A' => Provenance::Hipparcos1,
            'B' => Provenance::Hipparcos2,
            'C' => Provenance::Pulvoko,
            'D' => Provenance::BF,
            'E' => Provenance::Yale,
            'F' => Provenance::SIMBAD,
            'G' => Provenance::Custom,
            'H' => Provenance::Calculated,
            'I' => Provenance::HipparcosVariabilityAnnex,
            '-' => Provenance::None,
            _ => unreachable!(),
        },
    ))
}

/// Parse a record from OS Bright Star Catalog data file.
pub fn parse_record(input: &str) -> IResult<&str, OSBSCStar> {
    let (input, hipparcos_id) = map_parser(take(8usize), ws(u32))(input)?;

    let (input, right_ascension_hms) =
        map_parser(take(18usize), ws(sexagesimal_underscore))(input)?;

    let (input, declination_dms) = map_parser(take(18usize), ws(sexagesimal_underscore))(input)?;

    let (input, right_ascension_rad) = map_parser(take(14usize), ws(double))(input)?;

    let (input, declination_rad) = map_parser(take(14usize), ws(double))(input)?;

    let (input, parallax) = map_parser(take(8usize), ws(double))(input)?;

    let (input, proper_motion_ra) = map_parser(take(9usize), ws(double))(input)?;

    let (input, proper_motion_dec) = map_parser(take(9usize), ws(double))(input)?;

    let (input, radial_velocity) = take(8usize)(input)?;

    let (input, right_ascension_rad_err) = take(7usize)(input)?;

    let (input, declination_rad_err) = take(7usize)(input)?;

    let (input, parallax_err) = map_parser(take(7usize), ws(double))(input)?;

    let (input, proper_motion_ra_err) = map_parser(take(7usize), ws(double))(input)?;

    let (input, proper_motion_dec_err) = map_parser(take(7usize), ws(double))(input)?;

    let (input, radial_velocity_err) = take(6usize)(input)?;

    let (input, v_magnitude) = map_parser(take(6usize), ws(double))(input)?;

    let (input, variability_flag) = map_parser(take(2usize), one_of(" 123"))(input)?;

    let (input, spectral_type) = take(13usize)(input)?;

    let (input, bv_magnitude) = take(7usize)(input)?;

    let (input, multiplicity_flag) = take(2usize)(input)?;

    let (input, ccdm_id) = take(11usize)(input)?;

    let (input, hd_id) = take(7usize)(input)?;

    let (input, yale_id) = take(5usize)(input)?;

    let (input, bayer_id) = take(8usize)(input)?;

    let (input, flamsteed_id) = take(8usize)(input)?;

    let (input, proper_name) = take(15usize)(input)?;

    let (input, constellation) = take(4usize)(input)?;

    let (input, provs) = all_consuming(terminated(
        map_parser(take(27usize), many1(provenance)),
        multispace0,
    ))(input)?;

    Ok((
        input,
        OSBSCStar {
            Hipparcos_id: Field {
                value: hipparcos_id.try_into().unwrap(),
                provenance: provs[0],
            },

            right_ascension_hms: Field {
                value: Hms::new(
                    Sign::from(right_ascension_hms.major),
                    right_ascension_hms.major.unsigned_abs(),
                    right_ascension_hms.minor.unsigned_abs(),
                    right_ascension_hms.second,
                ),
                provenance: provs[1],
            },
            declination_dms: Field {
                value: Dms(
                    Sign::from(declination_dms.major),
                    declination_dms.major.unsigned_abs(),
                    declination_dms.minor.unsigned_abs(),
                    declination_dms.second,
                ),
                provenance: provs[2],
            },
            right_ascension_rad: Field {
                value: right_ascension_rad,
                provenance: provs[3],
            },
            declination_rad: Field {
                value: declination_rad,
                provenance: provs[4],
            },
            parallax: Field {
                value: parallax,
                provenance: provs[5],
            },
            proper_motion_ra: Field {
                value: proper_motion_ra,
                provenance: provs[6],
            },
            proper_motion_dec: Field {
                value: proper_motion_dec,
                provenance: provs[7],
            },
            radial_velocity: Field {
                value: match radial_velocity.trim().parse::<f64>() {
                    Ok(x) => Some(x),
                    Err(_) => None,
                },
                provenance: provs[8],
            },
            right_ascension_rad_err: Field {
                value: match right_ascension_rad_err.trim().parse::<f64>() {
                    Ok(x) => Some(x),
                    Err(_) => None,
                },
                provenance: provs[9],
            },
            declination_rad_err: Field {
                value: match declination_rad_err.trim().parse::<f64>() {
                    Ok(x) => Some(x),
                    Err(_) => None,
                },
                provenance: provs[10],
            },
            parallax_err: Field {
                value: parallax_err,
                provenance: provs[11],
            },
            proper_motion_ra_err: Field {
                value: proper_motion_ra_err,
                provenance: provs[12],
            },
            proper_motion_dec_err: Field {
                value: proper_motion_dec_err,
                provenance: provs[13],
            },
            radial_velocity_err: Field {
                value: match radial_velocity_err.trim().parse::<f64>() {
                    Ok(x) => Some(x),
                    Err(_) => None,
                },
                provenance: provs[14],
            },
            V_magnitude: Field {
                value: v_magnitude,
                provenance: provs[15],
            },
            variability_flag: Field {
                value: match variability_flag {
                    '1' => Some(1usize),
                    '2' => Some(2usize),
                    '3' => Some(3usize),
                    _ => None,
                },
                provenance: provs[16],
            },
            spectral_type: Field {
                value: spectral_type.trim().to_string(),
                provenance: provs[17],
            },
            BV_magnitude: Field {
                value: match bv_magnitude.trim().parse::<f64>() {
                    Ok(x) => Some(x),
                    Err(_) => None,
                },
                provenance: provs[18],
            },
            multiplicity_flag: Field {
                value: multiplicity_flag.trim().to_string(),
                provenance: provs[19],
            },
            CCDM_id: Field {
                value: ccdm_id.trim().to_string(),
                provenance: provs[20],
            },
            HD_id: Field {
                value: match hd_id.trim().parse::<usize>() {
                    Ok(x) => Some(x),
                    Err(_) => None,
                },
                provenance: provs[21],
            },
            Yale_id: Field {
                value: match yale_id.trim().parse::<usize>() {
                    Ok(x) => Some(x),
                    Err(_) => None,
                },
                provenance: provs[22],
            },
            Bayer_id: Field {
                value: bayer_id.trim().to_string(),
                provenance: provs[23],
            },
            Flamsteed_id: Field {
                value: flamsteed_id.trim().to_string(),
                provenance: provs[24],
            },
            proper_name: Field {
                value: proper_name.trim().to_string(),
                provenance: provs[25],
            },
            constellation: Field {
                value: constellation.trim().to_string(),
                provenance: provs[26],
            },
        },
    ))
}

impl TryFrom<&String> for OSBSCStar {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let result = parse_record(s);
        match result {
            Ok((_, star)) => Ok(star),
            Err(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::catalog::osbsc::*;

    #[test]
    fn test_osbscstar_parse_record() {
        // let s = String::from("    88  00_01_04.5982692  -48_48_35.492919  0.0046977187  -0.8518927495    5.50   -18.36    -5.82     8.0   0.26   0.29   0.48   0.46   0.38   0.7  5.71          G8III  0.911              224834 9081   τ Phe                        Phe BHHAAAAACAAAAACB-BB--BEE--H ");
        let s = String::from("   107  00_01_20.1124271  -50_20_14.636969  0.0058259401  -0.8785533522    6.01     7.88    11.40     2.3   0.21   0.20   0.32   0.25   0.24   0.9  5.53 1        M2III  1.615              224865 9082                                Phe BHHAAAAACAAAAACBBBB--BE---H ");
        let rec = parse_record(&s);
        println!("{:#?}", rec);
        rec.unwrap();
        // panic!()
    }

    #[test]
    // #[ignore]
    fn test_catalog() {
        use std::error::Error;
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::BufReader;
        use std::path::Path;

        let path = Path::new("data/OSBSC/os-bright-star-catalog-hip.utf8");
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that describes the error
            Err(why) => panic!(
                "couldn't open {}: {}",
                display,
                <dyn Error>::to_string(&why)
            ),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        let lines = reader.lines();
        // lines is a instance of some type which implements Iterator<Item=&str>

        let mut stars: Vec<OSBSCStar> = vec![];

        for l in lines {
            let s = l.unwrap();
            let result = OSBSCStar::try_from(&s);
            match result {
                Ok(star) => stars.push(star),
                Err(_) => {
                    println!("{:#?}", &s);
                    println!("{:#?}", result);
                    panic!()
                }
            };
        }
        println!("Number of stars: {}", stars.len());
        println!("Last Star: {:?}", stars.last().unwrap());
        // panic!()
    }
}
