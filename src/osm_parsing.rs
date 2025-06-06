use std::collections::HashMap;

use geo::Point;
use serde::{Deserialize, Serialize};

pub mod osm_data_types;
pub mod osm_parsing;
pub mod state_machine;

#[derive(Debug)]
pub enum CurrentlyReading {
    Node(u64),
    Way(u64),
    Relation(u64),
    None,
}

#[derive(Deserialize, Serialize)]
pub struct Node {
    pub id: u64,
    pub coordinate: Point,

    pub tags: HashMap<String, String>,
    pub ways: Vec<u64>,
    pub nodes: Vec<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Way {
    pub id: u64,
    pub node_ids: Vec<u64>,
    pub tags: HashMap<String, String>,
}

pub struct StateMachine {
    currently_reading: CurrentlyReading,
}
