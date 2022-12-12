use starstuff_types::angle::Angle;
use starstuff_types::coord::{
    Declination, Equitorial, Geographic, Horizontal, Latitude, Longitude, RightAscension,
};
use starstuff_types::star::{Star, StarCoordinates};
use starstuff_types::time::*;

fn main() {
    let mut stars: Vec<Star> = vec![Star {
        coordinates: StarCoordinates::Equitorial(Equitorial {
            right_ascension: RightAscension(Angle::Degree(10.0)),
            declination: Declination(Angle::Degree(20.0)),
        }),
        v_mag: 0.0,
        harvard: 1,
        name: String::from("dummy"),
    }];

    println!("{:?}", &stars[0]);

    let geo = Geographic {
        latitude: Latitude(Angle::Degree(0.0)),
        longitude: Longitude(Angle::Degree(0.0)),
    };

    let sidereal_time: GMST = GMST::from(JulianDate::from(
        Local.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap(),
    ));

    for star in &mut stars {
        let horiz_coord = StarCoordinates::Stereo(
            Horizontal::from_equitorial(&Equitorial::from(star.coordinates), &geo, &sidereal_time)
                .stereo_project(),
        );
        star.coordinates = horiz_coord;
    }

    println!("{:?}", stars[0]);
}
