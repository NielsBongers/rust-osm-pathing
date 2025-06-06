use super::OSMData;
use crate::data_handling::FilterSubset;

use log::{info, warn};

impl OSMData {
    pub fn search_landmarks(&self, search_string: String) {
        let mut result_counter = 0;

        for subset in self.node_subsets.iter() {
            match subset.filter_subset {
                FilterSubset::AllLandmarks => {
                    for node_id in subset.node_subset.iter() {
                        if let Some(node) = self.node_map.get(&node_id) {
                            for (_, value) in node.tags.iter() {
                                if value.contains(&search_string) {
                                    info!("{:?}", node);
                                    result_counter += 1;
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        if result_counter > 0 {
            info!(
                "Done! Found {} results for query {}",
                result_counter, search_string
            );
        } else {
            warn!("No results found for query: {}", search_string);
        }
    }
}
