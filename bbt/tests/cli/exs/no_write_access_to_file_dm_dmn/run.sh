#!/usr/bin/env bash

rm -rf examples

mkdir -p examples/dm
touch examples/dm/dm.dmn
chmod -w examples/dm/dm.dmn
dsntk exs examples 2>&1

rm -rf examples
