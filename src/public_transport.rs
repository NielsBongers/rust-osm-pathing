use chrono::{DateTime, Utc};

pub mod ns_api;
pub mod public_transport;
pub mod stations;

#[derive(Debug, Default)]
pub struct DepartureData {
    pub time: DateTime<Utc>,
    pub duration: f64,
    pub direction: String,
    pub transfers: i64,
    pub intermediate_stations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PublicTransportResult {
    pub start_station_id: u64,
    pub end_station_id: u64,
    pub waiting_time: f64,
    pub journey_duration: f64,
    pub direction: String,
    pub transfers: i64,
    pub intermediate_stations: Vec<String>,
}
