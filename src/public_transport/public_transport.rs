use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::data_handling::OSMData;
use crate::path_finding::nearest_road::find_closest_road;
use crate::path_finding::path_finding::path_finding;
use crate::path_finding::TransportMode;
use crate::route_manager::Route;
use crate::route_manager::RouteComponent;

#[allow(unused)]
use log::{info, warn};

use super::ns_api::get_next_train;
use super::stations::find_nearby_stations;
use super::PublicTransportResult;

impl PublicTransportResult {
    pub fn new(
        start_station_id: u64,
        end_station_id: u64,
        journey_duration: f64,
        waiting_time: f64,
        direction: String,
        transfers: i64,
        intermediate_stations: Vec<String>,
    ) -> Self {
        PublicTransportResult {
            start_station_id,
            end_station_id,
            journey_duration,
            waiting_time,
            direction,
            transfers,
            intermediate_stations,
        }
    }
}

pub fn public_transport(
    osm_data: &OSMData,
    transport_mode: &TransportMode,
    time: DateTime<Utc>,
    minimum_distance_to_station: f64,
    starting_node_id: u64,
    target_node_id: u64,
) -> Result<Vec<Route>, Box<dyn std::error::Error>> {
    // Setting up the starting/target node
    let start_landmark = osm_data
        .node_map
        .get(&starting_node_id)
        .expect("Node not in map!");

    let target_landmark = osm_data
        .node_map
        .get(&target_node_id)
        .expect("Node not in map!");

    // Getting the closest road nodes for both
    let start_to_start_road = find_closest_road(osm_data, starting_node_id, &transport_mode);
    let target_road_to_target = find_closest_road(osm_data, target_node_id, &transport_mode);

    // Finding stations near both
    let stations_near_start = find_nearby_stations(
        &osm_data,
        &start_landmark.coordinate,
        minimum_distance_to_station,
    );
    let stations_near_target = find_nearby_stations(
        &osm_data,
        &target_landmark.coordinate,
        minimum_distance_to_station,
    );

    let mut route_options: Vec<Route> = Vec::new();

    // Checking all the station combinations
    for start_station_id in stations_near_start.iter() {
        // Getting station nodes and names
        let start_station_node = osm_data
            .node_map
            .get(start_station_id)
            .expect("Couldn't find station on the map");
        let start_station_name = start_station_node
            .tags
            .get("name")
            .expect("Failed to read station name");

        // Roads close to starting station
        let road_to_start_station = find_closest_road(osm_data, *start_station_id, &transport_mode);

        let start_road_to_station = path_finding(
            &osm_data,
            start_to_start_road.end_node,
            road_to_start_station.end_node,
            &transport_mode,
        )
        .expect("Failed to find path.");

        for target_station_id in stations_near_target.iter() {
            if start_station_id == target_station_id {
                continue;
            }
            let target_station_node = osm_data
                .node_map
                .get(target_station_id)
                .expect("Couldn't find station on the map");

            let target_station_name = target_station_node
                .tags
                .get("name")
                .expect("Failed to read station name");

            let target_station_to_target_station_road =
                find_closest_road(osm_data, *target_station_id, &transport_mode);

            let target_station_road_to_target_road = path_finding(
                &osm_data,
                target_station_to_target_station_road.end_node,
                target_road_to_target.end_node,
                &transport_mode,
            )
            .ok_or("Error: no path found.")?;

            // Tracking time spent travelling to the station.
            let time_to_station = Duration::from_secs_f64(
                start_to_start_road.path_time
                    + start_road_to_station.path_time
                    + road_to_start_station.path_time,
            );

            let public_transport = get_next_train(
                *start_station_id,
                *target_station_id,
                start_station_name,
                target_station_name,
                &(time + time_to_station),
            );

            if let Some(public_transport) = public_transport {
                let start_to_start_road_component =
                    RouteComponent::Path(start_to_start_road.clone());
                let start_road_to_station_component =
                    RouteComponent::Path(start_road_to_station.clone());
                let road_to_start_station_component =
                    RouteComponent::Path(road_to_start_station.clone());
                let public_transport_component = RouteComponent::PublicTransport(public_transport);
                let target_station_to_target_station_road_component =
                    RouteComponent::Path(target_station_to_target_station_road.clone());
                let target_station_road_to_target_road_component =
                    RouteComponent::Path(target_station_road_to_target_road.clone());
                let target_road_to_target_component =
                    RouteComponent::Path(target_road_to_target.clone());

                let components = vec![
                    start_to_start_road_component,
                    start_road_to_station_component,
                    road_to_start_station_component,
                    public_transport_component,
                    target_station_to_target_station_road_component,
                    target_station_road_to_target_road_component,
                    target_road_to_target_component,
                ];

                let route = Route::new(components);

                route_options.push(route);
            }
        }
    }

    Ok(route_options)
}
