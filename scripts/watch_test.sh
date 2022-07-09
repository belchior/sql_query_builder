#!/bin/sh

# cargo watch -w ./src -w ./tests -x 'test --features postgresql -- --nocapture --color always'
cargo watch -w ./src -w ./tests -x 'test --features postgresql --test cargo_feature_postgresql'
