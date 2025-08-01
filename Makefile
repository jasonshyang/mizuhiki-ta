all: build

build:
	cargo build

release:
	cargo build --release

run:
	cargo run

test:
	cargo test

fmt:
	cargo fmt

check:
	cargo check --all-targets

clippy:
	cargo clippy -- -D warnings

doc:
	cargo doc --open

clean:
	cargo clean