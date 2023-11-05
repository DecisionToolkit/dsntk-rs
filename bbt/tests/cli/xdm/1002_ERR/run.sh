#!/usr/bin/env bash

rm -rf output.html

touch output.html

chmod -w output.html

dsntk xdm 2_0001.dmn output.html 2>&1

rm -rf output.html
