#!/bin/sh

# Prerequisites
# cargo install cargo-watch

# cargo watch -w ./src -w ./tests -x 'test --features postgresql -- --nocapture --color always'
cargo watch -w ./src -w ./tests -x 'test --features postgresql'
