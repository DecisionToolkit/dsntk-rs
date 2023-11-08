#!/usr/bin/env bash

set -e

###############################################################################
#
# Dependencies:
#
# $ sudo dnf install lcov
# $ rustup component add llvm-tools-preview
# $ cargo install grcov
# $ cargo install htop
# $ cargo install cargo-nextest
#
# REMARKS:
#
# Run this script when the dependencies to particular crates are specified
# by path. For some reason, code coverage is not accurate when crates
# versions are in place.
#
###############################################################################

WORKING_DIRECTORY=$(pwd)
CARGO_NAME=$(grep -oE '^name = "[^"]+"' ./dsntk/Cargo.toml | grep -oE '"[^"]+"' | grep -oE '[^"]+' | tr '[:lower:]' '[:upper:]')
CARGO_VERSION=$(grep -oE '^version = "[^"]+"' Cargo.toml | grep -oE '"[^"]+"' | grep -oE '[0-9\.]+')

# clean before proceeding
cargo clean

# set instrumenting variables
export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=unwind"
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=unwind"

# set features for parsing tables
export CARGO_FEATURE_PARSING_TABLES=1

if [ -n "$1" ]; then
  # run tests only for specified package
  cargo +nightly nextest run -p "$1"
else
  # run all tests
  cargo +nightly nextest run
  # build the binary again
  cargo +nightly build
  # run black-box tests
  cd "$WORKING_DIRECTORY"/bbt || exit 1
  ./bbt.sh
  cd "$WORKING_DIRECTORY" || exit 1
  # give some time to collect all data
fi

# prepare output directories for coverage results
mkdir ./target/lcov
mkdir ./target/coverage

# generate coverage info
grcov . --llvm -s . -t lcov --ignore-not-existing \
     --excl-line='\s*\}+\)*;?$|\s*/\*\s*$|\s*\} else \{$|\s*\);$|\s*\},$|\s*\)\)+$' \
     --ignore "*cargo*" --ignore "*chrono-tz*" --ignore "*tests*" \
     -o ./target/lcov/lcov.info

# generate coverage report in HTML format
genhtml -t "$CARGO_NAME v$CARGO_VERSION" -q -o ./target/coverage ./target/lcov/lcov.info

# generate coverage report in PDF format
if [ "$PDF_REPORT" != "" ]; then
  echo ""
  echo "Generating PDF report..."
  htop -b -l -p A4 --margin=4mm single ./target/coverage/index.html ./target/coverage/coverage.pdf
fi

# display final message
echo ""
echo "Open coverage report:"
echo "  HTML file://$WORKING_DIRECTORY/target/coverage/index.html"
if [ "$PDF_REPORT" != "" ]; then
  echo "   PDF file://$WORKING_DIRECTORY/target/coverage/coverage.pdf"
fi
echo ""

# reformat generated code
cargo +nightly fmt -p dsntk-feel-parser
