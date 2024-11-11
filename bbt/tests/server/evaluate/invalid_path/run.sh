#!/usr/bin/env bash

dsntk srv -D . > /dev/null 2>&1 &
_pid=$!
sleep 0.1

curl -s -d '{"Full Name":"John Doe"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evaluate/org/decision-toolkit/invalid/Greeting%20Message
echo ""

kill -s SIGINT "$_pid"
sleep 0.1
