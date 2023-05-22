# rustos

## A Freestanding Rust Binary

- cargo new blog_os --edition 2018
- no_std, panic_handler, Disabling Unwinding, no_main, \_start(), no_mangle
- Linker err: cargo build --target thumbv7em-none-eabihf
- x86_64-blog_os.json
- rustup override set nightly
- vga_buffer
- cargo bootimage
- qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin

## VGA Text Mode

- mod vga_buffer;
