// Parse Yale Bright Star Catalog
// http://tdc-www.harvard.edu/catalogs/bsc5.html
// NOTE: put the `catalog` file at the root of this repository for the tests to pass.

macro_rules! parse_yale_field {
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
}

#[allow(non_snake_case, dead_code)]
#[derive(Debug)]
struct YaleStar {
    HR: usize,
    Name: String,
    DM: String,
    HD: Option<usize>,
    SAO: Option<usize>,
    FK5: Option<usize>,
    IRflag: String,
    r_IRflag: String,
    Multiple: String,
    ADS: String,
    ADScomp: String,
    VarID: String,
    RAh1900: Option<usize>,
    RAm1900: Option<usize>,
    RAs1900: Option<f64>,
    DE_1900: String,
    DEd1900: Option<usize>,
    DEm1900: Option<usize>,
    DEs1900: Option<usize>,
    RAh: Option<usize>,
    RAm: Option<usize>,
    RAs: Option<f64>,
    DE_: String,
    DEd: Option<usize>,
    DEm: Option<usize>,
    DEs: Option<usize>,
    GLON: Option<f64>,
    GLAT: Option<f64>,
    Vmag: Option<f64>,
    n_Vmag: String,
    u_Vmag: String,
    B_V: Option<f64>,
    u_B_V: String,
    U_B: Option<f64>,
    u_U_B: String,
    R_I: Option<f64>,
    n_R_I: String,
    SpType: String,
    n_SpType: String,
    pmRA: Option<f64>,
    pmDe: Option<f64>,
    n_Parallax: String,
    Parallax: Option<f64>,
    RadVel: Option<usize>,
    n_RadVel: String,
    l_RotVel: String,
    RotVel: Option<usize>,
    u_RotVel: String,
    Dmag: Option<f64>,
    Sep: Option<f64>,
    MultID: String,
    MultCnt: Option<usize>,
    NoteFlag: String,
}

impl From<String> for YaleStar {
    fn from(s: String) -> YaleStar {
        let ss = &s[37..41];
        println!("{}", ss.trim());
        YaleStar {
            HR: parse_yale_field!(usize s[0..4]).unwrap(),
            Name: parse_yale_field!(String s[4..14]),
            DM: parse_yale_field!(String s[14..25]),
            HD: parse_yale_field!(usize s[25..31]),
            SAO: parse_yale_field!(usize s[31..37]),
            FK5: parse_yale_field!(usize s[37..41]),
            IRflag: parse_yale_field!(String s[41..42]),
            r_IRflag: parse_yale_field!(String s[42..43]),
            Multiple: parse_yale_field!(String s[43..44]),
            ADS: parse_yale_field!(String s[44..49]),
            ADScomp: parse_yale_field!(String s[49..51]),
            VarID: parse_yale_field!(String s[51..60]),
            RAh1900: parse_yale_field!(usize s[60..62]),
            RAm1900: parse_yale_field!(usize s[62..64]),
            RAs1900: parse_yale_field!(f64 s[64..68]),
            DE_1900: parse_yale_field!(String s[68..69]),
            DEd1900: parse_yale_field!(usize s[69..71]),
            DEm1900: parse_yale_field!(usize s[71..73]),
            DEs1900: parse_yale_field!(usize s[73..75]),
            RAh: parse_yale_field!(usize s[75..77]),
            RAm: parse_yale_field!(usize s[77..79]),
            RAs: parse_yale_field!(f64 s[79..83]),
            DE_: parse_yale_field!(String s[83..84]),
            DEd: parse_yale_field!(usize s[84..86]),
            DEm: parse_yale_field!(usize s[86..88]),
            DEs: parse_yale_field!(usize s[88..90]),
            GLON: parse_yale_field!(f64 s[90..96]),
            GLAT: parse_yale_field!(f64 s[96..102]),
            Vmag: parse_yale_field!(f64 s[102..107]),
            n_Vmag: parse_yale_field!(String s[107..108]),
            u_Vmag: parse_yale_field!(String s[108..109]),
            B_V: parse_yale_field!(f64 s[109..114]),
            u_B_V: parse_yale_field!(String s[114..115]),
            U_B: parse_yale_field!(f64 s[115..120]),
            u_U_B: parse_yale_field!(String s[120..121]),
            R_I: parse_yale_field!(f64 s[121..126]),
            n_R_I: parse_yale_field!(String s[126..127]),
            SpType: parse_yale_field!(String s[127..147]),
            n_SpType: parse_yale_field!(String s[147..148]),
            pmRA: parse_yale_field!(f64 s[148..154]),
            pmDe: parse_yale_field!(f64 s[154..160]),
            n_Parallax: parse_yale_field!(String s[160..161]),
            Parallax: parse_yale_field!(f64 s[161..166]),
            RadVel: parse_yale_field!(usize s[166..170]),
            n_RadVel: parse_yale_field!(String s[170..174]),
            l_RotVel: parse_yale_field!(String s[174..176]),
            RotVel: parse_yale_field!(usize s[176..179]),
            u_RotVel: parse_yale_field!(String s[179..180]),
            Dmag: parse_yale_field!(f64 s[180..184]),
            Sep: parse_yale_field!(f64 s[184..190]),
            MultID: parse_yale_field!(String s[190..194]),
            MultCnt: parse_yale_field!(usize s[194..196]),
            NoteFlag: parse_yale_field!(String s[196..197]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_yale::*;
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;

    #[test]
    fn test_yaleparse() {
        let s = String::from("   1          BD+44 4550      3 36042          46           000001.1+444022000509.9+451345114.44-16.88 6.70  +0.07 +0.08         A1Vn               -0.012-0.018      -018      195  4.2  21.6AC   3 ");
        println!("{}", s);
        let star = YaleStar::from(s);
        println!("{:?}", star);
        // panic!()
    }

    #[test]
    fn test_catalog() {
        // Create a path to the desired file
        let path = Path::new("catalog");
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
            let s = format!("{:<197}", l.unwrap());
            let _star = YaleStar::from(s);
            // println!("{:?}", star)
        }
        // panic!()
    }
}
