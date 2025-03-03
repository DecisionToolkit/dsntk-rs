#!/usr/bin/env bash

rm -rf examples

mkdir -p examples
touch examples/dt
dsntk exs examples 2>&1

rm -rf examples
