#!/bin/sh

alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder cargo build --release

wsk action delete putMeasurement -i
wsk action delete sendRealTimeMsg -i
wsk action delete logMeasurement -i

#Action: Save Measurement to DynamoDb.
mkdir ./temp
cd temp

cp ../target/x86_64-unknown-linux-musl/release/save_data ./exec
zip save_data.zip ./exec
wsk action create putMeasurement ./save_data.zip --native --param AWS_KEY $AWS_ACCESS_KEY_ID --param AWS_SECRET $AWS_SECRET_ACCESS_KEY  -i

cp ../target/x86_64-unknown-linux-musl/release/send_realtime_msg ./exec
zip send_realtime_msg.zip ./exec
wsk action create sendRealTimeMsg ./send_realtime_msg.zip --native --param RT_APP $RT_APP --param RT_SECRET $RT_SECRET -i

wsk action create logMeasurement --sequence putMeasurement,sendRealTimeMsg -i
