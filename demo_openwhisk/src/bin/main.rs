extern crate core;

extern crate chrono;
extern crate openssl_probe;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate rustc_serialize;
extern crate uuid;

use chrono::prelude::*;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput};
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::collections::HashMap;
use std::default::Default;
use std::env;
use uuid::Uuid;

fn main() {
    core::set_env();
    println!("{:?}", env::var("AWS_SECRET_ACCESS_KEY"));
    let mut measurement = "0".to_string();
    if let Some(arg1) = env::args().nth(1) {
        let params = Json::from_str(&arg1).unwrap();
        if let Some(params_obj) = params.as_object() {
            if let Some(params_name) = params_obj.get("measurement") {
                measurement = params_name.as_string().unwrap().to_string();
            }
        }
    };

    let client = DynamoDbClient::new(Region::UsEast2);
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(
        "id".to_string(),
        AttributeValue {
            s: Some(Uuid::new_v4().to_string()),
            ..Default::default()
        },
    );
    item.insert(
        "measurement".to_string(),
        AttributeValue {
            n: Some(measurement),
            ..Default::default()
        },
    );
    item.insert(
        "timestamp".to_string(),
        AttributeValue {
            s: Some(Utc::now().to_string()),
            ..Default::default()
        },
    );

    let put_item = PutItemInput {
        table_name: "Measurements".to_string(),
        item: item,
        ..Default::default()
    };

    match client.put_item(put_item).sync() {
        Ok(_) => {
            let out = core::Output {
                message: "ok".to_string(),
            };
            println!("{}", json::encode(&out).unwrap())
        }
        Err(error) => {
            let out = core::Output {
                message: error.to_string(),
            };
            println!("{}", json::encode(&out).unwrap())
        }
    };
}

