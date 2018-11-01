extern crate rustc_serialize;
extern crate core;
extern crate rusoto_dynamodb;
extern crate rusoto_core;
use rustc_serialize::json::Json;
use rustc_serialize::json;
use std::env;
use std::str::FromStr;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput, GetItemInput};
use std::collections::HashMap;

fn update_alarm_status(status:bool){
    let client = DynamoDbClient::new(Region::UsEast2);
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(
        "alarmId".to_string(),
        AttributeValue {
            s: Some("alarm1".to_string()),
            ..Default::default()
        },
    );
    item.insert(
        "value".to_string(),
        AttributeValue {
            s: Some(if status{"true".to_string()} else {"false".to_string()}),
            ..Default::default()
        },
    );

    let put_item = PutItemInput {
        table_name: "AlarmsSetting".to_string(),
        item: item,
        ..Default::default()
    };

    match client.put_item(put_item).sync() {
        Ok(_) => {}
        Err(_) => {}
    };
}

fn get_alarm_status() -> bool{
    let client = DynamoDbClient::new(Region::UsEast2);
    let mut query_key: HashMap<String, AttributeValue> = HashMap::new();
    query_key.insert(
        "alarmId".to_string(),
        AttributeValue {
            s: Some("alarm1".to_string()),
            ..Default::default()
        },
    );
    let query_alarm = GetItemInput {
        key: query_key,
        table_name: "AlarmsSetting".to_string(),
        ..Default::default()
    };
    match client.get_item(query_alarm).sync() {
        Ok(result) => {let current_status = result.item.unwrap().get(&"value".to_string()).unwrap().s.clone().unwrap();
        if current_status == "true".to_string(){ return true} else {return false}}
        Err(error) => {println!("{:?}",error); return false}
    }
}

fn main() {
    core::set_env();
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

    let launch = f32::from_str(&measurement).unwrap() > f32::from_str(&trigger_value).unwrap();
    let out;
    if launch && !get_alarm_status() {
        out = core::Output {
            status: "true".to_string(),
            message: format!("{} {}", "System in danger, measure value higher than:".to_string(), trigger_value.to_string()),
        };
    } else if get_alarm_status() && !launch {
        out = core::Output {
            status: "true".to_string(),
            message: format!("{} {}", "System has returned to normal status, measurement under:".to_string(), trigger_value.to_string()),
        };
    }
    else {
        out = core::Output {
            status: "false".to_string(),
            message: "".to_string(),
        };
    }
    update_alarm_status(launch);
    println!("{}", json::encode(&out).unwrap())
}
