#!/usr/bin/env bash

# set variables
NAME=dsntk
VERSION=0.0.1

# clean before proceeding
cargo clean
docker stop $NAME
docker rm $NAME
docker rmi "$(docker images | grep $VERSION | awk '{print $3}')"

# build the toolkit
cargo +stable build --release --target x86_64-unknown-linux-musl

# build the docker image
docker build -t $NAME:$VERSION .

# start the container
docker run --name $NAME -d -p 22022:22022 $NAME:$VERSION
