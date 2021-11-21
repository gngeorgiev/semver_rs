#!/bin/bash

pushd collect_packages || exit 1
npm install
popd || exit 1
node collect_packages > ranges.txt

pushd bench_semver || exit 1
npm install
popd || exit 1
node bench_semver > semver_node_res.txt

cargo run --release -- --semver > semver_res.txt
cargo run --release -- --semver_rs > semver_rs_res.txt

node combine_results > combined.txt

