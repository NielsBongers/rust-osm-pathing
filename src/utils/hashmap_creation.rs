use std::path::Path;

use crate::data_handling::{FilterSet, OSMData};

use super::{
    file_handling::reload_and_save,
    filtering_utilities::{filter_amenities, filter_bus_stops, filter_highways, filter_stations},
};

pub fn recreate_hashmap(file_path: &Path, destination_path: &Path) -> OSMData {
    let highway_filter = filter_highways();
    let landmark_filter = FilterSet::default();
    let station_filter = filter_stations();
    let amenities_filter = filter_amenities();
    let bus_stops_filter = filter_bus_stops();
    let filters = vec![
        highway_filter,
        landmark_filter,
        station_filter,
        amenities_filter,
        bus_stops_filter,
    ];
    reload_and_save(file_path, destination_path, filters)
}
