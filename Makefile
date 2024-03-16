.PHONY: all check clean

all: build

check: fmt test clippy

test:
	@if [ ! -d "examples" ]; then mkdir examples; fi
	(command -v cargo-nextest && cargo nextest run --all-features) || cargo test --all-features

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets --tests -- -D warnings

.PHONY: clean
clean:
	rm -rf examples
	cargo clean

build:
	cargo build --release

run: contracts
	cargo run
	@echo
	@echo "Contracts are available in the examples directory"
	@echo "---------------------------------"
	@echo "./examples:"
	@ls -l examples| grep 'rgb'
	@echo "---------------------------------"

contracts:
	@if [ ! -d "examples" ]; then mkdir examples; fi
