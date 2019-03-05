all: test

.PHONY: run
run:
	cargo run $(ARGS)

.PHONY: test
test:
	cargo test --verbose -- --nocapture

.PHONY: publish
publish:
	@cargo publish

.PHONY: example
example:
	cargo run --example main
