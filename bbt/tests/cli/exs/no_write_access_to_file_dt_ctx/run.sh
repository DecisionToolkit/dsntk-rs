#!/usr/bin/env bash

rm -rf examples

mkdir -p examples/dt
touch examples/dt/dt.ctx
chmod -w examples/dt/dt.ctx
dsntk exs examples 2>&1

rm -rf examples
