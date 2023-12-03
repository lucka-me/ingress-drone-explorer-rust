use super::ecef;
use super::lla;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cell {
    face: u8,
    level: u8,
    i: u32,
    j: u32,
}

pub type CellSet = std::collections::BTreeSet<Cell>;

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{},{},{},{}", self.face, self.level, self.i, self.j);
    }
}

impl Cell {
    pub fn from_lla(coordinate: &lla::Coordinate, level: u8) -> Self {
        let (face, s, t) = ecef::Coordinate::from_lla(coordinate).to_face_s_t();
        let max: u32 = 1 << level;
        return Cell {
            face,
            level,
            i: u32::clamp(f64::floor(s * max as f64) as u32, 0, max - 1),
            j: u32::clamp(f64::floor(t * max as f64) as u32, 0, max - 1)
        };
    }

    pub fn wrap(face: u8, i: i32, j: i32, level: u8) -> Self {
        let max: i32 = 1 << level;
        if i >= 0 && j >= 0 && i < max && j < max {
            return Cell { face, level, i: i as u32, j: j as u32}
        }
        let (face, s, t) = ecef::Coordinate::from_face_s_t(
            face, (0.5 + i as f64) / max as f64, (0.5 + j as f64) / max as f64
        ).to_face_s_t();
        return Cell {
            face,
            level,
            i: u32::clamp(f64::floor(s * max as f64) as u32, 0, max as u32 - 1),
            j: u32::clamp(f64::floor(t * max as f64) as u32, 0, max as u32 - 1)
        };
    }

    pub fn intersects_with_cap_of(&self, center: &lla::Coordinate, radius: f64) -> bool {
        let mut corners = self.shape();
        corners.sort_by(|a, b| {
            if center.closer(a, b) {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        });
        return center.distance_to(&corners[0]) < radius
            || center.distance_to_line(&corners[0], &corners[1]) < radius;
    }

    pub fn neighbored_cells_covering_cap_of(&self, center: &lla::Coordinate, radius: f64) -> CellSet {
        let mut result = CellSet::new();
        let mut outside = CellSet::new();
        let mut queue = CellSet::new();
        queue.insert(self.clone());
        while let Some(cell) = queue.pop_first() {
            if result.contains(&cell) || outside.contains(&cell) {
                continue;
            }
            if cell.intersects_with_cap_of(center, radius) {
                queue.extend(cell.neighbors());
                result.insert(cell);
            } else {
                outside.insert(cell);
            }
        }
        return result;
    }

    pub fn neighbored_cells_in(&self, rounds: i32) -> CellSet {
        let mut result = CellSet::new();
        let i = self.i as i32;
        let j = self.j as i32;
        for round in 0 .. rounds {
            let steps = (round + 1) * 2;
            for step in 0 .. steps {
                result.insert(Cell::wrap(self.face, i - round - 1   , j - round + step, self.level));   // Left, upward
                result.insert(Cell::wrap(self.face, i - round + step, j + round + 1   , self.level));   // Top, rightward
                result.insert(Cell::wrap(self.face, i + round + 1   , j + round - step, self.level));   // Right, downward
                result.insert(Cell::wrap(self.face, i + round - step, j - round - 1   , self.level));   // Bottom, leftward
            }
        }
        return result;
    }

    pub fn shape(&self) -> [ lla::Coordinate ; 4 ] {
        return [
            self.lla_at(0.0, 0.0),
            self.lla_at(0.0, 1.0),
            self.lla_at(1.0, 1.0),
            self.lla_at(1.0, 0.0),
        ];
    }

    fn lla_at(&self, d_i: f64, d_j: f64) -> lla::Coordinate {
        let max = (1 << self.level) as f64;
        return ecef::Coordinate::from_face_s_t(
            self.face, (d_i + self.i as f64) / max, (d_j + self.j as f64) / max
        ).to_lla();
    }

    fn neighbors(&self) -> CellSet {
        let mut result = CellSet::new();
        let i = self.i as i32;
        let j = self.j as i32;
        result.insert(Cell::wrap(self.face, i - 1, j, self.level));
        result.insert(Cell::wrap(self.face, i, j - 1, self.level));
        result.insert(Cell::wrap(self.face, i + 1, j, self.level));
        result.insert(Cell::wrap(self.face, i, j + 1, self.level));
        return result;
    }
}