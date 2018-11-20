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
pub mod models;
pub mod log_repository;
pub mod alarm_repository;
pub mod common;

use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput, GetItemInput};
use std::collections::HashMap;


