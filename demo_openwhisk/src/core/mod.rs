#[macro_use]
extern crate serde_derive;

extern crate openssl_probe;
extern crate rustc_serialize;
extern crate serde;
extern crate rusoto_dynamodb;
extern crate rusoto_core;
extern crate chrono;
extern crate time;
extern crate serde_json;
extern crate uuid;

use chrono::prelude::*;
use time::Duration;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, QueryInput, PutItemInput, GetItemInput};
use std::collections::HashMap;
use std::env;
use rustc_serialize::json::Json;
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
pub struct Output {
    pub message: String,
    pub status: String,
    pub data: DbData,
}

#[derive(Serialize, Deserialize)]
pub struct DbData {
    pub timestamp: String,
    pub sensor: String,
    pub measurement: f32,
}

pub fn set_env() {
    openssl_probe::init_ssl_cert_env_vars();
    if let Some(arg1) = env::args().nth(1) {
        let params = Json::from_str(&arg1).unwrap();
        if let Some(params_obj) = params.as_object() {
            if let Some(aws_key) = params_obj.get("AWS_KEY") {
                env::set_var(
                    "AWS_ACCESS_KEY_ID",
                    aws_key.as_string().unwrap().to_string(),
                );
            }
            if let Some(aws_secret) = params_obj.get("AWS_SECRET") {
                env::set_var(
                    "AWS_SECRET_ACCESS_KEY",
                    aws_secret.as_string().unwrap().to_string(),
                );
            }
        }
    };
}

pub fn get_data_from_db(sensor:String, minutes:i64) -> String{
    set_env();
    let client = DynamoDbClient::new(Region::UsEast2);
    let timestamp = Utc::now() - Duration::seconds(minutes*60);
    let mut attribute_name: HashMap<String, String> = HashMap::new();
    attribute_name.insert("#T".to_string(), "timestamp".to_string());

    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(
        ":time".to_string(),
        AttributeValue {
            s: Some(timestamp.to_string()),
            ..Default::default()
        },
    );
    item.insert(
        ":sensor".to_string(),
        AttributeValue {
            s: Some(sensor),
            ..Default::default()
        },
    );

    let query_historic = QueryInput {
        expression_attribute_names:Some(attribute_name),
        expression_attribute_values: Some(item),
        key_condition_expression: Some("sensor = :sensor".to_string()),
        filter_expression: Some("#T > :time".to_string()),
        table_name: "MeasurementLog".to_string(),
        index_name: Some("sensor-index".to_string()),
        ..Default::default()
    };
    let fetched_data = client.query(query_historic).sync().unwrap().items.unwrap();
    let output_object = fetched_data.iter().map(|x| DbData {
        timestamp: x.get("timestamp").unwrap().s.clone().unwrap(),
        sensor: x.get("sensor").unwrap().s.clone().unwrap(),
        measurement:  x.get("measurement").unwrap().n.clone().unwrap().parse().unwrap(),
    }).collect::<Vec<DbData>>();

    serde_json::to_string(&output_object).unwrap()
}

pub fn save_measurement_to_db(measurement:String, sensor:String) -> String{
    set_env(); 
    let client = DynamoDbClient::new(Region::UsEast2);
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(
        "uuid".to_string(),
        AttributeValue {
            s: Some(Uuid::new_v4().to_string()),
            ..Default::default()
        },
    );
    item.insert(
        "measurement".to_string(),
        AttributeValue {
            n: Some(measurement.clone()),
            ..Default::default()
        },
    );
    item.insert(
        "sensor".to_string(),
        AttributeValue {
            s: Some(sensor.clone()),
            ..Default::default()
        },
    );
    let timestamp = Utc::now().to_string();
    item.insert(
        "timestamp".to_string(),
        AttributeValue {
            s: Some(timestamp.clone()),
            ..Default::default()
        },
    );

    let put_item = PutItemInput {
        table_name: "MeasurementLog".to_string(),
        item: item,
        ..Default::default()
    };

    let out:Output;
    match client.put_item(put_item).sync() {
        Ok(_) => {
            out = Output {
                status: "ok".to_string(),
                message: "".to_string(),
                data: DbData {
                    measurement: measurement.parse().unwrap(),
                    timestamp: timestamp,
                    sensor: sensor,
                },
            };
        }
        Err(error) => {
            out = Output {
                status: "error".to_string(),
                message: error.to_string(),
                data: DbData {
                    measurement: measurement.parse().unwrap(),
                    timestamp: timestamp,
                    sensor: sensor,
                },
            };
        }
    };
    serde_json::to_string(&out).unwrap()
}

pub fn update_alarm_status(sensor:&String, status:bool){
    let client = DynamoDbClient::new(Region::UsEast2);
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(
        "alarmId".to_string(),
        AttributeValue {
            s: Some(sensor.to_string()),
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

pub fn get_alarm_status(sensor:&String) -> bool{
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
        Ok(result) => {let current_status = result.item.unwrap().get(&"value".to_string()).unwrap().s.clone().unwrap();
        if current_status == "true".to_string(){ return true} else {return false}}
        Err(error) => {println!("{:?}",error); return false}
    }
}

