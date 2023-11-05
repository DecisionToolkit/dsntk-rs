#!/usr/bin/env bash

rm -rf examples

mkdir examples
mkdir examples/e1
touch examples/e1/e1.ctx

chmod -w examples/e1/e1.ctx

dsntk exs examples 2>&1

rm -rf examples
