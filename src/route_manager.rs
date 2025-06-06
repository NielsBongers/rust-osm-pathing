use crate::{path_finding::PathResult, public_transport::PublicTransportResult};

pub mod route_manager;
pub mod transport_options;

#[derive(Clone, Debug)]
pub enum RouteComponent {
    Path(PathResult),
    PublicTransport(PublicTransportResult),
}

#[derive(Clone, Debug)]
pub struct Route {
    pub components: Vec<RouteComponent>,
}
