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

COVERAGE_ID="$(ls $COVERAGE_TARGET/debug/deps/$PKG_NAME-???????????????? | head -n1 | sed -E 's/.*-(.*)$/\1/')"

cargo cov -- report \
    --use-color \
    --ignore-filename-regex='/rustc' \
    --ignore-filename-regex='/.cargo/registry' \
    --ignore-filename-regex='./*_spec.rs$' \
    --instr-profile=$COVERAGE_TARGET/$PKG_NAME.profdata \
    --object $COVERAGE_TARGET/debug/deps/$PKG_NAME-$COVERAGE_ID;
    sql_query_builder-f6feeee0e0428c82

cargo cov -- show \
    --use-color \
    --ignore-filename-regex='/rustc' \
    --ignore-filename-regex='/.cargo/registry' \
    --ignore-filename-regex='./*_spec.rs$' \
    --instr-profile=$COVERAGE_TARGET/$PKG_NAME.profdata \
    --object $COVERAGE_TARGET/debug/deps/$PKG_NAME-$COVERAGE_ID \
    --show-instantiations \
    --show-line-counts-or-regions \
    --Xdemangler=rustfilt \
    --output-dir=$COVERAGE_OUTPUT \
    --format=html;

echo "\n\nAll files can be found at:\n$(pwd)/$COVERAGE_OUTPUT/index.html\n\n";

# Reference
# https://doc.rust-lang.org/stable/rustc/instrument-coverage.html
