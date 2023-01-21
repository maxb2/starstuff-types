/*!
WIP Open Source Constellation Catalog Parser

> NOTE: still under construction!
 */

use crate::catalog::osbsc::OSBSCStar;

use crate::catalog::parse::{identifier, int_list, ws};

use nom::combinator::all_consuming;
use nom::{character::complete::char, multi::separated_list0, IResult};

/// Polyline
#[allow(dead_code)] // FIXME
#[derive(Debug, Clone)]
pub struct Polyline<T> {
    pub lines: Vec<T>,
}

/// Constellation
#[allow(dead_code)] // FIXME
#[derive(Debug, Clone)]
pub struct Constellation<'a> {
    pub name: Option<String>,
    pub lines: Vec<Polyline<&'a OSBSCStar>>,
}

/// Parsed constellation
#[allow(dead_code)] // FIXME
#[derive(Debug)]
pub struct ParsedConstellation<'a> {
    name: String,
    lines: Vec<Vec<&'a str>>,
}

/// Parse a record from constellation data file.
pub fn parse_record(input: &str) -> IResult<&str, ParsedConstellation> {
    let (input, name) = ws(identifier)(input)?;
    let (input, _) = ws(char('='))(input)?;
    let (input, lines) = all_consuming(separated_list0(ws(char(';')), int_list))(input)?;

    Ok((
        input,
        ParsedConstellation {
            name: String::from(name),
            lines,
        },
    ))
}

/// WIP Parse Open Source Constellation Catalog
#[macro_export]
macro_rules! parse_constellation_catalog {
    ($path:expr, $stars:expr) => {{
        use std::error::Error;
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::BufReader;
        use std::path::Path;

        let display = $path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let file = match File::open(&$path) {
            // The `description` method of `io::Error` returns a string that describes the error
            Err(why) => panic!(
                "couldn't open {}: {}",
                display,
                <dyn Error>::to_string(&why)
            ),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        let file_lines = reader.lines();
        // lines is a instance of some type which implements Iterator<Item=&str>

        let mut constellations: Vec<Constellation> = [].to_vec();

        for l in file_lines {
            let mut poly_lines: Vec<Polyline<&OSBSCStar>> = vec![];
            let stuff = l.unwrap();
            let name_lines: Vec<&str> = stuff.split('=').collect();
            let line_strings = name_lines[1].trim().split(';');
            for line in line_strings {
                let mut pline: Vec<&OSBSCStar> = vec![];
                let ids = line.replace(&['[', ']'][..], "");
                for id in ids.split(',') {
                    pline.push(&$stars.get(&id.trim().parse::<usize>().unwrap()).unwrap());
                }
                poly_lines.push(Polyline { lines: pline });
            }
            constellations.push(Constellation {
                name: Some(String::from(name_lines[0].trim())),
                lines: poly_lines,
            })
        }
        constellations
    }};
}

#[cfg(test)]
mod tests {
    use crate::catalog::constellation::*;

    #[test]
    fn test_parser() {
        let rec = "Psc = [123, 32131, 321321]; [31312, 54654645]";
        let r = parse_record(&rec);
        println!("{:#?}", r);
        r.unwrap();
    }
}
