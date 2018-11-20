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
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, QueryInput, PutItemInput};
use std::collections::HashMap;
use uuid::Uuid;
use models::{DbData, Output};
use common::set_env;


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
    let output_object = fetched_data.iter().map(|x|DbData {
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
