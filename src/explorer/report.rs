
impl super::Explorer {
    pub fn report(&self) {
        let mut portals_count: usize = 0;
        let mut reachable_portals_count: usize = 0;
        let mut furthest_portal = super::data::Portal {
            guid: String::new(), title: String::new(), coordinate: self.start.clone()
        };
        for (cell ,portals) in &self.cells {
            portals_count += portals.len();
            if !self.reachable_cells.contains(&cell) {
                continue;
            }
            reachable_portals_count += portals.len();
            for portal in portals {
                if self.start.closer(&furthest_portal.coordinate, &portal.coordinate) {
                    furthest_portal = portal.clone();
                }
            }
        }
        if reachable_portals_count == 0 {
            println!("⛔️ There is no reachable portal in {portals_count} portal(s) from {}", self.start);
            return;
        }
        let total_number_digits = super::digits(portals_count);
        let reachable_number_digits = super::digits(reachable_portals_count);
        let unreachable_number_digits = super::digits(portals_count - reachable_portals_count);
        println!(
            "⬜️ In {:>total_number_digits$}   cell(s), \
            {:>reachable_number_digits$} are ✅ reachable, \
            {:>unreachable_number_digits$}",
            self.cells.len(), self.reachable_cells.len(), self.cells.len() - self.reachable_cells.len()
        );
        println!(
            "📍 In {portals_count:>total_number_digits$} Portal(s), \
            {reachable_portals_count:>reachable_number_digits$} are ✅ reachable, \
            {:>unreachable_number_digits$}",
            portals_count - reachable_portals_count
        );
        println!("🛬 The furthest Portal is {}.", furthest_portal.title);
        println!("  📍 It's located at {}", furthest_portal.coordinate);
        println!("  📏 Where is {} km away", self.start.distance_to(&furthest_portal.coordinate) / 1000.0);
        println!(
            "  🔗 Check it out: https://intel.ingress.com/?pll={},{}",
            furthest_portal.coordinate.lat, furthest_portal.coordinate.lng
        );
    }

    pub fn save_drawn_items_to(&self, path: &std::path::PathBuf) {
        let file = std::fs::File::create(&path)
            .expect("Unable to create the key list file.");
        let writer = std::io::BufWriter::new(file);
        let reachable_color = String::from("#783cbd");
        let unreachable_color = String::from("#404040");
        let items: Vec<super::data::DrawItem> = self.cells.iter().map(|(key, _)| {
            let shape = key.shape();
            if self.reachable_cells.contains(key) {
                return super::data::DrawItem::new(&reachable_color, shape);
            } else {
                return super::data::DrawItem::new(&unreachable_color, shape);
            }
        }).collect();
        serde_json::to_writer_pretty(writer, &items).expect("Unable to encode draw items to JSON.");
        println!("💾 Saved drawn items to {}", path.display());
    }
}