/*! # [Hipparcos Catalog](https://heasarc.gsfc.nasa.gov/W3Browse/all/hipparcos.html) parser

> NOTE: run the `get_data.sh` script to get the tests to pass.

## [The Hipparcos and Tycho Catalogues](https://heasarc.gsfc.nasa.gov/W3Browse/all/hipparcos.html) (ESA 1997)

- ESA 1997
- <ESA, 1997, The Hipparcos Catalogue, ESA SP-1200>
- <ESA, 1997, The Tycho Catalogue, ESA SP-1200>
- =1997HIP...C......0E

ADC_Keywords: Positional data ; Proper motions ; Parallaxes, trigonometric ;
              Photometry ; Fundamental catalog ; Stars, double and multiple

Mission_Name: Hipparcos

### Description

The Hipparcos and Tycho Catalogues are the primary products of the
European Space Agency's astrometric mission, Hipparcos. The satellite,
which operated for four years, returned high quality scientific data
from November 1989 to March 1993.

Each of the catalogues contains a large quantity of very high quality
astrometric and photometric data. In addition there are associated
annexes featuring variability and double/multiple star data, and solar
system astrometric and photometric measurements. In the case of the
Hipparcos Catalogue, the principal parts are provided in both printed
and machine-readable form (on CDROM). In the case of the Tycho
Catalogue, results are provided in machine-readable form only (on
CDROM). Although in general only the final reduced and calibrated
astrometric and photometric data are provided, some auxiliary files
containing results from intermediate stages of the data processing, of
relevance for the more-specialised user, have also been retained for
publication. (Some, but not all, data files are available from the
Centre de Donnees astronomiques de Strasbourg.)

The global data analysis tasks, proceeding from nearly 1000 Gbit of
raw satellite data to the final catalogues, was a lengthy and complex
process, and was undertaken by the NDAC and FAST Consortia, together
responsible for the production of the Hipparcos Catalogue, and the
Tycho Consortium, responsible for the production of the Tycho
Catalogue. A fourth scientific consortium, the INCA Consortium, was
responsible for the construction of the Hipparcos observing programme,
compiling the best-available data for the selected stars before launch
into the Hipparcos Input Catalogue. The production of the Hipparcos
and Tycho Catalogues marks the formal end of the involvement in the
mission by the European Space Agency and the four scientific
consortia.

For more complete and detailed information on the data, the user is
advised to refer to Volume 1 ("Introduction and Guide to the Data",
ESA SP-1200) of the printed Hipparcos and Tycho Catalogues. The user
should also note that in order to convert the Double and Multiple
Systems (Component solutions) data file hip_dm_c.dat into FITS format
it is first necessary to filter the file according to whether the
entry is a component record (identified by COMP in field DCM5) or a
correlation record (identified by CORR in field DCM5) because of the
different structures of the respective records. On a Unix system this
can be achieved as follows:

grep COMP hip_dm_c.dat > h_dm_com.dat
grep CORR hip_dm_c.dat > h_dm_cor.dat

The catalogue description file (this file) gives the relevant
information for converting the main data files, including h_dm_cor.dat
and h_dm_com.dat, into FITS format.

The machine readable data files (i.e. those available on CD-ROM and
the subset available from the CDS) contain several extra fields in
addition to the data from the printed catalogue. These fields are
identified by the letter `M' in the data label (e.g. the field DGM1
contains data only available in the machine readable file
hip_dm_g.dat).


### Byte-by-byte Description of file: hip_main.dat

```text
--------------------------------------------------------------------------------
   Bytes Format Units   Label     Explanations
--------------------------------------------------------------------------------
       1  A1    ---     Catalog   [H] Catalogue (H=Hipparcos)               (H0)
   9- 14  I6    ---     HIP       Identifier (HIP number)                   (H1)
      16  A1    ---     Proxy    *[HT] Proximity flag                       (H2)
  18- 28  A11   ---     RAhms     Right ascension in h m s, ICRS (J1991.25) (H3)
  30- 40  A11   ---     DEdms     Declination in deg ' ", ICRS (J1991.25)   (H4)
  42- 46  F5.2  mag     Vmag      ? Magnitude in Johnson V                  (H5)
      48  I1    ---     VarFlag  *[1,3]? Coarse variability flag            (H6)
      50  A1    ---   r_Vmag     *[GHT] Source of magnitude                 (H7)
  52- 63  F12.8 deg     RAdeg    *? alpha, degrees (ICRS, Epoch=J1991.25)   (H8)
  65- 76  F12.8 deg     DEdeg    *? delta, degrees (ICRS, Epoch=J1991.25)   (H9)
      78  A1    ---     AstroRef *[*+A-Z] Reference flag for astrometry    (H10)
  80- 86  F7.2  mas     Plx       ? Trigonometric parallax                 (H11)
  88- 95  F8.2 mas/yr   pmRA     *? Proper motion mu_alpha.cos(delta), ICRS(H12)
  97-104  F8.2 mas/yr   pmDE     *? Proper motion mu_delta, ICRS           (H13)
 106-111  F6.2  mas   e_RAdeg    *? Standard error in RA*cos(DEdeg)        (H14)
 113-118  F6.2  mas   e_DEdeg    *? Standard error in DE                   (H15)
 120-125  F6.2  mas   e_Plx       ? Standard error in Plx                  (H16)
 127-132  F6.2 mas/yr e_pmRA      ? Standard error in pmRA                 (H17)
 134-139  F6.2 mas/yr e_pmDE      ? Standard error in pmDE                 (H18)
 141-145  F5.2  ---     DE:RA     [-1/1]? Correlation, DE/RA*cos(delta)    (H19)
 147-151  F5.2  ---     Plx:RA    [-1/1]? Correlation, Plx/RA*cos(delta)   (H20)
 153-157  F5.2  ---     Plx:DE    [-1/1]? Correlation, Plx/DE              (H21)
 159-163  F5.2  ---     pmRA:RA   [-1/1]? Correlation, pmRA/RA*cos(delta)  (H22)
 165-169  F5.2  ---     pmRA:DE   [-1/1]? Correlation, pmRA/DE             (H23)
 171-175  F5.2  ---     pmRA:Plx  [-1/1]? Correlation, pmRA/Plx            (H24)
 177-181  F5.2  ---     pmDE:RA   [-1/1]? Correlation, pmDE/RA*cos(delta)  (H25)
 183-187  F5.2  ---     pmDE:DE   [-1/1]? Correlation, pmDE/DE             (H26)
 189-193  F5.2  ---     pmDE:Plx  [-1/1]? Correlation, pmDE/Plx            (H27)
 195-199  F5.2  ---     pmDE:pmRA [-1/1]? Correlation, pmDE/pmRA           (H28)
 201-203  I3    %       F1        ? Percentage of rejected data            (H29)
 205-209  F5.2  ---     F2       *? Goodness-of-fit parameter              (H30)
 211-216  I6    ---     ---       HIP number (repetition)                  (H31)
 218-223  F6.3  mag     BTmag     ? Mean BT magnitude                      (H32)
 225-229  F5.3  mag   e_BTmag     ? Standard error on BTmag                (H33)
 231-236  F6.3  mag     VTmag     ? Mean VT magnitude                      (H34)
 238-242  F5.3  mag   e_VTmag     ? Standard error on VTmag                (H35)
     244  A1    ---   m_BTmag    *[A-Z*-] Reference flag for BT and VTmag  (H36)
 246-251  F6.3  mag     B-V       ? Johnson B-V colour                     (H37)
 253-257  F5.3  mag   e_B-V       ? Standard error on B-V                  (H38)
     259  A1    ---   r_B-V       [GT] Source of B-V from Ground or Tycho  (H39)
 261-264  F4.2  mag     V-I       ? Colour index in Cousins' system        (H40)
 266-269  F4.2  mag   e_V-I       ? Standard error on V-I                  (H41)
     271  A1    ---   r_V-I      *[A-T] Source of V-I                      (H42)
     273  A1    ---     CombMag   [*] Flag for combined Vmag, B-V, V-I     (H43)
 275-281  F7.4  mag     Hpmag    *? Median magnitude in Hipparcos system   (H44)
 283-288  F6.4  mag   e_Hpmag    *? Standard error on Hpmag                (H45)
 290-294  F5.3  mag     Hpscat    ? Scatter on Hpmag                       (H46)
 296-298  I3    ---   o_Hpmag     ? Number of observations for Hpmag       (H47)
     300  A1    ---   m_Hpmag    *[A-Z*-] Reference flag for Hpmag         (H48)
 302-306  F5.2  mag     Hpmax     ? Hpmag at maximum (5th percentile)      (H49)
 308-312  F5.2  mag     HPmin     ? Hpmag at minimum (95th percentile)     (H50)
 314-320  F7.2  d       Period    ? Variability period (days)              (H51)
     322  A1    ---     HvarType *[CDMPRU]? variability type               (H52)
     324  A1    ---     moreVar  *[12] Additional data about variability   (H53)
     326  A1    ---     morePhoto [ABC] Light curve Annex                  (H54)
 328-337  A10   ---     CCDM      CCDM identifier                          (H55)
     339  A1    ---   n_CCDM     *[HIM] Historical status flag             (H56)
 341-342  I2    ---     Nsys      ? Number of entries with same CCDM       (H57)
 344-345  I2    ---     Ncomp     ? Number of components in this entry     (H58)
     347  A1    ---     MultFlag *[CGOVX] Double/Multiple Systems flag     (H59)
     349  A1    ---     Source   *[PFILS] Astrometric source flag          (H60)
     351  A1    ---     Qual     *[ABCDS] Solution quality                 (H61)
 353-354  A2    ---   m_HIP       Component identifiers                    (H62)
 356-358  I3    deg     theta     ? Position angle between components      (H63)
 360-366  F7.3  arcsec  rho       ? Angular separation between components  (H64)
 368-372  F5.3  arcsec  e_rho     ? Standard error on rho                  (H65)
 374-378  F5.2  mag     dHp       ? Magnitude difference of components     (H66)
 380-383  F4.2  mag   e_dHp       ? Standard error on dHp                  (H67)
     385  A1    ---     Survey    [S] Flag indicating a Survey Star        (H68)
     387  A1    ---     Chart    *[DG] Identification Chart                (H69)
     389  A1    ---     Notes    *[DGPWXYZ] Existence of notes             (H70)
 391-396  I6    ---     HD        [1/359083]? HD number <III/135>          (H71)
 398-407  A10   ---     BD        Bonner DM <I/119>, <I/122>               (H72)
 409-418  A10   ---     CoD       Cordoba Durchmusterung (DM) <I/114>      (H73)
 420-429  A10   ---     CPD       Cape Photographic DM <I/108>             (H74)
 431-434  F4.2  mag     (V-I)red  V-I used for reductions                  (H75)
 436-447  A12   ---     SpType    Spectral type                            (H76)
     449  A1    ---   r_SpType   *[1234GKSX]? Source of spectral type      (H77)
--------------------------------------------------------------------------------
```

Note on Proxy: this flag provides a coarse indication of the presence
of nearby objects within 10arcsec of the given entry.
If non-blank, it indicates that
'H' there is one or more distinct Hipparcos Catalogue entries,
    or distinct components of system from h_dm_com.dat
'T' there is one or more distinct Tycho entries
If 'H' and 'T' apply, 'H' is adopted.
The 'T' flag implies either an inconsistency between the Hipparcos
and Tycho catalogues, or a deficiency in one or both of the
catalogues.

Note on RAdeg, DEdeg: right ascension and declination are
expressed in degrees for epoch J1991.25 (JD2448349.0625 (TT)) in the
ICRS (International Celestial Reference System, consistent with
J2000) reference system.
There are 263 cases where these fields are missing (no astrometric
solution could be found)

Note on pmRA, pmDE:
The proper motions refer to the ICRS and to the epoch J1991.25.

Note on e_RAdeg, e_DEdeg:
The standard errors refer to the epoch J1991.25, and represent a
minimum of the error on the position. The actual standard error
on the positions is increasing for epochs increasingly differing
from the nominal J1991.25 epoch.

Note on VarFlag: the values are 1: < 0.06mag ; 2: 0.06-0.6mag ; 3: >0.6mag

Note on r_Vmag: the source is G = ground-based, H=HIP, T=Tycho

Note on AstroRef: this flag indicates that the astrometric parameters in H3-4 and H8-30 refer to:

- A to Z: the letter indicates the component of a double or multiple system
- \*: the photocentre of a double or multiple system
- +: the centre of mass

Note on F2: values exceeding +3 indicate a bad fit to the data.

Note on m_BTmag: this flag indicates the component or combined photometry:

- A to Z : the letter indicates the component measured in Tycho (non-single star)
- \* : the photometry refers to all components of the Hipparcos entry
- \- : single-pointing triple or quadruple system

Note on r_V-I: the origin of the V-I colour, in summary:

- 'A'        for an observation of V-I in Cousins' system;
- 'B' to 'K' when V-I derived from measurements in other bands/photoelectric systems
- 'L' to 'P' when V-I derived from Hipparcos and Star Mapper photometry
- 'Q'        for long-period variables
- 'R' to 'T' when colours are unknown

Note on Hpmag, e_Hpmag: the Hipparcos magnitude could not be determined for 14 stars.

Note on m_Hpmag: this flag indicates for double or multiple entries:

- A to Z : the letter indicates the specified component measured
- \* : combined Hpmag of a double system, corrected for attenuation
- \- : combined Hpmag of a multiple system, not corrected for attenuation

Note on HvarType: Hipparcos-defined type of variability (a blank entry signifies that the entry could not be classified as variable or constant):

- C : no variability detected ("constant")
- D : duplicity-induced variability
- M : possibly micro-variable (amplitude < 0.03mag)
- P : periodic variable
- R : V-I colour index was revised due to variability analysis
- U : unsolved variable which does not fall in the other categories

Note on moreVar: more data about periodic variability are provided

Note on n_CCDM: the flag takes the following values:

- H : determined multiple by Hipparcos, previously unknown
- I : system previously identified as multiple in HIC <I/196> (annex1)
- M : miscellaneous (system identified after publication of HIC)

Note on MultFlag: indicates that further details are given in the Double and Multiple Systems Annex:

- C : solutions for the components
- G : acceleration or higher order terms
- O : orbital solutions
- V : variability-induced movers (apparent motion arises from variability)
- X : stochastic solution (probably astrometric binaries with short period)

Note on Source: qualifies the source of the astrometric parameters H8-30 with a 'C' in MultFlag:

- P : primary target of a 2- or 3-pointing system
- F : secondary or tertiary of a 2- or 3-pointing 'fixed' system (common parallax and proper motions)
- I : secondary or tertiary of a 2- or 3-pointing 'independent' system (no constraints on parallax or proper motions)
- L : secondary or tertiary of a 2- or 3-pointing 'linear' system (common parallax)
- S : astrometric parameters from 'single-star merging' process.

Note on Qual: Reliability of the double or multiple star solution:
    A=good, B=fair, C=poor, D=uncertain, S=suspected non-single

Note on Chart: the chart was produced:

- D : from the STScI Digitized Sky Survey
- G : from the Guide Star Catalog

Note on Notes: the flag has the following meaning:

- D : double and multiple systems note only (note in hd_notes.doc file)
- G : general note only (note in hg_notes.doc file)
- P : photometric notes only (note in hp_notes.doc file)
- W : D + P
- X : D + G
- Y : G + P
- Z : D + G + P

Note on r_SpType: the flag indicates the source, as:

- 1 : Michigan catalogue for the HD stars, vol. 1 (Houk+, 1975) <III/31>
- 2 : Michigan catalogue for the HD stars, vol. 2 (Houk, 1978)  <III/51>
- 3 : Michigan Catalogue for the HD stars, vol. 3 (Houk, 1982)  <III/80>
- 4 : Michigan Catalogue for the HD stars, vol. 4 (Houk+, 1988) <III/133>
- G : updated after publication of the HIC <I/196>
- K : General Catalog of Variable Stars, 4th Ed. (Kholopov+ 1988) <II/214>
- S : SIMBAD data-base <http://cdsweb.u-strasbg.fr/Simbad.html>
- X : Miscellaneous
- A blank entry has no corresponding information.

The following is a correlation table of the Hipparcos Catalog fields, the parameter names as implemented by the CDS, and the HEASARC names for these parameters:

```text
Hipparcos   CDS Name    HEASARC Name       Description
Cat. Field

--          * New *    Name               /Catalog Designation
H0          Catalog    * Not Displayed *  /Catalogue (H=Hipparcos)
H1          HIP        HIP_Number         /Identifier (HIP number)
H2          Proxy      Prox_10asec        /Proximity flag
H3          RAhms      RA                 /RA in h m s, ICRS (J1991.25)
H4          DEdms      Dec                /Dec in deg ' ", ICRS (J1991.25)
H5          Vmag       Vmag               /Magnitude in Johnson V
H6          VarFlag    Var_Flag           /Coarse variability flag
H7          r_Vmag     Vmag_Source        /Source of magnitude
H8          RAdeg      RA_Deg             /RA in degrees (ICRS, Epoch-J1991.25)
H9          DEdeg      Dec_Deg            /Dec in degrees (ICRS, Epoch-J1991.25)
H10         AstroRef   Astrom_Ref_Dbl     /Reference flag for astrometry
H11         Plx        Parallax           /Trigonometric parallax
H12         pmRA       pm_RA              /Proper motion in RA
H13         pmDE       pm_Dec             /Proper motion in Dec
H14         e_RAdeg    RA_Error           /Standard error in RA*cos(Dec_Deg)
H15         e_DEdeg    Dec_Error          /Standard error in Dec_Deg
H16         e_Plx      Parallax_Error     /Standard error in Parallax
H17         e_pmRA     pm_RA_Error        /Standard error in pmRA
H18         e_pmDE     pm_Dec_Error       /Standard error in pmDE
H19         DE:RA      Crl_Dec_RA         /(DE over RA)xCos(delta)
H20         Plx:RA     Crl_Plx_RA         /(Plx over RA)xCos(delta)
H21         Plx:DE     Crl_Plx_Dec        /(Plx over DE)
H22         pmRA:RA    Crl_pmRA_RA        /(pmRA over RA)xCos(delta)
H23         pmRA:DE    Crl_pmRA_Dec       /(pmRA over DE)
H24         pmRA:Plx   Crl_pmRA_Plx       /(pmRA over Plx)
H25         pmDE:RA    Crl_pmDec_RA       /(pmDE over RA)xCos(delta)
H26         pmDE:DE    Crl_pmDec_Dec      /(pmDE over DE)
H27         pmDE:Plx   Crl_pmDec_Plx      /(pmDE over Plx)
H28         pmDE:pmRA  Crl_pmDec_pmRA     /(pmDE over pmRA)
H29         F1         Reject_Percent     /Percentage of rejected data
H30         F2         Quality_Fit        /Goodness-of-fit parameter
H31         ---        * Not Displayed *  /HIP number (repetition)
H32         BTmag      BT_Mag             /Mean BT magnitude
H33         e_BTmag    BT_Mag_Error       /Standard error on BTmag
H34         VTmag      VT_Mag             /Mean VT magnitude
H35         e_VTmag    VT_Mag_Error       /Standard error on VTmag
H36         m_BTmag    BT_Mag_Ref_Dbl     /Reference flag for BT and VTmag
H37         B-V        BV_Color           /Johnson BV colour
H38         e_B-V      BV_Color_Error     /Standard error on BV
H39         r_B-V      BV_Mag_Source      /Source of BV from Ground or Tycho
H40         V-I        VI_Color           /Colour index in Cousins' system
H41         e_V-I      VI_Color_Error     /Standard error on VI
H42         r_V-I      VI_Color_Source    /Source of VI
H43         CombMag    Mag_Ref_Dbl        /Flag for combined Vmag, BV, VI
H44         Hpmag      Hip_Mag            /Median magnitude in Hipparcos system
H45         e_Hpmag    Hip_Mag_Error      /Standard error on Hpmag
H46         Hpscat     Scat_Hip_Mag       /Scatter of Hpmag
H47         o_Hpmag    N_Obs_Hip_Mag      /Number of observations for Hpmag
H48         m_Hpmag    Hip_Mag_Ref_Dbl    /Reference flag for Hpmag
H49         Hpmax      Hip_Mag_Max        /Hpmag at maximum (5th percentile)
H50         HPmin      Hip_Mag_Min        /Hpmag at minimum (95th percentile)
H51         Period     Var_Period         /Variability period (days)
H52         HvarType   Hip_Var_Type       /Variability type
H53         moreVar    Var_Data_Annex     /Additional data about variability
H54         morePhoto  Var_Curv_Annex     /Light curve Annex
H55         CCDM       CCDM_Id            /CCDM identifier
H56         n_CCDM     CCDM_History       /Historical status flag
H57         Nsys       CCDM_N_Entries     /Number of entries with same CCDM
H58         Ncomp      CCDM_N_Comp        /Number of components in this entry
H59         MultFlag   Dbl_Mult_Annex     /Double and or Multiple Systems flag
H60         Source     Astrom_Mult_Source /Astrometric source flag
H61         Qual       Dbl_Soln_Qual      /Solution quality flag
H62         m_HIP      Dbl_Ref_ID         /Component identifiers
H63         theta      Dbl_Theta          /Position angle between components
H64         rho        Dbl_Rho            /Angular separation of components
H65         e_rho      Rho_Error          /Standard error of rho
H66         dHp        Diff_Hip_Mag       /Magnitude difference of components
H67         e_dHp      dHip_Mag_Error     /Standard error in dHp
H68         Survey     Survey_Star        /Flag indicating a Survey Star
H69         Chart      ID_Chart           /Identification Chart
H70         Notes      Notes              /Existence of notes
H71         HD         HD_Id              /HD number <III 135>
H72         BD         BD_Id              /Bonner DM <I 119>, <I 122>
H73         CoD        CoD_Id             /Cordoba Durchmusterung (DM) <I 114>
H74         CPD        CPD_Id             /Cape Photographic DM <I 108>
H75         (V-I)red   VI_Color_Reduct    /VI used for reductions
H76         SpType     Spect_Type         /Spectral type
H77         r_SpType   Spect_Type_Source  /Source of spectral type
--          * New *    Class              /HEASARC BROWSE classification
```

*/
use super::ValidParse;
use crate::angle::{Angle, Dms, Hms, Sign};
use crate::coord::{Declination, RightAscension};

use std::convert::TryFrom;

use crate::parse_trim;

/// Parse Hipparcos field
macro_rules! parse_hipparcos_field {
    // Right ascension from float degree
    (ra_deg, $s:expr) => {
        match parse_hipparcos_field!(f64, $s) {
            Some(deg) => Some(RightAscension(Angle::Degree(deg))),
            None => None,
        }
    };
    // Declination from float degree
    (dec_deg, $s:expr) => {
        match parse_hipparcos_field!(f64, $s) {
            Some(deg) => Some(Declination(Angle::Degree(deg))),
            None => None,
        }
    };
    // Right ascension from "hh mm ss.s"
    (ra_hms, $s:expr) => {{
        let fields: Vec<&str> = $s.split(" ").collect();
        RightAscension(Angle::from(Hms::new(
            Sign::Positive,
            parse_hipparcos_field!(u32, fields[0]).unwrap(),
            parse_hipparcos_field!(u32, fields[1]).unwrap(),
            parse_hipparcos_field!(f64, fields[2]).unwrap(),
        )))
    }};
    // Right ascension from "+dd mm ss.s"
    (dec_dms, $s:expr) => {{
        let fields: Vec<&str> = $s.split(" ").collect();
        let sign: Sign = match &fields[0][0..1] {
            "+" => Sign::Positive,
            "-" => Sign::Negative,
            _ => panic!(),
        };
        Declination(Angle::from(Dms::new(
            sign,
            parse_hipparcos_field!(u32, fields[0][1..]).unwrap(),
            parse_hipparcos_field!(u32, fields[1]).unwrap(),
            parse_hipparcos_field!(f64, fields[2]).unwrap(),
        )))
    }};
    // Parse a given type with the field trimmed of spaces
    ($T:ty, $s:expr) => {
        parse_trim!($T, $s)
    };
}

/**



*/

#[allow(non_snake_case)] // Copying field names from original data source
#[derive(Debug, Clone)]
pub struct HipparcosStar {
    /// Catalogue (H=Hipparcos)
    pub catalog: Option<String>,

    /// Identifier (HIP number)
    pub HIP: Option<usize>,

    /// Proximity flag
    pub Proxy: Option<String>,

    /// RA in h m s, ICRS (J1991.25)
    pub right_ascension: RightAscension,

    ///Dec in deg ' ", ICRS (J1991.25)
    pub declination: Declination,

    ///Magnitude in Johnson V
    pub Vmag: Option<f64>,

    ///Coarse variability flag
    pub VarFlag: Option<String>,

    ///Source of magnitude
    pub r_Vmag: Option<String>,

    ///RA in degrees (ICRS, Epoch-J1991.25)
    pub RAdeg: Option<RightAscension>,

    ///Dec in degrees (ICRS, Epoch-J1991.25)
    pub DEdeg: Option<Declination>,

    ///Reference flag for astrometry
    pub AstroRef: Option<String>,

    ///Trigonometric parallax
    pub Plx: Option<f64>,

    ///Proper motion in RA
    pub pmRA: Option<f64>,

    ///Proper motion in Dec
    pub pmDE: Option<f64>,

    ///Standard error in RA*cos(Dec_Deg)
    pub e_RAdeg: Option<f64>,

    ///Standard error in Dec_Deg
    pub e_DEdeg: Option<f64>,

    ///Standard error in Parallax
    pub e_Plx: Option<f64>,

    ///Standard error in pmRA
    pub e_pmRA: Option<f64>,

    ///Standard error in pmDE
    pub e_pmDE: Option<f64>,

    ///(DE over RA)xCos(delta)
    pub DE_RA: Option<f64>,

    ///(Plx over RA)xCos(delta)
    pub Plx_RA: Option<f64>,

    ///(Plx over DE)
    pub Plx_DE: Option<f64>,

    ///(pmRA over RA)xCos(delta)
    pub pmRA_RA: Option<f64>,

    ///(pmRA over DE)
    pub pmRA_DE: Option<f64>,

    ///(pmRA over Plx)
    pub pmRA_Plx: Option<f64>,

    ///(pmDE over RA)xCos(delta)
    pub pmDE_RA: Option<f64>,

    ///(pmDE over DE)
    pub pmDE_DE: Option<f64>,

    ///(pmDE over Plx)
    pub pmDE_Plx: Option<f64>,

    ///(pmDE over pmRA)
    pub pmDE_pmRA: Option<f64>,

    ///Percentage of rejected data
    pub F1: Option<usize>,

    ///Goodness-of-fit parameter
    pub F2: Option<f64>,

    ///Mean BT magnitude
    pub BTmag: Option<f64>,

    ///Standard error on BTmag
    pub e_BTmag: Option<f64>,

    ///Mean VT magnitude
    pub VTmag: Option<f64>,

    ///Standard error on VTmag
    pub e_VTmag: Option<f64>,

    ///Reference flag for BT and VTmag
    pub m_BTmag: Option<String>,

    ///Johnson BV colour
    pub B_V: Option<f64>,

    ///Standard error on BV
    pub e_B_V: Option<f64>,

    ///Source of BV from Ground or Tycho
    pub r_B_V: Option<String>,

    ///Colour index in Cousins' system
    pub V_I: Option<f64>,

    ///Standard error on VI
    pub e_V_I: Option<f64>,

    ///Source of VI
    pub r_V_I: Option<String>,

    ///Flag for combined Vmag, BV, VI
    pub CombMag: Option<String>,

    ///Median magnitude in Hipparcos system
    pub Hpmag: Option<f64>,

    ///Standard error on Hpmag
    pub e_Hpmag: Option<f64>,

    ///Scatter of Hpmag
    pub Hpscat: Option<f64>,

    ///Number of observations for Hpmag
    pub o_Hpmag: Option<usize>,

    ///Reference flag for Hpmag
    pub m_Hpmag: Option<String>,

    ///Hpmag at maximum (5th percentile)
    pub Hpmax: Option<f64>,

    ///Hpmag at minimum (95th percentile)
    pub HPmin: Option<f64>,

    ///Variability period (days)
    pub Period: Option<f64>,

    ///Variability type
    pub HvarType: Option<String>,

    ///Additional data about variability
    pub moreVar: Option<String>,

    ///Light curve Annex
    pub morePhoto: Option<String>,

    ///CCDM identifier
    pub CCDM: Option<String>,

    ///Historical status flag
    pub n_CCDM: Option<String>,

    ///Number of entries with same CCDM
    pub Nsys: Option<usize>,

    ///Number of components in this entry
    pub Ncomp: Option<usize>,

    ///Double and or Multiple Systems flag
    pub MultFlag: Option<String>,

    ///Astrometric source flag
    pub Source: Option<String>,

    ///Solution quality flag
    pub Qual: Option<String>,

    ///Component identifiers
    pub m_HIP: Option<String>,

    ///Position angle between components
    pub theta: Option<usize>,

    ///Angular separation of components
    pub rho: Option<f64>,

    ///Standard error of rho
    pub e_rho: Option<f64>,

    ///Magnitude difference of components
    pub dHp: Option<f64>,

    ///Standard error in dHp
    pub e_dHp: Option<f64>,

    ///Flag indicating a Survey Star
    pub Survey: Option<String>,

    ///Identification Chart
    pub Chart: Option<String>,

    ///Existence of notes
    pub Notes: Option<String>,

    ///HD number <III 135>
    pub HD: Option<usize>,

    ///Bonner DM <I 119>, <I 122>
    pub BD: Option<String>,

    ///Cordoba Durchmusterung (DM) <I 114>
    pub CoD: Option<String>,

    ///Cape Photographic DM <I 108>
    pub CPD: Option<String>,

    ///VI used for reductions
    pub V_I_red: Option<f64>,

    ///Spectral type
    pub SpType: Option<String>,

    ///Source of spectral type
    pub r_SpType: Option<String>,
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
