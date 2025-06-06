use bincode;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use crate::data_handling::{FilterSet, NodeSubset, OSMData};
use crate::osm_parsing::{Node, Way};

pub fn save_hashmaps(
    file_path: &Path,
    node_map: &HashMap<u64, Node>,
    way_map: &HashMap<u64, Way>,
    node_subsets: &Vec<NodeSubset>,
) {
    let file = File::create(file_path).expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    bincode::serialize_into(&mut writer, node_map).expect("Failed to serialize node_map");
    bincode::serialize_into(&mut writer, way_map).expect("Failed to serialize way_map");
    bincode::serialize_into(&mut writer, node_subsets).expect("Failed to serialize node_subsets");
    writer.flush().expect("Failed to flush writer");
}

pub fn load_hashmaps(file_path: &Path) -> (HashMap<u64, Node>, HashMap<u64, Way>, Vec<NodeSubset>) {
    let file = File::open(file_path).expect("Failed to open file");
    let mut reader = BufReader::new(file);

    let node_map: HashMap<u64, Node> =
        bincode::deserialize_from(&mut reader).expect("Failed to deserialize node_map");
    let way_map: HashMap<u64, Way> =
        bincode::deserialize_from(&mut reader).expect("Failed to deserialize way_map");
    let node_subsets: Vec<NodeSubset> =
        bincode::deserialize_from(&mut reader).expect("Failed to deserialize node_subsets");

    (node_map, way_map, node_subsets)
}

pub fn reload_and_save(
    file_path: &Path,
    destination_path: &Path,
    filters: Vec<FilterSet>,
) -> OSMData {
    let mut osm_data = OSMData::new(file_path);
    osm_data.filter(filters);

    osm_data.save_hashmaps(destination_path);

    osm_data
}
