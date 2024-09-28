#!/bin/sh

test_names=$(git status -s | grep tests/ | sed -e 's/.* //' -e 's/tests\//--test /' -e 's/.rs//' | tr '\n' ' ')

clear
cargo test $test_names
cargo test $test_names --features postgresql
cargo test $test_names --features sqlite

# run only one test
# cargo test --features sqlite --test name_of_the_test_file name_of_the_test -- --nocapture --color always
