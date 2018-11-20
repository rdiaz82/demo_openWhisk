extern crate rustc_serialize;
extern crate core;

use std::env;
use rustc_serialize::json::Json;


fn main() {
    let mut sensor:String = "".to_string();
    let mut minutes:i64 = 60;
    if let Some(arg1) = env::args().nth(1) {
        let params = Json::from_str(&arg1).unwrap();
        if let Some(params_obj) = params.as_object() {
            if let Some(params_name) = params_obj.get("sensor") {
                sensor = params_name.as_string().unwrap().to_string();
            }
            if let Some(params_name) = params_obj.get("minutes") {
                minutes = params_name.as_i64().unwrap();
            }
        }
    };
    println!("{{\"result\":{}}}",  core::log_repository::get_data_from_db(sensor, minutes));
}


