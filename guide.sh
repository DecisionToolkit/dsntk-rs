#!/usr/bin/env bash

REPLACEMENT_DSNTK='<span style="font-weight:bold;word-spacing:-0.15rem;">Decision Toolkit<\/span>'
REPLACEMENT_VERSION='0.0.9-dev'
REPLACEMENT_V_VERSION='v0.0.9-dev'
REPLACEMENT_ON_THE_WAY='⏳ Detailed documentation is on the way.'

function replace() {
  find "$1" -type f -name "*.md" -exec sed -i "s/#DSNTK/$REPLACEMENT_DSNTK/g" {} \;
  find "$1" -type f -name "*.md" -exec sed -i "s/#VERSION/$REPLACEMENT_VERSION/g" {} \;
  find "$1" -type f -name "*.md" -exec sed -i "s/#ON_THE_WAY/$REPLACEMENT_ON_THE_WAY/g" {} \;
  find "$1" -type f -name "*.md" -exec sed -i "s/#V_VERSION/$REPLACEMENT_V_VERSION/g" {} \;
}

function replace_config() {
  find "$1" -type f -name "*.mts" -exec sed -i "s/#V_VERSION/$REPLACEMENT_V_VERSION/g" {} \;
}

SRC_DIR_PAGES="./guide/pages"
DST_DIR_PAGES="../dsntk-docs/pages"
rm -rf "$DST_DIR_PAGES"
cp -rf "$SRC_DIR_PAGES" "$DST_DIR_PAGES"
replace "$DST_DIR_PAGES"

SRC_DIR="./guide/"
DST_DIR_VITEPRESS="../dsntk-docs/.vitepress"
cp -rf "$SRC_DIR/config.txt" "$DST_DIR_VITEPRESS/config.mts"
replace_config "$DST_DIR_VITEPRESS"
