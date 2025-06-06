use chrono::{DateTime, Utc};
#[allow(unused)]
use log::{info, warn};

use crate::{
    data_handling::OSMData,
    path_finding::{path_finding::direct_route, TransportMode},
    public_transport::public_transport::public_transport,
    utils::distance_utilities::f64_to_u64,
};

use super::Route;

pub fn search_routes(
    osm_data: &OSMData,
    transport_mode: &TransportMode,
    time: DateTime<Utc>,
    starting_node_id: u64,
    target_node_id: u64,
    minimum_distance_to_station: f64,
) -> Route {
    let public_transport_routes = public_transport(
        osm_data,
        &transport_mode,
        time,
        minimum_distance_to_station,
        starting_node_id,
        target_node_id,
    );

    let direct_route = direct_route(
        osm_data,
        &transport_mode,
        time,
        starting_node_id,
        target_node_id,
    );

    let mut routes: Vec<Route> = Vec::new();

    if let Ok(public_transport_routes) = public_transport_routes {
        routes.extend(public_transport_routes);
    }

    if let Ok(direct_route) = direct_route {
        routes.push(direct_route)
    }

    routes.sort_by_key(|route| f64_to_u64(route.total_duration()));

    let mut shortest_route: Route = Route::default();

    for (index, route) in routes.iter().enumerate() {
        if index == 0 {
            println!("Best route:");
            shortest_route = route.to_owned();
        } else {
            println!("Alternative route {}:", index);
        }
        route.print_route(osm_data);
    }

    if routes.len() == 0 {
        warn!("No routes found.")
    }

    shortest_route
}
