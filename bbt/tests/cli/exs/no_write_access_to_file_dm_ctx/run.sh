#!/usr/bin/env bash

rm -rf examples

mkdir -p examples/dm
touch examples/dm/dm.ctx
chmod -w examples/dm/dm.ctx
dsntk exs examples 2>&1

rm -rf examples
