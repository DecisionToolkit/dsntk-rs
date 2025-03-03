#!/usr/bin/env bash

rm -rf examples

mkdir -p examples/fe
touch examples/fe/fe.feel
chmod -w examples/fe/fe.feel
dsntk exs examples 2>&1

rm -rf examples
