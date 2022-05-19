#!/bin/sh

# Prerequisites
# cargo install rustfilt cargo-binutils
# rustup component add llvm-tools-preview

PKG_NAME="$(grep 'name\s*=\s*"' Cargo.toml | sed -E 's/.*"(.*)"/\1/')"
COVERAGE_OUTPUT="coverage"
COVERAGE_TARGET="target/coverage"

mkdir -p "$COVERAGE_OUTPUT"
mkdir -p "$COVERAGE_TARGET"
rm -f "$COVERAGE_TARGET/$PKG_NAME"*
rm -f "$COVERAGE_TARGET/debug/deps/$PKG_NAME"*

RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="$COVERAGE_TARGET/$PKG_NAME-%m.profraw" cargo test --target-dir $COVERAGE_TARGET;

cargo profdata -- merge -sparse $COVERAGE_TARGET/$PKG_NAME-*.profraw -o $COVERAGE_TARGET/$PKG_NAME.profdata;

OBJECT_PATH_LIB="$(ls $COVERAGE_TARGET/debug/deps/$PKG_NAME-???????????????? | xargs -I {} echo '--object {}')"
OBJECT_PATH_TEST="$(ls $COVERAGE_TARGET/debug/deps/*_spec-???????????????? | xargs -I {} echo '--object {}')"

cargo cov -- report \
    --use-color \
    --ignore-filename-regex='/rustc' \
    --ignore-filename-regex='/?cargo/registry' \
    --ignore-filename-regex='./tests/.*rs$' \
    --instr-profile=$COVERAGE_TARGET/$PKG_NAME.profdata \
    $OBJECT_PATH_LIB \
    $OBJECT_PATH_TEST;

cargo cov -- show \
    --use-color \
    --ignore-filename-regex='/rustc' \
    --ignore-filename-regex='/?cargo/registry' \
    --ignore-filename-regex='./tests/.*rs$' \
    --instr-profile=$COVERAGE_TARGET/$PKG_NAME.profdata \
    $OBJECT_PATH_LIB \
    $OBJECT_PATH_TEST \
    --show-instantiations \
    --show-line-counts-or-regions \
    --Xdemangler=rustfilt \
    --output-dir=$COVERAGE_OUTPUT \
    --format=html;

echo "\n\nAll files can be found at:\n$(pwd)/$COVERAGE_OUTPUT/index.html\n\n";

# Reference
# https://doc.rust-lang.org/stable/rustc/instrument-coverage.html
