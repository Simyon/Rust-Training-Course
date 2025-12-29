# Rust-Training-Course
Training exercises for the Rust course in 1C [Fall 2025]

## Make all
The best way to make by enter this line:
```
make all SKIP="format fix clippy" 1> make_output.txt 2> make_errors.txt
```
or if you want just to pass test:
```
make test 1> make_output.txt 2> make_errors.txt
```

## Reflexia
### c1_common_concepts.rs
I've gotten clippy errors so I've add #[allow(clippy::comparison_chain)] and #[allow(clippy::needless_range_loop)].

The absence of ternary operator has disappointed me. It's kind of unaestetic to write oneline-if-branches.

### c3_ownership_and_memory.rs
I've gotten clippy error so I've add #[allow(clippy::ptr_arg)] because as I understand the task it's necessary to use string exactly.

Choose indexes for slicing aren't clear. 