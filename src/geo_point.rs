/// A GeoPoint with latitude, longitude, and altitude
pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
}

impl GeoPoint {
    /// Returns a `GeoPoint` with heading and distance relative to current `GeoPoint`
    /// # Arguments
    /// * `heading` - A number that specifies the relative angle
    /// * `distance` - A number that specifies the relative distance
    pub fn point_from_heading_and_distance(&self, heading: f64, distance: f64) -> GeoPoint {
        const R: f64 = 6378.137; //Radius of the Earth
        let heading_in_rad: f64 = heading.deg_to_rad();
        let lat1: f64 = self.lat.deg_to_rad();
        let lon1: f64 = self.lon.deg_to_rad();
        let mut lat2 = f64::asin(
            f64::sin(lat1) * f64::cos(distance / R)
                + f64::cos(lat1) * f64::sin(distance / R) * f64::cos(heading_in_rad),
        );
        let mut lon2 = lon1
            + f64::atan2(
                f64::sin(heading_in_rad) * f64::sin(distance / R) * f64::cos(lat1),
                f64::cos(distance / R) - f64::sin(lat1) * f64::sin(lat2),
            );
        lat2 = lat2.rad_to_deg();
        lon2 = lon2.rad_to_deg();
        // var point = [lat2, lon2];
        let pfhd = GeoPoint {
            lat: lat2,
            lon: lon2,
            alt: 0.0,
        };
        pfhd
    }

    /// Returns the heading angle of other `GeoPoint` relative to current `GeoPoint`
    /// # Arguments
    /// * `other` - A `GeoPoint` to which the heading is calculated
    pub fn heading_to_point(&self, other: &GeoPoint) -> f64 {
        let phi1: f64 = self.lat.deg_to_rad();
        let phi2: f64 = other.lat.deg_to_rad();
        let lam1: f64 = self.lon.deg_to_rad();
        let lam2: f64 = other.lon.deg_to_rad();

        let x: f64;
        let y: f64;
        x = f64::cos(phi2) * f64::sin(lam2 - lam1);
        //printf("X is %lf\n", x);
        y = f64::cos(phi1) * f64::sin(phi2)
            - f64::sin(phi1) * f64::cos(phi2) * f64::cos(lam2 - lam1);
        //printf("Y is %lf\n", y);
        f64::atan2(x, y).rad_to_deg()
    }
}

trait Radians {
    fn deg_to_rad(self) -> Self;
    fn rad_to_deg(self) -> Self;
}

impl Radians for f64 {
    fn rad_to_deg(self) -> f64 {
        self * 180.0 / std::f64::consts::PI
    }
    fn deg_to_rad(self) -> f64 {
        self * std::f64::consts::PI / 180.0
    }
}

impl Radians for f32 {
    fn rad_to_deg(self) -> f32 {
        self * 180.0 / std::f64::consts::PI as f32
    }
    fn deg_to_rad(self) -> f32 {
        self * std::f64::consts::PI as f32 / 180.0
    }
}
