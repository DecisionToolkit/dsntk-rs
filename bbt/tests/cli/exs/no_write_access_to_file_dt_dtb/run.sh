#!/usr/bin/env bash

rm -rf examples

mkdir -p examples/dt
touch examples/dt/dt.dtb
chmod -w examples/dt/dt.dtb
dsntk exs examples 2>&1

rm -rf examples
