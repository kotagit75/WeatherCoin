use std::{env::current_dir, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Beacon {
    pub value: i32,
}

fn get_temperature(lon: f64, lat: f64) -> Option<i32> {
    match current_dir() {
        Ok(x) => {
            let status = Command::new("beacon/temperature.sh")
                .arg(lat.to_string())
                .arg(lon.to_string())
                .current_dir(x)
                .status();
            status.ok().and_then(|status| status.code())
        }
        Err(_) => None,
    }
}

pub fn get_beacon(history: &[Beacon]) -> Option<Beacon> {
    let positions = [
        (130.4016888888889, 33.59018333333334),
        (140.7290611111111, 41.76869722222222),
        (139.6379638888889, 35.443675),
        (135.76815, 35.01156388888889),
        (140.8694166666667, 38.26819444444445),
        (135.1956305555556, 34.69008055555555),
        (136.9065583333334, 35.18145),
        (132.4553055555556, 34.38528888888889),
        (140.4845583333333, 40.599217),
        (130.55, 31.5667),
    ];
    let sum: i32 = positions
        .map(|pos| get_temperature(pos.0, pos.1))
        .iter()
        .flatten()
        .sum();
    Some(Beacon {
        value: sum + history.iter().map(|b| b.value).sum::<i32>(),
    })
}
