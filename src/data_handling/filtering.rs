use crate::data_handling::FilterSet;
use crate::data_handling::NodeSubset;
use crate::data_handling::{FilterSubset, OSMData};
#[allow(unused)]
use log::{info, warn};
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Mutex, RwLock};

impl NodeSubset {
    pub fn new(nodes_filtered: HashSet<u64>, filter_subset: FilterSubset) -> Self {
        NodeSubset {
            filter_subset,
            node_subset: nodes_filtered,
        }
    }
}

impl OSMData {
    pub fn filter(&mut self, filters: Vec<FilterSet>) {
        let mut nodes_to_keep: HashSet<u64> = HashSet::new();
        let mut ways_to_keep: HashSet<u64> = HashSet::new();

        // Appling all the filters.
        for filter in filters.iter() {
            let (nodes_filtered, ways_filtered) = match filter.filter_subset {
                FilterSubset::AllLandmarks => self.filter_all_landmarks(filter),
                FilterSubset::Landmark(_) => self.filter_landmarks(filter),
                FilterSubset::Roads => self.filter_ways(filter),
            };

            // Have to clone here: I need to have the node u64s both here and in the to-keep list.
            let node_subset = NodeSubset::new(nodes_filtered.clone(), filter.filter_subset.clone());
            self.node_subsets.push(node_subset);

            // Updating the list of nodes to keep around.
            nodes_to_keep.extend(nodes_filtered);
            ways_to_keep.extend(ways_filtered);
        }

        self.node_map
            .retain(|node_id, _| nodes_to_keep.contains(node_id));

        self.way_map
            .retain(|way_id, _| ways_to_keep.contains(way_id));

        // This adds the ways each node is part of and adds adjacent nodes to each other for pathfinding.
        self.update_road_nodes();
    }

    pub fn filter_landmarks(&mut self, filter: &FilterSet) -> (HashSet<u64>, HashSet<u64>) {
        let filter_key = &filter.filter_key;
        let filter_values_set = &filter.filter_values;

        let mut nodes_to_keep: HashSet<u64> = HashSet::new();
        let ways_to_keep: HashSet<u64> = HashSet::new();

        for (node_id, node) in self.node_map.iter() {
            for (key, value) in node.tags.iter() {
                if key == filter_key && filter_values_set.contains(value) {
                    nodes_to_keep.insert(*node_id);
                }
            }
        }

        (nodes_to_keep, ways_to_keep)
    }

    pub fn filter_all_landmarks(&mut self, _filter: &FilterSet) -> (HashSet<u64>, HashSet<u64>) {
        let mut nodes_to_keep: HashSet<u64> = HashSet::new();
        let ways_to_keep: HashSet<u64> = HashSet::new();

        let mut nodes_in_ways: HashSet<u64> = HashSet::new();
        // Getting a hashset of any node that's also part of a way (whether a road or a house, area etc.).
        for (_, way) in self.way_map.iter() {
            for node_id in way.node_ids.iter() {
                nodes_in_ways.insert(*node_id);
            }
        }

        // Getting any node with at least 1 tag and that is not also part of a way.
        for (node_id, node) in self.node_map.iter() {
            if node.tags.len() > 0 && !nodes_in_ways.contains(node_id) {
                nodes_to_keep.insert(*node_id);
            }
        }

        (nodes_to_keep, ways_to_keep)
    }

    pub fn filter_ways(&mut self, filter: &FilterSet) -> (HashSet<u64>, HashSet<u64>) {
        let tag_key = &filter.filter_key;
        let tag_values = &filter.filter_values;

        let ways_to_keep = Mutex::new(HashSet::new());
        let nodes_to_keep = RwLock::new(HashSet::new());

        self.way_map.par_iter().for_each(|(way_id, way)| {
            let mut keep_way = false;

            if let Some(tag_value) = &way.tags.get(tag_key) {
                if tag_values.contains(*tag_value) {
                    keep_way = true;
                }
            }

            if keep_way {
                ways_to_keep.lock().unwrap().insert(*way_id);
                let mut nodes_lock = nodes_to_keep.write().unwrap();
                for node in &way.node_ids {
                    nodes_lock.insert(*node);
                }
            }
        });

        let ways_to_keep = ways_to_keep.into_inner().unwrap();
        let nodes_to_keep = nodes_to_keep.into_inner().unwrap();

        (nodes_to_keep, ways_to_keep)
    }
}
