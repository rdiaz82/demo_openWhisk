extern crate rustc_serialize;
extern crate core;
use rustc_serialize::json::Json;
use rustc_serialize::json;
use std::env;
use std::str::FromStr;

fn main() {
    let mut measurement = "0".to_string();
    let mut trigger_value = "0".to_string();
    if let Some(arg1) = env::args().nth(1) {
        let params = Json::from_str(&arg1).unwrap();
        if let Some(params_obj) = params.as_object() {
            if let Some(params_name) = params_obj.get("measurement") {
                measurement = params_name.as_string().unwrap().to_string();
            }
            if let Some(params_name) = params_obj.get("TRIGGER_VALUE") {
                trigger_value = params_name.as_string().unwrap().to_string();
            }
        }
    };
    if f32::from_str(&measurement).unwrap() > f32::from_str(&trigger_value).unwrap() {
        let out = core::Output {
            status: "ok".to_string(),
            message: "true".to_string(),
        };
        println!("{}", json::encode(&out).unwrap())

    } else {
        let out = core::Output {
            status: "ok".to_string(),
            message: "false".to_string(),
        };
        println!("{}", json::encode(&out).unwrap())
    }
}
