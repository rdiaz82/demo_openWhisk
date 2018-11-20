extern crate openssl_probe;
extern crate rustc_serialize;
extern crate serde;
extern crate rusoto_dynamodb;
extern crate rusoto_core;
extern crate chrono;
extern crate time;
extern crate serde_json;
extern crate uuid;

use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput, GetItemInput};
use std::collections::HashMap;

use models::{DbData, Output, AlarmInformation};
use common::set_env;

pub fn update_alarm_status(alarm_register:&AlarmInformation, status:bool){
    let client = DynamoDbClient::new(Region::UsEast2);
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(
        "alarmId".to_string(),
        AttributeValue {
            s: Some(alarm_register.clone().name),
            ..Default::default()
        },
    );
    item.insert(
        "trigger".to_string(),
        AttributeValue {
            n: Some(alarm_register.clone().trigger.to_string()),
            ..Default::default()
        },
    );
    item.insert(
        "trigger_msg".to_string(),
        AttributeValue {
            s: Some(alarm_register.clone().fire_message),
            ..Default::default()
        },
    );
    item.insert(
        "normal_msg".to_string(),
        AttributeValue {
            s: Some(alarm_register.clone().normal_message),
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

pub fn get_alarm_information(sensor:&String) ->Option<AlarmInformation>{
    let client = DynamoDbClient::new(Region::UsEast2);
    let mut query_key: HashMap<String, AttributeValue> = HashMap::new();
    query_key.insert(
        "alarmId".to_string(),
        AttributeValue {
            s: Some(sensor.to_string()),
            ..Default::default()
        },
    );
    let query_alarm = GetItemInput {
        key: query_key,
        table_name: "AlarmsSetting".to_string(),
        ..Default::default()
    };
    match client.get_item(query_alarm).sync() {
        Ok(result) => {
            let status = result.clone().item.unwrap().get(&"value".to_string()).unwrap().s.clone().unwrap() == "true".to_string();

            let trigger = result.clone().item.unwrap().get(&"trigger".to_string()).unwrap().n.clone().unwrap().parse::<f32>().unwrap();

            return Some(AlarmInformation{
                name:  result.clone().item.unwrap().get(&"alarmId".to_string()).unwrap().s.clone().unwrap(),
                status: status,
                trigger: trigger,
                fire_message: result.clone().item.unwrap().get(&"trigger_msg".to_string()).unwrap().s.clone().unwrap(),
                normal_message:  result.clone().item.unwrap().get(&"normal_msg".to_string()).unwrap().s.clone().unwrap()
            });
        },
        Err(error) => {
            println!("{}",error); return None;},
    }
}
