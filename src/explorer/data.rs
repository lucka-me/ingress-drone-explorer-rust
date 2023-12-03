use super::geography::lla;

#[derive(Clone, serde::Deserialize)]
pub struct Portal {
    pub guid: String,

    #[serde(default)]
    pub title: String,

    #[serde(rename = "lngLat")]
    pub coordinate: lla::Coordinate,
}

impl Ord for Portal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.guid.cmp(&other.guid);
    }
}

impl PartialOrd for Portal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Eq for Portal {

}

impl PartialEq for Portal {
    fn eq(&self, other: &Self) -> bool {
        return self.guid == other.guid
    }
}

impl Portal {
    pub fn with_guid(guid: String) -> Self {
        return Portal { guid, title: String::new(), coordinate: lla::Coordinate::new() };
    }
}

#[derive(serde::Serialize)]
pub struct DrawItem {
    #[serde(rename = "type")]
    shape_type: String,

    color: String,

    #[serde(rename = "latLngs")]
    shape: [ lla::Coordinate; 4 ],
}

impl DrawItem {
    pub fn new(color: &String, shape: [ lla::Coordinate; 4 ]) -> Self {
        return DrawItem { shape_type: String::from("polygon"), color: color.clone(), shape }
    }
}
