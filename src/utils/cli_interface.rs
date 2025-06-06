use log::error;
#[allow(unused)]
use log::{info, warn};

use crate::data_handling::OSMData;
use crate::path_finding::nearest_road::find_closest_road;
use crate::path_finding::path_finding::path_finding;
use crate::path_finding::TransportMode;

use std::process::Command;
use std::time::Instant;

pub fn get_input() -> String {
    let mut buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_string()
}

pub fn launch_cli_interface(osm_data: &OSMData) {
    let mut start_node_id: u64 = 0;
    let mut target_node_id: u64 = 0;

    let transport_mode = TransportMode::Bike(20.0 / 3.6);

    loop {
        info!(
            "
        Ready for input.

        Enter 'start' or 'target' to specify path locations.
        Enter 'path' to search for a path between the locations.
        Enter 'exit' to cancel.

        Other inputs will search the database."
        );
        let input_string = get_input();

        if input_string == "start".to_string() || input_string == "target".to_string() {
            info!("Enter {} node ID:", input_string);
            let node_id_input = get_input();

            let node_id_option: Option<u64> = match node_id_input.parse::<u64>() {
                Ok(node_id) => Some(node_id),
                Err(_) => {
                    error!("Cannot parse {} as node ID.", node_id_input);
                    None
                }
            };

            if let Some(node_id) = node_id_option {
                if osm_data.node_map.contains_key(&node_id) {
                    let closest_road_result = find_closest_road(osm_data, node_id, &transport_mode);

                    if let Some(_) = node_id_option {
                        match input_string.as_str() {
                            "start" => {
                                start_node_id = *closest_road_result.found_path.last().unwrap();
                                let minimum_distance = closest_road_result.path_length;
                                info!("Set start ID: road node {:.3}m away.", minimum_distance);
                            }
                            "target" => {
                                target_node_id = *closest_road_result.found_path.last().unwrap();
                                let minimum_distance = closest_road_result.path_length;
                                info!("Set target ID: road node {:.3}m away.", minimum_distance);
                            }
                            _ => {
                                error!("Unexpected input: {}", input_string);
                            }
                        };
                    }
                } else {
                    error!("Node ID not found in dataset: {}", node_id);
                }
            }
            continue;
        }

        if input_string == "path".to_string() {
            if start_node_id == 0 || target_node_id == 0 {
                warn!(
                    "Start or target nodes not set: {}, {}",
                    start_node_id, target_node_id
                );
                continue;
            }

            let start_time = Instant::now();
            let path_result =
                path_finding(&osm_data, start_node_id, target_node_id, &transport_mode);

            if let Some(path) = path_result {
                info!(
                    "Succeeded. Found path is {:.1}m ({:.1} km, {} nodes) long. Took {:.3}s",
                    path.path_length,
                    path.path_length / 1000.,
                    path.found_path.len(),
                    start_time.elapsed().as_secs_f64()
                );
                Command::new("python")
                    .arg("scripts/found_path_plotly.py")
                    .output()
                    .expect("Failed to execute Python script");
            } else {
                info!(
                    "Failed to find path. Took {:.3}s",
                    start_time.elapsed().as_secs_f64()
                );
            }
            continue;
        }

        if input_string == "exit".to_string() {
            break;
        }

        osm_data.search_landmarks(input_string);
    }
}
