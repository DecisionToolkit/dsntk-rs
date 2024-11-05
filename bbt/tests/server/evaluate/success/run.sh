#!/usr/bin/env bash

dsntk srv -D . > /dev/null 2>&1 &
_pid=$!
sleep 0.1

curl -s -d '{"Full Name":"John Doe"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/org/decision-toolkit/2_0001/compliance-level-2-test-0001/Greeting%20Message
curl -s -d '{"Monthly Salary":12000}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/org/decision-toolkit/2_0002/compliance-level-2-test-0002/Yearly%20Salary

kill -s SIGINT "$_pid"
sleep 0.1
