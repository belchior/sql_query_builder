#!/bin/sh

# Prerequisites
# cargo install cargo-watch

# Usage
# ./scripts/watch_test.sh            # will run without enable feature
# ./scripts/watch_test.sh all        # will enable all feature
# ./scripts/watch_test.sh postgresql # will enable only the postgresql feature

all_features='postgresql sqlite'
features=''

case "$@" in
  "")    features="";;
  "all") features="$all_features";;
  *)     features="$@";;
esac

[ ! -z "$features" ] && features="--features $features"

# cargo watch -w ./src -w ./tests -x 'test --features postgresql -- --nocapture --color always'
cargo watch -w ./src -w ./tests -x "test $features"
