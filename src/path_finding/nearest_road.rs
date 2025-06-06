use geo::{GeodesicDistance, Point};

use crate::data_handling::{FilterSubset, OSMData};

use super::{PathResult, TransportMode, ROAD_DEFAULT_SPEED};

pub fn find_closest_road_coordinate(osm_data: &OSMData, coordinate: Point) -> (u64, f64) {
    let mut minimum_distance: f64 = f64::MAX;
    let mut closest_node_id: u64 = 0;

    for subset in osm_data.node_subsets.iter() {
        match subset.filter_subset {
            FilterSubset::Roads => {
                for node_id in subset.node_subset.iter() {
                    if let Some(node) = osm_data.node_map.get(node_id) {
                        let distance = coordinate.geodesic_distance(&node.coordinate);
                        if distance < minimum_distance {
                            minimum_distance = distance;
                            closest_node_id = *node_id;
                        }
                    }
                }
            }
            _ => (),
        };
    }

    (closest_node_id, minimum_distance)
}

pub fn find_closest_road(
    osm_data: &OSMData,
    node_id: u64,
    transport_mode: &TransportMode,
) -> PathResult {
    let node_coordinates = osm_data
        .node_map
        .get(&node_id)
        .expect("Node does not exist")
        .coordinate;

    let (closest_node_id, minimum_distance) =
        find_closest_road_coordinate(osm_data, node_coordinates);

    let found_path: Vec<u64> = vec![node_id, closest_node_id];
    let path_length = minimum_distance;
    let path_time = match transport_mode {
        TransportMode::Walk(walking_speed) => minimum_distance / walking_speed,
        TransportMode::Bike(bike_speed) => minimum_distance / bike_speed,
        TransportMode::Car => minimum_distance / ROAD_DEFAULT_SPEED,
    };

    PathResult::new(found_path, path_length, path_time, node_id, closest_node_id)
}
