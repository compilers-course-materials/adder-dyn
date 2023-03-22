Idea is to dump the code into a .s file, then:

```
→ nasm -f macho64 build/our_code.s
→ ar rcs build/libour_code.a build/our_code.o
→ rustc -L build/ lib/start.rs
```

Basically a Rust port of https://github.com/compilers-course-materials/inlab1

`src/main.rs` has the parser/compiler

On my M1 mac had to do:

```
→ rustup target add x86_64-apple-darwin
→ rustc --target=x86_64-apple-darwin -L build/ lib/start.rs
```
