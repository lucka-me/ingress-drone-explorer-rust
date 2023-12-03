use super::consts;
use super::geography;

impl super::Explorer {
    pub fn explore_from(&mut self, start: geography::lla::Coordinate) {
        self.start = start;
        let start_time = std::time::SystemTime::now();
        let start_cell = geography::s2::Cell::from_lla(&self.start, geography::consts::S2_DEFAULT_LEVEL);
        println!("⏳ Explore from {} in cell #{start_cell}", self.start);
        let mut queue = geography::s2::CellSet::new();

        if self.cells.contains_key(&start_cell) {
            queue.insert(start_cell);
        } else {
            queue = start_cell.neighbored_cells_covering_cap_of(&self.start, consts::VISIBLE_RADIUS);
            queue.retain(|&a| self.cells.contains_key(&a));
        }
        self.cells_containing_keys.retain(|&key, _| queue.contains(&key));

        let mut previous_time = start_time;
        let progress_digits = super::digits(self.cells.len());

        let empty_portals = super::PortalSet::new();

        while let Some(cell) = queue.pop_first() {
            let portals = self.cells.get(&cell).unwrap_or(&empty_portals);
            if portals.is_empty() {
                continue;
            }
            self.reachable_cells.insert(cell);

            // Get all neighbors in the visible range (also the possible ones), filter the empty/pending/reached ones and
            // search for reachable ones
            let neighbors = cell.neighbored_cells_in(consts::SAFE_ROUNDS_FOR_VISIBLE_RADIUS);
            for neighbor in neighbors {
                if queue.contains(&neighbor)
                    || self.reachable_cells.contains(&neighbor)
                    || !self.cells.contains_key(&neighbor) {
                    continue;
                }
                for portal in portals {
                    if neighbor.intersects_with_cap_of(&portal.coordinate, consts::VISIBLE_RADIUS) {
                        queue.insert(neighbor);
                        break;
                    }
                }
            }

            // Find keys
            // TODO: Consider to use cell.neighbored_cells_in instead?
            if !self.cells_containing_keys.is_empty() {
                for portal in portals {
                    self.cells_containing_keys.retain(|key, value| {
                        let mut should_retain = true;
                        if queue.contains(key) {
                            should_retain = false;
                        } else {
                            if value.iter()
                                .any(|item| {
                                    return portal.coordinate.distance_to(&item.coordinate)
                                        < consts::REACHABLE_RADIUS_WITH_KEY
                                }) {
                                    queue.insert(*key);
                                    should_retain = false;
                            }
                            
                        }
                        return should_retain;
                    });
                    if self.cells_containing_keys.is_empty() {
                        break;
                    }
                }
            }

            let now = std::time::SystemTime::now();
            if now.duration_since(previous_time).expect("Time machine?").as_millis() > 1000 {
                println!("⏳ Reached {:>progress_digits$} / {} cell(s)", self.reachable_cells.len(), self.cells.len());
                previous_time = now;
            }
        }

        let duration = std::time::SystemTime::now().duration_since(start_time)
            .expect("Time machine?");
        println!("🔍 Exploration finished after {} seconds", duration.as_secs_f64());
    }
}