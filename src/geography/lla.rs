use super::consts;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Coordinate {
    pub lng: f64,
    pub lat: f64,
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{},{}", self.lng, self.lat);
    }
}

impl Coordinate {
    pub fn new() -> Self {
        return Coordinate { lng: 0.0, lat: 0.0 }
    }

    pub fn theta(&self) -> f64 {
        return self.lng * std::f64::consts::PI / 180.0;
    }

    pub fn phi(&self) -> f64 {
        return self.lat * std::f64::consts::PI / 180.0;
    }

    pub fn distance_to(&self, other: &Self) -> f64 {
        let sin_theta = f64::sin((other.theta() - self.theta()) / 2.0);
        let sin_phi = f64::sin((other.phi() - self.phi()) / 2.0);
        let a = sin_phi * sin_phi + sin_theta * sin_theta * f64::cos(self.phi()) * f64::cos(other.phi());
        return f64::atan2(a.sqrt(), 1.0 - a) * 2.0 * consts::EARTH_RADIUS;
    }

    pub fn distance_to_line(&self, a: &Self, b: &Self) -> f64 {
        let c_1 = (b.lat - a.lat) * (self.lat - a.lat) + (b.lng - a.lng) * (self.lng - a.lng);
        if c_1 <= 0.0 {
            return self.distance_to(a);
        }
        let c_2 = (b.lat - a.lat) * (b.lat - a.lat) + (b.lng - a.lng) * (b.lng - a.lng);
        if c_2 <= c_1 {
            return self.distance_to(b);
        }
        let ratio = c_1 / c_2;
        return self.distance_to(
            &Coordinate { lng: a.lng + ratio * (b.lng - a.lng), lat: a.lat + ratio * (b.lat - a.lat) }
        );
    }

    pub fn closer(&self, a: &Self, b: &Self) -> bool {
        let d_a = (self.lng - a.lng) * (self.lng - a.lng) + (self.lat - a.lat) * (self.lat - a.lat);
        let d_b = (self.lng - b.lng) * (self.lng - b.lng) + (self.lat - b.lat) * (self.lat - b.lat);
        return d_a < d_b;
    }
}