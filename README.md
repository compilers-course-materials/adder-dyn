Idea is to dump the code into a .s file, then:

→ nasm -f macho64 build/our_code.s
→ ar rcs build/libour_code.a build/our_code.o
→ rustc -L build/ lib/start.rs
