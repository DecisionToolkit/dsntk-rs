#!/usr/bin/env bash

dsntk srv -D . > /dev/null 2>&1 &
_pid=$!
sleep 0.1

dd if=/dev/urandom of=random.data bs=3M count=1 > /dev/null 2>&1

echo -n '{"content":"' > data.json
base64 --wrap=0 random.data >> data.json
echo -n '"}' >> data.json

curl -s -d '@data.json' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evaluate/io/dsntk/Greeting%20Message
echo ""

rm data.json
rm random.data

kill -s SIGINT "$_pid"
sleep 0.1
