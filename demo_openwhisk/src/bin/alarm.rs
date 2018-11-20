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
    core::common::set_env();
    let mut measurement = "0".to_string();
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
        }
    };
    let alarm_register:core::models::AlarmInformation = core::alarm_repository::get_alarm_information(&sensor).unwrap();
    let launch = f32::from_str(&measurement).unwrap() > alarm_register.trigger;
    let out;
    if launch && !alarm_register.status {
        out = core::models::Output {
            status: "true".to_string(),
            message: alarm_register.clone().fire_message,
            data: core::models::DbData{
                measurement: measurement.parse().unwrap(),
                sensor: sensor.clone(),
                timestamp: Utc::now().to_string(),
            },
        };
    } else if alarm_register.status && !launch {
        out = core::models::Output {
            status: "true".to_string(),
            message: alarm_register.clone().normal_message,
            data: core::models::DbData{
                measurement: measurement.parse().unwrap(),
                sensor: sensor.clone(),
                timestamp: Utc::now().to_string(),
            },
        };
    }
    else {
        out = core::models::Output {
            status: "false".to_string(),
            message: "".to_string(),
            data: core::models::DbData{
                measurement: measurement.parse().unwrap(),
                sensor: sensor.clone(),
                timestamp: Utc::now().to_string(),
            },
        };
    }
    core::alarm_repository::update_alarm_status(&alarm_register, launch);

    println!("{}", serde_json::to_string(&out).unwrap());
}
