RUST_PKG = jaq
SM_PKG = jaq
WASM_TARGET = wasm32-wasip1

clippy:
	cargo clippy --all-features --tests -- -D warnings
	cargo clippy --all-features --tests -p $(RUST_PKG) --target $(WASM_TARGET) -- -D warnings

build:
	smdk build


pack:
	smdk publish -pack

test:
	cargo test

fmt:
	cargo fmt -- --check


NAME_INPUT = "{ \"name\": \"John\", \"age\": 21 }"
NAME_OUTPUT = "John"
test_name:
	$(eval RESPONSE := $(shell smdk test --text $(NAME_INPUT) -e filter=.name))
	@echo "response from filter: $(RESPONSE)"
	@if [ "$(RESPONSE)" = "$(NAME_OUTPUT)" ]; then \
        echo "filtering name worked"; \
    else \
        echo "filtering name failed"; \
        exit 1; \
    fi


