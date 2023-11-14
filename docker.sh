#!/usr/bin/env bash

# set variables
NAME=dsntk
VERSION=0.0.3

# clean before proceeding
docker stop $NAME
docker rm $NAME
docker rmi "$(docker images | grep "$NAME " | awk '{print $3}')"

# build the Decision Toolkit
cargo +stable build --release --target x86_64-unknown-linux-musl

# build the Docker image
docker build -t $NAME:$VERSION .

# start the Docker container
docker run --name $NAME -d -p 22022:22022 $NAME:$VERSION
