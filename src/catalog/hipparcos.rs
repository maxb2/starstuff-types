// Hipparcos Catalog parser
// https://heasarc.gsfc.nasa.gov/W3Browse/all/hipparcos.html
// NOTE: run the `get_data.sh` script to get the tests to pass.

use super::ValidParse;
use crate::angle::{Angle, ArcMinuteSecond, Dms, Hms, Sign};
use crate::coord::{Declination, RightAscension};

use std::convert::TryFrom;

use crate::parse_trim;

macro_rules! parse_hipparcos_field {
    (ra_deg, $s:expr) => {
        match parse_hipparcos_field!(f64, $s) {
            Some(deg) => Some(RightAscension(Angle::Degree(deg))),
            None => None,
        }
    };
    (dec_deg, $s:expr) => {
        match parse_hipparcos_field!(f64, $s) {
            Some(deg) => Some(Declination(Angle::Degree(deg))),
            None => None,
        }
    };
    (ra_hms, $s:expr) => {{
        let fields: Vec<&str> = $s.split(" ").collect();
        RightAscension(Angle::from(Hms::new(
            Sign::Plus,
            parse_hipparcos_field!(u32, fields[0]).unwrap(),
            parse_hipparcos_field!(u32, fields[1]).unwrap(),
            parse_hipparcos_field!(f64, fields[2]).unwrap(),
        )))
    }};
    (dec_dms, $s:expr) => {{
        let fields: Vec<&str> = $s.split(" ").collect();
        let sign: Sign = match &fields[0][0..1] {
            "+" => Sign::Plus,
            "-" => Sign::Minus,
            _ => panic!(),
        };
        Declination(Angle::from(Dms::new(
            sign,
            parse_hipparcos_field!(u32, fields[0][1..]).unwrap(),
            parse_hipparcos_field!(u32, fields[1]).unwrap(),
            parse_hipparcos_field!(f64, fields[2]).unwrap(),
        )))
    }};
    ($T:ty, $s:expr) => {
        parse_trim!($T, $s)
    };
}

#[allow(non_snake_case)] // Copying field names from original data source
#[derive(Debug, Clone)]
pub struct HipparcosStar {
    pub catalog: Option<String>, //          [H] Catalogue (H=Hipparcos)               (H0)
    pub HIP: Option<usize>,      //          Identifier (HIP number)                   (H1)
    pub Proxy: Option<String>,   //         *[HT] Proximity flag                       (H2)
    pub right_ascension: RightAscension, //  Right ascension in h m s, ICRS (J1991.25) (H3)
    pub declination: Declination, //         Declination in deg ' ", ICRS (J1991.25)   (H4)
    pub Vmag: Option<f64>,       //          ? Magnitude in Johnson V                  (H5)
    pub VarFlag: Option<String>, //         *[1,3]? Coarse variability flag            (H6)
    pub r_Vmag: Option<String>,  //         *[GHT] Source of magnitude                 (H7)
    pub RAdeg: Option<RightAscension>, //   *? alpha, degrees (ICRS, Epoch=J1991.25)   (H8)
    pub DEdeg: Option<Declination>, //      *? delta, degrees (ICRS, Epoch=J1991.25)   (H9)
    pub AstroRef: Option<String>, //        *[*+A-Z] Reference flag for astrometry    (H10)
    pub Plx: Option<f64>,        //          ? Trigonometric parallax                 (H11)
    pub pmRA: Option<f64>,       //         *? Proper motion mu_alpha.cos(delta), ICRS(H12)
    pub pmDE: Option<f64>,       //         *? Proper motion mu_delta, ICRS           (H13)
    pub e_RAdeg: Option<f64>,    //         *? Standard error in RA*cos(DEdeg)        (H14)
    pub e_DEdeg: Option<f64>,    //         *? Standard error in DE                   (H15)
    pub e_Plx: Option<f64>,      //          ? Standard error in Plx                  (H16)
    pub e_pmRA: Option<f64>,     //          ? Standard error in pmRA                 (H17)
    pub e_pmDE: Option<f64>,     //          ? Standard error in pmDE                 (H18)
    pub DE_RA: Option<f64>,      //          [-1/1]? Correlation, DE/RA*cos(delta)    (H19)
    pub Plx_RA: Option<f64>,     //          [-1/1]? Correlation, Plx/RA*cos(delta)   (H20)
    pub Plx_DE: Option<f64>,     //          [-1/1]? Correlation, Plx/DE              (H21)
    pub pmRA_RA: Option<f64>,    //          [-1/1]? Correlation, pmRA/RA*cos(delta)  (H22)
    pub pmRA_DE: Option<f64>,    //          [-1/1]? Correlation, pmRA/DE             (H23)
    pub pmRA_Plx: Option<f64>,   //          [-1/1]? Correlation, pmRA/Plx            (H24)
    pub pmDE_RA: Option<f64>,    //          [-1/1]? Correlation, pmDE/RA*cos(delta)  (H25)
    pub pmDE_DE: Option<f64>,    //          [-1/1]? Correlation, pmDE/DE             (H26)
    pub pmDE_Plx: Option<f64>,   //          [-1/1]? Correlation, pmDE/Plx            (H27)
    pub pmDE_pmRA: Option<f64>,  //          [-1/1]? Correlation, pmDE/pmRA           (H28)
    pub F1: Option<usize>,       //          ? Percentage of rejected data            (H29)
    pub F2: Option<f64>,         //         *? Goodness-of-fit parameter              (H30)
    pub BTmag: Option<f64>,      //          ? Mean BT magnitude                      (H32)
    pub e_BTmag: Option<f64>,    //          ? Standard error on BTmag                (H33)
    pub VTmag: Option<f64>,      //          ? Mean VT magnitude                      (H34)
    pub e_VTmag: Option<f64>,    //          ? Standard error on VTmag                (H35)
    pub m_BTmag: Option<String>, //         *[A-Z*-] Reference flag for BT and VTmag  (H36)
    pub B_V: Option<f64>,        //          ? Johnson B-V colour                     (H37)
    pub e_B_V: Option<f64>,      //          ? Standard error on B-V                  (H38)
    pub r_B_V: Option<String>,   //          [GT] Source of B-V from Ground or Tycho  (H39)
    pub V_I: Option<f64>,        //          ? Colour index in Cousins' system        (H40)
    pub e_V_I: Option<f64>,      //          ? Standard error on V-I                  (H41)
    pub r_V_I: Option<String>,   //         *[A-T] Source of V-I                      (H42)
    pub CombMag: Option<String>, //          [*] Flag for combined Vmag, B-V, V-I     (H43)
    pub Hpmag: Option<f64>,      //         *? Median magnitude in Hipparcos system   (H44)
    pub e_Hpmag: Option<f64>,    //         *? Standard error on Hpmag                (H45)
    pub Hpscat: Option<f64>,     //          ? Scatter on Hpmag                       (H46)
    pub o_Hpmag: Option<usize>,  //          ? Number of observations for Hpmag       (H47)
    pub m_Hpmag: Option<String>, //         *[A-Z*-] Reference flag for Hpmag         (H48)
    pub Hpmax: Option<f64>,      //          ? Hpmag at maximum (5th percentile)      (H49)
    pub HPmin: Option<f64>,      //          ? Hpmag at minimum (95th percentile)     (H50)
    pub Period: Option<f64>,     //          ? Variability period (days)              (H51)
    pub HvarType: Option<String>, //        *[CDMPRU]? variability type               (H52)
    pub moreVar: Option<String>, //         *[12] Additional data about variability   (H53)
    pub morePhoto: Option<String>, //        [ABC] Light curve Annex                  (H54)
    pub CCDM: Option<String>,    //          CCDM identifier                          (H55)
    pub n_CCDM: Option<String>,  //         *[HIM] Historical status flag             (H56)
    pub Nsys: Option<usize>,     //          ? Number of entries with same CCDM       (H57)
    pub Ncomp: Option<usize>,    //          ? Number of components in this entry     (H58)
    pub MultFlag: Option<String>, //        *[CGOVX] Double/Multiple Systems flag     (H59)
    pub Source: Option<String>,  //         *[PFILS] Astrometric source flag          (H60)
    pub Qual: Option<String>,    //         *[ABCDS] Solution quality                 (H61)
    pub m_HIP: Option<String>,   //          Component identifiers                    (H62)
    pub theta: Option<usize>,    //          ? Position angle between components      (H63)
    pub rho: Option<f64>,        //          ? Angular separation between components  (H64)
    pub e_rho: Option<f64>,      //          ? Standard error on rho                  (H65)
    pub dHp: Option<f64>,        //          ? Magnitude difference of components     (H66)
    pub e_dHp: Option<f64>,      //          ? Standard error on dHp                  (H67)
    pub Survey: Option<String>,  //          [S] Flag indicating a Survey Star        (H68)
    pub Chart: Option<String>,   //         *[DG] Identification Chart                (H69)
    pub Notes: Option<String>,   //         *[DGPWXYZ] Existence of notes             (H70)
    pub HD: Option<usize>,       //          [1/359083]? HD number <III/135>          (H71)
    pub BD: Option<String>,      //          Bonner DM <I/119>, <I/122>               (H72)
    pub CoD: Option<String>,     //          Cordoba Durchmusterung (DM) <I/114>      (H73)
    pub CPD: Option<String>,     //          Cape Photographic DM <I/108>             (H74)
    pub V_I_red: Option<f64>,    //          V-I used for reductions                  (H75)
    pub SpType: Option<String>,  //          Spectral type                            (H76)
    pub r_SpType: Option<String>, //        *[1234GKSX]? Source of spectral type      (H77)
}

impl TryFrom<String> for HipparcosStar {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let fields: Vec<&str> = s.split('|').collect();
        let star = Self {
            catalog: parse_hipparcos_field!(String, fields[0]), //          [H] Catalogue (H=Hipparcos)               (H0)
            HIP: parse_hipparcos_field!(usize, fields[1]), //               Identifier (HIP number)                   (H1)
            Proxy: parse_hipparcos_field!(String, fields[2]), //           *[HT] Proximity flag                       (H2)
            right_ascension: parse_hipparcos_field!(ra_hms, fields[3]), //  Right ascension in h m s, ICRS (J1991.25) (H3)
            declination: parse_hipparcos_field!(dec_dms, fields[4]), //     Declination in deg ' ", ICRS (J1991.25)   (H4)
            Vmag: parse_hipparcos_field!(f64, fields[5]), //                ? Magnitude in Johnson V                  (H5)
            VarFlag: parse_hipparcos_field!(String, fields[6]), //         *[1,3]? Coarse variability flag            (H6)
            r_Vmag: parse_hipparcos_field!(String, fields[7]), //          *[GHT] Source of magnitude                 (H7)
            RAdeg: parse_hipparcos_field!(ra_deg, fields[8]), //           *? alpha, degrees (ICRS, Epoch=J1991.25)   (H8)
            DEdeg: parse_hipparcos_field!(dec_deg, fields[9]), //          *? delta, degrees (ICRS, Epoch=J1991.25)   (H9)
            AstroRef: parse_hipparcos_field!(String, fields[10]), //       *[*+A-Z] Reference flag for astrometry    (H10)
            Plx: parse_hipparcos_field!(f64, fields[11]), //                ? Trigonometric parallax                 (H11)
            pmRA: parse_hipparcos_field!(f64, fields[12]), //              *? Proper motion mu_alpha.cos(delta), ICRS(H12)
            pmDE: parse_hipparcos_field!(f64, fields[13]), //              *? Proper motion mu_delta, ICRS           (H13)
            e_RAdeg: parse_hipparcos_field!(f64, fields[14]), //           *? Standard error in RA*cos(DEdeg)        (H14)
            e_DEdeg: parse_hipparcos_field!(f64, fields[15]), //           *? Standard error in DE                   (H15)
            e_Plx: parse_hipparcos_field!(f64, fields[16]), //              ? Standard error in Plx                  (H16)
            e_pmRA: parse_hipparcos_field!(f64, fields[17]), //             ? Standard error in pmRA                 (H17)
            e_pmDE: parse_hipparcos_field!(f64, fields[18]), //             ? Standard error in pmDE                 (H18)
            DE_RA: parse_hipparcos_field!(f64, fields[19]), //              [-1/1]? Correlation, DE/RA*cos(delta)    (H19)
            Plx_RA: parse_hipparcos_field!(f64, fields[20]), //             [-1/1]? Correlation, Plx/RA*cos(delta)   (H20)
            Plx_DE: parse_hipparcos_field!(f64, fields[21]), //             [-1/1]? Correlation, Plx/DE              (H21)
            pmRA_RA: parse_hipparcos_field!(f64, fields[22]), //            [-1/1]? Correlation, pmRA/RA*cos(delta)  (H22)
            pmRA_DE: parse_hipparcos_field!(f64, fields[23]), //            [-1/1]? Correlation, pmRA/DE             (H23)
            pmRA_Plx: parse_hipparcos_field!(f64, fields[24]), //           [-1/1]? Correlation, pmRA/Plx            (H24)
            pmDE_RA: parse_hipparcos_field!(f64, fields[25]), //            [-1/1]? Correlation, pmDE/RA*cos(delta)  (H25)
            pmDE_DE: parse_hipparcos_field!(f64, fields[26]), //            [-1/1]? Correlation, pmDE/DE             (H26)
            pmDE_Plx: parse_hipparcos_field!(f64, fields[27]), //           [-1/1]? Correlation, pmDE/Plx            (H27)
            pmDE_pmRA: parse_hipparcos_field!(f64, fields[28]), //          [-1/1]? Correlation, pmDE/pmRA           (H28)
            F1: parse_hipparcos_field!(usize, fields[29]), //               ? Percentage of rejected data            (H29)
            F2: parse_hipparcos_field!(f64, fields[30]), //                *? Goodness-of-fit parameter              (H30)
            BTmag: parse_hipparcos_field!(f64, fields[32]), //              ? Mean BT magnitude                      (H32)
            e_BTmag: parse_hipparcos_field!(f64, fields[33]), //            ? Standard error on BTmag                (H33)
            VTmag: parse_hipparcos_field!(f64, fields[34]), //              ? Mean VT magnitude                      (H34)
            e_VTmag: parse_hipparcos_field!(f64, fields[35]), //            ? Standard error on VTmag                (H35)
            m_BTmag: parse_hipparcos_field!(String, fields[36]), //        *[A-Z*-] Reference flag for BT and VTmag  (H36)
            B_V: parse_hipparcos_field!(f64, fields[37]), //                ? Johnson B-V colour                     (H37)
            e_B_V: parse_hipparcos_field!(f64, fields[38]), //              ? Standard error on B-V                  (H38)
            r_B_V: parse_hipparcos_field!(String, fields[39]), //           [GT] Source of B-V from Ground or Tycho  (H39)
            V_I: parse_hipparcos_field!(f64, fields[40]), //                ? Colour index in Cousins' system        (H40)
            e_V_I: parse_hipparcos_field!(f64, fields[41]), //              ? Standard error on V-I                  (H41)
            r_V_I: parse_hipparcos_field!(String, fields[42]), //          *[A-T] Source of V-I                      (H42)
            CombMag: parse_hipparcos_field!(String, fields[43]), //         [*] Flag for combined Vmag, B-V, V-I     (H43)
            Hpmag: parse_hipparcos_field!(f64, fields[44]), //             *? Median magnitude in Hipparcos system   (H44)
            e_Hpmag: parse_hipparcos_field!(f64, fields[45]), //           *? Standard error on Hpmag                (H45)
            Hpscat: parse_hipparcos_field!(f64, fields[46]), //             ? Scatter on Hpmag                       (H46)
            o_Hpmag: parse_hipparcos_field!(usize, fields[47]), //          ? Number of observations for Hpmag       (H47)
            m_Hpmag: parse_hipparcos_field!(String, fields[48]), //        *[A-Z*-] Reference flag for Hpmag         (H48)
            Hpmax: parse_hipparcos_field!(f64, fields[49]), //              ? Hpmag at maximum (5th percentile)      (H49)
            HPmin: parse_hipparcos_field!(f64, fields[50]), //              ? Hpmag at minimum (95th percentile)     (H50)
            Period: parse_hipparcos_field!(f64, fields[51]), //             ? Variability period (days)              (H51)
            HvarType: parse_hipparcos_field!(String, fields[52]), //       *[CDMPRU]? variability type               (H52)
            moreVar: parse_hipparcos_field!(String, fields[53]), //        *[12] Additional data about variability   (H53)
            morePhoto: parse_hipparcos_field!(String, fields[54]), //       [ABC] Light curve Annex                  (H54)
            CCDM: parse_hipparcos_field!(String, fields[55]), //            CCDM identifier                          (H55)
            n_CCDM: parse_hipparcos_field!(String, fields[56]), //         *[HIM] Historical status flag             (H56)
            Nsys: parse_hipparcos_field!(usize, fields[57]), //             ? Number of entries with same CCDM       (H57)
            Ncomp: parse_hipparcos_field!(usize, fields[58]), //            ? Number of components in this entry     (H58)
            MultFlag: parse_hipparcos_field!(String, fields[59]), //       *[CGOVX] Double/Multiple Systems flag     (H59)
            Source: parse_hipparcos_field!(String, fields[60]), //         *[PFILS] Astrometric source flag          (H60)
            Qual: parse_hipparcos_field!(String, fields[61]), //           *[ABCDS] Solution quality                 (H61)
            m_HIP: parse_hipparcos_field!(String, fields[62]), //           Component identifiers                    (H62)
            theta: parse_hipparcos_field!(usize, fields[63]), //            ? Position angle between components      (H63)
            rho: parse_hipparcos_field!(f64, fields[64]), //                ? Angular separation between components  (H64)
            e_rho: parse_hipparcos_field!(f64, fields[65]), //              ? Standard error on rho                  (H65)
            dHp: parse_hipparcos_field!(f64, fields[66]), //                ? Magnitude difference of components     (H66)
            e_dHp: parse_hipparcos_field!(f64, fields[67]), //              ? Standard error on dHp                  (H67)
            Survey: parse_hipparcos_field!(String, fields[68]), //          [S] Flag indicating a Survey Star        (H68)
            Chart: parse_hipparcos_field!(String, fields[69]), //          *[DG] Identification Chart                (H69)
            Notes: parse_hipparcos_field!(String, fields[70]), //          *[DGPWXYZ] Existence of notes             (H70)
            HD: parse_hipparcos_field!(usize, fields[71]), //               [1/359083]? HD number <III/135>          (H71)
            BD: parse_hipparcos_field!(String, fields[72]), //              Bonner DM <I/119>, <I/122>               (H72)
            CoD: parse_hipparcos_field!(String, fields[73]), //             Cordoba Durchmusterung (DM) <I/114>      (H73)
            CPD: parse_hipparcos_field!(String, fields[74]), //             Cape Photographic DM <I/108>             (H74)
            V_I_red: parse_hipparcos_field!(f64, fields[75]), //            V-I used for reductions                  (H75)
            SpType: parse_hipparcos_field!(String, fields[76]), //          Spectral type                            (H76)
            r_SpType: parse_hipparcos_field!(String, fields[77]), //       *[1234GKSX]? Source of spectral type      (H77)
        };
        if star.is_valid_parse() {
            Ok(star)
        } else {
            Err(())
        }
    }
}

impl ValidParse for HipparcosStar {
    fn is_valid_parse(&self) -> bool {
        self.HIP.is_some() && self.RAdeg.is_some() && self.DEdeg.is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::catalog::hipparcos::*;
    use crate::parse_catalog;

    #[test]
    fn test_hipparcosstar_from() {
        let s = String::from("H|           1| |00 00 00.22|+01 05 20.4| 9.10| |H|000.00091185|+01.08901332| |   3.54|   -5.20|   -1.88|  1.32|  0.74|  1.39|  1.36|  0.81| 0.32|-0.07|-0.11|-0.24| 0.09|-0.01| 0.10|-0.01| 0.01| 0.34|  0| 0.74|     1| 9.643|0.020| 9.130|0.019| | 0.482|0.025|T|0.55|0.03|L| | 9.2043|0.0020|0.017| 87| | 9.17| 9.24|       | | | |          | |  | 1| | | |  |   |       |     |     |    |S| | |224700|B+00 5077 |          |          |0.66|F5          |S");
        HipparcosStar::try_from(s).unwrap();
    }

    #[test]
    fn test_catalog() {
        let _stars = parse_catalog!(
            HipparcosStar,
            Path::new("data/Hipparcos/hip_main.dat"),
            None
        );
        println!("Number of stars: {}", _stars.len());
        println!("Last Star: {:?}", _stars.last().unwrap());
    }
}
