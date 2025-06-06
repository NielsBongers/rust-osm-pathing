use core::fmt;
use std::collections::HashMap;

use geo::Point;

use crate::osm_parsing::{Node, Way};

impl Node {
    pub fn new(node_hashmap: HashMap<String, String>) -> Node {
        let tags: std::collections::HashMap<String, String> = HashMap::new();

        let nodes: Vec<u64> = Vec::<u64>::new();
        let ways: Vec<u64> = Vec::<u64>::new();

        let id = node_hashmap
            .get("id")
            .unwrap()
            .parse::<u64>()
            .expect("Failed to parse id");
        let lat = node_hashmap
            .get("lat")
            .unwrap()
            .parse::<f64>()
            .expect("Failed to parse latitude");
        let lon = node_hashmap
            .get("lon")
            .unwrap()
            .parse::<f64>()
            .expect("Failed to parse longitude");

        let coordinate = Point::new(lon, lat);

        Node {
            id,
            coordinate,
            tags,
            ways,
            nodes,
        }
    }

    pub fn map_link(&self) -> String {
        let base_url = "https://www.openstreetmap.org/node/".to_string();
        let node_id = self.id.to_string();
        base_url + &node_id
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tags: Vec<String> = Vec::new();

        for (key, value) in self.tags.iter() {
            let temp_tag: String = format!("{}: {}", key, value);
            tags.push(temp_tag);
        }

        let tag_string = tags.join("\n");

        write!(
            f,
            "Node ID: {}, Coordinates: {},{}. Tags:\n{}",
            self.id,
            self.coordinate.x(),
            self.coordinate.y(),
            tag_string
        )
    }
}

impl Way {
    pub fn new(way_hashmap: HashMap<String, String>) -> Way {
        let node_ids: Vec<u64> = Vec::<u64>::new();
        let tags: HashMap<String, String> = HashMap::new();

        let id = way_hashmap
            .get("id")
            .unwrap()
            .parse::<u64>()
            .expect("Failed to parse id");

        Way { id, node_ids, tags }
    }

    pub fn map_link(&self) -> String {
        let base_url = "https://www.openstreetmap.org/way/".to_string();
        let node_id = self.id.to_string();
        base_url + &node_id
    }
}
