#!/usr/bin/env bash

DSNTK_HOST=127.0.0.1 dsntk srv -c never 2>&1 &
_pid=$!
sleep 0.1

kill -s SIGINT "$_pid"
sleep 0.1
