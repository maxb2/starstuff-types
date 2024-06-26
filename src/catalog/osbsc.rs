/*!
[Open Source Bright Star Catalog](https://github.com/johanley/star-catalog) parser

> NOTE: run the `get_data.sh` script to get the tests to pass.
*/
use super::ValidParse;
use crate::angle::{DegMinSec, HourMinSec, Sign};
use crate::parse_trim;
use unicode_segmentation::UnicodeSegmentation;

/// Parse an arc/minute/second field.
macro_rules! parse_ams {
    // Hour/Minute/Second
    (hms, $s:expr) => {{
        let fields: Vec<&str> = $s.split("_").collect();
        Some(HourMinSec(
            Sign::Positive,
            parse_trim!(u32, fields[0]).unwrap(),
            parse_trim!(u32, fields[1]).unwrap(),
            parse_trim!(f64, fields[2]).unwrap(),
        ))
    }};
    // Degree/Minute/Second
    (dms, $s:expr) => {{
        let fields: Vec<&str> = $s.split("_").collect();
        let sign: Sign = match &fields[0][0..1] {
            " " => Sign::Positive,
            "-" => Sign::Negative,
            _ => panic!(),
        };
        Some(DegMinSec(
            sign,
            parse_trim!(u32, fields[0][1..]).unwrap(),
            parse_trim!(u32, fields[1]).unwrap(),
            parse_trim!(f64, fields[2]).unwrap(),
        ))
    }};
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
    pub Hipparcos_id: Option<usize>,

    /** 02: right Ascension in hours minutes seconds, ICRS \[9,16\]

    - Proper motion epoch is J1991.25
    - An underscore is used to separate the parts.
    - Calculated from the radians in field 04. Included for convenience.
    */
    pub right_ascension_hms: Option<crate::angle::HourMinSec>,

    /** 03: declination degrees minutes seconds, ICRS \[27,16\]

    - Proper motion epoch is J1991.25
    - An underscore is used to separate the parts.
    - Calculated from the radians in field 05. Included for convenience.
    */
    pub declination_dms: Option<crate::angle::DegMinSec>,

    /** 04: right ascension in radians, ICRS. \[45,12\]

    - Proper motion epoch is J1991.25
    */
    pub right_ascension_rad: Option<f64>,

    /** 05: declination in radians, ICRS.  \[59,13\]

    - Proper motion epoch is J1991.25
    */
    pub declination_rad: Option<f64>,

    /// 06: parallax in mas \[73,7\]
    pub parallax: Option<f64>,

    /// 07: proper motion in right ascension in mas/year, * cosine(declination), ICRS \[81,8\]
    pub proper_motion_ra: Option<f64>,

    /// 08: proper motion in declination in mas/year, ICRS \[90,8\]
    pub proper_motion_dec: Option<f64>,

    /// 09: radial velocity in kilometers per second \[99,7\]
    pub radial_velocity: Option<f64>,

    /// 10: formal error in right ascension in mas \[107,6\]
    pub right_ascension_rad_err: Option<f64>,

    /// 11: formal error in declination in mas \[114,6\]
    pub declination_rad_err: Option<f64>,

    /// 12: formal error in parallax in mas \[121,6\]
    pub parallax_err: Option<f64>,

    /// 13: formal error in proper motion in right ascension in mas/year \[128,6\]
    pub proper_motion_ra_err: Option<f64>,

    /// 14: formal error in proper motion in declination in mas/year \[135,6\]
    pub proper_motion_dec_err: Option<f64>,

    /// 15: formal error in radial velocity in kilometers per second \[142,5\]
    pub radial_velocity_err: Option<f64>,

    /// 16: magnitude in the Johnson V band \[148,5\]
    pub V_magnitude: Option<f64>,

    /** 17: coarse variability flag \[154,1\]

    - Hipparcos-1 field H6.
    - 1: < 0.06mag ; 2: 0.06-0.6mag ; 3: >0.6mag
    */
    pub variability_flag: Option<usize>,

    /// 18: spectral type \[156,12\]
    pub spectral_type: Option<String>,

    /// 29: color index Johnson B-V magnitude \[169,6\]
    pub BV_magnitude: Option<f64>,

    /** 20: multiplicity flag \[176,1\]

    - Hipparcos-1 H59, only for C values.
    */
    pub multiplicity_flag: Option<String>,

    /** 21: CCDM identifier \[178,10\]

    - A catalog of double/multiple stars.
    */
    pub CCDM_id: Option<String>,

    /** 22: HD identifier \[189,6\]

    - Henry Draper catalog.
    */
    pub HD_id: Option<usize>,

    /** 23: HR identifier \[196,4\]

    - Yale Bright Star Catalog, r5.
    */
    pub Yale_id: Option<usize>,

    /// 24: Bayer identifier \[201,7\]
    pub Bayer_id: Option<String>,

    /// 25: Flamsteed identifier \[209,7\]
    pub Flamsteed_id: Option<String>,

    /** 26: proper name \[217,14\]

    - From an internal list defined by OSBSC.
    */
    pub proper_name: Option<String>,

    /// 27: Constellation abbreviation \[232,3\]
    pub constellation: Option<String>,

    /** 28: provenance string for all fields (except for the provenance itself, of course) \[236,27\]
    Each field (other than this one) has a provenance.
    The provenance string is an ordered string of single letters, stating the provenance of each field in the given record, in order from left to right.

    - A: Primary source for astrometry -  Hipparcos2
    - B: Secondary source for astrometry - Hipparcos1
    - C: Primary source for radial velocities - Pulkovo
    - D: Secondary source for radial velocities - BF
    - E: Identifiers: Bayer, Flamsteed, and HR - Yale Bright Star Catalog
    - F: Backfill for a small number of items - SIMBAD
    - G: My own custom data for star names
    - H: Sexagesimal versions of RA, DEC - calculated fields
    - I: Vmag is the maximum (brightest) magnitude in the Hipparcos Variability Annex
    - \-: Blank fields have no provenance
    */
    pub provenence: Option<String>,
}

impl TryFrom<String> for OSBSCStar {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let g = UnicodeSegmentation::graphemes(s.as_str(), true).collect::<Vec<&str>>();
        let star = Self {
            Hipparcos_id: parse_trim!(usize, s[0..6]),

            right_ascension_hms: parse_ams!(hms, s[8..24]),
            declination_dms: parse_ams!(dms, s[26..42]),
            right_ascension_rad: parse_trim!(f64, s[44..56]),
            declination_rad: parse_trim!(f64, s[58..71]),
            parallax: parse_trim!(f64, s[72..79]),
            proper_motion_ra: parse_trim!(f64, s[80..88]),
            proper_motion_dec: parse_trim!(f64, s[89..97]),
            radial_velocity: parse_trim!(f64, s[98..105]),
            right_ascension_rad_err: parse_trim!(f64, s[106..112]),
            declination_rad_err: parse_trim!(f64, s[113..119]),
            parallax_err: parse_trim!(f64, s[120..126]),
            proper_motion_ra_err: parse_trim!(f64, s[127..133]),
            proper_motion_dec_err: parse_trim!(f64, s[134..140]),
            radial_velocity_err: parse_trim!(f64, s[141..146]),
            V_magnitude: parse_trim!(f64, s[147..152]),
            variability_flag: parse_trim!(usize, s[153..154]),
            spectral_type: parse_trim!(String, s[155..167]),
            BV_magnitude: parse_trim!(f64, s[168..174]),
            multiplicity_flag: parse_trim!(String, s[175..176]),
            CCDM_id: parse_trim!(String, s[177..187]),
            HD_id: parse_trim!(usize, s[188..194]),
            Yale_id: parse_trim!(usize, s[195..199]),
            Bayer_id: parse_trim!(String, &g[200..207].join("").to_string()),
            Flamsteed_id: parse_trim!(String, &g[208..215].join("").to_string()),
            proper_name: parse_trim!(String, &g[216..230].join("").to_string()),
            constellation: parse_trim!(String, &g[231..234].join("").to_string()),
            provenence: parse_trim!(String, &g[235..262].join("").to_string()),
        };
        if star.is_valid_parse() {
            Ok(star)
        } else {
            Err(())
        }
    }
}

impl ValidParse for OSBSCStar {
    fn is_valid_parse(&self) -> bool {
        self.Hipparcos_id.is_some()
            && self.right_ascension_rad.is_some()
            && self.declination_rad.is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::catalog::osbsc::*;
    use crate::parse_catalog;

    #[test]
    fn osbscstar_from() {
        let s = String::from("    88  00_01_04.5982692  -48_48_35.492919  0.0046977187  -0.8518927495    5.50   -18.36    -5.82     8.0   0.26   0.29   0.48   0.46   0.38   0.7  5.71          G8III  0.911              224834 9081   τ Phe                        Phe BHHAAAAACAAAAACB-BB--BEE--H ");
        let star = OSBSCStar::try_from(s).unwrap();
        assert_eq!(star.Hipparcos_id.unwrap(), 88_usize);
        assert_eq!(
            star.right_ascension_hms.unwrap(),
            HourMinSec(Sign::Positive, 00, 01, 04.5982692)
        );
        assert_eq!(
            star.declination_dms.unwrap(),
            DegMinSec(Sign::Negative, 48, 48, 35.492919)
        );
        assert_eq!(star.right_ascension_rad.unwrap(), 0.0046977187);
        assert_eq!(star.declination_rad.unwrap(), -0.8518927495);
        assert_eq!(star.parallax.unwrap(), 5.50);
        assert_eq!(star.proper_motion_ra.unwrap(), -18.36);
        assert_eq!(star.proper_motion_dec.unwrap(), -5.82);
        assert_eq!(star.radial_velocity.unwrap(), 8.0);
        assert_eq!(star.right_ascension_rad_err.unwrap(), 0.26);
        assert_eq!(star.declination_rad_err.unwrap(), 0.29);
        assert_eq!(star.parallax_err.unwrap(), 0.48);
        assert_eq!(star.proper_motion_ra_err.unwrap(), 0.46);
        assert_eq!(star.proper_motion_dec_err.unwrap(), 0.38);
        assert_eq!(star.radial_velocity_err.unwrap(), 0.7);
        assert_eq!(star.V_magnitude.unwrap(), 5.71);
        assert!(star.variability_flag.is_none());
        assert_eq!(star.spectral_type.unwrap(), String::from("G8III"));
        assert_eq!(star.BV_magnitude.unwrap(), 0.911);
        assert!(star.multiplicity_flag.is_none());
        assert!(star.CCDM_id.is_none());
        assert_eq!(star.HD_id.unwrap(), 224834);
        assert_eq!(star.Yale_id.unwrap(), 9081);
        assert_eq!(star.Bayer_id.unwrap(), String::from("τ Phe"));
        assert!(star.Flamsteed_id.is_none());
        assert!(star.proper_name.is_none());
        assert_eq!(star.constellation.unwrap(), String::from("Phe"));
        assert_eq!(
            star.provenence.unwrap(),
            String::from("BHHAAAAACAAAAACB-BB--BEE--H")
        );
    }

    #[test]
    #[ignore]
    fn catalog() {
        let data_file = "data/OSBSC/os-bright-star-catalog-hip.utf8";

        if !std::path::Path::new(&data_file).exists() {
            panic!("File \"{}\" doesn't exist. Please run \"get_data.sh\" to fetch the data required for this test.", &data_file)
        };

        let _stars = parse_catalog!(
            OSBSCStar,
            Path::new(&data_file),
            // NOTE: it seems like we don't need to pad this catalog even though it has no delimiters.
            // In case it breaks in the future: Some(262)
            None
        );
        println!("Number of stars: {}", _stars.len());
        println!("Last Star: {:?}", _stars.last().unwrap());
    }
}
