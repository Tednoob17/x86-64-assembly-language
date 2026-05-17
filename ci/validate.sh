#!/bin/bash

set -eu

for file in src/*.md ; do
    echo Checking references in "$file"
    cargo run --quiet --manifest-path packages/tools/Cargo.toml --bin link2print < "$file" > /dev/null
done