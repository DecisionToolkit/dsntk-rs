#!/usr/bin/env bash

###############################################################################
#                                                                             #
# Dependencies:                                                               #
#                                                                             #
# $ sudo dnf install lcov                                                     #
# $ rustup component add llvm-tools-preview                                   #
# $ cargo install grcov                                                       #
# $ cargo install htop                                                        #
#                                                                             #
###############################################################################

WORKING_DIRECTORY=$(pwd)
CARGO_NAME=$(grep -oE '^name = "[^"]+"' ./dsntk/Cargo.toml | grep -oE '"[^"]+"' | grep -oE '[^"]+' | tr '[:lower:]' '[:upper:]')
CARGO_VERSION=$(grep -oE '^version = "[^"]+"' Cargo.toml | grep -oE '"[^"]+"' | grep -oE '[0-9\.]+')

# clean before proceeding
cargo clean

# set instrumenting variables
export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=unwind"
export RUSTFLAGS="-Cinstrument-coverage -Ccodegen-units=1 -Copt-level=0 -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=unwind"
export LLVM_PROFILE_FILE="$WORKING_DIRECTORY/target/profraw/dsntk-%p-%m.profraw"

# set features for parsing tables
export CARGO_FEATURE_PARSING_TABLES=1

if [ -n "$1" ]; then
  # run tests only for specified package
  cargo +nightly test -p "$1"
else
  # run all tests
  cargo +nightly test --workspace
  # build the binary again
  cargo +nightly build --workspace
  # run black-box tests
  cd "$WORKING_DIRECTORY"/bbt || exit 1
  ./bbt.sh
  cd "$WORKING_DIRECTORY" || exit 1
fi

# prepare output directories for coverage results
mkdir ./target/lcov
mkdir ./target/coverage

# generate coverage info
grcov . \
     --llvm \
     -s . \
     --binary-path ./target/debug/ \
     -t lcov \
     --branch \
     --ignore-not-existing \
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

# format generated code
cargo +nightly fmt -p dsntk-feel-parser
