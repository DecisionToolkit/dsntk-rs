#!/usr/bin/env bash

# container name
NAME=dsntk

# container version
VERSION=0.0.8-rc

# stop existing Docker container
docker stop $NAME

# remove stopped Docker container
docker rm $NAME

# remove existing Docker image
docker rmi "$(docker images | grep "^$NAME " | awk '{print $3}')"

# build the Decision Toolkit
cargo +stable build --release --target x86_64-unknown-linux-musl

# build new Docker image
docker build -t $NAME:$VERSION .

# start new Docker container
docker run --name $NAME -d -p 22022:22022 $NAME:$VERSION

# display logs from running Docker container
docker logs -f $NAME

# press Ctrl+C to stop following the log file ;-)
