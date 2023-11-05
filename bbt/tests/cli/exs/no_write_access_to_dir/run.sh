#!/usr/bin/env bash

rm -rf examples

mkdir examples

chmod -w examples

dsntk exs examples 2>&1

rm -rf examples
