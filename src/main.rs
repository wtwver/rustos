#![no_std]
#![no_main] // error: requires `start` lang_item

mod vga_buffer;

// error: `#[panic_handler]` function required, but not found
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

//  disable name mangling 
#[no_mangle] 
// main doesn’t make sense without an underlying runtime that calls it
// this function is the entry point, since the linker looks for a function
// named `_start` by default
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}