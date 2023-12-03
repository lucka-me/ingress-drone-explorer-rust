use super::geography;

use super::data;

impl super::Explorer {
    pub fn load_portals(&mut self, paths: &Vec<std::path::PathBuf>) {
        let start_time = std::time::SystemTime::now();
        println!("⏳ Loading Portals...");
        let mut portal_count: usize = 0;
        for path in paths {
            let file = std::fs::File::open(&path)
                .expect("Unable to open the file.");
            let reader = std::io::BufReader::new(file);
            let portals: Vec<data::Portal> = serde_json::from_reader(reader)
                .expect("Unable to parse portals.");

            let mut file_add_portal_count: usize = 0;
            let mut file_add_cell_count: usize = 0;
            for portal in portals {
                let cell = geography::s2::Cell::from_lla(&portal.coordinate, geography::consts::S2_DEFAULT_LEVEL);
                if let Some(existing_cell) = self.cells.get_mut(&cell) {
                    if existing_cell.contains(&portal) {
                        if !portal.title.is_empty() {
                            existing_cell.replace(portal);
                        }
                    } else {
                        existing_cell.insert(portal);
                        file_add_portal_count += 1;
                    }
                } else {
                    let mut value = super::PortalSet::new();
                    value.insert(portal);
                    self.cells.insert(cell, value);
                    file_add_cell_count += 1;
                    file_add_portal_count += 1;
                }
            }
            portal_count += file_add_portal_count;
            println!(
                "  📃 Added {:>5} portal(s) and {:>4} cell(s) from {}",
                file_add_portal_count, file_add_cell_count, path.display()
            );
        }
        let duration = std::time::SystemTime::now().duration_since(start_time)
            .expect("Time machine?.");
        println!(
            "📍 Loaded {} Portal(s) in {} cell(s) from {} file(s), which took {} seconds",
            portal_count, self.cells.len(), paths.len(), duration.as_secs_f64()
        );
    }

    pub fn load_keys(&mut self, path: &std::path::PathBuf) {
        println!("⏳ Loading Keys from {}...", path.display());
        let file = std::fs::File::open(&path)
            .expect("Unable to open the key list file.");
        let reader = std::io::BufReader::new(file);
        let guids: Vec<String> = serde_json::from_reader(reader)
            .expect("Unable to parse keys.");
        let mut keys = super::PortalSet::new();
        for guid in guids {
            keys.insert(data::Portal::with_guid(guid));
        }
        let load_count = keys.len();
        for cell in &self.cells {
            let keys_in_cell: super::PortalSet = keys.intersection(&cell.1).cloned().collect();
            if keys_in_cell.is_empty() {
                continue;
            }
            let mut left_keys: super::PortalSet = keys.difference(&keys_in_cell).cloned().collect();
            std::mem::swap(&mut keys, &mut left_keys);
            self.cells_containing_keys.insert(cell.0.clone(), keys_in_cell);
        }
        println!(
            "🔑 Loaded {} Key(s) and matched {} in {} cell(s)",
            load_count, load_count - keys.len(), self.cells_containing_keys.len()
        );
    }
}