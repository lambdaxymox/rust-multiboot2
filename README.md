# Rust Multiboot 2
This library is a Rust implementation of the Multiboot 2 specification. In particular, it supports version 1.6 of Multiboot 2. This is found at
```
http://nongnu.askapache.com/grub/phcoder/multiboot.pdf
```

### Dependencies
The `rust-multiboot2` library is designed to have no external dependencies. It also operates without `std`.

### Status
This library is in early development. It presently lacks support for parsing ELF sections, VBE info, and APM tables.
