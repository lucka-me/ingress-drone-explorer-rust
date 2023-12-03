use super::lla;

pub struct Coordinate {
    x: f64,
    y: f64,
    z: f64,
}

impl Coordinate {
    pub fn from_lla(coordinate: &lla::Coordinate) -> Self {
        let theta = coordinate.theta();
        let phi = coordinate.phi();
        let cos_phi = f64::cos(phi);
        return Coordinate {
            x: f64::cos(theta) * cos_phi,
            y: f64::sin(theta) * cos_phi,
            z: f64::sin(phi)
        };
    }

    pub fn from_face_s_t(face: u8, s: f64, t: f64) -> Self {
        let mut u: f64 = 1.0 / 3.0;
        if s >= 0.5 {
            u *= 4.0 * s * s - 1.0;
        } else {
            u *= 1.0 - (4.0 * (1.0 - s) * (1.0 - s));
        }
        let mut v: f64 = 1.0 / 3.0;
        if t >= 0.5 {
            v *= 4.0 * t * t - 1.0;
        } else {
            v *= 1.0 - (4.0 * (1.0 - t) * (1.0 - t));
        }
        match face {
        0 => { return Coordinate { x:  1.0, y:    u, z:    v }; }
        1 => { return Coordinate { x:   -u, y:  1.0, z:    v }; }
        2 => { return Coordinate { x:   -u, y:   -v, z:  1.0 }; }
        3 => { return Coordinate { x: -1.0, y:   -v, z:   -u }; }
        4 => { return Coordinate { x:    v, y: -1.0, z:   -u }; }
        5 => { return Coordinate { x:    v, y:    u, z: -1.0 }; }
        _ => { unreachable!("Invalid face") }
        }
    }

    pub fn to_lla(&self) -> lla::Coordinate {
        return lla::Coordinate {
            lng: f64::atan2(self.y, self.x) / std::f64::consts::PI * 180.0,
            lat: f64::atan2(self.z, f64::hypot(self.x, self.y)) / std::f64::consts::PI * 180.0
        };
    }

    pub fn to_face_s_t(&self) -> (u8, f64, f64) {
        let abs_x = self.x.abs();
        let abs_y = self.y.abs();
        let mut face: u8;
        if abs_x > abs_y {
            if abs_x > self.z.abs() {
                face = 0;
            } else {
                face = 2;
            }
        } else {
            if abs_y > self.z.abs() {
                face = 1;
            } else {
                face = 2;
            }
        }
        if (face == 0 && self.x < 0.0) || (face == 1 && self.y < 0.0) || (face == 2 && self.z < 0.0) {
            face += 3;
        }
        let mut s: f64;
        let mut t: f64;
        // (s, t) as (u, v)
        match face {
        0 => { s =  self.y / self.x; t =  self.z / self.x; }
        1 => { s = -self.x / self.y; t =  self.z / self.y; }
        2 => { s = -self.x / self.z; t = -self.y / self.z; }
        3 => { s =  self.z / self.x; t =  self.y / self.x; }
        4 => { s =  self.z / self.y; t = -self.x / self.y; }
        5 => { s = -self.y / self.z; t = -self.x / self.z; }
        _ => { unreachable!("Invalid face") }
        }

        if s >= 0.0 {
            s = 0.5 * f64::sqrt(1.0 + 3.0 * s);
        } else {
            s = 1.0 - 0.5 * f64::sqrt(1.0 - 3.0 * s);
        }
        if t >= 0.0 {
            t = 0.5 * f64::sqrt(1.0 + 3.0 * t);
        } else {
            t = 1.0 - 0.5 * f64::sqrt(1.0 - 3.0 * t);
        }

        return (face, s, t);
    }
}