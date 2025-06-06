use std::cmp::Ordering;

use crate::path_finding::QueueItem;

impl QueueItem {
    pub fn new(node_id: u64, order_added: usize, cost: u64, time_to_start: u64) -> Self {
        QueueItem {
            cost,
            order_added,
            time_to_start,
            node_id,
        }
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // Pick the lowest cost.
        other
            .cost
            .cmp(&self.cost)
            // If costs are the same, compare by order instead.
            .then_with(|| other.order_added.cmp(&self.order_added))
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
