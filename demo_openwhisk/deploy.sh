#!/bin/sh

alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src  -v cargo-git:/home/rust/.cargo/git -v cargo-registry:/home/rust/.cargo/registry ekidd/rust-musl-builder'
rust-musl-builder sudo chown -R rust:rust /home/rust/.cargo/git /home/rust/.cargo/registry
rust-musl-builder cargo build --release

INVOKER="wsk"

if [ $1 == "bluemix" ]
then
    INVOKER="bx wsk"
fi

echo "Hello $INVOKER - hope you're well."
$INVOKER action delete putMeasurement -i
$INVOKER action delete sendRealTimeMsg -i
$INVOKER action delete logMeasurement -i
$INVOKER action delete dispatchAlarm -i
$INVOKER action delete sendTelegramMsg -i
$INVOKER action delete alarmMeasurement -i
$INVOKER rule delete dBProccess -i
$INVOKER rule delete alarmProccess -i
$INVOKER trigger delete commandSaveMeasurement -i
$INVOKER action delete fetchData -i

mkdir ./temp
cd temp

#Action: Save Measurement to DynamoDb
cp ../target/x86_64-unknown-linux-musl/release/save_data ./exec
zip save_data.zip ./exec
$INVOKER action create putMeasurement ./save_data.zip --native --param AWS_KEY $AWS_ACCESS_KEY_ID --param AWS_SECRET $AWS_SECRET_ACCESS_KEY  


#Action: Send message via RealTime
cp ../target/x86_64-unknown-linux-musl/release/send_realtime_msg ./exec
zip send_realtime_msg.zip ./exec
$INVOKER action create sendRealTimeMsg ./send_realtime_msg.zip --native --param RT_APP $RT_APP --param RT_SECRET $RT_SECRET -i

#Sequence action save_msg -> send_realTime_msg
$INVOKER action create logMeasurement --sequence putMeasurement,sendRealTimeMsg -i

#Action: Check alarm value
cp ../target/x86_64-unknown-linux-musl/release/alarm ./exec
zip alarm.zip ./exec
$INVOKER action create dispatchAlarm ./alarm.zip --native --param AWS_KEY $AWS_ACCESS_KEY_ID --param AWS_SECRET $AWS_SECRET_ACCESS_KEY --param TRIGGER_VALUE \"45\" -i

#Action: Send telegram message
cp ../target/x86_64-unknown-linux-musl/release/publish_telegram ./exec
zip publish_telegram.zip ./exec
$INVOKER action create sendTelegramMsg ./publish_telegram.zip --native --param API_KEY $TELEGRAM_API_KEY --param CHANNEL_ID   \"$TELEGRAM_CHANNEL\" -i

#Sequence action check_alarm -> send_telegram_msg
$INVOKER action create alarmMeasurement --sequence dispatchAlarm,sendTelegramMsg -i

#Trigger to run in parallel save in database an check alarms
$INVOKER trigger create commandSaveMeasurement -i
$INVOKER rule create dBProccess commandSaveMeasurement logMeasurement -i
$INVOKER rule create alarmProccess commandSaveMeasurement alarmMeasurement -i

#Create action for fetch_data from DynamoDb
cp ../target/x86_64-unknown-linux-musl/release/fetch_data ./exec
zip fetch_data.zip ./exec
$INVOKER action create fetchData ./fetch_data.zip --native --param AWS_KEY $AWS_ACCESS_KEY_ID --param AWS_SECRET $AWS_SECRET_ACCESS_KEY --param minutes 60 -i

