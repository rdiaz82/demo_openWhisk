extern crate rustc_serialize;
extern crate core;
extern crate rusoto_dynamodb;
extern crate rusoto_core;
extern crate chrono;
extern crate serde_json;

use chrono::prelude::*;
use rustc_serialize::json::Json;
use std::env;
use std::str::FromStr;

fn main() {
    core::set_env();
    let mut measurement = "0".to_string();
    let mut trigger_value = "0".to_string();
    let mut sensor = "".to_string();

    if let Some(arg1) = env::args().nth(1) {
        let params = Json::from_str(&arg1).unwrap();
        if let Some(params_obj) = params.as_object() {
            if let Some(params_name) = params_obj.get("measurement") {
                measurement = params_name.as_string().unwrap().to_string();
            }
            if let Some(params_name) = params_obj.get("sensor") {
                sensor = params_name.as_string().unwrap().to_string();
            }
            if let Some(params_name) = params_obj.get("TRIGGER_VALUE") {
                trigger_value = params_name.as_string().unwrap().to_string();
            }
        }
    };

    let launch = f32::from_str(&measurement).unwrap() > f32::from_str(&trigger_value).unwrap();
    let out;
    if launch && !core::get_alarm_status(&sensor) {
        out = core::Output {
            status: "true".to_string(),
            message: format!("System in danger, **{}** measure value higher than: **{}**", sensor, trigger_value.to_string()),
            data: core::DbData{
                measurement: measurement.parse().unwrap(),
                sensor: sensor.clone(),
                timestamp: Utc::now().to_string(),
            },
        };
    } else if core::get_alarm_status(&sensor) && !launch {
        out = core::Output {
            status: "true".to_string(),
            message: format!("System has returned to normal status, **{}** measurement under: **{}**",sensor, trigger_value.to_string()),
            data: core::DbData{
                measurement: measurement.parse().unwrap(),
                sensor: sensor.clone(),
                timestamp: Utc::now().to_string(),
            },
        };
    }
    else {
        out = core::Output {
            status: "false".to_string(),
            message: "".to_string(),
            data: core::DbData{
                measurement: measurement.parse().unwrap(),
                sensor: sensor.clone(),
                timestamp: Utc::now().to_string(),
            },
        };
    }
    core::update_alarm_status(&sensor, launch);

    println!("{}", serde_json::to_string(&out).unwrap());
}
