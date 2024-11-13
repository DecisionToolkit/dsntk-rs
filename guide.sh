#!/usr/bin/env bash

###############################################################################
#                                                                             #
# Utility script for copying documentation files to VitePress project.        #
#                                                                             #
###############################################################################

CARGO_VERSION=$(grep -oE '^version = "[^"]+"' Cargo.toml | grep -oE '"[^"]+"' | grep -oE '[0-9\.]+')

REPLACEMENT_DSNTK='<span style="font-weight:bold;word-spacing:-0.15rem;">Decision Toolkit<\/span>'
REPLACEMENT_VERSION="$CARGO_VERSION"
REPLACEMENT_V_VERSION="v$CARGO_VERSION"
REPLACEMENT_ON_THE_WAY="⏳ Detailed documentation is on the way."

function replace() {
  find "$1" -type f -name "*.md" -exec sed -i "s/#DSNTK/$REPLACEMENT_DSNTK/g" {} \;
  find "$1" -type f -name "*.md" -exec sed -i "s/#VERSION/$REPLACEMENT_VERSION/g" {} \;
  find "$1" -type f -name "*.md" -exec sed -i "s/#V_VERSION/$REPLACEMENT_V_VERSION/g" {} \;
  find "$1" -type f -name "*.md" -exec sed -i "s/#ON_THE_WAY/$REPLACEMENT_ON_THE_WAY/g" {} \;
  find "$1" -type f -name "*.mts" -exec sed -i "s/#V_VERSION/$REPLACEMENT_V_VERSION/g" {} \;
}

SRC_DIR_PAGES="./guide/pages"
DST_DIR_PAGES="../dsntk-docs/pages"
rm -rf "$DST_DIR_PAGES"
cp -rf "$SRC_DIR_PAGES" "$DST_DIR_PAGES"
replace "$DST_DIR_PAGES"

SRC_DIR="./guide/"
DST_DIR_VITEPRESS="../dsntk-docs/.vitepress"
cp -rf "$SRC_DIR/config.mdx" "$DST_DIR_VITEPRESS/config.mts"
replace "$DST_DIR_VITEPRESS"
