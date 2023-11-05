#!/usr/bin/env bash

dsntk srv -c never -H 'invalid host name' 2>&1 &
_pid=$!
sleep 0.1

kill -s SIGINT "$_pid"
sleep 0.1
