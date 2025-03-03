#!/usr/bin/env bash

rm -rf examples

mkdir -p examples/fe
chmod -w examples/fe
dsntk exs examples 2>&1

rm -rf examples
