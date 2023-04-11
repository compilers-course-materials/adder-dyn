Usage:

```
→ cargo build
   Compiling adder v0.1.0 (/Users/joe/src/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.94s
→ cargo run -- test/add.snek  test/add.s
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/x86_64-apple-darwin/debug/adder test/add.snek test/add.s`
Generated assembly:

section .text
global our_code_starts_here
our_code_starts_here:
  mov RAX, DWORD 73
add RAX, DWORD 1
sub RAX, DWORD 1
sub RAX, DWORD 1
  ret

Evaluates to:
72
```

Note that `main` does three different things for output:

1. It creates the `.s` file as usual
2. It prints out the assembly string
3. It converts the instructions to assembly directly, and evaluates the code

This shows some basics of how to use `dynasm` to generate code and invoke it
from the compiler.
