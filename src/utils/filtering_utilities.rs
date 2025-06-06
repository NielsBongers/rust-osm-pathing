use std::collections::HashSet;

use crate::data_handling::{FilterSet, FilterSubset};

pub fn filter_highways() -> FilterSet {
    let filter_key = "highway".to_string();
    let filter_values = HashSet::from([
        "motorway".to_string(),
        "trunk".to_string(),
        "primary".to_string(),
        "secondary".to_string(),
        "tertiary".to_string(),
        "unclassified".to_string(),
        "residential".to_string(),
        "motorway_link".to_string(),
        "trunk_link".to_string(),
        "primary_link".to_string(),
        "secondary_link".to_string(),
        "tertiary_link".to_string(),
        "living_street".to_string(),
        "unclassified".to_string(),
    ]);
    let filter_subset = FilterSubset::Roads;

    FilterSet {
        filter_key,
        filter_values,
        filter_subset,
    }
}

pub fn filter_stations() -> FilterSet {
    let filter_key = "public_transport".to_string();
    let filter_values = HashSet::from(["station".to_string()]);
    let filter_subset = FilterSubset::Landmark("stations".to_string());

    FilterSet {
        filter_key,
        filter_values,
        filter_subset,
    }
}

pub fn filter_amenities() -> FilterSet {
    let filter_key = "amenity".to_string();
    let filter_values = HashSet::from([
        "kindergarten".to_string(),
        "hospital".to_string(),
        "school".to_string(),
        "university".to_string(),
    ]);
    let filter_subset = FilterSubset::Landmark("amenity".to_string());

    FilterSet {
        filter_key,
        filter_values,
        filter_subset,
    }
}

pub fn filter_bus_stops() -> FilterSet {
    let filter_key: String = "highway".to_string();
    let filter_values = HashSet::from(["bus_stop".to_string()]);
    let filter_subset = FilterSubset::Landmark("bus_stop".to_string());

    FilterSet {
        filter_key,
        filter_values,
        filter_subset,
    }
}
