check-all: check clippy check_fmt

check:
	cargo check

clippy:
	cargo clippy -- -D warnings

check_fmt:
	cargo +nightly fmt -- --check
	# leptosfmt sample-crumbs/ --check

fmt:
	cargo +nightly fmt
