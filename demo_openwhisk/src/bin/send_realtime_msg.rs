extern crate reqwest;
extern crate rustc_serialize;
extern crate serde_json;

use serde_json::{Value};

use rustc_serialize::json::Json;
use std::env;


fn main() {
    let mut measurement = "0".to_string();
    let mut status = "".to_string();
    let mut rt_app ="".to_string();
    let mut rt_key = "".to_string();
    if let Some(arg1) = env::args().nth(1) {
        let json: Value = serde_json::from_str(&arg1).unwrap();
        measurement = serde_json::to_string(&json["data"]).unwrap();
        let params = Json::from_str(&arg1).unwrap();
        if let Some(params_obj) = params.as_object() {
            if let Some(params_name) = params_obj.get("status") {
                status = params_name.as_string().unwrap().to_string();
            }
            if let Some(params_name) = params_obj.get("RT_APP") {
                rt_app = params_name.as_string().unwrap().to_string();
            }
            if let Some(params_name) = params_obj.get("RT_SECRET") {
                rt_key = params_name.as_string().unwrap().to_string();
            }
        }
    };
    if status != "ok" {
        return;
    }
    println!("{}", measurement);
    reqwest::Client::new()
        .post("https://ortc-developers2-useast1-s0001.realtime.co/send")
        .form(&[("AK", rt_app), ("PK", rt_key), ("C","myChannel".to_string()), ("M",format!("3c261a88_1-1_{}", measurement))])
        .send()
        .unwrap();
}
