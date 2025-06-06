use super::{DepartureData, PublicTransportResult};
use chrono::{DateTime, Utc};
#[allow(unused)]
use log::{info, warn};
use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;

pub fn query_ns_api(
    start_station_name: &String,
    target_station_name: &String,
    time: &DateTime<Utc>,
) -> Result<Vec<DepartureData>, Box<dyn Error>> {
    let url: String = format!(
        "https://gateway.apiportal.ns.nl/reisinformatie-api/api/v3/trips?fromStation={}&toStation={}&originWalk=false&originBike=false&originCar=false&destinationWalk=false&destinationBike=false&destinationCar=false&dateTime={}&shorterChange=false&travelAssistance=false&searchForAccessibleTrip=false&localTrainsOnly=false&excludeHighSpeedTrains=false&excludeTrainsWithReservationRequired=false&discount=NO_DISCOUNT&travelClass=2&passing=false&travelRequestType=DEFAULT",
        start_station_name, target_station_name, time.to_rfc3339()
    ).replace(" ", "%20");

    let subscription_key = "3cce8b71a9c94892bce40ba5d7d05593";
    let client = Client::new();

    let response = client
        .get(&url)
        .header("Cache-Control", "no-cache")
        .header("Ocp-Apim-Subscription-Key", subscription_key)
        .send()?
        .text()?;

    let data: Value = serde_json::from_str(&response)?;

    let mut departures: Vec<DepartureData> = Vec::new();

    if let Some(trips) = data["trips"].as_array() {
        for trip in trips {
            let actual_duration = trip["actualDurationInMinutes"].as_f64().unwrap_or(0.0);
            let transfers = trip["transfers"].as_i64().unwrap_or(-1);
            let status = trip["status"].as_str().unwrap_or("unknown");

            if status != "NORMAL" {
                continue;
            }

            let mut departure_data = DepartureData::default();

            if let Some(legs) = trip["legs"].as_array() {
                for (index, leg) in legs.iter().enumerate() {
                    if index == 0 {
                        // Getting the departure time and direction for the first station
                        if let Some(planned_departure_str) =
                            leg["origin"]["plannedDateTime"].as_str()
                        {
                            if let Some(departure_direction) = leg["direction"].as_str() {
                                if let Ok(planned_departure) =
                                    planned_departure_str.parse::<DateTime<Utc>>()
                                {
                                    departure_data.duration = actual_duration * 60.0;
                                    departure_data.transfers = transfers;
                                    departure_data.time = planned_departure;
                                    departure_data.direction = departure_direction.to_string();
                                }
                            }
                        }
                    } else {
                        // Getting the series of stations
                        let origin_station = &leg["origin"]["name"].to_string().replace("\"", "");

                        departure_data
                            .intermediate_stations
                            .push(origin_station.clone())
                    }
                }
            }
            departures.push(departure_data);
        }
    } else {
        warn!(
            "Incorrect trip: {:?} for {} -> {}",
            data, start_station_name, target_station_name
        );
    }

    Ok(departures)
}

pub fn get_next_train(
    start_station_id: u64,
    end_station_id: u64,
    start_station_name: &String,
    target_station_name: &String,
    time: &DateTime<Utc>,
) -> Option<PublicTransportResult> {
    if let Ok(train_departures) = query_ns_api(start_station_name, target_station_name, &time) {
        for departure in train_departures.iter() {
            let waiting_time = (departure.time - time).num_seconds() as f64; // seconds

            let journey_duration = departure.duration;
            let direction = departure.direction.clone();

            let transfers = departure.transfers;
            let intermediate_stations = departure.intermediate_stations.clone();

            if waiting_time > 0.0 {
                let transport_result = PublicTransportResult::new(
                    start_station_id,
                    end_station_id,
                    journey_duration,
                    waiting_time,
                    direction,
                    transfers,
                    intermediate_stations,
                );

                return Some(transport_result);
            }
        }
    } else {
        warn!(
            "Incorrect trip between {} -> {}",
            start_station_name, target_station_name
        );
    }
    None
}
