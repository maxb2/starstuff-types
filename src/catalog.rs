//! Star catalog parsers

pub mod constellation;
pub mod hipparcos;
pub mod osbsc;
pub mod parse;
pub mod util;
pub mod yale;

/// Determines if a parsed record is valid. User can implement this to require certain fields or conditions.
pub trait ValidParse {
    fn is_valid_parse(&self) -> bool;
}
