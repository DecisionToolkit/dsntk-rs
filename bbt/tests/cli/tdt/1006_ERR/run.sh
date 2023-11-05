#!/usr/bin/env bash

dsntk tdt -c never input.test input.dtb 2>&1

dsntk tdt input.test input.dtb 2>&1 | sed -r 's/\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[m|K]//g'
