#!/usr/bin/env bash

set -uvex -o pipefail

BUILD_TARGET=${1:-""}
BUILD_PROFILE=${2:-"release"}

cd $(dirname ${BASH_SOURCE[0]})/../

which typos || cargo install typos-cli
which taplo || cargo install taplo-cli

BUILD_COMMON="--locked --profile ${BUILD_PROFILE}"
if [ x"${BUILD_TARGET}" != x"" ]; then
    BUILD_COMMON="${BUILD_COMMON} --target ${BUILD_TARGET}"
fi

typos
taplo fmt -o column_width=300 -o reorder_keys=true -o reorder_arrays=true -o array_trailing_comma=false
cargo clippy ${BUILD_COMMON} --all-targets --all-features -- -D warnings -D clippy::pedantic -A clippy::missing_errors_doc
cargo clippy ${BUILD_COMMON} --tests --all-targets --all-features -- -D warnings
cargo fmt --check
cargo build ${BUILD_COMMON}
cargo test ${BUILD_COMMON}
cargo run ${BUILD_COMMON} -- --account EMPTY readme > README.md
git diff --exit-code
