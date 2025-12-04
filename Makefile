docs_check:
	cargo doc --no-deps --document-private-items --all-features

docs:
	cargo doc --no-deps --document-private-items --all-features --open

fmt-check:
	cargo fmt --all -- --check

fmt:
	cargo fmt --all
	taplo format

clippy-check:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

clippy:
	cargo clippy --workspace --all-targets --all-features --fix --allow-staged

build:
	cargo build --workspace --bins

build-release:
	cargo build --release --workspace --bins

planner-test:
	cargo nextest -p plannertest

rewrite-planner-test:
	cargo run -p plannertest --bin apply
	
doc-test:
	cargo test --no-fail-fast --doc --all-features --workspace

unit-test: doc-test
	cargo test --no-fail-fast --lib --all-features --workspace

test: doc-test
	cargo test --no-fail-fast --all-targets --all-features --workspace

check: fmt_check clippy_check build test docs_check

clean:
	cargo clean

.PHONY: docs check fmt fmt_check clippy clippy_check build build_release test docs_check clean
