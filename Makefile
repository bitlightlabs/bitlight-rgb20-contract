.PHONY: all check clean

all: build

check: fmt test clippy

test: contracts
	(command -v cargo-nextest && cargo nextest run --all-features) || cargo test --all-features

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets --tests -- -D warnings

.PHONY: clean
clean:
	rm -rf test
	cargo clean

build:
	cargo build --release

run: contracts
	cargo run
	@echo
	@echo "Contracts are available in the test directory"
	@echo "---------------------------------"
	@echo "./test:"
	@ls -l test| grep 'rgb'
	@echo "---------------------------------"

contracts:
	@if [ ! -d "test" ]; then mkdir test; fi
