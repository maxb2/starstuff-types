use crate::angle::*;
pub mod angle;
pub mod coord;
pub mod parse;
pub mod parse_hip;
pub mod parse_yale;
pub mod star;
pub mod util;

pub fn stuff() {
    let _rad: Radian = Radian(1.0);
    let _hr: Hour = Hour(1.0);
    let _deg: Degree = Degree(1.0);
}

#[cfg(test)]
mod tests {}
