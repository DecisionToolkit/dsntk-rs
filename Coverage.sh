#!/usr/bin/env bash

WORKING_DIRECTORY=$(pwd)

# Set the environment variables needed to get coverage.
source <(cargo llvm-cov show-env --export-prefix --no-cfg-coverage --no-cfg-coverage-nightly)

# Remove artifacts that may affect the coverage results.
# This command should be called after show-env.
cargo llvm-cov clean --workspace

# Build rust binaries.
cargo build

# Run all tests.
cargo test

# run black-box tests
cd "$WORKING_DIRECTORY"/bbt || exit 1
./bbt.sh
cd "$WORKING_DIRECTORY" || exit 1

# Generate the final coverage report.
cargo llvm-cov report --html

if [ "$PDF_REPORT" != "" ]; then
  # generate coverage report in PDF format
  echo -e "\nGenerating PDF report..."
  htop -b -l -p A4 -m 10mm -s 90% single ./target/llvm-cov/html/index.html ./target/llvm-cov/html/index.pdf
fi

# Display links to the coverage reports.
echo ""
echo -e "\e[1;32mOpen HTML report\x3A\e[0m file:///$WORKING_DIRECTORY/target/llvm-cov/html/index.html"
if [ "$PDF_REPORT" != "" ]; then
  echo -e "\e[1;32mOpen PDF report\x3A\e[0m file:///$WORKING_DIRECTORY/target/llvm-cov/html/index.pdf"
fi
echo ""

# Reformat code.
cargo +nightly fmt
