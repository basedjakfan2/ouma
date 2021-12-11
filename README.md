# ouma libc
ouma libc is a free implementation of C standard library in Rust language for Linux-based systems. It aims to be secure, small and correct. It (will) corresponds to C17 and POSIX.1-2017 standards. ouma tries to provide all of Linux syscalls through the libc for easier programming on Linux systems.

## Features (for near future)
 * Implemented in Rust language, uses `unsafe` where it's needed.
 * Highly portable
 * BSD 2-clause licensed
 * Provides BSD extensions such as arc4random, strlcpy, issetugid, fts, vis and more!
 * LLVM sanitizer support

## Features not expected in libc
 * Support for shadow passwords

## Name
The project was called in honor of video game character Kokichi Ouma.
