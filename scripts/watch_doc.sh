#!/bin/sh

doc_path=$(realpath ./target/doc/sql_query_builder/index.html)
c_blue='\033[34;1m'
c_no='\033[0m'

echo "\nTo access the rendered documentation open the file below\n\n${c_blue}${doc_path}${c_no}\n"

RUSTDOCFLAGS="--cfg docsrs" cargo watch -w ./src -w ./tests -x "+nightly doc --all-features"
