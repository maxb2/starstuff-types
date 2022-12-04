// Parse Yale Bright Star Catalog
// http://tdc-www.harvard.edu/catalogs/bsc5.html
// NOTE: run the `get_data.sh` script to get the tests to pass.

use super::ValidParse;
use crate::parse_trim;

#[allow(non_snake_case)] // Copying field names from original data source
#[derive(Debug, Clone)]
pub struct YaleStar {
    pub HR: Option<usize>, //           [1/9110]+ Harvard Revised Number = Bright Star Number
    pub Name: Option<String>, //        Name, generally Bayer and/or Flamsteed name
    pub DM: Option<String>, //          Durchmusterung Identification (zone in bytes 17-19)
    pub HD: Option<usize>, //           [1/225300]? Henry Draper Catalog Number
    pub SAO: Option<usize>, //          [1/258997]? SAO Catalog Number
    pub FK5: Option<usize>, //          ? FK5 star Number
    pub IRflag: Option<String>, //      [I] I if infrared source
    pub r_IRflag: Option<String>, //   *[ ':] Coded reference for infrared source
    pub Multiple: Option<String>, //   *[AWDIRS] Double or multiple-star code
    pub ADS: Option<String>, //         Aitken's Double Star Catalog (ADS) designation
    pub ADScomp: Option<String>, //     ADS number components
    pub VarID: Option<String>, //       Variable star identification
    pub RAh1900: Option<usize>, //      ?Hours RA, equinox B1900, epoch 1900.0 (1)
    pub RAm1900: Option<usize>, //      ?Minutes RA, equinox B1900, epoch 1900.0 (1)
    pub RAs1900: Option<f64>, //        ?Seconds RA, equinox B1900, epoch 1900.0 (1)
    pub DE_1900: Option<String>, //     ?Sign Dec, equinox B1900, epoch 1900.0 (1)
    pub DEd1900: Option<usize>, //      ?Degrees Dec, equinox B1900, epoch 1900.0 (1)
    pub DEm1900: Option<usize>, //      ?Minutes Dec, equinox B1900, epoch 1900.0 (1)
    pub DEs1900: Option<usize>, //      ?Seconds Dec, equinox B1900, epoch 1900.0 (1)
    pub RAh: Option<usize>, //          ?Hours RA, equinox J2000, epoch 2000.0 (1)
    pub RAm: Option<usize>, //          ?Minutes RA, equinox J2000, epoch 2000.0 (1)
    pub RAs: Option<f64>,  //           ?Seconds RA, equinox J2000, epoch 2000.0 (1)
    pub DE_: Option<String>, //         ?Sign Dec, equinox J2000, epoch 2000.0 (1)
    pub DEd: Option<usize>, //          ?Degrees Dec, equinox J2000, epoch 2000.0 (1)
    pub DEm: Option<usize>, //          ?Minutes Dec, equinox J2000, epoch 2000.0 (1)
    pub DEs: Option<usize>, //          ?Seconds Dec, equinox J2000, epoch 2000.0 (1)
    pub GLON: Option<f64>, //           ?Galactic longitude (1)
    pub GLAT: Option<f64>, //           ?Galactic latitude (1)
    pub Vmag: Option<f64>, //           ?Visual magnitude (1)
    pub n_Vmag: Option<String>, //     *[ HR] Visual magnitude code
    pub u_Vmag: Option<String>, //      [ :?] Uncertainty flag on V
    pub B_V: Option<f64>,  //           ? B-V color in the UBV system
    pub u_B_V: Option<String>, //       [ :?] Uncertainty flag on B-V
    pub U_B: Option<f64>,  //           ? U-B color in the UBV system
    pub u_U_B: Option<String>, //       [ :?] Uncertainty flag on U-B
    pub R_I: Option<f64>,  //           ? R-I   in system specified by n_R-I
    pub n_R_I: Option<String>, //       [CE:?D] Code for R-I system (Cousin, Eggen)
    pub SpType: Option<String>, //      Spectral type
    pub n_SpType: Option<String>, //    [evt] Spectral type code
    pub pmRA: Option<f64>, //          *?Annual proper motion in RA J2000, FK5 system
    pub pmDe: Option<f64>, //           ?Annual proper motion in Dec J2000, FK5 system
    pub n_Parallax: Option<String>, //  [D] D indicates a dynamical parallax, otherwise a trigonometric parallax
    pub Parallax: Option<f64>,      //  ? Trigonometric parallax (unless n_Parallax)
    pub RadVel: Option<usize>,      //  ? Heliocentric Radial Velocity
    pub n_RadVel: Option<String>,   // *[V?SB123O ] Radial velocity comments
    pub l_RotVel: Option<String>,   //  [<=> ] Rotational velocity limit characters
    pub RotVel: Option<usize>,      //  ? Rotational velocity, v sin i
    pub u_RotVel: Option<String>,   //  [ :v] uncertainty and variability flag on RotVel
    pub Dmag: Option<f64>,          //  ? Magnitude difference of double, or brightest multiple
    pub Sep: Option<f64>, //            ? Separation of components in Dmag if occultation binary.
    pub MultID: Option<String>, //      Identifications of components in Dmag
    pub MultCnt: Option<usize>, //      ? Number of components assigned to a multiple
    pub NoteFlag: Option<String>, //    [*] a star indicates that there is a note (see file notes)
}

impl TryFrom<String> for YaleStar {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let star = Self {
            HR: parse_trim!(usize, s[0..4]),
            Name: parse_trim!(String, s[4..14]),
            DM: parse_trim!(String, s[14..25]),
            HD: parse_trim!(usize, s[25..31]),
            SAO: parse_trim!(usize, s[31..37]),
            FK5: parse_trim!(usize, s[37..41]),
            IRflag: parse_trim!(String, s[41..42]),
            r_IRflag: parse_trim!(String, s[42..43]),
            Multiple: parse_trim!(String, s[43..44]),
            ADS: parse_trim!(String, s[44..49]),
            ADScomp: parse_trim!(String, s[49..51]),
            VarID: parse_trim!(String, s[51..60]),
            RAh1900: parse_trim!(usize, s[60..62]),
            RAm1900: parse_trim!(usize, s[62..64]),
            RAs1900: parse_trim!(f64, s[64..68]),
            DE_1900: parse_trim!(String, s[68..69]),
            DEd1900: parse_trim!(usize, s[69..71]),
            DEm1900: parse_trim!(usize, s[71..73]),
            DEs1900: parse_trim!(usize, s[73..75]),
            RAh: parse_trim!(usize, s[75..77]),
            RAm: parse_trim!(usize, s[77..79]),
            RAs: parse_trim!(f64, s[79..83]),
            DE_: parse_trim!(String, s[83..84]),
            DEd: parse_trim!(usize, s[84..86]),
            DEm: parse_trim!(usize, s[86..88]),
            DEs: parse_trim!(usize, s[88..90]),
            GLON: parse_trim!(f64, s[90..96]),
            GLAT: parse_trim!(f64, s[96..102]),
            Vmag: parse_trim!(f64, s[102..107]),
            n_Vmag: parse_trim!(String, s[107..108]),
            u_Vmag: parse_trim!(String, s[108..109]),
            B_V: parse_trim!(f64, s[109..114]),
            u_B_V: parse_trim!(String, s[114..115]),
            U_B: parse_trim!(f64, s[115..120]),
            u_U_B: parse_trim!(String, s[120..121]),
            R_I: parse_trim!(f64, s[121..126]),
            n_R_I: parse_trim!(String, s[126..127]),
            SpType: parse_trim!(String, s[127..147]),
            n_SpType: parse_trim!(String, s[147..148]),
            pmRA: parse_trim!(f64, s[148..154]),
            pmDe: parse_trim!(f64, s[154..160]),
            n_Parallax: parse_trim!(String, s[160..161]),
            Parallax: parse_trim!(f64, s[161..166]),
            RadVel: parse_trim!(usize, s[166..170]),
            n_RadVel: parse_trim!(String, s[170..174]),
            l_RotVel: parse_trim!(String, s[174..176]),
            RotVel: parse_trim!(usize, s[176..179]),
            u_RotVel: parse_trim!(String, s[179..180]),
            Dmag: parse_trim!(f64, s[180..184]),
            Sep: parse_trim!(f64, s[184..190]),
            MultID: parse_trim!(String, s[190..194]),
            MultCnt: parse_trim!(usize, s[194..196]),
            NoteFlag: parse_trim!(String, s[196..197]),
        };
        if star.is_valid_parse() {
            Ok(star)
        } else {
            Err(())
        }
    }
}

impl ValidParse for YaleStar {
    fn is_valid_parse(&self) -> bool {
        self.HR.is_some()
            && self.DE_.is_some()
            && self.DEd.is_some()
            && self.DEm.is_some()
            && self.DEs.is_some()
            && self.RAh.is_some()
            && self.RAm.is_some()
            && self.RAs.is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::catalog::yale::*;
    use crate::parse_catalog;

    #[test]
    fn test_yalestar_from() {
        let s = String::from("   1          BD+44 4550      3 36042          46           000001.1+444022000509.9+451345114.44-16.88 6.70  +0.07 +0.08         A1Vn               -0.012-0.018      -018      195  4.2  21.6AC   3 ");
        YaleStar::try_from(s).unwrap();
    }

    #[test]
    fn test_catalog() {
        let _stars = parse_catalog!(YaleStar, Path::new("data/Yale/bsc5.dat"), Some(197));
        println!("Number of stars: {}", _stars.len());
        println!("Last Star: {:?}", _stars.last().unwrap());
    }
}
