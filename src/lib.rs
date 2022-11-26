use crate::angle::*;
pub mod angle;
pub mod coord;
pub mod parse;
pub mod parse_yale;
pub mod star;
pub mod util;

pub fn stuff() {
    let _rad: RadData = RadData(1.0);
    let _hr: HourData = HourData(1.0);
    let _deg: DegData = DegData(1.0);
}

#[cfg(test)]
mod tests {}
