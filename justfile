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

check-all:
    cargo clippy -- -D warnings
    cargo fmt -- --check

ci: test check-all

bench:
    #!/usr/bin/env bash
    pushd bench
    bash gen.sh
    cat combined.txt
    popd

changelog:
    #!/usr/bin/env bash
    if [ ! $(which gren) ]; then
        npm install -g gren
    fi
    gren changelog --generate --data-source=commits --override