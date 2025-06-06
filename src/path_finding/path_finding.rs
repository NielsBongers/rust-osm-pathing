use chrono::{DateTime, Utc};
use geo::GeodesicDistance;
#[allow(unused)]
use log::{info, warn};

use crate::{
    data_handling::OSMData,
    path_finding::{PathResult, QueueItem, ROAD_MAXIMUM_SPEED},
    route_manager::{Route, RouteComponent},
};
use std::{
    collections::{BinaryHeap, HashMap},
    fs::create_dir_all,
};

use crate::path_finding::ROAD_DEFAULT_SPEED;

use crate::utils::distance_utilities::f64_to_u64;

use super::{nearest_road::find_closest_road, TransportMode};

impl PathResult {
    pub fn new(
        found_path: Vec<u64>,
        path_length: f64,
        path_time: f64,
        start_node: u64,
        end_node: u64,
    ) -> Self {
        PathResult {
            start_node,
            end_node,
            found_path,
            path_length,
            path_time,
        }
    }
}

pub fn process_found_path(
    osm_data: &OSMData,
    start_node_id: u64,
    target_node_id: u64,
    parent_map: &HashMap<u64, u64>,
    transport_mode: &TransportMode,
) -> PathResult {
    let mut child_id = &target_node_id;

    let mut found_path: Vec<u64> = Vec::new();
    let mut path_length: f64 = 0.0;
    let mut path_time: f64 = 0.0;

    // let mut found_path_file = File::create(Path::new("results/pathing/found_path.csv"))
    //     .expect("Failed to open found path file");

    // let headers = "lat,lon,node_id\n";
    // found_path_file
    //     .write(headers.as_bytes())
    //     .expect("Failed to write headers");

    while *child_id != start_node_id {
        let parent_id = parent_map.get(&child_id).unwrap();
        let child_node = osm_data.node_map.get(child_id).unwrap();
        let parent_node = osm_data.node_map.get(&parent_id).unwrap();

        let distance_parent_child = child_node
            .coordinate
            .geodesic_distance(&parent_node.coordinate);

        let node_speed = match transport_mode {
            TransportMode::Walk(walking_speed) => *walking_speed,
            TransportMode::Bike(bike_speed) => *bike_speed,
            TransportMode::Car => osm_data
                .node_max_speed(*child_id)
                .unwrap_or(ROAD_DEFAULT_SPEED),
        };

        path_length += distance_parent_child;
        path_time += distance_parent_child / node_speed;

        found_path.push(*child_id);

        child_id = parent_id;

        // let coordinate_data = format!(
        //     "{},{},{}\n",
        //     child_node.coordinate.y(),
        //     child_node.coordinate.x(),
        //     child_id
        // );

        // found_path_file
        //     .write(coordinate_data.as_bytes())
        //     .expect("Failed to write to coordinates file");
    }

    PathResult::new(
        found_path,
        path_length,
        path_time,
        start_node_id,
        target_node_id,
    )
}

pub fn path_finding(
    osm_data: &OSMData,
    start_node_id: u64,
    target_node_id: u64,
    transport_mode: &TransportMode,
) -> Option<PathResult> {
    assert!(
        osm_data.node_map.contains_key(&start_node_id),
        "Starting node not in map: {}",
        start_node_id
    );
    assert!(
        osm_data.node_map.contains_key(&target_node_id),
        "Target node not in map: {}",
        target_node_id
    );
    assert!(
        start_node_id != target_node_id,
        "Start and target node should not be the same: {}",
        start_node_id
    );

    let mut node_priority_queue: BinaryHeap<QueueItem> = BinaryHeap::new();
    let mut time_from_start: HashMap<u64, u64> = HashMap::new();
    let mut parent_map: HashMap<u64, u64> = HashMap::new();
    let mut insertion_counter: usize = 0;

    let heuristic_weight = 1.0;

    // Getting the target coordinates for A*.
    let target_coordinate = osm_data
        .node_map
        .get(&target_node_id)
        .unwrap()
        .coordinate
        .clone();

    let starting_node = QueueItem::new(start_node_id, insertion_counter, 0, 0);

    node_priority_queue.push(starting_node);
    parent_map.insert(start_node_id, start_node_id);
    time_from_start.insert(start_node_id, 0);

    let mut has_succeeded = false;

    create_dir_all("results/pathing").expect("Failed to create results/pathing directory.");
    // let mut coordinates_file = File::create("results/pathing/search_coordinates.csv")
    //     .expect("Failed to open coordinates file");

    // let headers = "lat,lon,node_id\n";
    // coordinates_file
    //     .write(headers.as_bytes())
    //     .expect("Failed to write headers");

    // Taking from the front of the queue.
    while let Some(queue_item) = node_priority_queue.pop() {
        // Incrementing the node order counter. Starts at 0 for the first node, so has to be incremented here already.
        insertion_counter += 1;

        if let Some(parent_node) = osm_data.node_map.get(&queue_item.node_id) {
            // let coordinate_data = format!(
            //     "{},{},{}\n",
            //     parent_node.coordinate.y(),
            //     parent_node.coordinate.x(),
            //     parent_node.id
            // );

            // coordinates_file
            //     .write(coordinate_data.as_bytes())
            //     .expect("Failed to write to coordinates file");

            let time_start_to_parent = queue_item.time_to_start;

            for child_node_id in parent_node.nodes.iter() {
                if let Some(child_node) = osm_data.node_map.get(child_node_id) {
                    let distance_parent_child = parent_node
                        .coordinate
                        .geodesic_distance(&child_node.coordinate);

                    let node_speed = match transport_mode {
                        TransportMode::Walk(walking_speed) => *walking_speed,
                        TransportMode::Bike(bike_speed) => *bike_speed,
                        TransportMode::Car => osm_data
                            .node_max_speed(*child_node_id)
                            .unwrap_or(ROAD_DEFAULT_SPEED),
                    };

                    let parent_to_child_time = distance_parent_child / node_speed;

                    let time_start_to_child =
                        time_start_to_parent + f64_to_u64(parent_to_child_time);

                    if !parent_map.contains_key(child_node_id)
                        || time_start_to_child < *time_from_start.get(child_node_id).unwrap()
                    {
                        parent_map.insert(*child_node_id, parent_node.id);

                        if *child_node_id == target_node_id {
                            node_priority_queue.clear();
                            has_succeeded = true;
                            break;
                        }

                        time_from_start.insert(*child_node_id, time_start_to_child);

                        let heuristic_speed = match transport_mode {
                            TransportMode::Walk(walking_speed) => walking_speed,
                            TransportMode::Bike(bike_speed) => bike_speed,
                            TransportMode::Car => &ROAD_MAXIMUM_SPEED,
                        };

                        let time_to_target = f64_to_u64(
                            child_node.coordinate.geodesic_distance(&target_coordinate)
                                / heuristic_speed
                                * heuristic_weight,
                        );

                        let overall_cost = time_start_to_child + time_to_target;

                        let child_node_queue_item = QueueItem::new(
                            *child_node_id,
                            insertion_counter,
                            overall_cost,
                            time_start_to_child,
                        );
                        node_priority_queue.push(child_node_queue_item);
                    }
                }
            }
        }
    }

    if has_succeeded {
        let found_result = process_found_path(
            osm_data,
            start_node_id,
            target_node_id,
            &parent_map,
            transport_mode,
        );
        Some(found_result)
    } else {
        None
    }
}

pub fn direct_route(
    osm_data: &OSMData,
    transport_mode: &TransportMode,
    _time: DateTime<Utc>,
    starting_node_id: u64,
    target_node_id: u64,
) -> Result<Route, Box<dyn std::error::Error>> {
    // Getting the closest road nodes for both
    let start_to_start_road = find_closest_road(osm_data, starting_node_id, &transport_mode);
    let target_road_to_target = find_closest_road(osm_data, target_node_id, &transport_mode);

    let start_road_node = start_to_start_road.end_node;
    let target_road_node = target_road_to_target.end_node;

    // Regular path without public transport
    let regular_path = path_finding(osm_data, start_road_node, target_road_node, &transport_mode)
        .ok_or("Error: no path found.")?;

    let regular_path_component = RouteComponent::Path(regular_path);
    let route = Route::new(vec![regular_path_component]);

    Ok(route)
}
