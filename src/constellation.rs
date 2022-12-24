/*!
WIP Open Source Constellation Catalog Parser

> NOTE: still under construction!
 */

use crate::catalog::osbsc::OSBSCStar;

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
    use std::collections::HashMap;

    use crate::catalog::ValidParse;
    use crate::constellation::*;
    use crate::parse_catalog;

    #[test]
    #[ignore]
    fn test_constellations_1() {
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

        let mut _star_map = HashMap::new();

        for star in _stars {
            _star_map.insert(star.Hipparcos_id.unwrap(), star);
        }

        let pline1 = Polyline {
            lines: vec![
                _star_map.get(&88).unwrap(),
                _star_map.get(&107).unwrap(),
                _star_map.get(&122).unwrap(),
            ],
        };

        let pline2 = Polyline {
            lines: vec![
                _star_map.get(&145).unwrap(),
                _star_map.get(&194).unwrap(),
                _star_map.get(&418).unwrap(),
            ],
        };
        let _con = Constellation {
            name: Some(String::from("test")),
            lines: vec![pline1, pline2],
        };

        println!("{:?}", _con);
        // panic!()
    }

    #[test]
    #[ignore]
    fn test_constellations_2() {
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

        let mut _star_map = HashMap::new();

        for star in _stars {
            _star_map.insert(star.Hipparcos_id.unwrap(), star);
        }

        let data_file = "data/OSBSC/constellation-lines-hip.utf8";

        if !std::path::Path::new(&data_file).exists() {
            panic!("File \"{}\" doesn't exist. Please run \"get_data.sh\" to fetch the data required for this test.", &data_file)
        };

        let constells = parse_constellation_catalog!(
            Path::new(&data_file),
            _star_map
        );

        println!("Number of Constellations: {}", constells.len());
        println!("{:?}", constells.first());
        // panic!();
    }
}
