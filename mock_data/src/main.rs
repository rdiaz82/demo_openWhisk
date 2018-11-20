extern crate reqwest;
use std::collections::HashMap;
use std::env;
use std::{thread, time};

fn sendMeasurement(sensor: &str, measurement: &str) {
    let openwhisk_url;
    let openwhisk_user;
    let openwhisk_password;
    match env::var("OPENWHISK_URL") {
        Ok(val) => openwhisk_url = val,
        Err(_e) => return,
    }
    match env::var("OPENWHISK_USER") {
        Ok(val) => openwhisk_user = val,
        Err(_e) => return,
    }
    match env::var("OPENWHISK_PASSWORD") {
        Ok(val) => openwhisk_password = val,
        Err(_e) => return,
    }
    let mut map = HashMap::new();
    map.insert("sensor", sensor);
    map.insert("measurement", measurement);
    let client = reqwest::Client::new();
    let result = client
        .post(&openwhisk_url)
        .basic_auth(
            openwhisk_user.to_string(),
            Some(openwhisk_password.to_string()),
        ).json(&map)
        .send();
}

#[derive(PartialEq)]
enum state {
    FillTank1,
    HeatTank1,
    MaintainTempTank1,
    TransferTank2,
    HeatTank2,
    MaintainTempTank2,
    EmptyTank2
}

fn main() {
    let mut level_tank_1 = 0;
    let mut keep_temp_tank_1 = 0;
    let mut temp_tank_1 = 20;
    let mut level_tank_2 = 0;
    let mut keep_temp_tank_2 = 0;
    let mut temp_tank_2 = 20;
    let mut valve1 = 0;
    let mut valve2 = 0;
    let mut valve3 = 0;
    let mut valve4 = 0;
    let mut pump = 0;
    let mut heater1 = 0;
    let mut heater2 = 0;
    let mut current_state = state::FillTank1;

    sendMeasurement("sensor5","1");
    sendMeasurement("sensor6","0");
    sendMeasurement("sensor7","0");
    sendMeasurement("sensor8","0");
    sendMeasurement("sensor9","0");
    sendMeasurement("sensor10","0");
    println!("starting to fill tank 1");

    loop {
        match (current_state) {
            state::FillTank1 => {
                level_tank_1 += 10;
                if level_tank_1 >= 90 {
                    current_state = state::HeatTank1;
                    sendMeasurement("sensor5","0");
                    sendMeasurement("sensor9","1");
                    println!("starting to heat tank 1");


                }
            }
            state::HeatTank1 => {
                temp_tank_1 += 5;
                if temp_tank_1 >= 70 {
                    current_state = state::MaintainTempTank1;
                    sendMeasurement("sensor9","0");
                    println!("starting mashing process, maintaining temp in tank 1 for a while...");

                }
            }
            state::MaintainTempTank1 => {
                keep_temp_tank_1 += 1;
                if keep_temp_tank_1 == 7 {
                    current_state = state::TransferTank2;
                    sendMeasurement("sensor6","1");
                    sendMeasurement("sensor8","1");
                    keep_temp_tank_1 = 0;
                    println!("transfering wort to tank2");

                }
            }
            state::TransferTank2 => {
                level_tank_1 -= 10;
                level_tank_2 += 10;

                if level_tank_2 >= 90 {
                    sendMeasurement("sensor6","0");
                    sendMeasurement("sensor8","0");
                    sendMeasurement("sensor10","1");
                    current_state = state::HeatTank2;
                    println!("starting to boil the wort in tank2");
                }
            },
            state::HeatTank2 => {
                temp_tank_2 += 5;
                if temp_tank_2 >= 100 {
                    sendMeasurement("sensor10","0");
                    current_state = state::MaintainTempTank2;
                    println!("achieved boiling temp, keep it for a while...");
                }
            },
            state::MaintainTempTank2 =>{
                keep_temp_tank_2 += 1;
                if keep_temp_tank_2 == 7 {
                    current_state = state::EmptyTank2;
                    keep_temp_tank_2 = 0;
                    sendMeasurement("sensor7","1");
                    println!("Drain Tank 2, warning it's hot!! xD");
                }
            },
            state::EmptyTank2 =>{
                level_tank_2 -= 10;
                if level_tank_2 == 0 {
                    sendMeasurement("sensor7","0");
                    sendMeasurement("sensor5","1");
                    current_state = state::FillTank1;
                    println!("Let´s go for another batch, filling Tank 1");
                }
            }
        };

        if current_state != state::MaintainTempTank1 && current_state != state::HeatTank1 && current_state != state::TransferTank2 && temp_tank_1 > 20 {
            temp_tank_1 -= 10;
        }

        if current_state != state::MaintainTempTank2 && current_state != state::HeatTank2 && current_state != state::EmptyTank2 && temp_tank_2 > 20 {
            temp_tank_2 -= 10;
        }

        sendMeasurement("sensor1", &level_tank_1.to_string());
        sendMeasurement("sensor2", &level_tank_2.to_string());
        sendMeasurement("sensor3", &temp_tank_1.to_string());
        sendMeasurement("sensor4", &temp_tank_2.to_string());
        thread::sleep_ms(2000);
    }
}
