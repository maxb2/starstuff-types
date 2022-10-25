use crate::coord::{Cartesian, Equitorial, Horizontal, Polar};

#[derive(Debug)]
pub enum StarCoordinates {
    Cartesian(Cartesian),
    Equitorial(Equitorial),
    Horizontal(Horizontal),
    Stereo(Polar),
}

#[derive(Debug)]
pub struct Star {
    pub coordinates: StarCoordinates,
    pub v_mag: f64,
    pub name: String,
    pub harvard: u32,
}
