.PHONY: setup check lint fmt build run

setup:
	rustup component add rust-analyzer clippy rustfmt
	cargo fetch
	cargo check --workspace

check:
	cargo check --workspace

lint:
	cargo clippy --workspace --all-targets -- -D warnings

fmt:
	cargo fmt --all

build:
	cargo build --workspace

run:
	cargo run
