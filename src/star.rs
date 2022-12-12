use crate::coord::{Cartesian, Equitorial, Horizontal, Polar};

// TODO: simplify this enum like crate::angle::Angle
#[derive(Debug, Copy, Clone)]
pub enum StarCoordinates {
    Cartesian(Cartesian),
    Equitorial(Equitorial),
    Horizontal(Horizontal),
    Stereo(Polar),
}

// TODO: move this to a different crate and implement `from` traits for the catalog star structs in this crate.
#[derive(Debug, Clone)]
pub struct Star {
    pub coordinates: StarCoordinates,
    pub v_mag: f64,
    pub name: String,
    pub harvard: u32,
}

impl From<StarCoordinates> for Cartesian {
    fn from(coord: StarCoordinates) -> Self {
        match coord {
            StarCoordinates::Cartesian(coord) => coord,
            _ => panic!("Could not convert StarCoordinates to Cartesian!"),
        }
    }
}

impl From<StarCoordinates> for Equitorial {
    fn from(coord: StarCoordinates) -> Self {
        match coord {
            StarCoordinates::Equitorial(coord) => coord,
            _ => panic!("Could not convert StarCoordinates to Equitorial!"),
        }
    }
}

impl From<StarCoordinates> for Horizontal {
    fn from(coord: StarCoordinates) -> Self {
        match coord {
            StarCoordinates::Horizontal(coord) => coord,
            _ => panic!("Could not convert StarCoordinates to Horizontal!"),
        }
    }
}

impl From<StarCoordinates> for Polar {
    fn from(coord: StarCoordinates) -> Self {
        match coord {
            StarCoordinates::Stereo(coord) => coord,
            _ => panic!("Could not convert StarCoordinates to Polar!"),
        }
    }
}
