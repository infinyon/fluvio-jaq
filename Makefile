build:
	smdk build

test:
	cargo test


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


