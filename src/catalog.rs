pub mod hipparcos;
pub mod osbsc;
pub mod util;
pub mod yale;

pub trait ValidParse {
    fn is_valid_parse(&self) -> bool;
}
