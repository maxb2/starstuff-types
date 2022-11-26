// Hipparcos Catalog parser
// https://heasarc.gsfc.nasa.gov/W3Browse/all/hipparcos.html

use crate::angle::{Angle, ArcMinuteSecond, DegData, Dms, Hms, Sign};
use crate::coord::{Declination, RightAscension};

use std::convert::TryFrom;

macro_rules! parse_trim {
    ($T:ty, $s:expr) => {
        // Need to trim because numbers are space padded in YBSC
        match $s.trim().parse::<$T>() {
            Ok(inner) => Some(inner),
            Err(_) => None,
        }
    };
}

macro_rules! parse_hip_field {
    (u32 $s:expr) => {parse_trim!(u32, $s)};
    (usize $s:expr) => {parse_trim!(usize, $s)};
    (f64 $s:expr) => {parse_trim!(f64, $s)};
    (String $s:expr) => {
        match $s.trim() {
            "" => None,
            _ => Some($s.to_string())
        }
        // $s.to_string()
    };

    (ra_deg $s:expr) => {
        match parse_hip_field!(f64 $s) {
            Some(deg) => Some(RightAscension(Angle::from(DegData(deg)))),
            None => None
        }
    };
    (dec_deg $s:expr) => {
        match parse_hip_field!(f64 $s) {
            Some(deg) => Some(Declination(Angle::from(DegData(deg)))),
            None => None
        }
    };
    (ra_hms $s:expr) => {
        {
        let fields: Vec<&str> = $s.split(" ").collect();
        RightAscension(Angle::from(Hms::new(Sign::Plus, parse_hip_field!(u32 fields[0]).unwrap(), parse_hip_field!(u32 fields[1]).unwrap(), parse_hip_field!(f64 fields[2]).unwrap())))
        }
    };
    (dec_dms $s:expr) => {
        {
        let fields: Vec<&str> = $s.split(" ").collect();
        let sign: Sign = match &fields[0][0..1] {
            "+" => Sign::Plus,
            "-" => Sign::Minus,
            _ => panic!()
        };
        Declination(Angle::from(Dms::new(sign, parse_hip_field!(u32 fields[0][1..]).unwrap(), parse_hip_field!(u32 fields[1]).unwrap(), parse_hip_field!(f64 fields[2]).unwrap())))
    }
    };
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug)]
struct HipStar {
    catalog: Option<String>, //  [H] Catalogue (H=Hipparcos)               (H0)
    HIP: Option<usize>,      //  Identifier (HIP number)                   (H1)
    Proxy: Option<String>,   // *[HT] Proximity flag                       (H2)
    right_ascension: RightAscension, //  Right ascension in h m s, ICRS (J1991.25) (H3)
    declination: Declination, //  Declination in deg ' ", ICRS (J1991.25)   (H4)
    Vmag: Option<f64>,       //  ? Magnitude in Johnson V                  (H5)
    VarFlag: Option<String>, // *[1,3]? Coarse variability flag            (H6)
    r_Vmag: Option<String>,  // *[GHT] Source of magnitude                 (H7)
    RAdeg: Option<RightAscension>, // *? alpha, degrees (ICRS, Epoch=J1991.25)   (H8)
    DEdeg: Option<Declination>, // *? delta, degrees (ICRS, Epoch=J1991.25)   (H9)
    AstroRef: Option<String>, // *[*+A-Z] Reference flag for astrometry    (H10)
    Plx: Option<f64>,        //  ? Trigonometric parallax                 (H11)
    pmRA: Option<f64>,       // *? Proper motion mu_alpha.cos(delta), ICRS(H12)
    pmDE: Option<f64>,       // *? Proper motion mu_delta, ICRS           (H13)
    e_RAdeg: Option<f64>,    // *? Standard error in RA*cos(DEdeg)        (H14)
    e_DEdeg: Option<f64>,    // *? Standard error in DE                   (H15)
    e_Plx: Option<f64>,      //  ? Standard error in Plx                  (H16)
    e_pmRA: Option<f64>,     //  ? Standard error in pmRA                 (H17)
    e_pmDE: Option<f64>,     //  ? Standard error in pmDE                 (H18)
    DE_RA: Option<f64>,      //  [-1/1]? Correlation, DE/RA*cos(delta)    (H19)
    Plx_RA: Option<f64>,     //  [-1/1]? Correlation, Plx/RA*cos(delta)   (H20)
    Plx_DE: Option<f64>,     //  [-1/1]? Correlation, Plx/DE              (H21)
    pmRA_RA: Option<f64>,    //  [-1/1]? Correlation, pmRA/RA*cos(delta)  (H22)
    pmRA_DE: Option<f64>,    //  [-1/1]? Correlation, pmRA/DE             (H23)
    pmRA_Plx: Option<f64>,   //  [-1/1]? Correlation, pmRA/Plx            (H24)
    pmDE_RA: Option<f64>,    //  [-1/1]? Correlation, pmDE/RA*cos(delta)  (H25)
    pmDE_DE: Option<f64>,    //  [-1/1]? Correlation, pmDE/DE             (H26)
    pmDE_Plx: Option<f64>,   //  [-1/1]? Correlation, pmDE/Plx            (H27)
    pmDE_pmRA: Option<f64>,  //  [-1/1]? Correlation, pmDE/pmRA           (H28)
    F1: Option<usize>,       //  ? Percentage of rejected data            (H29)
    F2: Option<f64>,         // *? Goodness-of-fit parameter              (H30)
    BTmag: Option<f64>,      //  ? Mean BT magnitude                      (H32)
    e_BTmag: Option<f64>,    //  ? Standard error on BTmag                (H33)
    VTmag: Option<f64>,      //  ? Mean VT magnitude                      (H34)
    e_VTmag: Option<f64>,    //  ? Standard error on VTmag                (H35)
    m_BTmag: Option<String>, // *[A-Z*-] Reference flag for BT and VTmag  (H36)
    B_V: Option<f64>,        //  ? Johnson B-V colour                     (H37)
    e_B_V: Option<f64>,      //  ? Standard error on B-V                  (H38)
    r_B_V: Option<String>,   //  [GT] Source of B-V from Ground or Tycho  (H39)
    V_I: Option<f64>,        //  ? Colour index in Cousins' system        (H40)
    e_V_I: Option<f64>,      //  ? Standard error on V-I                  (H41)
    r_V_I: Option<String>,   // *[A-T] Source of V-I                      (H42)
    CombMag: Option<String>, //  [*] Flag for combined Vmag, B-V, V-I     (H43)
    Hpmag: Option<f64>,      // *? Median magnitude in Hipparcos system   (H44)
    e_Hpmag: Option<f64>,    // *? Standard error on Hpmag                (H45)
    Hpscat: Option<f64>,     //  ? Scatter on Hpmag                       (H46)
    o_Hpmag: Option<usize>,  //  ? Number of observations for Hpmag       (H47)
    m_Hpmag: Option<String>, // *[A-Z*-] Reference flag for Hpmag         (H48)
    Hpmax: Option<f64>,      //  ? Hpmag at maximum (5th percentile)      (H49)
    HPmin: Option<f64>,      //  ? Hpmag at minimum (95th percentile)     (H50)
    Period: Option<f64>,     //  ? Variability period (days)              (H51)
    HvarType: Option<String>, // *[CDMPRU]? variability type               (H52)
    moreVar: Option<String>, // *[12] Additional data about variability   (H53)
    morePhoto: Option<String>, //  [ABC] Light curve Annex                  (H54)
    CCDM: Option<String>,    //  CCDM identifier                          (H55)
    n_CCDM: Option<String>,  // *[HIM] Historical status flag             (H56)
    Nsys: Option<usize>,     //  ? Number of entries with same CCDM       (H57)
    Ncomp: Option<usize>,    //  ? Number of components in this entry     (H58)
    MultFlag: Option<String>, // *[CGOVX] Double/Multiple Systems flag     (H59)
    Source: Option<String>,  // *[PFILS] Astrometric source flag          (H60)
    Qual: Option<String>,    // *[ABCDS] Solution quality                 (H61)
    m_HIP: Option<String>,   //  Component identifiers                    (H62)
    theta: Option<usize>,    //  ? Position angle between components      (H63)
    rho: Option<f64>,        //  ? Angular separation between components  (H64)
    e_rho: Option<f64>,      //  ? Standard error on rho                  (H65)
    dHp: Option<f64>,        //  ? Magnitude difference of components     (H66)
    e_dHp: Option<f64>,      //  ? Standard error on dHp                  (H67)
    Survey: Option<String>,  //  [S] Flag indicating a Survey Star        (H68)
    Chart: Option<String>,   // *[DG] Identification Chart                (H69)
    Notes: Option<String>,   // *[DGPWXYZ] Existence of notes             (H70)
    HD: Option<usize>,       //  [1/359083]? HD number <III/135>          (H71)
    BD: Option<String>,      //  Bonner DM <I/119>, <I/122>               (H72)
    CoD: Option<String>,     //  Cordoba Durchmusterung (DM) <I/114>      (H73)
    CPD: Option<String>,     //  Cape Photographic DM <I/108>             (H74)
    V_I_red: Option<f64>,    //  V-I used for reductions                  (H75)
    SpType: Option<String>,  //  Spectral type                            (H76)
    r_SpType: Option<String>, // *[1234GKSX]? Source of spectral type      (H77)
}

impl TryFrom<String> for HipStar {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let fields: Vec<&str> = s.split('|').collect();
        Ok(Self {
            catalog: parse_hip_field!(String fields[0]), //  [H] Catalogue (H=Hipparcos)               (H0)
            HIP: parse_hip_field!(usize fields[1]), //  Identifier (HIP number)                   (H1)
            Proxy: parse_hip_field!(String fields[2]), // *[HT] Proximity flag                       (H2)
            right_ascension: parse_hip_field!(ra_hms fields[3]), //  Right ascension in h m s, ICRS (J1991.25) (H3)
            declination: parse_hip_field!(dec_dms fields[4]), //  Declination in deg ' ", ICRS (J1991.25)   (H4)
            Vmag: parse_hip_field!(f64 fields[5]), //  ? Magnitude in Johnson V                  (H5)
            VarFlag: parse_hip_field!(String fields[6]), // *[1,3]? Coarse variability flag            (H6)
            r_Vmag: parse_hip_field!(String fields[7]), // *[GHT] Source of magnitude                 (H7)
            RAdeg: parse_hip_field!(ra_deg fields[8]), // *? alpha, degrees (ICRS, Epoch=J1991.25)   (H8)
            DEdeg: parse_hip_field!(dec_deg fields[9]), // *? delta, degrees (ICRS, Epoch=J1991.25)   (H9)
            AstroRef: parse_hip_field!(String fields[10]), // *[*+A-Z] Reference flag for astrometry    (H10)
            Plx: parse_hip_field!(f64 fields[11]), //  ? Trigonometric parallax                 (H11)
            pmRA: parse_hip_field!(f64 fields[12]), // *? Proper motion mu_alpha.cos(delta), ICRS(H12)
            pmDE: parse_hip_field!(f64 fields[13]), // *? Proper motion mu_delta, ICRS           (H13)
            e_RAdeg: parse_hip_field!(f64 fields[14]), // *? Standard error in RA*cos(DEdeg)        (H14)
            e_DEdeg: parse_hip_field!(f64 fields[15]), // *? Standard error in DE                   (H15)
            e_Plx: parse_hip_field!(f64 fields[16]), //  ? Standard error in Plx                  (H16)
            e_pmRA: parse_hip_field!(f64 fields[17]), //  ? Standard error in pmRA                 (H17)
            e_pmDE: parse_hip_field!(f64 fields[18]), //  ? Standard error in pmDE                 (H18)
            DE_RA: parse_hip_field!(f64 fields[19]), //  [-1/1]? Correlation, DE/RA*cos(delta)    (H19)
            Plx_RA: parse_hip_field!(f64 fields[20]), //  [-1/1]? Correlation, Plx/RA*cos(delta)   (H20)
            Plx_DE: parse_hip_field!(f64 fields[21]), //  [-1/1]? Correlation, Plx/DE              (H21)
            pmRA_RA: parse_hip_field!(f64 fields[22]), //  [-1/1]? Correlation, pmRA/RA*cos(delta)  (H22)
            pmRA_DE: parse_hip_field!(f64 fields[23]), //  [-1/1]? Correlation, pmRA/DE             (H23)
            pmRA_Plx: parse_hip_field!(f64 fields[24]), //  [-1/1]? Correlation, pmRA/Plx            (H24)
            pmDE_RA: parse_hip_field!(f64 fields[25]), //  [-1/1]? Correlation, pmDE/RA*cos(delta)  (H25)
            pmDE_DE: parse_hip_field!(f64 fields[26]), //  [-1/1]? Correlation, pmDE/DE             (H26)
            pmDE_Plx: parse_hip_field!(f64 fields[27]), //  [-1/1]? Correlation, pmDE/Plx            (H27)
            pmDE_pmRA: parse_hip_field!(f64 fields[28]), //  [-1/1]? Correlation, pmDE/pmRA           (H28)
            F1: parse_hip_field!(usize fields[29]), //  ? Percentage of rejected data            (H29)
            F2: parse_hip_field!(f64 fields[30]), // *? Goodness-of-fit parameter              (H30)
            BTmag: parse_hip_field!(f64 fields[32]), //  ? Mean BT magnitude                      (H32)
            e_BTmag: parse_hip_field!(f64 fields[33]), //  ? Standard error on BTmag                (H33)
            VTmag: parse_hip_field!(f64 fields[34]), //  ? Mean VT magnitude                      (H34)
            e_VTmag: parse_hip_field!(f64 fields[35]), //  ? Standard error on VTmag                (H35)
            m_BTmag: parse_hip_field!(String fields[36]), // *[A-Z*-] Reference flag for BT and VTmag  (H36)
            B_V: parse_hip_field!(f64 fields[37]), //  ? Johnson B-V colour                     (H37)
            e_B_V: parse_hip_field!(f64 fields[38]), //  ? Standard error on B-V                  (H38)
            r_B_V: parse_hip_field!(String fields[39]), //  [GT] Source of B-V from Ground or Tycho  (H39)
            V_I: parse_hip_field!(f64 fields[40]), //  ? Colour index in Cousins' system        (H40)
            e_V_I: parse_hip_field!(f64 fields[41]), //  ? Standard error on V-I                  (H41)
            r_V_I: parse_hip_field!(String fields[42]), // *[A-T] Source of V-I                      (H42)
            CombMag: parse_hip_field!(String fields[43]), //  [*] Flag for combined Vmag, B-V, V-I     (H43)
            Hpmag: parse_hip_field!(f64 fields[44]), // *? Median magnitude in Hipparcos system   (H44)
            e_Hpmag: parse_hip_field!(f64 fields[45]), // *? Standard error on Hpmag                (H45)
            Hpscat: parse_hip_field!(f64 fields[46]), //  ? Scatter on Hpmag                       (H46)
            o_Hpmag: parse_hip_field!(usize fields[47]), //  ? Number of observations for Hpmag       (H47)
            m_Hpmag: parse_hip_field!(String fields[48]), // *[A-Z*-] Reference flag for Hpmag         (H48)
            Hpmax: parse_hip_field!(f64 fields[49]), //  ? Hpmag at maximum (5th percentile)      (H49)
            HPmin: parse_hip_field!(f64 fields[50]), //  ? Hpmag at minimum (95th percentile)     (H50)
            Period: parse_hip_field!(f64 fields[51]), //  ? Variability period (days)              (H51)
            HvarType: parse_hip_field!(String fields[52]), // *[CDMPRU]? variability type               (H52)
            moreVar: parse_hip_field!(String fields[53]), // *[12] Additional data about variability   (H53)
            morePhoto: parse_hip_field!(String fields[54]), //  [ABC] Light curve Annex                  (H54)
            CCDM: parse_hip_field!(String fields[55]), //  CCDM identifier                          (H55)
            n_CCDM: parse_hip_field!(String fields[56]), // *[HIM] Historical status flag             (H56)
            Nsys: parse_hip_field!(usize fields[57]), //  ? Number of entries with same CCDM       (H57)
            Ncomp: parse_hip_field!(usize fields[58]), //  ? Number of components in this entry     (H58)
            MultFlag: parse_hip_field!(String fields[59]), // *[CGOVX] Double/Multiple Systems flag     (H59)
            Source: parse_hip_field!(String fields[60]), // *[PFILS] Astrometric source flag          (H60)
            Qual: parse_hip_field!(String fields[61]), // *[ABCDS] Solution quality                 (H61)
            m_HIP: parse_hip_field!(String fields[62]), //  Component identifiers                    (H62)
            theta: parse_hip_field!(usize fields[63]), //  ? Position angle between components      (H63)
            rho: parse_hip_field!(f64 fields[64]), //  ? Angular separation between components  (H64)
            e_rho: parse_hip_field!(f64 fields[65]), //  ? Standard error on rho                  (H65)
            dHp: parse_hip_field!(f64 fields[66]), //  ? Magnitude difference of components     (H66)
            e_dHp: parse_hip_field!(f64 fields[67]), //  ? Standard error on dHp                  (H67)
            Survey: parse_hip_field!(String fields[68]), //  [S] Flag indicating a Survey Star        (H68)
            Chart: parse_hip_field!(String fields[69]), // *[DG] Identification Chart                (H69)
            Notes: parse_hip_field!(String fields[70]), // *[DGPWXYZ] Existence of notes             (H70)
            HD: parse_hip_field!(usize fields[71]), //  [1/359083]? HD number <III/135>          (H71)
            BD: parse_hip_field!(String fields[72]), //  Bonner DM <I/119>, <I/122>               (H72)
            CoD: parse_hip_field!(String fields[73]), //  Cordoba Durchmusterung (DM) <I/114>      (H73)
            CPD: parse_hip_field!(String fields[74]), //  Cape Photographic DM <I/108>             (H74)
            V_I_red: parse_hip_field!(f64 fields[75]), //  V-I used for reductions                  (H75)
            SpType: parse_hip_field!(String fields[76]), //  Spectral type                            (H76)
            r_SpType: parse_hip_field!(String fields[77]), // *[1234GKSX]? Source of spectral type      (H77)
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
    fn test_hipparse() {
        let s = String::from("H|           1| |00 00 00.22|+01 05 20.4| 9.10| |H|000.00091185|+01.08901332| |   3.54|   -5.20|   -1.88|  1.32|  0.74|  1.39|  1.36|  0.81| 0.32|-0.07|-0.11|-0.24| 0.09|-0.01| 0.10|-0.01| 0.01| 0.34|  0| 0.74|     1| 9.643|0.020| 9.130|0.019| | 0.482|0.025|T|0.55|0.03|L| | 9.2043|0.0020|0.017| 87| | 9.17| 9.24|       | | | |          | |  | 1| | | |  |   |       |     |     |    |S| | |224700|B+00 5077 |          |          |0.66|F5          |S");
        println!("{}", s);
        let star = HipStar::try_from(s);
        println!("{:?}", star);
        // panic!()
    }

    #[test]
    fn test_catalog() {
        // Create a path to the desired file
        let path = Path::new("hip_main.dat");
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

        for l in lines {
            // NOTE: need to pad the line with empty space because it will terminate early with empty fields at the end.
            let s = l.unwrap();
            let _star = HipStar::try_from(s).expect("Couldn't parse");

            // println!("{:?}", star)
        }
        // panic!()
    }
}
