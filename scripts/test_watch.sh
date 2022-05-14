#!/bin/sh

# cargo watch -w ./src -w ./tests -x 'test -- --nocapture --color always'
cargo watch -w ./src -w ./tests -x 'test'
