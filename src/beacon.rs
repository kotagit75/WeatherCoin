use geojson::{FeatureCollection, GeometryValue};
use std::{env::current_dir, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Beacon {
    pub value: f32,
}

fn get_temperature(lon: f64, lat: f64) -> Option<f32> {
    match current_dir() {
        Ok(x) => {
            let status = Command::new("beacon/temperature.sh")
                .arg(lat.to_string())
                .arg(lon.to_string())
                .current_dir(x)
                .status();
            status
                .ok()
                .and_then(|status| status.code())
                .map(|code| code as f32 / 10.0)
        }
        Err(_) => None,
    }
}

pub fn get_beacon(history: &[Beacon]) -> Option<Beacon> {
    let Ok(collection) = include_str!("beacon/target.geojson").parse::<FeatureCollection>() else {
        return None;
    };
    let locations: Vec<geojson::Position> = collection
        .features
        .iter()
        .map(|feature| feature.geometry.clone())
        .flatten()
        .map(|geometry| match geometry.value {
            GeometryValue::Point { coordinates } => Some(coordinates),
            _ => None,
        })
        .flatten()
        .collect();
    let sum: f32 = locations
        .iter()
        .map(|pos| get_temperature(pos[0], pos[1]))
        .flatten()
        .sum();
    Some(Beacon {
        value: sum + history.iter().map(|b| b.value).sum::<f32>(),
    })
}

pub fn is_valid_beacon(target_beacon: &Beacon, history: &[Beacon]) -> bool {
    match get_beacon(history) {
        Some(beacon) => (beacon.value - target_beacon.value).abs() <= 0.5,
        None => false,
    }
}
