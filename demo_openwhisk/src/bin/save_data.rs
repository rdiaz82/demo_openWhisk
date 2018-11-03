extern crate core;
extern crate rustc_serialize;

use rustc_serialize::json::Json;
use std::env;

fn main() {
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
    println!("{}", core::save_measurement_to_db(measurement, sensor));
}
