#!/bin/sh

clear
cargo test
cargo test --features postgresql
cargo test --features sqlite
