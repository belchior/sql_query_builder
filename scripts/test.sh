#!/bin/sh

cargo test --features postgresql --test cargo_feature_*
cargo test
