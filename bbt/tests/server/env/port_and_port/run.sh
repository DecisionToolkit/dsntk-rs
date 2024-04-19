#!/usr/bin/env bash

DSNTK_PORT=22033 dsntk srv -c never --port=22034 2>&1 &
_pid=$!
sleep 0.1

kill -s SIGINT "$_pid"
sleep 0.1
