#!/usr/bin/bash

set -uvex -o pipefail

cd $(dirname ${BASH_SOURCE[0]})/../

which typos || cargo install typos

BUILD_COMMON="--locked --release"

cargo clippy ${BUILD_COMMON} --all-targets --all-features -- -D warnings -D clippy::pedantic -A clippy::missing_errors_doc
cargo clippy ${BUILD_COMMON} --tests --all-targets --all-features -- -D warnings
cargo fmt --check
cargo build ${BUILD_COMMON}
cargo test ${BUILD_COMMON}
cargo run ${BUILD_COMMON} -- --account EMPTY readme --check
