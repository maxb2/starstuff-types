use starmap_rs::angle::{Angle, Degree};
use starmap_rs::coord::{Equitorial, Geographic, Horizontal, Latitude, Longitude};
use starmap_rs::parse::parse_catalog;
use starmap_rs::star::StarCoordinates;
use starmap_rs::util::*;

fn main() {
    let mut stars = parse_catalog(&String::from("./catalog.json"));

    println!("{:?}", &stars[0]);

    let geo = Geographic {
        latitude: Latitude(Angle::from(Degree(0.0))),
        longitude: Longitude(Angle::from(Degree(0.0))),
    };

    let sidereal_time: GMST = GMST::from(JulianDate::from(Local.ymd(2000, 1, 1).and_hms(12, 0, 0)));

    for star in &mut stars {
        let horiz_coord = StarCoordinates::Stereo(
            Horizontal::from_equitorial(&Equitorial::from(star.coordinates), &geo, &sidereal_time)
                .stereo_project(),
        );
        star.coordinates = horiz_coord;
    }

    println!("{:?}", stars[0]);
}
