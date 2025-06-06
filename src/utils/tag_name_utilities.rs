use crate::osm_parsing::Node;
#[allow(unused)]
use log::{info, warn};

pub fn node_name_or_address(node: &Node) -> Option<String> {
    let landmark_name = node.tags.get("name");
    let street_name = node.tags.get("addr:street");
    let house_number = node.tags.get("addr:housenumber");

    if let Some(landmark_name) = landmark_name {
        return Some(format!("{}", landmark_name));
    }

    if let Some(street_name) = street_name {
        if let Some(house_number) = house_number {
            return Some(format!("{} {}", street_name, house_number));
        } else {
            return None;
        }
    } else {
        return None;
    }
}
