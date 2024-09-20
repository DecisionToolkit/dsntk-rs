#!/usr/bin/env bash

dsntk srv -c never -P 11011 -H 127.0.0.1 -D d1 -D d2 &
_pid=$!
sleep 0.1

kill -s SIGINT "$_pid"
sleep 0.1
