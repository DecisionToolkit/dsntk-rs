#!/usr/bin/env bash

rm -rf output.html

dsntk xfe input.ctx input.feel output.html

cat output.html

rm -rf output.html
