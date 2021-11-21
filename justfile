setup:
    cargo install bacon

watch:
    bacon clippy

watch-all:
    bacon clippy-all

watch-test:
    bacon test-all

test:
    cargo test --all-features

bench:
    #!/usr/bin/env bash
    pushd bench
    bash gen.sh
    cat combined.txt
    popd