#!/bin/sh

alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src  -v cargo-git:/home/rust/.cargo/git -v cargo-registry:/home/rust/.cargo/registry ekidd/rust-musl-builder'
rust-musl-builder sudo chown -R rust:rust /home/rust/.cargo/git /home/rust/.cargo/registry
rust-musl-builder cargo build --release

wsk action delete putMeasurement -i
wsk action delete sendRealTimeMsg -i
wsk action delete logMeasurement -i
wsk action delete dispatchAlarm -i
wsk action delete sendTelegramMsg -i
wsk action delete alarmMeasurement -i
wsk rule delete dBProccess -i
wsk rule delete alarmProccess -i
wsk trigger delete commandSaveMeasurement -i
wsk action delete fetchData -i

mkdir ./temp
cd temp

#Action: Save Measurement to DynamoDb
cp ../target/x86_64-unknown-linux-musl/release/save_data ./exec
zip save_data.zip ./exec
wsk action create putMeasurement ./save_data.zip --native --param AWS_KEY $AWS_ACCESS_KEY_ID --param AWS_SECRET $AWS_SECRET_ACCESS_KEY  -i

#Action: Send message via RealTime
cp ../target/x86_64-unknown-linux-musl/release/send_realtime_msg ./exec
zip send_realtime_msg.zip ./exec
wsk action create sendRealTimeMsg ./send_realtime_msg.zip --native --param RT_APP $RT_APP --param RT_SECRET $RT_SECRET -i

#Sequence action save_msg -> send_realTime_msg
wsk action create logMeasurement --sequence putMeasurement,sendRealTimeMsg -i

#Action: Check alarm value
cp ../target/x86_64-unknown-linux-musl/release/alarm ./exec
zip alarm.zip ./exec
wsk action create dispatchAlarm ./alarm.zip --native --param AWS_KEY $AWS_ACCESS_KEY_ID --param AWS_SECRET $AWS_SECRET_ACCESS_KEY --param TRIGGER_VALUE \"45\" -i

#Action: Send telegram message
cp ../target/x86_64-unknown-linux-musl/release/publish_telegram ./exec
zip publish_telegram.zip ./exec
wsk action create sendTelegramMsg ./publish_telegram.zip --native --param API_KEY $TELEGRAM_API_KEY --param CHANNEL_ID   \"$TELEGRAM_CHANNEL\" -i

#Sequence action check_alarm -> send_telegram_msg
wsk action create alarmMeasurement --sequence dispatchAlarm,sendTelegramMsg -i

#Trigger to run in parallel save in database an check alarms
wsk trigger create commandSaveMeasurement -i
wsk rule create dBProccess commandSaveMeasurement logMeasurement -i
wsk rule create alarmProccess commandSaveMeasurement alarmMeasurement -i

#Create action for fetch_data from DynamoDb
cp ../target/x86_64-unknown-linux-musl/release/fetch_data ./exec
zip fetch_data.zip ./exec
wsk action create fetchData ./fetch_data.zip --native --param AWS_KEY $AWS_ACCESS_KEY_ID --param AWS_SECRET $AWS_SECRET_ACCESS_KEY --param minutes 60 -i

