// #[path = "geo_point.rs"]
use rand::random;
mod geo_point;
use geo_point::GeoPoint;

fn main() {
    // println!("Hello, world!");
    // println!("Point example");

    let mut h: GeoPoint;
    let mut k: f64;
    let p1 = GeoPoint {
        lat: 29.0,
        lon: 52.0,
        alt: 0.0,
    };
    for _i in 1..10 {
        let he = random::<f64>() * 360.0;
        let d = random::<f64>() * 10000.0;
        h = p1.point_from_heading_and_distance(he, d);
        k = p1.heading_to_point(&h);
        println!("lat: {} \nlon: {} \nh1: {} \nh2: {}\n", h.lat, h.lon, he, k);
    }
}
