use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::osm_parsing::{Node, Way};

pub mod data_handling;
pub mod filtering;
pub mod searching;

#[derive(Default, Debug)]
pub struct OSMData {
    pub node_map: HashMap<u64, Node>,
    pub way_map: HashMap<u64, Way>,
    pub node_subsets: Vec<NodeSubset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeSubset {
    pub filter_subset: FilterSubset,
    pub node_subset: HashSet<u64>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum FilterSubset {
    Roads,
    Landmark(String),
    #[default]
    AllLandmarks,
}

#[derive(Default, Debug)]
pub struct FilterSet {
    pub filter_key: String,
    pub filter_values: HashSet<String>,
    pub filter_subset: FilterSubset,
}
