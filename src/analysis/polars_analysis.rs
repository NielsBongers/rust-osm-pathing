use core::f64;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::data_handling::OSMData;
use crate::path_finding::nearest_road::find_closest_road_coordinate;
use crate::path_finding::path_finding::path_finding;
use crate::path_finding::TransportMode;
use crate::utils::geographic_areas::GeographicArea;
use geo::GeodesicBearing;
#[allow(unused)]
use log::{info, warn};
use polars::prelude::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;

pub fn deviation_great_circle(osm_data: &OSMData) {
    let maximum_samples = 10000;

    let minimum_latitude = 49.02110;
    let maximum_latitude = 49.03956;

    let minimum_longitude = 104.03546;
    let maximum_longitude = 104.06881;

    let geographic_area = GeographicArea::new(
        minimum_latitude,
        maximum_latitude,
        minimum_longitude,
        maximum_longitude,
    );

    let mut coordinate_1_x = Vec::new();
    let mut coordinate_1_y = Vec::new();
    let mut coordinate_2_x = Vec::new();
    let mut coordinate_2_y = Vec::new();

    let mut path_length_vector: Vec<f64> = Vec::new();
    let mut great_circle_vector: Vec<f64> = Vec::new();
    let mut bearing_vector: Vec<f64> = Vec::new();

    let mut rng = SmallRng::from_entropy();

    let mut worst_performer_ratio = 1.0;
    let mut worst_performer_path: Vec<u64> = Vec::new();

    for _ in 0..maximum_samples {
        let coordinate_1 = geographic_area.random_coordinate(&mut rng);
        let coordinate_2 = geographic_area.random_coordinate(&mut rng);

        let (road_node_id_1, _) = find_closest_road_coordinate(osm_data, coordinate_1);
        let (road_node_id_2, _) = find_closest_road_coordinate(osm_data, coordinate_2);

        if road_node_id_1 == road_node_id_2 {
            warn!("Same IDs found! Skipping.");
            continue;
        }

        let road_node_coordinate_1 = osm_data.node_map.get(&road_node_id_1).unwrap().coordinate;
        let road_node_coordinate_2 = osm_data.node_map.get(&road_node_id_2).unwrap().coordinate;

        let (bearing, great_circle_distance) =
            road_node_coordinate_1.geodesic_bearing_distance(road_node_coordinate_2);

        let transport_mode = TransportMode::Walk(6.0 / 3.6);

        let path_result = path_finding(osm_data, road_node_id_1, road_node_id_2, &transport_mode);

        if let Some(path_result) = path_result {
            assert!(
                path_result.path_length >= great_circle_distance,
                "Did we break the spacetime continuum?"
            );

            coordinate_1_x.push(coordinate_1.x());
            coordinate_1_y.push(coordinate_1.y());

            coordinate_2_x.push(coordinate_2.x());
            coordinate_2_y.push(coordinate_2.y());

            path_length_vector.push(path_result.path_length);
            great_circle_vector.push(great_circle_distance);
            bearing_vector.push(bearing);

            let ratio = path_result.path_length / great_circle_distance;

            if ratio > worst_performer_ratio {
                info!(
                    "New worst path found, with ratio {}: {:?}",
                    ratio, path_result
                );

                worst_performer_ratio = ratio;
                worst_performer_path = path_result.found_path;
            }

            // info!(
            //     "Path length: {:.3}. Great circle: {:.3}",
            //     path_result.path_length, great_circle_distance
            // );
        } else {
            warn!(
                "Invalid path encountered, between {} and {} - got {:?}",
                road_node_id_1, road_node_id_2, path_result
            );
        }
    }

    let mut worst_path_file = File::create(Path::new("results/analysis/worst_case_map.csv"))
        .expect("Failed to create file");

    worst_path_file
        .write("lat,lon,node_id\n".as_bytes())
        .expect("Failed to write headers");

    for node_id in worst_performer_path.iter() {
        let coordinate = osm_data.node_map.get(node_id).unwrap().coordinate;

        worst_path_file
            .write(format!("{},{},{}\n", coordinate.y(), coordinate.x(), node_id).as_bytes())
            .expect("Failed to write line");
    }

    let df = DataFrame::new(vec![
        Series::new("coordinate_1_x".into(), coordinate_1_x),
        Series::new("coordinate_1_y".into(), coordinate_1_y),
        Series::new("coordinate_2_x".into(), coordinate_2_x),
        Series::new("coordinate_2_y".into(), coordinate_2_y),
        Series::new("path_length".into(), path_length_vector),
        Series::new("great_circle".into(), great_circle_vector),
        Series::new("bearing".into(), bearing_vector),
    ])
    .expect("Failed to create Polars series");

    let mut df = df
        .lazy()
        .with_column((col("path_length") / col("great_circle")).alias("path_ratio"))
        .collect()
        .expect("Failed to calculate ratio");

    let save_path = Path::new("results/analysis/analysis_results.parquet");

    let mut file = File::create(save_path).expect("Failed to create parquet file");

    ParquetWriter::new(&mut file)
        .finish(&mut df)
        .expect("Failed to write parquet file");

    // let mean_lengths = df
    //     .lazy()
    //     .select([
    //         col("path_length").mean().alias("mean_path_length"),
    //         col("great_circle").mean().alias("mean_great_circle"),
    //     ])
    //     .with_column(
    //         (col("mean_path_length") / col("mean_great_circle"))
    //             .alias("ratio_path_length_great_circle"),
    //     )
    //     .collect()
    //     .expect("Failed to calculate means");

    // info!("Mean: {}", mean_lengths);
}
