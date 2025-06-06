#![allow(unused)]
use env_logger::{Builder, Env};
use log::{info, warn};

use geo::GeodesicDistance;
use geo::Point;
use osm_rust::analysis::amenity_analysis::amenity_analysis;
use osm_rust::data_handling::FilterSubset::Landmark;
use osm_rust::data_handling::OSMData;
use osm_rust::path_finding::TransportMode;
use osm_rust::route_manager::transport_options::search_routes;
use osm_rust::utils::coordinate_files::load_coordinate_file;
use osm_rust::utils::hashmap_creation::recreate_hashmap;
use osm_rust::utils::node_examples::get_path_example;
use osm_rust::utils::node_examples::PathExamples;
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting!");

    let num_threads = 20;
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .expect("Failed to build thread pool");

    let start_time = Instant::now();

    let source_path = Path::new("path/to/osm_file.osm");
    let file_path = Path::new("data/hashmaps/netherlands-recreated-adjusted.hashmap");
    let osm_data = recreate_hashmap(source_path, file_path);

    // let osm_data: OSMData = OSMData::new(file_path);

    info!(
        "Loaded in the data - took {:.3}s",
        start_time.elapsed().as_secs_f64()
    );

    amenity_analysis(&osm_data);

    let (starting_node_id, target_node_id) = get_path_example(&PathExamples::Weert);
    let transport_mode = TransportMode::Bike(20.0 / 3.6);
    let time = chrono::Utc::now();
    let minimum_distance_to_station = 8000.0;

    search_routes(
        &osm_data,
        &transport_mode,
        time,
        starting_node_id,
        target_node_id,
        minimum_distance_to_station,
    );

    std::process::exit(0);
}
