#!/usr/bin/env bash

dsntk tdt -c never  input.test input.dtb 2>&1
dsntk tdt -c always input.test input.dtb 2>&1
