use starmap_rs::parse::parse_catalog;
use starmap_rs::util::*;

fn main() {
    println!("Hello, world!");
    println!(
        "{:?}",
        JulianDate::from(Local.ymd(2000, 1, 1).and_hms(12, 0, 0))
    );

    let stars = parse_catalog(&String::from("./catalog.json"));

    println!("{:?}", stars[0])
}
