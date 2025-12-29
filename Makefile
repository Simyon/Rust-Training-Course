.DEFAULT_GOAL := help

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

# Tasks list to enable and not to have merge conflicts
# just uncomment when task is done and ready to be checked
TASKS_TO_ANALYZE =
# ---------------------
TASKS_TO_ANALYZE += c1_common_concepts
# ---------------------
TASKS_TO_ANALYZE += c3_ownership_and_memory
# ---------------------
TASKS_TO_ANALYZE += c4_structs_methods_enums_pattern_matching
# ---------------------
TASKS_TO_ANALYZE += c5_collections
# ---------------------
TASKS_TO_ANALYZE += c6_error_handling_tests_docs
# ---------------------
#TASKS_TO_ANALYZE += c7_generics_traits_lifetimes
# ---------------------
#TASKS_TO_ANALYZE += c8_iterators_and_closures
# ---------------------
#TASKS_TO_ANALYZE += c9_smart_pointers
# ---------------------
#TASKS_TO_ANALYZE += c10_concurrency
# ---------------------
#TASKS_TO_ANALYZE += c12_object_oriented_programming
# ---------------------

# Clippy generates a lot of irritating errors at the start
# Usage make ... skip=clippy or skip="clippy fix" ...
SKIP ?= 
should_skip = $(if $(filter $(1),$(SKIP)),skip,run)

.PHONY: check_empty
check_empty:
	@[ -n "$(TASKS_TO_ANALYZE)" ] || ( echo "\033[31m No tasks enabled for analysis!\nPlease check the Makefile to enable at least one task. \033[0m" && false )

# -- linting --------------------------------------------------------------------------------------

.PHONY: clippy
clippy: check_empty ## Runs Clippy with configs
ifeq ($(call should_skip,clippy),skip)
	@echo "Skipping clippy"
else
	@which cargo > /dev/null 2>&1 && cargo clippy --workspace --all-targets --no-default-features --features "$(TASKS_TO_ANALYZE)" -- -D warnings || echo "cargo not found, skipping clippy"
endif


.PHONY: fix
fix: check_empty ## Runs Fix with configs
ifeq ($(call should_skip,fix),skip)
	@echo "Skipping fix"
else
	@which cargo > /dev/null 2>&1 && cargo fix --workspace --allow-staged --allow-dirty --all-targets --no-default-features --features "$(TASKS_TO_ANALYZE)" || echo "cargo not found, skipping fix"
endif


.PHONY: format
format: check_empty ## Runs Format using nightly toolchain
ifeq ($(call should_skip,format),skip)
	@echo "Skipping format"
else
	@which rustfmt > /dev/null 2>&1 && rustfmt --edition 2021 src/tasks/*.rs src/tests/*.rs src/*.rs || echo "rustfmt not found, skipping formatting"
endif


.PHONY: lint
lint: check_empty ## Runs all linting tasks at once (Clippy, fixing, formatting, typos)
	$(MAKE) format
	$(MAKE) fix
	$(MAKE) clippy

.PHONY: test
test: check_empty ## Runs tests
	@which cargo > /dev/null 2>&1 && cargo test --workspace --all-targets --no-default-features --features "$(TASKS_TO_ANALYZE)" || echo "cargo not found, skipping tests"

.PHONY: all
all: lint test ## Runs lint + test

.PHONY: clean
clean: ## Cleans build artifacts
	@which cargo > /dev/null 2>&1 && cargo clean || echo "cargo not found, skipping cargo clean"
	rm -f *.rlib
	rm -rf *.txt