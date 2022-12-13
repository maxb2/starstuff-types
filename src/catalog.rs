//! Star catalog parsers

pub mod hipparcos;
pub mod osbsc;
pub mod util;
pub mod yale;

/// Determines if a parsed record is valid. User can implement this to require certain fields or conditions.
pub trait ValidParse {
    fn is_valid_parse(&self) -> bool;
}
