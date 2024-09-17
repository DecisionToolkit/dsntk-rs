#!/usr/bin/env bash

dsntk srv -c always -v -P 11011 -H 127.0.0.1 -D . &
_pid=$!
sleep 0.1

kill -s SIGINT "$_pid"
sleep 0.1
