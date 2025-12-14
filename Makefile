RUSTC = rustc
RUST_FLAGS = --edition 2021 -A dead_code -A unused_imports
TASKS_DIR = tasks
TESTS_DIR = tests

.PHONY: all clean test help c1 c3 test-c1 test-c3 demo

help:
	@echo "make all - скомпилировать и запустить все задания."
	
all: tests 

c1: $(TESTS_DIR)/test_c1.rs $(TASKS_DIR)/c1_common_concepts.rs
	$(RUSTC) $(RUST_FLAGS) --crate-type bin -o c1_demo $(TESTS_DIR)/test_c1.rs
	./c1_demo

c3: $(TESTS_DIR)/c3.rs $(TASKS_DIR)/c3_ownership_and_memory.rs
	$(RUSTC) $(RUST_FLAGS) --crate-type bin -o c3_demo $(TESTS_DIR)/c3.rs
	./c3_demo

tests: c1 c3

clean:
	rm -f c1_demo c3_demo
	rm -f *.rlib

compile-c1:
	$(RUSTC) $(RUST_FLAGS) --crate-type rlib $(TASKS_DIR)/c1_common_concepts.rs

compile-c3:
	$(RUSTC) $(RUST_FLAGS) --crate-type rlib $(TASKS_DIR)/c3_ownership_and_memory.rs

check: compile-c1 compile-c3