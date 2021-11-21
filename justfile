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