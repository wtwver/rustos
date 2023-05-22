#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[macro_export]
macro_rules! println {
    () => ($crate::print(b"Default"));
    ($($arg:tt)*) => ($crate::print(b"ggg"));
}

static HELLO: &[u8] = b"Default text";
fn print(name: &[u8]) {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in name.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = i as u8;
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("asad");

    loop {}
}