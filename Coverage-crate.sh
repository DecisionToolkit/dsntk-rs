#!/usr/bin/env bash

# Generate the coverage report.
cargo +nightly llvm-cov -p "$1" --no-cfg-coverage --no-cfg-coverage-nightly --html

# Display the link to coverage report.
echo -e "\n\e[1;32mOpen coverage report\x3A\e[0m file://$(pwd)/target/llvm-cov/html/index.html\n"
