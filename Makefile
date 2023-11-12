all: build
build:
	cargo build --workspace

.PHONY: run-examples
run-examples:
	cargo run --example random_users

.PHONY: test
test:
	cargo test -- --test-threads=8

.PHONY: style-fix
style-fix:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy

.PHONY: clean
clean:
	cargo clean