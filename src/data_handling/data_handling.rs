use log::info;

use std::path::Path;

use crate::utils::file_handling::{load_hashmaps, save_hashmaps};
use crate::{data_handling::OSMData, osm_parsing::osm_parsing::parse_xml};

use rayon::prelude::*;
use std::sync::RwLock;

impl OSMData {
    pub fn new(file_path: &Path) -> Self {
        let (node_map, way_map, node_subsets) =
            match file_path.extension().and_then(|ext| ext.to_str()) {
                Some("osm") => parse_xml(file_path),
                Some("hashmap") => load_hashmaps(file_path),
                _ => panic!("Unsupported file extension or file doesn't exist."),
            };

        OSMData {
            node_map,
            way_map,
            node_subsets,
        }
    }

    pub fn update_road_nodes(&mut self) {
        // Deleting all the way entries in the nodes.
        for node in self.node_map.values_mut() {
            node.ways.clear();
        }

        // Locking the map, then iterating over it.
        // For every way, we push the way id to all the nodes it contains.
        let node_map = RwLock::new(&mut self.node_map);
        self.way_map.par_iter_mut().for_each(|(way_id, way)| {
            for node_id in &way.node_ids {
                if let Some(node) = node_map.write().unwrap().get_mut(node_id) {
                    node.ways.push(*way_id);
                }
            }
        });

        // We loop over all the ways, and add the adjacent nodes to those nodes.
        for (_, way) in self.way_map.iter() {
            for node_ids in way.node_ids.windows(2) {
                let left_node_id = node_ids[0];
                let right_node_id = node_ids[1];

                self.node_map
                    .get_mut(&left_node_id)
                    .expect("Failed to read left node")
                    .nodes
                    .push(right_node_id);
                self.node_map
                    .get_mut(&right_node_id)
                    .expect("Failed to read right node")
                    .nodes
                    .push(left_node_id);
            }
        }
    }

    pub fn node_max_speed(&self, node_id: u64) -> Option<f64> {
        if let Some(node) = self.node_map.get(&node_id) {
            for way_id in node.ways.iter() {
                if let Some(way) = self.way_map.get(way_id) {
                    if let Some(max_speed) = way.tags.get("maxspeed") {
                        if let Ok(max_speed) = max_speed.parse::<f64>() {
                            return Some(max_speed);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn list_ways(&self) {
        for (_, way) in self.way_map.iter() {
            info!("{:?}\n{:?}\n\n", way, way.map_link());
        }
    }

    pub fn list_nodes(&self) {
        for (_, node) in self.node_map.iter() {
            info!("{:?}\n{:?}\n\n", node, node.map_link());
        }
    }

    pub fn save_hashmaps(&self, file_path: &Path) {
        save_hashmaps(file_path, &self.node_map, &self.way_map, &self.node_subsets);
    }

    pub fn load_hashmaps(&mut self, file_path: &Path) {
        let (node_map, way_map, node_subsets) = load_hashmaps(file_path);

        self.node_map = node_map;
        self.way_map = way_map;
        self.node_subsets = node_subsets;
    }
}
