// Parse Yale Bright Star Catalog
// http://tdc-www.harvard.edu/catalogs/bsc5.html
// NOTE: put the `catalog` file at the root of this repository for the tests to pass.

use std::option::Option;
use std::str::FromStr;

fn parse_optional_field<T>(s: String) -> Option<T>
where
    T: FromStr,
{
    match s.parse::<T>() {
        Ok(u) => Some(u),
        Err(_) => None,
    }
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
            HR: s[0..4].trim().parse().unwrap(),
            Name: s[4..14].parse().unwrap(),
            DM: s[14..25].parse().unwrap(),
            HD: parse_optional_field::<usize>(s[25..31].trim().to_string()),
            SAO: parse_optional_field::<usize>(s[31..37].trim().to_string()),
            FK5: parse_optional_field::<usize>(s[37..41].to_string()),
            IRflag: s[41..42].parse().unwrap(),
            r_IRflag: s[42..43].parse().unwrap(),
            Multiple: s[43..44].parse().unwrap(),
            ADS: s[44..49].parse().unwrap(),
            ADScomp: s[49..51].parse().unwrap(),
            VarID: s[51..60].parse().unwrap(),
            RAh1900: parse_optional_field::<usize>(s[60..62].trim().to_string()),
            RAm1900: parse_optional_field::<usize>(s[62..64].trim().to_string()),
            RAs1900: parse_optional_field::<f64>(s[64..68].trim().to_string()),
            DE_1900: s[68..69].parse().unwrap(),
            DEd1900: parse_optional_field::<usize>(s[69..71].trim().to_string()),
            DEm1900: parse_optional_field::<usize>(s[71..73].trim().to_string()),
            DEs1900: parse_optional_field::<usize>(s[73..75].trim().to_string()),
            RAh: parse_optional_field::<usize>(s[75..77].trim().to_string()),
            RAm: parse_optional_field::<usize>(s[77..79].trim().to_string()),
            RAs: parse_optional_field::<f64>(s[79..83].trim().to_string()),
            DE_: s[83..84].parse().unwrap(),
            DEd: parse_optional_field::<usize>(s[84..86].trim().to_string()),
            DEm: parse_optional_field::<usize>(s[86..88].trim().to_string()),
            DEs: parse_optional_field::<usize>(s[88..90].trim().to_string()),
            GLON: parse_optional_field::<f64>(s[90..96].trim().to_string()),
            GLAT: parse_optional_field::<f64>(s[96..102].trim().to_string()),
            Vmag: parse_optional_field::<f64>(s[102..107].trim().to_string()),
            n_Vmag: s[107..108].parse().unwrap(),
            u_Vmag: s[108..109].parse().unwrap(),
            B_V: parse_optional_field::<f64>(s[109..114].trim().to_string()),
            u_B_V: s[114..115].parse().unwrap(),
            U_B: parse_optional_field::<f64>(s[115..120].trim().to_string()),
            u_U_B: s[120..121].parse().unwrap(),
            R_I: parse_optional_field::<f64>(s[121..126].trim().to_string()),
            n_R_I: s[126..127].parse().unwrap(),
            SpType: s[127..147].parse().unwrap(),
            n_SpType: s[147..148].parse().unwrap(),
            pmRA: parse_optional_field::<f64>(s[148..154].trim().to_string()),
            pmDe: parse_optional_field::<f64>(s[154..160].trim().to_string()),
            n_Parallax: s[160..161].parse().unwrap(),
            Parallax: parse_optional_field::<f64>(s[161..166].trim().to_string()),
            RadVel: parse_optional_field::<usize>(s[166..170].trim().to_string()),
            n_RadVel: s[170..174].parse().unwrap(),
            l_RotVel: s[174..176].parse().unwrap(),
            RotVel: parse_optional_field::<usize>(s[176..179].trim().to_string()),
            u_RotVel: s[179..180].parse().unwrap(),
            Dmag: parse_optional_field::<f64>(s[180..184].trim().to_string()),
            Sep: parse_optional_field::<f64>(s[184..190].trim().to_string()),
            MultID: s[190..194].parse().unwrap(),
            MultCnt: parse_optional_field::<usize>(s[194..196].trim().to_string()),
            NoteFlag: s[196..197].parse().unwrap(),
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
