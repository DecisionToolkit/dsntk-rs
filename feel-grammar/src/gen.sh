#!/usr/bin/env bash

rm -f feel.output
rm -f feel.tab.c

LANG=C bison -l -r states -L C feel.y
