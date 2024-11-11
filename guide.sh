#!/usr/bin/env bash

SRC_DIR="./guide/guide"
DST_DIR="../dsntk-docs/pages/guide"

REPLACEMENT_DSNTK='<span style="font-weight:bold;word-spacing:-0.15rem;">Decision Toolkit<\/span>'
REPLACEMENT_VERSION='0.0.9-dev'
REPLACEMENT_ON_THE_WAY='⏳ Detailed documentation is on the way.'

rm -rf "$DST_DIR"
cp -r "$SRC_DIR" "$DST_DIR"

find "$DST_DIR" -type f -name "*.md" -exec sed -i "s/#DSNTK/$REPLACEMENT_DSNTK/g" {} \;
find "$DST_DIR" -type f -name "*.md" -exec sed -i "s/#VERSION/$REPLACEMENT_VERSION/g" {} \;
find "$DST_DIR" -type f -name "*.md" -exec sed -i "s/#ON_THE_WAY/$REPLACEMENT_ON_THE_WAY/g" {} \;
