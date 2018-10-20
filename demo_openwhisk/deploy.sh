#!/bin/sh

#Action: Save Measurement to DynamoDb.
mkdir ./temp
cd temp
cp ./target/x86_64-unknown-linux-musl/release/main ./exec
zip save_data.zip ./exec
wsk action create putMeasurement ./save_data.zip --native --param AWS_KEY $AWS_ACCESS_KEY_ID --param AWS_SECRET $AWS_SECRET_ACCESS_KEY  -i
