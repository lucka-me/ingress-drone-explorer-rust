mod data;
mod explore;
mod load;
mod report;

use super::geography;
type PortalSet = std::collections::BTreeSet<data::Portal>;
type CellPortalsMap = std::collections::BTreeMap<geography::s2::Cell, PortalSet>;

pub mod consts {
    pub const VISIBLE_RADIUS: f64 = 500.0;
    pub const REACHABLE_RADIUS_WITH_KEY: f64 = 500.0;
    pub const SAFE_ROUNDS_FOR_VISIBLE_RADIUS: i32 = (REACHABLE_RADIUS_WITH_KEY / 80.0) as i32 + 1;
}

pub struct Explorer {
    start: geography::lla::Coordinate,
    cells: CellPortalsMap,
    reachable_cells: geography::s2::CellSet,
    cells_containing_keys: CellPortalsMap,
}

impl Explorer {
    pub fn new() -> Self {
        return Explorer {
            start: geography::lla::Coordinate::new(),
            cells: CellPortalsMap::new(),
            reachable_cells: geography::s2::CellSet::new(),
            cells_containing_keys: CellPortalsMap::new()
        }
    }
}

pub fn digits(value: usize) -> usize {
    return (value.checked_ilog10().unwrap_or(0) + 1) as usize;
}