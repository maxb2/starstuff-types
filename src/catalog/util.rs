//! Shared utilities for catalog parsing

/// Trim a string before parsing into a given type.
#[macro_export]
macro_rules! parse_trim {
    // Need to trim because numbers are space padded in catalogs
    (String, $s:expr) => {
        match $s.trim() {
            "" => None,
            _ => Some($s.to_string()),
        }
    };
    ($T:ty, $s:expr) => {
        match $s.trim().parse::<$T>() {
            Ok(inner) => Some(inner),
            Err(_) => None,
        }
    };
}

/// Parse a catalog file into a vector of a given type
#[macro_export]
macro_rules! parse_catalog {
    ($T:ty, $path:expr, $pad:expr) => {
        {

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
        let lines = reader.lines();
        // lines is a instance of some type which implements Iterator<Item=&str>

        let mut stars: Vec<$T> = [].to_vec();

        for l in lines {
            // NOTE: need to pad the line with empty space because it will terminate early with empty fields at the end.
            let s = match $pad {
                Some(n) => format!("{:<width$}", l.unwrap(), width = n),
                None => l.unwrap(),
            };
            let result = <$T>::try_from(s);
            match result {
                Ok(star) => if star.is_valid_parse() { stars.push(star) },
                Err(_) => ()
            };
            // println!("{:?}", star)
        };
        stars
    }
    };
}
