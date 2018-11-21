all:
	@echo "no default"

.PHONY: run
run:
	cargo run $(ARGS)

.PHONY: test
test:
	cargo test

.PHONY: publish
publish:
	@cargo publish

.PHONY: test
test:
	@cargo test --verbose
