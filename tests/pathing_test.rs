use env_logger::{Builder, Env};

use osm_rust::{
    data_handling::OSMData,
    path_finding::{path_finding::path_finding, TransportMode},
    utils::filtering_utilities::filter_highways,
};
use rayon::ThreadPoolBuilder;
use std::path::Path;

#[test]
fn pathing_test() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let num_threads = 20;

    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .expect("Failed to build thread pool");

    let file_path = Path::new("data/maps/full.osm");

    let mut osm_data = OSMData::new(file_path);

    let valid_road_types = filter_highways();
    let filters = vec![valid_road_types];
    osm_data.filter(filters);

    let start_node_id: u64 = 3738046045;
    let target_node_id: u64 = 2659727380;

    let transport_mode = TransportMode::Bike(20.0 / 3.6);

    path_finding(&osm_data, start_node_id, target_node_id, &transport_mode)
        .expect("Failed to find path");
}
