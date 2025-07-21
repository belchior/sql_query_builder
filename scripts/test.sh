#!/bin/sh

test_names=$(git status -s | grep 'A[[:space:]]*tests/\|M[[:space:]]*tests/' | sed -e 's/.* //' -e 's/tests\//--test /' -e 's/\.rs//' | tr '\n' ' ')

clear
echo "\n-- ------------------------------------------------------------------------------"
echo "-- Testing SQL Standard"
echo "-- ------------------------------------------------------------------------------\n"
cargo test $test_names

echo "\n-- ------------------------------------------------------------------------------"
echo "-- Testing PostgreSQL syntax"
echo "-- ------------------------------------------------------------------------------\n"
cargo test $test_names --features postgresql

echo "\n-- ------------------------------------------------------------------------------"
echo "-- Testing SQLite syntax"
echo "-- ------------------------------------------------------------------------------\n"
cargo test $test_names --features sqlite
cargo test $test_names --features mysql

echo "\n-- ------------------------------------------------------------------------------"
echo "-- Testing MySQL syntax"
echo "-- ------------------------------------------------------------------------------\n"
cargo test $test_names --features mysql

# run only one test
# cargo test --features sqlite --test name_of_the_test_file name_of_the_test -- --nocapture --color always
