#!/usr/bin/env bash

CURRENT_DIR=$(pwd)
PARENT_DIR=$(realpath "$CURRENT_DIR/..")
DSNTK_PATH=$PARENT_DIR/target/debug
DSNTK_BINARY=$DSNTK_PATH/dsntk

if [ ! -f "$DSNTK_BINARY" ] ; then
  echo "dsntk binary not found in path: $DSNTK_PATH"
  exit 1
fi

PATH=$DSNTK_PATH:$PATH ./run.sh "$@"
