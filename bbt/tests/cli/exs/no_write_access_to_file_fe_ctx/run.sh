#!/usr/bin/env bash

rm -rf examples

mkdir -p examples/fe
touch examples/fe/fe.ctx
chmod -w examples/fe/fe.ctx
dsntk exs examples 2>&1

rm -rf examples
