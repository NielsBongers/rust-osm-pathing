use super::{Route, RouteComponent};
use crate::{data_handling::OSMData, utils::tag_name_utilities::node_name_or_address};
#[allow(unused)]
use log::{info, warn};

impl Route {
    pub fn new(components: Vec<RouteComponent>) -> Self {
        Route { components }
    }

    pub fn default() -> Self {
        let components: Vec<RouteComponent> = Vec::new();
        Route { components }
    }

    pub fn total_duration(&self) -> f64 {
        let mut total_duration = 0.0;
        for component in self.components.iter() {
            let duration = match component {
                RouteComponent::Path(path_result) => path_result.path_time,
                RouteComponent::PublicTransport(public_transport) => {
                    public_transport.journey_duration + public_transport.waiting_time
                }
            };
            total_duration += duration;
        }
        total_duration
    }

    pub fn print_route(&self, osm_data: &OSMData) {
        for component in self.components.iter() {
            match component {
                RouteComponent::Path(path_result) => {
                    if let Some(start_node) = osm_data.node_map.get(&path_result.start_node) {
                        let route_information =
                            if let Some(node_text) = node_name_or_address(start_node) {
                                format!(
                                    "Name: {}. Travel {:.1}m ({:.1} minutes).",
                                    node_text,
                                    path_result.path_length,
                                    path_result.path_time / 60.0
                                )
                            } else {
                                format!(
                                    "Travel {:.1}m ({:.1} minutes).",
                                    path_result.path_length,
                                    path_result.path_time / 60.0
                                )
                            };

                        println!("\t{}", route_information);
                    }
                }
                RouteComponent::PublicTransport(public_transport) => {
                    let transfer_string = if public_transport.transfers == 0 {
                        format!("no transfers")
                    } else {
                        format!(
                            "{} transfer{} at {}",
                            public_transport.transfers,
                            (if public_transport.transfers == 1 {
                                ""
                            } else {
                                "s"
                            }),
                            public_transport.intermediate_stations.join(", ")
                        )
                    };

                    println!(
                        "\tTrain in the direction of {}. Wait {:.1} minutes. Journey duration of {:.1} minutes, with {}.",
                        public_transport.direction,
                        public_transport.waiting_time / 60.0,
                        public_transport.journey_duration / 60.0,
                        transfer_string,
                    );
                }
            };
        }

        println!(
            "\tTotal journey duration: {:.0} minutes.",
            self.total_duration() / 60.0
        );
    }
}
