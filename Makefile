check-all: check clippy fmt

check:
	cargo check

clippy:
	cargo clippy -- -D warnings

fmt:
	cargo +nightly fmt -- --check
	leptosfmt sample-crumbs/ --check
