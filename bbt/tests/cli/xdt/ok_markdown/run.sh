#!/usr/bin/env bash

rm -rf output.html

dsntk xdt --markdown input.md output.html 2>&1

head -n 10 output.html

rm -rf output.html
