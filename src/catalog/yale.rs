/*!  [Yale Bright Star Catalog](http://tdc-www.harvard.edu/catalogs/bsc5.html) parser

> NOTE: run the `get_data.sh` script to get the tests to pass.

The Bright Star Catalogue,  5th Revised Ed. (Preliminary Version)

- Hoffleit D., Warren Jr W.H.
- <Astronomical Data Center, NSSDC/ADC (1991)>
- =1964BS....C......0H

ADC_Keywords: Combined data ; Stars, bright


## Description

> (prepared by Wayne H. Warren Jr., 1991 June 28)

The  Bright  Star  Catalogue  (BSC) is widely used as a source of
basic astronomical and astrophysical data for stars brighter than
magnitude 6.5.   The  catalog  contains  the  identifications  of
included stars in several other widely-used catalogs, double- and
multiple-star  identifications,  indication  of  variability  and
variable-star identifiers, equatorial positions for  B1900.0  and
J2000.0,  galactic  coordinates,  UBVRI photoelectric photometric
data when they exist, spectral types on  the  Morgan-Keenan  (MK)
classification   system,   proper  motions  (J2000.0),  parallax,
radial-   and   rotational-velocity   data,   and   multiple-star
information  (number  of  components,  separation,  and magnitude
differences) for known nonsingle stars.  In addition to the  data
file, there is an extensive remarks file that gives more detailed
information  on  individual  entries.   This information includes
star  names,  colors,  spectra,   variability   details,   binary
characteristics,  radial  and rotational velocities for companion
stars,  duplicity  information,  dynamical  parallaxes,   stellar
dimensions (radii and diameters), polarization, and membership in
stellar groups and clusters.  The existence of remarks is flagged
in the main data file.

The  BSC  contains  9110  objects,  of  which  9096 are stars (14
objects catalogued in the original compilation of 1908 are  novae
or  extragalactic objects that have been retained to preserve the
numbering, but most of their data are omitted), while the remarks
section is slightly larger than the main catalog.    The  present
edition of the compilation includes many new data and the remarks
section has been enlarged considerably.

This  preliminary version of the fifth edition of the Bright Star
Catalogue supersedes the published and machine-readable  versions
of  Hoffleit  (1982, Yale University Observatory) and is intended
for use until the final version of this edition is completed.  It
has  been  made  available  only   for   dissemination   on   the
Astronomical Data Center CD ROM.

The  brief  format description applies to the preliminary version
of the catalog only.   The  format  will  change  for  the  final
edition.


## Byte-by-byte Description of file: bsc5.dat

```text
--------------------------------------------------------------------------------
   Bytes Format  Units   Label    Explanations
--------------------------------------------------------------------------------
   1-  4  I4     ---     HR       [1/9110]+ Harvard Revised Number
                                    = Bright Star Number
   5- 14  A10    ---     Name     Name, generally Bayer and/or Flamsteed name
  15- 25  A11    ---     DM       Durchmusterung Identification (zone in
                                    bytes 17-19)
  26- 31  I6     ---     HD       [1/225300]? Henry Draper Catalog Number
  32- 37  I6     ---     SAO      [1/258997]? SAO Catalog Number
  38- 41  I4     ---     FK5      ? FK5 star Number
      42  A1     ---     IRflag   [I] I if infrared source
      43  A1     ---   r_IRflag  *[ ':] Coded reference for infrared source
      44  A1     ---    Multiple *[AWDIRS] Double or multiple-star code
  45- 49  A5     ---     ADS      Aitken's Double Star Catalog (ADS) designation
  50- 51  A2     ---     ADScomp  ADS number components
  52- 60  A9     ---     VarID    Variable star identification
  61- 62  I2     h       RAh1900  ?Hours RA, equinox B1900, epoch 1900.0 (1)
  63- 64  I2     min     RAm1900  ?Minutes RA, equinox B1900, epoch 1900.0 (1)
  65- 68  F4.1   s       RAs1900  ?Seconds RA, equinox B1900, epoch 1900.0 (1)
      69  A1     ---     DE-1900  ?Sign Dec, equinox B1900, epoch 1900.0 (1)
  70- 71  I2     deg     DEd1900  ?Degrees Dec, equinox B1900, epoch 1900.0 (1)
  72- 73  I2     arcmin  DEm1900  ?Minutes Dec, equinox B1900, epoch 1900.0 (1)
  74- 75  I2     arcsec  DEs1900  ?Seconds Dec, equinox B1900, epoch 1900.0 (1)
  76- 77  I2     h       RAh      ?Hours RA, equinox J2000, epoch 2000.0 (1)
  78- 79  I2     min     RAm      ?Minutes RA, equinox J2000, epoch 2000.0 (1)
  80- 83  F4.1   s       RAs      ?Seconds RA, equinox J2000, epoch 2000.0 (1)
      84  A1     ---     DE-      ?Sign Dec, equinox J2000, epoch 2000.0 (1)
  85- 86  I2     deg     DEd      ?Degrees Dec, equinox J2000, epoch 2000.0 (1)
  87- 88  I2     arcmin  DEm      ?Minutes Dec, equinox J2000, epoch 2000.0 (1)
  89- 90  I2     arcsec  DEs      ?Seconds Dec, equinox J2000, epoch 2000.0 (1)
  91- 96  F6.2   deg     GLON     ?Galactic longitude (1)
  97-102  F6.2   deg     GLAT     ?Galactic latitude (1)
 103-107  F5.2   mag     Vmag     ?Visual magnitude (1)
     108  A1     ---   n_Vmag    *[ HR] Visual magnitude code
     109  A1     ---   u_Vmag     [ :?] Uncertainty flag on V
 110-114  F5.2   mag     B-V      ? B-V color in the UBV system
     115  A1     ---   u_B-V      [ :?] Uncertainty flag on B-V
 116-120  F5.2   mag     U-B      ? U-B color in the UBV system
     121  A1     ---   u_U-B      [ :?] Uncertainty flag on U-B
 122-126  F5.2   mag     R-I      ? R-I   in system specified by n_R-I
     127  A1     ---   n_R-I      [CE:?D] Code for R-I system (Cousin, Eggen)
 128-147  A20    ---     SpType   Spectral type
     148  A1     ---   n_SpType   [evt] Spectral type code
 149-154  F6.3 arcsec/yr pmRA    *?Annual proper motion in RA J2000, FK5 system
 155-160  F6.3 arcsec/yr pmDE     ?Annual proper motion in Dec J2000, FK5 system
     161  A1     ---   n_Parallax [D] D indicates a dynamical parallax,
                                    otherwise a trigonometric parallax
 162-166  F5.3   arcsec  Parallax ? Trigonometric parallax (unless n_Parallax)
 167-170  I4     km/s    RadVel   ? Heliocentric Radial Velocity
 171-174  A4     ---   n_RadVel  *[V?SB123O ] Radial velocity comments
 175-176  A2     ---   l_RotVel   [<=> ] Rotational velocity limit characters
 177-179  I3     km/s    RotVel   ? Rotational velocity, v sin i
     180  A1     ---   u_RotVel   [ :v] uncertainty and variability flag on
                                    RotVel
 181-184  F4.1   mag     Dmag     ? Magnitude difference of double,
                                    or brightest multiple
 185-190  F6.1   arcsec  Sep      ? Separation of components in Dmag
                                    if occultation binary.
 191-194  A4     ---     MultID   Identifications of components in Dmag
 195-196  I2     ---     MultCnt  ? Number of components assigned to a multiple
     197  A1     ---     NoteFlag [*] a star indicates that there is a note
                                    (see file notes)
--------------------------------------------------------------------------------
```

Note (1): These fields are all blanks for stars removed from the Bright Star Catalogue (see notes).

Note on r_IRflag:
- Blank if from NASA merged Infrared Catalogue, Schmitz et al., 1978;
- ' if from Engles et al. 1982
- : if uncertain identification

Note on Multiple:

- A = Astrometric binary
- D = Duplicity discovered by occultation;
- I = Innes, Southern Double Star Catalogue (1927)
- R = Rossiter, Michigan Publ. 9, 1955
- S = Duplicity discovered by speckle interferometry.
- W = Worley (1978) update of the IDS;

Note on n_Vmag:
- blank = V on UBV Johnson system;
- R = HR magnitudes reduced to the UBV system;
- H = original HR magnitude.

Note on pmRA:
As usually assumed, the proper motion in RA is the projected
motion (cos(DE).d(RA)/dt), i.e. the total proper motion is
sqrt(pmRA^2^+pmDE^2^)

Note on n_RadVel:
- V  = variable radial velocity;
- V? = suspected variable radial velocity;
- SB, SB1, SB2, SB3 = spectroscopic binaries, single, double or triple lined spectra;
- O = orbital data available.

*/
use super::ValidParse;
use crate::parse_trim;

#[allow(non_snake_case)] // Copying field names from original data source
#[derive(Debug, Clone)]
pub struct YaleStar {
    /// \[1/9110\]+ Harvard Revised Number = Bright Star Number
    pub HR: Option<usize>,

    /// Name, generally Bayer and/or Flamsteed name
    pub Name: Option<String>,

    /// Durchmusterung Identification (zone in bytes 17-19)
    pub DM: Option<String>,

    /// \[1/225300\]? Henry Draper Catalog Number
    pub HD: Option<usize>,

    /// \[1/258997\]? SAO Catalog Number
    pub SAO: Option<usize>,

    /// ? FK5 star Number
    pub FK5: Option<usize>,

    /// \[I\] I if infrared source
    pub IRflag: Option<String>,

    /// \*\[ ':\] Coded reference for infrared source
    pub r_IRflag: Option<String>,

    /// \*\[AWDIRS\] Double or multiple-star code
    pub Multiple: Option<String>,

    /// Aitken's Double Star Catalog (ADS) designation
    pub ADS: Option<String>,

    /// ADS number components
    pub ADScomp: Option<String>,

    /// Variable star identification
    pub VarID: Option<String>,

    /// ?Hours RA, equinox B1900, epoch 1900.0 (1)
    pub RAh1900: Option<usize>,

    /// ?Minutes RA, equinox B1900, epoch 1900.0 (1)
    pub RAm1900: Option<usize>,

    /// ?Seconds RA, equinox B1900, epoch 1900.0 (1)
    pub RAs1900: Option<f64>,

    /// ?Sign Dec, equinox B1900, epoch 1900.0 (1)
    pub DE_1900: Option<String>,

    /// ?Degrees Dec, equinox B1900, epoch 1900.0 (1)
    pub DEd1900: Option<usize>,

    /// ?Minutes Dec, equinox B1900, epoch 1900.0 (1)
    pub DEm1900: Option<usize>,

    /// ?Seconds Dec, equinox B1900, epoch 1900.0 (1)
    pub DEs1900: Option<usize>,

    /// ?Hours RA, equinox J2000, epoch 2000.0 (1)
    pub RAh: Option<usize>,

    /// ?Minutes RA, equinox J2000, epoch 2000.0 (1)
    pub RAm: Option<usize>,

    /// ?Seconds RA, equinox J2000, epoch 2000.0 (1)
    pub RAs: Option<f64>,

    /// ?Sign Dec, equinox J2000, epoch 2000.0 (1)
    pub DE_: Option<String>,

    /// ?Degrees Dec, equinox J2000, epoch 2000.0 (1)
    pub DEd: Option<usize>,

    /// ?Minutes Dec, equinox J2000, epoch 2000.0 (1)
    pub DEm: Option<usize>,

    /// ?Seconds Dec, equinox J2000, epoch 2000.0 (1)
    pub DEs: Option<usize>,

    /// ?Galactic longitude (1)
    pub GLON: Option<f64>,

    /// ?Galactic latitude (1)
    pub GLAT: Option<f64>,

    /// ?Visual magnitude (1)
    pub Vmag: Option<f64>,

    /// \*\[ HR\] Visual magnitude code
    pub n_Vmag: Option<String>,

    /// \[ :?\] Uncertainty flag on V
    pub u_Vmag: Option<String>,

    /// ? B-V color in the UBV system
    pub B_V: Option<f64>,

    /// \[ :?\] Uncertainty flag on B-V
    pub u_B_V: Option<String>,

    /// ? U-B color in the UBV system
    pub U_B: Option<f64>,

    /// \[ :?\] Uncertainty flag on U-B
    pub u_U_B: Option<String>,

    /// ? R-I   in system specified by n_R-I
    pub R_I: Option<f64>,

    /// \[CE:?D\] Code for R-I system (Cousin, Eggen)
    pub n_R_I: Option<String>,

    /// Spectral type
    pub SpType: Option<String>,

    /// \[evt\] Spectral type code
    pub n_SpType: Option<String>,

    /// *?Annual proper motion in RA J2000, FK5 system
    pub pmRA: Option<f64>,

    /// ?Annual proper motion in Dec J2000, FK5 system
    pub pmDe: Option<f64>,

    /// \[D\] D indicates a dynamical parallax, otherwise a trigonometric parallax
    pub n_Parallax: Option<String>,

    /// ? Trigonometric parallax (unless n_Parallax)
    pub Parallax: Option<f64>,

    /// ? Heliocentric Radial Velocity
    pub RadVel: Option<usize>,

    /// \*\[V?SB123O \] Radial velocity comments
    pub n_RadVel: Option<String>,

    /// \[<=> \] Rotational velocity limit characters
    pub l_RotVel: Option<String>,

    /// ? Rotational velocity, v sin i
    pub RotVel: Option<usize>,

    /// \[ :v\] uncertainty and variability flag on RotVel
    pub u_RotVel: Option<String>,

    /// ? Magnitude difference of double, or brightest multiple
    pub Dmag: Option<f64>,

    /// ? Separation of components in Dmag if occultation binary.
    pub Sep: Option<f64>,

    /// Identifications of components in Dmag
    pub MultID: Option<String>,

    /// ? Number of components assigned to a multiple
    pub MultCnt: Option<usize>,

    /// \[\*\] a star indicates that there is a note (see file notes)
    pub NoteFlag: Option<String>,
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
    #[ignore]
    fn test_catalog() {
        let data_file = "data/Yale/bsc5.dat";

        if !std::path::Path::new(&data_file).exists() {
            panic!("File \"{}\" doesn't exist. Please run \"get_data.sh\" to fetch the data required for this test.", &data_file)
        };

        let _stars = parse_catalog!(YaleStar, Path::new(&data_file), Some(197));
        println!("Number of stars: {}", _stars.len());
        println!("Last Star: {:?}", _stars.last().unwrap());
    }
}
