#!/usr/bin/env bash

#============================================================
# Utility script for publishing crates.
#============================================================

# Published version number.
PUBLISHED_VERSION="0.0.6"

publish() {
  echo "┌─────────────────────────────────────────────────────────────────────"
  echo "│ Publishing $1 ($3/18) "
  echo "└─────────────────────────────────────────────────────────────────────"
  cargo update
  cd "$1" && cargo publish && cd ..
  if [ "$1" != "dsntk" ]; then
    sed -i -E "s/$2 = .+/$2 = \"$PUBLISHED_VERSION\"/g" Cargo.toml
  fi
  echo "┌─────────────────────────────────────────────────────────────────────"
  echo "│ Published $1 ($3/18) "
  echo "└─────────────────────────────────────────────────────────────────────"
  echo "...sleeping 20s..."
  sleep 20s
}

publish_all() {
  publish "examples"          "dsntk-examples"             1
  publish "macros"            "dsntk-macros"               2
  publish "common"            "dsntk-common"               3
  publish "feel-regex"        "dsntk-feel-regex"           4
  publish "feel-number"       "dsntk-feel-number"          5
  publish "feel-temporal"     "dsntk-feel-temporal"        6
  publish "feel-grammar"      "dsntk-feel-grammar"         7
  publish "feel"              "dsntk-feel"                 8
  publish "feel-parser"       "dsntk-feel-parser"          9
  publish "feel-evaluator"    "dsntk-feel-evaluator"      10
  publish "model"             "dsntk-model"               11
  publish "recognizer"        "dsntk-recognizer"          12
  publish "model-evaluator"   "dsntk-model-evaluator"     13
  publish "evaluator"         "dsntk-evaluator"           14
  publish "gendoc"            "dsntk-gendoc"              15
  publish "workspace"         "dsntk-workspace"           16
  publish "server"            "dsntk-server"              17
  publish "dsntk"             "dsntk"                     18
}

publish_all
