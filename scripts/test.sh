#!/bin/sh

cargo test
cargo test --features postgresql --test feature_flag_postgresql
