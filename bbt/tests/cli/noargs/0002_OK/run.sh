#!/usr/bin/env bash

dsntk srv -c never 2>&1 &
_pid_a=$!
sleep 0.1

dsntk srv -c never 2>&1

kill -s SIGINT "$_pid_a"
sleep 0.1
