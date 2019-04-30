#!/bin/bash

node collect_packages > ranges.txt
node bench_semver > semver_node_res.txt
cargo run --release -- --semver > semver_res.txt
cargo run --release -- --semver_rs > semver_rs_res.txt

node combine_results > combined.txt

