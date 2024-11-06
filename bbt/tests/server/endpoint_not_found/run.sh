#!/usr/bin/env bash

dsntk srv > /dev/null 2>&1 &
_pid=$!
sleep 0.1

curl -s -X TRACE 0.0.0.0:22022/tck/evaluate

kill -s SIGINT "$_pid"
sleep 0.1
