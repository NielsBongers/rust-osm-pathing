use crate::data_handling::FilterSubset::Landmark;
use crate::{data_handling::OSMData, utils::coordinate_files::load_coordinate_file};
use core::f64;
use csv::Writer;
use geo::GeodesicDistance;
use geo::Point;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Serialize)]
pub struct ShortestDistances {
    index: i64,
    latitude: f64,
    longitude: f64,
    kindergarten: f64,
    hospital: f64,
    school: f64,
    university: f64,
    bus_stop: f64,
    central_square: f64,
}

pub fn amenity_analysis(osm_data: &OSMData) {
    let coordinate_file_path =
        Path::new(r"..\mongolian-scraping\results\rust_interface\python_results.csv");
    let coordinate_results =
        Path::new(r"..\mongolian-scraping\results\rust_interface\rust_results.csv");

    let central_square = Point::new(106.917574, 47.918821);

    let coordinate_vector = load_coordinate_file(&coordinate_file_path);

    let mut base_amenity_map: HashMap<String, f64> = HashMap::new();
    base_amenity_map.insert("kindergarten".to_string(), f64::INFINITY);
    base_amenity_map.insert("hospital".to_string(), f64::INFINITY);
    base_amenity_map.insert("school".to_string(), f64::INFINITY);
    base_amenity_map.insert("university".to_string(), f64::INFINITY);
    base_amenity_map.insert("bus_stop".to_string(), f64::INFINITY);
    base_amenity_map.insert("central_square".to_string(), f64::INFINITY);

    let mut amenity_distance_results: Vec<ShortestDistances> = Vec::new();

    for coordinate_entry in coordinate_vector.iter() {
        let coordinate = Point::new(coordinate_entry.longitude, coordinate_entry.latitude);
        let mut amenity_map = base_amenity_map.clone();

        for node_subset in osm_data.node_subsets.iter() {
            match &node_subset.filter_subset {
                Landmark(name) => match name.as_str() {
                    "amenity" => {
                        for node_id in &node_subset.node_subset {
                            if let Some(node) = osm_data.node_map.get(node_id) {
                                if let Some(amenity_type) = node.tags.get("amenity") {
                                    let distance_to_amenity =
                                        coordinate.geodesic_distance(&node.coordinate);

                                    if let Some(distance) = amenity_map.get_mut(amenity_type) {
                                        if distance_to_amenity < *distance {
                                            *distance = distance_to_amenity;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    "bus_stop" => {
                        for node_id in &node_subset.node_subset {
                            if let Some(node) = osm_data.node_map.get(node_id) {
                                if let Some(amenity_type) = node.tags.get("highway") {
                                    let distance_to_amenity =
                                        coordinate.geodesic_distance(&node.coordinate);

                                    if let Some(distance) = amenity_map.get_mut(amenity_type) {
                                        if distance_to_amenity < *distance {
                                            *distance = distance_to_amenity;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        let central_square_distance = coordinate.geodesic_distance(&central_square);

        let shortest_distances = ShortestDistances {
            index: coordinate_entry.index,
            latitude: coordinate_entry.latitude,
            longitude: coordinate_entry.longitude,
            kindergarten: *amenity_map.get("kindergarten").unwrap(),
            hospital: *amenity_map.get("hospital").unwrap(),
            school: *amenity_map.get("school").unwrap(),
            university: *amenity_map.get("university").unwrap(),
            bus_stop: *amenity_map.get("bus_stop").unwrap(),
            central_square: central_square_distance,
        };

        amenity_distance_results.push(shortest_distances);
    }

    let file = File::create(coordinate_results).expect("Failed to create file");
    let mut wtr = Writer::from_writer(file);

    // Iterate over the results and serialize each row, but only write the header once
    for entry in &amenity_distance_results {
        wtr.serialize(entry)
            .expect("Failed to serialize data to CSV");
    }

    // Flush the writer to ensure all data is written
    wtr.flush().expect("Failed to flush writer");
}
