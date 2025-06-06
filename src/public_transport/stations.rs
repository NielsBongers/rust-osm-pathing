use geo::{GeodesicDistance, Point};
#[allow(unused)]
use log::{info, warn};

use crate::data_handling::OSMData;

use crate::data_handling::FilterSubset::Landmark;

pub fn find_nearby_stations(
    osm_data: &OSMData,
    coordinate: &Point,
    minimum_distance: f64,
) -> Vec<u64> {
    let mut valid_stations = Vec::<u64>::new();

    for node_subset in osm_data.node_subsets.iter() {
        match &node_subset.filter_subset {
            Landmark(landmark_name) => {
                if *landmark_name == "stations".to_string() {
                    for node_id in node_subset.node_subset.iter() {
                        if let Some(node) = osm_data.node_map.get(node_id) {
                            if node.tags.get("public_transport") == Some(&"station".to_string())
                                && node.tags.get("railway") == Some(&"station".to_string())
                            {
                                let distance_to_station =
                                    node.coordinate.geodesic_distance(&coordinate);

                                if distance_to_station <= minimum_distance {
                                    valid_stations.push(*node_id);
                                }
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    valid_stations
}
