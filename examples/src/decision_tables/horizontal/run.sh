#!/usr/bin/env bash

dsntk edt --unicode "$1" "$2" | jq
dsntk edt --markdown "$1" "$2" | jq
