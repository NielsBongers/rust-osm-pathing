pub mod nearest_road;
pub mod path_finding;
pub mod queue_handling;

/// This implements items for the priority queue in the form of a BinaryHeap.
/// distance_to_start is a u64: we multiply the float distances (in meters) by 1e6 and convert them to u64.
/// This allows for far easier comparisons without floating point math, and without any real
#[derive(Debug, Eq, PartialEq)]
pub struct QueueItem {
    cost: u64,
    order_added: usize,
    time_to_start: u64,
    node_id: u64,
}

/// For nearest road node: end_node is always the road.
#[derive(Debug, Clone, Default)]
pub struct PathResult {
    pub start_node: u64,
    pub end_node: u64,

    pub found_path: Vec<u64>,
    pub path_length: f64,
    pub path_time: f64,
}

#[derive(Debug)]
pub enum TransportMode {
    Car,
    Bike(f64),
    Walk(f64),
}

const ROAD_DEFAULT_SPEED: f64 = 60. / 3.6;
const ROAD_MAXIMUM_SPEED: f64 = 120. / 3.6;
