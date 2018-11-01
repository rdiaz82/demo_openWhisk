extern crate reqwest;
extern crate rustc_serialize;

use rustc_serialize::json::Json;
use std::env;

fn main() {
        let mut message = "".to_string();
    let mut status = "".to_string();
    let mut api_key ="".to_string();
    let mut channel_id = "".to_string();
        if let Some(arg1) = env::args().nth(1) {
            let params = Json::from_str(&arg1).unwrap();
            if let Some(params_obj) = params.as_object() {
                if let Some(params_name) = params_obj.get("status") {
                    status = params_name.as_string().unwrap().to_string();
                }
                if let Some(params_name) = params_obj.get("message") {
                    message = params_name.as_string().unwrap().to_string();
                }
                if let Some(params_name) = params_obj.get("CHANNEL_ID") {
                    channel_id = params_name.as_string().unwrap().to_string();
                }
                if let Some(params_name) = params_obj.get("API_KEY") {
                    api_key = params_name.as_string().unwrap().to_string();
                }
            }
        };
    if status != "true" {
        return;
    }
    let url = format!("{}{}{}{}{}{}", "https://api.telegram.org/bot",
                      api_key,
                      "/sendMessage?chat_id=",
                      channel_id,
                      "&text=",
                      message);
    let response = reqwest::get(&url);
    println!("{:?}", response);
}
