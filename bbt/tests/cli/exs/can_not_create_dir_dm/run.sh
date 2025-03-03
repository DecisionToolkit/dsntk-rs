#!/usr/bin/env bash

rm -rf examples

mkdir -p examples
touch examples/dm
dsntk exs examples 2>&1

rm -rf examples
